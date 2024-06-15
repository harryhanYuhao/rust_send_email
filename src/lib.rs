//! Simple rust email sender.

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
pub use email::Email;

/// To send email, we need three things: 
/// sender information, receiver information, and email content,
/// which are represented by [`Sender`], [`Recipient`], and [`Email`] respectively.
/// Example: 
/// ```rust
///let sender_info = SenderInfo::new(
///    "example@gmail.com",           // credential_username (email address)
///    "PASSWORD",              // password
///    "Harry Han",             // sender_name
///    SmtpServer::Gmail,       // smtp provider
///);
///
///let message = EmailInfo::new(
///    "Hi",                           // subject
///    "Hello, this is a test email.", // body
///    false,                          // is_html
///    vec!["pic.jpg", "Cargo.toml"],  // path to attachments; leave empty if no attachment
///);
///
///let recipients = vec![RecipientInfo::new(
///    "Harry",                // name
///    "y.han@joblist.org.uk", // email
///)];
///
///send_email(&sender_info, &message, &recipients).unwrap();
/// ```
/// You can send to more recipients by adding more [`Recipient`] to the vector. You can avoid
/// sending attachments by leaving the attachemnt vector empty.
///
/// To furthur modify the email content, see more constructors for [`Email`].
pub fn send_email(
    sender: &Sender,
    email_info: &Email,
    recipient: &[Recipient],
) -> Result<(), Box<dyn std::error::Error>> {
    // Create Sender info
    let creds = sender.get_credentials();
    let send_mailbox = Mailbox::new(sender.get_name(), sender.get_address());
    let reply_addr = sender.get_reply_address();
    let smtp_server = sender.get_smtp_server();

    // Content type
    let content_type = match email_info.is_html {
        true => ContentType::TEXT_HTML,
        false => ContentType::TEXT_PLAIN,
    };

    // add body and attachments
    let mut multipart = MultiPart::mixed().singlepart(
        lettre::message::SinglePart::builder()
            .header(content_type)
            .body(String::from(email_info.content.to_owned())),
    );
    for i in email_info.attachments.iter() {
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
        .subject(email_info.subject.to_owned())
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
