use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use tapa_trait_serde::IJsonSerializable;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, IJsonSerializable)]
pub struct MessageSent {
    pub origin_offset: Option<i64>,
    pub service_instance_name: String,
    pub draft_id: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

impl MessageSent {
    pub fn new(origin_offset: Option<i64>, service_instance_name: &str, draft_id: Uuid) -> Self {
        Self {
            origin_offset,
            draft_id,
            service_instance_name: service_instance_name.into(),
            timestamp: Utc::now().into(),
        }
    }
}
