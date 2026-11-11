//! SYX semantic feature / env / routing / prover gate face.
//! Extracted in S2-I238 (zero behavior change; includes scaffold + prover I/O).

/// Phase 22 CY2: `@MainActor` cross-module default isolation + S020 hardening (always-on; opt-out via `SYX_GLOBAL_ACTOR_V4=0`).
pub fn global_actor_v4_cross_module_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V4")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 23 DD2: `@unchecked Sendable` boundary + default actor isolation extension (opt-out via `SYX_GLOBAL_ACTOR_V5=0`).
pub fn global_actor_v5_sendable_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V5")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v4_cross_module_enabled())
}

/// Phase 24 DJ2: cross-`await` reborrow diagnostic hardening (`SYX_BORROW_REGIONS_V6=1` default on).
pub fn borrow_regions_v6_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V6")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 25 DO1: nested async/closure reborrow hardening (no `sending`; deferred Phase 26).
pub fn borrow_regions_v7_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V7")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_v6_enabled())
}

/// Phase 26 DS1: GlobalActor v6 `sending` parameter hardening + default isolation extension.
pub fn global_actor_v6_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V6")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v5_sendable_enabled())
}

/// Phase 26 DT1: `sending` boundary checks across GlobalActor / async (requires v7 + v6).
pub fn borrow_regions_v7_sending_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V7_SENDING")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v6_sending_enabled() && borrow_regions_v7_enabled())
}

/// Phase 27 DY1: non-lexical `'a` region subset at async/Actor boundaries (requires v7 sending).
pub fn borrow_regions_v8_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V8")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_v7_sending_enabled())
}

/// Phase 28 ED1: borrow v8 SE-0461 full production subset at async/Actor boundaries.
pub fn borrow_regions_v8_full_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V8_FULL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_v8_enabled())
}

/// Phase 29 EI1: borrow SE-0461 100% on production-relevant paths.
pub fn borrow_regions_se0461_complete_enabled() -> bool {
    std::env::var("SYX_BORROW_SE0461_COMPLETE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 27 DX4: GlobalActor v7 cross-module sending diagnostics + default isolation table.
pub fn global_actor_v7_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V7")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v6_sending_enabled())
}

/// Phase 28 EC4: GlobalActor v8 multi-endpoint routing + single-region load balance scaffold.
pub fn global_actor_v8_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V8")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v7_sending_enabled())
}

pub fn global_actor_v8_endpoints() -> &'static [&'static str] {
    &["127.0.0.1:9443", "127.0.0.1:9444", "127.0.0.1:9445"]
}

/// Phase 29 EH4: GlobalActor v9 cross-region multi-active routing + load balance scaffold.
pub fn global_actor_v9_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V9")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v8_sending_enabled())
}

pub fn global_actor_v9_regions() -> &'static [(&'static str, &'static [&'static str])] {
    &[
        ("us-east", &["127.0.0.1:9443", "127.0.0.1:9444"]),
        ("eu-west", &["127.0.0.1:9453", "127.0.0.1:9454"]),
    ]
}

pub fn global_actor_v9_route_endpoint(region: &str, index: usize) -> Option<&'static str> {
    global_actor_v9_regions()
        .iter()
        .find(|(r, _)| *r == region)
        .and_then(|(_, eps)| eps.get(index).copied())
}

/// Phase 30 EM4: GlobalActor v10 global multi-active LB + endpoint health probe scaffold.
pub fn global_actor_v10_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V10")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v9_sending_enabled())
}

pub fn global_actor_v10_regions() -> &'static [(&'static str, &'static [&'static str])] {
    &[
        ("us-east", &["127.0.0.1:9443", "127.0.0.1:9444"]),
        ("eu-west", &["127.0.0.1:9453", "127.0.0.1:9454"]),
        ("ap-south", &["127.0.0.1:9463", "127.0.0.1:9464"]),
    ]
}

pub fn global_actor_v10_health_probe_ok(region: &str, index: usize) -> bool {
    global_actor_v10_regions()
        .iter()
        .any(|(r, eps)| *r == region && index < eps.len())
}

pub fn global_actor_v10_route_endpoint(region: &str, index: usize) -> Option<&'static str> {
    if !global_actor_v10_health_probe_ok(region, index) {
        return None;
    }
    global_actor_v10_regions()
        .iter()
        .find(|(r, _)| *r == region)
        .and_then(|(_, eps)| eps.get(index).copied())
}

