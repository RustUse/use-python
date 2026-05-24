use use_uv::{UvCommand, UvLockfile, UvPackageSpec};

fn main() -> Result<(), use_uv::UvTextError> {
    let command: UvCommand = "sync".parse()?;
    let package = UvPackageSpec::new("ruff>=0.4")?;

    assert_eq!(command.to_string(), "sync");
    assert_eq!(package.as_str(), "ruff>=0.4");
    assert_eq!(UvLockfile::UvLock.as_str(), "uv.lock");
    Ok(())
}
