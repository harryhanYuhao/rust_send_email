# Simple Send Email Client in Rust

## Quick Start 

To send a email, just provide three structs to `send_email` function:

```rust 
use send_email::*;

fn main() {
    let sender_info = SenderInfo::new(
        "example@gmail.com",           // credential_username (email address)
        "PASSWORD",              // password
        "Elon Scott",             // sender_name
        SmtpServer::Gmail,       // smtp provider
        "example@gmail.com", // reply_addr

    );

    let message = EmailInfo::new(
        "Hi",                           // subject
        "Hello, this is a test email.", // body
        false,                          // is_html
        vec!["pic.jpg", "Cargo.toml"],  // path to attachments; leave empty if no attachment
    );

    let recipients = vec![RecipientInfo::new(
        "Mike",                // name
        "example@gmail.com", // email
        Category::To,           // category can be To, Cc, Bcc
    )];

    send_email(&sender_info, &message, &recipients).unwrap();
}
```

You can also stores the password in toml file like this and read from it securely: 

```rust
// main.rs
let sender_info = SenderInfo::new_passwd_from_file(
    "example.com", // credential_username
    ".password.toml",        // file_path to password
    "Francis Waverley",             // sender_name
    SmtpServer::Gmail,       // provider
);
```

```toml
# .password.toml
password = "PASSWORD"
```
Sending an email is easy as this. 
