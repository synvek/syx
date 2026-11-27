//! C-3cg: HostFn arity pilot expand — MIR081 covers new batch ids.

use smol_str::SmolStr;
use syx_mir::{
    verify_mir_call_host_arity_floor, MirFunction, MirInstr, MirModule, MirOp,
};
use syx_stdlib::{host_fn_arity, host_fn_index, HostFnId};

fn flat_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![],
        instructions: ops
            .into_iter()
            .map(|op| MirInstr {
                dest: None,
                op,
                span: None,
            })
            .collect(),
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

fn idx(id: HostFnId) -> u32 {
    host_fn_index(id)
}

#[test]
fn expand_table_samples_match_vm() {
    // One sample per expand group (C-3cg table).
    let samples: &[(HostFnId, u32, u32)] = &[
        (HostFnId::DirCreate, 1, 1),
        (HostFnId::FileCopy, 2, 1),
        (HostFnId::JsonEncode, 1, 1),
        (HostFnId::RegexMatch, 2, 1),
        (HostFnId::RegexCompile, 1, 1),
        (HostFnId::HasherCombine, 2, 1),
        (HostFnId::HasherFinalize, 1, 1),
        (HostFnId::HttpGet, 1, 1),
        (HostFnId::HttpPost, 2, 1),
        (HostFnId::HttpResponseJson, 1, 1),
        (HostFnId::MathFloatTan, 1, 1),
        (HostFnId::MathFloatAtan2, 2, 1),
    ];
    for &(id, pop, push) in samples {
        assert_eq!(
            host_fn_arity(idx(id)),
            Some((pop, push)),
            "{id:?} arity"
        );
    }
}

#[test]
fn expand_pilot_count_at_least_41() {
    let n = HostFnId::all()
        .iter()
        .filter(|id| host_fn_arity(host_fn_index(**id)).is_some())
        .count();
    assert!(n >= 41, "pilot size {n} < 41 after C-3cg expand");
}

#[test]
fn http_get_underflow_mir081() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::CallHost(idx(HostFnId::HttpGet)), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_call_host_arity_floor(&mir).expect_err("HttpGet");
    assert_eq!(err.code.as_deref(), Some("MIR081"), "{err}");
}

#[test]
fn file_copy_underflow_mir081() {
    // pop=2: one value still underflows.
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![
                MirOp::ConstI32(1),
                MirOp::CallHost(idx(HostFnId::FileCopy)),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    let err = verify_mir_call_host_arity_floor(&mir).expect_err("FileCopy");
    assert_eq!(err.code.as_deref(), Some("MIR081"), "{err}");
}

#[test]
fn regex_match_underflow_mir081() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::CallHost(idx(HostFnId::RegexMatch)), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_call_host_arity_floor(&mir).expect_err("RegexMatch");
    assert_eq!(err.code.as_deref(), Some("MIR081"), "{err}");
}

#[test]
fn http_get_ok_height() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::CallHost(idx(HostFnId::HttpGet)),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_call_host_arity_floor(&mir).expect("HttpGet ok");
}

#[test]
fn file_copy_ok_height() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![
                MirOp::ConstI32(1),
                MirOp::ConstI32(2),
                MirOp::CallHost(idx(HostFnId::FileCopy)),
                MirOp::Return,
            ],
        )],
        ..Default::default()
    };
    verify_mir_call_host_arity_floor(&mir).expect("FileCopy ok");
}

#[test]
fn unknown_arity_still_abandons() {
    assert_eq!(host_fn_arity(u32::MAX), None);
    let mir = MirModule {
        functions: vec![flat_fn(
            "ok",
            vec![MirOp::CallHost(u32::MAX), MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_call_host_arity_floor(&mir).expect("unknown abandon");
}

#[test]
fn excluded_hosts_remain_none() {
    // C-3cg non-batch: zero-pop / special Http* stay abandon.
    assert_eq!(host_fn_arity(idx(HostFnId::HasherNew)), None);
    assert_eq!(host_fn_arity(idx(HostFnId::HttpServe)), None);
    assert_eq!(host_fn_arity(idx(HostFnId::HttpShutdown)), None);
    assert_eq!(host_fn_arity(idx(HostFnId::HttpRequestAsync)), None);
}
