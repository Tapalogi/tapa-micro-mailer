use crate::AnyResult;
use env_logger::builder as log_builder;
use regex::Regex;
use std::env::{set_var, var};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::select as wait_for_any;
use tokio::signal::unix::{signal, SignalKind};

pub(crate) const MINUTE_IN_SECONDS: u64 = 60;
pub(crate) const HOUR_IN_SECONDS: u64 = 60 * MINUTE_IN_SECONDS;
pub(crate) const DAY_IN_SECONDS: u64 = 24 * HOUR_IN_SECONDS;

const REGEX_VALID_EMAIL: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
const RUST_LOG: &str = "RUST_LOG";

pub fn is_valid_email_string(email_string: &str) -> bool {
    Regex::new(REGEX_VALID_EMAIL).unwrap().is_match(email_string)
}

pub fn get_hostname() -> String {
    match hostname::get() {
        Err(_) => "none".into(),
        Ok(instance_hostname) => instance_hostname.to_str().unwrap().into(),
    }
}

pub fn init_logger() {
    if var(RUST_LOG).is_err() {
        #[cfg(debug_assertions)]
        set_var(RUST_LOG, "debug");
        #[cfg(not(debug_assertions))]
        set_var(RUST_LOG, "info");
    }

    log_builder().default_format().format_timestamp_nanos().format_indent(None).init();
}

async fn wait_for_signal(signal_kind: SignalKind) -> AnyResult<()> {
    let mut stream = signal(signal_kind)?;
    stream.recv().await;

    Ok(())
}

pub(crate) async fn wait_for_stop_signals(shutdown_flag: Arc<AtomicBool>) {
    let _ = wait_for_any! {
        res_a = wait_for_signal(SignalKind::interrupt()) => res_a,
        res_b = wait_for_signal(SignalKind::terminate()) => res_b,
    };

    shutdown_flag.store(true, Ordering::Relaxed);
}
