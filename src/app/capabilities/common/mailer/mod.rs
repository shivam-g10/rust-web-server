use std::{borrow::Borrow, env};

use lettre::{
    message::{header::ContentType, MessageBuilder},
    transport::smtp::authentication::Credentials,
    SmtpTransport, Transport,
};
use poem_openapi::types::Email;

pub enum SendEmailResult {
    Ok(bool),
    Error(String),
}

#[derive(Clone, Debug)]
pub struct MailUser {
    name: Option<String>,
    email: Email,
}
impl MailUser {
    pub fn new(email: Email, name: Option<String>) -> MailUser {
        MailUser { name, email }
    }
    pub fn to_string(&self) -> String {
        match &self.name {
            Some(name) => format!("{} <{}>", name, self.email.to_string()),
            None => format!("<{}>", self.email.to_string()),
        }
    }

    pub fn from_mail_user(mut self, mu: MailUser) {
        self.name = mu.name.to_owned();
        self.email = mu.email.to_owned();
    }
}

#[derive(Debug, Clone)]
pub struct Mailer {
    mailer: SmtpTransport,
    default_from: MailUser,
    default_reply_to: MailUser,
}

impl Mailer {
    pub fn new(
        username: String,
        password: String,
        smtp: String,
        default_from: MailUser,
        default_reply_to: MailUser,
    ) -> Mailer {
        let creds = Credentials::new(username, password);
        let mut transport = SmtpTransport::from_url(smtp.as_str()).unwrap();
        if env::var("RUN_ENV").unwrap_or("DEVELOPMENT".to_string()) == "PRODUCTION" {
            transport = transport.credentials(creds);
        }
        let mailer = transport.build();
        Mailer {
            mailer,
            default_from,
            default_reply_to,
        }
    }

    pub async fn send_email(
        &self,
        subject: String,
        body: String,
        receiver: MailUser,
        sender: Option<MailUser>,
    ) -> SendEmailResult {
        tracing::debug!("send_email");
        let from = self.default_from.borrow();
        match sender {
            Some(sender) => from.clone().from_mail_user(sender),
            None => (),
        };
        let message = MessageBuilder::new()
            .from(from.to_string().as_str().parse().unwrap())
            .reply_to(self.default_reply_to.to_string().as_str().parse().unwrap())
            .to(receiver.to_string().as_str().parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)
            .unwrap();

        match self.mailer.send(&message) {
            Ok(_) => SendEmailResult::Ok(true),
            Err(e) => {
                tracing::error!("{}", e);
                SendEmailResult::Error(e.to_string())
            }
        }
    }
}
