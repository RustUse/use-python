#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates non-empty pyproject metadata text.
            ///
            /// # Errors
            ///
            /// Returns [`PyProjectTextError::Empty`] when `input` is empty after trimming.
            pub fn new(input: &str) -> Result<Self, PyProjectTextError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PyProjectTextError::Empty)
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
            type Err = PyProjectTextError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = PyProjectTextError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

text_newtype!(PyProjectDependency);
text_newtype!(PyProjectOptionalDependencyGroup);
text_newtype!(PyProjectScript);
text_newtype!(PyProjectEntryPoint);
text_newtype!(PyProjectToolSection);

/// Partial `pyproject.toml` metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PyProject {
    build_system: Option<PyProjectBuildSystem>,
    project: Option<PyProjectProjectMetadata>,
    tool_sections: Vec<PyProjectToolSection>,
}

impl PyProject {
    /// Creates empty pyproject metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            build_system: None,
            project: None,
            tool_sections: Vec::new(),
        }
    }

    /// Adds build-system metadata.
    #[must_use]
    pub fn with_build_system(mut self, build_system: PyProjectBuildSystem) -> Self {
        self.build_system = Some(build_system);
        self
    }

    /// Adds project metadata.
    #[must_use]
    pub fn with_project(mut self, project: PyProjectProjectMetadata) -> Self {
        self.project = Some(project);
        self
    }

    /// Adds a tool section label.
    #[must_use]
    pub fn with_tool_section(mut self, section: PyProjectToolSection) -> Self {
        self.tool_sections.push(section);
        self
    }

    /// Returns the project name when present.
    #[must_use]
    pub fn project_name(&self) -> Option<&str> {
        self.project
            .as_ref()
            .and_then(PyProjectProjectMetadata::name)
    }

    /// Returns the project version when present.
    #[must_use]
    pub fn project_version(&self) -> Option<&str> {
        self.project
            .as_ref()
            .and_then(PyProjectProjectMetadata::version)
    }

    /// Returns declared dependencies when project metadata is present.
    #[must_use]
    pub fn dependencies(&self) -> &[PyProjectDependency] {
        self.project
            .as_ref()
            .map_or(&[], PyProjectProjectMetadata::dependencies)
    }

    /// Returns optional dependency group labels when present.
    #[must_use]
    pub fn optional_dependency_groups(&self) -> &[PyProjectOptionalDependencyGroup] {
        self.project
            .as_ref()
            .map_or(&[], PyProjectProjectMetadata::optional_dependency_groups)
    }

    /// Returns script labels when present.
    #[must_use]
    pub fn scripts(&self) -> &[PyProjectScript] {
        self.project
            .as_ref()
            .map_or(&[], PyProjectProjectMetadata::scripts)
    }

    /// Returns the build backend when present.
    #[must_use]
    pub fn build_backend(&self) -> Option<&PyProjectBuildBackend> {
        self.build_system
            .as_ref()
            .and_then(PyProjectBuildSystem::build_backend)
    }

    /// Returns tool section labels.
    #[must_use]
    pub fn tool_sections(&self) -> &[PyProjectToolSection] {
        &self.tool_sections
    }
}

/// Partial `[build-system]` metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PyProjectBuildSystem {
    requires: Vec<PyProjectDependency>,
    build_backend: Option<PyProjectBuildBackend>,
}

impl PyProjectBuildSystem {
    /// Creates build-system metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requires: Vec::new(),
            build_backend: None,
        }
    }

    /// Adds a build requirement.
    #[must_use]
    pub fn with_requirement(mut self, requirement: PyProjectDependency) -> Self {
        self.requires.push(requirement);
        self
    }

    /// Adds a build backend label.
    #[must_use]
    pub fn with_build_backend(mut self, build_backend: PyProjectBuildBackend) -> Self {
        self.build_backend = Some(build_backend);
        self
    }

    /// Returns build requirements.
    #[must_use]
    pub fn requires(&self) -> &[PyProjectDependency] {
        &self.requires
    }

    /// Returns the build backend when present.
    #[must_use]
    pub const fn build_backend(&self) -> Option<&PyProjectBuildBackend> {
        self.build_backend.as_ref()
    }
}

impl Default for PyProjectBuildSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Partial `[project]` metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PyProjectProjectMetadata {
    name: Option<String>,
    version: Option<String>,
    dependencies: Vec<PyProjectDependency>,
    optional_dependency_groups: Vec<PyProjectOptionalDependencyGroup>,
    scripts: Vec<PyProjectScript>,
    entry_points: Vec<PyProjectEntryPoint>,
}

