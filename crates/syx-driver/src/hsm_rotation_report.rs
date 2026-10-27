//! Phase 81: HSM rotation cron report for `syx doctor --hsm-rotation-report`.
//! Phase 128: also exports `hsm-ref-bound` capability when PKCS#11 bind MVP is present.
//! Phase 129: also exports `hsm-sign-bound` when C_Sign+hsm-ref joint verify MVP is present.
//! Phase 130: also exports `hsm-sign-verify` when C_Sign→C_Verify+hsm-ref MVP is present.
//! Phase 131: also exports `hsm-digest-bound` when C_Digest+hsm-ref joint Sign/Verify MVP is present.
//! Phase 132: also exports `hsm-digest-streaming` when C_DigestInit/Update/Final+hsm-ref MVP is present.
//! Phase 133: also exports `hsm-multi-slot` when C_GetSlotList multi-slot MVP is present.
//! Phase 134: also exports `hsm-session-pin` when session PIN persist/check MVP is present.
//! Phase 135: also exports `hsm-key-label` when key label bind/check MVP is present.
//! Phase 136: also exports `hsm-key-id` when CKA_ID-shaped key id bind/check MVP is present.

#[derive(Debug, Clone, Default)]
pub struct HsmRotationReport {
    pub lock_v15_api: bool,
    pub hsm_rotation_cron_field: bool,
    pub billing_settlement_field: bool,
    pub marketplace_epoch_field: bool,
    pub hsm_ref_bound: bool,
    pub hsm_ref_bound_export: &'static str,
    pub hsm_sign_bound: bool,
    pub hsm_sign_bound_export: &'static str,
    pub hsm_sign_verify: bool,
    pub hsm_sign_verify_export: &'static str,
    pub hsm_digest_bound: bool,
    pub hsm_digest_bound_export: &'static str,
    pub hsm_digest_streaming: bool,
    pub hsm_digest_streaming_export: &'static str,
    pub hsm_multi_slot: bool,
    pub hsm_multi_slot_export: &'static str,
    pub hsm_session_pin: bool,
    pub hsm_session_pin_export: &'static str,
    pub hsm_key_label: bool,
    pub hsm_key_label_export: &'static str,
    pub hsm_key_id: bool,
    pub hsm_key_id_export: &'static str,
}

