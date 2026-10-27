//! build.syb Phase 64 executor tests.
// was: build_syx_executor

use std::fs;
use std::path::PathBuf;
use syx_driver::build_syx::{
    execute_build_plan, order_build_tasks, parse_build_syx, prepare_build_plan, run_build_tasks,
    BuildPlan, BuildTask, BuildTaskContext, OptLevel,
};
use syx_driver::DriverOptions;

fn temp_build_syx(content: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_build_exec_{}",
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
fn order_tasks_linear() {
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "a".into(),
                depends_on: vec![],
                target: None,
            },
            BuildTask {
                name: "b".into(),
                depends_on: vec!["a".into()],
                target: None,
            },
        ],
        ..Default::default()
    };
    let order = order_build_tasks(&plan).unwrap();
    assert_eq!(order, vec!["a", "b"]);
}

#[test]
fn order_tasks_cycle_fails_s402() {
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "a".into(),
                depends_on: vec!["b".into()],
                target: None,
            },
            BuildTask {
                name: "b".into(),
                depends_on: vec!["a".into()],
                target: None,
            },
        ],
        ..Default::default()
    };
    let err = order_build_tasks(&plan).unwrap_err();
    assert_eq!(err.code.as_deref(), Some("S402"));
}

#[test]
fn order_unknown_dep_s401() {
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "x".into(),
            depends_on: vec!["missing".into()],
            target: None,
        }],
        ..Default::default()
    };
    let err = order_build_tasks(&plan).unwrap_err();
    assert_eq!(err.code.as_deref(), Some("S401"));
}

#[test]
fn execute_wires_wasm_aot_feature() {
    let mut opts = DriverOptions::default();
    let mut manifest = syx_pkg::SyxManifest::default();
    let plan = BuildPlan {
        features: indexmap::indexmap! { "wasm_aot".into() => true },
        ..Default::default()
    };
    let order = prepare_build_plan(&plan, &mut opts, &mut manifest, PathBuf::from(".").as_path())
        .unwrap();
    assert!(opts.wasm_aot);
    assert!(order.is_empty());
}

#[test]
fn execute_applies_release_profile() {
    let mut opts = DriverOptions::default();
    let mut manifest = syx_pkg::SyxManifest::default();
    let plan = BuildPlan {
        compile_options: syx_driver::build_syx::CompileOptions {
            opt_level: OptLevel::Release,
        },
        ..Default::default()
    };
    prepare_build_plan(&plan, &mut opts, &mut manifest, PathBuf::from(".").as_path()).unwrap();
    assert!(opts.wasm_aot);
}

#[test]
fn parse_multi_task_dag() {
    let path = temp_build_syx(
        r#"
build {
  task codegen {
    target = "mir"
  }
  task link {
    dependsOn = ["codegen"]
    target = "native"
  }
  task test {
    dependsOn = ["link"]
  }
}
"#,
    );
    let plan = parse_build_syx(&path).unwrap();
    let order = order_build_tasks(&plan).unwrap();
    assert_eq!(order, vec!["codegen", "link", "test"]);
}

#[test]
fn parse_plugins_section() {
    let path = temp_build_syx(
        r#"
build {
  plugins {
    echo
    crypto
  }
}
"#,
    );
    let plan = parse_build_syx(&path).unwrap();
    assert_eq!(plan.plugins.len(), 2);
}

#[test]
fn parse_native_include_dirs() {
    let path = temp_build_syx(
        r#"
build {
  native {
    include_dirs = ["/usr/include"]
  }
}
"#,
    );
    let plan = parse_build_syx(&path).unwrap();
    assert_eq!(plan.native.include_dirs, vec!["/usr/include"]);
}

#[test]
fn parse_features_http_flag() {
    let path = temp_build_syx(
        r#"
build {
  features {
    http = true
  }
}
"#,
    );
    let plan = parse_build_syx(&path).unwrap();
    assert_eq!(plan.features.get("http"), Some(&true));
}

#[test]
fn parse_empty_build_block() {
    let path = temp_build_syx("build { }");
    let plan = parse_build_syx(&path).unwrap();
    assert!(plan.tasks.is_empty());
}

#[test]
fn order_diamond_deps() {
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "base".into(),
                depends_on: vec![],
                target: None,
            },
            BuildTask {
                name: "left".into(),
                depends_on: vec!["base".into()],
                target: None,
            },
            BuildTask {
                name: "right".into(),
                depends_on: vec!["base".into()],
                target: None,
            },
            BuildTask {
                name: "final".into(),
                depends_on: vec!["left".into(), "right".into()],
                target: None,
            },
        ],
        ..Default::default()
    };
    let order = order_build_tasks(&plan).unwrap();
    assert_eq!(order.len(), 4);
    assert_eq!(order[0], "base");
    assert_eq!(order[3], "final");
}

#[test]
fn execute_returns_task_order() {
    let mut opts = DriverOptions::default();
    let mut manifest = syx_pkg::SyxManifest::default();
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "only".into(),
            depends_on: vec![],
            target: Some("wasm".into()),
        }],
        ..Default::default()
    };
    let order =
        prepare_build_plan(&plan, &mut opts, &mut manifest, PathBuf::from(".").as_path()).unwrap();
    assert_eq!(order, vec!["only"]);
}

#[test]
fn parse_task_target_string() {
    let path = temp_build_syx(
        r#"
build {
  task wasm {
    target = "wasm"
  }
}
"#,
    );
    let plan = parse_build_syx(&path).unwrap();
    assert_eq!(plan.tasks[0].target.as_deref(), Some("wasm"));
}

#[test]
fn parse_compile_debug_level() {
    let path = temp_build_syx(
        r#"
build {
  compileOptions { optLevel = debug }
}
"#,
    );
    let plan = parse_build_syx(&path).unwrap();
    assert_eq!(plan.compile_options.opt_level, OptLevel::Debug);
}

#[test]
fn apply_overrides_native_link_flags() {
    let mut manifest = syx_pkg::SyxManifest::default();
    let plan = BuildPlan {
        native: syx_pkg::NativeConfig {
            link_flags: vec!["-lm".into()],
            ..Default::default()
        },
        ..Default::default()
    };
    prepare_build_plan(
        &plan,
        &mut DriverOptions::default(),
        &mut manifest,
        PathBuf::from(".").as_path(),
    )
    .unwrap();
    assert_eq!(manifest.native.link_flags, vec!["-lm"]);
}
