//! Language edge: actor isolation / C009 / policy / serial mutate (native MIR + wasm AOT).

use std::fs;

use syx_driver::{run_native_mir_file_printed, run_wasm_source_printed, DriverOptions};

const ISOLATION: &str = r#"
actor Counter {
    var value: Int
    init(value: Int) {
        self.value = value
    }
    func increment() {
        self.value = self.value + 1
    }
    func get() -> Int {
        return self.value
    }
}

func main() {
    let c = Counter(value: 0)
    c.increment()
    c.increment()
    print(c.get())
}
"#;

const ISOLATION_EXPECTED: &str = "2";

const C009: &str = r#"
actor Counter {
    var value: Int
    init(value: Int) {
        self.value = value
    }
    func inc() {
        self.value = self.value + 1
    }
    func bump() {
        self.inc()
    }
}

func main() {
    let c = Counter(value: 0)
    try {
        c.bump()
        print("OK")
    } catch (err) {
        print("DENIED")
        print(err.code)
    }
}
"#;

const C009_EXPECTED: &str = "DENIED\nC009";

const POLICY: &str = r#"
func main() {
    print(__syx_actor_reentrancy_policy())
}
"#;

const SERIAL_MUTATE: &str = r#"
actor Box {
    var value: Int
    init(value: Int) {
        self.value = value
    }
    func set(v: Int) {
        self.value = v
    }
    func get() -> Int {
        return self.value
    }
}

func main() {
    let b = Box(value: 0)
    b.set(1)
    b.set(2)
    print(b.get())
}
"#;

const SERIAL_MUTATE_EXPECTED: &str = "2";

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_src(label: &str, src: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w287_a04_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let path = dir.join("main.sy");
    fs::write(&path, src).expect("write main");
    path
}

fn run_native_mir(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &DriverOptions::default()).expect("native mir");
    assert_eq!(code, 0, "native mir exit {code}: {lines:?}");
    lines_joined(&lines)
}

fn run_wasm(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let path = write_src(label, src);
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &opts).expect("wasm");
    assert_eq!(code, 0, "wasm exit {code}: {lines:?}");
    lines_joined(&lines)
}

#[test]
fn edge_actor_isolation_native_mir() {
    let out = run_native_mir("iso", ISOLATION);
    assert_eq!(out, ISOLATION_EXPECTED, "native mir isolation: {out}");
}

#[test]
fn edge_actor_isolation_wasm() {
    let out = run_wasm("iso", ISOLATION);
    assert_eq!(out, ISOLATION_EXPECTED, "wasm isolation: {out}");
}

#[test]
fn edge_actor_c009_native_mir() {
    let out = run_native_mir("c009", C009);
    assert_eq!(out, C009_EXPECTED, "native mir C009: {out}");
    assert!(!out.contains("OK"), "reentrant must not succeed: {out}");
}

#[test]
fn edge_actor_c009_wasm() {
    let out = run_wasm("c009", C009);
    assert_eq!(out, C009_EXPECTED, "wasm C009: {out}");
    assert!(!out.contains("OK"), "reentrant must not succeed: {out}");
}

#[test]
fn edge_actor_policy_native_mir() {
    let out = run_native_mir("policy", POLICY);
    assert!(
        out.contains("serial-no-reenter"),
        "native mir policy: {out}"
    );
}

#[test]
fn edge_actor_policy_wasm() {
    let out = run_wasm("policy", POLICY);
    assert!(out.contains("serial-no-reenter"), "wasm policy: {out}");
}

#[test]
fn edge_actor_serial_mutate_native_mir() {
    let out = run_native_mir("serial", SERIAL_MUTATE);
    assert_eq!(out, SERIAL_MUTATE_EXPECTED, "native mir serial mutate: {out}");
}

#[test]
fn edge_actor_serial_mutate_wasm() {
    let out = run_wasm("serial", SERIAL_MUTATE);
    assert_eq!(out, SERIAL_MUTATE_EXPECTED, "wasm serial mutate: {out}");
}

#[test]
fn edge_spec_doc_cites_adr631() {
    let spec = include_str!("../../../../docs/gates/gate-a04-edge-semantics.md");
    assert!(
        spec.contains("ADR-631") && (spec.contains("actor ordering") || spec.contains("Actor")),
        "a04-edge-semantics.md must cite ADR-631 actor ordering"
    );
    assert!(
        !spec.contains("actor ordering | deferred")
            && !spec.contains("| actor ordering | deferred"),
        "spec must not leave actor deferred: remaining deferred domains forbidden"
    );
    assert!(
        spec.contains("ADR-632") && spec.contains("Q-L4 floor"),
        "spec must cite ADR-632 Q-L4 floor signed"
    );
    assert!(
        spec.contains("Not** Wave A")
            || spec.contains("**Not** Wave A")
            || spec.contains("Not Wave A"),
        "spec must remain Not Wave A exit"
    );
}

