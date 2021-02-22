mod config;
mod mailer;
mod messages;
mod utils;

pub use log::{debug, error, info, log, warn};

use anyhow::{anyhow as anyerror, Result as AnyResult};
use bytes::Bytes;
use config::{MQConfig, MailerConfig};
use mailer::{EmailSendingResult, Mailer};
use messages::{MessageDraft, MessageFail, MessageFailType};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tapa_cgloop_nats::{CGLoop, NatsMessage, NatsMessageHandler, NatsOptions, ProcessResult};
use tapa_trait_serde::IJsonSerializable;
use tokio::runtime::Runtime;
use tokio::{join as wait_for_all, main as async_main};
use utils::{init_logger, wait_for_stop_signals};

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
        Some(Duration::from_secs(1)),
    )
}

struct DraftEmailConsumer {
    mailer: Mailer,
    config: MailerConfig,
    async_runtime: Runtime,
}

impl NatsMessageHandler for DraftEmailConsumer {
    fn handle_message(&mut self, message: &NatsMessage) -> AnyResult<ProcessResult> {
        let ten_seconds = Duration::from_secs(10);
        let service_instance_name = &self.config.instance_name;

        if let Ok(message_draft) = MessageDraft::from_json_bytes(&message.data[..]) {
            debug!("Got new message draft: {}", message_draft.to_json_string_pretty());

            loop {
                let retry_draft = message_draft.clone();

                match self.async_runtime.block_on(self.mailer.compose_and_send(
                    None,
                    service_instance_name,
                    retry_draft,
                )) {
                    EmailSendingResult::Fail(message_fail) => match &message_fail.fail_reason {
                        MessageFailType::Unknown => {
                            return Err(anyerror!("MessageFailType::Unknown should never occur!"));
                        }
                        MessageFailType::QuotaExhausted(duration_to_wait, error_string) => {
                            warn!("{}", error_string);
                            sleep(*duration_to_wait);
                            continue;
                        }
                        MessageFailType::Other(reason) | MessageFailType::BadDraft(reason) => {
                            error!("{}", reason);
                            let message_fail = Bytes::from(message_fail.to_json_bytes_pretty());
                            sleep(ten_seconds);
                            return Ok(ProcessResult::Failure(message_fail));
                        }
                    },
                    EmailSendingResult::Sent(message_success) => {
                        let message_success = Bytes::from(message_success.to_json_bytes_pretty());

                        return Ok(ProcessResult::Success(message_success));
                    }
                }
            }
        } else {
            let error_message = format!(
                "Cannot parse to correct JSON format, draft message length is {}",
                message.data.len()
            );
            error!("{}", error_message);
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
    let message_handler =
        Box::new(DraftEmailConsumer { config, mailer, async_runtime: Runtime::new()? });

    let results = wait_for_all! {
        async move {
            cg_loop.run(nats_options, shutdown_flag_clone, message_handler).await
        },
        async move {
            wait_for_stop_signals(shutdown_flag).await
        }
    };

    results.0.unwrap();

    Ok(())
}

#[async_main]
async fn main() -> AnyResult<()> {
    init_logger();

    let mailer_config = MailerConfig::load_from_env()?;
    info!("Mailer Config:\n{:#?}", mailer_config);

    run_mailer(mailer_config).await
}
