use use_pyproject::{PyProject, PyProjectDependency, PyProjectProjectMetadata};

fn main() -> Result<(), use_pyproject::PyProjectTextError> {
    let project = PyProjectProjectMetadata::new()
        .with_name("demo")?
        .with_version("0.1.0")?
        .with_dependency(PyProjectDependency::new("requests>=2")?);
    let pyproject = PyProject::new().with_project(project);

    assert_eq!(pyproject.project_name(), Some("demo"));
    assert_eq!(pyproject.dependencies()[0].as_str(), "requests>=2");
    Ok(())
}