pub fn run_hsm_rotation_report() -> HsmRotationReport {
    let lock_src = include_str!("../../syx-pkg/src/lockfile.rs");
    let farm_src = include_str!("farm_lease_auth.rs");
    let bound = farm_src.contains("farm-lease-token-v11:hsm-ref-bound-mvp")
        && farm_src.contains("C_GetAttributeValue")
        && farm_src.contains("hsm-ref-bound");
    let sign_bound = farm_src.contains("farm-lease-token-v12:hsm-sign-bound-mvp")
        && farm_src.contains("C_Sign")
        && farm_src.contains("hsm-sign-bound")
        && farm_src.contains("farm_lease_pkcs11_hsm_sign_bound");
    let sign_verify = farm_src.contains("farm-lease-token-v13:hsm-sign-verify-mvp")
        && farm_src.contains("C_Verify")
        && farm_src.contains("hsm-sign-verify")
        && farm_src.contains("farm_lease_pkcs11_hsm_sign_verify");
    let digest_bound = farm_src.contains("farm-lease-token-v14:hsm-digest-bound-mvp")
        && farm_src.contains("C_Digest")
        && farm_src.contains("hsm-digest-bound")
        && farm_src.contains("farm_lease_pkcs11_hsm_digest_bound");
    let digest_streaming = farm_src.contains("farm-lease-token-v15:hsm-digest-streaming-mvp")
        && farm_src.contains("C_DigestInit")
        && farm_src.contains("C_DigestUpdate")
        && farm_src.contains("C_DigestFinal")
        && farm_src.contains("hsm-digest-streaming")
        && farm_src.contains("farm_lease_pkcs11_hsm_digest_streaming");
    let multi_slot = farm_src.contains("farm-lease-token-v16:hsm-multi-slot-mvp")
        && farm_src.contains("C_GetSlotList")
        && farm_src.contains("hsm-multi-slot")
        && farm_src.contains("farm_lease_pkcs11_hsm_multi_slot");
    let session_pin = farm_src.contains("farm-lease-token-v17:hsm-session-pin-mvp")
        && farm_src.contains("hsm-session-pin")
        && farm_src.contains("farm_lease_pkcs11_session_pin_persist")
        && farm_src.contains("farm_lease_pkcs11_session_pin_verify");
    let key_label = farm_src.contains("farm-lease-token-v18:hsm-key-label-mvp")
        && farm_src.contains("hsm-key-label")
        && farm_src.contains("farm_lease_pkcs11_key_label_persist")
        && farm_src.contains("farm_lease_pkcs11_key_label_verify");
    let key_id = farm_src.contains("farm-lease-token-v19:hsm-key-id-mvp")
        && farm_src.contains("hsm-key-id")
        && farm_src.contains("farm_lease_pkcs11_key_id_persist")
        && farm_src.contains("farm_lease_pkcs11_key_id_verify")
        && farm_src.contains("CKA_ID");
    HsmRotationReport {
        lock_v15_api: lock_src.contains("default_lock_version() -> u32") && lock_src.contains("15"),
        hsm_rotation_cron_field: lock_src.contains("hsm_rotation_cron_job"),
        billing_settlement_field: lock_src.contains("billing_settlement_id"),
        marketplace_epoch_field: lock_src.contains("marketplace_plugin_epoch"),
        hsm_ref_bound: bound,
        hsm_ref_bound_export: if bound {
            crate::farm_lease_auth::farm_lease_hsm_ref_bound_doctor_export()
        } else {
            ""
        },
        hsm_sign_bound: sign_bound,
        hsm_sign_bound_export: if sign_bound {
            crate::farm_lease_auth::farm_lease_hsm_sign_bound_doctor_export()
        } else {
            ""
        },
        hsm_sign_verify: sign_verify,
        hsm_sign_verify_export: if sign_verify {
            crate::farm_lease_auth::farm_lease_hsm_sign_verify_doctor_export()
        } else {
            ""
        },
        hsm_digest_bound: digest_bound,
        hsm_digest_bound_export: if digest_bound {
            crate::farm_lease_auth::farm_lease_hsm_digest_bound_doctor_export()
        } else {
            ""
        },
        hsm_digest_streaming: digest_streaming,
        hsm_digest_streaming_export: if digest_streaming {
            crate::farm_lease_auth::farm_lease_hsm_digest_streaming_doctor_export()
        } else {
            ""
        },
        hsm_multi_slot: multi_slot,
        hsm_multi_slot_export: if multi_slot {
            crate::farm_lease_auth::farm_lease_hsm_multi_slot_doctor_export()
        } else {
            ""
        },
        hsm_session_pin: session_pin,
        hsm_session_pin_export: if session_pin {
            crate::farm_lease_auth::farm_lease_hsm_session_pin_doctor_export()
        } else {
            ""
        },
        hsm_key_label: key_label,
        hsm_key_label_export: if key_label {
            crate::farm_lease_auth::farm_lease_hsm_key_label_doctor_export()
        } else {
            ""
        },
        hsm_key_id: key_id,
        hsm_key_id_export: if key_id {
            crate::farm_lease_auth::farm_lease_hsm_key_id_doctor_export()
        } else {
            ""
        },
    }
}

pub fn print_hsm_rotation_report(report: &HsmRotationReport) {
    println!("SYX HSM rotation report (Phase 81)");
    println!(
        "  lock v15 API: {}",
        if report.lock_v15_api { "yes" } else { "no" }
    );
    println!(
        "  hsm_rotation_cron_job: {}",
        if report.hsm_rotation_cron_field {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  billing_settlement_id: {}",
        if report.billing_settlement_field {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  marketplace_plugin_epoch: {}",
        if report.marketplace_epoch_field {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  hsm-ref-bound: {}",
        if report.hsm_ref_bound {
            report.hsm_ref_bound_export
        } else {
            "no"
        }
    );
    println!(
        "  hsm-sign-bound: {}",
        if report.hsm_sign_bound {
            report.hsm_sign_bound_export
        } else {
            "no"
        }
    );
    println!(
        "  hsm-sign-verify: {}",
        if report.hsm_sign_verify {
            report.hsm_sign_verify_export
        } else {
            "no"
        }
    );
    println!(
        "  hsm-digest-bound: {}",
        if report.hsm_digest_bound {
            report.hsm_digest_bound_export
        } else {
            "no"
        }
    );
    println!(
        "  hsm-digest-streaming: {}",
        if report.hsm_digest_streaming {
            report.hsm_digest_streaming_export
        } else {
            "no"
        }
    );
    println!(
        "  hsm-multi-slot: {}",
        if report.hsm_multi_slot {
            report.hsm_multi_slot_export
        } else {
            "no"
        }
    );
    println!(
        "  hsm-session-pin: {}",
        if report.hsm_session_pin {
            report.hsm_session_pin_export
        } else {
            "no"
        }
    );
    println!(
        "  hsm-key-label: {}",
        if report.hsm_key_label {
            report.hsm_key_label_export
        } else {
            "no"
        }
    );
    println!(
        "  hsm-key-id: {}",
        if report.hsm_key_id {
            report.hsm_key_id_export
        } else {
            "no"
        }
    );
}
