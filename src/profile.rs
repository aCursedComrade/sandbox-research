use crate::ipc_srv::ProfileData;
use rand::Rng;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
/// An application profile.
pub struct Profile {
    /// Internal ID
    pub id: u32,
    /// Profile name
    pub name: String,
    /// Profile description
    pub description: String,
    /// The command or path to the application with arguements
    pub command: String,
    /// Is the command actively running?
    pub is_running: bool,
    /// Last recorded process ID
    pub pid: u32,
}

#[derive(Clone, Copy)]
pub enum ValidationError {
    NameTooShort,
    CommandTooShort,
}

impl Profile {
    pub fn new<S: Into<String>>(id: u32, name: S, description: S, command: S) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            command: command.into(),
            is_running: false,
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

        // minimum 3 character name
        if self.name.len() < 3 {
            errors.push(ValidationError::NameTooShort);
        }

        // minimum 5 character command
        // this is to avoid possible weirdness when executing programs
        // minimum viable examples are: b.exe | c.bat | d.ps1
        if self.command.len() < 5 {
            errors.push(ValidationError::CommandTooShort);
        }

        errors
    }
}

impl Default for Profile {
    fn default() -> Self {
        let id = rand::thread_rng().gen();

        Self {
            id,
            name: "New Profile".to_owned(),
            description: "<No description>".to_owned(),
            command: "cmd.exe".to_owned(),
            is_running: false,
            pid: 0,
        }
    }
}

impl From<ProfileData> for Profile {
    fn from(val: ProfileData) -> Self {
        Self {
            id: val.id,
            name: val.name,
            description: val.description,
            command: val.command,
            is_running: val.is_running,
            pid: val.pid,
        }
    }
}

impl From<Profile> for ProfileData {
    fn from(val: Profile) -> Self {
        Self {
            id: val.id,
            name: val.name,
            description: val.description,
            command: val.command,
            is_running: val.is_running,
            pid: val.pid,
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NameTooShort => write!(f, "Profile name is too short (at least 3 characters)"),
            Self::CommandTooShort => write!(f, "Command is invalid (minimum viable example would be \"b.exe\")"),
        }
    }
}

impl Debug for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
