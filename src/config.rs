pub struct MailerConfig {
    pub queue_subject: String,
    pub queue_group: String,
    pub queue_hostname: String,
    pub queue_instance: Option<String>,
    pub queue_user: Option<String>,
    pub queue_pass: Option<String>,
    pub max_email_per_day: u32,
    pub max_email_per_second: u16,
}
