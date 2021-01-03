use crate::config::SmtpConfig;
use crate::messages::{
    MessageDraft, MessageDraftBodyType, MessageFail, MessageFailType, MessageSent,
};
use crate::{IOError, IOErrorKind, IOResult};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Message as Email, Tokio02Connector, Tokio02Transport};
use tokio::time::{Duration, Instant};

const MINUTE_IN_SECONDS: u64 = 60;
const HOUR_IN_SECONDS: u64 = 60 * MINUTE_IN_SECONDS;
const DAY_IN_SECONDS: u64 = 24 * HOUR_IN_SECONDS;

pub enum EmailSendingResult {
    Fail(MessageFail),
    Sent(MessageSent),
}

pub struct ResettableBucket {
    bucket_size: usize,
    bucket_interval: Duration,
    current_bucket_size: usize,
    last_reset: Instant,
}

impl ResettableBucket {
    fn new(bucket_size: usize, bucket_interval: Duration) -> Self {
        Self {
            bucket_size,
            bucket_interval,
            last_reset: Instant::now(),
            current_bucket_size: bucket_size,
        }
    }

    fn try_take(&mut self, current_instant: Instant) -> bool {
        if current_instant.duration_since(self.last_reset) >= self.bucket_interval {
            self.current_bucket_size = self.bucket_size;
            self.last_reset = Instant::now();
        }

        if self.current_bucket_size > 0 {
            self.current_bucket_size -= 1;

            true
        } else {
            false
        }
    }
}

pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio02Connector>,
    config: SmtpConfig,
    bucket_second: Option<ResettableBucket>,
    bucket_minute: Option<ResettableBucket>,
    bucket_hour: Option<ResettableBucket>,
    bucket_day: Option<ResettableBucket>,
}

impl Mailer {
    pub fn new(smtp_config: SmtpConfig) -> IOResult<Self> {
        let creds = Credentials::new(smtp_config.user.clone(), smtp_config.pass.clone());
        let mailer_build_result = if smtp_config.use_starttls {
            AsyncSmtpTransport::<Tokio02Connector>::starttls_relay(&smtp_config.host)
        } else {
            AsyncSmtpTransport::<Tokio02Connector>::relay(&smtp_config.host)
        };

        match mailer_build_result {
            Err(build_error) => Err(IOError::new(IOErrorKind::Other, build_error.to_string())),
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
                    config: smtp_config,
                })
            }
        }
    }

    async fn compose_and_send(draft: &MessageDraft) -> IOResult<EmailSendingResult> {
        todo!("Implement send mail!");
    }
}
