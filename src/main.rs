#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod messages;
mod utils;

pub use log::{debug, error, info, log, warn};
pub use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

use config::MailerConfig;
use utils::{get_hostname, init_logger};

#[tokio::main]
async fn main() -> IOResult<()> {
    init_logger();

    let runtime_config = MailerConfig::load_from_env()?;
    let service_hostname = get_hostname();

    Ok(())
}
