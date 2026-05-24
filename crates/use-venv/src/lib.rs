#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Python virtual environment metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PythonVirtualEnv {
    name: PythonVirtualEnvName,
    kind: PythonVirtualEnvKind,
    path: Option<PythonVirtualEnvPath>,
}

impl PythonVirtualEnv {
    /// Creates virtual environment metadata.
    #[must_use]
    pub const fn new(name: PythonVirtualEnvName, kind: PythonVirtualEnvKind) -> Self {
        Self {
            name,
            kind,
            path: None,
        }
    }

    /// Adds path metadata.
    #[must_use]
    pub fn with_path(mut self, path: PythonVirtualEnvPath) -> Self {
        self.path = Some(path);
        self
    }

    /// Returns the environment name.
    #[must_use]
    pub const fn name(&self) -> &PythonVirtualEnvName {
        &self.name
    }

    /// Returns the environment kind.
    #[must_use]
    pub const fn kind(&self) -> PythonVirtualEnvKind {
        self.kind
    }

    /// Returns path metadata when present.
    #[must_use]
    pub const fn path(&self) -> Option<&PythonVirtualEnvPath> {
        self.path.as_ref()
    }
}

/// Validated virtual environment name metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonVirtualEnvName(String);

impl PythonVirtualEnvName {
    /// Creates virtual environment name metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PythonVirtualEnvError`] when `input` is empty after trimming or contains path separators.
    pub fn new(input: &str) -> Result<Self, PythonVirtualEnvError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(PythonVirtualEnvError::Empty);
        }
        if trimmed.contains(['/', '\\']) {
            return Err(PythonVirtualEnvError::ContainsPathSeparator);
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Returns the environment name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PythonVirtualEnvName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonVirtualEnvName {
    type Err = PythonVirtualEnvError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

impl TryFrom<&str> for PythonVirtualEnvName {
    type Error = PythonVirtualEnvError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Python virtual environment manager kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonVirtualEnvKind {
    Venv,
    Virtualenv,
    Conda,
    Poetry,
    Uv,
    Pipenv,
}

impl PythonVirtualEnvKind {
    /// Returns the environment kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Venv => "venv",
            Self::Virtualenv => "virtualenv",
            Self::Conda => "conda",
            Self::Poetry => "poetry",
            Self::Uv => "uv",
            Self::Pipenv => "pipenv",
        }
    }
}

impl fmt::Display for PythonVirtualEnvKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonVirtualEnvKind {
    type Err = PythonVirtualEnvError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "venv" => Ok(Self::Venv),
            "virtualenv" => Ok(Self::Virtualenv),
            "conda" => Ok(Self::Conda),
            "poetry" => Ok(Self::Poetry),
            "uv" => Ok(Self::Uv),
            "pipenv" => Ok(Self::Pipenv),
            _ => Err(PythonVirtualEnvError::UnknownLabel),
        }
    }
}

/// Virtual environment path metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonVirtualEnvPath(String);

impl PythonVirtualEnvPath {
    /// Creates path metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PythonVirtualEnvError::Empty`] when `input` is empty after trimming.
    pub fn new(input: &str) -> Result<Self, PythonVirtualEnvError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err(PythonVirtualEnvError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the path text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PythonVirtualEnvPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Activation shell labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonActivationShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Cmd,
}

impl PythonActivationShell {
    /// Returns the activation shell label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::PowerShell => "powershell",
            Self::Cmd => "cmd",
        }
    }
}

impl fmt::Display for PythonActivationShell {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonActivationShell {
    type Err = PythonVirtualEnvError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "fish" => Ok(Self::Fish),
            "powershell" | "pwsh" => Ok(Self::PowerShell),
            "cmd" => Ok(Self::Cmd),
            _ => Err(PythonVirtualEnvError::UnknownLabel),
        }
    }
}

/// Python environment variable labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonEnvVar {
    VirtualEnv,
    PythonPath,
    PythonHome,
}

impl PythonEnvVar {
    /// Returns the environment variable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VirtualEnv => "VIRTUAL_ENV",
            Self::PythonPath => "PYTHONPATH",
            Self::PythonHome => "PYTHONHOME",
        }
    }
}

impl fmt::Display for PythonEnvVar {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonEnvVar {
    type Err = PythonVirtualEnvError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "virtualenv" => Ok(Self::VirtualEnv),
            "pythonpath" => Ok(Self::PythonPath),
            "pythonhome" => Ok(Self::PythonHome),
            _ => Err(PythonVirtualEnvError::UnknownLabel),
        }
    }
}

/// Error returned when virtual environment metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PythonVirtualEnvError {
    Empty,
    ContainsPathSeparator,
    UnknownLabel,
}

impl fmt::Display for PythonVirtualEnvError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("virtual environment metadata cannot be empty"),
            Self::ContainsPathSeparator => {
                formatter.write_str("virtual environment name cannot contain path separators")
            }
            Self::UnknownLabel => formatter.write_str("unknown virtual environment metadata label"),
        }
    }
}

impl Error for PythonVirtualEnvError {}

fn normalized_label(input: &str) -> Result<String, PythonVirtualEnvError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PythonVirtualEnvError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['-', '_', ' '], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PythonActivationShell, PythonEnvVar, PythonVirtualEnv, PythonVirtualEnvError,
        PythonVirtualEnvKind, PythonVirtualEnvName, PythonVirtualEnvPath,
    };

    #[test]
    fn validates_virtual_environment_names() -> Result<(), PythonVirtualEnvError> {
        let name = PythonVirtualEnvName::new(".venv")?;

        assert_eq!(name.as_str(), ".venv");
        assert_eq!(
            PythonVirtualEnvName::new(""),
            Err(PythonVirtualEnvError::Empty)
        );
        assert_eq!(
            PythonVirtualEnvName::new("env/bin"),
            Err(PythonVirtualEnvError::ContainsPathSeparator)
        );
        Ok(())
    }

    #[test]
    fn models_environment_metadata() -> Result<(), PythonVirtualEnvError> {
        let env = PythonVirtualEnv::new(
            PythonVirtualEnvName::new(".venv")?,
            PythonVirtualEnvKind::Venv,
        )
        .with_path(PythonVirtualEnvPath::new(".venv")?);

        assert_eq!(env.kind(), PythonVirtualEnvKind::Venv);
        assert_eq!(
            "venv".parse::<PythonVirtualEnvKind>()?,
            PythonVirtualEnvKind::Venv
        );
        assert_eq!(PythonActivationShell::PowerShell.to_string(), "powershell");
        assert_eq!(
            "pwsh".parse::<PythonActivationShell>()?,
            PythonActivationShell::PowerShell
        );
        assert_eq!(PythonEnvVar::VirtualEnv.to_string(), "VIRTUAL_ENV");
        assert_eq!(
            "PYTHONPATH".parse::<PythonEnvVar>()?,
            PythonEnvVar::PythonPath
        );
        Ok(())
    }
}
