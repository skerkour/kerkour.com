// use lettre::{
//     transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
//     Tokio1Executor,
// };

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let smtp_credentials =
//         Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

//     let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.email.com")?
//         .credentials(smtp_credentials)
//         .build();

//     let from = "Hello World <hello@world.com>";
//     let to = "42 <42@42.com>";
//     let subject = "Hello World";
//     let body = "<h1>Hello World</h1>".to_string();

//     send_email_smtp(&mailer, from, to, subject, body).await
// }

// async fn send_email_smtp(
//     mailer: &AsyncSmtpTransport<Tokio1Executor>,
//     from: &str,
//     to: &str,
//     subject: &str,
//     body: String,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let email = Message::builder()
//         .from(from.parse()?)
//         .to(to.parse()?)
//         .subject(subject)
//         .body(body.to_string())?;

//     mailer.send(email).await?;

//     Ok(())
// }

////////////////////////////////////////////////////////////////////////
use lettre::Message;
use rusoto_ses::{RawMessage, SendRawEmailRequest, Ses, SesClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ses_client = SesClient::new(rusoto_core::Region::UsEast1);

    let from = "Hello World <hello@world.com>";
    let to = "42 <42@42.com>";
    let subject = "Hello World";
    let body = "<h1>Hello World</h1>".to_string();

    send_email_ses(&ses_client, from, to, subject, body).await
}

async fn send_email_ses(
    ses_client: &SesClient,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    let raw_email = email.formatted();

    let ses_request = SendRawEmailRequest {
        raw_message: RawMessage {
            data: base64::encode(raw_email).into(),
        },
        ..Default::default()
    };

    ses_client.send_raw_email(ses_request).await?;

    Ok(())
}
