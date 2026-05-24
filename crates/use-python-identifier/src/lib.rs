#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_python_keyword::is_python_keyword;

/// Validated ASCII-safe Python identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonIdentifier(String);

impl PythonIdentifier {
    /// Creates an ASCII-safe Python identifier that is not a hard Python keyword.
    ///
    /// # Errors
    ///
    /// Returns [`PythonIdentifierError`] when `input` is empty, keyword-shaped, or not ASCII identifier-shaped.
    pub fn new(input: &str) -> Result<Self, PythonIdentifierError> {
        validate_ascii_python_identifier(input)?;
        if is_python_keyword(input) {
            return Err(PythonIdentifierError::Keyword);
        }
        Ok(Self(input.to_string()))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PythonIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonIdentifier {
    type Err = PythonIdentifierError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

impl TryFrom<&str> for PythonIdentifier {
    type Error = PythonIdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Validated Python dunder name metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonDunderName(PythonIdentifier);

impl PythonDunderName {
    /// Creates Python dunder name metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PythonIdentifierError`] when `input` is not a valid dunder identifier.
    pub fn new(input: &str) -> Result<Self, PythonIdentifierError> {
        let identifier = PythonIdentifier::new(input)?;
        if is_dunder_name(identifier.as_str()) {
            Ok(Self(identifier))
        } else {
            Err(PythonIdentifierError::NotDunderName)
        }
    }

    /// Returns the dunder name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for PythonDunderName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Validated Python private-name metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonPrivateName(PythonIdentifier);

impl PythonPrivateName {
    /// Creates Python private-name metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PythonIdentifierError`] when `input` is not a valid private identifier.
    pub fn new(input: &str) -> Result<Self, PythonIdentifierError> {
        let identifier = PythonIdentifier::new(input)?;
        if is_private_name(identifier.as_str()) {
            Ok(Self(identifier))
        } else {
            Err(PythonIdentifierError::NotPrivateName)
        }
    }

    /// Returns the private name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for PythonPrivateName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Error returned when an ASCII Python identifier is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PythonIdentifierError {
    Empty,
    Keyword,
    InvalidStart { character: char },
    InvalidContinue { index: usize, character: char },
    NotDunderName,
    NotPrivateName,
}

impl fmt::Display for PythonIdentifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Python identifier cannot be empty"),
            Self::Keyword => formatter.write_str("Python identifier cannot be a hard keyword"),
            Self::InvalidStart { character } => {
                write!(formatter, "invalid Python identifier start `{character}`")
            }
            Self::InvalidContinue { index, character } => write!(
                formatter,
                "invalid Python identifier continuation `{character}` at byte index {index}"
            ),
            Self::NotDunderName => formatter.write_str("Python identifier is not a dunder name"),
            Self::NotPrivateName => formatter.write_str("Python identifier is not a private name"),
        }
    }
}

impl Error for PythonIdentifierError {}

/// Returns whether `character` is accepted as an ASCII Python identifier start.
#[must_use]
pub const fn is_ascii_python_identifier_start(character: char) -> bool {
    character == '_' || character.is_ascii_alphabetic()
}

/// Returns whether `character` is accepted after the first identifier character.
#[must_use]
pub const fn is_ascii_python_identifier_continue(character: char) -> bool {
    is_ascii_python_identifier_start(character) || character.is_ascii_digit()
}

/// Returns whether `input` is an ASCII-safe Python identifier and not a hard keyword.
#[must_use]
pub fn is_valid_ascii_python_identifier(input: &str) -> bool {
    PythonIdentifier::new(input).is_ok()
}

/// Returns whether `input` looks like a Python dunder name such as `__init__`.
#[must_use]
pub fn is_dunder_name(input: &str) -> bool {
    input.len() > 4 && input.starts_with("__") && input.ends_with("__")
}

/// Returns whether `input` looks like a single-underscore private name.
#[must_use]
pub fn is_private_name(input: &str) -> bool {
    input.starts_with('_') && !is_dunder_name(input)
}

fn validate_ascii_python_identifier(input: &str) -> Result<(), PythonIdentifierError> {
    if input.trim().is_empty() {
        return Err(PythonIdentifierError::Empty);
    }

    let mut characters = input.char_indices();
    let Some((_, first)) = characters.next() else {
        return Err(PythonIdentifierError::Empty);
    };

    if !is_ascii_python_identifier_start(first) {
        return Err(PythonIdentifierError::InvalidStart { character: first });
    }

    for (index, character) in characters {
        if !is_ascii_python_identifier_continue(character) {
            return Err(PythonIdentifierError::InvalidContinue { index, character });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        PythonDunderName, PythonIdentifier, PythonIdentifierError, PythonPrivateName,
        is_dunder_name, is_private_name, is_valid_ascii_python_identifier,
    };

    #[test]
    fn accepts_ascii_identifiers() -> Result<(), PythonIdentifierError> {
        let identifier = PythonIdentifier::new("async_task_1")?;

        assert_eq!(identifier.as_str(), "async_task_1");
        assert!(is_valid_ascii_python_identifier("_internal"));
        assert!(is_valid_ascii_python_identifier("match"));
        Ok(())
    }

    #[test]
    fn rejects_invalid_identifiers_and_keywords() {
        assert_eq!(PythonIdentifier::new(""), Err(PythonIdentifierError::Empty));
        assert_eq!(
            PythonIdentifier::new("class"),
            Err(PythonIdentifierError::Keyword)
        );
        assert_eq!(
            PythonIdentifier::new("1value"),
            Err(PythonIdentifierError::InvalidStart { character: '1' })
        );
        assert!(!is_valid_ascii_python_identifier("has-dash"));
        assert!(!is_valid_ascii_python_identifier("π"));
    }

    #[test]
    fn validates_dunder_and_private_names() -> Result<(), PythonIdentifierError> {
        let dunder = PythonDunderName::new("__init__")?;
        let private = PythonPrivateName::new("_cache")?;

        assert_eq!(dunder.as_str(), "__init__");
        assert_eq!(private.as_str(), "_cache");
        assert!(is_dunder_name("__len__"));
        assert!(is_private_name("_name"));
        assert!(!is_private_name("__name__"));
        Ok(())
    }
}
