#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod messages;
mod utils;

pub use log::{debug, error, info, log, warn};
pub use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

use config::MailerConfig;
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaResult;
use rdkafka::producer::{FutureProducer, FutureRecord};
use utils::{get_hostname, init_logger};

fn create_event_producer(runtime_config: &MailerConfig) -> KafkaResult<FutureProducer> {
    ClientConfig::new()
        .set("bootstrap.servers", &runtime_config.kafka_brokers)
        .set("message.timeout.ms", &runtime_config.kafka_produce_timeout_ms.to_string())
        .set("heartbeat.interval.ms", &runtime_config.kafka_heartbeat_interval_ms.to_string())
        .create()
}

#[tokio::main]
async fn main() -> IOResult<()> {
    init_logger();

    let runtime_config = MailerConfig::load_from_env()?;
    let service_hostname = get_hostname();
    let kafka_producer;

    match create_event_producer(&runtime_config) {
        Err(kafka_error) => {
            return Err(IOError::new(
                IOErrorKind::Other,
                format!("Kafka Error! {}", kafka_error.to_string()),
            ))
        }
        Ok(producer) => kafka_producer = producer,
    }

    Ok(())
}
