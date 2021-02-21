mod config;
mod mailer;
mod messages;
mod utils;

pub use log::{debug, error, info, log, warn};

use anyhow::{anyhow as anyerror, Result as AnyResult};
use config::{MQConfig, MailerConfig};
use mailer::{EmailSendingResult, Mailer};
use messages::{MessageDraft, MessageFail, MessageFailType};
use tapa_cgloop_nats::{CGLoop, ProcessResult};
use tokio::time::{delay_for, Duration};
use utils::init_logger;

#[tokio::main]
async fn main() -> AnyResult<()> {
    init_logger();

    let runtime_config = MailerConfig::load_from_env()?;
    info!("Mailer Config:\n{:#?}", runtime_config);

    Ok(())
}
