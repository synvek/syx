//! Native SQL host domain. `syx_host_sql_install` fills shakeable host slots.

use syx_diagnostics::SyxResult;
use syx_runtime_core::{register_shakeable_host_id, Value};
use syx_stdlib::HostFnId;

fn orm_find_where_dispatch(stack: &mut Vec<Value>) -> SyxResult<()> {
    if std::env::var("SYX_ORM_BIND")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        syx_sql_impl::host_orm_find_where_dict_prepare(stack)
    } else {
        syx_sql_impl::host_orm_find_where(stack)
    }
}

/// Register every SQL-domain `HostFnId` into the shakeable host table.
pub fn install() {
    register_shakeable_host_id(HostFnId::SqlExec, syx_sql_impl::host_sql_exec_rusqlite);
    register_shakeable_host_id(HostFnId::SqlQuery, syx_sql_impl::host_sql_query_rusqlite);
    register_shakeable_host_id(HostFnId::SqlPrepare, syx_sql_impl::host_sql_prepare);
    register_shakeable_host_id(HostFnId::SqlOpen, syx_sql_impl::host_sql_open);
    register_shakeable_host_id(HostFnId::SqlClose, syx_sql_impl::host_sql_close);
    register_shakeable_host_id(HostFnId::SqlExecOn, syx_sql_impl::host_sql_exec_on);
    register_shakeable_host_id(HostFnId::SqlQueryOn, syx_sql_impl::host_sql_query_on);
    register_shakeable_host_id(HostFnId::SqlPrepareOn, syx_sql_impl::host_sql_prepare_on);
    register_shakeable_host_id(HostFnId::SqlBindStepOn, syx_sql_impl::host_sql_bind_step_on);
    register_shakeable_host_id(
        HostFnId::SqlBindPreparedOn,
        syx_sql_impl::host_sql_bind_prepared_on,
    );
    register_shakeable_host_id(
        HostFnId::SqlBeginTransactionOn,
        syx_sql_impl::host_sql_begin_transaction_on,
    );
    register_shakeable_host_id(
        HostFnId::SqlCommitTransactionOn,
        syx_sql_impl::host_sql_commit_transaction_on,
    );
    register_shakeable_host_id(
        HostFnId::SqlRollbackTransactionOn,
        syx_sql_impl::host_sql_rollback_transaction_on,
    );
    register_shakeable_host_id(HostFnId::OrmSave, syx_sql_impl::host_orm_save);
    register_shakeable_host_id(HostFnId::OrmSaveOn, syx_sql_impl::host_orm_save_on);
    register_shakeable_host_id(
        HostFnId::OrmFindWhereOn,
        syx_sql_impl::host_orm_find_where_on,
    );
    register_shakeable_host_id(
        HostFnId::OrmUpdateWhereOn,
        syx_sql_impl::host_orm_update_where_on,
    );
    register_shakeable_host_id(
        HostFnId::OrmDeleteWhereOn,
        syx_sql_impl::host_orm_delete_where_on,
    );
    register_shakeable_host_id(HostFnId::OrmFind, syx_sql_impl::host_orm_find);
    register_shakeable_host_id(HostFnId::SqlMigrate, syx_sql_impl::host_sql_migrate);
    register_shakeable_host_id(HostFnId::SqlBindStep, syx_sql_impl::host_sql_bind_step);
    register_shakeable_host_id(HostFnId::OrmFindWhere, orm_find_where_dispatch);
    register_shakeable_host_id(
        HostFnId::OrmUpdateWhere,
        syx_sql_impl::host_orm_update_where,
    );
    register_shakeable_host_id(
        HostFnId::OrmDeleteWhere,
        syx_sql_impl::host_orm_delete_where,
    );
    register_shakeable_host_id(
        HostFnId::SqlBeginTransaction,
        syx_sql_impl::host_sql_begin_transaction,
    );
    register_shakeable_host_id(
        HostFnId::SqlCommitTransaction,
        syx_sql_impl::host_sql_commit_transaction,
    );
    register_shakeable_host_id(
        HostFnId::SqlRollback,
        syx_sql_impl::host_sql_rollback_transaction,
    );
    register_shakeable_host_id(HostFnId::SqlBindMulti, syx_sql_impl::host_sql_bind_multi);
    register_shakeable_host_id(HostFnId::SqlPoolCreate, syx_sql_impl::host_sql_pool_create);
    register_shakeable_host_id(
        HostFnId::SqlPoolAcquire,
        syx_sql_impl::host_sql_pool_acquire,
    );
    register_shakeable_host_id(
        HostFnId::SqlPoolRelease,
        syx_sql_impl::host_sql_pool_release,
    );
    register_shakeable_host_id(HostFnId::SqlPoolClose, syx_sql_impl::host_sql_pool_close);
    register_shakeable_host_id(
        HostFnId::SqlRowGetString,
        syx_sql_impl::host_sql_row_get_string,
    );
    register_shakeable_host_id(HostFnId::SqlRowGetInt, syx_sql_impl::host_sql_row_get_int);
    register_shakeable_host_id(
        HostFnId::SqlRowGetFloat,
        syx_sql_impl::host_sql_row_get_float,
    );
    register_shakeable_host_id(HostFnId::SqlRowGetBlob, syx_sql_impl::host_sql_row_get_blob);
    register_shakeable_host_id(HostFnId::SqlRowIsNull, syx_sql_impl::host_sql_row_is_null);
    register_shakeable_host_id(
        HostFnId::SqlRowColumnNames,
        syx_sql_impl::host_sql_row_column_names,
    );
}

/// C ABI entry the native init calls when the SQL domain is linked.
#[no_mangle]
pub extern "C" fn syx_host_sql_install() {
    install();
}
