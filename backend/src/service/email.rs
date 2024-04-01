
use askama::Template;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use super::user::User;

#[derive(Clone)]
pub struct Config {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from: String,
    pub app_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            smtp_host: String::from(""),
            smtp_port: 587,
            smtp_user: String::from(""),
            smtp_pass: String::from(""),
            smtp_from: String::from(""),
            app_url: String::from("http://localhost:3000"),
        }
    }
}

#[derive(Clone)]
pub struct Service {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    app_url: String,
    from: String,
}

impl Service {
    pub async fn new(
        Config {
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_pass,
            smtp_from,
            app_url,
        }: Config,
    ) -> Result<Self, lettre::transport::smtp::Error> {
        let credentials = Credentials::new(smtp_user, smtp_pass);

        let transport =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host.to_owned())?
                .port(smtp_port)
                .credentials(credentials)
                .build();

        // transport.test_connection().await?;

        Ok(Self {
            transport,
            app_url,
            from: smtp_from,
        })
    }
}

impl Service {
    pub async fn send_verification_link(
        &self,
        token: &str,
        user: &User,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let link = format!("{}/change-password?token={}", self.app_url, token);
        let subject = format!("Welcome, {}", user.first_name);

        let template = ChangePassword {
            subject: subject.clone(),
            user: user.clone(),
            message:
                String::from("Welcome onboard! Please, click the button below to set a password for your account"),
            link,
        }.render()?;

        let email = Message::builder()
            .to(format!("{} <{}>", user.first_name, user.email)
                .parse()
                .unwrap())
            .reply_to(self.from.parse().unwrap())
            .from(self.from.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(template)?;

        self.transport.send(email).await?;

        Ok(())
    }
    pub async fn send_reset_link(
        &self,
        token: &str,
        user: &User,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let link = format!("{}/change-password?token={}", self.app_url, token);
        let subject = format!("Your reset link, {}", user.first_name);

        let template = ChangePassword {
            subject: subject.clone(),
            user: user.clone(),
            message:
                String::from("A password reset was issued for your account! Please, click the button below to set a new password for your account"),
            link,
        }.render()?;

        let email = Message::builder()
            .to(format!("{} <{}>", user.first_name, user.email)
                .parse()
                .unwrap())
            .reply_to(self.from.parse().unwrap())
            .from(self.from.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(template)?;

        self.transport.send(email).await?;

        Ok(())
    }
}

#[derive(Template)]
#[template(path = "change-password.html")]
struct ChangePassword {
    subject: String,
    user: User,
    message: String,
    link: String,
}
