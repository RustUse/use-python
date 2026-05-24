#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_python_version::*;

pub use use_python_identifier::*;

pub use use_python_keyword::*;

pub use use_python_value::*;

pub use use_python_module::*;

pub use use_pyproject::*;

pub use use_pip::*;

pub use use_uv::*;

pub use use_venv::*;

pub use use_pytest::*;

#[cfg(test)]
mod tests {
    use super::{
        PipRequirement, PyProject, PyProjectProjectMetadata, PytestMarkerName, PythonIdentifier,
        PythonImplementation, PythonKeyword, PythonModuleName, PythonPrimitiveValue, PythonVersion,
        PythonVirtualEnvKind, PythonVirtualEnvName, UvPackageSpec,
    };

    #[test]
    fn facade_exposes_every_child_crate() -> Result<(), Box<dyn std::error::Error>> {
        let version: PythonVersion = "3.12.1".parse()?;
        let identifier = PythonIdentifier::new("async_task")?;
        let module = PythonModuleName::new("package.module")?;
        let project =
            PyProject::new().with_project(PyProjectProjectMetadata::new().with_name("demo")?);
        let requirement = PipRequirement::new("requests>=2")?;
        let package = UvPackageSpec::new("ruff>=0.4")?;
        let env_name = PythonVirtualEnvName::new(".venv")?;
        let marker = PytestMarkerName::new("slow")?;

        assert!(version.is_python3());
        assert_eq!(identifier.as_str(), "async_task");
        assert_eq!(PythonKeyword::Return.as_str(), "return");
        assert_eq!(PythonPrimitiveValue::None.type_name(), "NoneType");
        assert_eq!(module.as_str(), "package.module");
        assert_eq!(project.project_name(), Some("demo"));
        assert_eq!(requirement.as_str(), "requests>=2");
        assert_eq!(package.as_str(), "ruff>=0.4");
        assert_eq!(env_name.as_str(), ".venv");
        assert_eq!(marker.as_str(), "slow");
        assert_eq!(PythonImplementation::CPython.as_str(), "cpython");
        assert_eq!(PythonVirtualEnvKind::Venv.as_str(), "venv");
        Ok(())
    }
}
