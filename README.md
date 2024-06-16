# Simple Send Email Client in Rust

This library provides a simple api to send email via SMTP. This api is largely a wrapper for `lettre` crate.

## Quick Start 

To send a email, provide two structs `Sender`, `Email`, and a vector of `recipient` to `send_email` function:

```rust 
use send_email::*;

fn main() {
    let sender = Sender::new(
        "example@gmail.com", // credential_username
        "PASSWORD",    // password
        "Eric Elon",         // sender name. Leave empty if not needed
        SmtpServer::Gmail,   // provider
        "example@gmail.com", // reply_addr
    );

    let message = EmailContent::new(
        "Hi",                           // subject
        "Hello, this is a test email.", // body
        false,                          // is_html
        vec!["pic.jpg", "Cargo.toml"],  // path to attachments
    );

    let recipients = vec![
        Recipient::new(
            "Esther Frank",      // name
            "example@gmail.com", // email
            Category::To,        // category. can be To, Cc, or Bcc
        ),
        Recipient::new(
            "", // name leave empty if not needed
            "example@outlook.com",
            Category::Cc, // Category.
        ),
    ];

    send_email(&sender, &message, &recipients).unwrap();
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
