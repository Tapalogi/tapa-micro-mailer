mod resettable_bucket;

use crate::config::SmtpConfig;
use crate::messages::{
    MessageDraft, MessageDraftBodyType, MessageFail, MessageFailType, MessageSent,
};
use crate::utils::{DAY_IN_SECONDS, HOUR_IN_SECONDS, MINUTE_IN_SECONDS};
use crate::{anyerror, AnyResult};
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Address, AsyncSmtpTransport, Message as Email, Tokio02Connector, Tokio02Transport};
use resettable_bucket::ResettableBucket;
use tapa_trait_serde::IJsonSerializable;
use tokio::time::{Duration, Instant};

pub enum EmailSendingResult {
    Fail(MessageFail),
    Sent(MessageSent),
}

pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio02Connector>,
    bucket_second: Option<ResettableBucket>,
    bucket_minute: Option<ResettableBucket>,
    bucket_hour: Option<ResettableBucket>,
    bucket_day: Option<ResettableBucket>,
}

impl Mailer {
    pub fn new(smtp_config: &SmtpConfig) -> AnyResult<Self> {
        let creds = Credentials::new(smtp_config.user.clone(), smtp_config.pass.to_string());
        let mailer_build_result = if smtp_config.use_starttls {
            AsyncSmtpTransport::<Tokio02Connector>::starttls_relay(&smtp_config.host)
        } else {
            AsyncSmtpTransport::<Tokio02Connector>::relay(&smtp_config.host)
        };

        match mailer_build_result {
            Err(build_error) => Err(anyerror!(build_error.to_string())),
            Ok(mailer) => {
                let mut bucket_second = None;
                let mut bucket_minute = None;
                let mut bucket_hour = None;
                let mut bucket_day = None;

                if let Some(mpt) = smtp_config.max_per_second.as_ref() {
                    bucket_second = Some(ResettableBucket::new(*mpt, Duration::from_secs(1)));
                }

                if let Some(mpt) = smtp_config.max_per_minute.as_ref() {
                    bucket_minute =
                        Some(ResettableBucket::new(*mpt, Duration::from_secs(MINUTE_IN_SECONDS)));
                }

                if let Some(mpt) = smtp_config.max_per_hour.as_ref() {
                    bucket_hour =
                        Some(ResettableBucket::new(*mpt, Duration::from_secs(HOUR_IN_SECONDS)));
                }

                if let Some(mpt) = smtp_config.max_per_day.as_ref() {
                    bucket_day =
                        Some(ResettableBucket::new(*mpt, Duration::from_secs(DAY_IN_SECONDS)));
                }

                Ok(Self {
                    bucket_day,
                    bucket_hour,
                    bucket_minute,
                    bucket_second,
                    transport: mailer.credentials(creds).build(),
                })
            }
        }
    }

    pub async fn compose_and_send(
        &mut self,
        origin_offset: Option<i64>,
        service_instance_name: &str,
        draft: MessageDraft,
    ) -> EmailSendingResult {
        let current_instant = Instant::now();
        let mut message_fail = MessageFail::new(
            origin_offset,
            service_instance_name,
            draft.to_json_string_pretty(),
            MessageFailType::Unknown,
        );

        if draft.has_empty_body() {
            message_fail.fail_reason = MessageFailType::BadDraft("Empty body!".into());
            return EmailSendingResult::Fail(message_fail);
        }

        if draft.has_empty_subject() {
            message_fail.fail_reason = MessageFailType::BadDraft("Empty subject!".into());
            return EmailSendingResult::Fail(message_fail);
        }

        if draft.has_invalid_destination() {
            message_fail.fail_reason = MessageFailType::BadDraft("Invalid destination!".into());
            return EmailSendingResult::Fail(message_fail);
        }

        if draft.has_invalid_sender() {
            message_fail.fail_reason = MessageFailType::BadDraft("Invalid sender!".into());
            return EmailSendingResult::Fail(message_fail);
        }

        // Check max per second bucket
        if let Some(bucket) = self.bucket_second.as_mut() {
            if let Some(duration_to_wait) = bucket.try_take(&current_instant) {
                message_fail.fail_reason = MessageFailType::QuotaExhausted(
                    duration_to_wait,
                    "Exhausted maximum email per second!".into(),
                );
                return EmailSendingResult::Fail(message_fail);
            }
        }

        // Check max per minute bucket
        if let Some(bucket) = self.bucket_minute.as_mut() {
            if let Some(duration_to_wait) = bucket.try_take(&current_instant) {
                message_fail.fail_reason = MessageFailType::QuotaExhausted(
                    duration_to_wait,
                    "Exhausted maximum email per minute!".into(),
                );
                return EmailSendingResult::Fail(message_fail);
            }
        }

        // Check max per hour bucket
        if let Some(bucket) = self.bucket_hour.as_mut() {
            if let Some(duration_to_wait) = bucket.try_take(&current_instant) {
                message_fail.fail_reason = MessageFailType::QuotaExhausted(
                    duration_to_wait,
                    "Exhausted maximum email per minute!".into(),
                );
                return EmailSendingResult::Fail(message_fail);
            }
        }

        // Check max per day bucket
        if let Some(bucket) = self.bucket_day.as_mut() {
            if let Some(duration_to_wait) = bucket.try_take(&current_instant) {
                message_fail.fail_reason = MessageFailType::QuotaExhausted(
                    duration_to_wait,
                    "Exhausted maximum email per minute!".into(),
                );
                return EmailSendingResult::Fail(message_fail);
            }
        }

        let from_address;

        match (&draft.email_from).parse::<Address>() {
            Err(e) => {
                message_fail.fail_reason = MessageFailType::BadDraft(e.to_string());
                return EmailSendingResult::Fail(message_fail);
            }
            Ok(address) => from_address = Mailbox::new(draft.email_from_name.clone(), address),
        }

        let to_address;

        match (&draft.email_to).parse::<Address>() {
            Err(e) => {
                message_fail.fail_reason = MessageFailType::BadDraft(e.to_string());
                return EmailSendingResult::Fail(message_fail);
            }
            Ok(address) => to_address = Mailbox::new(draft.email_to_name.clone(), address),
        }

        let email_builder =
            Email::builder().from(from_address).to(to_address).subject(draft.subject);
        let email;

        match draft.body_type {
            MessageDraftBodyType::Ascii => match email_builder.body(draft.body) {
                Err(e) => {
                    message_fail.fail_reason = MessageFailType::BadDraft(e.to_string());
                    return EmailSendingResult::Fail(message_fail);
                }
                Ok(valid_email) => email = valid_email,
            },
            MessageDraftBodyType::Html => {
                let body = SinglePart::builder().header(ContentType::html()).body(draft.body);

                match email_builder.singlepart(body) {
                    Err(e) => {
                        message_fail.fail_reason = MessageFailType::BadDraft(e.to_string());
                        return EmailSendingResult::Fail(message_fail);
                    }
                    Ok(valid_email) => email = valid_email,
                }
            }
        }

        match self.transport.send(email).await {
            Err(e) => {
                message_fail.fail_reason = MessageFailType::Other(e.to_string());
                EmailSendingResult::Fail(message_fail)
            }
            Ok(_) => EmailSendingResult::Sent(MessageSent::new(
                origin_offset,
                service_instance_name,
                draft.id,
            )),
        }
    }
}
