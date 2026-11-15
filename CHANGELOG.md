### SYX 0.2.3
- feat: implement optional statement separators in block statements
- feat: implement `while let` syntax desugaring in parser and add tests
- feat: enhance stdlib MIR handling with caching and fingerprinting
- feat: add support for stdlib MIR embedding and enhance asset management

### SYX 0.2.2

- fix: `syx build --native` resolves prebuilt slim `libsyx_rt.a` / `lib/hosts/*` only — never spawns `cargo` at link time; prebuild via `scripts/build_native_shake_libs.sh` or SDK package
- chore: consolidate syx-vm stdlib_phase83–98/120 hosts into thematic modules (`stdlib_core_hosts`, `stdlib_process_hosts`, `stdlib_encoding_hosts`, `stdlib_json_grpc_hosts`, `stdlib_wire_hosts`, `stdlib_orm_gui_hosts`, `stdlib_gui_sql_hosts/`, `service_hosts`)
- chore: merge syx-macro phase119–138 unit probes into `unit_roles_rb` / `unit_service_di` / `unit_token_pointcut` and drop duplicate nails
- feat: native link-time host shaking — slim `libsyx_rt.a` plus on-demand `syx-host-{sql,http,tls}` from MIR `CallHost` set (no user-facing Cargo features for Sql/Http/Tls); `syx doctor --native-host-shake` / `scripts/native_host_shake_size.sh`
- chore: gitignore generated prelude embeds (`prelude_snapshot.bin` / `prelude_vm_image.rkyv`); CI/release regen via `gen_prelude_snapshot`; `build.rs` writes empty placeholders on clean checkout
- fix: nest XML children + attrs in `__syx_xml_parse`, correct `__syx_xml_xpath(expr, root)` stack order, and soft-skip backends native AOT nails without `--features full-native`
- fix: serialize BGP mesh tests; keep data SQL nails idempotent (`IF NOT EXISTS` / rusqlite reset); Array `.count` on SQL/ORM stdout gates
- chore: install ripgrep before behavioral count audit on CI; lower LSP anti-delete floor to 1700 after credibility-diet (~1788 real nails)
- fix: point `gen_ir_capabilities.py` at `mir_types.rs` after MirOp extraction so `--check` matches generated tables
- fix: drop gitignored `examples/test.sy` from the Examples VM allowlist (CI clean checkouts lack it) and refuse gitignored allowlist entries
- fix: type `SyxHost.*` host-call returns from the host registry (not as `SyxHost`) so `h < 0` stays numeric; mangle instance twins of static same-name methods as `{Type}__{name}` so `FilePath.exists` no longer stack-overflows
- refactor: Inject public imported structs before semantic analysis to maintain instance method typing; enhance MIR handling for struct method chains
- refactor: remove outdated credibility tests and contracts to streamline codebase; enhance documentation for clarity on remaining contracts and census processes
- refactor: remove outdated credibility tests and contracts related to exit-open and LSP-book claims to streamline the codebase; introduce new tests for dual-accept `catch let` and `func...async` syntax
- feat: add support for dynamic target triple in LLVM IR emission
- feat: implement MIR lowering for loop control statements including `break`, `continue`, and `repeat-while`
- fix: enhance type compatibility checks in type_checker.rs
