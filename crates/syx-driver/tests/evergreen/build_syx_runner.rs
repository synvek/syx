//! Phase 65 build.syb task runner tests.
// was: build_syx_runner

use std::fs;
use std::path::{Path, PathBuf};
use syx_driver::build_syx::{
    execute_build_plan, order_build_tasks, parse_build_syx, prepare_build_plan, run_build_tasks,
    BuildPlan, BuildTask, BuildTaskContext,
};
use syx_driver::DriverOptions;

fn default_opts() -> DriverOptions {
    DriverOptions {
        enable_std: false,
        ..DriverOptions::default()
    }
}

fn temp_project(name: &str, main: &str, build_syx: Option<&str>) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_build_runner_{name}_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("syx.toml"),
        format!(
            r#"name = "{name}"
version = "0.1.0"
main = "main.sy"
"#
        ),
    )
    .unwrap();
    fs::write(dir.join("main.sy"), main).unwrap();
    if let Some(content) = build_syx {
        fs::write(dir.join("build.syb"), content).unwrap();
    }
    dir
}

fn ctx<'a>(
    root: &'a Path,
    opts: &'a mut DriverOptions,
    manifest: &'a mut syx_pkg::SyxManifest,
) -> BuildTaskContext<'a> {
    BuildTaskContext::new(root, opts, manifest)
}

fn enable_native_test_stub() {
    std::env::set_var("SYX_NATIVE_C_STUB", "1");
}

#[test]
fn runner_mir_writes_artifact() {
    let dir = temp_project("mir1", r#"func main() { print("hi") }"#, None);
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "emit".into(),
            depends_on: vec![],
            target: Some("mir".into()),
        }],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let mut c = ctx(&dir, &mut opts, &mut manifest);
    run_build_tasks(&plan, &mut c).unwrap();
    let mir = dir.join("target/debug/out.mir");
    assert!(mir.exists(), "expected {}", mir.display());
    let text = fs::read_to_string(mir).unwrap();
    assert!(text.contains("main"));
}

#[test]
fn runner_wasm_writes_artifact() {
    let dir = temp_project("wasm1", r#"func main() { return 0 }"#, None);
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "wasm".into(),
            depends_on: vec![],
            target: Some("wasm".into()),
        }],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let mut c = ctx(&dir, &mut opts, &mut manifest);
    run_build_tasks(&plan, &mut c).unwrap();
    assert!(dir.join("target/debug/out.wasm").exists());
}

#[test]
fn runner_unknown_target_s403() {
    let dir = temp_project("badtgt", r#"func main() {}"#, None);
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "x".into(),
            depends_on: vec![],
            target: Some("docker".into()),
        }],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let err = run_build_tasks(&plan, &mut ctx(&dir, &mut opts, &mut manifest)).unwrap_err();
    assert_eq!(err.code.as_deref(), Some("S403"));
}

#[test]
fn runner_compile_failure_s404() {
    let dir = temp_project("broken", r#"func main() { let x: Int = "bad" }"#, None);
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "mir".into(),
            depends_on: vec![],
            target: Some("mir".into()),
        }],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let err = run_build_tasks(&plan, &mut ctx(&dir, &mut opts, &mut manifest)).unwrap_err();
    assert_eq!(err.code.as_deref(), Some("S404"));
}

#[test]
fn runner_topological_mir_before_wasm() {
    let dir = temp_project("dag1", r#"func main() { return 1 }"#, None);
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "mir".into(),
                depends_on: vec![],
                target: Some("mir".into()),
            },
            BuildTask {
                name: "wasm".into(),
                depends_on: vec!["mir".into()],
                target: Some("wasm".into()),
            },
        ],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let mut c = ctx(&dir, &mut opts, &mut manifest);
    run_build_tasks(&plan, &mut c).unwrap();
    assert_eq!(c.executed, vec!["mir", "wasm"]);
    assert!(dir.join("target/debug/out.mir").exists());
    assert!(dir.join("target/debug/out.wasm").exists());
}

#[test]
fn runner_task_without_target_records_name() {
    let dir = temp_project("notgt", r#"func main() {}"#, None);
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "noop".into(),
            depends_on: vec![],
            target: None,
        }],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let mut c = ctx(&dir, &mut opts, &mut manifest);
    run_build_tasks(&plan, &mut c).unwrap();
    assert_eq!(c.executed, vec!["noop"]);
}

