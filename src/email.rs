use lettre::message::header::ContentType;
use lettre::message::{Attachment, SinglePart};
use std::fs;

/// Represents the email content.
/// You should create an email object by using its constructors.
pub struct EmailContent {
    /// subject of email 
    pub subject: String,
    /// content of email represented in plain text
    pub content: String,
    /// is_html shall be true if the content is in HTML format
    pub is_html: bool,
    pub(crate) attachments: Vec<SinglePart>,
}

fn convert_path_to_attachment(path: &str) -> SinglePart {
    let filename = path.to_string();
    let filebody = fs::read(path).expect(&format!(
        "Failed to Execute path_to_attachment:\nUnable to read file: {}",
        path
    ));
    let content_type_att = ContentType::TEXT_PLAIN;
    Attachment::new(filename).body(filebody, content_type_att)
}

impl EmailContent {
    /// Shortcut for creating a plain text email.
    pub fn plain_messasge(subject: &str, content: &str) -> Self {
        Self {
            subject: subject.to_owned(),
            content: content.to_owned(),
            is_html: false,
            attachments: Vec::new(),
        }
    }

    /// Create a complete email object for use in send_email function 
    /// path_to_attachments is a Vec<&str>, holding relative paths to the attachments.
    /// If no attachment leave the vec empty.
    pub fn new(subject: &str, content: &str, is_html: bool, path_to_attachments: Vec<&str>) -> Self {
        let attachments = path_to_attachments
            .iter()
            .map(|x| convert_path_to_attachment(x))
            .collect::<Vec<SinglePart>>();
        Self {
            subject: subject.to_owned(),
            content: content.to_owned(),
            is_html,
            attachments,
        }
    }

    /// Create an email object for use in send_email function
    /// The body of the email is read from a file. 
    pub fn new_email_body_from_file(
        subject: &str,
        body_file_path: &str,
        is_html: bool,
        attachments: Vec<&str>,
    ) -> Self {
        let attachments = attachments
            .iter()
            .map(|x| convert_path_to_attachment(x))
            .collect::<Vec<SinglePart>>();

        let content = fs::read_to_string(body_file_path).expect(&format!(
            "Failed to Execute new_body_from_file:\nUnable to read file: {}",
            body_file_path
        ));

        Self {
            subject: subject.to_owned(),
            content,
            is_html,
            attachments,
        }
    }
}
