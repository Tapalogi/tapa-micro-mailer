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
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::error::KafkaResult;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use utils::{get_hostname, init_logger};

fn create_event_producer(runtime_config: &MailerConfig) -> IOResult<FutureProducer> {
    let producer_creation_result: KafkaResult<FutureProducer> = ClientConfig::new()
        .set("bootstrap.servers", &runtime_config.kafka_brokers)
        .set("message.timeout.ms", &runtime_config.kafka_produce_timeout_ms.to_string())
        .set("heartbeat.interval.ms", &runtime_config.kafka_heartbeat_interval_ms.to_string())
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
        .set("group.id", &runtime_config.kafka_consumer_group_id)
        .set("bootstrap.servers", &runtime_config.kafka_brokers)
        .set("session.timeout.ms", &runtime_config.kafka_session_timeout_ms.to_string())
        .set("heartbeat.interval.ms", &runtime_config.kafka_heartbeat_interval_ms.to_string())
        .set("enable.auto.commit", "false")
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

#[tokio::main]
async fn main() -> IOResult<()> {
    init_logger();

    let runtime_config = MailerConfig::load_from_env()?;
    let service_hostname = get_hostname();
    let kafka_producer = create_event_producer(&runtime_config)?;
    let kafka_consumer = create_event_consumer(&runtime_config)?;

    Ok(())
}
