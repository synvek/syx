//! ABI v2 / v2m pack helpers (W1045 / ADR-1390; W1046 / ADR-1391).

use syx_host_plugin::{
    dispatch_plugin_host, dot_f64_arrays, drop_handle, grow_copy_f64, pack_f64_le, pack_handle,
    pin_f64_values, register_v2probe_dot_in_process, register_v2probe_drop_in_process,
    register_v2probe_grow_in_process, register_v2probe_pin_in_process,
    register_v2probe_sum_in_process, register_v2probe_sumh_in_process, sum_f64_array, sum_handle,
    unpack_f64_le, unpack_handle, value_as_handle, PLUGIN_HEADER, SYX_PLUGIN_BUF_HANDLE,
    SYX_PLUGIN_V2M_GROW_INITIAL_CAP,
};
use syx_runtime_core::Value;

#[test]
fn plugin_header_keeps_v1_v2_and_adds_v2m() {
    assert!(PLUGIN_HEADER.contains("#define SYX_PLUGIN_API_VERSION 1"));
    assert!(PLUGIN_HEADER.contains("SYX_PLUGIN_API_VERSION_V2 2"));
    assert!(PLUGIN_HEADER.contains("SYX_PLUGIN_API_VERSION_V2M 3"));
    assert!(PLUGIN_HEADER.contains("syx_plugin_info_v2"));
    assert!(PLUGIN_HEADER.contains("syx_plugin_info_v2m"));
    assert!(PLUGIN_HEADER.contains("syx_plugin_iov"));
    assert!(PLUGIN_HEADER.contains("syx_plugin_alloc"));
    assert!(PLUGIN_HEADER.contains("syx_host_fn_v2m"));
    assert!(PLUGIN_HEADER.contains("SYX_PLUGIN_BUF_HANDLE"));
    assert!(PLUGIN_HEADER.contains("syx_plugin_borrow_fn"));
    assert!(PLUGIN_HEADER.contains("syx_plugin_pin_fn"));
    assert!(PLUGIN_HEADER.contains("syx_plugin_drop_fn"));
}

#[test]
fn pack_unpack_f64_roundtrip_and_sum() {
    let vals = vec![Value::F64(1.0), Value::F64(2.0), Value::F64(3.5)];
    let packed = pack_f64_le(&vals).expect("pack");
    assert_eq!(packed.len(), 24);
    let back = unpack_f64_le(&packed).expect("unpack");
    assert_eq!(back, vals);
    assert_eq!(sum_f64_array(&vals).unwrap(), 6.5);
}

#[test]
fn v2probe_in_process_dispatch_sum() {
    register_v2probe_sum_in_process();
    let mut stack = vec![Value::Array(vec![
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(3.0),
    ])];
    assert!(dispatch_plugin_host("__syx_plugin_v2probe_sum", &mut stack));
    let out = Value::materialize_array(stack.pop().unwrap()).expect("array out");
    assert_eq!(out, vec![Value::F64(6.0)]);
}

#[test]
fn v2probe_in_process_dispatch_dot() {
    register_v2probe_dot_in_process();
    let mut stack = vec![
        Value::Array(vec![Value::F64(1.0), Value::F64(2.0)]),
        Value::Array(vec![Value::F64(3.0), Value::F64(4.0)]),
    ];
    assert!(dispatch_plugin_host("__syx_plugin_v2probe_dot", &mut stack));
    let out = Value::materialize_array(stack.pop().unwrap()).expect("array out");
    assert_eq!(out, vec![Value::F64(11.0)]);
    assert_eq!(
        dot_f64_arrays(
            &[Value::F64(1.0), Value::F64(2.0)],
            &[Value::F64(3.0), Value::F64(4.0)]
        )
        .unwrap(),
        11.0
    );
}

#[test]
fn v2probe_in_process_grow_past_initial_cap() {
    register_v2probe_grow_in_process();
    let vals = vec![Value::F64(1.0), Value::F64(2.0), Value::F64(3.0)];
    let grown = grow_copy_f64(&vals, SYX_PLUGIN_V2M_GROW_INITIAL_CAP).expect("grow");
    assert_eq!(grown.len(), 24);
    assert!(grown.len() > SYX_PLUGIN_V2M_GROW_INITIAL_CAP);
    let mut stack = vec![Value::Array(vals.clone())];
    assert!(dispatch_plugin_host(
        "__syx_plugin_v2probe_grow",
        &mut stack
    ));
    let out = Value::materialize_array(stack.pop().unwrap()).expect("array out");
    assert_eq!(out, vals);
}

