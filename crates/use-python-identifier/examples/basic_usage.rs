use use_python_identifier::{PythonIdentifier, is_valid_ascii_python_identifier};

fn main() -> Result<(), use_python_identifier::PythonIdentifierError> {
    let identifier = PythonIdentifier::new("async_task")?;

    assert_eq!(identifier.as_str(), "async_task");
    assert!(is_valid_ascii_python_identifier("_value_1"));
    Ok(())
}
