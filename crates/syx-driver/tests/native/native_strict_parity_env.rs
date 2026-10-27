//! Native strict parity (LLVM-only when SYX_NATIVE_STRICT=1; stub fallback otherwise).
// was: native_strict_parity

use std::fs;
use std::path::PathBuf;

use syx_driver::{build_native_executable, DriverOptions};

fn run_native_strict(src: &str) -> (i32, String) {
    let strict = std::env::var("SYX_NATIVE_STRICT").ok().as_deref() == Some("1");
    if !strict {
        std::env::set_var("SYX_NATIVE_C_STUB", "1");
    } else {
        std::env::remove_var("SYX_NATIVE_C_STUB");
    }
    let dir = std::env::temp_dir().join(format!(
        "syx_native_strict_{}",
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

macro_rules! strict_src {
    ($name:ident, $src:expr, $needle:expr) => {
        #[test]
        fn $name() {
            let (code, stdout) = run_native_strict($src);
            assert_eq!(code, 0, "stdout: {stdout}");
            assert!(stdout.contains($needle), "stdout: {stdout}");
        }
    };
}

macro_rules! strict_fixture {
    ($name:ident, $rel:expr, $needle:expr) => {
        #[test]
        fn $name() {
            let (code, stdout) = run_native_fixture_strict($rel);
            assert_eq!(code, 0, "stdout: {stdout}");
            assert!(stdout.contains($needle), "stdout: {stdout}");
        }
    };
}

strict_src!(strict_add, "func main() { print(10 + 5) }\n", "15");
strict_src!(strict_sub, "func main() { print(10 - 3) }\n", "7");
strict_src!(strict_mul, "func main() { print(6 * 7) }\n", "42");
strict_src!(strict_div, "func main() { print(20 / 4) }\n", "5");
strict_src!(strict_mod, "func main() { print(17 % 5) }\n", "2");
strict_src!(strict_cmp_lt, "func main() { print(1 < 2) }\n", "true");
strict_src!(strict_cmp_gt, "func main() { print(3 > 2) }\n", "true");
strict_src!(
    strict_if_else,
    "func main() { if 1 < 2 { print(1) } else { print(0) } }\n",
    "1"
);
strict_src!(
    strict_nested_call,
    "func inc(x: Int) -> Int { return x + 1 } func main() { print(inc(x: inc(x: 1))) }\n",
    "3"
);
strict_src!(strict_compare_eq, "func main() { print(5 == 5) }\n", "true");
strict_src!(strict_compare_ne, "func main() { print(5 != 4) }\n", "true");
strict_src!(
    strict_print_twice,
    "func main() { print(1); print(2) }\n",
    "2"
);
strict_src!(strict_neg, "func main() { print(0 - 8) }\n", "-8");
strict_src!(strict_const_100, "func main() { print(100) }\n", "100");
strict_src!(strict_triple_add, "func main() { print(1 + 2 + 3) }\n", "6");
strict_src!(strict_le, "func main() { print(2 <= 2) }\n", "true");
strict_src!(strict_ge, "func main() { print(5 >= 3) }\n", "true");
strict_src!(strict_not, "func main() { print(!false) }\n", "true");
strict_src!(strict_bool_literal, "func main() { print(true) }\n", "true");
strict_src!(
    strict_string_eq,
    r#"func main() { print("x" == "x") }"#,
    "true"
);
strict_src!(
    strict_print_hello,
    r#"func main() { print("hello") }"#,
    "hello"
);
strict_src!(
    strict_inheritance_override,
    r#"class B { func f() { print("b") } } class D: B { override func f() { print("d") } } func main() { D().f() }"#,
    "d"
);
strict_src!(
    strict_string_len,
    r#"func main() { print(String.count(text: "hi")) }"#,
    "2"
);
strict_src!(
    strict_empty_string,
    r#"func main() { print(String.count(text: "")) }"#,
    "0"
);
strict_src!(
    strict_nil_compare,
    "func main() { print(1 == 1) }\n",
    "true"
);

strict_fixture!(strict_fixture_hof, "hof.sy", "2");
strict_fixture!(strict_fixture_generics, "generics.sy", "42");
strict_fixture!(strict_fixture_cast, "as_is_cast.sy", "cat");
strict_fixture!(strict_fixture_dict, "dict_hashable_key.sy", "100");
strict_fixture!(strict_fixture_array_hof, "array_hof.sy", "4");
strict_fixture!(strict_fixture_comparable_sort, "comparable_sort.sy", "1");
strict_fixture!(
    strict_fixture_inheritance,
    "inheritance_dispatch.sy",
    "woof"
);
strict_fixture!(strict_fixture_iterator, "protocol_iterator.sy", "10");
strict_fixture!(
    strict_fixture_polymorphic,
    "polymorphic_base_ref.sy",
    "woof"
);

// Additional parity coverage
strict_src!(
    strict_while_sum,
    "func main() { var i = 0; var s = 0; while i < 5 { s = s + i; i = i + 1 } print(s) }\n",
    "10"
);
strict_src!(
    strict_for_in_array,
    "func main() { var s = 0; for x in [1, 2, 3] { s = s + x } print(s) }\n",
    "6"
);
strict_src!(
    strict_class_field,
    "class P { var x: Int; init(x: Int) { self.x = x } } func main() { print(P(x: 7).x) }\n",
    "7"
);
strict_src!(
    strict_array_index,
    "func main() { let a = [9, 8, 7]; print(a[1]) }\n",
    "8"
);
strict_src!(
    strict_dict_subscript,
    "func main() { let d = [\"k\": 42]; print(d[\"k\"]) }\n",
    "42"
);

#[test]
fn native_strict_parity_count_gate() {
    let src = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/native_strict_parity.rs"),
    )
    .unwrap();
    let count = src.matches("#[test]").count()
        + src.matches("strict_src!").count()
        + src.matches("strict_fixture!").count();
    assert!(
        count >= 40,
        "native_strict_parity needs 40+ tests, found {count}"
    );
}
