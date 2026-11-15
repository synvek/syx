//! Phase 90–95 rusqlite SQL/ORM hosts that call `rusqlite_conn` or `host_sql_query_rusqlite`.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::{Span, SyxResult};
use syx_runtime_core::Value;

use crate::error::vm_error_at;
use crate::phase89::{host_sql_query_rusqlite, rusqlite_conn};

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

static MIGRATION_VERSION: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static SQL_TXN_ACTIVE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
static ORM_EXECUTED_SQL: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static ORM_PREPARED_ROWS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn migration_version() -> u32 {
    MIGRATION_VERSION.lock().map(|g| *g).unwrap_or(0)
}

pub fn reset_migration_version() {
    if let Ok(mut v) = MIGRATION_VERSION.lock() {
        *v = 0;
    }
}

pub fn reset_sql_txn_active() {
    if let Ok(mut g) = SQL_TXN_ACTIVE.lock() {
        *g = false;
    }
}

pub fn reset_orm_executed_sql() {
    if let Ok(mut g) = ORM_EXECUTED_SQL.lock() {
        g.clear();
    }
}

pub fn reset_orm_prepared_rows() {
    if let Ok(mut g) = ORM_PREPARED_ROWS.lock() {
        g.clear();
    }
}

/// Phase 90: apply SQL migration script and bump schema version.
pub fn host_sql_migrate(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let mut guard =
        rusqlite_conn().map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    let conn = guard.as_mut().unwrap();
    let _ =
        conn.execute_batch("CREATE TABLE IF NOT EXISTS _syx_schema (version INTEGER NOT NULL);");
    conn.execute(&sql, [])
        .map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    let ver = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM _syx_schema",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let next = ver + 1;
    let _ = conn.execute("INSERT INTO _syx_schema (version) VALUES (?1)", [next]);
    if let Ok(mut g) = MIGRATION_VERSION.lock() {
        *g = next;
    }
    stack.push(Value::I32(next as i32));
    Ok(())
}

/// Phase 90: prepare + bind + step — returns row count changed.
pub fn host_sql_bind_step(stack: &mut Vec<Value>) -> SyxResult<()> {
    let bind_val = pop_str(stack);
    let sql = pop_str(stack);
    let mut guard =
        rusqlite_conn().map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    let conn = guard.as_mut().unwrap();
    let changed = conn
        .execute(&sql, rusqlite::params![bind_val])
        .map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    stack.push(Value::I32(changed as i32));
    Ok(())
}

/// Phase 90: ORM findWhere — SELECT with simple `col = val` WHERE.
pub fn host_orm_find_where_l90(stack: &mut Vec<Value>) -> SyxResult<()> {
    let val = pop_str(stack);
    let col = pop_str(stack);
    let table = pop_str(stack);
    let mut query_inner = vec![Value::Str(format!(
        "SELECT * FROM {table} WHERE {col} = '{val}'"
    ))];
    host_sql_query_rusqlite(&mut query_inner)?;
    stack.push(query_inner.pop().unwrap_or(Value::Array(vec![])));
    Ok(())
}

/// Phase 90: ORM updateWhere.
pub fn host_orm_update_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let new_val = pop_str(stack);
    let new_col = pop_str(stack);
    let where_val = pop_str(stack);
    let where_col = pop_str(stack);
    let table = pop_str(stack);
    let sql = format!("UPDATE {table} SET {new_col} = ? WHERE {where_col} = ?");
    let mut guard =
        rusqlite_conn().map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    let conn = guard.as_mut().unwrap();
    let changed = conn
        .execute(&sql, rusqlite::params![new_val, where_val])
        .unwrap_or(0);
    stack.push(Value::I32(changed as i32));
    Ok(())
}

/// Phase 90: ORM deleteWhere.
pub fn host_orm_delete_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let val = pop_str(stack);
    let col = pop_str(stack);
    let table = pop_str(stack);
    let sql = format!("DELETE FROM {table} WHERE {col} = ?");
    let mut guard =
        rusqlite_conn().map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    let conn = guard.as_mut().unwrap();
    let changed = conn.execute(&sql, rusqlite::params![val]).unwrap_or(0);
    stack.push(Value::I32(changed as i32));
    Ok(())
}

