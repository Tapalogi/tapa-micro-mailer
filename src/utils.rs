use env_logger::builder as log_builder;
use regex::Regex;
use std::env::{set_var, var};
use uuid::Uuid;

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

pub fn get_new_uuid_v4() -> Uuid {
    Uuid::new_v4()
}

pub fn init_logger() {
    if var(RUST_LOG).is_err() {
        #[cfg(debug_assertions)]
        set_var(RUST_LOG, "debug");
        #[cfg(not(debug_assertions))]
        set_var(RUST_LOG, "info");
    }

    log_builder().default_format().format_timestamp_nanos().format_indent(Some(2)).init();
}
