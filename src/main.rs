#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod mailer;
mod messages;
mod utils;

pub use log::{debug, error, info, log, warn};
pub use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

use config::{KafkaConfig, MailerConfig};
use futures::StreamExt;
use mailer::{EmailSendingResult, Mailer};
use messages::{IJsonSerializable, MessageDraft, MessageFail, MessageFailType};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{Message, Offset};
use tokio::time::{delay_for, Duration};
use utils::{init_logger, MINUTE_IN_SECONDS};

pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

const AUTO_OFFSET_RESET: &str = "auto.offset.reset";
const MAX_POLL_INTERVAL_MS: &str = "max.poll.interval.ms";
const BOOTSTRAP_SERVERS: &str = "bootstrap.servers";
const GROUP_ID: &str = "group.id";
const MESSAGE_TIMEOUT_MS: &str = "message.timeout.ms";
const HEARTBEAT_INTERVAL_MS: &str = "heartbeat.interval.ms";
const SESSION_TIMEOUT_MS: &str = "session.timeout.ms";
const ENABLE_AUTO_COMMIT: &str = "enable.auto.commit";

fn create_event_producer(kafka_config: &KafkaConfig) -> AnyResult<FutureProducer> {
    ClientConfig::new()
        .set(BOOTSTRAP_SERVERS, &kafka_config.kafka_brokers)
        .set(MESSAGE_TIMEOUT_MS, &kafka_config.kafka_produce_timeout_ms.to_string())
        .set(HEARTBEAT_INTERVAL_MS, &kafka_config.kafka_heartbeat_interval_ms.to_string())
        .create()
        .map_err(|x| x.into())
}

fn create_event_consumer(kafka_config: &KafkaConfig) -> AnyResult<StreamConsumer> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set(AUTO_OFFSET_RESET, "earliest")
        .set(MAX_POLL_INTERVAL_MS, "86400000")
        .set(GROUP_ID, &kafka_config.kafka_consumer_group_id)
        .set(BOOTSTRAP_SERVERS, &kafka_config.kafka_brokers)
        .set(SESSION_TIMEOUT_MS, &kafka_config.kafka_session_timeout_ms.to_string())
        .set(HEARTBEAT_INTERVAL_MS, &kafka_config.kafka_heartbeat_interval_ms.to_string())
        .set(ENABLE_AUTO_COMMIT, "false")
        .create()?;
    consumer.subscribe(&[&kafka_config.kafka_topic_draft])?;

    Ok(consumer)
}

