
### SYX 0.2.2

- fix: point `gen_ir_capabilities.py` at `mir_types.rs` after MirOp extraction so `--check` matches generated tables

- fix: drop gitignored `examples/test.sy` from the Examples VM allowlist (CI clean checkouts lack it) and refuse gitignored allowlist entries

- fix: type `SyxHost.*` host-call returns from the host registry (not as `SyxHost`) so `h < 0` stays numeric; mangle instance twins of static same-name methods as `{Type}__{name}` so `FilePath.exists` no longer stack-overflows

- refactor: Inject public imported structs before semantic analysis to maintain instance method typing; enhance MIR handling for struct method chains

- refactor: remove outdated credibility tests and contracts to streamline codebase; enhance documentation for clarity on remaining contracts and census processes

- refactor: remove outdated credibility tests and contracts related to exit-open and LSP-book claims to streamline the codebase; introduce new tests for dual-accept `catch let` and `func...async` syntax

- feat: add support for dynamic target triple in LLVM IR emission

- feat: implement MIR lowering for loop control statements including `break`, `continue`, and `repeat-while`