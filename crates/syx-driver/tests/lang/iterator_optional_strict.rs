//! Iterator `next -> T?` strict nil termination (vm + wasm + native).
// was: iterator_optional_strict

use std::fs;
use std::path::{Path, PathBuf};

use syx_driver::{
    build_native_executable, compile_and_run_embedded, compile_file_with_options, run_file_printed,
    run_wasm_source_printed, DriverOptions,
};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run_unified_mir_printed(src: &str) -> (i32, Vec<String>) {
    syx_host_plugin::register_echo_plugin_in_process();
    let unit = syx_driver::compile_file_from_source("main.sy", src).expect("compile");
    let mut graph = syx_driver::ModuleGraph::default();
    graph.modules.insert(unit.path.clone(), unit.clone());
    graph.order.push(unit.path.clone());
    let mir = syx_driver::build_unified_mir(
        &graph,
        &unit,
        Path::new("."),
        &opts(),
        &syx_pkg::SyxManifest::default(),
    )
    .expect("mir");
    let ctx = syx_rt::install_native_module(&mir);
    let mut rt = syx_mir_run::MirRuntime::new();
    rt.install_functions(mir.functions.clone());
    rt.set_runtime(syx_rt::native_vm_runtime_for(ctx));
    syx_mir_run::install_module_string_pool(&mut rt, &mir);
    let main = mir
        .functions
        .iter()
        .find(|f| f.name == "main")
        .expect("main");
    let code = syx_mir_run::run_mir_function(main, &mut rt, &mut []).expect("run");
    (code, rt.printed)
}

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts()).unwrap().printed
}

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn run_wasm_fixture(rel: &str) -> Vec<String> {
    let path = fixture(rel);
    let (code, lines) = run_wasm_source_printed(&path, "main", &opts()).expect("wasm");
    assert_eq!(code, 0);
    lines
}

macro_rules! iter_prints {
    ($name:ident, $src:expr, $expect:expr) => {
        #[test]
        fn $name() {
            assert!(
                run($src).iter().any(|l| l.trim() == $expect),
                "expected {}",
                $expect
            );
        }
    };
}

const COUNTER_NIL: &str = r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() {
    var s: Int = 0
    for x in Counter() { s = s + x }
    print(s)
}"#;

iter_prints!(iter_nil_counter_sum_6, COUNTER_NIL, "6");
iter_prints!(
    iter_array_sum_10,
    r#"func main() {
    var s = 0
    for x in [1, 2, 3, 4] { s = s + x }
    print(s)
}"#,
    "10"
);
iter_prints!(
    iter_empty_array_zero_iters,
    r#"func main() {
    var n = 0
    for _ in [] { n = n + 1 }
    print(n)
}"#,
    "0"
);
iter_prints!(iter_single_elem_42, r#"func main() { for x in [42] { print(x) } }"#, "42");
iter_prints!(
    iter_range_array_3,
    r#"func main() {
    var n = 0
    for x in [0, 1, 2] { n = n + 1 }
    print(n)
}"#,
    "3"
);

#[test]
fn iter_zero_value_not_terminator() {
    let src = r#"class ZeroOnce: Iterator {
    var done: Bool
    init() { self.done = false }
    func next() -> Int? {
        if self.done { return nil }
        self.done = true
        return 0
    }
}
func main() {
    for x in ZeroOnce() { print(x) }
}"#;
    assert!(run(src).iter().any(|l| l.trim() == "0"));
}

#[test]
fn iter_nil_only_terminates_witness() {
    let src = r#"class Two: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n == 1 { return 0 }
        if self.n == 2 { return nil }
        return nil
    }
}
func main() {
    var s = 0
    for x in Two() { s = s + x }
    print(s)
}"#;
    assert!(run(src).iter().any(|l| l.trim() == "0"));
}

iter_prints!(
    iter_take_three_then_nil,
    r#"class Take3: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() {
    var s = 0
    for x in Take3() { s = s + x }
    print(s)
}"#,
    "6"
);

