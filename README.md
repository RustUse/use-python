# RustUse/use-python

Composable Python ecosystem primitives for `RustUse`.

`use-python` is a focused RustUse set for representing Python language, packaging, virtual environment, and testing concepts as small Rust primitives. It is not a Python interpreter, parser, package manager, virtual environment manager, linter, formatter, test runner, project generator, dependency resolver, or registry client.

## Workspace crates

| Crate                   | Path                            | Purpose                                                    |
| ----------------------- | ------------------------------- | ---------------------------------------------------------- |
| `use-python`            | `crates/use-python/`            | Facade over the focused Python ecosystem crates            |
| `use-python-version`    | `crates/use-python-version/`    | Python version, implementation, and tag primitives         |
| `use-python-identifier` | `crates/use-python-identifier/` | ASCII-safe Python identifier validation                    |
| `use-python-keyword`    | `crates/use-python-keyword/`    | Python keyword, soft-keyword, and reserved-word vocabulary |
| `use-python-value`      | `crates/use-python-value/`      | Python-like primitive value metadata                       |
| `use-python-module`     | `crates/use-python-module/`     | Module, package, import, and file-kind primitives          |
| `use-pyproject`         | `crates/use-pyproject/`         | Partial practical `pyproject.toml` metadata primitives     |
| `use-pip`               | `crates/use-pip/`               | pip command, requirement, index, and install metadata      |
| `use-uv`                | `crates/use-uv/`                | uv command, lockfile, config, and package metadata         |
| `use-venv`              | `crates/use-venv/`              | Python virtual environment metadata primitives             |
| `use-pytest`            | `crates/use-pytest/`            | pytest node, marker, fixture, config, and outcome metadata |

## Experimental

`use-python` is experimental while the workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_python::{
    PipRequirement, PythonIdentifier, PythonImplementation, PythonPrimitiveValue, PythonVersion,
    PytestNodeId,
};

let version: PythonVersion = "Python 3.12.1".parse()?;
let identifier = PythonIdentifier::new("async_task")?;
let requirement = PipRequirement::new("requests>=2")?;
let node_id = PytestNodeId::new("tests/test_app.py::test_smoke")?;

assert!(version.is_python3());
assert_eq!(identifier.as_str(), "async_task");
assert_eq!(PythonImplementation::CPython.as_str(), "cpython");
assert_eq!(PythonPrimitiveValue::None.type_name(), "NoneType");
assert!(!requirement.is_editable());
assert!(node_id.as_str().contains("::"));
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Small, dependency-light Rust primitives for Python ecosystem concepts.
- Lightweight validation for names, versions, commands, files, options, and metadata shapes.
- Facade access through `use-python` plus narrow focused crates for direct use.

## Non-goals

- Running Python code or implementing Python object semantics.
- Parsing complete Python source, `pyproject.toml`, requirements files, lockfiles, or test files.
- Shelling out to `python`, `pip`, `uv`, `pytest`, `venv`, `poetry`, `conda`, or any other external tool.
- Creating virtual environments or generating Python projects.
- Implementing package resolution, package installation, linting, formatting, or test execution.
- Contacting `PyPI` or any package registry.

## Development

```sh
cargo fmt --all --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
