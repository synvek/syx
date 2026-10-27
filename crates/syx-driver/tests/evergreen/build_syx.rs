//! build.syb MVP tests (SYX 40.0).
// was: build_syx

use std::fs;
use std::path::PathBuf;
use syx_driver::build_syx::{apply_build_plan, parse_build_syx, BuildPlan};

fn temp_build_syx(content: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_build_syx_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("build.syb");
    fs::write(&path, content).unwrap();
    path
}

#[test]
fn parse_native_libs() {
    let path = temp_build_syx(
        r#"
build {
  native {
    libs = ["z", "m"]
    link_flags = ["-pthread"]
  }
}
"#,
    );
    let plan = parse_build_syx(&path).expect("parse");
    assert_eq!(plan.native.libs, vec!["z", "m"]);
    assert_eq!(plan.native.link_flags, vec!["-pthread"]);
}

#[test]
fn parse_features_and_plugins() {
    let path = temp_build_syx(
        r#"
build {
  plugins {
    "mycrypto"
  }
  features {
    http = true
    debug = false
  }
}
"#,
    );
    let plan = parse_build_syx(&path).expect("parse");
    assert!(plan.plugins.iter().any(|p| p == "mycrypto"));
    assert_eq!(plan.features.get("http"), Some(&true));
    assert_eq!(plan.features.get("debug"), Some(&false));
}

#[test]
fn parse_compile_options_and_task() {
    let path = temp_build_syx(
        r#"
build {
  compileOptions {
    optLevel = release
  }
  task gen {
    target = "wasm"
    dependsOn = ["codegen"]
  }
}
"#,
    );
    let plan = parse_build_syx(&path).expect("parse");
    assert_eq!(plan.compile_options.opt_level, syx_driver::build_syx::OptLevel::Release);
    assert_eq!(plan.tasks.len(), 1);
    assert_eq!(plan.tasks[0].name, "gen");
    assert_eq!(plan.tasks[0].target.as_deref(), Some("wasm"));
    assert_eq!(plan.tasks[0].depends_on, vec!["codegen"]);
}

#[test]
fn apply_release_enables_wasm_aot() {
    let mut opts = syx_driver::DriverOptions::default();
    let plan = BuildPlan {
        compile_options: syx_driver::build_syx::CompileOptions {
            opt_level: syx_driver::build_syx::OptLevel::Release,
        },
        ..Default::default()
    };
    apply_build_plan(&plan, &mut opts, &mut syx_pkg::SyxManifest::default());
    assert!(opts.wasm_aot);
    assert_eq!(opts.build_profile, syx_driver::BuildProfile::Release);
}

#[test]
fn apply_overrides_manifest_native() {
    let mut manifest = syx_pkg::SyxManifest::default();
    manifest.native.libs = vec!["old".into()];
    let plan = BuildPlan {
        native: syx_pkg::NativeConfig {
            libs: vec!["z".into()],
            ..Default::default()
        },
        ..Default::default()
    };
    apply_build_plan(&plan, &mut syx_driver::DriverOptions::default(), &mut manifest);
    assert_eq!(manifest.native.libs, vec!["z"]);
}
