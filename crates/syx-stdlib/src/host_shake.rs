//! Native link-time host shaking: classify absolute `HostFnId` indices into heavy domains.
//!
//! Domain membership drives whether `syx build --native` links `syx-host-{sql,http,tls}`.
//! Indices stay absolute (`host_fn_index`); do not renumber `HostFnId::all()`.

use crate::registry::{host_fn_index, HostFnId};

/// Heavy native host domains eligible for per-program archive linking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HostShakeDomain {
    Sql,
    Http,
    Tls,
}

impl HostShakeDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sql => "sql",
            Self::Http => "http",
            Self::Tls => "tls",
        }
    }

    pub fn all() -> &'static [HostShakeDomain] {
        &[Self::Sql, Self::Http, Self::Tls]
    }

    /// Cargo package / staticlib stem linked when this domain is referenced.
    pub fn archive_package(self) -> &'static str {
        match self {
            Self::Sql => "syx-host-sql",
            Self::Http => "syx-host-http",
            Self::Tls => "syx-host-tls",
        }
    }

    /// `#[no_mangle]` install entry the generated native init must call.
    pub fn install_symbol(self) -> &'static str {
        match self {
            Self::Sql => "syx_host_sql_install",
            Self::Http => "syx_host_http_install",
            Self::Tls => "syx_host_tls_install",
        }
    }
}

/// Map a `HostFnId` to a shakeable domain, if any.
pub fn host_fn_shake_domain(id: HostFnId) -> Option<HostShakeDomain> {
    match id {
        HostFnId::SqlExec
        | HostFnId::SqlQuery
        | HostFnId::SqlPrepare
        | HostFnId::SqlOpen
        | HostFnId::SqlClose
        | HostFnId::SqlExecOn
        | HostFnId::SqlQueryOn
        | HostFnId::SqlPrepareOn
        | HostFnId::SqlBindStepOn
        | HostFnId::SqlBindPreparedOn
        | HostFnId::SqlBeginTransactionOn
        | HostFnId::SqlCommitTransactionOn
        | HostFnId::SqlRollbackTransactionOn
        | HostFnId::OrmSave
        | HostFnId::OrmSaveOn
        | HostFnId::OrmFindWhereOn
        | HostFnId::OrmUpdateWhereOn
        | HostFnId::OrmDeleteWhereOn
        | HostFnId::OrmFind
        | HostFnId::SqlMigrate
        | HostFnId::SqlBindStep
        | HostFnId::OrmFindWhere
        | HostFnId::OrmUpdateWhere
        | HostFnId::OrmDeleteWhere
        | HostFnId::SqlBeginTransaction
        | HostFnId::SqlCommitTransaction
        | HostFnId::SqlRollback
        | HostFnId::SqlBindMulti
        | HostFnId::SqlPoolCreate
        | HostFnId::SqlPoolAcquire
        | HostFnId::SqlPoolRelease
        | HostFnId::SqlPoolClose
        | HostFnId::SqlRowGetString
        | HostFnId::SqlRowGetInt
        | HostFnId::SqlRowGetFloat
        | HostFnId::SqlRowGetBlob
        | HostFnId::SqlRowIsNull
        | HostFnId::SqlRowColumnNames => Some(HostShakeDomain::Sql),

        HostFnId::HttpGet
        | HostFnId::HttpPost
        | HostFnId::HttpResponseJson
        | HostFnId::HttpRequest
        | HostFnId::HttpResponseStatus
        | HostFnId::HttpResponseText
        | HostFnId::HttpResponseOk
        | HostFnId::HttpResponseHeader
        | HostFnId::HttpResponseHeaders
        | HostFnId::HttpServe
        | HostFnId::HttpMakeResponse
        | HostFnId::HttpRequestAsync
        | HostFnId::WsConnect
        | HostFnId::WsConnectAsync
        | HostFnId::WsSendAsync
        | HostFnId::WsSend
        | HostFnId::WsSendProd
        | HostFnId::WsRecv
        | HostFnId::WsClose
        | HostFnId::HttpShutdown
        | HostFnId::HttpBodyStreamOpen
        | HostFnId::HttpBodyStreamOpenOpts
        | HostFnId::HttpBodyStreamReadChunk
        | HostFnId::HttpBodyStreamClose => Some(HostShakeDomain::Http),

        HostFnId::TlsServe => Some(HostShakeDomain::Tls),

        _ => None,
    }
}

/// Absolute index → shake domain (None = always linked in core `host_fn_for`).
pub fn host_index_shake_domain(idx: u32) -> Option<HostShakeDomain> {
    let id = HostFnId::all().get(idx as usize).copied()?;
    host_fn_shake_domain(id)
}

/// Domains touched by a set of absolute CallHost indices.
pub fn shake_domains_for_indices(indices: &std::collections::BTreeSet<u32>) -> Vec<HostShakeDomain> {
    let mut out = Vec::new();
    for &idx in indices {
        if let Some(d) = host_index_shake_domain(idx) {
            if !out.contains(&d) {
                out.push(d);
            }
        }
    }
    out.sort();
    out
}

/// All absolute indices that belong to `domain` (for full-domain install).
pub fn host_indices_for_domain(domain: HostShakeDomain) -> Vec<u32> {
    HostFnId::all()
        .iter()
        .copied()
        .filter(|id| host_fn_shake_domain(*id) == Some(domain))
        .map(host_fn_index)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sql_exec_is_sql_domain() {
        assert_eq!(
            host_fn_shake_domain(HostFnId::SqlExec),
            Some(HostShakeDomain::Sql)
        );
        assert_eq!(
            host_fn_shake_domain(HostFnId::HttpGet),
            Some(HostShakeDomain::Http)
        );
        assert_eq!(
            host_fn_shake_domain(HostFnId::TlsServe),
            Some(HostShakeDomain::Tls)
        );
        assert_eq!(host_fn_shake_domain(HostFnId::FmtPrint), None);
    }

    #[test]
    fn domains_for_hello_empty() {
        let empty = std::collections::BTreeSet::new();
        assert!(shake_domains_for_indices(&empty).is_empty());
    }
}
