//! W1050 / ADR-1395: real faerlin cdylib load + F64LE mul smoke.
//! W1051 / ADR-1396: HANDLE×cdylib hmul via host_ops borrow/pin.
//! W1052 / ADR-1397: HANDLE×cdylib hsolve (faer LU).
//! W1053 / ADR-1398: HANDLE×cdylib hfrom/hto packed round-trip.
//! W1055 / ADR-1400: HANDLE×cdylib hdrop via host_ops drop_fn.
//! W1059 / ADR-1405: F64LE packed zeros via v2 `__syx_plugin_faer_cdylib_zeros`.
//! W1105 / ADR-1451: HANDLE×cdylib hadd/hsub via host_ops borrow/pin.

use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

use syx_host_plugin::{
    buf_to_values, dispatch_plugin_host, drop_handle, load_library, pin_shaped, pin_zeros,
    unload_plugin, value_as_handle, PluginHostManifest, PluginManifest,
};
use syx_runtime_core::Value;

static FAERLIN_TEST_LOCK: Mutex<()> = Mutex::new(());

fn faerlin_test_lock() -> std::sync::MutexGuard<'static, ()> {
    FAERLIN_TEST_LOCK
        .lock()
        .unwrap_or_else(|p| p.into_inner())
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn faerlin_dir() -> PathBuf {
    repo_root().join("plugins/faerlin")
}

fn platform_library_name() -> &'static str {
    if cfg!(target_os = "macos") {
        "libfaerlin_plugin.dylib"
    } else if cfg!(target_os = "windows") {
        "faerlin_plugin.dll"
    } else {
        "libfaerlin_plugin.so"
    }
}

fn build_faerlin_cdylib() -> PathBuf {
    let dir = faerlin_dir();
    let manifest = dir.join("Cargo.toml");
    assert!(
        manifest.is_file(),
        "plugins/faerlin/Cargo.toml missing (W1050–W1059)"
    );
    let status = Command::new("cargo")
        .args(["build", "--manifest-path"])
        .arg(&manifest)
        .status()
        .expect("spawn cargo build faerlin");
    assert!(status.success(), "cargo build plugins/faerlin failed");
    let lib = dir.join("target/debug").join(platform_library_name());
    assert!(
        lib.is_file(),
        "faerlin cdylib missing after build: {}",
        lib.display()
    );
    lib
}