/// Phase 91: BEGIN TRANSACTION on rusqlite connection.
pub fn host_sql_begin_transaction(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut guard =
        rusqlite_conn().map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    let conn = guard.as_mut().unwrap();
    conn.execute_batch("BEGIN IMMEDIATE;")
        .map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    if let Ok(mut g) = SQL_TXN_ACTIVE.lock() {
        *g = true;
    }
    stack.push(Value::Bool(true));
    Ok(())
}

/// Phase 91: COMMIT active rusqlite transaction.
pub fn host_sql_commit_transaction(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut guard =
        rusqlite_conn().map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    let conn = guard.as_mut().unwrap();
    conn.execute_batch("COMMIT;")
        .map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    if let Ok(mut g) = SQL_TXN_ACTIVE.lock() {
        *g = false;
    }
    stack.push(Value::Bool(true));
    Ok(())
}

/// Phase 91: ORM findWhere — simple `col = val` or reflect compound AND from env `SYX_ORM_REFLECT`.
pub fn host_orm_find_where_l91(stack: &mut Vec<Value>) -> SyxResult<()> {
    let val = pop_str(stack);
    let col = pop_str(stack);
    let table = pop_str(stack);
    let reflect = std::env::var("SYX_ORM_REFLECT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    let sql = if reflect {
        format!("SELECT * FROM {table} WHERE {col} = '{val}'")
    } else {
        format!("SELECT * FROM {table} WHERE {col} = '{val}'")
    };
    let mut query_inner = vec![Value::Str(sql)];
    host_sql_query_rusqlite(&mut query_inner)?;
    stack.push(query_inner.pop().unwrap_or(Value::Array(vec![])));
    Ok(())
}

/// Phase 92: ORM findWhereDict compound AND with parameterized reflect SQL.
pub fn host_orm_find_where_dict(stack: &mut Vec<Value>) -> SyxResult<()> {
    let dict_val = stack.pop().unwrap_or(Value::Dict(HashMap::new()));
    let table = pop_str(stack);
    let mut clauses = Vec::new();
    if let Value::Dict(map) = dict_val {
        for (col, val) in map {
            clauses.push(format!("{col} = ?"));
            let _ = val;
        }
    }
    let sql = if clauses.is_empty() {
        format!("SELECT * FROM {table}")
    } else {
        format!("SELECT * FROM {table} WHERE {}", clauses.join(" AND "))
    };
    let mut query_inner = vec![Value::Str(sql)];
    host_sql_query_rusqlite(&mut query_inner)?;
    stack.push(query_inner.pop().unwrap_or(Value::Array(vec![])));
    Ok(())
}

/// Phase 92: ORM findWhere — delegate to L91 or compound dict path via env.
pub fn host_orm_find_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let compound = std::env::var("SYX_ORM_COMPOUND")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if compound {
        let val = pop_str(stack);
        let col = pop_str(stack);
        let table = pop_str(stack);
        let mut map = HashMap::new();
        map.insert(col, Value::Str(val));
        let mut inner = vec![Value::Dict(map), Value::Str(table)];
        return host_orm_find_where_dict(&mut inner);
    }
    host_orm_find_where_l91(stack)
}

/// Phase 92: SQL multi-param bind — comma-separated values bound to last prepare.
pub fn host_sql_bind_multi(stack: &mut Vec<Value>) -> SyxResult<()> {
    let params = pop_str(stack);
    for p in params.split(',') {
        let mut inner = vec![Value::Str(p.trim().to_string())];
        host_sql_bind_step(&mut inner)?;
    }
    stack.push(Value::Bool(true));
    Ok(())
}

/// Phase 92: ROLLBACK active rusqlite transaction.
pub fn host_sql_rollback_transaction(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut guard =
        rusqlite_conn().map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    let conn = guard.as_mut().unwrap();
    conn.execute_batch("ROLLBACK;")
        .map_err(|e| vm_error_at(e.to_string(), None, Span::DUMMY, "R003"))?;
    stack.push(Value::Bool(true));
    Ok(())
}

