//! Phase 56: macro expansion pass registry.
//! Phase 130 Track A fingerprints (owned surfaces): `macro-token-v1:expr-token-mvp`,
//! `macro-service-v11:pointcut-name-mvp`.
//! Phase 131 Track A fingerprints: `macro-token-v2:decl-param-subst-mvp`,
//! `macro-service-v12:pointcut-execution-name-mvp`.
//! Phase 133 Track A fingerprints: `macro-token-v4:stmt-token-subst-mvp`,
//! `macro-service-v14:pointcut-within-name-mvp`.
//! Phase 134 Track A fingerprints: `macro-service-v15:pointcut-args-name-mvp`,
//! Phase 135 Track E fingerprints: `macro-service-v16:pointcut-and-compose-mvp`,
//! `macro-token-v6:decl-token-subst-mvp`.
//! `macro-token-v5:expr-token-subst-mvp`.
//! Phase 136 Track E fingerprints: `macro-service-v17:pointcut-or-compose-mvp`,
//! `macro-token-v7:token-stream-mvp`.

use syx_ast::Module;
use syx_diagnostics::SyxResult;

use crate::attached;
use crate::builtin;
use crate::context::MacroContext;
use crate::external;
use crate::user;
use crate::wrapper;

/// One expansion round: builtin → attached → user → external.
pub fn expand_one_round(module: &mut Module, ctx: &MacroContext) -> SyxResult<()> {
    builtin::expand_decls(&mut module.decls)?;
    attached::expand_attached_macros(module)?;
    wrapper::expand_property_wrappers(module)?;
    user::expand_user_macros(module)?;
    external::expand_external_macros(module)?;
    let _ = ctx;
    Ok(())
}
