use lettre::address::Address;

/// This enum represents if the recipient is direct recipient, carbon copy, or blind carbon copy.
/// The values are Category::To, Category::Cc, and Category::Bcc.
#[allow(missing_docs)]
pub enum Category {
    To,
    Cc,
    Bcc,
}

/// The object represents the recipients and used in send_email function.
#[allow(missing_docs)]
pub struct Recipient {
    pub name: String,  // if no name, leave it empty
    pub email: String,
    pub category: Category,
}

impl Recipient {
    /// Create a new recipient.
    /// # Example
    /// ```
    /// let recipient = Recipient::new("Jack Francis", "example@gmail.com", Category::To);
    /// ```
    /// There are only three kinds of [`Category`]: `To`, `Cc`, and `Bcc`.
    /// You can provide empty name with ""
    pub fn new(name: &str, email: &str, category: Category) -> Self {
        Self {
            name: name.to_owned(),
            email: email.to_owned(),
            category,
        }
    }

    /// short hand for creating a recipient with only email address.
    pub fn address(email: &str) -> Self {
        Self {
            name: "".to_owned(),
            email: email.to_owned(),
            category: Category::To,
        }
    }

    pub(crate) fn get_address(&self) -> Address {
        self.email
            .parse::<Address>()
            .expect("Invalid email address")
    }

    pub(crate) fn get_name(&self) -> Option<String> {
        if self.name.is_empty() {
            None
        } else {
            Some(self.name.clone())
        }
    }
}
