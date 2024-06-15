// cargo add lettre
use lettre::address::Address;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Password {
    password: String,
}

pub enum SmtpServer {
    Gmail,
    FastMail,
    Custom(String),
}

pub struct EmailInfo {
    subject: String,
    content: String,
    is_html: bool,
}

pub struct RecipientInfo {
    name: Option<String>,
    email: String,
}

impl RecipientInfo {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            email: email.to_owned(),
        }
    }

    pub fn address(email: &str) -> Self {
        Self {
            name: None,
            email: email.to_owned(),
        }
    }
}

impl EmailInfo {
    pub fn plain_messasge(subject: &str, content: &str) -> Self {
        Self {
            subject: subject.to_owned(),
            content: content.to_owned(),
            is_html: false,
        }
    }
}

pub struct SenderInfo {
    credential_username: String,
    credential_password: String,
    sender_addr: String,
    sender_name: String,
    reply_addr: String,
    provider: SmtpServer,
}

impl SenderInfo {
    pub fn new(
        credential_username: &str,
        credential_password: &str,
        sender_name: &str,
        provider: SmtpServer,
    ) -> Self {
        Self {
            credential_username: credential_username.to_owned(),
            credential_password: credential_password.to_owned(),
            sender_addr: credential_username.to_owned(),
            sender_name: sender_name.to_owned(),
            reply_addr: credential_username.to_owned(),
            provider,
        }
    }

    pub fn new_passwd_from_file(
        credential_username: &str,
        file_path: &str,
        sender_name: &str,
        provider: SmtpServer,
    ) -> Self {
        let file = fs::read_to_string(file_path).expect(&format!(
            "Failed to Execute new_passwd_from_file:\nUnable to read file: {}",
            file_path
        ));
        let password: Password = toml::from_str(&file).unwrap();
        let password = password.password;
        Self {
            credential_username: credential_username.to_owned(),
            credential_password: password,
            sender_addr: credential_username.to_owned(),
            sender_name: sender_name.to_owned(),
            reply_addr: credential_username.to_owned(),
            provider,
        }
    }
}

pub fn send_email(
    sender_info: &SenderInfo,
    email_info: &EmailInfo,
    recipient: &[RecipientInfo],
) -> Result<(), Box<dyn std::error::Error>> {
    // Create Sender info
    let creds = Credentials::new(
        sender_info.credential_username.to_owned(),
        sender_info.credential_password.to_owned(),
    );
    let source_address = sender_info.sender_addr.parse::<Address>()?;
    let name: Option<String> = Some(sender_info.sender_name.to_owned());
    let send_mailbox = Mailbox::new(name, source_address);
    let reply_addr = sender_info.reply_addr.parse::<Address>()?;

    let smtp_server = match &sender_info.provider {
        SmtpServer::Gmail => "smtp.gmail.com",
        SmtpServer::FastMail => "smtp.fastmail.com",
        SmtpServer::Custom(s) => s.as_str(),
    };

    // Content type
    let content_type = match email_info.is_html {
        true => ContentType::TEXT_HTML,
        false => ContentType::TEXT_PLAIN,
    };

    // Attachements, if any
    let filename = "a.txt".to_string();
    let filebody = fs::read("a.txt").unwrap();
    let content_type = ContentType::TEXT_PLAIN;
    let attachment = Attachment::new(filename).body(filebody, content_type.clone());
    // Create recipient mailboxes
    let mut mail_boxes: Vec<Mailbox> = Vec::new();
    for i in recipient {
        let email_addr = i.email.parse::<Address>()?;
        mail_boxes.push(Mailbox::new(i.name.clone(), email_addr));
    }

    // Create recipient mailboxes
    let mut mail_boxes: Vec<Mailbox> = Vec::new();
    for i in recipient {
        let email_addr = i.email.parse::<Address>()?;
        mail_boxes.push(Mailbox::new(i.name.clone(), email_addr));
    }

    // configure to send to multiple recipients
    let mut email = Message::builder();
    for i in mail_boxes {
        email = email.to(i);
    }
    // add the rest of info
    let email = email
        .from(send_mailbox)
        .reply_to(reply_addr.into())
        .subject(email_info.subject.to_owned())
        .header(content_type)
        .body(String::from(email_info.content.to_owned()))
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
