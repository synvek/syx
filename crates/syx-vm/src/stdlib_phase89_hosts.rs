//! Phase 89: rusqlite SQL, ORM MVP, XML XPath, Codable L6 schema, GUI loopback hosts.

use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

fn value_as_map(value: &Value) -> Option<HashMap<String, Value>> {
    match Value::resolve_heap_slot(value) {
        Value::Object(map) | Value::Dict(map) => Some(map),
        _ => None,
    }
}

/// M2 W16 / ADR-160: multi-Connection store. Conn **0** = implicit default for legacy static Sql.*.
static RUSQLITE_CONNS: LazyLock<Mutex<HashMap<i32, rusqlite::Connection>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static NEXT_CONN_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(1);

/// ADR-245 / M2-S8-01c: thin connection pool over `RUSQLITE_CONNS`.
struct SqlPool {
    dsn: String,
    max: usize,
    free: Vec<i32>,
    checked_out: HashSet<i32>,
}

static SQL_POOLS: LazyLock<Mutex<HashMap<i32, SqlPool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static NEXT_POOL_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(1);

/// ADR-247 / M2-S8-03b: explicit-conn open txn set (`beginTransactionOn` success).
/// Does **not** touch legacy conn-0 `SQL_TXN_ACTIVE` (phase91).
static SQL_TXN_OPEN: LazyLock<Mutex<HashSet<i32>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

/// Guard over the conn map; `as_mut()` yields the implicit default conn **0** (legacy call sites).
pub(crate) struct RusqliteConnGuard {
    guard: std::sync::MutexGuard<'static, HashMap<i32, rusqlite::Connection>>,
}

impl RusqliteConnGuard {
    pub fn as_mut(&mut self) -> Option<&mut rusqlite::Connection> {
        self.guard.get_mut(&0)
    }
}

pub(crate) fn rusqlite_conn() -> rusqlite::Result<RusqliteConnGuard> {
    let mut guard = RUSQLITE_CONNS
        .lock()
        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
    if !guard.contains_key(&0) {
        guard.insert(0, rusqlite::Connection::open_in_memory()?);
    }
    Ok(RusqliteConnGuard { guard })
}

fn open_connection(dsn: &str) -> rusqlite::Result<rusqlite::Connection> {
    if dsn.is_empty() || dsn == ":memory:" {
        rusqlite::Connection::open_in_memory()
    } else {
        rusqlite::Connection::open(dsn)
    }
}

/// Reset rusqlite store between tests.
pub fn reset_rusqlite_store() {
    if let Ok(mut pools) = SQL_POOLS.lock() {
        pools.clear();
    }
    let _ = NEXT_POOL_ID.store(1, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut g) = RUSQLITE_CONNS.lock() {
        g.clear();
    }
    let _ = NEXT_CONN_ID.store(1, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut g) = SQL_TXN_OPEN.lock() {
        g.clear();
    }
    if let Ok(mut g) = PREPARED_SQL.lock() {
        g.clear();
    }
}

/// M2 W35 / ADR-179: live explicit Sql conn count (id ≥ 1) for EC-M2-3 leak nails (test/ops only).
/// Implicit default conn 0 is excluded.
pub fn sql_conn_live_count() -> usize {
    RUSQLITE_CONNS
        .lock()
        .map(|g| g.keys().filter(|&&id| id >= 1).count())
        .unwrap_or(0)
}

/// Test cleanup: drop an explicit connection by id (id ≥ 1) without going through the SYX facade.
pub fn sql_conn_force_close(id: i32) -> bool {
    if id < 1 {
        return false;
    }
    let removed = RUSQLITE_CONNS
        .lock()
        .ok()
        .and_then(|mut g| g.remove(&id))
        .is_some();
    if removed {
        if let Ok(mut txn) = SQL_TXN_OPEN.lock() {
            txn.remove(&id);
        }
    }
    removed
}

