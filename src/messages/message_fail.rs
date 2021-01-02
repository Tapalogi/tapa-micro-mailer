use super::{IJsonSerializable, MessageDraft};
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum MessageFailType {
    #[serde(rename = "REJECTED_BY_SERVER")]
    RejectedByServer(String),
    #[serde(rename = "BAD_DRAFT")]
    BadDraft(String),
    #[serde(rename = "BAD_CREDENTIALS")]
    BadCredentials,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MessageFail {
    pub origin_offset: i64,
    pub service_instance_name: String,
    pub message_copy: MessageDraft,
    pub fail_reason: MessageFailType,
    pub timestamp: DateTime<FixedOffset>,
}

impl MessageFail {
    pub fn new(
        origin_offset: i64,
        service_instance_name: &str,
        message_copy: MessageDraft,
        fail_reason: MessageFailType,
    ) -> Self {
        Self {
            origin_offset,
            service_instance_name: service_instance_name.into(),
            message_copy,
            fail_reason,
            timestamp: Utc::now().into(),
        }
    }
}

impl IJsonSerializable for MessageFailType {}
impl IJsonSerializable for MessageFail {}
