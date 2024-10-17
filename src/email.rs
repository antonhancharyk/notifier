use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub async fn send(body: String) {
    let from_mail: String = env::var("FROM_MAIL").unwrap();
    let host_mail: String = env::var("HOST_MAIL").unwrap();
    let password_mail: String = env::var("PASSWORD_MAIL").unwrap();

    let email = Message::builder()
        .from(from_mail.parse().unwrap())
        .to("ant.goncharik.development@gmail.com".parse().unwrap())
        .subject("Message from RabbitMQ")
        .body(body)
        .unwrap();

    let creds = Credentials::new(from_mail, password_mail);

    let mailer = SmtpTransport::relay(&host_mail)
        .unwrap()
        .credentials(creds)
        .build();

    _ = mailer.send(&email);
}
