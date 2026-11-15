//! rusqlite-backed SQL/ORM host implementations.
//!
//! Linked only when a program references the SQL shake domain (`syx-host-sql`).

#![allow(dead_code)]
#![allow(unused_imports)]

mod error;
mod phase89;
mod sql_ext;

pub use phase89::{
    host_orm_delete_where_on, host_orm_find, host_orm_find_where_on, host_orm_save,
    host_orm_save_on, host_orm_update_where_on, host_sql_begin_transaction_on,
    host_sql_bind_prepared_on, host_sql_bind_step_on, host_sql_close,
    host_sql_commit_transaction_on, host_sql_exec_on, host_sql_exec_rusqlite, host_sql_open,
    host_sql_pool_acquire, host_sql_pool_close, host_sql_pool_create, host_sql_pool_release,
    host_sql_prepare, host_sql_prepare_on, host_sql_query_on, host_sql_query_rusqlite,
    host_sql_rollback_transaction_on, host_sql_row_column_names, host_sql_row_get_blob,
    host_sql_row_get_float, host_sql_row_get_int, host_sql_row_get_string, host_sql_row_is_null,
    reset_rusqlite_store, rusqlite_conn, sql_conn_force_close, sql_conn_live_count,
    RusqliteConnGuard,
};
pub use sql_ext::{
    host_orm_delete_where, host_orm_find_where, host_orm_find_where_dict,
    host_orm_find_where_dict_prepare, host_orm_find_where_dict_rusqlite, host_orm_find_where_l90,
    host_orm_find_where_l91, host_orm_update_where, host_sql_begin_transaction,
    host_sql_bind_multi, host_sql_bind_step, host_sql_commit_transaction, host_sql_migrate,
    host_sql_rollback_transaction, migration_version, orm_executed_sql_count,
    orm_last_executed_sql, orm_prepared_row_count, orm_prepared_rows, reset_migration_version,
    reset_orm_executed_sql, reset_orm_prepared_rows, reset_sql_txn_active,
};
