//! Phase 98: re-export Phase 97 hosts (L14 surface GA, GUI dlopen, GC churn).

pub use crate::stdlib_phase97_hosts::{
    host_codable_decode_l14, host_codable_encode_l14, host_codable_l14_register_migration,
    host_codable_l14_register_rename, reset_phase97_state,
};

pub fn reset_phase98_state() {
    crate::stdlib_phase97_hosts::reset_phase97_state();
}
