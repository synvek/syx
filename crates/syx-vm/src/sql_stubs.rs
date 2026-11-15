//! Wasm-style SQL/ORM stubs used when rusqlite hosts are not linked into this crate.
#![allow(unused_variables)]

use syx_diagnostics::{Span, SyxResult};
use syx_runtime_core::Value;

use crate::vm_error_at;

fn unavailable(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(vm_error_at(
        "native host unavailable on wasm32",
        None,
        Span::DUMMY,
        "V081",
    ))
}

macro_rules! sql_stub {
    ($($name:ident),+ $(,)?) => {
        $(
            pub fn $name(stack: &mut Vec<Value>) -> SyxResult<()> {
                unavailable(stack)
            }
        )+
    };
}

sql_stub! {
    host_sql_open,
    host_sql_close,
    host_sql_pool_create,
    host_sql_pool_acquire,
    host_sql_pool_release,
    host_sql_pool_close,
    host_sql_row_get_string,
    host_sql_row_get_int,
    host_sql_row_get_float,
    host_sql_row_get_blob,
    host_sql_row_is_null,
    host_sql_row_column_names,
    host_sql_exec_on,
    host_sql_query_on,
    host_sql_exec_rusqlite,
    host_sql_query_rusqlite,
    host_sql_prepare,
    host_sql_prepare_on,
    host_sql_bind_step_on,
    host_sql_bind_prepared_on,
    host_sql_begin_transaction_on,
    host_sql_commit_transaction_on,
    host_sql_rollback_transaction_on,
    host_orm_save,
    host_orm_save_on,
    host_orm_find_where_on,
    host_orm_update_where_on,
    host_orm_delete_where_on,
    host_orm_find,
    host_sql_migrate,
    host_sql_bind_step,
    host_orm_find_where,
    host_orm_find_where_l90,
    host_orm_find_where_l91,
    host_orm_update_where,
    host_orm_delete_where,
    host_sql_begin_transaction,
    host_sql_commit_transaction,
    host_orm_find_where_dict,
    host_sql_bind_multi,
    host_sql_rollback_transaction,
    host_orm_find_where_dict_rusqlite,
    host_orm_find_where_dict_prepare,
}

pub fn reset_rusqlite_store() {}

pub fn sql_conn_live_count() -> usize {
    0
}

pub fn sql_conn_force_close(id: i32) -> bool {
    let _ = id;
    false
}

pub fn migration_version() -> u32 {
    0
}

pub fn reset_migration_version() {}

pub fn reset_sql_txn_active() {}

pub fn reset_orm_executed_sql() {}

pub fn reset_orm_prepared_rows() {}

pub fn orm_executed_sql_count() -> usize {
    0
}

pub fn orm_last_executed_sql() -> Option<String> {
    None
}

pub fn orm_prepared_row_count() -> usize {
    0
}

pub fn orm_prepared_rows() -> Vec<String> {
    Vec::new()
}
