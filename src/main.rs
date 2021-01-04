#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod mailer;
mod messages;
mod utils;

pub use log::{debug, error, info, log, warn};
pub use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

use config::{KafkaConfig, MailerConfig, SmtpConfig};
use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use messages::MessageDraft;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::error::KafkaResult;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use utils::{get_hostname, init_logger};

pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

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
    let kafka_producer = create_event_producer(&runtime_config.kafka_config)?;
    let kafka_consumer = create_event_consumer(&runtime_config.kafka_config)?;

    todo!("Implement mailer business logic!");
}

#[tokio::main]
async fn main() -> AnyResult<()> {
    init_logger();

    let runtime_config = MailerConfig::load_from_env()?;
    info!("Runtime Config:\n{:#?}", runtime_config);

    mailer_event_loop(runtime_config).await
}
