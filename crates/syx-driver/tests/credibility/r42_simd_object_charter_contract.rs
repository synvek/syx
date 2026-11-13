//! R-42 simd Object honesty charter contracts.
//! Nails: ADR + census; simd residual ≥159 (no wipe); inventory ceiling ≤180/10.

use std::path::PathBuf;
use std::process::Command;

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn repo_file(rel: &str) -> String {
    std::fs::read_to_string(repo_path(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn r42_charter_adr_and_census_locked() {
    let adr = repo_file("docs/adr/r-42-simd-object-charter.md");
    assert!(adr.contains("R-42"), "ADR must name R-42");
    assert!(
        adr.contains("wipe") && (adr.contains("动刀") || adr.contains("允许动刀")),
        "ADR must ban wipe and define knife gates"
    );
    assert!(adr.contains("TypeId::Simd"), "ADR must cite TypeId::Simd");
    assert!(
        adr.contains("Non-claims") || adr.contains("≠"),
        "ADR must Non-claim wipe / −159"
    );
    assert!(
        adr.contains("SIMD2"),
        "ADR must suggest SIMD2-first slice order"
    );

    let census = repo_file("docs/census/r-42-simd-object-census.md");
    assert!(census.contains("159"), "census must cite 159");
    assert!(census.contains("SIMD2") && census.contains("SIMD16"));
    assert!(
        census.contains("wipe") || census.contains("charter"),
        "census must mark charter / no wipe"
    );
    assert!(
        census.contains("TypeId::Simd"),
        "census must document TypeId::Simd gap"
    );
}

#[test]
fn r42_simd_residual_not_wiped() {
    let out = Command::new("python3")
        .arg("-c")
        .arg(
            r#"
import re
from pathlib import Path
obj_re = re.compile(r"(?::\s*Object\b|->\s*Object\b|Array\s*<\s*Object\s*>)")
code = "\n".join(
    ln for ln in Path("packages/core/simd.sy").read_text(encoding="utf-8").splitlines()
    if not ln.strip().startswith("//")
)
n = len(obj_re.findall(code))
print(f"simd_object_residual={n}")
raise SystemExit(0 if n >= 159 else 1)
"#,
        )
        .current_dir(repo_path(""))
        .output()
        .expect("python residual");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "simd residual must stay ≥159 (no wipe)\n{stdout}\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(stdout.contains("simd_object_residual="));
}

#[test]
fn r42_roadmap_exit_points_r43() {
    let plan = repo_file("plan/02-improvement-roadmap.md");
    let rows: Vec<_> = plan
        .lines()
        .filter(|l| l.contains("| **R-42** |"))
        .collect();
    assert!(!rows.is_empty(), "missing R-42 row");
    assert!(
        rows.iter()
            .any(|r| r.contains("`[x]`") || r.contains("| [x] |")),
        "R-42 must be [x]: {rows:?}"
    );
    let tail = plan.split("默认下一窗").nth(1).expect("默认下一窗 missing");
    assert!(
        tail.chars().take(120).collect::<String>().contains("R-43"),
        "default next must be R-43"
    );
}
