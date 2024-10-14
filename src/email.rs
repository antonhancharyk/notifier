// use lettre::transport::smtp::authentication::Credentials;
// use lettre::{Message, SmtpTransport, Transport};
// use std::env;

// fn main() {
//     // let my_env_var = env::var("MY_ENV_VAR").expect("MY_ENV_VAR not found");
//     let email = Message::builder()
//         .from("you@example.com".parse().unwrap())
//         .to("recipient@example.com".parse().unwrap())
//         .subject("Test email from Rust")
//         .body(String::from("Hello, this is a test email from Rust!"))
//         .unwrap();

//     let creds = Credentials::new("your_username".to_string(), "your_password".to_string());

//     let mailer = SmtpTransport::relay("smtp.gmail.com")
//         .unwrap()
//         .credentials(creds)
//         .build();

//     match mailer.send(&email) {
//         Ok(_) => println!("Email sent successfully!"),
//         Err(e) => eprintln!("Could not send email: {:?}", e),
//     }
// }
