//! WASM AOT matrix reporting for `wasm_aot_matrix_gate` and `syx doctor --aot-matrix`.
//! Probe pivot (Wave A W164 / ADR-508): examples/main.sy + guidemo.sy absent; hello.sy/hot_loop.sy only.

use std::path::{Path, PathBuf};

use syx_config::SyxRuntimeConfig;
use syx_wasm_emit::{
    measure_aot_mir_exec_histogram, measure_aot_ratio, measure_aot_tier_stats, AotTierStats,
};

use crate::{emit_wasm_bytes, DriverOptions};

#[derive(Debug, Clone)]
pub struct AotFixtureReport {
    pub name: String,
    pub path: PathBuf,
    pub stats: AotTierStats,
    pub ratio: f64,
}

#[derive(Debug, Clone)]
pub struct AotMatrixReport {
    pub fixtures: Vec<AotFixtureReport>,
    pub strict_min: f64,
    pub main_ratio: Option<f64>,
}

impl AotMatrixReport {
    pub fn strict_min_ratio(&self) -> f64 {
        self.strict_min
    }

    pub fn full_main_ratio(&self) -> Option<f64> {
        self.main_ratio
    }
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub fn default_fixture_paths() -> Vec<(&'static str, PathBuf)> {
    let root = repo_root();
    vec![
        ("hot_loop.sy", root.join("examples/hot_loop.sy")),
        ("hello.sy", root.join("examples/hello.sy")),
        (
            "comparable_sort.sy",
            root.join("crates/syx-driver/tests/fixtures/parity/comparable_sort.sy"),
        ),
        (
            "inheritance_dispatch.sy",
            root.join("crates/syx-driver/tests/fixtures/parity/inheritance_dispatch.sy"),
        ),
        (
            "protocol_iterator.sy",
            root.join("crates/syx-driver/tests/fixtures/parity/protocol_iterator.sy"),
        ),
    ]
}

pub fn ratio_for_path(path: &Path) -> (AotTierStats, f64) {
    syx_config::install_runtime_config(SyxRuntimeConfig {
        wasm: syx_config::WasmConfig {
            aot: true,
            mir_exec_only: false,
        },
        ..SyxRuntimeConfig::default()
    });
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let bytes = emit_wasm_bytes(path, &opts).expect("emit wasm");
    let stats = measure_aot_tier_stats(&bytes);
    let (_, _, ratio) = measure_aot_ratio(&bytes);
    (stats, ratio)
}

pub fn run_aot_matrix(breakdown: bool) -> AotMatrixReport {
    let mut fixtures = Vec::new();
    let mut strict_min = 1.0_f64;
    let mut main_ratio = None;

    for (name, path) in default_fixture_paths() {
        if !path.exists() {
            if breakdown {
                eprintln!("{name}: skip (missing {})", path.display());
            }
            continue;
        }
        let (stats, ratio) = ratio_for_path(&path);
        if breakdown {
            eprintln!(
                "{name}: ratio={ratio:.6} tier1={} tier2a={} tier2b={} tier2c={} mir_exec={}",
                stats.tier1, stats.tier2a, stats.tier2b, stats.tier2c, stats.mir_exec
            );
        } else {
            eprintln!("{name}: {ratio:.6}");
        }
        if name == "hello.sy" {
            main_ratio = Some(ratio);
        } else {
            strict_min = strict_min.min(ratio);
        }
        fixtures.push(AotFixtureReport {
            name: name.to_string(),
            path,
            stats,
            ratio,
        });
    }

    AotMatrixReport {
        fixtures,
        strict_min,
        main_ratio,
    }
}

pub fn print_doctor_summary(report: &AotMatrixReport) {
    println!("SYX WASM AOT matrix (Phase 70)");
    println!("strict fixtures min: {:.6}", report.strict_min);
    if let Some(main) = report.main_ratio {
        println!("full hello.sy:      {:.6}", main);
    }
    println!();
    println!(
        "{:<28} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
        "fixture", "ratio", "tier1", "tier2a", "tier2b", "tier2c", "mir_exec"
    );
    for fx in &report.fixtures {
        println!(
            "{:<28} {:>8.4} {:>8} {:>8} {:>8} {:>8} {:>8}",
            fx.name,
            fx.ratio,
            fx.stats.tier1,
            fx.stats.tier2a,
            fx.stats.tier2b,
            fx.stats.tier2c,
            fx.stats.mir_exec
        );
        if fx.stats.mir_exec > 0 {
            let mut opts = DriverOptions::default();
            opts.wasm_aot = true;
            if let Ok(bytes) = crate::emit_wasm_bytes(&fx.path, &opts) {
                let hist = measure_aot_mir_exec_histogram(&bytes);
                let mut tags: Vec<_> = hist.into_iter().collect();
                tags.sort_by(|a, b| b.1.cmp(&a.1));
                for (tag, count) in tags.into_iter().take(5) {
                    println!("  mir_exec op {tag}: {count}");
                }
            }
        }
    }
}