async fn mailer_event_loop(runtime_config: MailerConfig) -> AnyResult<()> {
    let one_minute = Duration::from_secs(MINUTE_IN_SECONDS);
    let wait_forever = Duration::from_secs(0);
    let kafka_producer = create_event_producer(&runtime_config.kafka_config)?;
    let kafka_consumer = create_event_consumer(&runtime_config.kafka_config)?;
    let mut smtp_mailer = Mailer::new(runtime_config.smtp_config)?;
    let draft_topic = runtime_config.kafka_config.kafka_topic_draft;
    let fail_topic = runtime_config.kafka_config.kafka_topic_fail;
    let sent_topic = runtime_config.kafka_config.kafka_topic_sent;
    let instance_name = runtime_config.instance_name;
    info!("Running Mailer Event Loop...");

    while let Some(borrowed_message) = kafka_consumer.start().next().await {
        let borrowed_message: BorrowedMessage = borrowed_message?;
        let message_offset = borrowed_message.offset();
        let message_payload = borrowed_message.payload();
        let message_partition = borrowed_message.partition();

        if message_payload.is_none() {
            error!("Empty Message! Topic: {} Offset: {}", draft_topic, message_offset);
            // Commit first, because the mail already sent and the next step might be a transient fault
            kafka_consumer.commit_message(&borrowed_message, CommitMode::Async)?;
            let message_fail = MessageFail::new(
                message_offset,
                instance_name.clone(),
                "".into(),
                MessageFailType::BadDraft("Empty message".into()),
            )
            .to_json();

            while let Err((kafka_error, _)) = kafka_producer
                .send::<String, String, Duration>(
                    FutureRecord::to(&fail_topic).payload(&message_fail),
                    wait_forever,
                )
                .await
            {
                warn!(
                    "Error while producing to Topic: {} Error: {}.\nRetrying in {:?}...",
                    sent_topic, kafka_error, one_minute
                );
                delay_for(one_minute).await;
            }

            continue;
        }

        let message_payload = MessageDraft::from_slice(message_payload.unwrap());

        if message_payload.is_none() {
            error!("Un-serializable Message! Topic: {} Offset: {}", draft_topic, message_offset);
            // Commit first, because the mail already sent and the next step might be a transient fault
            kafka_consumer.commit_message(&borrowed_message, CommitMode::Async)?;
            let message_fail = MessageFail::new(
                message_offset,
                instance_name.clone(),
                "".into(),
                MessageFailType::BadDraft("Malformed message".into()),
            )
            .to_json();

            while let Err((kafka_error, _)) = kafka_producer
                .send::<String, String, Duration>(
                    FutureRecord::to(&fail_topic).payload(&message_fail),
                    wait_forever,
                )
                .await
            {
                warn!(
                    "Error while producing to Topic: {} Error: {}.\nRetrying in {:?}...",
                    sent_topic, kafka_error, one_minute
                );
                delay_for(one_minute).await;
            }

            continue;
        }

        let draft = message_payload.unwrap();

        match smtp_mailer.compose_and_send(message_offset, &instance_name, draft).await {
            EmailSendingResult::Sent(sent_info) => {
                let sent_info_json = sent_info.to_json();
                // Commit first, because the mail already sent and the next step might be a transient fault
                kafka_consumer.commit_message(&borrowed_message, CommitMode::Async)?;

                while let Err((kafka_error, _)) = kafka_producer
                    .send::<String, String, Duration>(
                        FutureRecord::to(&sent_topic).payload(&sent_info_json),
                        wait_forever,
                    )
                    .await
                {
                    warn!(
                        "Error while producing to Topic: {} Error: {}.\nRetrying in {:?}...",
                        sent_topic, kafka_error, one_minute
                    );
                    delay_for(one_minute).await;
                }
            }
            EmailSendingResult::Fail(fail_info) => match &fail_info.fail_reason {
                MessageFailType::Unknown => {
                    panic!("MessageFailType::Unknown should never occur!")
                }
                MessageFailType::QuotaExhausted(duration_to_wait, error_string) => {
                    warn!("{}", error_string);
                    kafka_consumer
                        .seek(
                            &draft_topic,
                            message_partition,
                            Offset::Offset(message_offset),
                            wait_forever,
                        )
                        .unwrap();
                    delay_for(*duration_to_wait).await;
                    continue;
                }
                MessageFailType::Other(reason) | MessageFailType::BadDraft(reason) => {
                    error!("{}", reason);
                    kafka_consumer.commit_message(&borrowed_message, CommitMode::Async)?;
                    let message_fail = fail_info.to_json();

                    while let Err((kafka_error, _)) = kafka_producer
                        .send::<String, String, Duration>(
                            FutureRecord::to(&fail_topic).payload(&message_fail),
                            wait_forever,
                        )
                        .await
                    {
                        warn!(
                            "Error while producing to Topic: {} Error: {}.\nRetrying in {:?}...",
                            sent_topic, kafka_error, one_minute
                        );
                        delay_for(one_minute).await;
                    }
                }
            },
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> AnyResult<()> {
    init_logger();

    let runtime_config = MailerConfig::load_from_env()?;
    info!("Runtime Config:\n{:#?}", runtime_config);

    mailer_event_loop(runtime_config).await
}
