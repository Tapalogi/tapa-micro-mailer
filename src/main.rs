#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod messages;
mod utils;

pub use log::{debug, error, info, log, warn};
pub use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

use config::MailerConfig;
use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use messages::{EmailSendingResult, MessageDraft};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::error::KafkaResult;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use utils::{get_hostname, get_new_uuid_v4, init_logger};

const BOOTSTRAP_SERVERS: &str = "bootstrap.servers";
const GROUP_ID: &str = "group.id";
const MESSAGE_TIMEOUT_MS: &str = "message.timeout.ms";
const HEARTBEAT_INTERVAL_MS: &str = "heartbeat.interval.ms";
const SESSION_TIMEOUT_MS: &str = "session.timeout.ms";
const ENABLE_AUTO_COMMIT: &str = "enable.auto.commit";

fn create_event_producer(runtime_config: &MailerConfig) -> IOResult<FutureProducer> {
    let producer_creation_result: KafkaResult<FutureProducer> = ClientConfig::new()
        .set(BOOTSTRAP_SERVERS, &runtime_config.kafka_brokers)
        .set(MESSAGE_TIMEOUT_MS, &runtime_config.kafka_produce_timeout_ms.to_string())
        .set(HEARTBEAT_INTERVAL_MS, &runtime_config.kafka_heartbeat_interval_ms.to_string())
        .create();

    match producer_creation_result {
        Err(kafka_error) => Err(IOError::new(
            IOErrorKind::Other,
            format!("Kafka Error! {}", kafka_error.to_string()),
        )),
        Ok(producer) => Ok(producer),
    }
}

fn create_event_consumer(runtime_config: &MailerConfig) -> IOResult<StreamConsumer> {
    let consumer_creation_result: KafkaResult<StreamConsumer> = ClientConfig::new()
        .set(GROUP_ID, &runtime_config.kafka_consumer_group_id)
        .set(BOOTSTRAP_SERVERS, &runtime_config.kafka_brokers)
        .set(SESSION_TIMEOUT_MS, &runtime_config.kafka_session_timeout_ms.to_string())
        .set(HEARTBEAT_INTERVAL_MS, &runtime_config.kafka_heartbeat_interval_ms.to_string())
        .set(ENABLE_AUTO_COMMIT, "false")
        .create();

    if let Err(kafka_error) = consumer_creation_result {
        return Err(IOError::new(
            IOErrorKind::Other,
            format!("Kafka Error! {}", kafka_error.to_string()),
        ));
    }

    let consumer = consumer_creation_result.unwrap();

    if let Err(consume_error) = consumer.subscribe(&[&runtime_config.kafka_topic_draft]) {
        return Err(IOError::new(
            IOErrorKind::Other,
            format!("Kafka Consume Error! {}", consume_error.to_string()),
        ));
    }

    Ok(consumer)
}

async fn do_send_mail(draft: &MessageDraft) -> IOResult<EmailSendingResult> {
    todo!("Implement send mail!");
}

async fn mailer_event_loop(
    runtime_config: MailerConfig,
    service_hostname: String,
    kafka_producer: FutureProducer,
    kafka_consumer: StreamConsumer,
) -> IOResult<()> {
    todo!("Implement mailer business logic!");
}

#[tokio::main]
async fn main() -> IOResult<()> {
    init_logger();

    let runtime_config = MailerConfig::load_from_env()?;
    let service_hostname = get_hostname();
    let kafka_producer = create_event_producer(&runtime_config)?;
    let kafka_consumer = create_event_consumer(&runtime_config)?;

    mailer_event_loop(runtime_config, service_hostname, kafka_producer, kafka_consumer).await
}