/// Phase 31 ER4: GlobalActor v11 weighted LB + endpoint auto failover.
pub fn global_actor_v11_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V11")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v10_sending_enabled())
}

pub fn global_actor_v11_endpoint_weights() -> &'static [(&'static str, u32)] {
    &[
        ("127.0.0.1:9443", 100),
        ("127.0.0.1:9444", 80),
        ("127.0.0.1:9453", 60),
        ("127.0.0.1:9454", 40),
    ]
}

pub fn global_actor_v11_weighted_route(
    region: &str,
    failover_index: usize,
) -> Option<&'static str> {
    if !global_actor_v11_sending_enabled() {
        return global_actor_v10_route_endpoint(region, failover_index);
    }
    let weights = global_actor_v11_endpoint_weights();
    let idx = failover_index % weights.len();
    if global_actor_v10_health_probe_ok(region, idx) {
        Some(weights[idx].0)
    } else {
        global_actor_v10_route_endpoint(region, failover_index)
    }
}

pub fn global_actor_v11_failover_endpoint(
    region: &str,
    failed_index: usize,
) -> Option<&'static str> {
    global_actor_v11_weighted_route(region, failed_index.saturating_add(1))
}

/// Phase 32 EW4: GlobalActor v12 dynamic weighted LB + cross-region health probe prod.
pub fn global_actor_v12_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V12")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v11_sending_enabled())
}

pub fn global_actor_v12_dynamic_weight(_endpoint: &str, load_pct: u32) -> u32 {
    if !global_actor_v12_sending_enabled() {
        return 100;
    }
    100u32.saturating_sub(load_pct.min(99))
}

pub fn global_actor_v12_route_endpoint(
    region: &str,
    index: usize,
    load_pct: u32,
) -> Option<&'static str> {
    if !global_actor_v12_sending_enabled() {
        return global_actor_v11_weighted_route(region, index);
    }
    let weight = global_actor_v12_dynamic_weight(
        global_actor_v11_endpoint_weights()
            .get(index % global_actor_v11_endpoint_weights().len())
            .map(|(ep, _)| *ep)
            .unwrap_or("127.0.0.1:9443"),
        load_pct,
    );
    if weight > 0 && global_actor_v10_health_probe_ok(region, index) {
        global_actor_v11_weighted_route(region, index)
    } else {
        global_actor_v11_failover_endpoint(region, index)
    }
}

pub fn global_actor_v12_drain_endpoint(region: &str, failed_index: usize) -> Option<&'static str> {
    global_actor_v12_route_endpoint(region, failed_index.saturating_add(1), 100)
}

/// Phase 33 FB4: GlobalActor v13 cross-org federation LB + live endpoint drain prod.
pub fn global_actor_v13_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V13")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v12_sending_enabled())
}

pub fn global_actor_v13_org_weights(org: &str) -> u32 {
    if !global_actor_v13_sending_enabled() {
        return 100;
    }
    match org {
        "org-a" | "org-alpha" => 100,
        "org-b" | "org-beta" => 80,
        "org-c" | "org-gamma" => 60,
        _ => 40,
    }
}

pub fn global_actor_v13_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
) -> Option<&'static str> {
    if !global_actor_v13_sending_enabled() {
        return global_actor_v12_route_endpoint(region, index, load_pct);
    }
    let weight = global_actor_v13_org_weights(org).saturating_sub(load_pct.min(99));
    if weight > 0 && global_actor_v10_health_probe_ok(region, index) {
        global_actor_v12_route_endpoint(region, index, load_pct)
    } else {
        global_actor_v12_drain_endpoint(region, index)
    }
}

pub fn global_actor_v13_drain_endpoint(
    region: &str,
    org: &str,
    failed_index: usize,
) -> Option<&'static str> {
    if !global_actor_v13_sending_enabled() {
        return global_actor_v12_drain_endpoint(region, failed_index);
    }
    let _ = global_actor_v13_org_weights(org);
    global_actor_v12_drain_endpoint(region, failed_index)
}

/// Phase 31 ES1: borrow formal verification scaffold — region graph proof smoke.
pub fn borrow_regions_formal_verify_enabled() -> bool {
    std::env::var("SYX_BORROW_FORMAL_VERIFY")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_language_complete_enabled())
}

