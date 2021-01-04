use super::{debug, IOError, IOErrorKind, IOResult};
use crate::utils::get_hostname;
use secstr::SecStr;
use std::env::var;

#[derive(Debug)]
pub struct KafkaConfig {
    pub kafka_brokers: String,
    pub kafka_consumer_group_id: String,
    pub kafka_topic_draft: String,
    pub kafka_topic_fail: String,
    pub kafka_topic_sent: String,
    pub kafka_session_timeout_ms: u32,
    pub kafka_heartbeat_interval_ms: u32,
    pub kafka_produce_timeout_ms: u32,
}

impl KafkaConfig {
    pub fn load_from_env() -> IOResult<Self> {
        let kafka_brokers;
        let kafka_consumer_group_id;
        let kafka_topic_draft;
        let kafka_topic_fail;
        let kafka_topic_sent;
        let mut kafka_session_timeout_ms = 5000;
        let mut kafka_heartbeat_interval_ms = 1000;
        let mut kafka_produce_timeout_ms = 5000;

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

        if let Ok(session_timeout_ms) = var("KAFKA_SESSION_TIMEOUT_MS") {
            if let Ok(parsed_session_timeout_ms) = session_timeout_ms.parse::<u32>() {
                kafka_session_timeout_ms = parsed_session_timeout_ms;
                debug!("KAFKA_SESSION_TIMEOUT_MS overridden with {}", parsed_session_timeout_ms);
            }
        }

        if let Ok(heartbeat_interval_ms) = var("KAFKA_HEARTBEAT_INTERVAL_MS") {
            if let Ok(parsed_heartbeat_interval_ms) = heartbeat_interval_ms.parse::<u32>() {
                kafka_heartbeat_interval_ms = parsed_heartbeat_interval_ms;
                debug!(
                    "KAFKA_HEARTBEAT_INTERVAL_MS overridden with {}",
                    parsed_heartbeat_interval_ms
                );
            }
        }

        if let Ok(produce_timeout_ms) = var("KAFKA_PRODUCE_TIMEOUT_MS") {
            if let Ok(parsed_produce_timeout_ms) = produce_timeout_ms.parse::<u32>() {
                kafka_produce_timeout_ms = parsed_produce_timeout_ms;
                debug!("KAFKA_PRODUCE_TIMEOUT_MS overridden with {}", parsed_produce_timeout_ms);
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
        })
    }
}

#[derive(Debug)]
pub struct SmtpConfig {
    pub use_starttls: bool,
    pub host: String,
    pub user: String,
    pub pass: SecStr,
    pub max_per_second: Option<usize>,
    pub max_per_minute: Option<usize>,
    pub max_per_hour: Option<usize>,
    pub max_per_day: Option<usize>,
}

impl SmtpConfig {
    pub fn load_from_env() -> IOResult<Self> {
        let host;
        let user;
        let pass;
        let mut use_starttls = false;
        let mut max_per_second = None;
        let mut max_per_minute = None;
        let mut max_per_hour = None;
        let mut max_per_day = None;

        if let Ok(smtp_host) = var("SMTP_HOST") {
            host = smtp_host;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "SMTP_HOST not set!"));
        }

        if let Ok(smtp_user) = var("SMTP_USER") {
            user = smtp_user;
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "SMTP_USER not set!"));
        }

        if let Ok(smtp_pass) = var("SMTP_PASS") {
            pass = SecStr::from(smtp_pass);
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "SMTP_PASS not set!"));
        }

        if let Ok(smtp_use_starttls) = var("SMTP_USE_STARTTLS") {
            if let Ok(parsed_use_starttls) = smtp_use_starttls.parse::<bool>() {
                use_starttls = parsed_use_starttls;
                debug!("SMTP_USE_STARTTLS overridden with {}", parsed_use_starttls);
            }
        }

        if let Ok(smtp_max_per_second) = var("SMTP_MAX_PER_SECOND") {
            if let Ok(parsed_max_per_second) = smtp_max_per_second.parse::<usize>() {
                max_per_second = Some(parsed_max_per_second);
                debug!("SMTP_MAX_PER_SECOND overridden with {}", parsed_max_per_second);
            }
        }

        if let Ok(smtp_max_per_minute) = var("SMTP_MAX_PER_MINUTE") {
            if let Ok(parsed_max_per_minute) = smtp_max_per_minute.parse::<usize>() {
                max_per_minute = Some(parsed_max_per_minute);
                debug!("SMTP_MAX_PER_MINUTE overridden with {}", parsed_max_per_minute);
            }
        }

        if let Ok(smtp_max_per_hour) = var("SMTP_MAX_PER_HOUR") {
            if let Ok(parsed_max_per_hour) = smtp_max_per_hour.parse::<usize>() {
                max_per_hour = Some(parsed_max_per_hour);
                debug!("SMTP_MAX_PER_HOUR overridden with {}", parsed_max_per_hour);
            }
        }

        if let Ok(smtp_max_per_day) = var("SMTP_MAX_PER_DAY") {
            if let Ok(parsed_max_per_day) = smtp_max_per_day.parse::<usize>() {
                max_per_day = Some(parsed_max_per_day);
                debug!("SMTP_MAX_PER_DAY overridden with {}", parsed_max_per_day);
            }
        }

        Ok(Self {
            max_per_second,
            max_per_minute,
            max_per_hour,
            max_per_day,
            use_starttls,
            host,
            user,
            pass,
        })
    }
}

#[derive(Debug)]
pub struct MailerConfig {
    pub kafka_config: KafkaConfig,
    pub smtp_config: SmtpConfig,
    pub instance_name: String,
}

impl MailerConfig {
    pub fn load_from_env() -> IOResult<Self> {
        let kafka_config = KafkaConfig::load_from_env()?;
        let smtp_config = SmtpConfig::load_from_env()?;
        let instance_name;

        if let Ok(mailer_instance_name) = var("MAILER_INSTANCE_NAME") {
            instance_name = format!("{}_{}", mailer_instance_name, get_hostname());
        } else {
            return Err(IOError::new(IOErrorKind::InvalidData, "MAILER_INSTANCE_NAME not set!"));
        }

        Ok(Self { instance_name, kafka_config, smtp_config })
    }
}
