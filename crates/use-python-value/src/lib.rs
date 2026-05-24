#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Primitive Python-like values for metadata and validation helpers.
#[derive(Clone, Debug, PartialEq)]
pub enum PythonPrimitiveValue {
    None,
    Bool(bool),
    Int(String),
    Float(f64),
    Complex { real: f64, imag: f64 },
    String(String),
    Bytes(Vec<u8>),
    Ellipsis,
}

impl PythonPrimitiveValue {
    /// Returns a Python-style primitive type label.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::None => "NoneType",
            Self::Bool(_) => "bool",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Complex { .. } => "complex",
            Self::String(_) => "str",
            Self::Bytes(_) => "bytes",
            Self::Ellipsis => "ellipsis",
        }
    }

    /// Returns whether the value is Python `None`.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns whether the value is approximately truthy by common Python primitive rules.
    #[must_use]
    pub fn is_truthy_like(&self) -> bool {
        match self {
            Self::None => false,
            Self::Bool(value) => *value,
            Self::Int(value) => !matches!(normalized_int_text(value).as_str(), "" | "0"),
            Self::Float(value) => *value != 0.0 && !value.is_nan(),
            Self::Complex { real, imag } => {
                (*real != 0.0 || *imag != 0.0) && !real.is_nan() && !imag.is_nan()
            }
            Self::String(value) => !value.is_empty(),
            Self::Bytes(value) => !value.is_empty(),
            Self::Ellipsis => true,
        }
    }

    /// Returns whether the value is numeric-like.
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Bool(_) | Self::Int(_) | Self::Float(_) | Self::Complex { .. }
        )
    }
}

/// Primitive Python number value metadata.
#[derive(Clone, Debug, PartialEq)]
pub enum PythonNumberValue {
    Bool(bool),
    Int(String),
    Float(f64),
    Complex { real: f64, imag: f64 },
}

impl PythonNumberValue {
    /// Returns a Python-style numeric type label.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Bool(_) => "bool",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Complex { .. } => "complex",
        }
    }
}

/// Python string literal kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonStringKind {
    String,
    RawString,
    FormatString,
    TemplateString,
}

impl PythonStringKind {
    /// Returns the string kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::RawString => "raw-string",
            Self::FormatString => "format-string",
            Self::TemplateString => "template-string",
        }
    }
}

impl fmt::Display for PythonStringKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonStringKind {
    type Err = PythonValueParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "string" | "str" => Ok(Self::String),
            "rawstring" | "raw" | "r" => Ok(Self::RawString),
            "formatstring" | "fstring" | "f" => Ok(Self::FormatString),
            "templatestring" | "tstring" | "t" => Ok(Self::TemplateString),
            _ => Err(PythonValueParseError::UnknownLabel),
        }
    }
}

/// Python bytes value metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonBytesValue(Vec<u8>);

impl PythonBytesValue {
    /// Creates Python bytes metadata from raw bytes.
    #[must_use]
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    /// Returns the stored bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Python `None` metadata marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonNone;

/// Python ellipsis metadata marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonEllipsis;

/// Error returned when Python value metadata labels are invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PythonValueParseError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for PythonValueParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Python value metadata label cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown Python value metadata label"),
        }
    }
}

impl Error for PythonValueParseError {}

fn normalized_int_text(input: &str) -> String {
    let trimmed = input.trim();
    let unsigned = trimmed.strip_prefix(['+', '-']).unwrap_or(trimmed);
    unsigned.trim_start_matches('0').to_string()
}

fn normalized_label(input: &str) -> Result<String, PythonValueParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PythonValueParseError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['-', '_', ' '], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::{PythonBytesValue, PythonPrimitiveValue, PythonStringKind, PythonValueParseError};

    #[test]
    fn reports_type_names() {
        assert_eq!(PythonPrimitiveValue::None.type_name(), "NoneType");
        assert_eq!(
            PythonPrimitiveValue::Int(String::from("10")).type_name(),
            "int"
        );
        assert_eq!(PythonStringKind::RawString.as_str(), "raw-string");
    }

    #[test]
    fn parses_and_displays_string_kinds() -> Result<(), PythonValueParseError> {
        assert_eq!(
            "f-string".parse::<PythonStringKind>()?,
            PythonStringKind::FormatString
        );
        assert_eq!(
            PythonStringKind::TemplateString.to_string(),
            "template-string"
        );
        Ok(())
    }

    #[test]
    fn checks_truthy_and_numeric_values() {
        assert!(!PythonPrimitiveValue::None.is_truthy_like());
        assert!(!PythonPrimitiveValue::Int(String::from("000")).is_truthy_like());
        assert!(PythonPrimitiveValue::String(String::from("x")).is_truthy_like());
        assert!(PythonPrimitiveValue::Bool(false).is_numeric());
    }

    #[test]
    fn stores_bytes_metadata() {
        let bytes = PythonBytesValue::new([1_u8, 2, 3]);

        assert_eq!(bytes.as_bytes(), &[1, 2, 3]);
    }
}
