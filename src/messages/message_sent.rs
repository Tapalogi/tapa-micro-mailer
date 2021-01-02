use super::IJsonSerializable;
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct MessageSent {
    pub origin_offset: i64,
    pub draft_id: Uuid,
    pub service_instance_name: String,
    pub timestamp: DateTime<FixedOffset>,
}

impl MessageSent {
    pub fn new(origin_offset: i64, draft_id: Uuid, service_instance_name: &str) -> Self {
        Self {
            origin_offset,
            draft_id,
            service_instance_name: service_instance_name.into(),
            timestamp: Utc::now().into(),
        }
    }
}

impl IJsonSerializable for MessageSent {}
