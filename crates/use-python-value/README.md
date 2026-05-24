# use-python-value

Python-like primitive value metadata for `RustUse`.

## Experimental

`use-python-value` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_python_value::PythonPrimitiveValue;

let value = PythonPrimitiveValue::Int(String::from("12345678901234567890"));

assert_eq!(value.type_name(), "int");
assert!(value.is_numeric());
assert!(value.is_truthy_like());
```

## Scope

- Primitive Python-like value metadata.
- Truthy-like and numeric classification helpers.
- Text storage for large integer literals without arbitrary-precision dependencies.

## Non-goals

- Full Python object semantics.
- Arbitrary-precision math.
- Evaluating Python literals or source code.

## License

Licensed under either Apache-2.0 or MIT.
