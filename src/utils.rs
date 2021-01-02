use regex::Regex;

const REGEX_VALID_EMAIL: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

pub fn is_valid_email_string(email_string: &str) -> bool {
    Regex::new(REGEX_VALID_EMAIL).unwrap().is_match(email_string)
}