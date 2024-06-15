use send_email::{Category, *};

fn main() {
    let sender_info = Sender::new_passwd_from_file(
        "harryhan912@gmail.com", // credential_username
        ".password.toml",        // file_path to password
        "Harry Han",             // sender_name
        SmtpServer::Gmail,       // provider
        "harryhan912@gmail.com", // reply_addr
    );

    let message = Email::new(
        "Hi",                           // subject
        "Hello, this is a test email.", // body
        false,                          // is_html
        vec!["pic.jpg", "Cargo.toml"],  // path to attachments
    );

    let recipients = vec![
        Recipient::new(
            "Harry",                // name
            "y.han@joblist.org.uk", // email
            Category::To,           // category
        ),
        Recipient::new(
            "", // name
            "s2162783@ed.ac.uk",
            Category::Cc, // Category
        ),
    ];

    send_email(&sender_info, &message, &recipients).unwrap();
}
