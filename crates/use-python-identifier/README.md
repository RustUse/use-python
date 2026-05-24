# use-python-identifier

ASCII-safe Python identifier primitives for `RustUse`.

## Experimental

`use-python-identifier` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_python_identifier::{PythonIdentifier, is_valid_ascii_python_identifier};

let identifier = PythonIdentifier::new("async_task")?;

assert_eq!(identifier.as_str(), "async_task");
assert!(is_valid_ascii_python_identifier("_value_1"));
# Ok::<(), use_python_identifier::PythonIdentifierError>(())
```

## Scope

- Ordinary ASCII-safe Python identifier validation.
- Hard Python keyword rejection through `use-python-keyword`.
- Dunder-name and private-name metadata helpers.

## Non-goals

- Complete Unicode Python identifier validation.
- Soft-keyword context analysis.
- Python source-code parsing.

Unicode-complete Python identifier validation is future work.

## License

Licensed under either Apache-2.0 or MIT.