impl PyProjectProjectMetadata {
    /// Creates empty project metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: None,
            version: None,
            dependencies: Vec::new(),
            optional_dependency_groups: Vec::new(),
            scripts: Vec::new(),
            entry_points: Vec::new(),
        }
    }

    /// Sets the project name.
    ///
    /// # Errors
    ///
    /// Returns [`PyProjectTextError::Empty`] when `name` is empty after trimming.
    pub fn with_name(mut self, name: &str) -> Result<Self, PyProjectTextError> {
        self.name = Some(non_empty_text(name)?.to_string());
        Ok(self)
    }

    /// Sets the project version.
    ///
    /// # Errors
    ///
    /// Returns [`PyProjectTextError::Empty`] when `version` is empty after trimming.
    pub fn with_version(mut self, version: &str) -> Result<Self, PyProjectTextError> {
        self.version = Some(non_empty_text(version)?.to_string());
        Ok(self)
    }

    /// Adds a dependency.
    #[must_use]
    pub fn with_dependency(mut self, dependency: PyProjectDependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Adds an optional dependency group label.
    #[must_use]
    pub fn with_optional_dependency_group(
        mut self,
        group: PyProjectOptionalDependencyGroup,
    ) -> Self {
        self.optional_dependency_groups.push(group);
        self
    }

    /// Adds a script label.
    #[must_use]
    pub fn with_script(mut self, script: PyProjectScript) -> Self {
        self.scripts.push(script);
        self
    }

    /// Adds an entry point label.
    #[must_use]
    pub fn with_entry_point(mut self, entry_point: PyProjectEntryPoint) -> Self {
        self.entry_points.push(entry_point);
        self
    }

    /// Returns the project name when present.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the project version when present.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[PyProjectDependency] {
        &self.dependencies
    }

    /// Returns optional dependency group labels.
    #[must_use]
    pub fn optional_dependency_groups(&self) -> &[PyProjectOptionalDependencyGroup] {
        &self.optional_dependency_groups
    }

    /// Returns script labels.
    #[must_use]
    pub fn scripts(&self) -> &[PyProjectScript] {
        &self.scripts
    }

    /// Returns entry point labels.
    #[must_use]
    pub fn entry_points(&self) -> &[PyProjectEntryPoint] {
        &self.entry_points
    }
}

/// Common pyproject config file labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyProjectConfigFile {
    PyProjectToml,
}

impl PyProjectConfigFile {
    /// Returns the config file label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        "pyproject.toml"
    }
}

impl fmt::Display for PyProjectConfigFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PyProjectConfigFile {
    type Err = PyProjectTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "pyprojecttoml" => Ok(Self::PyProjectToml),
            _ => Err(PyProjectTextError::UnknownLabel),
        }
    }
}

/// Common Python build backends.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyProjectBuildBackend {
    SetuptoolsBuildMeta,
    HatchlingBuild,
    PoetryCore,
    FlitCore,
    Maturin,
    ScikitBuildCore,
    Custom(String),
}

impl PyProjectBuildBackend {
    /// Returns the normalized build backend label.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::SetuptoolsBuildMeta => "setuptools.build_meta",
            Self::HatchlingBuild => "hatchling.build",
            Self::PoetryCore => "poetry.core.masonry.api",
            Self::FlitCore => "flit_core.buildapi",
            Self::Maturin => "maturin",
            Self::ScikitBuildCore => "scikit_build_core.build",
            Self::Custom(label) => label.as_str(),
        }
    }
}

impl fmt::Display for PyProjectBuildBackend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PyProjectBuildBackend {
    type Err = PyProjectTextError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = non_empty_text(input)?;
        Ok(match trimmed {
            "setuptools.build_meta" => Self::SetuptoolsBuildMeta,
            "hatchling.build" => Self::HatchlingBuild,
            "poetry.core.masonry.api" => Self::PoetryCore,
            "flit_core.buildapi" => Self::FlitCore,
            "maturin" => Self::Maturin,
            "scikit_build_core.build" => Self::ScikitBuildCore,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

/// Error returned when pyproject metadata text is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PyProjectTextError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for PyProjectTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pyproject metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown pyproject metadata label"),
        }
    }
}

impl Error for PyProjectTextError {}

fn non_empty_text(input: &str) -> Result<&str, PyProjectTextError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PyProjectTextError::Empty)
    } else {
        Ok(trimmed)
    }
}

fn normalized_label(input: &str) -> Result<String, PyProjectTextError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PyProjectTextError::Empty)
    } else {
        Ok(trimmed
            .to_ascii_lowercase()
            .replace(['-', '_', '.', ' '], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PyProject, PyProjectBuildBackend, PyProjectBuildSystem, PyProjectConfigFile,
        PyProjectDependency, PyProjectProjectMetadata, PyProjectTextError,
    };

    #[test]
    fn models_partial_pyproject_metadata() -> Result<(), PyProjectTextError> {
        let project = PyProjectProjectMetadata::new()
            .with_name("demo")?
            .with_version("0.1.0")?
            .with_dependency(PyProjectDependency::new("requests>=2")?);
        let build_system = PyProjectBuildSystem::new()
            .with_requirement(PyProjectDependency::new("hatchling")?)
            .with_build_backend(PyProjectBuildBackend::HatchlingBuild);
        let pyproject = PyProject::new()
            .with_project(project)
            .with_build_system(build_system);

        assert_eq!(pyproject.project_name(), Some("demo"));
        assert_eq!(pyproject.project_version(), Some("0.1.0"));
        assert_eq!(pyproject.dependencies()[0].as_str(), "requests>=2");
        assert_eq!(
            pyproject.build_backend(),
            Some(&PyProjectBuildBackend::HatchlingBuild)
        );
        Ok(())
    }

    #[test]
    fn parses_known_and_custom_backends() -> Result<(), PyProjectTextError> {
        assert_eq!(
            "maturin".parse::<PyProjectBuildBackend>()?,
            PyProjectBuildBackend::Maturin
        );
        assert_eq!(
            "pyproject.toml".parse::<PyProjectConfigFile>()?,
            PyProjectConfigFile::PyProjectToml
        );
        assert_eq!(
            PyProjectConfigFile::PyProjectToml.to_string(),
            "pyproject.toml"
        );
        assert_eq!(
            "custom.backend".parse::<PyProjectBuildBackend>()?.as_str(),
            "custom.backend"
        );
        Ok(())
    }
}
