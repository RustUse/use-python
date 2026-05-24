# use-python

Facade crate for the focused `RustUse` Python ecosystem primitives.

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
assert!(node_id.has_scope_separator());
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Thin re-exports over the focused crates in this workspace.
- One dependency when consumer ergonomics matter more than the narrowest possible dependency graph.
- Primitive metadata and validation helpers for Python ecosystem concepts.

## Non-goals

- Python runtime behavior or object semantics.
- Full source, TOML, requirement, lockfile, or test-file parsing.
- Shelling out to Python ecosystem tools.
- Package resolution, installation, virtual environment creation, linting, formatting, testing, or project generation.

## License

Licensed under either Apache-2.0 or MIT.