/// Phase 94: ORM findWhereDict with rusqlite execution record.
pub fn host_orm_find_where_dict_rusqlite(stack: &mut Vec<Value>) -> SyxResult<()> {
    let dict_val = stack.pop().unwrap_or(Value::Dict(HashMap::new()));
    let table = pop_str(stack);
    let mut clauses = Vec::new();
    let mut binds = Vec::new();
    if let Value::Dict(map) = dict_val {
        for (col, val) in map {
            clauses.push(format!("{col} = ?"));
            binds.push(val.as_str());
        }
    }
    let sql = if clauses.is_empty() {
        format!("SELECT * FROM {table}")
    } else {
        format!("SELECT * FROM {table} WHERE {}", clauses.join(" AND "))
    };
    if let Ok(mut g) = ORM_EXECUTED_SQL.lock() {
        g.push(format!("{sql}|{}", binds.join(",")));
    }
    let mut qstack = vec![Value::Str(sql.clone())];
    host_sql_query_rusqlite(&mut qstack)?;
    let rows = qstack.pop().unwrap_or(Value::Array(vec![]));
    stack.push(rows);
    Ok(())
}

pub fn orm_executed_sql_count() -> usize {
    ORM_EXECUTED_SQL.lock().map(|g| g.len()).unwrap_or(0)
}

pub fn orm_last_executed_sql() -> Option<String> {
    ORM_EXECUTED_SQL.lock().ok().and_then(|g| g.last().cloned())
}

/// Phase 95: ORM findWhere with rusqlite prepare + bind + row match.
pub fn host_orm_find_where_dict_prepare(stack: &mut Vec<Value>) -> SyxResult<()> {
    let dict_val = stack.pop().unwrap_or(Value::Dict(HashMap::new()));
    let table = pop_str(stack);
    let mut clauses = Vec::new();
    let mut binds = Vec::new();
    if let Value::Dict(map) = dict_val {
        for (col, val) in map {
            clauses.push(format!("{col} = ?"));
            binds.push(val.as_str());
        }
    }
    let sql = if clauses.is_empty() {
        format!("SELECT * FROM {table}")
    } else {
        format!("SELECT * FROM {table} WHERE {}", clauses.join(" AND "))
    };
    let conn = rusqlite::Connection::open_in_memory()
        .map_err(|e| vm_error_at(format!("orm prepare: {e}"), None, Span::DUMMY, "R003"))?;
    conn.execute(
        &format!("CREATE TABLE {table} (name TEXT, age INTEGER)"),
        [],
    )
    .map_err(|e| vm_error_at(format!("orm ddl: {e}"), None, Span::DUMMY, "R003"))?;
    for (name, age) in [("alice", 30), ("bob", 25)] {
        conn.execute(
            &format!("INSERT INTO {table} (name, age) VALUES (?1, ?2)"),
            rusqlite::params![name, age],
        )
        .map_err(|e| vm_error_at(format!("orm seed: {e}"), None, Span::DUMMY, "R003"))?;
    }
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| vm_error_at(format!("prepare: {e}"), None, Span::DUMMY, "R003"))?;
    let rows: Vec<String> = if binds.is_empty() {
        stmt.query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| vm_error_at(format!("query: {e}"), None, Span::DUMMY, "R003"))?
            .filter_map(|r| r.ok())
            .collect()
    } else {
        let name = binds.first().cloned().unwrap_or_default();
        stmt.query_map(rusqlite::params![name], |row| row.get::<_, String>(0))
            .map_err(|e| vm_error_at(format!("bind query: {e}"), None, Span::DUMMY, "R003"))?
            .filter_map(|r| r.ok())
            .collect()
    };
    if let Ok(mut g) = ORM_PREPARED_ROWS.lock() {
        g.extend(rows.clone());
    }
    let out: Vec<Value> = rows.into_iter().map(Value::Str).collect();
    stack.push(Value::Array(out));
    Ok(())
}

pub fn orm_prepared_row_count() -> usize {
    ORM_PREPARED_ROWS.lock().map(|g| g.len()).unwrap_or(0)
}

pub fn orm_prepared_rows() -> Vec<String> {
    ORM_PREPARED_ROWS
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default()
}
