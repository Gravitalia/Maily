use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use anyhow::Result;

pub fn send_mail(to: String, subject: String, content: String) -> Result<()> {
    let email = Message::builder()
        .from("Gravitalia <contact@gravitalia.com>".parse()?)
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(content)?;

    let creds = Credentials::new(dotenv::var("USERNAME")?, dotenv::var("PASSWORD")?);

    let mailer = SmtpTransport::relay(dotenv::var("SERVER")?.as_str())?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}