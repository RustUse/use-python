#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! uv_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates non-empty uv metadata text.
            ///
            /// # Errors
            ///
            /// Returns [`UvTextError::Empty`] when `input` is empty after trimming.
            pub fn new(input: &str) -> Result<Self, UvTextError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(UvTextError::Empty)
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
            type Err = UvTextError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = UvTextError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

/// Common uv command labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UvCommand {
    Init,
    Add,
    Remove,
    Sync,
    Lock,
    Run,
    Build,
    Publish,
    Python,
    Pip,
    Tool,
    Venv,
}

impl UvCommand {
    /// Returns the command label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Add => "add",
            Self::Remove => "remove",
            Self::Sync => "sync",
            Self::Lock => "lock",
            Self::Run => "run",
            Self::Build => "build",
            Self::Publish => "publish",
            Self::Python => "python",
            Self::Pip => "pip",
            Self::Tool => "tool",
            Self::Venv => "venv",
        }
    }
}

impl fmt::Display for UvCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for UvCommand {
    type Err = UvTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized(input)?.as_str() {
            "init" => Ok(Self::Init),
            "add" => Ok(Self::Add),
            "remove" => Ok(Self::Remove),
            "sync" => Ok(Self::Sync),
            "lock" => Ok(Self::Lock),
            "run" => Ok(Self::Run),
            "build" => Ok(Self::Build),
            "publish" => Ok(Self::Publish),
            "python" => Ok(Self::Python),
            "pip" => Ok(Self::Pip),
            "tool" => Ok(Self::Tool),
            "venv" => Ok(Self::Venv),
            _ => Err(UvTextError::Unknown),
        }
    }
}

/// uv project subcommand labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UvProjectCommand {
    Init,
    Add,
    Remove,
    Sync,
    Lock,
    Run,
    Build,
    Publish,
}

impl UvProjectCommand {
    /// Returns the subcommand label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Add => "add",
            Self::Remove => "remove",
            Self::Sync => "sync",
            Self::Lock => "lock",
            Self::Run => "run",
            Self::Build => "build",
            Self::Publish => "publish",
        }
    }
}

impl fmt::Display for UvProjectCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for UvProjectCommand {
    type Err = UvTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized(input)?.as_str() {
            "init" => Ok(Self::Init),
            "add" => Ok(Self::Add),
            "remove" => Ok(Self::Remove),
            "sync" => Ok(Self::Sync),
            "lock" => Ok(Self::Lock),
            "run" => Ok(Self::Run),
            "build" => Ok(Self::Build),
            "publish" => Ok(Self::Publish),
            _ => Err(UvTextError::Unknown),
        }
    }
}

/// uv python subcommand labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UvPythonCommand {
    Install,
    List,
    Pin,
    Dir,
}

impl UvPythonCommand {
    /// Returns the subcommand label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::List => "list",
            Self::Pin => "pin",
            Self::Dir => "dir",
        }
    }
}

impl fmt::Display for UvPythonCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for UvPythonCommand {
    type Err = UvTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized(input)?.as_str() {
            "install" => Ok(Self::Install),
            "list" => Ok(Self::List),
            "pin" => Ok(Self::Pin),
            "dir" => Ok(Self::Dir),
            _ => Err(UvTextError::Unknown),
        }
    }
}

/// uv tool subcommand labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UvToolCommand {
    Install,
    Run,
    List,
    Uninstall,
    Upgrade,
}

impl UvToolCommand {
    /// Returns the subcommand label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Run => "run",
            Self::List => "list",
            Self::Uninstall => "uninstall",
            Self::Upgrade => "upgrade",
        }
    }
}

impl fmt::Display for UvToolCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for UvToolCommand {
    type Err = UvTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized(input)?.as_str() {
            "install" => Ok(Self::Install),
            "run" => Ok(Self::Run),
            "list" => Ok(Self::List),
            "uninstall" => Ok(Self::Uninstall),
            "upgrade" => Ok(Self::Upgrade),
            _ => Err(UvTextError::Unknown),
        }
    }
}

/// uv lockfile labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UvLockfile {
    UvLock,
}

impl UvLockfile {
    /// Returns the lockfile label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        "uv.lock"
    }
}

impl fmt::Display for UvLockfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for UvLockfile {
    type Err = UvTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_ascii_lowercase().as_str() {
            "uv.lock" | "uvlock" => Ok(Self::UvLock),
            "" => Err(UvTextError::Empty),
            _ => Err(UvTextError::Unknown),
        }
    }
}

/// uv config file labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UvConfigFile {
    PyProjectToml,
    UvToml,
}

impl UvConfigFile {
    /// Returns the config file label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PyProjectToml => "pyproject.toml",
            Self::UvToml => "uv.toml",
        }
    }
}

impl fmt::Display for UvConfigFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for UvConfigFile {
    type Err = UvTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_ascii_lowercase().as_str() {
            "pyproject.toml" | "pyprojecttoml" => Ok(Self::PyProjectToml),
            "uv.toml" | "uvtoml" => Ok(Self::UvToml),
            "" => Err(UvTextError::Empty),
            _ => Err(UvTextError::Unknown),
        }
    }
}

uv_text_newtype!(UvWorkspace);
uv_text_newtype!(UvPackageSpec);

/// Error returned when uv metadata text is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UvTextError {
    Empty,
    Unknown,
}

impl fmt::Display for UvTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("uv metadata text cannot be empty"),
            Self::Unknown => formatter.write_str("unknown uv command"),
        }
    }
}

impl Error for UvTextError {}

fn normalized(input: &str) -> Result<String, UvTextError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(UvTextError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UvCommand, UvConfigFile, UvLockfile, UvPackageSpec, UvProjectCommand, UvPythonCommand,
        UvTextError, UvToolCommand, UvWorkspace,
    };

    #[test]
    fn models_uv_commands_and_files() -> Result<(), UvTextError> {
        assert_eq!("sync".parse::<UvCommand>()?, UvCommand::Sync);
        assert_eq!(
            "build".parse::<UvProjectCommand>()?,
            UvProjectCommand::Build
        );
        assert_eq!("pin".parse::<UvPythonCommand>()?, UvPythonCommand::Pin);
        assert_eq!("upgrade".parse::<UvToolCommand>()?, UvToolCommand::Upgrade);
        assert_eq!(UvLockfile::UvLock.as_str(), "uv.lock");
        assert_eq!("uv.lock".parse::<UvLockfile>()?.to_string(), "uv.lock");
        assert_eq!(UvConfigFile::UvToml.as_str(), "uv.toml");
        assert_eq!(
            "pyproject.toml".parse::<UvConfigFile>()?,
            UvConfigFile::PyProjectToml
        );
        Ok(())
    }

    #[test]
    fn validates_workspace_and_package_specs() -> Result<(), UvTextError> {
        assert_eq!(UvWorkspace::new("workspace")?.as_str(), "workspace");
        assert_eq!(UvPackageSpec::new("ruff>=0.4")?.as_str(), "ruff>=0.4");
        assert_eq!(UvPackageSpec::new(""), Err(UvTextError::Empty));
        Ok(())
    }
}
