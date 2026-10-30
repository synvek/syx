//! 03 Child.signal nail.
//! ChildSignal HostFn llvm=real.
// was: m2_s6_03_child_signal

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w246_child_signal_llvm_{}",
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
    let output = Command::new(&exe).output().expect("run native exe");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

fn run(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => r.printed.join("\n"),
        Err(e) => panic!("compile/run failed: {e:?}\n---src---\n{src}"),
    }
}

fn facade_src(body: &str) -> String {
    let proc = include_str!("../../../../packages/process/mod.sy");
    let pb = include_str!("../../../../packages/process/process_builder.sy");
    format!("{proc}\n{pb}\n{body}")
}

#[cfg(unix)]
#[test]
fn m2_s6_03_term_then_wait_nonzero() {
    // Bare hosts: signal must not remove handle; wait must observe non-zero (signaled).
    let out = run(&facade_src(
        r#"
func main() {
  let b = __syx_process_builder_create("/bin/sleep", ["30"])
  let h = __syx_process_builder_spawn(b)
  __syx_child_signal(h, "TERM")
  let code = __syx_child_wait(h)
  print("OK")
  print(code)
}
"#,
    ));
    // Wave A W88 / ADR-432 / A-03: TERM then wait signaled exit product exact.
    assert_eq!(out, "OK\n-1", "TERM+wait must exact: {out}");
}

#[cfg(unix)]
#[test]
fn m2_s6_03_kill_smoke_then_signal_after_wait_fails() {
    // Wave A W58 / ADR-402 / A-03: Child.signal KILL fail-after product exact (drop print(code)).
    let out = run(&facade_src(
        r#"
func main() {
  let b = __syx_process_builder_create("/bin/sleep", ["30"])
  let h = __syx_process_builder_spawn(b)
  __syx_child_signal(h, "KILL")
  __syx_child_wait(h)
  print("WAITED")
  try {
    __syx_child_signal(h, "TERM")
    print("SIGNAL_OK")
  } catch (e) {
    print("SIGNAL_FAIL")
  }
}
"#,
    ));
    assert_eq!(
        out, "WAITED\nSIGNAL_FAIL",
        "KILL then signal-after-wait must exact: {out}"
    );
}

#[test]
fn m2_s6_03_illegal_name_throws() {
    // Wave A W58 / ADR-402 / A-03: Child.signal illegal name product exact (drop print(e)).
    // Measured: inner catch prints BAD_NAME; wait code after catch is not in stdout.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    try {
      Child.signal(handle: h, name: "FOO")
      print("BAD_OK")
    } catch (e) {
      print("BAD_NAME")
    }
    let code = Child.wait(handle: h)
  } catch (e) {
    print("SETUP_ERR")
  }
}
"#,
    ));
    assert_eq!(out, "BAD_NAME", "illegal signal name must exact: {out}");
}

#[test]
fn m2_s6_03_docs_and_capability() {
    let pb = include_str!("../../../../packages/process/process_builder.sy");
    assert!(
        pb.contains("func signal")
            && pb.contains("__syx_child_signal")
            && pb.contains("ADR-244"),
        "Child.signal facade required"
    );
    let hosts = include_str!("../../../syx-vm/src/stdlib_phase84_hosts.rs");
    assert!(
        hosts.contains("host_child_signal")
            && hosts.contains("SIGTERM")
            && hosts.contains("unsupported on Windows"),
        "signal host + Windows honesty required"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("Child.signal")
            && guide.contains("ADR-244")
            && guide.contains("Child.kill"),
        "UsageGuide must document signal vs kill"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "os.process.signal");
    // Honesty: wasm=unsupported keeps overall status=partial (≠ product implemented).
    assert!(
        block.contains("status = \"partial\"")
            && block.contains("native = \"real\"")
            && block.contains("wasm = \"unsupported\""),
        "os.process.signal must stay partial/native=real/wasm=unsupported: {block}"
    );
    assert!(
        (block.contains("ADR-602") || block.contains("ADR-244")) && block.contains("W258"),
        "os.process.signal notes must cite ADR-602: {block}"
    );
    assert!(
        block.contains("m2_s6_03_child_signal"),
        "tests must list nail"
    );
    assert!(
        block.contains("Windows") || block.contains("KILL-only") || block.contains("TERM/INT"),
        "notes must document Windows honesty: {block}"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s6_03_child_signal_tier = \"signed-ADR-244\""),
        "inventory must note signed-ADR-244"
    );
}


#[cfg(unix)]
#[test]
fn m2_s6_03_term_native_llvm() {
    // Wave A W246 / ADR-590 — bare hosts under SYX_NATIVE_STRICT.
    let src = facade_src(
        r#"
func main() {
  let b = __syx_process_builder_create("/bin/sleep", ["30"])
  let h = __syx_process_builder_spawn(b)
  __syx_child_signal(h, "TERM")
  let code = __syx_child_wait(h)
  print("OK")
  print(code)
}
"#,
    );
    let (code, stdout) = run_native_strict(&src);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "OK\n-1",
        "native llvm ChildSignal TERM+wait must exact: {out}"
    );
}

#[test]
fn m2_s6_03_child_signal_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "ChildSignal");
    assert!(
        block.contains("llvm = \"real\""),
        "ChildSignal must declare llvm=real (ADR-590): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "ChildSignal must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"unsupported\""),
        "ChildSignal must keep wasm=unsupported: {block}"
    );
    assert!(
        block.contains("ADR-590") && block.contains("ADR-244"),
        "ChildSignal notes must cite ADR-590 + ADR-244: {block}"
    );
}