/// ADR-160: open a new connection; returns handle id ≥ 1 (never 0).
pub fn host_sql_open(stack: &mut Vec<Value>) -> SyxResult<()> {
    let dsn = pop_str(stack);
    let conn = open_connection(&dsn).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("sql open failed: {e}")).with_code("R003")
    })?;
    let id = NEXT_CONN_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    RUSQLITE_CONNS
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?
        .insert(id, conn);
    stack.push(Value::I32(id));
    Ok(())
}

/// ADR-160: close an explicitly opened connection (id ≥ 1). Conn 0 cannot be closed.
pub fn host_sql_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    if conn_id == 0 {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "sql close failed: cannot close implicit default connection 0",
        )
        .with_code("R003"));
    }
    let removed = RUSQLITE_CONNS
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?
        .remove(&conn_id)
        .is_some();
    if !removed {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "sql close failed: unknown connection {conn_id}"
        ))
        .with_code("R003"));
    }
    if let Ok(mut txn) = SQL_TXN_OPEN.lock() {
        txn.remove(&conn_id);
    }
    Ok(())
}

/// ADR-245: `Sql.poolCreate(dsn, maxSize) -> poolId`.
pub fn host_sql_pool_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let max_size = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let dsn = pop_str(stack);
    if max_size < 1 {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "sql poolCreate failed: maxSize must be >= 1",
        )
        .with_code("R003"));
    }
    let id = NEXT_POOL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    SQL_POOLS
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?
        .insert(
            id,
            SqlPool {
                dsn,
                max: max_size as usize,
                free: Vec::new(),
                checked_out: HashSet::new(),
            },
        );
    stack.push(Value::I32(id));
    Ok(())
}

/// ADR-245: `Sql.poolAcquire(poolId) -> connId` — non-blocking; exhaust throws.
pub fn host_sql_pool_acquire(stack: &mut Vec<Value>) -> SyxResult<()> {
    let pool_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let mut pools = SQL_POOLS
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?;
    let Some(pool) = pools.get_mut(&pool_id) else {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "sql poolAcquire failed: unknown pool {pool_id}"
        ))
        .with_code("R003"));
    };
    if let Some(conn_id) = pool.free.pop() {
        pool.checked_out.insert(conn_id);
        stack.push(Value::I32(conn_id));
        return Ok(());
    }
    let live = pool.free.len() + pool.checked_out.len();
    if live >= pool.max {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "sql poolAcquire failed: pool {pool_id} exhausted (max {})",
            pool.max
        ))
        .with_code("R003"));
    }
    let dsn = pool.dsn.clone();
    let conn = open_connection(&dsn).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("sql poolAcquire open failed: {e}"))
            .with_code("R003")
    })?;
    let conn_id = NEXT_CONN_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    RUSQLITE_CONNS
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?
        .insert(conn_id, conn);
    pool.checked_out.insert(conn_id);
    stack.push(Value::I32(conn_id));
    Ok(())
}

/// ADR-245 / ADR-247: `Sql.poolRelease(poolId, connId)` — return to free list (no close).
/// Rejects mid-txn release (`SQL_TXN_OPEN`).
pub fn host_sql_pool_release(stack: &mut Vec<Value>) -> SyxResult<()> {
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let pool_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    if SQL_TXN_OPEN
        .lock()
        .map(|g| g.contains(&conn_id))
        .unwrap_or(false)
    {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "sql poolRelease failed: conn {conn_id} has an open transaction"
        ))
        .with_code("R003"));
    }
    let mut pools = SQL_POOLS
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?;
    let Some(pool) = pools.get_mut(&pool_id) else {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "sql poolRelease failed: unknown pool {pool_id}"
        ))
        .with_code("R003"));
    };
    if !pool.checked_out.remove(&conn_id) {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "sql poolRelease failed: conn {conn_id} not checked out from pool {pool_id}"
        ))
        .with_code("R003"));
    }
    pool.free.push(conn_id);
    stack.push(Value::Nil);
    Ok(())
}

