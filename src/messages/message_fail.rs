use super::{IJsonSerializable, MessageDraft};
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

#[derive(Deserialize, Serialize, Clone)]
pub enum MessageFailType {
    #[serde(rename = "OTHER")]
    Other(String),
    #[serde(rename = "BAD_DRAFT")]
    BadDraft(String),
    #[serde(rename = "QUOTA_EXHAUSTED")]
    QuotaExhausted(Duration, String),
    #[serde(rename = "UNKNOWN")]
    Unknown, // This kind of error should not exist
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MessageFail {
    pub origin_offset: Option<i64>,
    pub service_instance_name: Option<String>,
    pub message_copy: MessageDraft,
    pub fail_reason: MessageFailType,
    pub timestamp: DateTime<FixedOffset>,
}

impl MessageFail {
    pub fn new(
        origin_offset: Option<i64>,
        service_instance_name: Option<String>,
        message_copy: MessageDraft,
        fail_reason: MessageFailType,
    ) -> Self {
        Self {
            origin_offset,
            service_instance_name,
            message_copy,
            fail_reason,
            timestamp: Utc::now().into(),
        }
    }
}

impl IJsonSerializable for MessageFailType {}
impl IJsonSerializable for MessageFail {}
