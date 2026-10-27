//! Phase 135 Track D: PKCS#11 key label bind/check on stub HSM path (multi-slot);
//! wrong label reject; file persist + reload (stub; not hardware GA).

use std::fs;
use std::sync::Mutex;

use syx_driver::{
    farm_lease_hsm_key_label_doctor_export, farm_lease_pkcs11_active_slot_id,
    farm_lease_pkcs11_hsm_key_label_fingerprint, farm_lease_pkcs11_hsm_key_label_mvp_ready,
    farm_lease_pkcs11_hsm_multi_slot_bind, farm_lease_pkcs11_hsm_multi_slot_mvp_ready,
    farm_lease_pkcs11_key_label_loaded, farm_lease_pkcs11_key_label_persist,
    farm_lease_pkcs11_key_label_verify, farm_lease_pkcs11_session_pin_persist,
    farm_lease_pkcs11_session_pin_verify, farm_lease_pkcs11_slot_list,
    farm_lease_test_lock, ENV_FARM_PKCS11_KEY_LABEL_FILE, ENV_FARM_PKCS11_PIN_FILE,
};
use syx_driver::hsm_rotation_report::run_hsm_rotation_report;

static P135_FARM_LABEL_BUILD: Mutex<()> = Mutex::new(());

#[test]
fn p135_farm_token_v18_hsm_key_label_fingerprint() {
    assert_eq!(
        farm_lease_pkcs11_hsm_key_label_fingerprint(),
        "farm-lease-token-v18:hsm-key-label-mvp"
    );
    assert_eq!(farm_lease_hsm_key_label_doctor_export(), "hsm-key-label");
    let src = include_str!("../../src/farm_lease_auth.rs");
    assert!(src.contains("farm-lease-token-v18:hsm-key-label-mvp"));
    assert!(src.contains("hsm-key-label"));
    assert!(src.contains("farm_lease_pkcs11_key_label_persist"));
    assert!(src.contains("farm_lease_pkcs11_key_label_verify"));
    assert!(src.contains("SYX_FARM_PKCS11_KEY_LABEL_FILE"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("farm-lease-token-v18:hsm-key-label-mvp"));
    assert!(lib.contains("heap_v107_phase135_track_d_farm_dist_fingerprint"));
}

#[test]
fn p135_farm_hsm_key_label_doctor_export() {
    let report = run_hsm_rotation_report();
    assert!(report.hsm_key_label);
    assert_eq!(report.hsm_key_label_export, "hsm-key-label");
}

#[test]
fn p135_farm_pkcs11_mock_key_label_persist_check_bind() {
    let _g = farm_lease_test_lock();
    std::env::set_var("SYX_FARM_PKCS11_BRIDGE", "mock");

    let label_file = std::env::temp_dir().join(format!(
        "syx-p135-farm-label-{}.lbl",
        std::process::id()
    ));
    let pin_file = std::env::temp_dir().join(format!(
        "syx-p135-farm-pin-{}.pin",
        std::process::id()
    ));
    let _ = fs::remove_file(&label_file);
    let _ = fs::remove_file(&pin_file);
    std::env::set_var(
        ENV_FARM_PKCS11_KEY_LABEL_FILE,
        label_file.to_string_lossy().as_ref(),
    );
    std::env::set_var(
        ENV_FARM_PKCS11_PIN_FILE,
        pin_file.to_string_lossy().as_ref(),
    );

    farm_lease_pkcs11_session_pin_persist("stub-pin-p135").expect("persist pin");
    farm_lease_pkcs11_key_label_persist("lease-key-p135").expect("persist label");
    assert!(farm_lease_pkcs11_key_label_loaded());
    assert!(label_file.exists(), "label must persist to file");

    let slots = farm_lease_pkcs11_slot_list().expect("slot list");
    assert!(slots.len() >= 2, "need ≥2 slots, got {slots:?}");
    let slot_a = slots[0];

    let bound = farm_lease_pkcs11_hsm_multi_slot_bind(slot_a).expect("bind with label");
    assert_eq!(farm_lease_pkcs11_active_slot_id(), Some(slot_a));
    assert!(bound.contains("-label-lease-key-p135"), "bound ref: {bound}");
    assert!(farm_lease_pkcs11_hsm_multi_slot_mvp_ready());
    assert!(farm_lease_pkcs11_hsm_key_label_mvp_ready());

    farm_lease_pkcs11_key_label_verify("lease-key-p135").expect("verify ok");
    let wrong = farm_lease_pkcs11_key_label_verify("wrong-label").expect_err("wrong label");
    assert!(wrong.contains("mismatch"), "unexpected err: {wrong}");

    std::env::remove_var("SYX_FARM_PKCS11_BRIDGE");
    std::env::remove_var(ENV_FARM_PKCS11_KEY_LABEL_FILE);
    std::env::remove_var(ENV_FARM_PKCS11_PIN_FILE);
    let _ = fs::remove_file(&label_file);
    let _ = fs::remove_file(&pin_file);
}

#[test]
fn p135_farm_pkcs11_key_label_file_reload() {
    let _build_guard = P135_FARM_LABEL_BUILD.lock().unwrap();
    let _g = farm_lease_test_lock();
    std::env::set_var("SYX_FARM_PKCS11_BRIDGE", "mock");

    let label_file = std::env::temp_dir().join(format!(
        "syx-p135-farm-label-reload-{}.lbl",
        std::process::id()
    ));
    let pin_file = std::env::temp_dir().join(format!(
        "syx-p135-farm-pin-reload-{}.pin",
        std::process::id()
    ));
    let _ = fs::remove_file(&label_file);
    let _ = fs::remove_file(&pin_file);
    fs::write(&label_file, "reload-label-p135").expect("seed label file");
    fs::write(&pin_file, "reload-pin-p135").expect("seed pin file");
    std::env::set_var(
        ENV_FARM_PKCS11_KEY_LABEL_FILE,
        label_file.to_string_lossy().as_ref(),
    );
    std::env::set_var(
        ENV_FARM_PKCS11_PIN_FILE,
        pin_file.to_string_lossy().as_ref(),
    );

    farm_lease_pkcs11_session_pin_persist("reload-pin-p135").expect("persist pin");
    farm_lease_pkcs11_key_label_persist("reload-label-p135").expect("persist from file");
    assert!(farm_lease_pkcs11_key_label_loaded());
    farm_lease_pkcs11_session_pin_verify("reload-pin-p135").expect("pin ok");

    let slots = farm_lease_pkcs11_slot_list().expect("slots");
    let bound = farm_lease_pkcs11_hsm_multi_slot_bind(slots[0]).expect("bind after reload");
    assert!(bound.contains("reload-label-p135"), "bound: {bound}");

    std::env::remove_var("SYX_FARM_PKCS11_BRIDGE");
    std::env::remove_var(ENV_FARM_PKCS11_KEY_LABEL_FILE);
    std::env::remove_var(ENV_FARM_PKCS11_PIN_FILE);
    let _ = fs::remove_file(&label_file);
    let _ = fs::remove_file(&pin_file);
}
