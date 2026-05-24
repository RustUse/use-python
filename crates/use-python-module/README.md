# use-python-module

Python module, package, import, and path-label primitives for `RustUse`.

## Experimental

`use-python-module` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_python_module::{PythonImportKind, PythonModuleName};

let module = PythonModuleName::new("package.submodule")?;

assert_eq!(module.segments(), vec!["package", "submodule"]);
assert_eq!(PythonImportKind::Absolute.as_str(), "absolute");
# Ok::<(), use_python_module::PythonModuleNameError>(())
```

## Scope

- Lightly validated dotted module, package, and import names.
- Import kind, module path, file kind, and package layout metadata.
- ASCII identifier validation for each dotted segment.

## Non-goals

- Parsing Python source code or import statements.
- Resolving modules on disk.
- Complete package discovery or namespace-package behavior.

## License

Licensed under either Apache-2.0 or MIT.
