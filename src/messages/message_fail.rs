use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use tapa_trait_serde::IJsonSerializable;
use tokio::time::Duration;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, IJsonSerializable)]
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

#[derive(Deserialize, Serialize, Clone, IJsonSerializable)]
pub struct MessageFail {
    pub origin_offset: i64,
    pub service_instance_name: String,
    pub message_copy: String,
    pub fail_reason: MessageFailType,
    pub timestamp: DateTime<FixedOffset>,
}

impl MessageFail {
    pub fn new(
        origin_offset: i64,
        service_instance_name: String,
        message_copy: String,
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
