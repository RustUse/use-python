#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! pip_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates non-empty pip metadata text.
            ///
            /// # Errors
            ///
            /// Returns [`PipTextError::Empty`] when `input` is empty after trimming.
            pub fn new(input: &str) -> Result<Self, PipTextError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PipTextError::Empty)
                } else {
                    Ok(Self(trimmed.to_string()))
                }
            }

            /// Returns the stored text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = PipTextError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = PipTextError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

/// Common pip command labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PipCommand {
    Install,
    Uninstall,
    Freeze,
    List,
    Show,
    Check,
    Download,
    Wheel,
    Config,
}

impl PipCommand {
    /// Returns the command label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Uninstall => "uninstall",
            Self::Freeze => "freeze",
            Self::List => "list",
            Self::Show => "show",
            Self::Check => "check",
            Self::Download => "download",
            Self::Wheel => "wheel",
            Self::Config => "config",
        }
    }
}

impl fmt::Display for PipCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PipCommand {
    type Err = PipTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized(input)?.as_str() {
            "install" => Ok(Self::Install),
            "uninstall" => Ok(Self::Uninstall),
            "freeze" => Ok(Self::Freeze),
            "list" => Ok(Self::List),
            "show" => Ok(Self::Show),
            "check" => Ok(Self::Check),
            "download" => Ok(Self::Download),
            "wheel" => Ok(Self::Wheel),
            "config" => Ok(Self::Config),
            _ => Err(PipTextError::Unknown),
        }
    }
}

pip_text_newtype!(PipPackageSpec);
pip_text_newtype!(PipInstallTarget);

/// Validated pip requirement text.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PipRequirement(String);

impl PipRequirement {
    /// Creates pip requirement metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PipTextError::Empty`] when `input` is empty after trimming.
    pub fn new(input: &str) -> Result<Self, PipTextError> {
        let trimmed = non_empty(input)?;
        Ok(Self(trimmed.to_string()))
    }

    /// Returns the requirement text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether the requirement looks like an editable install option.
    #[must_use]
    pub fn is_editable(&self) -> bool {
        is_editable(self.as_str())
    }

    /// Returns whether the requirement looks like a requirements-file option.
    #[must_use]
    pub fn is_requirements_file(&self) -> bool {
        is_requirements_file(self.as_str())
    }
}

impl fmt::Display for PipRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PipRequirement {
    type Err = PipTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

impl TryFrom<&str> for PipRequirement {
    type Error = PipTextError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// pip requirements-file metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PipRequirementFile(String);

impl PipRequirementFile {
    /// Creates requirements-file metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PipTextError::Empty`] when `input` is empty after trimming.
    pub fn new(input: &str) -> Result<Self, PipTextError> {
        Ok(Self(non_empty(input)?.to_string()))
    }

    /// Returns the requirements-file path text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns `true` for requirements-file metadata.
    #[must_use]
    pub const fn is_requirements_file(&self) -> bool {
        true
    }
}

impl fmt::Display for PipRequirementFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// pip package index URL metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PipIndexUrl(String);

impl PipIndexUrl {
    /// Creates index URL metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PipTextError`] when `input` is empty, contains whitespace, or is not HTTP(S)-shaped.
    pub fn new(input: &str) -> Result<Self, PipTextError> {
        let trimmed = non_empty(input)?;
        if trimmed.chars().any(char::is_whitespace) {
            return Err(PipTextError::ContainsWhitespace);
        }
        if !(trimmed.starts_with("https://") || trimmed.starts_with("http://")) {
            return Err(PipTextError::InvalidUrl);
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Returns the index URL text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PipIndexUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// pip editable install metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PipEditableInstall {
    target: PipInstallTarget,
}

impl PipEditableInstall {
    /// Creates editable install metadata.
    #[must_use]
    pub const fn new(target: PipInstallTarget) -> Self {
        Self { target }
    }

    /// Returns the editable target.
    #[must_use]
    pub const fn target(&self) -> &PipInstallTarget {
        &self.target
    }

    /// Returns `true` for editable install metadata.
    #[must_use]
    pub const fn is_editable(&self) -> bool {
        true
    }
}

/// Error returned when pip metadata text is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PipTextError {
    Empty,
    ContainsWhitespace,
    InvalidUrl,
    Unknown,
}

impl fmt::Display for PipTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pip metadata text cannot be empty"),
            Self::ContainsWhitespace => {
                formatter.write_str("pip metadata text cannot contain whitespace")
            }
            Self::InvalidUrl => {
                formatter.write_str("pip index URL must start with http:// or https://")
            }
            Self::Unknown => formatter.write_str("unknown pip command"),
        }
    }
}

impl Error for PipTextError {}

/// Returns whether `input` looks like an editable install option.
#[must_use]
pub fn is_editable(input: &str) -> bool {
    let trimmed = input.trim();
    trimmed == "-e" || trimmed.starts_with("-e ") || trimmed.starts_with("--editable ")
}

/// Returns whether `input` looks like a requirements-file option.
#[must_use]
pub fn is_requirements_file(input: &str) -> bool {
    let trimmed = input.trim();
    trimmed == "-r" || trimmed.starts_with("-r ") || trimmed.starts_with("--requirement ")
}

fn normalized(input: &str) -> Result<String, PipTextError> {
    Ok(non_empty(input)?.to_ascii_lowercase())
}

fn non_empty(input: &str) -> Result<&str, PipTextError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PipTextError::Empty)
    } else {
        Ok(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PipCommand, PipEditableInstall, PipIndexUrl, PipInstallTarget, PipRequirement,
        PipRequirementFile, PipTextError, is_requirements_file,
    };

    #[test]
    fn parses_commands_and_requirements() -> Result<(), PipTextError> {
        let requirement = PipRequirement::new("requests>=2")?;

        assert_eq!("install".parse::<PipCommand>()?, PipCommand::Install);
        assert_eq!(requirement.as_str(), "requests>=2");
        assert!(!requirement.is_editable());
        Ok(())
    }

    #[test]
    fn models_requirement_files_and_editable_installs() -> Result<(), PipTextError> {
        let file = PipRequirementFile::new("requirements.txt")?;
        let editable = PipEditableInstall::new(PipInstallTarget::new(".")?);

        assert!(file.is_requirements_file());
        assert!(editable.is_editable());
        assert!(is_requirements_file("-r requirements-dev.txt"));
        Ok(())
    }

    #[test]
    fn validates_index_urls() -> Result<(), PipTextError> {
        assert_eq!(
            PipIndexUrl::new("https://pypi.org/simple")?.as_str(),
            "https://pypi.org/simple"
        );
        assert_eq!(
            PipIndexUrl::new("ftp://example.test"),
            Err(PipTextError::InvalidUrl)
        );
        Ok(())
    }
}