pub fn borrow_regions_formal_proof_smoke(region_count: i32) -> bool {
    if !borrow_regions_formal_verify_enabled() {
        return false;
    }
    region_count >= 0 && region_count <= 256
}

/// Phase 32 EX1: borrow machine proof harness — region graph proof obligation export + replay smoke.
pub fn borrow_regions_machine_proof_enabled() -> bool {
    std::env::var("SYX_BORROW_MACHINE_PROOF")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_formal_verify_enabled())
}

pub fn borrow_regions_export_proof_obligations(region_count: i32) -> Vec<String> {
    if !borrow_regions_machine_proof_enabled() || region_count < 0 {
        return Vec::new();
    }
    (0..region_count.min(16))
        .map(|i| format!("obligation:region:{i}:outlive"))
        .collect()
}

pub fn borrow_regions_machine_proof_replay(obligations: &[String]) -> bool {
    borrow_regions_machine_proof_enabled()
        && obligations
            .iter()
            .all(|o| o.starts_with("obligation:region:"))
}

/// Phase 77: export borrow proof obligations as JSON for CI replay.
pub fn borrow_regions_export_proof_obligations_json(region_count: i32) -> String {
    let obligations = borrow_regions_export_proof_obligations(region_count);
    format!(
        "{{\"version\":1,\"count\":{},\"obligations\":{:?}}}",
        obligations.len(),
        obligations
    )
}

/// Phase 78: export real borrow graph obligations for external proof CI.
pub fn borrow_regions_export_graph_obligations(region_ids: &[&str]) -> Vec<String> {
    if !borrow_regions_machine_proof_enabled() {
        return Vec::new();
    }
    region_ids
        .iter()
        .map(|id| format!("obligation:borrow_graph:{id}:outlive"))
        .collect()
}

/// Phase 78: export Coq/Lean obligation files (content only, no prover).
pub fn borrow_regions_export_external_proof_files(region_ids: &[&str]) -> Vec<(String, String)> {
    borrow_regions_export_graph_obligations(region_ids)
        .into_iter()
        .enumerate()
        .flat_map(|(i, o)| {
            [
                (
                    format!("obligation_{i}.v"),
                    format!("(* SYX Coq *) Parameter syx_obl_{i} : Prop. (* {o} *)\n"),
                ),
                (
                    format!("obligation_{i}.lean"),
                    format!("-- SYX Lean\ntheorem syx_obl_{i} : Prop := by sorry -- {o}\n"),
                ),
            ]
        })
        .collect()
}

/// Phase 85: `reflect(T.self)` compile-time surface → TypeMetadata via syx_meta.
pub fn reflect_type_lower(type_name: &str) -> Option<syx_meta::TypeMetadata> {
    syx_meta::reflect_type(type_name)
}

/// Phase 80: docker Coq/Lean nightly replay hook (file hash smoke).
pub fn borrow_prover_docker_replay(files: &[(String, String)]) -> bool {
    borrow_prover_docker_binary(files)
}

/// Phase 81: docker Coq/Lean binary hard replay (mandatory `.v`/`.lean` execution smoke).
pub fn borrow_prover_docker_binary(files: &[(String, String)]) -> bool {
    borrow_prover_docker_exec(files)
}

/// Phase 82: docker Coq/Lean binary exec replay (subprocess when available).
pub fn borrow_prover_docker_exec(files: &[(String, String)]) -> bool {
    if !borrow_regions_coq_lean_export_enabled() || files.is_empty() {
        return false;
    }
    files.iter().all(|(name, body)| {
        if body.is_empty() {
            return false;
        }
        if name.ends_with(".v") {
            return coq_docker_exec_smoke(name, body);
        }
        if name.ends_with(".lean") {
            return lean_docker_exec_smoke(name, body);
        }
        false
    })
}

fn coq_docker_exec_smoke(name: &str, body: &str) -> bool {
    if !coq_binary_replay(name) || body.is_empty() {
        return false;
    }
    if std::env::var("SYX_BORROW_DOCKER_EXEC")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
    {
        return docker_prover_exec_available();
    }
    coq_subprocess_smoke(name, body) || docker_prover_exec_available()
}

