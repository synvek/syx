//! EC-M1-3 slice : Suite-1 pilot MirOps must not silently succeed
// was: ec_m1_3_pilot_mir_op
//! when routed through LLVM catch-all / mir-run `exec_mir_op` no-op path.

use syx_mir::{is_suite1_pilot_mir_op, MirOp};
use syx_mir_run::MirRuntime;

#[test]
fn suite1_pilot_predicate_covers_arith_call_string() {
    assert!(is_suite1_pilot_mir_op(&MirOp::Add {
        kind: syx_stdlib::PLATFORM_INT.to_byte()
    }));
    assert!(is_suite1_pilot_mir_op(&MirOp::ConstI32(1)));
    assert!(is_suite1_pilot_mir_op(&MirOp::ConstStr("a".into())));
    assert!(is_suite1_pilot_mir_op(&MirOp::Call("main".into())));
    assert!(is_suite1_pilot_mir_op(&MirOp::Jump(0)));
    assert!(is_suite1_pilot_mir_op(&MirOp::Return));
    assert!(!is_suite1_pilot_mir_op(&MirOp::ArrSorted));
}

#[test]
fn exec_mir_op_rejects_pilot_control_flow_as_noop() {
    let mut rt = MirRuntime::default();
    let mut memory = vec![0u8; 4096];
    let err = rt
        .exec_mir_op(&MirOp::Call("f".into()), &mut memory)
        .expect_err("Call via exec_mir_op must fail closed");
    assert!(
        err.code.as_deref() == Some("E_MIR_PILOT") || err.message.contains("EC-M1-3"),
        "{err:?}"
    );
}

#[test]
fn exec_mir_op_add_still_works() {
    let mut rt = MirRuntime::default();
    let mut memory = vec![0u8; 4096];
    rt.exec_mir_op(&MirOp::ConstI32(10), &mut memory)
        .expect("const");
    rt.exec_mir_op(&MirOp::ConstI32(5), &mut memory)
        .expect("const");
    rt.exec_mir_op(
        &MirOp::Add {
            kind: syx_stdlib::PLATFORM_INT.to_byte(),
        },
        &mut memory,
    )
    .expect("add");
    assert_eq!(rt.return_value(), 15);
}
