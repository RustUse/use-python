use use_pip::{PipCommand, PipRequirement, is_requirements_file};

fn main() -> Result<(), use_pip::PipTextError> {
    let command: PipCommand = "install".parse()?;
    let requirement = PipRequirement::new("requests>=2")?;

    assert_eq!(command.to_string(), "install");
    assert_eq!(requirement.as_str(), "requests>=2");
    assert!(is_requirements_file("-r requirements.txt"));
    Ok(())
}
