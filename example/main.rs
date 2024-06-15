// cargo add send_email
use send_email::*;

fn main() {
    let sender_info = Sender::new_passwd_from_file(
        "example@gmail.com", // credential_username
        ".password.toml",    // file_path to password
        "Eric Elon",         // sender_name
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

    send_email(&sender_info, &message, &recipients).unwrap();
}
