//! A-06 / Q-S2 exit readiness inventory (Wave A W341–W345 / ADR-685–689).
//!
//! After W345 exit charter, A-06 / Q-S2 complete flags are on while A-07+ remain open.

/// Inventory of A-06 exit readiness (thin + native AOT + raise + malformed floor + exit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateA06ExitReadinessInventory {
    /// W295 five-cluster thin floor is indexed and signed.
    pub relational_thin_floor_ok: bool,
    /// W343: `timeout_tls` + `tls.client` product `native=real` (raise met).
    pub relational_native_raise_ok: bool,
    /// W342 native hang AOT nail indexed.
    pub relational_native_hang_aot_nail_ok: bool,
    /// W342 native HTTPS EXTRA_ROOT AOT nail indexed.
    pub relational_native_https_aot_nail_ok: bool,
    /// W344: full loopback + malformed GA floor signed.
    pub relational_malformed_ga_ok: bool,
    /// Count of relational predicates tracked (5).
    pub candidate_predicate_count: usize,
    /// `true` after W345 — A-06 exit charter signed.
    pub a06_exit_enabled: bool,
    /// `true` after W345 — Q-S2 checklist checked.
    pub qs2_complete_enabled: bool,
    /// `true` after W343 — native raise on.
    pub native_raise_enabled: bool,
}

/// Inventory A-06 exit readiness (W341–W345).
pub fn gate_a06_exit_readiness_inventory() -> GateA06ExitReadinessInventory {
    GateA06ExitReadinessInventory {
        relational_thin_floor_ok: true,
        relational_native_raise_ok: true,
        relational_native_hang_aot_nail_ok: true,
        relational_native_https_aot_nail_ok: true,
        relational_malformed_ga_ok: true,
        candidate_predicate_count: 5,
        a06_exit_enabled: true,
        qs2_complete_enabled: true,
        native_raise_enabled: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caps_native_real_for(id: &str) -> bool {
        let caps = include_str!("../../../packages/internal/capabilities.toml");
        let marker = format!("id = \"{id}\"");
        let Some(start) = caps.find(&marker) else {
            return false;
        };
        let window = &caps[start..start.saturating_add(400).min(caps.len())];
        window.contains("native = \"real\"") && !window.contains("native = \"partial\"")
    }

    #[test]
    fn gate_a06_exit_readiness_inventory_preds_and_honesty() {
        let inv = gate_a06_exit_readiness_inventory();
        assert!(inv.relational_thin_floor_ok);
        assert!(inv.relational_native_raise_ok);
        assert!(inv.relational_native_hang_aot_nail_ok);
        assert!(inv.relational_native_https_aot_nail_ok);
        assert!(inv.relational_malformed_ga_ok);
        assert_eq!(inv.candidate_predicate_count, 5);
        assert!(
            inv.a06_exit_enabled,
            "A-06 exit must be on after W345: {inv:?}"
        );
        assert!(
            inv.qs2_complete_enabled,
            "Q-S2 complete must be on after W345: {inv:?}"
        );
        assert!(inv.native_raise_enabled);
    }

    #[test]
    fn gate_a06_exit_readiness_inventory_matches_thin_floor_and_caps() {
        let inv = gate_a06_exit_readiness_inventory();
        let thin = include_str!("../../../plan/archive/gates/gate-a06-network-negatives.md");
        assert!(thin.contains("A-06 thin floor") && thin.contains("ADR-639"));
        let floor = include_str!("../../../plan/archive/gates/gate-a06-qs2-floor.md");
        assert!(
            floor.contains("ADR-688")
                && floor.contains("m2_s4_a06_malformed_truncated_response_loopback")
        );
        let exit = include_str!("../../../plan/archive/gates/gate-a06-qs2-exit.md");
        assert!(
            exit.contains("ADR-689")
                && (exit.contains("exit signed") || exit.contains("Exit signed")),
            "exit charter policy must cite ADR-689"
        );
        assert!(caps_native_real_for("net.http.timeout_tls"));
        assert!(caps_native_real_for("net.tls.client"));
        assert_eq!(inv.a06_exit_enabled, true);
        assert_eq!(inv.qs2_complete_enabled, true);
        assert_eq!(inv.native_raise_enabled, true);
    }
}
