use super::{IOError, IOErrorKind, IOResult};
use std::env::var;

pub struct MailerConfig {
    pub kafka_brokers: String,
    pub kafka_consumer_group_id: String,
    pub kafka_topic_draft: String,
    pub kafka_topic_fail: String,
    pub kafka_topic_sent: String,
    pub kafka_session_timeout_ms: u32,
    pub kafka_heartbeat_interval_ms: u32,
    pub kafka_produce_timeout_ms: u32,
    pub smtp_use_ssl: bool,
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_pass: String,
}

impl MailerConfig {
    pub fn load_from_env() -> IOResult<Self> {
        let kafka_brokers;
        let kafka_consumer_group_id;
        let kafka_topic_draft;
        let kafka_topic_fail;
        let kafka_topic_sent;
        let smtp_host;
        let smtp_user;
        let smtp_pass;
        let mut kafka_session_timeout_ms = 5000;
        let mut kafka_heartbeat_interval_ms = 1000;
        let mut kafka_produce_timeout_ms = 5000;
        let mut smtp_use_ssl = false;

        if let Ok(brokers) = var("KAFKA_BROKERS") {
            kafka_brokers = brokers;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "KAFKA_BROKERS not set!"));
        }

        if let Ok(consumer_group_id) = var("KAFKA_CONSUMER_GROUP_ID") {
            kafka_consumer_group_id = consumer_group_id;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "KAFKA_CONSUMER_GROUP_ID not set!"));
        }

        if let Ok(topic_draft) = var("KAFKA_TOPIC_DRAFT") {
            kafka_topic_draft = topic_draft;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "KAFKA_TOPIC_DRAFT not set!"));
        }

        if let Ok(topic_fail) = var("KAFKA_TOPIC_FAIL") {
            kafka_topic_fail = topic_fail;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "KAFKA_TOPIC_FAIL not set!"));
        }

        if let Ok(topic_sent) = var("KAFKA_TOPIC_SENT") {
            kafka_topic_sent = topic_sent;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "KAFKA_TOPIC_SENT not set!"));
        }

        if let Ok(host) = var("SMTP_HOST") {
            smtp_host = host;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "SMTP_HOST not set!"));
        }

        if let Ok(user) = var("SMTP_USER") {
            smtp_user = user;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "SMTP_USER not set!"));
        }

        if let Ok(pass) = var("SMTP_PASS") {
            smtp_pass = pass;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "SMTP_PASS not set!"));
        }

        if let Ok(session_timeout_ms) = var("KAFKA_SESSION_TIMEOUT_MS") {
            if let Ok(parsed_session_timeout_ms) = session_timeout_ms.parse::<u32>() {
                kafka_session_timeout_ms = parsed_session_timeout_ms;
            }
        }

        if let Ok(heartbeat_interval_ms) = var("KAFKA_HEARTBEAT_INTERVAL_MS") {
            if let Ok(parsed_heartbeat_interval_ms) = heartbeat_interval_ms.parse::<u32>() {
                kafka_heartbeat_interval_ms = parsed_heartbeat_interval_ms;
            }
        }

        if let Ok(produce_timeout_ms) = var("KAFKA_PRODUCE_TIMEOUT_MS") {
            if let Ok(parsed_produce_timeout_ms) = produce_timeout_ms.parse::<u32>() {
                kafka_produce_timeout_ms = parsed_produce_timeout_ms;
            }
        }

        if let Ok(use_ssl) = var("SMTP_USE_SSL") {
            if let Ok(parsed_use_ssl) = use_ssl.parse::<bool>() {
                smtp_use_ssl = parsed_use_ssl;
            }
        }

        Ok(Self {
            kafka_brokers,
            kafka_consumer_group_id,
            kafka_topic_draft,
            kafka_topic_fail,
            kafka_topic_sent,
            kafka_session_timeout_ms,
            kafka_heartbeat_interval_ms,
            kafka_produce_timeout_ms,
            smtp_use_ssl,
            smtp_host,
            smtp_user,
            smtp_pass,
        })
    }
}
