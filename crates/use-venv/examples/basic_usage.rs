use use_venv::{PythonVirtualEnv, PythonVirtualEnvKind, PythonVirtualEnvName};

fn main() -> Result<(), use_venv::PythonVirtualEnvError> {
    let env = PythonVirtualEnv::new(
        PythonVirtualEnvName::new(".venv")?,
        PythonVirtualEnvKind::Venv,
    );

    assert_eq!(env.name().as_str(), ".venv");
    assert_eq!(env.kind().as_str(), "venv");
    Ok(())
}