/// ADR-245 / ADR-247: `Sql.poolClose(poolId)` — close all pooled conns and drop pool.
/// Clears `SQL_TXN_OPEN` for those ids so live-count stays honest.
pub fn host_sql_pool_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let pool_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let mut pools = SQL_POOLS
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?;
    let Some(pool) = pools.remove(&pool_id) else {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "sql poolClose failed: unknown pool {pool_id}"
        ))
        .with_code("R003"));
    };
    let mut ids: Vec<i32> = pool.free;
    ids.extend(pool.checked_out.iter().copied());
    let mut conns = RUSQLITE_CONNS
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?;
    for id in &ids {
        let _ = conns.remove(id);
    }
    drop(conns);
    if let Ok(mut txn) = SQL_TXN_OPEN.lock() {
        for id in ids {
            txn.remove(&id);
        }
    }
    stack.push(Value::Nil);
    Ok(())
}

/// ADR-246: resolve a query row Object bag tagged `__type=SqlRow`.
fn sql_row_map(row: &Value) -> SyxResult<HashMap<String, Value>> {
    let resolved = Value::resolve_heap_slot(row);
    let map = match resolved {
        Value::Object(m) | Value::Dict(m) => m,
        _ => {
            return Err(syx_diagnostics::SyxDiagnostic::error(
                "SqlRow accessor requires SqlRow object",
            )
            .with_code("R003"));
        }
    };
    match map.get("__type") {
        Some(Value::Str(t)) if t == "SqlRow" => Ok(map),
        _ => Err(
            syx_diagnostics::SyxDiagnostic::error("SqlRow accessor requires __type=SqlRow")
                .with_code("R003"),
        ),
    }
}

fn sql_row_get_cell(row: &Value, name: &str) -> SyxResult<Value> {
    let map = sql_row_map(row)?;
    map.get(name).cloned().ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error(format!("SqlRow missing column '{name}'"))
            .with_code("R003")
    })
}

/// ADR-246: `SqlRow.getString(row, name)`.
pub fn host_sql_row_get_string(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = pop_str(stack);
    let row = stack.pop().unwrap_or(Value::Nil);
    match sql_row_get_cell(&row, &name)? {
        Value::Str(s) => stack.push(Value::Str(s)),
        _ => {
            return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                "SqlRow.getString column '{name}' type mismatch"
            ))
            .with_code("R003"));
        }
    }
    Ok(())
}

/// ADR-246: `SqlRow.getInt(row, name)`.
pub fn host_sql_row_get_int(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = pop_str(stack);
    let row = stack.pop().unwrap_or(Value::Nil);
    match sql_row_get_cell(&row, &name)? {
        Value::I64(n) => stack.push(Value::I64(n)),
        Value::I32(n) => stack.push(Value::I32(n)),
        _ => {
            return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                "SqlRow.getInt column '{name}' type mismatch"
            ))
            .with_code("R003"));
        }
    }
    Ok(())
}

/// ADR-246: `SqlRow.getFloat(row, name)`.
pub fn host_sql_row_get_float(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = pop_str(stack);
    let row = stack.pop().unwrap_or(Value::Nil);
    match sql_row_get_cell(&row, &name)? {
        Value::F64(f) => stack.push(Value::F64(f)),
        Value::F32(f) => stack.push(Value::F64(f as f64)),
        _ => {
            return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                "SqlRow.getFloat column '{name}' type mismatch"
            ))
            .with_code("R003"));
        }
    }
    Ok(())
}

/// ADR-246: `SqlRow.getBlob(row, name)`.
pub fn host_sql_row_get_blob(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = pop_str(stack);
    let row = stack.pop().unwrap_or(Value::Nil);
    match sql_row_get_cell(&row, &name)? {
        Value::Array(items) => stack.push(Value::Array(items)),
        _ => {
            return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                "SqlRow.getBlob column '{name}' type mismatch"
            ))
            .with_code("R003"));
        }
    }
    Ok(())
}

/// ADR-246: `SqlRow.isNull(row, name)` — missing column throws; Nil → true.
pub fn host_sql_row_is_null(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = pop_str(stack);
    let row = stack.pop().unwrap_or(Value::Nil);
    let cell = sql_row_get_cell(&row, &name)?;
    stack.push(Value::Bool(matches!(cell, Value::Nil)));
    Ok(())
}