#[test]
fn handle_pack_roundtrip_and_pin_sumh_drop() {
    let packed = pack_handle(42);
    assert_eq!(packed.len(), 8);
    assert_eq!(unpack_handle(&packed).unwrap(), 42);
    assert_eq!(SYX_PLUGIN_BUF_HANDLE, 2);
    register_v2probe_pin_in_process();
    register_v2probe_sumh_in_process();
    register_v2probe_drop_in_process();
    let mut stack = vec![Value::Array(vec![
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(3.0),
    ])];
    assert!(dispatch_plugin_host("__syx_plugin_v2probe_pin", &mut stack));
    let h = stack.pop().unwrap();
    assert!(matches!(h, Value::I64(id) if id > 0));
    assert!(value_as_handle(&Value::F64(1.0)).is_err());
    let id = value_as_handle(&h).unwrap();
    assert_eq!(sum_handle(id).unwrap(), 6.0);
    let mut stack = vec![h.clone()];
    assert!(dispatch_plugin_host(
        "__syx_plugin_v2probe_sumh",
        &mut stack
    ));
    let out = Value::materialize_array(stack.pop().unwrap()).expect("array out");
    assert_eq!(out, vec![Value::F64(6.0)]);
    let mut stack = vec![h.clone()];
    assert!(dispatch_plugin_host(
        "__syx_plugin_v2probe_drop",
        &mut stack
    ));
    assert!(!dispatch_plugin_host(
        "__syx_plugin_v2probe_drop",
        &mut vec![h]
    ));
    assert!(drop_handle(id).is_err());
    let id2 = pin_f64_values(&[Value::F64(4.0), Value::F64(5.0)]).unwrap();
    assert_eq!(sum_handle(id2 as u64).unwrap(), 9.0);
    drop_handle(id2 as u64).unwrap();
}

#[cfg(feature = "p-faer")]
#[test]
fn faer_add_sub_same_shape_and_mismatch() {
    use syx_host_plugin::{
        add_handles, hadamard_handles, pin_shaped, register_faer_add_in_process,
        register_faer_from_in_process, register_faer_hadamard_in_process,
        register_faer_sub_in_process, register_faer_to_in_process, sub_handles,
    };
    register_faer_from_in_process();
    register_faer_add_in_process();
    register_faer_sub_in_process();
    register_faer_hadamard_in_process();
    register_faer_to_in_process();
    let a = pin_shaped(
        &[
            Value::F64(1.0),
            Value::F64(2.0),
            Value::F64(3.0),
            Value::F64(4.0),
        ],
        2,
        2,
    )
    .unwrap();
    let b = pin_shaped(
        &[
            Value::F64(10.0),
            Value::F64(20.0),
            Value::F64(30.0),
            Value::F64(40.0),
        ],
        2,
        2,
    )
    .unwrap();
    let s = add_handles(a as u64, b as u64).unwrap();
    let d = sub_handles(a as u64, b as u64).unwrap();
    let h = hadamard_handles(a as u64, b as u64).unwrap();
    assert_eq!(sum_handle(s as u64).unwrap(), 110.0);
    assert_eq!(sum_handle(d as u64).unwrap(), -90.0);
    assert_eq!(sum_handle(h as u64).unwrap(), 300.0);
    let wide = pin_shaped(&[Value::F64(1.0), Value::F64(2.0)], 1, 2).unwrap();
    assert!(add_handles(a as u64, wide as u64).is_err());
    assert!(sub_handles(a as u64, wide as u64).is_err());
    assert!(hadamard_handles(a as u64, wide as u64).is_err());
    drop_handle(a as u64).unwrap();
    drop_handle(b as u64).unwrap();
    drop_handle(s as u64).unwrap();
    drop_handle(d as u64).unwrap();
    drop_handle(h as u64).unwrap();
    drop_handle(wide as u64).unwrap();
    let mut stack = vec![
        Value::Array(vec![
            Value::F64(1.0),
            Value::F64(2.0),
            Value::F64(3.0),
            Value::F64(4.0),
        ]),
        Value::I64(2),
        Value::I64(2),
    ];
    assert!(dispatch_plugin_host("__syx_plugin_faer_from", &mut stack));
    let ah = stack.pop().unwrap();
    let mut stack = vec![
        Value::Array(vec![
            Value::F64(10.0),
            Value::F64(20.0),
            Value::F64(30.0),
            Value::F64(40.0),
        ]),
        Value::I64(2),
        Value::I64(2),
    ];
    assert!(dispatch_plugin_host("__syx_plugin_faer_from", &mut stack));
    let bh = stack.pop().unwrap();
    let mut stack = vec![ah.clone(), bh.clone()];
    assert!(dispatch_plugin_host("__syx_plugin_faer_add", &mut stack));
    let sh = stack.pop().unwrap();
    let mut stack = vec![ah.clone(), bh.clone()];
    assert!(dispatch_plugin_host("__syx_plugin_faer_sub", &mut stack));
    let dh = stack.pop().unwrap();
    let mut stack = vec![ah, bh];
    assert!(dispatch_plugin_host("__syx_plugin_faer_hadamard", &mut stack));
    let hh = stack.pop().unwrap();
    let mut stack = vec![sh];
    assert!(dispatch_plugin_host("__syx_plugin_faer_to", &mut stack));
    let sout = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(
        sout,
        vec![
            Value::F64(11.0),
            Value::F64(22.0),
            Value::F64(33.0),
            Value::F64(44.0)
        ]
    );
    let mut stack = vec![dh];
    assert!(dispatch_plugin_host("__syx_plugin_faer_to", &mut stack));
    let dout = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(
        dout,
        vec![
            Value::F64(-9.0),
            Value::F64(-18.0),
            Value::F64(-27.0),
            Value::F64(-36.0)
        ]
    );
    let mut stack = vec![hh];
    assert!(dispatch_plugin_host("__syx_plugin_faer_to", &mut stack));
    let hout = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(
        hout,
        vec![
            Value::F64(10.0),
            Value::F64(40.0),
            Value::F64(90.0),
            Value::F64(160.0)
        ]
    );
}

