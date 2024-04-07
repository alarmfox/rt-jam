use std::env ;

use crate::service::email;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub smtp_from: String,
    pub app_url: String,
    pub session_key: String,
    pub listen_address: String,
    pub webtransport_address: String,
    pub cert_path: String,
    pub key_path: String
}

impl Config {
    pub fn load_from_env() -> Result<Config, Box<dyn std::error::Error>> {
        Ok(Config {
            database_url: env::var("RTJAM_DATABASE_URL")?,
            listen_address: env::var("RTJAM_LISTEN_ADDRESS")?, 
            session_key: env::var("RTJAM_SESSION_KEY")?,
            smtp_server: env::var("RTJAM_SMTP_HOST")?,
            smtp_port: env::var("RTJAM_SMTP_PORT")?.parse::<u16>()?,
            smtp_user: env::var("RTJAM_SMTP_USER")?,
            smtp_password: env::var("RTJAM_SMTP_PASSWORD")?,
            smtp_from: env::var("RTJAM_SMTP_FROM")?,
            app_url: env::var("RTJAM_APP_URL")?,
            webtransport_address: env::var("RTJAM_WEBTRANSPORT_ADDRESS")?,
            cert_path: env::var("RTJAM_CERT_PATH")?,
            key_path: env::var("RTJAM_KEY_PATH")?,
        })
    }
}

impl From<Config> for email::Config {
    fn from(
        Config {
            smtp_server,
            smtp_port,
            smtp_user,
            smtp_password,
            smtp_from,
            app_url,
            ..
        }: Config,
    ) -> Self {
        Self {
            smtp_host: smtp_server,
            smtp_port,
            smtp_user,
            smtp_pass: smtp_password,
            smtp_from,
            app_url,
        }
    }
}
