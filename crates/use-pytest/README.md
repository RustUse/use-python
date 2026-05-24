# use-pytest

pytest testing metadata primitives for `RustUse`.

## Experimental

`use-pytest` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_pytest::{PytestMarkerName, PytestNodeId, PytestOutcome};

let marker = PytestMarkerName::new("slow")?;
let node_id = PytestNodeId::new("tests/test_app.py::test_smoke")?;

assert_eq!(marker.as_str(), "slow");
assert!(node_id.has_scope_separator());
assert_eq!(PytestOutcome::Passed.as_str(), "passed");
# Ok::<(), use_pytest::PytestNameError>(())
```

## Scope

- Test, marker, fixture, and node ID metadata.
- Config file, outcome, fixture scope, and file-kind labels.
- Light ASCII identifier validation where appropriate.

## Non-goals

- Executing pytest.
- Parsing Python test files.
- Collecting tests or interpreting pytest plugins.

## License

Licensed under either Apache-2.0 or MIT.