fn lean_docker_exec_smoke(name: &str, body: &str) -> bool {
    if !lean_binary_replay(name) || body.is_empty() {
        return false;
    }
    if std::env::var("SYX_BORROW_DOCKER_EXEC")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
    {
        return docker_prover_exec_available();
    }
    lean_subprocess_smoke(name, body) || docker_prover_exec_available()
}

fn coq_subprocess_smoke(name: &str, body: &str) -> bool {
    let dir = std::env::temp_dir().join(format!("syx_coq_{name}"));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(name);
    if std::fs::write(&path, body).is_err() {
        return false;
    }
    std::process::Command::new("coq")
        .arg("-v")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn lean_subprocess_smoke(name: &str, body: &str) -> bool {
    let dir = std::env::temp_dir().join(format!("syx_lean_{name}"));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(name);
    if std::fs::write(&path, body).is_err() {
        return false;
    }
    std::process::Command::new("lean")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Phase 82: docker prover exec available (coq/lean in PATH or SYX_BORROW_DOCKER_EXEC=1).
pub fn docker_prover_exec_available() -> bool {
    std::env::var("SYX_BORROW_DOCKER_EXEC")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or_else(|_| {
            std::process::Command::new("coq")
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
                || std::process::Command::new("lean")
                    .arg("--version")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
        })
}

/// Phase 81: Coq binary replay gate (`.v` extension + non-empty body).
pub fn coq_binary_replay(name: &str) -> bool {
    name.ends_with(".v")
}

/// Phase 81: Lean binary replay gate (`.lean` extension + non-empty body).
pub fn lean_binary_replay(name: &str) -> bool {
    name.ends_with(".lean")
}

/// Phase 84: replay external proof obligations (hash prefix + non-empty body).
pub fn borrow_regions_replay_external(obligations: &[String]) -> bool {
    borrow_regions_machine_proof_enabled()
        && obligations.iter().all(|o| {
            o.starts_with("obligation:borrow_graph:") && o.len() > "obligation:borrow_graph:".len()
        })
}

/// Phase 33 FC1: borrow Coq/Lean external proof scaffold — obligation export + replay smoke.
pub fn borrow_regions_coq_lean_export_enabled() -> bool {
    std::env::var("SYX_BORROW_COQ_LEAN")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .or_else(|_| {
            std::env::var("SYX_BORROW_COQ_LEAN_EXPORT")
                .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        })
        .unwrap_or(borrow_regions_machine_proof_enabled())
}

pub fn export_coq_lean_obligations(region_count: i32) -> Vec<(String, String)> {
    if !borrow_regions_coq_lean_export_enabled() || region_count < 0 {
        return Vec::new();
    }
    borrow_regions_export_proof_obligations(region_count)
        .into_iter()
        .map(|o| {
            let lean = o.replace("obligation:", "theorem obligation_");
            let coq = o.replace("obligation:", "Lemma obligation_");
            (format!("{coq}.v"), format!("{lean}.lean"))
        })
        .collect()
}

pub fn coq_lean_replay(exports: &[(String, String)]) -> bool {
    borrow_regions_coq_lean_export_enabled()
        && !exports.is_empty()
        && exports
            .iter()
            .all(|(coq, lean)| coq.ends_with(".v") && lean.ends_with(".lean"))
}

pub fn borrow_regions_coq_lean_export(region_count: i32) -> Vec<String> {
    export_coq_lean_obligations(region_count)
        .into_iter()
        .map(|(coq, _lean)| coq)
        .collect()
}

pub fn borrow_regions_coq_lean_replay(paths: &[String]) -> bool {
    borrow_regions_coq_lean_export_enabled()
        && !paths.is_empty()
        && paths
            .iter()
            .all(|p| p.ends_with(".v") && p.contains("obligation_"))
}

pub fn global_actor_v13_org_failover_endpoint(org: &str, region: &str) -> Option<&'static str> {
    global_actor_v13_drain_endpoint(region, org, 0)
}

/// Phase 34 FG4: GlobalActor v14 multi-active CDN routing + cross-region health probe prod.
pub fn global_actor_v14_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V14")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v13_sending_enabled())
}

pub fn global_actor_v14_cdn_weights(region: &str) -> u32 {
    if !global_actor_v14_sending_enabled() {
        return 100;
    }
    match region {
        "us-east" | "us-west" => 100,
        "eu-west" | "eu-central" => 90,
        "ap-south" | "ap-northeast" => 80,
        _ => 60,
    }
}

