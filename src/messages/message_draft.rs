use super::IJsonSerializable;
use crate::utils::is_valid_email_string;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum MessageDraftBodyType {
    #[serde(rename = "ASCII")]
    Ascii,
    #[serde(rename = "HTML")]
    Html,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MessageDraft {
    pub id: Uuid,
    pub email_to: String,
    pub email_to_name: Option<String>,
    pub email_from: String,
    pub email_from_name: Option<String>,
    pub subject: String,
    pub body_type: MessageDraftBodyType,
    pub body: String,
    pub timestamp: DateTime<FixedOffset>,
}

impl MessageDraft {
    pub fn has_invalid_sender(&self) -> bool {
        !is_valid_email_string(&self.email_from)
    }

    pub fn has_invalid_destination(&self) -> bool {
        !is_valid_email_string(&self.email_to)
    }

    pub fn has_empty_body(&self) -> bool {
        self.body.is_empty()
    }

    pub fn has_empty_subject(&self) -> bool {
        self.subject.is_empty()
    }
}

impl IJsonSerializable for MessageDraftBodyType {}
impl IJsonSerializable for MessageDraft {}
