# use-uv

uv command and project metadata primitives for `RustUse`.

## Experimental

`use-uv` is experimental while `use-python` remains below `0.3.0`.

## Example

```rust
use use_uv::{UvCommand, UvLockfile, UvPackageSpec};

let command: UvCommand = "sync".parse()?;
let package = UvPackageSpec::new("ruff>=0.4")?;

assert_eq!(command.to_string(), "sync");
assert_eq!(package.as_str(), "ruff>=0.4");
assert_eq!(UvLockfile::UvLock.as_str(), "uv.lock");
# Ok::<(), use_uv::UvTextError>(())
```

## Scope

- uv command and subcommand labels.
- uv lockfile and config file labels.
- Workspace and package spec metadata text.

## Non-goals

- Shelling out to uv.
- Implementing dependency resolution.
- Contacting package indexes.
- Managing Python installations or virtual environments.

## License

Licensed under either Apache-2.0 or MIT.