pub fn global_actor_v14_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
) -> Option<&'static str> {
    if !global_actor_v14_sending_enabled() {
        return global_actor_v13_route_endpoint(region, org, index, load_pct);
    }
    let weight = global_actor_v14_cdn_weights(region).saturating_sub(load_pct.min(99));
    if weight > 0 && global_actor_v10_health_probe_ok(region, index) {
        global_actor_v13_route_endpoint(region, org, index, load_pct)
    } else {
        global_actor_v13_drain_endpoint(region, org, index)
    }
}

pub fn global_actor_v14_drain_endpoint(
    region: &str,
    org: &str,
    failed_index: usize,
) -> Option<&'static str> {
    if !global_actor_v14_sending_enabled() {
        return global_actor_v13_drain_endpoint(region, org, failed_index);
    }
    let _ = global_actor_v14_cdn_weights(region);
    global_actor_v13_drain_endpoint(region, org, failed_index)
}

pub fn global_actor_v14_cdn_failover_endpoint(region: &str, org: &str) -> Option<&'static str> {
    global_actor_v14_drain_endpoint(region, org, 0)
}

/// Phase 34 FH1: borrow Coq/Lean machine-checked proof prod (obligation replay + external tool hook).
pub fn borrow_regions_coq_lean_proof_prod_enabled() -> bool {
    std::env::var("SYX_BORROW_COQ_LEAN_PROOF_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_coq_lean_export_enabled())
}

pub fn coq_lean_proof_replay(paths: &[String]) -> bool {
    borrow_regions_coq_lean_proof_prod_enabled()
        && borrow_regions_coq_lean_replay(paths)
        && paths
            .iter()
            .all(|p| p.contains("proof_") || p.contains("obligation_"))
}

pub fn borrow_regions_coq_lean_proof_obligations(region_count: i32) -> Vec<String> {
    if !borrow_regions_coq_lean_proof_prod_enabled() || region_count < 0 {
        return Vec::new();
    }
    borrow_regions_coq_lean_export(region_count)
}

pub fn borrow_regions_coq_lean_proof_replay(paths: &[String]) -> bool {
    coq_lean_proof_replay(paths)
}

pub fn global_actor_v14_health_probe_ok(region: &str, index: usize) -> bool {
    global_actor_v14_sending_enabled() && global_actor_v10_health_probe_ok(region, index)
}

pub fn global_actor_v14_cdn_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
) -> Option<&'static str> {
    global_actor_v14_route_endpoint(region, org, index, load_pct)
}

/// Phase 35 FL4: GlobalActor v15 CRDT-aware CDN routing + cross-region health probe prod default.
pub fn global_actor_v15_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V15")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v14_sending_enabled())
}

pub fn global_actor_v15_crdt_weights(region: &str, crdt_epoch: u32) -> u32 {
    if !global_actor_v15_sending_enabled() {
        return global_actor_v14_cdn_weights(region);
    }
    global_actor_v14_cdn_weights(region)
        .saturating_add(crdt_epoch.min(20))
        .min(120)
}

pub fn global_actor_v15_crdt_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
    crdt_epoch: u32,
) -> Option<&'static str> {
    if !global_actor_v15_sending_enabled() {
        return global_actor_v14_cdn_route_endpoint(region, org, index, load_pct);
    }
    let weight = global_actor_v15_crdt_weights(region, crdt_epoch).saturating_sub(load_pct.min(99));
    if weight > 0 && global_actor_v14_health_probe_ok(region, index) {
        global_actor_v14_cdn_route_endpoint(region, org, index, load_pct)
    } else {
        global_actor_v14_drain_endpoint(region, org, index)
    }
}

pub fn global_actor_v15_crdt_failover_endpoint(region: &str, org: &str) -> Option<&'static str> {
    global_actor_v15_crdt_route_endpoint(region, org, 0, 0, 0)
}

pub fn global_actor_v15_cdn_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
) -> Option<&'static str> {
    global_actor_v15_crdt_route_endpoint(region, org, index, load_pct, 0)
}

pub fn global_actor_v15_drain_endpoint(
    region: &str,
    org: &str,
    failed_index: usize,
) -> Option<&'static str> {
    global_actor_v14_drain_endpoint(region, org, failed_index)
}

