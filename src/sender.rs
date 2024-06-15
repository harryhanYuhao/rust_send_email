#![allow(missing_docs)]
use lettre::address::Address;
use lettre::transport::smtp::authentication::Credentials;

use serde::Deserialize;
use std::fs;

/// Represents the SMTP server.
/// SMTP server is the server that sends the email
#[allow(missing_docs)]
pub enum SmtpServer {
    Gmail,
    FastMail,
    Outlook,
    Yahoo,
    Custom(String),
}

// For deserializing password from file (serde)
#[derive(Deserialize)]
struct Password {
    password: String,
}

fn credential_from_user_and_passwd(username: &str, passwd: &str) -> Credentials {
    Credentials::new(username.to_owned(), passwd.to_owned())
}

/// Represents the sender information.
/// You should create a sender object by using its constructors.
/// Example
/// ```rust
/// let sender_info = Sender::new_passwd_from_file(
///    "example@gmail.com", // credential_username
///    ".password.toml",    // file_path to Password
///    "Jack Ma",         // sender_name
///    SmtpServer::Gmail,   // provider
///    );
/// ```
pub struct Sender {
    credential: Credentials,
    address: Address,
    sender_name: String,
    reply_addr: Address,
    provider: SmtpServer,
}

impl Sender {
    pub(crate) fn get_smtp_server(&self) -> &str {
        match &self.provider {
            SmtpServer::Gmail => "smtp.gmail.com",
            SmtpServer::FastMail => "smtp.fastmail.com",
            SmtpServer::Outlook => "smtp-mail.outlook.com",
            SmtpServer::Yahoo => "smtp.mail.yahoo.com",
            SmtpServer::Custom(s) => s.as_str(),
        }
    }

    pub(crate) fn get_credentials(&self) -> Credentials {
        self.credential.clone()
    }

    pub(crate) fn get_address(&self) -> Address {
        self.address.clone()
    }

    pub(crate) fn get_name(&self) -> Option<String> {
        if self.sender_name.is_empty() {
            None
        } else {
            Some(self.sender_name.clone())
        }
    }

    pub(crate) fn get_reply_address(&self) -> Address {
        self.reply_addr.clone()
    }

    pub fn new(
        credential_username: &str,
        credential_password: &str,
        sender_name: &str,
        provider: SmtpServer,
        reply_addr: &str,
    ) -> Self {
        let credential = credential_from_user_and_passwd(credential_username, credential_password);
        Self {
            credential,
            address: credential_username
                .parse::<Address>()
                .expect("Invalid email address"),
            sender_name: sender_name.to_owned(),
            reply_addr: reply_addr
                .parse::<Address>()
                .expect("Invalid email address"),
            provider,
        }
    }

    pub fn new_passwd_from_file(
        credential_username: &str,
        file_path: &str,
        sender_name: &str,
        provider: SmtpServer,
        reply_addr: &str,
    ) -> Self {
        let file = fs::read_to_string(file_path).expect(&format!(
            "Failed to Execute new_passwd_from_file:\nUnable to read file: {}",
            file_path
        ));
        let password: Password = toml::from_str(&file).unwrap();
        let password = password.password;
        let credential = credential_from_user_and_passwd(credential_username, &password);

        Self {
            credential,
            address: credential_username
                .parse::<Address>()
                .expect("Invalid email address"),
            sender_name: sender_name.to_owned(),
            reply_addr: reply_addr
                .parse::<Address>()
                .expect("Invalid email address"),
            provider,
        }
    }
}
