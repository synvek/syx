# Core builtin `.sy` templates

Editable sources for **generated** numeric authority files under
`packages/core/builtins/numeric/`.

- Edit `*.sy.in` / `fragments/*.sy.in`, then run `cargo build -p syx-driver`.
- Placeholders are `{{NAME}}` (simple string substitution in `syx-driver/build.rs`).
- Hand-authored types (Array, String, Bool, Result, protocols, …) live beside
  this tree and are **not** overwritten by the build script.