/// ADR-246: `SqlRow.columnNames(row)` — excludes `__type`.
pub fn host_sql_row_column_names(stack: &mut Vec<Value>) -> SyxResult<()> {
    let row = stack.pop().unwrap_or(Value::Nil);
    let map = sql_row_map(&row)?;
    let mut names: Vec<Value> = map
        .keys()
        .filter(|k| k.as_str() != "__type")
        .map(|k| Value::Str(k.clone()))
        .collect();
    names.sort_by(|a, b| a.as_str().cmp(&b.as_str()));
    stack.push(Value::Array(names));
    Ok(())
}

fn with_conn_id<F>(conn_id: i32, f: F) -> SyxResult<()>
where
    F: FnOnce(&mut rusqlite::Connection) -> SyxResult<()>,
{
    let mut guard = RUSQLITE_CONNS
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?;
    if conn_id == 0 && !guard.contains_key(&0) {
        let conn = rusqlite::Connection::open_in_memory()
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?;
        guard.insert(0, conn);
    }
    let conn = guard.get_mut(&conn_id).ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error(format!("sql connection {conn_id} not found"))
            .with_code("R003")
    })?;
    f(conn)
}

/// ADR-160: exec on a specific connection id.
pub fn host_sql_exec_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let mut changed = 0i32;
    with_conn_id(conn_id, |conn| {
        changed = conn.execute(&sql, []).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("sql execOn failed: {e}"))
                .with_code("R003")
        })? as i32;
        Ok(())
    })?;
    stack.push(Value::I32(changed));
    Ok(())
}

/// ADR-160: query on a specific connection id.
pub fn host_sql_query_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let mut rows = Vec::new();
    with_conn_id(conn_id, |conn| {
        let mut stmt = conn.prepare(&sql).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("sql queryOn failed: {e}"))
                .with_code("R003")
        })?;
        let col_count = stmt.column_count();
        let col_names: Vec<String> = (0..col_count)
            .map(|i| stmt.column_name(i).unwrap_or("col").to_string())
            .collect();
        let mut query = stmt.query([]).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("sql queryOn failed: {e}"))
                .with_code("R003")
        })?;
        while let Some(row) = query.next().map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("sql queryOn failed: {e}"))
                .with_code("R003")
        })? {
            let mut map = HashMap::new();
            map.insert("__type".into(), Value::Str("SqlRow".into()));
            for (i, name) in col_names.iter().enumerate() {
                let val: rusqlite::types::Value =
                    row.get(i).unwrap_or(rusqlite::types::Value::Null);
                map.insert(
                    name.clone(),
                    match val {
                        rusqlite::types::Value::Integer(n) => Value::I64(n),
                        rusqlite::types::Value::Real(f) => Value::F64(f),
                        rusqlite::types::Value::Text(s) => Value::Str(s),
                        rusqlite::types::Value::Blob(b) => {
                            Value::Array(b.into_iter().map(|x| Value::U8(x)).collect())
                        }
                        rusqlite::types::Value::Null => Value::Nil,
                    },
                );
            }
            rows.push(Value::Object(map));
        }
        Ok(())
    })?;
    stack.push(Value::Array(rows));
    Ok(())
}

static NEXT_STMT_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(1);
/// ADR-164: stmt id → (boundConnId, sql text). No live Statement handles.
static PREPARED_SQL: LazyLock<Mutex<HashMap<i32, (i32, String)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Phase 89: rusqlite-backed SQL exec.
pub fn host_sql_exec_rusqlite(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let mut guard = rusqlite_conn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?;
    let conn = guard.as_mut().unwrap();
    // ADR-155: fail-closed Err (no empty-success sentinel on execute failure).
    let changed = conn.execute(&sql, []).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("sql exec failed: {e}")).with_code("R003")
    })?;
    stack.push(Value::I32(changed as i32));
    Ok(())
}