#[cfg(feature = "p-faer")]
#[test]
fn faer_from_mul_to_and_shape_mismatch() {
    use syx_host_plugin::{
        mul_handles, pin_shaped, pin_zeros, register_faer_from_in_process,
        register_faer_mul_in_process, register_faer_to_in_process, register_faer_zeros_in_process,
    };
    register_faer_from_in_process();
    register_faer_mul_in_process();
    register_faer_to_in_process();
    register_faer_zeros_in_process();
    let z = pin_zeros(2, 2).unwrap();
    assert_eq!(sum_handle(z as u64).unwrap(), 0.0);
    drop_handle(z as u64).unwrap();
    let mut stack = vec![
        Value::Array(vec![
            Value::F64(1.0),
            Value::F64(2.0),
            Value::F64(3.0),
            Value::F64(4.0),
        ]),
        Value::I64(2),
        Value::I64(2),
    ];
    assert!(dispatch_plugin_host("__syx_plugin_faer_from", &mut stack));
    let a = stack.pop().unwrap();
    let mut stack = vec![
        Value::Array(vec![
            Value::F64(1.0),
            Value::F64(0.0),
            Value::F64(0.0),
            Value::F64(1.0),
        ]),
        Value::I64(2),
        Value::I64(2),
    ];
    assert!(dispatch_plugin_host("__syx_plugin_faer_from", &mut stack));
    let b = stack.pop().unwrap();
    let mut stack = vec![a.clone(), b.clone()];
    assert!(dispatch_plugin_host("__syx_plugin_faer_mul", &mut stack));
    let c = stack.pop().unwrap();
    let mut stack = vec![c.clone()];
    assert!(dispatch_plugin_host("__syx_plugin_faer_to", &mut stack));
    let out = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(
        out,
        vec![
            Value::F64(1.0),
            Value::F64(2.0),
            Value::F64(3.0),
            Value::F64(4.0)
        ]
    );
    let wide = pin_shaped(&[Value::F64(1.0), Value::F64(2.0)], 1, 2).unwrap();
    let tall = pin_shaped(&[Value::F64(1.0), Value::F64(2.0), Value::F64(3.0)], 3, 1).unwrap();
    assert!(mul_handles(wide as u64, tall as u64).is_err());
    drop_handle(value_as_handle(&a).unwrap()).unwrap();
    drop_handle(value_as_handle(&b).unwrap()).unwrap();
    drop_handle(value_as_handle(&c).unwrap()).unwrap();
    drop_handle(wide as u64).unwrap();
    drop_handle(tall as u64).unwrap();
}