fn faerlin_manifest(library_file_name: &str) -> PluginManifest {
    PluginManifest {
        id: "faerlin".into(),
        api_version: 3,
        library: library_file_name.into(),
        hosts: vec![
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_mul".into(),
                syx_name: Some("faerCdylibMul".into()),
                module: None,
                return_type: "Array".into(),
                params: vec!["packed: Array".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_solve".into(),
                syx_name: Some("faerCdylibSolve".into()),
                module: None,
                return_type: "Array".into(),
                params: vec!["packed: Array".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_zeros".into(),
                syx_name: Some("faerCdylibZeros".into()),
                module: None,
                return_type: "Array".into(),
                params: vec!["packed: Array".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_add".into(),
                syx_name: Some("faerCdylibAdd".into()),
                module: None,
                return_type: "Array".into(),
                params: vec!["packed: Array".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_sub".into(),
                syx_name: Some("faerCdylibSub".into()),
                module: None,
                return_type: "Array".into(),
                params: vec!["packed: Array".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_hadamard".into(),
                syx_name: Some("faerCdylibHadamard".into()),
                module: None,
                return_type: "Array".into(),
                params: vec!["packed: Array".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_hmul".into(),
                syx_name: Some("faerCdylibHmul".into()),
                module: None,
                return_type: "Int".into(),
                params: vec!["a: Int".into(), "b: Int".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_hsolve".into(),
                syx_name: Some("faerCdylibHsolve".into()),
                module: None,
                return_type: "Int".into(),
                params: vec!["a: Int".into(), "b: Int".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_hfrom".into(),
                syx_name: Some("faerCdylibHfrom".into()),
                module: None,
                return_type: "Int".into(),
                params: vec!["packed: Array".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_hto".into(),
                syx_name: Some("faerCdylibHto".into()),
                module: None,
                return_type: "Array".into(),
                params: vec!["handle: Int".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_hdrop".into(),
                syx_name: Some("faerCdylibHdrop".into()),
                module: None,
                return_type: "Int".into(),
                params: vec!["handle: Int".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_hadd".into(),
                syx_name: Some("faerCdylibHadd".into()),
                module: None,
                return_type: "Int".into(),
                params: vec!["a: Int".into(), "b: Int".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_hsub".into(),
                syx_name: Some("faerCdylibHsub".into()),
                module: None,
                return_type: "Int".into(),
                params: vec!["a: Int".into(), "b: Int".into()],
            },
            PluginHostManifest {
                name: "__syx_plugin_faer_cdylib_hhadamard".into(),
                syx_name: Some("faerCdylibHhadamard".into()),
                module: None,
                return_type: "Int".into(),
                params: vec!["a: Int".into(), "b: Int".into()],
            },
        ],
        min_syx_version: Some("117.0".into()),
        capabilities: vec!["linalg".into()],
        signature: None,
    }
}

#[test]
fn faerlin_cdylib_load_and_f64le_mul() {
    let _guard = faerlin_test_lock();
    let lib = build_faerlin_cdylib();
    let manifest = faerlin_manifest(platform_library_name());
    load_library(&lib, &manifest).expect("dlopen faerlin cdylib");
    // 2x2 A = [[1,2],[3,4]] × I → same
    // packed: nrows_a,ncols_a,nrows_b,ncols_b, A..., B...
    let packed = vec![
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(3.0),
        Value::F64(4.0),
        Value::F64(1.0),
        Value::F64(0.0),
        Value::F64(0.0),
        Value::F64(1.0),
    ];
    let mut stack = vec![Value::Array(packed)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_mul", &mut stack),
        "cdylib F64LE mul dispatch failed"
    );
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
    // shape mismatch must fail (not silent zeros)
    let bad = vec![
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(3.0),
        Value::F64(1.0),
        Value::F64(1.0),
        Value::F64(0.0),
        Value::F64(0.0),
        Value::F64(1.0),
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(3.0),
    ];
    let mut stack = vec![Value::Array(bad)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_mul", &mut stack),
        "shape mismatch must fail"
    );
    let _ = unload_plugin("faerlin");
}

#[test]
fn faerlin_cdylib_f64le_packed_solve() {
    let _guard = faerlin_test_lock();
    let lib = build_faerlin_cdylib();
    let manifest = faerlin_manifest(platform_library_name());
    load_library(&lib, &manifest).expect("dlopen faerlin cdylib");
    // 2x2 A = [[2,0],[0,2]], b = [[2],[4]] → x = [1, 2]
    let packed = vec![
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(0.0),
        Value::F64(0.0),
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(4.0),
    ];
    let mut stack = vec![Value::Array(packed)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_solve", &mut stack),
        "cdylib F64LE packed solve dispatch failed"
    );
    let out = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(out, vec![Value::F64(1.0), Value::F64(2.0)]);
    let wide = vec![
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(1.0),
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(1.0),
    ];
    let mut stack = vec![Value::Array(wide)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_solve", &mut stack),
        "non-square A must fail"
    );
    let mismatch = vec![
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(1.0),
        Value::F64(1.0),
        Value::F64(0.0),
        Value::F64(0.0),
        Value::F64(1.0),
        Value::F64(1.0),
    ];
    let mut stack = vec![Value::Array(mismatch)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_solve", &mut stack),
        "row mismatch must fail"
    );
    let singular = vec![
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(0.0),
        Value::F64(0.0),
        Value::F64(0.0),
        Value::F64(0.0),
        Value::F64(1.0),
        Value::F64(1.0),
    ];
    let mut stack = vec![Value::Array(singular)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_solve", &mut stack),
        "singular A must fail"
    );
    let _ = unload_plugin("faerlin");
}

#[test]
fn faerlin_cdylib_handle_borrow_hmul() {
    let _guard = faerlin_test_lock();
    let lib = build_faerlin_cdylib();
    let manifest = faerlin_manifest(platform_library_name());
    load_library(&lib, &manifest).expect("dlopen faerlin cdylib");
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
    let i = pin_shaped(
        &[
            Value::F64(1.0),
            Value::F64(0.0),
            Value::F64(0.0),
            Value::F64(1.0),
        ],
        2,
        2,
    )
    .unwrap();
    let mut stack = vec![Value::I64(a), Value::I64(i)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hmul", &mut stack),
        "HANDLE×cdylib hmul dispatch failed"
    );
    let cid = value_as_handle(&stack.pop().unwrap()).expect("result handle");
    let out = buf_to_values(cid).expect("to");
    assert_eq!(
        out,
        vec![
            Value::F64(1.0),
            Value::F64(2.0),
            Value::F64(3.0),
            Value::F64(4.0)
        ]
    );
    let tall = pin_shaped(&[Value::F64(1.0), Value::F64(2.0), Value::F64(3.0)], 3, 1).unwrap();
    let mut stack = vec![Value::I64(a), Value::I64(tall)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hmul", &mut stack),
        "shape mismatch must fail"
    );
    let mut stack = vec![Value::I64(a), Value::I64(9_999_999)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hmul", &mut stack),
        "unknown handle must fail"
    );
    drop_handle(a as u64).unwrap();
    drop_handle(i as u64).unwrap();
    drop_handle(cid).unwrap();
    drop_handle(tall as u64).unwrap();
    let _ = unload_plugin("faerlin");
}

#[test]
fn faerlin_cdylib_handle_hsolve() {
    let _guard = faerlin_test_lock();
    let lib = build_faerlin_cdylib();
    let manifest = faerlin_manifest(platform_library_name());
    load_library(&lib, &manifest).expect("dlopen faerlin cdylib");
    let a = pin_shaped(
        &[
            Value::F64(2.0),
            Value::F64(0.0),
            Value::F64(0.0),
            Value::F64(2.0),
        ],
        2,
        2,
    )
    .unwrap();
    let b = pin_shaped(&[Value::F64(2.0), Value::F64(4.0)], 2, 1).unwrap();
    let mut stack = vec![Value::I64(a), Value::I64(b)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hsolve", &mut stack),
        "HANDLE×cdylib hsolve dispatch failed"
    );
    let xid = value_as_handle(&stack.pop().unwrap()).expect("result handle");
    let out = buf_to_values(xid).expect("to");
    assert_eq!(out, vec![Value::F64(1.0), Value::F64(2.0)]);
    let wide = pin_shaped(&[Value::F64(1.0), Value::F64(2.0)], 1, 2).unwrap();
    let rhs = pin_shaped(&[Value::F64(1.0)], 1, 1).unwrap();
    let mut stack = vec![Value::I64(wide), Value::I64(rhs)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hsolve", &mut stack),
        "non-square A must fail"
    );
    let zero = pin_zeros(2, 2).unwrap();
    let ones = pin_shaped(&[Value::F64(1.0), Value::F64(1.0)], 2, 1).unwrap();
    let mut stack = vec![Value::I64(zero), Value::I64(ones)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hsolve", &mut stack),
        "singular A must fail"
    );
    let mut stack = vec![Value::I64(a), Value::I64(9_999_999)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hsolve", &mut stack),
        "unknown handle must fail"
    );
    drop_handle(a as u64).unwrap();
    drop_handle(b as u64).unwrap();
    drop_handle(xid).unwrap();
    drop_handle(wide as u64).unwrap();
    drop_handle(rhs as u64).unwrap();
    drop_handle(zero as u64).unwrap();
    drop_handle(ones as u64).unwrap();
    let _ = unload_plugin("faerlin");
}

#[test]
fn faerlin_cdylib_handle_hfrom_hto() {
    let _guard = faerlin_test_lock();
    let lib = build_faerlin_cdylib();
    let manifest = faerlin_manifest(platform_library_name());
    load_library(&lib, &manifest).expect("dlopen faerlin cdylib");
    let packed = vec![
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(3.0),
        Value::F64(4.0),
    ];
    let mut stack = vec![Value::Array(packed.clone())];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hfrom", &mut stack),
        "HANDLE×cdylib hfrom dispatch failed"
    );
    let hid = value_as_handle(&stack.pop().unwrap()).expect("from handle");
    let mut stack = vec![Value::I64(hid as i64)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hto", &mut stack),
        "HANDLE×cdylib hto dispatch failed"
    );
    let out = Value::materialize_array(stack.pop().unwrap()).expect("packed");
    assert_eq!(out, packed);
    let mut stack = vec![Value::I64(9_999_999)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hto", &mut stack),
        "unknown handle must fail"
    );
    let empty = vec![];
    let mut stack = vec![Value::Array(empty)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hfrom", &mut stack),
        "empty packing must fail"
    );
    let frac = vec![
        Value::F64(2.5),
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(3.0),
        Value::F64(4.0),
    ];
    let mut stack = vec![Value::Array(frac)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hfrom", &mut stack),
        "non-integer dim must fail"
    );
    let mismatch = vec![
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(3.0),
    ];
    let mut stack = vec![Value::Array(mismatch)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hfrom", &mut stack),
        "len != nrows*ncols must fail"
    );
    drop_handle(hid).unwrap();
    let _ = unload_plugin("faerlin");
}

#[test]
fn faerlin_cdylib_handle_hdrop() {
    let _guard = faerlin_test_lock();
    let lib = build_faerlin_cdylib();
    let manifest = faerlin_manifest(platform_library_name());
    load_library(&lib, &manifest).expect("dlopen faerlin cdylib");
    let packed = vec![
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(7.0),
        Value::F64(8.0),
    ];
    let mut stack = vec![Value::Array(packed)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hfrom", &mut stack),
        "hfrom before hdrop must succeed"
    );
    let hid = value_as_handle(&stack.pop().unwrap()).expect("from handle");
    let mut stack = vec![Value::I64(hid as i64)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hdrop", &mut stack),
        "HANDLE×cdylib hdrop dispatch failed"
    );
    let rc = stack.pop().unwrap();
    assert_eq!(rc, Value::I64(0), "hdrop success must return Int 0");
    let mut stack = vec![Value::I64(hid as i64)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hdrop", &mut stack),
        "double drop must fail"
    );
    let mut stack = vec![Value::I64(hid as i64)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hto", &mut stack),
        "hto after drop must fail"
    );
    let mut stack = vec![Value::I64(9_999_999)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hdrop", &mut stack),
        "unknown handle must fail"
    );
    let _ = unload_plugin("faerlin");
}

#[test]
fn faerlin_cdylib_f64le_packed_zeros() {
    let _guard = faerlin_test_lock();
    let lib = build_faerlin_cdylib();
    let manifest = faerlin_manifest(platform_library_name());
    load_library(&lib, &manifest).expect("dlopen faerlin cdylib");
    let packed = vec![Value::F64(2.0), Value::F64(2.0)];
    let mut stack = vec![Value::Array(packed)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_zeros", &mut stack),
        "cdylib F64LE zeros dispatch failed"
    );
    let out = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(
        out,
        vec![
            Value::F64(0.0),
            Value::F64(0.0),
            Value::F64(0.0),
            Value::F64(0.0)
        ]
    );
    let mut stack = vec![Value::Array(vec![Value::F64(0.0), Value::F64(2.0)])];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_zeros", &mut stack),
        "zero rows must fail"
    );
    let _ = unload_plugin("faerlin");
}

#[test]
fn faerlin_cdylib_f64le_packed_add_sub() {
    let _guard = faerlin_test_lock();
    let lib = build_faerlin_cdylib();
    let manifest = faerlin_manifest(platform_library_name());
    load_library(&lib, &manifest).expect("dlopen faerlin cdylib");
    // 2x2 [1,2,3,4] ± [10,20,30,40]
    let packed = vec![
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(3.0),
        Value::F64(4.0),
        Value::F64(10.0),
        Value::F64(20.0),
        Value::F64(30.0),
        Value::F64(40.0),
    ];
    let mut stack = vec![Value::Array(packed.clone())];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_add", &mut stack),
        "cdylib F64LE add dispatch failed"
    );
    let add_out = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(
        add_out,
        vec![
            Value::F64(11.0),
            Value::F64(22.0),
            Value::F64(33.0),
            Value::F64(44.0)
        ]
    );
    let mut stack = vec![Value::Array(packed.clone())];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_sub", &mut stack),
        "cdylib F64LE sub dispatch failed"
    );
    let sub_out = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(
        sub_out,
        vec![
            Value::F64(-9.0),
            Value::F64(-18.0),
            Value::F64(-27.0),
            Value::F64(-36.0)
        ]
    );
    let mut stack = vec![Value::Array(packed)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hadamard", &mut stack),
        "cdylib F64LE hadamard dispatch failed"
    );
    let had_out = Value::materialize_array(stack.pop().unwrap()).expect("array");
    assert_eq!(
        had_out,
        vec![
            Value::F64(10.0),
            Value::F64(40.0),
            Value::F64(90.0),
            Value::F64(160.0)
        ]
    );
    // shape mismatch must fail (not silent zeros)
    let bad = vec![
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(2.0),
        Value::F64(3.0),
        Value::F64(1.0),
        Value::F64(2.0),
        Value::F64(3.0),
        Value::F64(4.0),
        Value::F64(10.0),
        Value::F64(20.0),
        Value::F64(30.0),
        Value::F64(40.0),
        Value::F64(50.0),
        Value::F64(60.0),
    ];
    let mut stack = vec![Value::Array(bad.clone())];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_add", &mut stack),
        "shape mismatch must fail for cdylib add"
    );
    let mut stack = vec![Value::Array(bad)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hadamard", &mut stack),
        "shape mismatch must fail for cdylib hadamard"
    );
    let _ = unload_plugin("faerlin");
}

#[test]
fn faerlin_cdylib_handle_hadd_hsub() {
    let _guard = faerlin_test_lock();
    let lib = build_faerlin_cdylib();
    let manifest = faerlin_manifest(platform_library_name());
    load_library(&lib, &manifest).expect("dlopen faerlin cdylib");
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
    let mut stack = vec![Value::I64(a), Value::I64(b)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hadd", &mut stack),
        "HANDLE×cdylib hadd dispatch failed"
    );
    let add_id = value_as_handle(&stack.pop().unwrap()).expect("add handle");
    let add_out = buf_to_values(add_id).expect("hto add");
    assert_eq!(
        add_out,
        vec![
            Value::F64(11.0),
            Value::F64(22.0),
            Value::F64(33.0),
            Value::F64(44.0)
        ]
    );
    let mut stack = vec![Value::I64(a), Value::I64(b)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hsub", &mut stack),
        "HANDLE×cdylib hsub dispatch failed"
    );
    let sub_id = value_as_handle(&stack.pop().unwrap()).expect("sub handle");
    let sub_out = buf_to_values(sub_id).expect("hto sub");
    assert_eq!(
        sub_out,
        vec![
            Value::F64(-9.0),
            Value::F64(-18.0),
            Value::F64(-27.0),
            Value::F64(-36.0)
        ]
    );
    let mut stack = vec![Value::I64(a), Value::I64(b)];
    assert!(
        dispatch_plugin_host("__syx_plugin_faer_cdylib_hhadamard", &mut stack),
        "HANDLE×cdylib hhadamard dispatch failed"
    );
    let had_id = value_as_handle(&stack.pop().unwrap()).expect("hadamard handle");
    let had_out = buf_to_values(had_id).expect("hto hadamard");
    assert_eq!(
        had_out,
        vec![
            Value::F64(10.0),
            Value::F64(40.0),
            Value::F64(90.0),
            Value::F64(160.0)
        ]
    );
    let tall = pin_shaped(&[Value::F64(1.0), Value::F64(2.0), Value::F64(3.0)], 3, 1).unwrap();
    let mut stack = vec![Value::I64(a), Value::I64(tall)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hadd", &mut stack),
        "shape mismatch must fail for hadd"
    );
    let mut stack = vec![Value::I64(a), Value::I64(tall)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hhadamard", &mut stack),
        "shape mismatch must fail for hhadamard"
    );
    let mut stack = vec![Value::I64(a), Value::I64(9_999_999)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hsub", &mut stack),
        "unknown handle must fail for hsub"
    );
    let mut stack = vec![Value::I64(a), Value::I64(9_999_999)];
    assert!(
        !dispatch_plugin_host("__syx_plugin_faer_cdylib_hhadamard", &mut stack),
        "unknown handle must fail for hhadamard"
    );
    drop_handle(a as u64).unwrap();
    drop_handle(b as u64).unwrap();
    drop_handle(tall as u64).unwrap();
    drop_handle(add_id as u64).unwrap();
    drop_handle(sub_id as u64).unwrap();
    drop_handle(had_id as u64).unwrap();
    let _ = unload_plugin("faerlin");
}

#[test]
fn faerlin_cdylib_crate_not_workspace_member() {
    let root_toml = std::fs::read_to_string(repo_root().join("Cargo.toml")).unwrap();
    assert!(
        root_toml.contains("members = [\"crates/*\"]")
            || root_toml.contains("members = [\"crates/*\","),
        "workspace members must stay crates/* only"
    );
    assert!(
        !root_toml.contains("plugins/faerlin") && !root_toml.contains("plugins/*"),
        "plugins/faerlin must not be a workspace member"
    );
    assert!(faerlin_dir().join("Cargo.toml").is_file());
    assert!(faerlin_dir().join("src/lib.rs").is_file());
    assert!(faerlin_dir().join("plugin.json").is_file());
}
