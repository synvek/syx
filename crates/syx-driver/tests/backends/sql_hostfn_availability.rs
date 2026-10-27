//! ADR-187 / M2 W43: EC-M2-4 Sql HostFn availability first nail (matrix honesty).
// was: ec_m2_4_sql_availability

#[test]
fn ec_m2_4_sql_availability_host_fn_overrides() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
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
    ] {
        assert!(
            toml.contains(&format!("name = \"{name}\"")),
            "host_fn_capabilities.toml must override {name}"
        );
    }
    for name in ["SqlOpen", "SqlExecOn"] {
        let block = toml
            .split(&format!("name = \"{name}\""))
            .nth(1)
            .unwrap_or("")
            .split("[[host]]")
            .next()
            .unwrap_or("");
        assert!(
            block.contains("vm = \"real\""),
            "{name} must declare vm=real: {block}"
        );
    }
    assert!(
        !toml.contains("name = \"OrmSave\""),
        "OrmSave must stay deferred this window"
    );
    assert!(
        !toml.contains("name = \"SqlMigrate\""),
        "SqlMigrate must stay deferred this window"
    );
    assert!(
        !toml.contains("name = \"SqlBindMulti\""),
        "SqlBindMulti must stay deferred this window"
    );
}

#[test]
fn ec_m2_4_sql_availability_product_data_sqlite() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"data.sqlite\""),
        "capabilities.toml must declare data.sqlite"
    );
    assert!(
        caps.contains("ADR-187") || caps.contains("availability"),
        "data.sqlite notes must cite ADR-187"
    );
    assert!(
        caps.contains("ec_m2_4_sql_availability"),
        "data.sqlite tests must list ec_m2_4_sql_availability"
    );
}

#[test]
fn ec_m2_4_sql_availability_generated_md_not_unsupported() {
    let md = include_str!("../../../../books/shared/host-fn-capabilities.md");
    for name in ["SqlOpen", "SqlExecOn"] {
        let row = md
            .lines()
            .find(|l| l.starts_with(&format!("| {name} |")))
            .unwrap_or_else(|| panic!("{name} row missing"));
        assert!(
            !row.contains("| unsupported | unsupported |"),
            "{name} must not be all-unsupported: {row}"
        );
        assert!(
            row.contains("| real |"),
            "{name} must list a real backend: {row}"
        );
    }
}

#[test]
fn ec_m2_4_sql_availability_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_sql_availability_slice"),
        "inventory must contain ec_m2_4_sql_availability_slice"
    );
    assert!(
        inv.contains("ADR-187") || inv.contains("signed-ADR-187"),
        "inventory must note ADR-187"
    );
}
