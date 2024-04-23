use std::fmt::{Debug, Display};

#[derive(Default, Debug, Clone)]
/// An application profile.
pub struct Profile {
    /// Profile name
    pub name: String,
    /// Profile description
    pub description: String,
    /// The command or path to the application with arguements
    pub command: String,
    /// Last recorded process ID (0 == not running)
    pub pid: u32,
}

#[derive(Clone, Copy)]
pub enum ValidationError {
    NameTooShort,
}

impl Profile {
    pub fn new<S: Into<String>>(name: S, description: S, command: S) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            command: command.into(),
            pid: 0,
        }
    }

    /// Validates (and truncates when necessary) struct fields.
    ///
    /// - Name: min 3 chars, max 512 chars, truncates automatically
    /// - Description: max 2048 chars, truncates automatically
    pub fn validate(&mut self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        self.name = self.name.chars().take(512).collect();
        self.description = self.description.chars().take(2048).collect();

        // name
        if self.name.len() < 3 {
            errors.push(ValidationError::NameTooShort);
        }

        errors
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NameTooShort => write!(f, "Profile name is too short"),
        }
    }
}

impl Debug for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