#[test]
fn runner_empty_tasks_ok() {
    let dir = temp_project("empty", r#"func main() {}"#, None);
    let plan = BuildPlan::default();
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    run_build_tasks(&plan, &mut ctx(&dir, &mut opts, &mut manifest)).unwrap();
}

#[test]
fn runner_cycle_s402() {
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "a".into(),
                depends_on: vec!["b".into()],
                target: Some("mir".into()),
            },
            BuildTask {
                name: "b".into(),
                depends_on: vec!["a".into()],
                target: Some("mir".into()),
            },
        ],
        ..Default::default()
    };
    let err = order_build_tasks(&plan).unwrap_err();
    assert_eq!(err.code.as_deref(), Some("S402"));
}

#[test]
fn runner_unknown_dep_s401() {
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "a".into(),
            depends_on: vec!["ghost".into()],
            target: Some("mir".into()),
        }],
        ..Default::default()
    };
    let err = order_build_tasks(&plan).unwrap_err();
    assert_eq!(err.code.as_deref(), Some("S401"));
}

#[test]
fn runner_native_object() {
    enable_native_test_stub();
    let dir = temp_project("native1", r#"func main() { return 0 }"#, None);
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "obj".into(),
            depends_on: vec![],
            target: Some("native".into()),
        }],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    run_build_tasks(&plan, &mut ctx(&dir, &mut opts, &mut manifest)).unwrap();
    assert!(dir.join("target/debug/out.o").exists());
}

#[test]
fn runner_test_target_runs_main() {
    let dir = temp_project("testmain", r#"func main() { print("runner") }"#, None);
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "test".into(),
            depends_on: vec![],
            target: Some("test".into()),
        }],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    run_build_tasks(&plan, &mut ctx(&dir, &mut opts, &mut manifest)).unwrap();
}

#[test]
fn runner_diamond_deps() {
    let dir = temp_project("diamond", r#"func main() { return 0 }"#, None);
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "base".into(),
                depends_on: vec![],
                target: Some("mir".into()),
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
                target: Some("wasm".into()),
            },
        ],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let mut c = ctx(&dir, &mut opts, &mut manifest);
    run_build_tasks(&plan, &mut c).unwrap();
    assert_eq!(c.executed.len(), 4);
    assert_eq!(c.executed[0], "base");
    assert_eq!(c.executed[3], "final");
}

#[test]
fn execute_build_plan_full_pipeline() {
    let dir = temp_project(
        "full",
        r#"func main() { return 42 }"#,
        Some(
            r#"
build {
  task mir {
    target = "mir"
  }
  task wasm {
    dependsOn = ["mir"]
    target = "wasm"
  }
}
"#,
        ),
    );
    let plan = parse_build_syx(&dir.join("build.syb")).unwrap();
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let executed = execute_build_plan(&plan, &mut opts, &mut manifest, &dir).unwrap();
    assert_eq!(executed, vec!["mir", "wasm"]);
}

#[test]
fn runner_all_known_targets_recognized() {
    enable_native_test_stub();
    for target in ["mir", "wasm", "native", "link", "test"] {
        let dir = temp_project(
            &format!("tgt_{target}"),
            r#"func main() { return 0 }"#,
            None,
        );
        let plan = BuildPlan {
            tasks: vec![BuildTask {
                name: "t".into(),
                depends_on: vec![],
                target: Some(target.into()),
            }],
            ..Default::default()
        };
        let mut opts = default_opts();
        let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
        let err = run_build_tasks(&plan, &mut ctx(&dir, &mut opts, &mut manifest));
        if target == "link" {
            // link may fail without full toolchain; S404 is acceptable, not S403
            if let Err(e) = err {
                assert_ne!(e.code.as_deref(), Some("S403"), "target {target}");
            }
        } else {
            err.unwrap_or_else(|e| panic!("target {target}: {e}"));
        }
    }
}

