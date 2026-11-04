//! PKCS#11 CKA_ID-shaped key id bind/check on stub HSM path (not hardware GA).

use std::fs;
use std::sync::Mutex;

use syx_driver::{
    farm_lease_hsm_key_id_doctor_export,
    farm_lease_pkcs11_active_slot_id,
    farm_lease_pkcs11_hsm_key_id_mvp_ready,
    farm_lease_pkcs11_hsm_key_label_mvp_ready,
    farm_lease_pkcs11_hsm_multi_slot_bind,
    farm_lease_pkcs11_key_id_loaded,
    farm_lease_pkcs11_key_id_persist,
    farm_lease_pkcs11_key_id_verify,
    farm_lease_pkcs11_key_label_persist,
    farm_lease_pkcs11_session_pin_persist,
    farm_lease_pkcs11_slot_list,
    farm_lease_test_lock,
    ENV_FARM_PKCS11_KEY_ID_FILE,
    ENV_FARM_PKCS11_KEY_LABEL_FILE,
    ENV_FARM_PKCS11_PIN_FILE,
};
static P136_FARM_KEY_ID_BUILD: Mutex<()> = Mutex::new(());


#[test]
fn farm_hsm_key_id_doctor_export() {
    assert!(farm_lease_pkcs11_hsm_key_id_mvp_ready());
    assert_eq!(farm_lease_hsm_key_id_doctor_export(), "hsm-key-id");
}

#[test]
fn farm_hsm_pkcs11_mock_key_id_persist_check_bind() {
    let _g = farm_lease_test_lock();
    std::env::set_var("SYX_FARM_PKCS11_BRIDGE", "mock");

    let id_file = std::env::temp_dir().join(format!(
        "syx-p136-farm-key-id-{}.id",
        std::process::id()
    ));
    let label_file = std::env::temp_dir().join(format!(
        "syx-p136-farm-label-{}.lbl",
        std::process::id()
    ));
    let pin_file = std::env::temp_dir().join(format!(
        "syx-p136-farm-pin-{}.pin",
        std::process::id()
    ));
    let _ = fs::remove_file(&id_file);
    let _ = fs::remove_file(&label_file);
    let _ = fs::remove_file(&pin_file);
    std::env::set_var(
        ENV_FARM_PKCS11_KEY_ID_FILE,
        id_file.to_string_lossy().as_ref(),
    );
    std::env::set_var(
        ENV_FARM_PKCS11_KEY_LABEL_FILE,
        label_file.to_string_lossy().as_ref(),
    );
    std::env::set_var(
        ENV_FARM_PKCS11_PIN_FILE,
        pin_file.to_string_lossy().as_ref(),
    );

    farm_lease_pkcs11_session_pin_persist("stub-pin-p136").expect("persist pin");
    farm_lease_pkcs11_key_label_persist("lease-key-p136").expect("persist label");
    farm_lease_pkcs11_key_id_persist("a1:b2:c3:d4").expect("persist key id");
    assert!(farm_lease_pkcs11_key_id_loaded());
    assert!(id_file.exists(), "key id must persist to file");

    let slots = farm_lease_pkcs11_slot_list().expect("slot list");
    assert!(slots.len() >= 2, "need ≥2 slots, got {slots:?}");
    let slot_a = slots[0];

    let bound = farm_lease_pkcs11_hsm_multi_slot_bind(slot_a).expect("bind with key id");
    assert_eq!(farm_lease_pkcs11_active_slot_id(), Some(slot_a));
    assert!(bound.contains("-cka-id-a1b2c3d4"), "bound ref: {bound}");
    assert!(farm_lease_pkcs11_hsm_key_label_mvp_ready());
    assert!(farm_lease_pkcs11_hsm_key_id_mvp_ready());

    farm_lease_pkcs11_key_id_verify("a1b2c3d4").expect("verify ok");
    let wrong = farm_lease_pkcs11_key_id_verify("deadbeef").expect_err("wrong id");
    assert!(wrong.contains("mismatch"), "unexpected err: {wrong}");
    let bad_shape = farm_lease_pkcs11_key_id_persist("not-hex").expect_err("bad shape");
    assert!(
        bad_shape.contains("CKA_ID") || bad_shape.contains("non-CKA_ID"),
        "unexpected err: {bad_shape}"
    );

    std::env::remove_var("SYX_FARM_PKCS11_BRIDGE");
    std::env::remove_var(ENV_FARM_PKCS11_KEY_ID_FILE);
    std::env::remove_var(ENV_FARM_PKCS11_KEY_LABEL_FILE);
    std::env::remove_var(ENV_FARM_PKCS11_PIN_FILE);
    let _ = fs::remove_file(&id_file);
    let _ = fs::remove_file(&label_file);
    let _ = fs::remove_file(&pin_file);
}

#[test]
fn farm_hsm_pkcs11_key_id_file_reload() {
    let _build_guard = P136_FARM_KEY_ID_BUILD.lock().unwrap();
    let _g = farm_lease_test_lock();
    std::env::set_var("SYX_FARM_PKCS11_BRIDGE", "mock");

    let id_file = std::env::temp_dir().join(format!(
        "syx-p136-farm-key-id-reload-{}.id",
        std::process::id()
    ));
    let label_file = std::env::temp_dir().join(format!(
        "syx-p136-farm-label-reload-{}.lbl",
        std::process::id()
    ));
    let pin_file = std::env::temp_dir().join(format!(
        "syx-p136-farm-pin-reload-{}.pin",
        std::process::id()
    ));
    let _ = fs::remove_file(&id_file);
    let _ = fs::remove_file(&label_file);
    let _ = fs::remove_file(&pin_file);
    fs::write(&id_file, "01020304").expect("seed key id file");
    fs::write(&label_file, "reload-label-p136").expect("seed label file");
    fs::write(&pin_file, "reload-pin-p136").expect("seed pin file");
    std::env::set_var(
        ENV_FARM_PKCS11_KEY_ID_FILE,
        id_file.to_string_lossy().as_ref(),
    );
    std::env::set_var(
        ENV_FARM_PKCS11_KEY_LABEL_FILE,
        label_file.to_string_lossy().as_ref(),
    );
    std::env::set_var(
        ENV_FARM_PKCS11_PIN_FILE,
        pin_file.to_string_lossy().as_ref(),
    );

    farm_lease_pkcs11_session_pin_persist("reload-pin-p136").expect("persist pin");
    farm_lease_pkcs11_key_label_persist("reload-label-p136").expect("persist label");
    farm_lease_pkcs11_key_id_persist("01020304").expect("persist from file");
    assert!(farm_lease_pkcs11_key_id_loaded());

    let slots = farm_lease_pkcs11_slot_list().expect("slots");
    let bound = farm_lease_pkcs11_hsm_multi_slot_bind(slots[0]).expect("bind after reload");
    assert!(bound.contains("01020304"), "bound: {bound}");

    std::env::remove_var("SYX_FARM_PKCS11_BRIDGE");
    std::env::remove_var(ENV_FARM_PKCS11_KEY_ID_FILE);
    std::env::remove_var(ENV_FARM_PKCS11_KEY_LABEL_FILE);
    std::env::remove_var(ENV_FARM_PKCS11_PIN_FILE);
    let _ = fs::remove_file(&id_file);
    let _ = fs::remove_file(&label_file);
    let _ = fs::remove_file(&pin_file);
}
