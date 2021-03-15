+++
date = 2021-03-15T08:00:00+01:00
title = "How to send emails with Rust"
type = "post"
tags = ["tutorial", "rust", "web dev", "email"]
authors = ["Sylvain Kerkour"]
url = "/blog/send-email-in-rust"

draft = true

[extra]
lang = "en"
+++

Sending emails in Rust can be achieved in two ways: either by using an SMTP server or by using a third-party service with an API such as AWS SES or Sendgrid.

{{< bhr_banner >}}

## SMTP

SMTP is the standard protocol for sending emails. Thus, it's the most portable way to send emails as every provider accepts it.

**Cargo.toml**
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
lettre = "0.10.0-beta.2"
```

**main.rs**
```rust
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let smtp_credentials =
        Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.email.com")?
        .credentials(smtp_credentials)
        .build();

    let from = "Hello World <hello@world.com>";
    let to = "42 <42@42.com>";
    let subject = "Hello World";
    let body = "<h1>Hello World</h1>".to_string();

    send_email_smtp(&mailer, from, to, subject, body).await
}

async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
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

    mailer.send(email).await?;

    Ok(())
}
```


## AWS SES

Unfortunately, using SMTP may not be possible all the time because the port might be blocked to avoid spam or because
your email provider imposes a limit on the messages' throughout sent through SMTP.

To send an email with SES, we first need to create and format the email appropriately and then send it through its API.

We need to use the `lettre` crate with only the `builder` feature enabled (to reduces bloat) to format the email and send it with the `rusoto` crate which is the unofficial AWS SDK.


**Cargo.toml**
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
lettre = { version = "0.10.0-beta.2", default-features = false, features = ["builder"] }
rusoto_core = { version = "0.46", default-features = false, features = ["rustls"] }
rusoto_ses = { version = "0.46", default-features = false, features = ["rustls"] }
base64 = "0.13"
```

**main.rs**
```rust
use lettre::Message;
use rusoto_ses::{RawMessage, SendRawEmailRequest, Ses, SesClient};
use std::env;

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
```

If you want to learn more from real-world Rust experience, I'm writing a book (available in early access). Here is a coupon to save 10€ on the book: [https://academy.kerkour.com/black-hat-rust?coupon=BLOG](https://academy.kerkour.com/black-hat-rust?coupon=BLOG)