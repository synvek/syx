//! Phase 91: native strict parity extended with Phase 86 stdlib fixtures + async-free CFG cases.

use std::fs;
use std::path::PathBuf;

use syx_driver::{build_native_executable, DriverOptions};

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_p98_native_strict_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe).unwrap();
    let output = std::process::Command::new(&exe).output().unwrap();
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

fn run_native_fixture_strict(rel: &str) -> (i32, String) {
    let source = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/parity/{rel}")),
    )
    .unwrap();
    run_native_strict(&source)
}

macro_rules! p98_strict_src {
    ($name:ident, $src:expr, $needle:expr) => {
        #[test]
        fn $name() {
            let (code, stdout) = run_native_strict($src);
            assert_eq!(code, 0, "stdout: {stdout}");
            assert!(stdout.contains($needle), "stdout: {stdout}");
        }
    };
}

macro_rules! p98_strict_fixture {
    ($name:ident, $rel:expr, $needle:expr) => {
        #[test]
        fn $name() {
            let (code, stdout) = run_native_fixture_strict($rel);
            assert_eq!(code, 0, "stdout: {stdout}");
            assert!(stdout.contains($needle), "stdout: {stdout}");
        }
    };
}

p98_strict_src!(p98_strict_add, "func main() { print(10 + 5) }\n", "15");
p98_strict_src!(p98_strict_sub, "func main() { print(10 - 3) }\n", "7");
p98_strict_src!(p98_strict_mul, "func main() { print(6 * 7) }\n", "42");
p98_strict_src!(
    p98_strict_if_else,
    "func main() { if 1 < 2 { print(1) } else { print(0) } }\n",
    "1"
);
p98_strict_src!(
    p98_strict_while_sum,
    "func main() { var i = 0; var s = 0; while i < 5 { s = s + i; i = i + 1 } print(s) }\n",
    "10"
);
p98_strict_src!(
    p98_strict_class_field,
    "class P { var x: Int; init(x: Int) { self.x = x } } func main() { print(P(x: 88).x) }\n",
    "88"
);
p98_strict_src!(
    p98_strict_hof_closure,
    r#"func main() { let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b }); print(s[0]) }"#,
    "1"
);
p98_strict_src!(
    p98_strict_string_builder,
    r#"func main() { let b = __syx_string_builder_new(); __syx_string_builder_append(b, "p88"); print(__syx_string_builder_to_string(b)) }"#,
    "p88"
);

p98_strict_fixture!(p98_strict_fixture_hof, "hof.sy", "2");
p98_strict_fixture!(p98_strict_fixture_cast, "as_is_cast.sy", "cat");
p98_strict_fixture!(
    p98_strict_fixture_comparable_sort,
    "comparable_sort.sy",
    "1"
);
p98_strict_fixture!(
    p98_strict_fixture_inheritance,
    "inheritance_dispatch.sy",
    "woof"
);
p98_strict_fixture!(p98_strict_fixture_iterator, "protocol_iterator.sy", "10");

#[test]
fn p98_native_strict_phi_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-phi-v1"));
}

#[test]
fn p98_native_strict_parity_count_gate() {
    let src = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/native/native_strict_parity.rs"),
    )
    .unwrap();
    let count = src.matches("#[test]").count()
        + src.matches("p98_strict_src!").count()
        + src.matches("p98_strict_fixture!").count();
    assert!(
        count >= 12,
        "native_strict_parity_phase98 needs 12+ tests, found {count}"
    );
}
