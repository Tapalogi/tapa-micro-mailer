mod config;
mod mailer;
mod messages;
mod utils;

pub use log::{debug, error, info, log, warn};

use anyhow::{anyhow as anyerror, Result as AnyResult};
use async_trait::async_trait;
use bytes::Bytes;
use config::{MQConfig, MailerConfig};
use mailer::{EmailSendingResult, Mailer};
use messages::{MessageDraft, MessageFail, MessageFailType};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tapa_cgloop_nats::{CGLoop, NatsMessage, NatsMessageHandler, NatsOptions, ProcessResult};
use tapa_trait_serde::IJsonSerializable;
use tokio::time::{delay_for, Duration};
use tokio::{join as wait_for_all, main as async_main};
use utils::{init_logger, wait_for_stop_signals, MINUTE_IN_SECONDS};

fn create_nats_options(instance_name: &str) -> NatsOptions {
    NatsOptions::new().max_reconnects(None).with_name(instance_name)
}

fn create_cg_loop(mq_config: &MQConfig) -> CGLoop {
    CGLoop::new(
        &mq_config.mq_url,
        &mq_config.mq_topic_source,
        &mq_config.mq_topic_success,
        &mq_config.mq_topic_failure,
        &mq_config.mq_consumer_group,
        false,
    )
}

struct DraftEmailConsumer {
    mailer: Mailer,
    config: MailerConfig,
}

#[async_trait]
impl NatsMessageHandler for DraftEmailConsumer {
    async fn handle_message<'a>(&mut self, message: &'a NatsMessage) -> AnyResult<ProcessResult> {
        let one_minute = Duration::from_secs(MINUTE_IN_SECONDS);
        let service_instance_name = &self.config.instance_name;

        if let Ok(message_draft) = MessageDraft::from_json_bytes(&message.data[..]) {
            match self.mailer.compose_and_send(None, service_instance_name, message_draft).await {
                EmailSendingResult::Fail(message_fail) => match &message_fail.fail_reason {
                    MessageFailType::Unknown => {
                        Err(anyerror!("MessageFailType::Unknown should never occur!"))
                    }
                    MessageFailType::QuotaExhausted(duration_to_wait, error_string) => {
                        warn!("{}", error_string);
                        let message_fail = Bytes::from(message_fail.to_json_bytes_pretty());
                        delay_for(*duration_to_wait).await;

                        Ok(ProcessResult::Failure(message_fail))
                    }
                    MessageFailType::Other(reason) | MessageFailType::BadDraft(reason) => {
                        error!("{}", reason);
                        let message_fail = Bytes::from(message_fail.to_json_bytes_pretty());
                        delay_for(one_minute).await;

                        Ok(ProcessResult::Failure(message_fail))
                    }
                },
                EmailSendingResult::Sent(message_success) => {
                    let message_success = Bytes::from(message_success.to_json_bytes_pretty());

                    Ok(ProcessResult::Success(message_success))
                }
            }
        } else {
            let error_message = format!(
                "Cannot parse to correct JSON format, draft message length is {}",
                message.data.len()
            );
            let message_fail = MessageFail::new(
                None,
                service_instance_name,
                error_message.clone(),
                MessageFailType::BadDraft(error_message),
            );
            let message_fail = Bytes::from(message_fail.to_json_bytes_pretty());

            Ok(ProcessResult::Failure(message_fail))
        }
    }
}

async fn run_mailer(config: MailerConfig) -> AnyResult<()> {
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let shutdown_flag_clone = shutdown_flag.clone();
    let cg_loop = create_cg_loop(&config.mq_config);
    let nats_options = create_nats_options(&config.instance_name);
    let mailer = Mailer::new(&config.smtp_config)?;
    let mut message_handler = DraftEmailConsumer { config, mailer };

    let _ = wait_for_all! {
        async move {
            cg_loop.run(nats_options, shutdown_flag_clone, &mut message_handler).await
        },
        async move {
            wait_for_stop_signals(shutdown_flag).await
        }
    };

    Ok(())
}

#[async_main]
async fn main() -> AnyResult<()> {
    init_logger();

    let mailer_config = MailerConfig::load_from_env()?;
    info!("Mailer Config:\n{:#?}", mailer_config);

    run_mailer(mailer_config).await
}
