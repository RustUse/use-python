# use-python-keyword

Python keyword and soft-keyword primitives for `RustUse`.

## Experimental

`use-python-keyword` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_python_keyword::{PythonKeyword, is_python_reserved_word};

let keyword: PythonKeyword = "async".parse()?;

assert_eq!(keyword.to_string(), "async");
assert!(is_python_reserved_word("match"));
# Ok::<(), use_python_keyword::PythonKeywordParseError>(())
```

## Scope

- Common hard Python keywords.
- Common soft keywords such as `match`, `case`, `type`, and `_`.
- Display and parse helpers that preserve Python source spelling.

## Non-goals

- Parsing Python source code.
- Determining whether a soft keyword is active in a specific grammar context.
- Tracking future Python grammar changes at runtime.

## License

Licensed under either Apache-2.0 or MIT.
