# use-python-version

Python version and implementation primitives for `RustUse`.

## Experimental

`use-python-version` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_python_version::{PythonImplementation, PythonVersion};

let version: PythonVersion = "Python 3.12.1".parse()?;

assert_eq!(version.major(), 3);
assert_eq!(version.minor(), Some(12));
assert!(version.is_python3());
assert_eq!(PythonImplementation::CPython.as_str(), "cpython");
# Ok::<(), use_python_version::PythonVersionParseError>(())
```

## Scope

- Lightweight Python version components and parsing.
- Python implementation labels.
- Compatibility, ABI, and platform tag text newtypes.

## Non-goals

- Full PEP 440 parsing.
- Support-window or EOL-date policy.
- Querying installed Python interpreters.

## License

Licensed under either Apache-2.0 or MIT.