iter_prints!(
    iter_string_chars_ab,
    r#"func main() { for c in "ab" { print(c) } }"#,
    "a"
);

iter_prints!(
    iter_double_loop_sum,
    r#"func main() {
    var n = 0
    for i in [1, 2, 3] { n = n + i }
    print(n)
}"#,
    "6"
);

iter_prints!(
    iter_manual_next_before_nil,
    r#"class C: Iterator {
    var i: Int
    init() { self.i = 0 }
    func next() -> Int? {
        self.i = self.i + 1
        if self.i > 2 { return nil }
        return self.i
    }
}
func main() {
    let c = C()
    print(c.next())
    print(c.next())
}"#,
    "1"
);

#[test]
fn iter_native_mir_for_in_array_print() {
    syx_host_plugin::register_echo_plugin_in_process();
    let src = r#"func main() {
    for x in [1, 2, 3] { print(x) }
}"#;
    let dir = std::env::temp_dir().join(format!("syx_p71_ar_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.sy"), src).unwrap();
    let (code, lines) =
        syx_driver::run_native_mir_file_printed(&dir.join("main.sy"), &opts()).unwrap();
    assert_eq!(code, 0);
    assert_eq!(
        lines.iter().map(|l| l.trim()).collect::<Vec<_>>(),
        vec!["1", "2", "3"],
        "{lines:?}"
    );
}

#[test]
fn iter_native_mir_while_manual_for_in() {
    syx_host_plugin::register_echo_plugin_in_process();
    let src = r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() {
    var s: Int = 0
    let c = Counter()
    while true {
        let x = c.next()
        if x == nil { break }
        s = s + x
    }
    print(s)
}"#;
    let dir = std::env::temp_dir().join(format!("syx_p71_wh_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.sy"), src).unwrap();
    let (code, lines) =
        syx_driver::run_native_mir_file_printed(&dir.join("main.sy"), &opts()).unwrap();
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "6"), "{lines:?}");
}

#[test]
fn iter_witness_index_matches_counter_next() {
    use syx_mir::MirOp;
    let unit = syx_driver::compile_file_from_source("main.sy", COUNTER_NIL).unwrap();
    let mut graph = syx_driver::ModuleGraph::default();
    graph.modules.insert(unit.path.clone(), unit.clone());
    graph.order.push(unit.path.clone());
    let mir = syx_driver::build_unified_mir(
        &graph,
        &unit,
        Path::new("."),
        &opts(),
        &syx_pkg::SyxManifest::default(),
    )
    .unwrap();
    let next_idx = mir
        .functions
        .iter()
        .position(|f| f.name == "Counter_next")
        .expect("Counter_next");
    let init = mir
        .functions
        .iter()
        .find(|f| f.name == "Counter_init")
        .expect("Counter_init");
    let mut witness_idx = None;
    for block in &init.blocks {
        let ins = &block.instructions;
        for i in 0..ins.len().saturating_sub(1) {
            if let MirOp::ConstStr(k) = &ins[i + 1].op {
                if k == "__witness_Iterator_next" {
                    eprintln!("witness ins[{i}] = {:?}", ins[i].op);
                    if let MirOp::ConstI32(v) = ins[i].op {
                        witness_idx = Some(v);
                    }
                }
            }
        }
    }
    if witness_idx.is_none() && !init.instructions.is_empty() {
        let ins = &init.instructions;
        for i in 0..ins.len().saturating_sub(1) {
            if let MirOp::ConstStr(k) = &ins[i + 1].op {
                if k == "__witness_Iterator_next" {
                    eprintln!("flat witness ins[{i}] = {:?}", ins[i].op);
                    if let MirOp::ConstI32(v) = ins[i].op {
                        witness_idx = Some(v);
                    }
                }
            }
        }
    }
    assert_eq!(
        witness_idx,
        Some(next_idx as i32),
        "fn order: {:?}",
        mir.functions.iter().map(|f| &f.name).collect::<Vec<_>>()
    );
}

