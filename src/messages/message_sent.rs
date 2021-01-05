use super::IJsonSerializable;
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct MessageSent {
    pub origin_offset: i64,
    pub service_instance_name: String,
    pub draft_id: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

impl MessageSent {
    pub fn new(origin_offset: i64, service_instance_name: String, draft_id: Uuid) -> Self {
        Self { origin_offset, draft_id, service_instance_name, timestamp: Utc::now().into() }
    }
}

impl IJsonSerializable for MessageSent {}
