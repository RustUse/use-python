use use_python_version::{PythonImplementation, PythonVersion};

fn main() -> Result<(), use_python_version::PythonVersionParseError> {
    let version: PythonVersion = "v3.13.0".parse()?;

    assert_eq!(version.to_string(), "3.13.0");
    assert!(version.is_python3());
    assert_eq!(PythonImplementation::CPython.as_str(), "cpython");
    Ok(())
}
