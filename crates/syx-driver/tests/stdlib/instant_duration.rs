//! 01 Instant/Duration separation (monotonic vs wall).
//! datetime.clock native=real + TimeNowMs/InstantNowMs llvm=real.
//! TimeNowMs/InstantNowMs HostFn wasm=real.
//! datetime.clock product wasm=real.
// was: m2_s5_01_instant_duration

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_wasm_source_printed, DriverOptions,
};

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn datetime_src(body: &str) -> String {
    let dt = include_str!("../../../../packages/datetime/mod.sy");
    format!("{dt}\n{body}")
}

#[test]
fn m2_s5_01_instant_api_surface() {
    let src = include_str!("../../../../packages/datetime/mod.sy");
    assert!(
        src.contains("class Instant"),
        "Instant class required: {src}"
    );
    assert!(
        src.contains("__syx_instant_now_ms"),
        "Instant must call monotonic host: {src}"
    );
    assert!(
        src.contains("elapsedMs"),
        "Instant.elapsedMs required: {src}"
    );
    assert!(
        src.contains("class Duration"),
        "Duration helpers must remain: {src}"
    );
    assert!(
        src.contains("wall") || src.contains("Wall") || src.contains("epoch"),
        "Clock/wall honesty comments required: {src}"
    );
}

#[test]
fn m2_s5_01_host_fn_columns() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["InstantNowMs", "TimeNowMs"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("vm = \"real\""),
            "{name} must declare vm=real: {block}"
        );
        assert!(
            block.contains("mir = \"real\""),
            "{name} must declare mir=real: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-600): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-618): {block}"
        );
        assert!(
            block.contains("ADR-618") && block.contains("ADR-600"),
            "{name} notes must cite ADR-618+600: {block}"
        );
    }
}

#[test]
fn m2_s5_01_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "datetime.clock");
    assert!(
        block.contains("Instant"),
        "datetime.clock surface must mention Instant: {block}"
    );
    assert!(
        block.contains("native = \"real\""),
        "datetime.clock must declare native=real (ADR-600): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "datetime.clock must declare wasm=real (ADR-622): {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "datetime.clock status must stay implemented: {block}"
    );
    assert!(
        (block.contains("ADR-600") || block.contains("ADR-220")) && block.contains("W256"),
        "datetime.clock notes must cite ADR-600+220: {block}"
    );
    assert!(
        block.contains("ADR-622") && block.contains("W278"),
        "datetime.clock notes must cite ADR-622+W278: {block}"
    );
    assert!(
        block.contains("m2_s5_01_instant_duration"),
        "datetime.clock tests must include nail: {block}"
    );
    assert!(
        !block.contains("Monotonic/wall ms + Duration arithmetic"),
        "must not keep pre-separation dishonest notes: {block}"
    );
}

#[test]
fn m2_s5_01_wall_vs_monotonic_and_elapsed() {
    let out = run(&datetime_src(
        r#"func main() {
    let t0 = Instant.now()
    __syx_time_sleep_ms(50)
    let elapsed = Instant.elapsedMs(start: t0)
    let wall = Clock.nowMs()
    let tick = Instant.now()
    print(elapsed)
    print(wall)
    print(tick)
    print(Duration.seconds(value: 1))
}"#,
    ));
    let lines: Vec<&str> = out.lines().map(|l| l.trim()).collect();
    assert!(
        lines.len() >= 4,
        "expected elapsed, wall, tick, duration; got {out:?}"
    );
    let elapsed: i64 = lines[0].parse().expect("elapsed int");
    let wall: i64 = lines[1].parse().expect("wall int");
    let tick: i64 = lines[2].parse().expect("tick int");
    assert!(
        elapsed >= 40,
        "elapsed after sleepMs(50) must be >= 40; got {elapsed}; out={out}"
    );
    assert!(
        wall > 1_000_000_000_000,
        "wall Clock.nowMs must be UNIX epoch-scale; got {wall}"
    );
    assert!(
        tick < wall / 1000,
        "monotonic tick must be << wall epoch; tick={tick} wall={wall}"
    );
    assert_eq!(lines[3], "1000", "Duration.seconds(1) must stay 1000: {out}");
}

/// Wave A W256 / ADR-600 — Clock.nowMs + Instant.now native llvm epoch-scale / tick.
#[cfg(unix)]
#[test]
fn m2_s5_01_clock_instant_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = datetime_src(
        r#"func main() {
  let wall = Clock.nowMs()
  let tick = Instant.now()
  print(wall)
  print(tick)
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_clock_instant_native_llvm_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines.len(),
        2,
        "clock/instant native llvm must print wall+tick: {lines:?}"
    );
    let wall: i64 = lines[0].parse().expect("wall int");
    let tick: i64 = lines[1].parse().expect("tick int");
    assert!(
        wall > 1_000_000_000_000,
        "Clock.nowMs native llvm must be epoch-scale; got {wall}"
    );
    assert!(
        tick < wall / 1000,
        "Instant.now native llvm tick must be << wall; tick={tick} wall={wall}"
    );
}

/// Wave A W274 / ADR-618 — TimeNowMs/InstantNowMs wasm epoch-scale / tick narrow.
#[test]
fn m2_s5_01_clock_instant_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = datetime_src(
        r#"func main() {
  let wall = Clock.nowMs()
  let tick = Instant.now()
  print(wall)
  print(tick)
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_clock_instant_wasm_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm clock/instant failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<&str> = lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines.len(),
        2,
        "clock/instant wasm must print wall+tick: {lines:?}"
    );
    let wall: i64 = lines[0].parse().expect("wall int");
    let tick: i64 = lines[1].parse().expect("tick int");
    assert!(
        wall > 1_000_000_000_000,
        "Clock.nowMs wasm must be epoch-scale; got {wall}"
    );
    assert!(
        tick < wall / 1000,
        "Instant.now wasm tick must be << wall; tick={tick} wall={wall}"
    );
}

#[test]
fn m2_s5_01_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s5_01_instant_duration_tier"),
        "inventory must contain m2_s5_01_instant_duration_tier"
    );
    assert!(
        inv.contains("ADR-220") || inv.contains("signed-ADR-220"),
        "inventory must note ADR-220"
    );
}

#[test]
fn m2_s5_01_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s5_01_instant_duration"
    );
}
