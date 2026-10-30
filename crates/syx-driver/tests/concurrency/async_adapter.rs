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

