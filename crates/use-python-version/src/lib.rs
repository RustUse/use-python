#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Python major version component.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonMajorVersion(u16);

impl PythonMajorVersion {
    /// Creates a non-zero Python major version component.
    ///
    /// # Errors
    ///
    /// Returns [`PythonVersionParseError::InvalidVersion`] when `value` is zero.
    pub const fn new(value: u16) -> Result<Self, PythonVersionParseError> {
        if value == 0 {
            Err(PythonVersionParseError::InvalidVersion)
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the numeric component.
    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }
}

/// Python minor version component.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonMinorVersion(u16);

impl PythonMinorVersion {
    /// Creates a Python minor version component.
    #[must_use]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the numeric component.
    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }
}

/// Python patch version component.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonPatchVersion(u16);

impl PythonPatchVersion {
    /// Creates a Python patch version component.
    #[must_use]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the numeric component.
    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }
}

/// Lightweight Python version metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonVersion {
    major: PythonMajorVersion,
    minor: Option<PythonMinorVersion>,
    patch: Option<PythonPatchVersion>,
    suffix: Option<String>,
}

impl PythonVersion {
    /// Creates Python version metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PythonVersionParseError::InvalidVersion`] when the major version is zero or
    /// a patch component is present without a minor component.
    pub fn new(
        major: u16,
        minor: Option<u16>,
        patch: Option<u16>,
    ) -> Result<Self, PythonVersionParseError> {
        if minor.is_none() && patch.is_some() {
            return Err(PythonVersionParseError::InvalidVersion);
        }

        Ok(Self {
            major: PythonMajorVersion::new(major)?,
            minor: minor.map(PythonMinorVersion::new),
            patch: patch.map(PythonPatchVersion::new),
            suffix: None,
        })
    }

    /// Returns the major version number.
    #[must_use]
    pub const fn major(&self) -> u16 {
        self.major.get()
    }

    /// Returns the optional minor version number.
    #[must_use]
    pub const fn minor(&self) -> Option<u16> {
        match self.minor {
            Some(value) => Some(value.get()),
            None => None,
        }
    }

    /// Returns the optional patch version number.
    #[must_use]
    pub const fn patch(&self) -> Option<u16> {
        match self.patch {
            Some(value) => Some(value.get()),
            None => None,
        }
    }

    /// Returns whether this version is in the Python 3 family.
    #[must_use]
    pub const fn is_python3(&self) -> bool {
        self.major() == 3
    }

    /// Returns whether the parsed suffix looks like a prerelease marker.
    #[must_use]
    pub fn is_prerelease_like(&self) -> bool {
        self.suffix.as_deref().is_some_and(|suffix| {
            suffix
                .chars()
                .any(|character| character.is_ascii_alphabetic())
        })
    }

    /// Returns the parsed non-numeric suffix when one was present.
    #[must_use]
    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }
}

impl fmt::Display for PythonVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.major())?;
        if let Some(minor) = self.minor() {
            write!(formatter, ".{minor}")?;
        }
        if let Some(patch) = self.patch() {
            write!(formatter, ".{patch}")?;
        }
        if let Some(suffix) = self.suffix() {
            formatter.write_str(suffix)?;
        }
        Ok(())
    }
}

impl FromStr for PythonVersion {
    type Err = PythonVersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_python_version(input)
    }
}

/// Python version family label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonVersionFamily {
    Python2,
    Python3,
}

impl PythonVersionFamily {
    /// Returns the lowercase family label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Python2 => "python2",
            Self::Python3 => "python3",
        }
    }
}

impl fmt::Display for PythonVersionFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonVersionFamily {
    type Err = PythonVersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "python2" | "py2" | "2" => Ok(Self::Python2),
            "python3" | "py3" | "3" => Ok(Self::Python3),
            _ => Err(PythonVersionParseError::UnknownLabel),
        }
    }
}

/// Python implementation label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonImplementation {
    CPython,
    PyPy,
    MicroPython,
    GraalPy,
    RustPython,
}

impl PythonImplementation {
    /// Returns the normalized implementation label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CPython => "cpython",
            Self::PyPy => "pypy",
            Self::MicroPython => "micropython",
            Self::GraalPy => "graalpy",
            Self::RustPython => "rustpython",
        }
    }
}

impl fmt::Display for PythonImplementation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonImplementation {
    type Err = PythonVersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "cpython" | "cp" => Ok(Self::CPython),
            "pypy" | "pp" => Ok(Self::PyPy),
            "micropython" => Ok(Self::MicroPython),
            "graalpy" => Ok(Self::GraalPy),
            "rustpython" => Ok(Self::RustPython),
            _ => Err(PythonVersionParseError::UnknownLabel),
        }
    }
}

macro_rules! tag_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates non-empty Python tag metadata.
            ///
            /// # Errors
            ///
            /// Returns [`PythonVersionParseError::Empty`] when `input` is empty after trimming.
            pub fn new(input: &str) -> Result<Self, PythonVersionParseError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PythonVersionParseError::Empty)
                } else {
                    Ok(Self(trimmed.to_string()))
                }
            }

            /// Returns the stored tag text.
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
            type Err = PythonVersionParseError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = PythonVersionParseError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

