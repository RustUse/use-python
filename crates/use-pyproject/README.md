# use-pyproject

Partial practical `pyproject.toml` metadata primitives for `RustUse`.

## Experimental

`use-pyproject` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_pyproject::{PyProject, PyProjectDependency, PyProjectProjectMetadata};

let project = PyProjectProjectMetadata::new()
    .with_name("demo")?
    .with_version("0.1.0")?
    .with_dependency(PyProjectDependency::new("requests>=2")?);
let pyproject = PyProject::new().with_project(project);

assert_eq!(pyproject.project_name(), Some("demo"));
assert_eq!(pyproject.dependencies()[0].as_str(), "requests>=2");
# Ok::<(), use_pyproject::PyProjectTextError>(())
```

## Scope

- Partial project, dependency, script, entry point, and build-system metadata.
- Build backend labels for common Python build backends.
- Tool section labels as simple metadata.

## Non-goals

- Full TOML parsing or serialization.
- Complete `pyproject.toml` coverage.
- Dependency resolution or package building.

## License

Licensed under either Apache-2.0 or MIT.
