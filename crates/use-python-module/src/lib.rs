#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_python_identifier::{PythonIdentifier, PythonIdentifierError};

macro_rules! dotted_name_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates lightly validated dotted Python name metadata.
            ///
            /// # Errors
            ///
            /// Returns [`PythonModuleNameError`] when `input` is empty, has empty segments, or contains invalid identifier segments.
            pub fn new(input: &str) -> Result<Self, PythonModuleNameError> {
                validate_dotted_name(input)?;
                Ok(Self(input.to_string()))
            }

            /// Returns the dotted name.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Returns the dotted name segments.
            #[must_use]
            pub fn segments(&self) -> Vec<&str> {
                self.0.split('.').collect()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = PythonModuleNameError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = PythonModuleNameError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

dotted_name_newtype!(PythonModuleName);
dotted_name_newtype!(PythonPackageName);
dotted_name_newtype!(PythonImportName);

/// Python import statement kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonImportKind {
    Absolute,
    Relative,
    FromImport,
    StarImport,
}

impl PythonImportKind {
    /// Returns the import kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Absolute => "absolute",
            Self::Relative => "relative",
            Self::FromImport => "from-import",
            Self::StarImport => "star-import",
        }
    }
}

impl fmt::Display for PythonImportKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonImportKind {
    type Err = PythonModuleNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "absolute" => Ok(Self::Absolute),
            "relative" => Ok(Self::Relative),
            "fromimport" | "from" => Ok(Self::FromImport),
            "starimport" | "star" => Ok(Self::StarImport),
            _ => Err(PythonModuleNameError::UnknownLabel),
        }
    }
}

/// Python module path metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PythonModulePath(String);

impl PythonModulePath {
    /// Creates module path metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PythonModuleNameError::Empty`] when `input` is empty after trimming.
    pub fn new(input: &str) -> Result<Self, PythonModuleNameError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err(PythonModuleNameError::Empty)
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

/// Python file-kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonFileKind {
    Module,
    PackageInit,
    Script,
    Test,
    Stub,
    Config,
}

impl PythonFileKind {
    /// Returns the file kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::PackageInit => "package-init",
            Self::Script => "script",
            Self::Test => "test",
            Self::Stub => "stub",
            Self::Config => "config",
        }
    }
}

impl fmt::Display for PythonFileKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonFileKind {
    type Err = PythonModuleNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "module" => Ok(Self::Module),
            "packageinit" | "init" => Ok(Self::PackageInit),
            "script" => Ok(Self::Script),
            "test" => Ok(Self::Test),
            "stub" | "pyi" => Ok(Self::Stub),
            "config" => Ok(Self::Config),
            _ => Err(PythonModuleNameError::UnknownLabel),
        }
    }
}

/// Python package layout metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonPackageLayout {
    Flat,
    Src,
    NamespacePackage,
}

impl PythonPackageLayout {
    /// Returns the package layout label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Src => "src",
            Self::NamespacePackage => "namespace-package",
        }
    }
}

impl fmt::Display for PythonPackageLayout {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonPackageLayout {
    type Err = PythonModuleNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "flat" => Ok(Self::Flat),
            "src" => Ok(Self::Src),
            "namespacepackage" | "namespace" => Ok(Self::NamespacePackage),
            _ => Err(PythonModuleNameError::UnknownLabel),
        }
    }
}

/// Error returned when Python module name metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PythonModuleNameError {
    Empty,
    EmptySegment,
    Identifier(PythonIdentifierError),
    UnknownLabel,
}

impl fmt::Display for PythonModuleNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Python module metadata cannot be empty"),
            Self::EmptySegment => {
                formatter.write_str("Python module name cannot contain empty segments")
            }
            Self::Identifier(error) => write!(formatter, "invalid Python module segment: {error}"),
            Self::UnknownLabel => formatter.write_str("unknown Python module metadata label"),
        }
    }
}

impl Error for PythonModuleNameError {}

fn validate_dotted_name(input: &str) -> Result<(), PythonModuleNameError> {
    if input.trim().is_empty() {
        return Err(PythonModuleNameError::Empty);
    }

    for segment in input.split('.') {
        if segment.is_empty() {
            return Err(PythonModuleNameError::EmptySegment);
        }
        PythonIdentifier::new(segment).map_err(PythonModuleNameError::Identifier)?;
    }

    Ok(())
}

fn normalized_label(input: &str) -> Result<String, PythonModuleNameError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PythonModuleNameError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['-', '_', ' '], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PythonFileKind, PythonImportKind, PythonImportName, PythonModuleName,
        PythonModuleNameError, PythonPackageLayout, PythonPackageName,
    };

    #[test]
    fn validates_dotted_names() -> Result<(), PythonModuleNameError> {
        let module = PythonModuleName::new("package.module")?;
        let package = PythonPackageName::new("package")?;
        let import_name = PythonImportName::new("package.submodule")?;

        assert_eq!(module.segments(), vec!["package", "module"]);
        assert_eq!(package.as_str(), "package");
        assert_eq!(import_name.as_str(), "package.submodule");
        Ok(())
    }

    #[test]
    fn rejects_empty_or_invalid_segments() {
        assert_eq!(PythonModuleName::new(""), Err(PythonModuleNameError::Empty));
        assert_eq!(
            PythonModuleName::new("package..module"),
            Err(PythonModuleNameError::EmptySegment)
        );
        assert!(matches!(
            PythonModuleName::new("package.class"),
            Err(PythonModuleNameError::Identifier(_))
        ));
    }

    #[test]
    fn parses_and_displays_import_file_and_layout_labels() -> Result<(), PythonModuleNameError> {
        assert_eq!(
            "from-import".parse::<PythonImportKind>()?,
            PythonImportKind::FromImport
        );
        assert_eq!(PythonImportKind::StarImport.to_string(), "star-import");
        assert_eq!(
            "package-init".parse::<PythonFileKind>()?,
            PythonFileKind::PackageInit
        );
        assert_eq!(PythonFileKind::Stub.to_string(), "stub");
        assert_eq!(
            "namespace-package".parse::<PythonPackageLayout>()?,
            PythonPackageLayout::NamespacePackage
        );
        assert_eq!(PythonPackageLayout::Src.to_string(), "src");
        Ok(())
    }
}