#[test]
fn runner_plugins_wired_via_prepare() {
    let dir = temp_project("plug", r#"func main() {}"#, None);
    fs::write(
        dir.join("build.syb"),
        r#"build { plugins { echo } }"#,
    )
    .unwrap();
    let plan = parse_build_syx(&dir.join("build.syb")).unwrap();
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    prepare_build_plan(&plan, &mut opts, &mut manifest, &dir).unwrap();
}

#[test]
fn runner_release_profile_from_build_syx() {
    let dir = temp_project(
        "rel",
        r#"func main() { return 0 }"#,
        Some(
            r#"
build {
  compileOptions {
    optLevel = release
  }
}
"#,
        ),
    );
    let plan = parse_build_syx(&dir.join("build.syb")).unwrap();
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    prepare_build_plan(&plan, &mut opts, &mut manifest, &dir).unwrap();
    assert!(opts.wasm_aot);
}

#[test]
fn runner_wasm_aot_feature_flag() {
    let dir = temp_project(
        "aot",
        r#"func main() { return 0 }"#,
        Some(
            r#"
build {
  features {
    wasm_aot = true
  }
}
"#,
        ),
    );
    let plan = parse_build_syx(&dir.join("build.syb")).unwrap();
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    prepare_build_plan(&plan, &mut opts, &mut manifest, &dir).unwrap();
    assert!(opts.wasm_aot);
}

#[test]
fn runner_native_link_flags_applied() {
    let dir = temp_project("linkflags", r#"func main() {}"#, None);
    let plan = BuildPlan {
        native: syx_pkg::NativeConfig {
            link_flags: vec!["-lm".into()],
            ..Default::default()
        },
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    prepare_build_plan(&plan, &mut opts, &mut manifest, &dir).unwrap();
    assert_eq!(manifest.native.link_flags, vec!["-lm"]);
}

#[test]
fn runner_parse_file_dag_order() {
    let dir = temp_project(
        "parsedag",
        r#"func main() {}"#,
        Some(
            r#"
build {
  task codegen {
    target = "mir"
  }
  task link {
    dependsOn = ["codegen"]
    target = "native"
  }
  task verify {
    dependsOn = ["link"]
  }
}
"#,
        ),
    );
    let plan = parse_build_syx(&dir.join("build.syb")).unwrap();
    let order = order_build_tasks(&plan).unwrap();
    assert_eq!(order, vec!["codegen", "link", "verify"]);
}

#[test]
fn runner_mir_then_native_chain() {
    enable_native_test_stub();
    let dir = temp_project("chain", r#"func main() { return 0 }"#, None);
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "mir".into(),
                depends_on: vec![],
                target: Some("mir".into()),
            },
            BuildTask {
                name: "native".into(),
                depends_on: vec!["mir".into()],
                target: Some("native".into()),
            },
        ],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    run_build_tasks(&plan, &mut ctx(&dir, &mut opts, &mut manifest)).unwrap();
    assert!(dir.join("target/debug/out.mir").exists());
    assert!(dir.join("target/debug/out.o").exists());
}

#[test]
fn runner_unknown_target_mid_pipeline_s403() {
    let dir = temp_project("midfail", r#"func main() { return 0 }"#, None);
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "ok".into(),
                depends_on: vec![],
                target: Some("mir".into()),
            },
            BuildTask {
                name: "bad".into(),
                depends_on: vec!["ok".into()],
                target: Some("unknown".into()),
            },
        ],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let mut c = ctx(&dir, &mut opts, &mut manifest);
    let err = run_build_tasks(&plan, &mut c).unwrap_err();
    assert_eq!(err.code.as_deref(), Some("S403"));
    assert_eq!(c.executed, vec!["ok"]);
}

#[test]
fn runner_in_build_syx_task_prevents_recursion() {
    let dir = temp_project(
        "recurse",
        r#"func main() { return 0 }"#,
        Some(
            r#"
build {
  task mir {
    target = "mir"
  }
}
"#,
        ),
    );
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    let plan = parse_build_syx(&dir.join("build.syb")).unwrap();
    execute_build_plan(&plan, &mut opts, &mut manifest, &dir).unwrap();
    // compile_project during mir task must not re-enter task runner
    syx_driver::compile_project_with_options(&dir, &opts).unwrap();
}

#[test]
fn runner_test_dir_runs_syx_files() {
    let dir = temp_project("testdir", r#"func main() {}"#, None);
    let tests = dir.join("tests");
    fs::create_dir_all(&tests).unwrap();
    fs::write(tests.join("smoke.sy"), r#"func main() { print("t") }"#).unwrap();
    let plan = BuildPlan {
        tasks: vec![BuildTask {
            name: "tests".into(),
            depends_on: vec![],
            target: Some("test".into()),
        }],
        ..Default::default()
    };
    let mut opts = default_opts();
    let mut manifest = syx_pkg::load_manifest(&dir).unwrap();
    run_build_tasks(&plan, &mut ctx(&dir, &mut opts, &mut manifest)).unwrap();
}
