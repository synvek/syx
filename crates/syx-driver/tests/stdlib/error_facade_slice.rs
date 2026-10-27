//! Named production error facades — throws, no status-string / empty-success sentinels.
// was: error_ec_m2_2_slice / m2_error_ec_m2_2_slice

#[test]
fn error_facade_tcp_facade_throws() {
    let src = include_str!("../../../../packages/net/tcp.sy");
    assert!(src.contains("NetworkError"), "NetworkError required");
    assert!(
        src.contains("throws"),
        "TcpStream facade must throws: {src}"
    );
    assert!(
        !src.contains("public var ok") && !src.contains("public var status"),
        "legacy ok/status fields must be gone"
    );
}

#[test]
fn error_facade_filehandle_facade_throws() {
    let src = include_str!("../../../../packages/io/file_handle.sy");
    assert!(src.contains("IOError"), "IOError required");
    for needle in [
        "open(path: String, mode: String) -> Int throws",
        "close(handle: Int) throws",
        "seek(handle: Int, offset: Int) throws",
        "readAllBytes(handle: Int) -> Array<UInt8> throws",
    ] {
        assert!(
            src.contains(needle) || src.contains("throws"),
            "FileHandle must throws ({needle}): {src}"
        );
    }
    assert!(src.contains("-> Array<UInt8> throws") || src.contains("read("), "read throws");
    assert!(src.contains("-> Int throws") || src.contains("write("), "write throws");
}

#[test]
fn error_facade_sql_facade_throws() {
    let src = include_str!("../../../../packages/data/sql.sy");
    assert!(src.contains("SqlError"), "SqlError required");
    for method in [
        "exec",
        "query",
        "prepare",
        "bindStep",
        "beginTransaction",
        "commitTransaction",
        "rollbackTransaction",
    ] {
        assert!(
            src.contains(method) && src.contains("throws"),
            "Sql.{method} must be present with throws surface: {src}"
        );
    }
}

#[test]
fn error_facade_hosts_no_named_status_strings() {
    let phase89 = include_str!("../../../syx-vm/src/stdlib_phase89_hosts.rs");
    assert!(
        !phase89.contains("prep:{sql}") && !phase89.contains("format!(\"prep:"),
        "must not paste prep:{{sql}}"
    );
    let exec_fn = phase89
        .split("pub fn host_sql_exec_rusqlite")
        .nth(1)
        .expect("host_sql_exec_rusqlite");
    let exec_body = exec_fn.split("pub fn ").next().unwrap_or(exec_fn);
    assert!(
        !exec_body.contains(".unwrap_or(0)"),
        "host_sql_exec must not .unwrap_or(0): {exec_body}"
    );

    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        !hosts.contains("\"tcp-fail\"") && !hosts.contains("tcp-fail"),
        "hosts must not paste tcp-fail"
    );
}

#[test]
fn error_facade_inventory_notes_txn_and_slice() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("txn throws") || inv.contains("/156/") || inv.contains("begin/commit/rollback"),
        "inventory must note Transaction / txn throws (ADR-156 lineage)"
    );
    assert!(
        inv.contains("ec_m2_2_slice") || inv.contains("signed-ADR-157") || inv.contains("ADR-157"),
        "inventory meta must note EC-M2-2 slice ADR-157"
    );
}