/// Phase 89: rusqlite-backed SQL query — returns array of row objects.
pub fn host_sql_query_rusqlite(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let mut guard =
        rusqlite_conn().map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let conn = guard.as_mut().unwrap();
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let col_count = stmt.column_count();
    let col_names: Vec<String> = (0..col_count)
        .map(|i| stmt.column_name(i).unwrap_or("col").to_string())
        .collect();
    let mut rows = Vec::new();
    let mut query = stmt
        .query([])
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    while let Some(row) = query
        .next()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?
    {
        let mut map = HashMap::new();
        map.insert("__type".into(), Value::Str("SqlRow".into()));
        for (i, name) in col_names.iter().enumerate() {
            let val: rusqlite::types::Value = row.get(i).unwrap_or(rusqlite::types::Value::Null);
            map.insert(
                name.clone(),
                match val {
                    rusqlite::types::Value::Integer(n) => Value::I64(n),
                    rusqlite::types::Value::Real(f) => Value::F64(f),
                    rusqlite::types::Value::Text(s) => Value::Str(s),
                    rusqlite::types::Value::Blob(b) => {
                        Value::Array(b.into_iter().map(|x| Value::U8(x)).collect())
                    }
                    rusqlite::types::Value::Null => Value::Nil,
                },
            );
        }
        rows.push(Value::Object(map));
    }
    stack.push(Value::Array(rows));
    Ok(())
}

/// Phase 89 / ADR-148: SQL prepare — validate via rusqlite; return Int stmt id (never `prep:`).
pub fn host_sql_prepare(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let mut guard = rusqlite_conn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?;
    let conn = guard.as_mut().unwrap();
    // Validate SQL; drop Statement immediately (do not hold Connection borrow).
    let _stmt = conn.prepare(&sql).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("sql prepare failed: {e}")).with_code("R003")
    })?;
    drop(_stmt);
    let id = NEXT_STMT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    PREPARED_SQL
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?
        .insert(id, (0, sql));
    stack.push(Value::I32(id));
    Ok(())
}

/// ADR-162: prepare on a specific connection id (schema validation); ADR-164 stores (connId, sql).
pub fn host_sql_prepare_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    with_conn_id(conn_id, |conn| {
        let _stmt = conn.prepare(&sql).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("sql prepareOn failed: {e}"))
                .with_code("R003")
        })?;
        drop(_stmt);
        Ok(())
    })?;
    let id = NEXT_STMT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    PREPARED_SQL
        .lock()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?
        .insert(id, (conn_id, sql));
    stack.push(Value::I32(id));
    Ok(())
}

/// ADR-163: parameterized execute on a specific connection id (same semantics as bindStep, target conn).
pub fn host_sql_bind_step_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let bind_val = pop_str(stack);
    let sql = pop_str(stack);
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let mut changed = 0i32;
    with_conn_id(conn_id, |conn| {
        changed = conn
            .execute(&sql, rusqlite::params![bind_val])
            .map_err(|e| {
                syx_diagnostics::SyxDiagnostic::error(format!("sql bindStepOn failed: {e}"))
                    .with_code("R003")
            })? as i32;
        Ok(())
    })?;
    stack.push(Value::I32(changed));
    Ok(())
}

/// ADR-164: execute a prepared stmt on its bound connection (rejects conn mismatch).
pub fn host_sql_bind_prepared_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let bind_val = pop_str(stack);
    let stmt_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let (bound_conn, sql) = {
        let guard = PREPARED_SQL
            .lock()
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()).with_code("R003"))?;
        guard.get(&stmt_id).cloned().ok_or_else(|| {
            syx_diagnostics::SyxDiagnostic::error(format!(
                "sql bindPreparedOn failed: unknown stmt {stmt_id}"
            ))
            .with_code("R003")
        })?
    };
    if bound_conn != conn_id {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "sql bindPreparedOn failed: stmt {stmt_id} bound to conn {bound_conn}, not {conn_id}"
        ))
        .with_code("R003"));
    }
    let mut changed = 0i32;
    with_conn_id(conn_id, |conn| {
        changed = conn
            .execute(&sql, rusqlite::params![bind_val])
            .map_err(|e| {
                syx_diagnostics::SyxDiagnostic::error(format!("sql bindPreparedOn failed: {e}"))
                    .with_code("R003")
            })? as i32;
        Ok(())
    })?;
    stack.push(Value::I32(changed));
    Ok(())
}

