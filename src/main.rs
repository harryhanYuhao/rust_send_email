use send_email::*;

fn main() {
    let sender_info = SenderInfo::new_passwd_from_file(
        "harryhan912@gmail.com",
        ".password.toml",
        "Harry Han",
        SmtpServer::Gmail,
    );

    let message = EmailInfo::plain_messasge("Hi", "Hello, this is a test email.");

    let recipients = vec![
        RecipientInfo::new("Harry", "harryhan912@gmail.com"),
        RecipientInfo::new("Harry", "s2162783@ed.ac.uk"),
        RecipientInfo::new("Harry", "y.han@joblist.org.uk"),
    ];


    send_email(&sender_info, &message, &recipients).unwrap();
}
