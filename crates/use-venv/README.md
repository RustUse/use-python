# use-venv

Python virtual environment metadata primitives for `RustUse`.

## Experimental

`use-venv` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_venv::{PythonVirtualEnv, PythonVirtualEnvKind, PythonVirtualEnvName};

let env = PythonVirtualEnv::new(PythonVirtualEnvName::new(".venv")?, PythonVirtualEnvKind::Venv);

assert_eq!(env.name().as_str(), ".venv");
assert_eq!(env.kind().as_str(), "venv");
# Ok::<(), use_venv::PythonVirtualEnvError>(())
```

## Scope

- Virtual environment name, kind, path, activation shell, and environment variable labels.
- Lightweight name validation.
- Passive environment metadata.

## Non-goals

- Creating, modifying, activating, or deleting virtual environments.
- Shelling out to `python`, `venv`, `virtualenv`, `conda`, `poetry`, `uv`, or `pipenv`.
- Managing dependencies or interpreter installations.

## License

Licensed under either Apache-2.0 or MIT.
