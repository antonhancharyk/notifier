use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::Value;
use std::env;

pub async fn send(body: String) {
    let from_mail: &str = &env::var("FROM_MAIL").unwrap();
    let host_mail: String = env::var("HOST_MAIL").unwrap();
    let password_mail: String = env::var("PASSWORD_MAIL").unwrap();

    let parsed_json: Value = serde_json::from_str(&body).unwrap();

    let subject = parsed_json["subject"].as_str().unwrap_or("").to_string();
    let msg = parsed_json["msg"].as_str().unwrap_or("").to_string();

    let emails = ["ant.goncharik.development@gmail.com"];

    let value = Credentials::new(from_mail.to_string(), password_mail);

    for email in emails {
        let email_data = Message::builder()
            .from(from_mail.parse().unwrap())
            .to(email.parse().unwrap())
            .subject(&subject.clone())
            .body(msg.clone())
            .unwrap();

        let creds = value.clone();

        let mailer = SmtpTransport::relay(&host_mail)
            .unwrap()
            .credentials(creds)
            .build();

        _ = mailer.send(&email_data);
    }
}
