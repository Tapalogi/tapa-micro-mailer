#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod messages;
mod utils;

pub use log::{debug, error, info, log, warn};
pub use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

use config::MailerConfig;

#[tokio::main]
async fn main() -> IOResult<()> {
    let runtime_config = MailerConfig::load_from_env()?;

    Ok(())
}
