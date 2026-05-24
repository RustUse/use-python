use use_python_module::{PythonImportKind, PythonModuleName};

fn main() -> Result<(), use_python_module::PythonModuleNameError> {
    let module = PythonModuleName::new("package.submodule")?;

    assert_eq!(module.segments(), vec!["package", "submodule"]);
    assert_eq!(PythonImportKind::Absolute.as_str(), "absolute");
    Ok(())
}
