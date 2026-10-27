//! async adapter thin start (File/HTTP on blocking_pool).
// was: m3_b02_async_adapter

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_b02_async_adapter_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_io_adapter_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("blocking-pool-async-adapter-v1-thin")
            || out.contains("blocking-pool-async-adapter-v1-tcp"),
        "ioAdapterPolicy fingerprint: {out}"
    );
}

#[test]
fn m3_b02_async_adapter_file_read_async_await() {
    let dir = tempfile::tempdir().expect("tempdir");
    let file_path = dir.path().join("b02_async.txt");
    std::fs::write(&file_path, "B02_FILE_OK").expect("write");
    let path = file_path.to_string_lossy().replace('\\', "\\\\");
    let src = format!(
        r#"
async func main() {{
  let text = await __syx_file_read_async("{path}")
  print(text)
}}
"#
    );
    for _ in 0..8 {
        let out = run(&src);
        assert!(
            out.contains("B02_FILE_OK"),
            "await FileReadAsync must return file content: {out}"
        );
        assert!(
            !out.trim().ends_with("b02_async.txt"),
            "await must not surface leftover CallHost path arg: {out}"
        );
    }
    drop(dir);
}

#[test]
fn m3_b02_async_adapter_does_not_starve_short_task() {
    // Task pool size=1: long work on blocking_pool must not starve a short Task.
    let src = r#"
func main() {
  __syx_task_set_worker_count(1)
  let longT = Task.detached {
    __syx_blocking_sleep(30000)
    print("LONG_DONE")
  }
  let shortT = Task.detached {
    print("SHORT_DONE")
  }
  __syx_task_join(shortT)
  print("SHORT_JOINED")
  __syx_task_cancel(longT)
  try {
    __syx_task_join(longT)
  } catch (err) {
    print("LONG_CANCELLED")
  }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        out.contains("SHORT_DONE") && out.contains("SHORT_JOINED"),
        "short Task must complete: {out}"
    );
    assert!(
        elapsed.as_millis() < 1500,
        "blocking_pool path must not starve Task pool: elapsed={elapsed:?} out={out}"
    );
}

#[test]
fn m3_b02_async_adapter_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_b02_async_adapter"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_b02_async_adapter = \"signed-ADR-278\""),
        "inventory must note signed-ADR-278"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-278")
            || guide.contains("readAsync")
            || guide.contains("M3-B02"),
        "UsageGuide must document M3-B02 async adapter"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-B02 |"))
        .expect("M3-B02 row");
    assert!(
        b02.contains("| [~] |") || b02.contains("| [x] |"),
        "M3-B02 may be [~] or [x] after later windows: {b02}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W22")
            && (roadmap.contains("C01")
                || roadmap.contains("Mutex")
                || roadmap.contains("M3-C01")))
            || (roadmap.contains("W21")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-280")
                    || roadmap.contains("B02")))
            || (roadmap.contains("W20")
                && (roadmap.contains("deepen")
                    || roadmap.contains("ADR-279")
                    || roadmap.contains("B02")))
            || (roadmap.contains("W19") && roadmap.contains("ADR-278")),
        "roadmap may advance W19–W21 done / W22 = C01 thin start"
    );
    assert!(
        !guide.to_lowercase().contains("epoll reactor ga")
            || guide.contains("≠")
            || guide.contains("非"),
        "docs must not claim epoll reactor GA without negation"
    );
    let facade = include_str!("../../../../packages/io/mod.sy");
    assert!(
        facade.contains("readAsync") && facade.contains("__syx_file_read_async"),
        "io File facade must expose readAsync"
    );
    let http = include_str!("../../../../crates/syx-vm/src/http.rs");
    assert!(
        http.contains("execute_on_blocking_pool"),
        "HttpRequestAsync must submit via blocking_pool bridge"
    );
}
