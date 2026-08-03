# Simple Send Email Client in Rust

This library provides a simple api to send email via SMTP. This api is largely a wrapper for `lettre` crate, which can be found [here](https://github.com/lettre/lettre).

## Quick Start 

To send a email, provide two structs `Sender`, `Email`, and a vector of `recipient` to `send_email` function:

```rust
// cargo add send_email
use send_email::*;

fn main() {
    let sender_info = Sender::new_passwd_from_file(
        "example@gmail.com", // credential_username
        ".password.toml",    // file_path to password
        "Eric Elon",         // sender_name
        SmtpServer::Gmail,   // provider
        "example@gmail.com", // reply address. (Must be provided)
    );

    let message = EmailContent::new(
        "Hi",                           // subject
        "Hello, this is a test email.", // body
        false,                          // is_html
        vec!["pic.jpg", "Cargo.toml"],  // path to attachments. Leave an empty vec 
                                        // if there are no attachements
    );

    let recipients = vec![
        Recipient::new(
            "Esther Frank",      // name
            "example@gmail.com", // email
            Category::To,        // category. can be To, Cc, or Bcc
        ),
        Recipient::new(
            "", // name, leave empty if not needed
            "example@outlook.com",
            Category::Cc, // Category.
        ),
    ];

    send_email(&sender_info, &message, &recipients).unwrap();
}
```

Password can be stored in toml file and the program can read from it securely: 

```rust
// main.rs
let sender = Sender::new_passwd_from_file(
    "example.com",           // credential_username
    ".password.toml",        // file_path to password
    "Francis Waverley",      // sender_name
    SmtpServer::Gmail,       // provider
);
```

```toml
# .password.toml
password = "PASSWORD"
```
Sending an email is easy as this. 

## Authentication  

Most modern email providers use Multi-Factor authentications, which our program obviously do not support. Fortunately there are some walk arounds.

### Gmail

A google app password is needed required for sending gmails.
Google app password is a 16-letter automatically generated password that is different from the password you created for your google account. 
Get your google app password [here](https://myaccount.google.com/apppasswords), and use it for authentication.

## Junk

The emails sent by this program have a high chance to be classified as junk by many email clients. 
To minimize this chance, fill in all the information as detailed as possible, and use paragraph styled message body.