tag_newtype!(PythonCompatibilityTag);
tag_newtype!(PythonAbiTag);
tag_newtype!(PythonPlatformTag);

/// Error returned while parsing Python version metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PythonVersionParseError {
    Empty,
    InvalidVersion,
    UnknownLabel,
}

impl fmt::Display for PythonVersionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Python version metadata cannot be empty"),
            Self::InvalidVersion => formatter.write_str("invalid Python version"),
            Self::UnknownLabel => formatter.write_str("unknown Python version metadata label"),
        }
    }
}

impl Error for PythonVersionParseError {}

fn parse_python_version(input: &str) -> Result<PythonVersion, PythonVersionParseError> {
    let mut text = input.trim();
    if text.is_empty() {
        return Err(PythonVersionParseError::Empty);
    }
    if text.len() >= 6 && text[..6].eq_ignore_ascii_case("python") {
        text = text[6..].trim_start();
    }
    if let Some(stripped) = text.strip_prefix(['v', 'V']) {
        text = stripped;
    }

    let mut parts = text.splitn(3, '.');
    let Some(major_text) = parts.next() else {
        return Err(PythonVersionParseError::InvalidVersion);
    };
    let (major, major_suffix) = parse_component_with_suffix(major_text)?;
    if !major_suffix.is_empty() {
        return PythonVersion::new(major, None, None).map(|mut version| {
            version.suffix = Some(major_suffix.to_string());
            version
        });
    }

    let minor = match parts.next() {
        Some(minor_text) => {
            let (value, suffix) = parse_component_with_suffix(minor_text)?;
            if suffix.is_empty() {
                Some(value)
            } else {
                return PythonVersion::new(major, Some(value), None).map(|mut version| {
                    version.suffix = Some(suffix.to_string());
                    version
                });
            }
        }
        None => None,
    };

    let (patch, suffix) = match parts.next() {
        Some(patch_text) => {
            let (value, suffix) = parse_component_with_suffix(patch_text)?;
            (Some(value), suffix)
        }
        None => (None, ""),
    };

    PythonVersion::new(major, minor, patch).map(|mut version| {
        if !suffix.is_empty() {
            version.suffix = Some(suffix.to_string());
        }
        version
    })
}

fn parse_component_with_suffix(input: &str) -> Result<(u16, &str), PythonVersionParseError> {
    if input.is_empty() {
        return Err(PythonVersionParseError::InvalidVersion);
    }
    let digit_len = input
        .char_indices()
        .take_while(|(_, character)| character.is_ascii_digit())
        .map(|(index, character)| index + character.len_utf8())
        .last()
        .ok_or(PythonVersionParseError::InvalidVersion)?;
    let digits = &input[..digit_len];
    let suffix = &input[digit_len..];
    let value = digits
        .parse::<u16>()
        .map_err(|_| PythonVersionParseError::InvalidVersion)?;
    Ok((value, suffix))
}

fn normalized_label(input: &str) -> Result<String, PythonVersionParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PythonVersionParseError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['-', '_', ' '], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PythonAbiTag, PythonImplementation, PythonPlatformTag, PythonVersion, PythonVersionFamily,
        PythonVersionParseError,
    };

    #[test]
    fn parses_common_version_shapes() -> Result<(), PythonVersionParseError> {
        let major_only: PythonVersion = "3".parse()?;
        let minor: PythonVersion = "3.11".parse()?;
        let patch: PythonVersion = "Python 3.12.1".parse()?;
        let prefixed: PythonVersion = "v3.13.0".parse()?;

        assert_eq!(major_only.major(), 3);
        assert_eq!(minor.minor(), Some(11));
        assert_eq!(patch.patch(), Some(1));
        assert_eq!(prefixed.to_string(), "3.13.0");
        assert!(prefixed.is_python3());
        Ok(())
    }

    #[test]
    fn parses_prerelease_like_suffixes() -> Result<(), PythonVersionParseError> {
        let version: PythonVersion = "3.14.0rc1".parse()?;

        assert!(version.is_prerelease_like());
        assert_eq!(version.suffix(), Some("rc1"));
        assert_eq!(version.to_string(), "3.14.0rc1");
        Ok(())
    }

    #[test]
    fn models_implementation_and_tags() -> Result<(), PythonVersionParseError> {
        assert_eq!(
            "CPython".parse::<PythonImplementation>()?,
            PythonImplementation::CPython
        );
        assert_eq!(
            "py3".parse::<PythonVersionFamily>()?,
            PythonVersionFamily::Python3
        );
        assert_eq!(PythonVersionFamily::Python2.to_string(), "python2");
        assert_eq!(PythonAbiTag::new("cp312")?.as_str(), "cp312");
        assert_eq!(
            PythonPlatformTag::new("manylinux_x86_64")?.to_string(),
            "manylinux_x86_64"
        );
        Ok(())
    }
}
