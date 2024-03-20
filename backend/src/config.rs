use std::env;

use crate::{service::email, Error};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_uri: String,
    pub listen_address: String,
    pub session_key: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from: String,
    pub app_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_uri: String::default(),
            listen_address: String::from("127.0.0.1:3000"),
            session_key: String::default(),
            smtp_host: String::default(),
            smtp_port: 587,
            smtp_user: String::default(),
            smtp_pass: String::default(),
            smtp_from: String::default(),
            app_url: String::from("http://localhost:3000"),
        }
    }
}

impl Config {
    pub fn load_from_env() -> Result<Config, Error> {
        Ok(Config {
            database_uri: get_env("RTJAM_DATABASE_URI")?,
            listen_address: get_env("RTJAM_LISTEN_ADDRESS")
                .unwrap_or_else(|_| Config::default().listen_address),
            session_key: get_env("RTJAM_SESSION_KEY")?,
            smtp_host: get_env("RTJAM_SMTP_HOST")?,
            smtp_port: get_env("RTJAM_SMTP_PORT")?.parse::<u16>().map_err(|e| {
                Error::InvalidConfig {
                    detail: e.to_string(),
                }
            })?,
            smtp_user: get_env("RTJAM_SMTP_USER")?,
            smtp_pass: get_env("RTJAM_SMTP_PASSWORD")?,
            smtp_from: get_env("RTJAM_SMTP_FROM")?,
            app_url: get_env("RTJAM_APP_URL").unwrap_or_else(|_| Config::default().app_url),
        })
    }
}

fn get_env(name: &'static str) -> Result<String, Error> {
    env::var(name).map_err(|e| Error::InvalidConfig {
        detail: format!("{}: {:?}", name.to_string(), e.to_string()),
    })
}
impl From<Config> for email::Config {
    fn from(
        Config {
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_pass,
            smtp_from,
            app_url,
            ..
        }: Config,
    ) -> Self {
        Self {
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_pass,
            smtp_from,
            app_url,
        }
    }
}
