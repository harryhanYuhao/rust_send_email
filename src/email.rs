use lettre::message::header::ContentType;
use lettre::message::{Attachment, SinglePart};
use std::fs;

/// Represents the email content.
/// You should create an email object by using its constructors.
#[allow(missing_docs)]
pub struct EmailContent {
    pub subject: String,
    pub content: String,
    pub is_html: bool,
    pub attachments: Vec<SinglePart>,
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

    /// Create a complete email.
    /// If no attachment is needed, leave the attachment vector empty.
    pub fn new(subject: &str, content: &str, is_html: bool, attachment_paths: Vec<&str>) -> Self {
        let attachments = attachment_paths
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

    /// Create a complete email from a file.
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
