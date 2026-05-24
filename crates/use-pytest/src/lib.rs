#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_python_identifier::{PythonIdentifier, PythonIdentifierError};

macro_rules! pytest_identifier_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(PythonIdentifier);

        impl $name {
            /// Creates pytest identifier metadata.
            ///
            /// # Errors
            ///
            /// Returns [`PytestNameError::Identifier`] when `input` is not an ASCII-safe Python identifier.
            pub fn new(input: &str) -> Result<Self, PytestNameError> {
                PythonIdentifier::new(input)
                    .map(Self)
                    .map_err(PytestNameError::Identifier)
            }

            /// Returns the stored name.
            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = PytestNameError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = PytestNameError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

pytest_identifier_newtype!(PytestTestName);
pytest_identifier_newtype!(PytestMarkerName);
pytest_identifier_newtype!(PytestFixtureName);

/// pytest node ID metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PytestNodeId(String);

impl PytestNodeId {
    /// Creates pytest node ID metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PytestNameError::Empty`] when `input` is empty after trimming.
    pub fn new(input: &str) -> Result<Self, PytestNameError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err(PytestNameError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the node ID text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether the node ID contains a pytest `::` separator.
    #[must_use]
    pub fn has_scope_separator(&self) -> bool {
        self.0.contains("::")
    }
}

impl fmt::Display for PytestNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PytestNodeId {
    type Err = PytestNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

impl TryFrom<&str> for PytestNodeId {
    type Error = PytestNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Common pytest config file labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PytestConfigFile {
    PyProjectToml,
    PytestIni,
    SetupCfg,
    ToxIni,
}

impl PytestConfigFile {
    /// Returns the config file label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PyProjectToml => "pyproject.toml",
            Self::PytestIni => "pytest.ini",
            Self::SetupCfg => "setup.cfg",
            Self::ToxIni => "tox.ini",
        }
    }
}

impl fmt::Display for PytestConfigFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PytestConfigFile {
    type Err = PytestNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_ascii_lowercase().as_str() {
            "pyproject.toml" | "pyprojecttoml" => Ok(Self::PyProjectToml),
            "pytest.ini" | "pytestini" => Ok(Self::PytestIni),
            "setup.cfg" | "setupcfg" => Ok(Self::SetupCfg),
            "tox.ini" | "toxini" => Ok(Self::ToxIni),
            "" => Err(PytestNameError::Empty),
            _ => Err(PytestNameError::UnknownLabel),
        }
    }
}

/// pytest outcome labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PytestOutcome {
    Passed,
    Failed,
    Skipped,
    XFailed,
    XPassed,
    Error,
}

impl PytestOutcome {
    /// Returns the outcome label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
            Self::XFailed => "xfailed",
            Self::XPassed => "xpassed",
            Self::Error => "error",
        }
    }
}

impl fmt::Display for PytestOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PytestOutcome {
    type Err = PytestNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "passed" | "pass" => Ok(Self::Passed),
            "failed" | "fail" => Ok(Self::Failed),
            "skipped" | "skip" => Ok(Self::Skipped),
            "xfailed" | "xfail" => Ok(Self::XFailed),
            "xpassed" | "xpass" => Ok(Self::XPassed),
            "error" => Ok(Self::Error),
            _ => Err(PytestNameError::UnknownLabel),
        }
    }
}

/// pytest fixture scope labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PytestScope {
    Function,
    Class,
    Module,
    Package,
    Session,
}

impl PytestScope {
    /// Returns the fixture scope label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Class => "class",
            Self::Module => "module",
            Self::Package => "package",
            Self::Session => "session",
        }
    }
}

impl fmt::Display for PytestScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PytestScope {
    type Err = PytestNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "function" => Ok(Self::Function),
            "class" => Ok(Self::Class),
            "module" => Ok(Self::Module),
            "package" => Ok(Self::Package),
            "session" => Ok(Self::Session),
            _ => Err(PytestNameError::UnknownLabel),
        }
    }
}

/// pytest file-kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PytestFileKind {
    TestModule,
    Conftest,
    FixtureModule,
}

impl PytestFileKind {
    /// Returns the file kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestModule => "test-module",
            Self::Conftest => "conftest",
            Self::FixtureModule => "fixture-module",
        }
    }
}

impl fmt::Display for PytestFileKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PytestFileKind {
    type Err = PytestNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "testmodule" | "test" => Ok(Self::TestModule),
            "conftest" => Ok(Self::Conftest),
            "fixturemodule" | "fixture" => Ok(Self::FixtureModule),
            _ => Err(PytestNameError::UnknownLabel),
        }
    }
}

/// Error returned when pytest metadata names are invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PytestNameError {
    Empty,
    Identifier(PythonIdentifierError),
    UnknownLabel,
}

impl fmt::Display for PytestNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pytest metadata name cannot be empty"),
            Self::Identifier(error) => write!(formatter, "invalid pytest identifier: {error}"),
            Self::UnknownLabel => formatter.write_str("unknown pytest metadata label"),
        }
    }
}

impl Error for PytestNameError {}

fn normalized_label(input: &str) -> Result<String, PytestNameError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PytestNameError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['-', '_', ' '], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PytestConfigFile, PytestFileKind, PytestFixtureName, PytestMarkerName, PytestNameError,
        PytestNodeId, PytestOutcome, PytestScope, PytestTestName,
    };

    #[test]
    fn validates_pytest_identifier_names() -> Result<(), PytestNameError> {
        let test_name = PytestTestName::new("test_smoke")?;
        let marker = PytestMarkerName::new("slow")?;
        let fixture = PytestFixtureName::new("tmp_path")?;

        assert_eq!(test_name.as_str(), "test_smoke");
        assert_eq!(marker.as_str(), "slow");
        assert_eq!(fixture.as_str(), "tmp_path");
        Ok(())
    }

    #[test]
    fn validates_node_ids_and_labels() -> Result<(), PytestNameError> {
        let node_id = PytestNodeId::new("tests/test_app.py::test_smoke")?;

        assert!(node_id.has_scope_separator());
        assert_eq!(
            "pyproject.toml".parse::<PytestConfigFile>()?,
            PytestConfigFile::PyProjectToml
        );
        assert_eq!(PytestConfigFile::ToxIni.to_string(), "tox.ini");
        assert_eq!("xfail".parse::<PytestOutcome>()?, PytestOutcome::XFailed);
        assert_eq!(PytestOutcome::Passed.to_string(), "passed");
        assert_eq!("session".parse::<PytestScope>()?, PytestScope::Session);
        assert_eq!(PytestScope::Function.to_string(), "function");
        assert_eq!(
            "fixture-module".parse::<PytestFileKind>()?,
            PytestFileKind::FixtureModule
        );
        assert_eq!(PytestFileKind::Conftest.to_string(), "conftest");
        Ok(())
    }
}
