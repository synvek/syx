//! TaskGroupCreate/Add/Wait/Cancel HostFn llvm=real.

use std::fs;
use std::process::Command;
use std::time::Instant;

use syx_driver::{build_native_executable, DriverOptions};

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn run_native_strict(src: &str) -> (i32, String, std::time::Duration) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w263_task_group_hosts_llvm_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let started = Instant::now();
    let output = Command::new(&exe).output().expect("run native exe");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        started.elapsed(),
    )
}

const TASK_GROUP_CANCEL_WAIT: &str = r#"
func main() {
  let g = __syx_task_group_create()
  let t = Task.detached {
    __syx_task_sleep(30000)
    print("SLEPT")
  }
  __syx_task_group_add(g, t)
  __syx_task_group_cancel(g)
  __syx_task_group_wait(g)
  print("WAITED")
  try {
    __syx_task_join(t)
    print("JOIN_OK")
  } catch (err) {
    print("JOIN_CANCEL")
    print(err.code)
  }
}
"#;

#[test]
fn task_group_cancel_wait_native_llvm() {
    let (code, stdout, elapsed) = run_native_strict(TASK_GROUP_CANCEL_WAIT);
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert!(
        elapsed.as_secs() < 5,
        "cancel+wait must not hang for full sleep: elapsed={elapsed:?} out={stdout}"
    );
    assert!(stdout.contains("WAITED"), "wait must complete: {stdout}");
    assert!(
        !stdout.contains("SLEPT"),
        "cancelled sleeper must not finish sleep: {stdout}"
    );
    assert!(
        stdout.contains("JOIN_CANCEL"),
        "join after cancel must C004 path: {stdout}"
    );
    assert!(
        stdout.contains("C004"),
        "err.code must be C004: {stdout}"
    );
    assert!(!stdout.contains("JOIN_OK"), "{stdout}");
}

#[test]
fn task_group_hosts_llvm_real_meta() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "TaskGroupCreate",
        "TaskGroupAdd",
        "TaskGroupWait",
        "TaskGroupCancel",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-607): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-620): {block}"
        );
        assert!(
            block.contains("ADR-620") && block.contains("ADR-607"),
            "{name} notes must cite ADR-620+607: {block}"
        );
    }
}
