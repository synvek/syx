//! ADR-201 / M2 W57: EC-M2-4 Sql native/WASM availability matrix first deepening.
//! Wave A W272 / ADR-616: Sql core HostFn wasm8.
//! Wave A W273 / ADR-617: Sql residual HostFn wasm9 收官 (Sql* HostFn wasm 收官).
//! Wave A W280 / ADR-624: data.sqlite product wasm=real.
//! Wave A W292 / ADR-636: data.sqlite product native=real (status still partial).
// was: ec_m2_4_sql_native_wasm_matrix

const SQL_HOSTS: &[&str] = &[
    "SqlOpen",
    "SqlClose",
    "SqlExecOn",
    "SqlQueryOn",
    "SqlExec",
    "SqlQuery",
    "SqlPrepare",
    "SqlBindStep",
    "SqlBeginTransaction",
    "SqlCommitTransaction",
    "SqlRollback",
    "SqlPrepareOn",
    "SqlBindStepOn",
    "SqlBindPreparedOn",
    "SqlBeginTransactionOn",
    "SqlCommitTransactionOn",
    "SqlRollbackTransactionOn",
];

const W272_WASM_REAL: &[&str] = &[
    "SqlOpen",
    "SqlClose",
    "SqlExec",
    "SqlQuery",
    "SqlPrepare",
    "SqlBindStep",
    "SqlExecOn",
    "SqlQueryOn",
];

const W273_WASM_REAL: &[&str] = &[
    "SqlPrepareOn",
    "SqlBindStepOn",
    "SqlBindPreparedOn",
    "SqlBeginTransaction",
    "SqlCommitTransaction",
    "SqlRollback",
    "SqlBeginTransactionOn",
    "SqlCommitTransactionOn",
    "SqlRollbackTransactionOn",
];

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn is_w272_wasm_real(name: &str) -> bool {
    W272_WASM_REAL.contains(&name)
}

fn is_w273_wasm_real(name: &str) -> bool {
    W273_WASM_REAL.contains(&name)
}

#[test]
fn ec_m2_4_sql_native_wasm_matrix_host_fn_columns() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in SQL_HOSTS {
        let block = host_block(toml, name);
        assert!(
            block.contains("vm = \"real\""),
            "{name} must keep vm=real: {block}"
        );
        assert!(
            block.contains("cranelift = \"unsupported\""),
            "{name} must declare cranelift=unsupported: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real: {block}"
        );
        // Wave A W272/W273: all listed Sql* HostFn wasm=real (Sql* wasm 收官).
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-616/617): {block}"
        );
        if is_w273_wasm_real(name) {
            assert!(
                block.contains("ADR-617"),
                "{name} notes must cite ADR-617: {block}"
            );
        } else if is_w272_wasm_real(name) {
            assert!(
                block.contains("ADR-616"),
                "{name} notes must cite ADR-616: {block}"
            );
        }
    }
}

#[test]
fn ec_m2_4_sql_native_wasm_matrix_generated_md_columns() {
    let md = include_str!("../../../../books/shared/host-fn-capabilities.md");
    for name in [
        "SqlOpen",
        "SqlClose",
        "SqlExec",
        "SqlQueryOn",
        "SqlPrepareOn",
        "SqlBeginTransaction",
        "SqlBeginTransactionOn",
        "SqlRollback",
    ] {
        let row = md
            .lines()
            .find(|l| l.starts_with(&format!("| {name} |")))
            .unwrap_or_else(|| panic!("{name} row missing"));
        assert!(
            row.contains("| unsupported |"),
            "{name} row must list unsupported: {row}"
        );
        assert!(
            row.contains("| real | real | real | real |"),
            "{name} row must list wasm=real (ADR-616/617): {row}"
        );
        assert!(
            !row.contains("| host-assisted |"),
            "{name} must not list host-assisted after ADR-617: {row}"
        );
    }
}

#[test]
fn ec_m2_4_sql_native_wasm_matrix_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.sqlite");
    assert!(
        block.contains("native = \"real\""),
        "data.sqlite must declare native=real (ADR-636): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "data.sqlite must declare wasm=real (ADR-624): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "data.sqlite must keep status=partial (≠ implemented): {block}"
    );
    assert!(
        block.contains("ADR-636") && block.contains("W292"),
        "data.sqlite notes must cite ADR-636+W292: {block}"
    );
    assert!(
        block.contains("ec_m2_4_sql_native_wasm_matrix"),
        "data.sqlite tests must include ec_m2_4_sql_native_wasm_matrix: {block}"
    );
}

#[test]
fn ec_m2_4_sql_native_wasm_matrix_honesty_fences() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["OrmSave", "SqlMigrate", "SqlBindMulti"] {
        assert!(
            !toml.contains(&format!("name = \"{name}\"")),
            "{name} must remain uncovered (fail-closed default); found override"
        );
    }

    let stubs = include_str!("../../../../crates/syx-rt/src/llvm_stubs.rs");
    assert!(
        !stubs.contains("syx_rt_sql"),
        "llvm_stubs must not claim dedicated syx_rt_sql AOT (no Sql POP stub yet)"
    );

    let oracle = include_str!("../fixtures/differential/suite4/sql_count_select.oracle.json");
    assert!(
        oracle.contains("\"wasm\": true") || oracle.contains("\"wasm\":true"),
        "Suite-4 sql_count_select must still skip wasm: {oracle}"
    );
}

#[test]
fn ec_m2_4_sql_native_wasm_matrix_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_sql_native_wasm_matrix_tier"),
        "inventory must contain ec_m2_4_sql_native_wasm_matrix_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-201") || inv.contains("signed-ADR-201"),
        "inventory must note ADR-201: {inv}"
    );
}

#[test]
fn ec_m2_4_sql_native_wasm_matrix_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire ec_m2_4_sql_native_wasm_matrix"
    );
}
