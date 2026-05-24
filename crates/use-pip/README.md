# use-pip

pip command and install metadata primitives for `RustUse`.

## Experimental

`use-pip` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_pip::{PipCommand, PipRequirement, is_requirements_file};

let command: PipCommand = "install".parse()?;
let requirement = PipRequirement::new("requests>=2")?;

assert_eq!(command.to_string(), "install");
assert_eq!(requirement.as_str(), "requests>=2");
assert!(is_requirements_file("-r requirements.txt"));
# Ok::<(), use_pip::PipTextError>(())
```

## Scope

- Common pip command labels.
- Requirement, package spec, index URL, editable install, and requirement-file metadata.
- Lightweight non-empty validation.

## Non-goals

- Shelling out to pip.
- Contacting `PyPI` or package indexes.
- Installing packages or resolving dependencies.
- Fully parsing requirement specifiers.

## License

Licensed under either Apache-2.0 or MIT.