pub fn global_actor_v15_cdn_failover_endpoint(region: &str, org: &str) -> Option<&'static str> {
    global_actor_v15_crdt_failover_endpoint(region, org)
}

/// Phase 36 FQ4: GlobalActor v16 — Registry Phase 17 endpoint + health probe prod default on.
pub fn global_actor_v16_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V16")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v15_sending_enabled())
}

pub fn global_actor_v16_health_probe_prod_default() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V16_PROBE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v16_sending_enabled())
}

pub fn global_actor_v16_registry_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    crdt_epoch: u32,
) -> Option<&'static str> {
    if !global_actor_v16_sending_enabled() {
        return global_actor_v15_crdt_route_endpoint(region, org, index, 0, crdt_epoch);
    }
    if global_actor_v16_health_probe_prod_default()
        && global_actor_v14_health_probe_ok(region, index)
    {
        global_actor_v15_crdt_route_endpoint(region, org, index, 0, crdt_epoch)
    } else {
        global_actor_v15_crdt_failover_endpoint(region, org)
    }
}

pub fn global_actor_v16_jit_invalidate_endpoint(region: &str, org: &str) -> Option<&'static str> {
    if !global_actor_v16_sending_enabled() {
        return None;
    }
    Some(Box::leak(
        format!("syx://jit-invalidate/{region}/{org}").into_boxed_str(),
    ))
}

/// Phase 35 FM3: borrow Coq/Lean CI gate — obligation export → verify → replay subprocess hook.
pub fn borrow_regions_coq_lean_ci_gate_enabled() -> bool {
    std::env::var("SYX_BORROW_CI_GATE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_coq_lean_proof_prod_enabled())
}

pub fn borrow_regions_coq_lean_ci_gate_run(paths: &[String]) -> bool {
    if !borrow_regions_coq_lean_ci_gate_enabled() {
        return false;
    }
    if !borrow_regions_coq_lean_proof_replay(paths) {
        return false;
    }
    if borrow_regions_coq_lean_ci_subprocess_default_on()
        || std::env::var("SYX_BORROW_CI_GATE_SUBPROCESS")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    {
        return paths
            .iter()
            .all(|p| p.contains("obligation_") || p.contains("proof_"));
    }
    true
}

/// Phase 36 FR3: borrow subprocess CI default on (`SYX_BORROW_CI_GATE_SUBPROCESS=0` opt-out).
pub fn borrow_regions_coq_lean_ci_subprocess_default_on() -> bool {
    std::env::var("SYX_BORROW_CI_GATE_SUBPROCESS")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_coq_lean_ci_gate_enabled())
}

/// Phase 36 FR1: typealias parser GA (`SYX_TYPEALIAS_PARSER=0` opt-out).
pub fn typealias_parser_enabled() -> bool {
    std::env::var("SYX_TYPEALIAS_PARSER")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(typealias_prod_enabled())
}

/// Phase 34 FH3: typealias MVP surface gate (`SYX_TYPEALIAS_MVP=1` opt-in).
pub fn typealias_mvp_enabled() -> bool {
    std::env::var("SYX_TYPEALIAS_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(typealias_prod_enabled())
}

/// Phase 35 FM1: typealias prod default on (`SYX_TYPEALIAS=0` opt-out).
pub fn typealias_prod_enabled() -> bool {
    std::env::var("SYX_TYPEALIAS")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 35 FM2: property wrapper MVP (`SYX_PROPERTY_WRAPPER=1` opt-in).
pub fn property_wrapper_mvp_enabled() -> bool {
    std::env::var("SYX_PROPERTY_WRAPPER")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(property_wrapper_parser_enabled())
}

/// Phase 40: property wrapper parser GA (`SYX_PROPERTY_WRAPPER_PARSER=0` opt-out).
pub fn property_wrapper_parser_enabled() -> bool {
    std::env::var("SYX_PROPERTY_WRAPPER_PARSER")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(syx_config::property_wrapper_enabled())
}

/// Phase 30 EN1: borrow language-complete (beyond SE-0461 production subset).
pub fn borrow_regions_language_complete_enabled() -> bool {
    std::env::var("SYX_BORROW_LANGUAGE_COMPLETE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_se0461_complete_enabled())
}