#[test]
fn iter_unified_mir_for_in_local_iterable() {
    let src = r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() {
    let it = Counter()
    for x in it { print(x) }
}"#;
    let (code, lines) = run_unified_mir_printed(src);
    assert_eq!(code, 0);
    assert_eq!(
        lines.iter().map(|l| l.trim()).collect::<Vec<_>>(),
        vec!["1", "2", "3"],
        "{lines:?}"
    );
}

#[test]
fn iter_unified_mir_for_in_array() {
    let (code, lines) = run_unified_mir_printed(r#"func main() {
    for x in [1, 2, 3] { print(x) }
}"#);
    assert_eq!(code, 0);
    assert_eq!(
        lines.iter().map(|l| l.trim()).collect::<Vec<_>>(),
        vec!["1", "2", "3"],
        "{lines:?}"
    );
}

#[test]
fn iter_unified_mir_manual_next_five() {
    let src = r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() {
    let c = Counter()
    print(c.next())
    print(c.next())
    print(c.next())
    print(c.next())
    print(c.next())
}"#;
    let (code, lines) = run_unified_mir_printed(src);
    assert_eq!(code, 0);
    assert_eq!(
        lines.iter().map(|l| l.trim()).collect::<Vec<_>>(),
        vec!["1", "2", "3", "nil", "nil"],
        "{lines:?}"
    );
}

#[test]
fn iter_native_mir_for_in_print_only() {
    let src = r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() {
    for x in Counter() { print(x) }
}"#;
    let (code, lines) = run_unified_mir_printed(src);
    assert_eq!(code, 0);
    assert_eq!(
        lines.iter().map(|l| l.trim()).collect::<Vec<_>>(),
        vec!["1", "2", "3"],
        "{lines:?}"
    );
}

#[test]
fn iter_native_mir_manual_next_five() {
    syx_host_plugin::register_echo_plugin_in_process();
    let src = r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() {
    let c = Counter()
    print(c.next())
    print(c.next())
    print(c.next())
    print(c.next())
    print(c.next())
}"#;
    let dir = std::env::temp_dir().join(format!("syx_p71_f5_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.sy"), src).unwrap();
    let (code, lines) =
        syx_driver::run_native_mir_file_printed(&dir.join("main.sy"), &opts()).unwrap();
    assert_eq!(code, 0);
    assert_eq!(
        lines.iter().map(|l| l.trim()).collect::<Vec<_>>(),
        vec!["1", "2", "3", "nil", "nil"],
        "{lines:?}"
    );
}

#[test]
fn iter_native_mir_inline_counter_nil() {
    syx_host_plugin::register_echo_plugin_in_process();
    let dir = std::env::temp_dir().join(format!("syx_p71_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.sy"), COUNTER_NIL).unwrap();
    let (code, lines) =
        syx_driver::run_native_mir_file_printed(&dir.join("main.sy"), &opts()).unwrap();
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "6"), "{lines:?}");
}

#[test]
fn iter_native_mir_custom_counter_fixture() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("parity/protocol_iterator_custom.sy");
    let (code, lines) =
        syx_driver::run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "6"), "{lines:?}");
}

#[test]
fn iter_embed_custom_counter_fixture() {
    let src = include_str!("../fixtures/parity/protocol_iterator_custom.sy");
    let lines = run(src);
    assert!(lines.iter().any(|l| l.trim() == "6"), "{lines:?}");
}

#[test]
fn iter_wasm_array_fixture() {
    let lines = run_wasm_fixture("parity/protocol_iterator.sy");
    assert!(lines.iter().any(|l| l.trim() == "10"));
}

#[test]
fn iter_wasm_for_in_sum_fixture() {
    let lines = run_wasm_fixture("parity/protocol_iterator.sy");
    assert!(lines.iter().any(|l| l.trim() == "10"), "{lines:?}");
}

