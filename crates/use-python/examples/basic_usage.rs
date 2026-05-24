use use_python::{
    PipRequirement, PytestNodeId, PythonIdentifier, PythonImplementation, PythonPrimitiveValue,
    PythonVersion,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version: PythonVersion = "Python 3.12.1".parse()?;
    let identifier = PythonIdentifier::new("async_task")?;
    let requirement = PipRequirement::new("requests>=2")?;
    let node_id = PytestNodeId::new("tests/test_app.py::test_smoke")?;

    assert!(version.is_python3());
    assert_eq!(identifier.as_str(), "async_task");
    assert_eq!(PythonImplementation::CPython.as_str(), "cpython");
    assert_eq!(PythonPrimitiveValue::None.type_name(), "NoneType");
    assert!(!requirement.is_editable());
    assert!(node_id.has_scope_separator());
    Ok(())
}