/// ADR-165 / ADR-247: BEGIN IMMEDIATE on a specific connection id (does not touch SQL_TXN_ACTIVE).
pub fn host_sql_begin_transaction_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    with_conn_id(conn_id, |conn| {
        conn.execute_batch("BEGIN IMMEDIATE;").map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("sql beginTransactionOn failed: {e}"))
                .with_code("R003")
        })?;
        Ok(())
    })?;
    if let Ok(mut txn) = SQL_TXN_OPEN.lock() {
        txn.insert(conn_id);
    }
    Ok(())
}

/// ADR-165 / ADR-247: COMMIT on a specific connection id.
pub fn host_sql_commit_transaction_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    with_conn_id(conn_id, |conn| {
        conn.execute_batch("COMMIT;").map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("sql commitTransactionOn failed: {e}"))
                .with_code("R003")
        })?;
        Ok(())
    })?;
    if let Ok(mut txn) = SQL_TXN_OPEN.lock() {
        txn.remove(&conn_id);
    }
    Ok(())
}

/// ADR-165 / ADR-247: ROLLBACK on a specific connection id.
pub fn host_sql_rollback_transaction_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    with_conn_id(conn_id, |conn| {
        conn.execute_batch("ROLLBACK;").map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("sql rollbackTransactionOn failed: {e}"))
                .with_code("R003")
        })?;
        Ok(())
    })?;
    if let Ok(mut txn) = SQL_TXN_OPEN.lock() {
        txn.remove(&conn_id);
    }
    Ok(())
}

/// Phase 89: ORM save — reflect fields into INSERT.
pub fn host_orm_save(stack: &mut Vec<Value>) -> SyxResult<()> {
    let record = stack.pop().unwrap_or(Value::Nil);
    let table = pop_str(stack);
    let Some(map) = value_as_map(&record) else {
        stack.push(Value::Bool(false));
        return Ok(());
    };
    let mut cols = Vec::new();
    let mut vals = Vec::new();
    for (k, v) in &map {
        if k.starts_with("__") {
            continue;
        }
        cols.push(k.clone());
        vals.push(v.as_str());
    }
    if cols.is_empty() {
        stack.push(Value::Bool(false));
        return Ok(());
    }
    let placeholders: Vec<&str> = cols.iter().map(|_| "?").collect();
    let sql = format!(
        "INSERT INTO {table} ({}) VALUES ({})",
        cols.join(", "),
        placeholders.join(", ")
    );
    let mut guard =
        rusqlite_conn().map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let conn = guard.as_mut().unwrap();
    let col_defs: Vec<String> = cols.iter().map(|c| format!("{c} TEXT")).collect();
    let _ = conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {table} ({})",
            col_defs.join(", ")
        ),
        [],
    );
    let ok = conn
        .execute(
            &sql,
            rusqlite::params_from_iter(vals.iter().map(|s| s.as_str())),
        )
        .is_ok();
    stack.push(Value::Bool(ok));
    Ok(())
}

/// ADR-166: reflective INSERT on a specific connection id (fail-closed).
pub fn host_orm_save_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let record = stack.pop().unwrap_or(Value::Nil);
    let table = pop_str(stack);
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let Some(map) = value_as_map(&record) else {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "orm saveOn failed: record is not an object",
        )
        .with_code("R003"));
    };
    let mut cols = Vec::new();
    let mut vals = Vec::new();
    for (k, v) in &map {
        if k.starts_with("__") {
            continue;
        }
        cols.push(k.clone());
        vals.push(v.as_str());
    }
    if cols.is_empty() {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "orm saveOn failed: no columns in record",
        )
        .with_code("R003"));
    }
    let placeholders: Vec<&str> = cols.iter().map(|_| "?").collect();
    let sql = format!(
        "INSERT INTO {table} ({}) VALUES ({})",
        cols.join(", "),
        placeholders.join(", ")
    );
    let col_defs: Vec<String> = cols.iter().map(|c| format!("{c} TEXT")).collect();
    let create_sql = format!(
        "CREATE TABLE IF NOT EXISTS {table} ({})",
        col_defs.join(", ")
    );
    with_conn_id(conn_id, |conn| {
        conn.execute(&create_sql, []).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("orm saveOn failed: {e}"))
                .with_code("R003")
        })?;
        conn.execute(
            &sql,
            rusqlite::params_from_iter(vals.iter().map(|s| s.as_str())),
        )
        .map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("orm saveOn failed: {e}"))
                .with_code("R003")
        })?;
        Ok(())
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