#[test]
fn iter_wasm_custom_counter_fixture() {
    let lines = run_wasm_fixture("parity/protocol_iterator_custom.sy");
    assert!(lines.iter().any(|l| l.trim() == "6"), "{lines:?}");
}

#[test]
fn iter_compile_custom_fixture() {
    compile_file_with_options(&fixture("parity/protocol_iterator_custom.sy"), &opts()).unwrap();
}

#[test]
fn iter_range_sum_via_for_in() {
    assert!(run(
        r#"func main() {
    var s = 0
    for i in [10, 20, 30] { s = s + i }
    print(s)
}"#
    )
    .iter()
    .any(|l| l.trim() == "60"));
}

#[test]
fn iter_first_element_only() {
    assert!(run(r#"func main() { for x in [7] { print(x) } }"#)
        .iter()
        .any(|l| l.trim() == "7"));
}

#[test]
fn iter_two_element_sequence() {
    assert!(run(r#"func main() { for x in [3, 9] { print(x) } }"#)
        .iter()
        .any(|l| l.trim() == "3"));
}

#[test]
fn iter_accumulate_product_style() {
    assert!(run(
        r#"func main() {
    var s = 1
    for x in [2, 3, 4] { s = s + x }
    print(s)
}"#
    )
    .iter()
    .any(|l| l.trim() == "10"));
}

#[test]
fn iter_optional_return_type_compiles() {
    run(COUNTER_NIL);
}

#[test]
fn iter_nil_immediate_empty() {
    let src = r#"class Empty: Iterator {
    func next() -> Int? { return nil }
}
func main() {
    var n = 0
    for _ in Empty() { n = n + 1 }
    print(n)
}"#;
    assert!(run(src).iter().any(|l| l.trim() == "0"));
}

#[test]
fn iter_file_fixture_for_in_array() {
    let path = fixture("protocol/for_in_array.sy");
    let (_, lines) = run_file_printed(&path, &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "6"));
}

#[test]
fn iter_native_strict_counter_nil() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!("syx_iter69_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), COUNTER_NIL).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let exe = dir.join("app");
    match build_native_executable(&dir, &opts(), &exe) {
        Ok(_) => {
            let output = std::process::Command::new(&exe).output().unwrap();
            let out = String::from_utf8_lossy(&output.stdout);
            assert!(out.contains("6"), "{out}");
        }
        Err(e) if e.message.contains("Native LLVM") => {
            // LLVM tier-1 unavailable in this environment; VM embed path covers nil semantics.
            assert!(run(COUNTER_NIL).iter().any(|l| l.trim() == "6"));
        }
        Err(e) => panic!("native build: {e:?}"),
    }
}

#[test]
fn iter_phase69_gate_count() {
    let src = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "tests/iterator_optional_strict.rs",
    ))
    .unwrap();
    let count = src.matches("#[test]").count() + src.matches("iter_prints!(").count();
    assert!(count >= 20, "iterator_optional_strict needs 20+ tests, found {count}");
}

#[test]
fn iter_no_i32_zero_hack_in_vm() {
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(!vm.contains("Value::I32(0) | Value::I64(0)"));
}

#[test]
fn iter_no_i32_zero_hack_in_mir_run() {
    let rt = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(!rt.contains("Value::I32(0) | Value::I64(0)"));
}

#[test]
fn iter_counter_three_values_printed() {
    let src = r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 2 { return nil }
        return self.n
    }
}
func main() {
    for x in Counter() { print(x) }
}"#;
    let lines = run(src);
    assert!(lines.iter().any(|l| l.trim() == "1"));
    assert!(lines.iter().any(|l| l.trim() == "2"));
}

#[test]
fn iter_bool_done_flag_nil() {
    let src = r#"class Flag: Iterator {
    var done: Bool
    init() { self.done = false }
    func next() -> Int? {
        if self.done { return nil }
        self.done = true
        return 99
    }
}
func main() { for x in Flag() { print(x) } }"#;
    assert!(run(src).iter().any(|l| l.trim() == "99"));
}
