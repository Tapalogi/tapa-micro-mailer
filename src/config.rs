use super::{anyerror, debug, AnyResult};
use crate::utils::get_hostname;
use secstr::SecStr;
use std::env::var;

#[derive(Debug)]
pub struct MQConfig {
    pub mq_url: String,
    pub mq_consumer_group: String,
    pub mq_topic_source: String,
    pub mq_topic_failure: String,
    pub mq_topic_success: String,
}

impl MQConfig {
    pub fn load_from_env() -> AnyResult<Self> {
        let mq_url;
        let mq_consumer_group;
        let mq_topic_source;
        let mq_topic_failure;
        let mq_topic_success;

        if let Ok(brokers) = var("MQ_URL") {
            mq_url = brokers;
        } else {
            return Err(anyerror!("MQ_URL not set!"));
        }

        if let Ok(consumer_group_id) = var("MQ_CONSUMER_GROUP") {
            mq_consumer_group = consumer_group_id;
        } else {
            return Err(anyerror!("MQ_CONSUMER_GROUP not set!"));
        }

        if let Ok(topic_draft) = var("MQ_TOPIC_SOURCE") {
            mq_topic_source = topic_draft;
        } else {
            return Err(anyerror!("MQ_TOPIC_SOURCE not set!"));
        }

        if let Ok(topic_fail) = var("MQ_TOPIC_FAILURE") {
            mq_topic_failure = topic_fail;
        } else {
            return Err(anyerror!("MQ_TOPIC_FAILURE not set!"));
        }

        if let Ok(topic_sent) = var("MQ_TOPIC_SUCCESS") {
            mq_topic_success = topic_sent;
        } else {
            return Err(anyerror!("MQ_TOPIC_SUCCESS not set!"));
        }

        Ok(Self { mq_url, mq_consumer_group, mq_topic_source, mq_topic_failure, mq_topic_success })
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
    pub fn load_from_env() -> AnyResult<Self> {
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
            return Err(anyerror!("SMTP_HOST not set!"));
        }

        if let Ok(smtp_user) = var("SMTP_USER") {
            user = smtp_user;
        } else {
            return Err(anyerror!("SMTP_USER not set!"));
        }

        if let Ok(smtp_pass) = var("SMTP_PASS") {
            pass = SecStr::from(smtp_pass);
        } else {
            return Err(anyerror!("SMTP_PASS not set!"));
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
    pub mq_config: MQConfig,
    pub smtp_config: SmtpConfig,
    pub instance_name: String,
}

impl MailerConfig {
    pub fn load_from_env() -> AnyResult<Self> {
        let mq_config = MQConfig::load_from_env()?;
        let smtp_config = SmtpConfig::load_from_env()?;
        let instance_name;

        if let Ok(mailer_instance_name) = var("MAILER_INSTANCE_NAME") {
            instance_name = format!("{}_{}", mailer_instance_name, get_hostname());
        } else {
            return Err(anyerror!("MAILER_INSTANCE_NAME not set!"));
        }

        Ok(Self { instance_name, mq_config, smtp_config })
    }
}