/// ADR-167: parameterized SELECT WHERE on a specific connection id (fail-closed).
pub fn host_orm_find_where_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let val = pop_str(stack);
    let col = pop_str(stack);
    let table = pop_str(stack);
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let sql = format!("SELECT * FROM {table} WHERE {col} = ?");
    let mut rows = Vec::new();
    with_conn_id(conn_id, |conn| {
        let mut stmt = conn.prepare(&sql).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("orm findWhereOn failed: {e}"))
                .with_code("R003")
        })?;
        let col_count = stmt.column_count();
        let col_names: Vec<String> = (0..col_count)
            .map(|i| stmt.column_name(i).unwrap_or("col").to_string())
            .collect();
        let mut query = stmt.query(rusqlite::params![val]).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("orm findWhereOn failed: {e}"))
                .with_code("R003")
        })?;
        while let Some(row) = query.next().map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("orm findWhereOn failed: {e}"))
                .with_code("R003")
        })? {
            let mut map = HashMap::new();
            map.insert("__type".into(), Value::Str("SqlRow".into()));
            for (i, name) in col_names.iter().enumerate() {
                let cell: rusqlite::types::Value =
                    row.get(i).unwrap_or(rusqlite::types::Value::Null);
                map.insert(
                    name.clone(),
                    match cell {
                        rusqlite::types::Value::Integer(n) => Value::I64(n),
                        rusqlite::types::Value::Real(f) => Value::F64(f),
                        rusqlite::types::Value::Text(s) => Value::Str(s),
                        rusqlite::types::Value::Blob(b) => {
                            Value::Array(b.into_iter().map(|x| Value::U8(x)).collect())
                        }
                        rusqlite::types::Value::Null => Value::Nil,
                    },
                );
            }
            rows.push(Value::Object(map));
        }
        Ok(())
    })?;
    stack.push(Value::Array(rows));
    Ok(())
}

/// ADR-168: parameterized UPDATE WHERE on a specific connection id (fail-closed).
pub fn host_orm_update_where_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let new_val = pop_str(stack);
    let new_col = pop_str(stack);
    let where_val = pop_str(stack);
    let where_col = pop_str(stack);
    let table = pop_str(stack);
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let sql = format!("UPDATE {table} SET {new_col} = ? WHERE {where_col} = ?");
    let mut changed = 0i32;
    with_conn_id(conn_id, |conn| {
        changed = conn
            .execute(&sql, rusqlite::params![new_val, where_val])
            .map_err(|e| {
                syx_diagnostics::SyxDiagnostic::error(format!("orm updateWhereOn failed: {e}"))
                    .with_code("R003")
            })? as i32;
        Ok(())
    })?;
    stack.push(Value::I32(changed));
    Ok(())
}

/// ADR-169: parameterized DELETE WHERE on a specific connection id (fail-closed).
pub fn host_orm_delete_where_on(stack: &mut Vec<Value>) -> SyxResult<()> {
    let where_val = pop_str(stack);
    let where_col = pop_str(stack);
    let table = pop_str(stack);
    let conn_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as i32;
    let sql = format!("DELETE FROM {table} WHERE {where_col} = ?");
    let mut changed = 0i32;
    with_conn_id(conn_id, |conn| {
        changed = conn
            .execute(&sql, rusqlite::params![where_val])
            .map_err(|e| {
                syx_diagnostics::SyxDiagnostic::error(format!("orm deleteWhereOn failed: {e}"))
                    .with_code("R003")
            })? as i32;
        Ok(())
    })?;
    stack.push(Value::I32(changed));
    Ok(())
}

