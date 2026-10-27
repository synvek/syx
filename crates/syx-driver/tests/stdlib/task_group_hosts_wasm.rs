//! TaskGroupCreate/Add/Wait/Cancel HostFn wasm=real.

use std::fs;
use std::time::Instant;

use syx_driver::{run_wasm_source_printed, DriverOptions};

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn run_wasm(src: &str) -> (Vec<String>, std::time::Duration) {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let dir = std::env::temp_dir().join(format!(
        "syx_w276_task_group_hosts_wasm_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let started = Instant::now();
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm task group host failed: {e:?}"));
    let elapsed = started.elapsed();
    let _ = fs::remove_dir_all(&dir);
    (
        lines
            .into_iter()
            .filter(|l| !l.is_empty())
            .collect(),
        elapsed,
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
fn task_group_cancel_wait_wasm() {
    let (lines, elapsed) = run_wasm(TASK_GROUP_CANCEL_WAIT);
    let stdout = lines.join("\n");
    // Wall clock includes AOT emit (llvm nail times exe only). Budget must still
    // fail if sleep ignores cancel (full 30s); allow compile overhead under 25s.
    assert!(
        elapsed.as_secs() < 25,
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
fn task_group_hosts_wasm_real_meta() {
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
            "{name} must keep llvm=real: {block}"
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