#[cfg(feature = "p-faer")]
#[test]
fn faer_solve_and_singular() {
    use syx_host_plugin::{
        pin_shaped, pin_zeros, register_faer_from_in_process, register_faer_solve_in_process,
        register_faer_to_in_process, solve_handles,
    };
    register_faer_from_in_process();
    register_faer_solve_in_process();
    register_faer_to_in_process();
    let mut stack = vec![
        Value::Array(vec![
            Value::F64(2.0),
            Value::F64(0.0),
            Value::F64(0.0),
            Value::F64(2.0),
        ]),
        Value::I64(2),
        Value::I64(2),
    ];
    assert!(dispatch_plugin_host("__syx_plugin_faer_from", &mut stack));
    let a = stack.pop().unwrap();
    let mut stack = vec![
        Value::Array(vec![Value::F64(2.0), Value::F64(4.0)]),
        Value::I64(2),
        Value::I64(1),
    ];
    assert!(dispatch_plugin_host("__syx_plugin_faer_from", &mut stack));
    let b = stack.pop().unwrap();
    let mut stack = vec![a.clone(), b.clone()];
    assert!(dispatch_plugin_host("__syx_plugin_faer_solve", &mut stack));
    let x = stack.pop().unwrap();
    let mut stack = vec![x.clone()];
    assert!(dispatch_plugin_host("__syx_plugin_faer_to", &mut stack));
    let out = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(out, vec![Value::F64(1.0), Value::F64(2.0)]);
    let wide = pin_shaped(&[Value::F64(1.0), Value::F64(2.0)], 1, 2).unwrap();
    let rhs = pin_shaped(&[Value::F64(1.0)], 1, 1).unwrap();
    assert!(solve_handles(wide as u64, rhs as u64).is_err());
    let zero = pin_zeros(2, 2).unwrap();
    let ones = pin_shaped(&[Value::F64(1.0), Value::F64(1.0)], 2, 1).unwrap();
    assert!(solve_handles(zero as u64, ones as u64).is_err());
    drop_handle(value_as_handle(&a).unwrap()).unwrap();
    drop_handle(value_as_handle(&b).unwrap()).unwrap();
    drop_handle(value_as_handle(&x).unwrap()).unwrap();
    drop_handle(wide as u64).unwrap();
    drop_handle(rhs as u64).unwrap();
    drop_handle(zero as u64).unwrap();
    drop_handle(ones as u64).unwrap();
}

#[cfg(feature = "p-faer")]
#[test]
fn faer_solve_and_singular_or_shape_mismatch() {
    use syx_host_plugin::{
        pin_shaped, pin_zeros, register_faer_from_in_process, register_faer_solve_in_process,
        register_faer_to_in_process, solve_handles,
    };
    register_faer_from_in_process();
    register_faer_solve_in_process();
    register_faer_to_in_process();
    let mut stack = vec![
        Value::Array(vec![
            Value::F64(2.0),
            Value::F64(0.0),
            Value::F64(0.0),
            Value::F64(2.0),
        ]),
        Value::I64(2),
        Value::I64(2),
    ];
    assert!(dispatch_plugin_host("__syx_plugin_faer_from", &mut stack));
    let a = stack.pop().unwrap();
    let mut stack = vec![
        Value::Array(vec![Value::F64(2.0), Value::F64(4.0)]),
        Value::I64(2),
        Value::I64(1),
    ];
    assert!(dispatch_plugin_host("__syx_plugin_faer_from", &mut stack));
    let b = stack.pop().unwrap();
    let mut stack = vec![a.clone(), b.clone()];
    assert!(dispatch_plugin_host("__syx_plugin_faer_solve", &mut stack));
    let x = stack.pop().unwrap();
    let mut stack = vec![x.clone()];
    assert!(dispatch_plugin_host("__syx_plugin_faer_to", &mut stack));
    let out = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(out, vec![Value::F64(1.0), Value::F64(2.0)]);
    let wide = pin_shaped(&[Value::F64(1.0), Value::F64(2.0)], 1, 2).unwrap();
    let rhs = pin_shaped(&[Value::F64(1.0), Value::F64(2.0)], 2, 1).unwrap();
    assert!(solve_handles(wide as u64, rhs as u64).is_err());
    let z = pin_zeros(2, 2).unwrap();
    assert!(solve_handles(z as u64, value_as_handle(&b).unwrap()).is_err());
    drop_handle(value_as_handle(&a).unwrap()).unwrap();
    drop_handle(value_as_handle(&b).unwrap()).unwrap();
    drop_handle(value_as_handle(&x).unwrap()).unwrap();
    drop_handle(wide as u64).unwrap();
    drop_handle(rhs as u64).unwrap();
    drop_handle(z as u64).unwrap();
}