/// Phase 89: ORM find — SELECT * FROM table.
pub fn host_orm_find(stack: &mut Vec<Value>) -> SyxResult<()> {
    let table = pop_str(stack);
    let mut inner = vec![Value::Str(format!("SELECT * FROM {table}"))];
    host_sql_query_rusqlite(&mut inner)?;
    stack.push(inner.pop().unwrap_or(Value::Array(vec![])));
    Ok(())
}

static CODABLE_SCHEMAS: LazyLock<Mutex<HashMap<String, u32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Phase 89: register wire schema id for a type name.
pub fn host_codable_schema_register(stack: &mut Vec<Value>) -> SyxResult<()> {
    let wire_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as u32;
    let type_name = pop_str(stack);
    if let Ok(mut reg) = CODABLE_SCHEMAS.lock() {
        reg.insert(type_name, wire_id);
    }
    stack.push(Value::Bool(true));
    Ok(())
}

fn xpath_match(expr: &str, el: &HashMap<String, Value>) -> bool {
    let tag = el.get("tag").map(|v| v.as_str()).unwrap_or_default();
    match expr.trim() {
        e if e.starts_with("//") => tag == e.trim_start_matches("//"),
        e if e.starts_with('/') => {
            let parts: Vec<&str> = e.trim_start_matches('/').split('/').collect();
            tag == *parts.last().unwrap_or(&"")
        }
        e if e.starts_with('@') => el.contains_key(&e[1..]),
        _ => tag == expr,
    }
}

fn xpath_collect(expr: &str, el: &HashMap<String, Value>) -> Vec<Value> {
    let mut out = Vec::new();
    if xpath_match(expr, el) {
        out.push(Value::Object(el.clone()));
    }
    if let Some(Value::Array(children)) = el.get("children") {
        for child in children {
            if let Value::Object(map) = child {
                out.extend(xpath_collect(expr, map));
            }
        }
    }
    out
}

/// Phase 89: XPath subset query on parsed XML element tree.
pub fn host_xml_xpath(stack: &mut Vec<Value>) -> SyxResult<()> {
    let expr = pop_str(stack);
    let root = stack.pop().unwrap_or(Value::Nil);
    let Some(map) = value_as_map(&root) else {
        stack.push(Value::Array(vec![]));
        return Ok(());
    };
    stack.push(Value::Array(xpath_collect(&expr, &map)));
    Ok(())
}

static GUI_WINDOWS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Phase 89: create GUI window (loopback stub).
pub fn host_gui_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let title = pop_str(stack);
    let id = format!("win:{title}");
    if let Ok(mut wins) = GUI_WINDOWS.lock() {
        wins.push(id.clone());
    }
    stack.push(Value::Str(id));
    Ok(())
}

/// Phase 89: pump GUI event loop once (loopback).
pub fn host_gui_event_pump(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _win = pop_str(stack);
    stack.push(Value::Str("event:idle".into()));
    Ok(())
}

/// Phase 89: canvas clear/draw stub.
pub fn host_gui_canvas_draw(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _color = pop_str(stack);
    let _win = pop_str(stack);
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn gui_window_count() -> usize {
    GUI_WINDOWS.lock().map(|g| g.len()).unwrap_or(0)
}

pub fn reset_gui_state() {
    if let Ok(mut wins) = GUI_WINDOWS.lock() {
        wins.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rusqlite_insert_select() {
        reset_rusqlite_store();
        host_sql_exec_rusqlite(&mut vec![Value::Str(
            "CREATE TABLE t (id INTEGER, name TEXT)".into(),
        )])
        .unwrap();
        host_sql_exec_rusqlite(&mut vec![Value::Str(
            "INSERT INTO t VALUES (1, 'a')".into(),
        )])
        .unwrap();
        let mut q = vec![Value::Str("SELECT * FROM t".into())];
        host_sql_query_rusqlite(&mut q).unwrap();
        let rows = q.pop().unwrap();
        assert!(matches!(rows, Value::Array(v) if !v.is_empty()));
    }
}
