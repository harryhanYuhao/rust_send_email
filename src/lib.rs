//! Simple rust email sender.
//! This library is mostly a wrapper for lettre crate.
//! See examples and documentations on https://github.com/harryhanYuhao/rust_send_email
#![warn(missing_docs)]
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MultiPart};
use lettre::{Message, SmtpTransport, Transport};

mod sender;
mod recipient;
mod email;

pub use sender::SmtpServer;
pub use sender::Sender;
pub use recipient::{Recipient, Category};
pub use email::EmailContent;

/// Send email, providing the sender, email content, and recipients.
pub fn send_email(
    sender: &Sender,
    content: &EmailContent,
    recipient: &[Recipient],
) -> Result<(), Box<dyn std::error::Error>> {
    // Create Sender info
    let creds = sender.get_credentials();
    let send_mailbox = Mailbox::new(sender.get_name(), sender.get_address());
    let reply_addr = sender.get_reply_address();
    let smtp_server = sender.get_smtp_server();

    // Content type
    let content_type = match content.is_html {
        true => ContentType::TEXT_HTML,
        false => ContentType::TEXT_PLAIN,
    };

    // add body and attachments
    let mut multipart = MultiPart::mixed().singlepart(
        lettre::message::SinglePart::builder()
            .header(content_type)
            .body(String::from(content.content.to_owned())),
    );
    for i in content.attachments.iter() {
        multipart = multipart.singlepart(i.clone());
    }

    // configure to send to multiple recipients
    let mut email = Message::builder();
    for i in recipient {
        match i.category {
            Category::To => email = email.to(Mailbox::new(i.get_name(), i.get_address())),
            Category::Cc => email = email.cc(Mailbox::new(i.get_name(), i.get_address())),
            Category::Bcc => email = email.bcc(Mailbox::new(i.get_name(), i.get_address())),
        }
    }

    let email = email
        .from(send_mailbox)
        .reply_to(reply_addr.into())
        .subject(content.subject.to_owned())
        .multipart(multipart)
        .unwrap();

    // Open a secure connection to the SMTP server using STARTTLS
    let mailer = SmtpTransport::starttls_relay(smtp_server)
        .unwrap() // Unwrap the Result, panics in case of error
        .credentials(creds) // Provide the credentials to the transport
        .build(); // Construct the transport

    // Attempt to send the email via the SMTP transport
    mailer.send(&email)?;
    Ok(())
}
