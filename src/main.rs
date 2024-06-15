use send_email::*;

fn main() {
    let sender_info = SenderInfo::new_passwd_from_file(
        "harryhan912@gmail.com", // credential_username
        ".password.toml",        // file_path to password
        "Harry Han",             // sender_name
        SmtpServer::Gmail,       // provider
    );

    let message = EmailInfo::new(
        "Hi",                           // subject
        "Hello, this is a test email.", // body
        false,                          // is_html
        vec!["pic.jpg", "Cargo.toml"],  // path to attachments
    );

    let recipients = vec![RecipientInfo::new(
        "Harry",                // name
        "y.han@joblist.org.uk", // email
    )];

    send_email(&sender_info, &message, &recipients).unwrap();
}
