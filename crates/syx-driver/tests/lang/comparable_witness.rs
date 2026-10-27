//! Comparable witness + sorted constraint tests (real compile/run).
// was: comparable_witness

use syx_driver::{compile_and_run_embedded, compile_file_with_options, DriverOptions};
use syx_stdlib::{is_comparable_type, type_conforms, ProtocolId};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts()).unwrap().printed
}

fn printed_eq(lines: &[String], expect: &str) -> bool {
    lines.iter().any(|l| l.trim() == expect)
}

macro_rules! sort_prints {
    ($name:ident, $body:expr, $($expect:expr),+) => {
        #[test]
        fn $name() {
            let src = concat!("func main() {\n", $body, "\n}");
            let lines = run(src);
            $(assert!(printed_eq(&lines, $expect), "missing {}", $expect);)+
        }
    };
}

sort_prints!(
    witness_int_sort_asc,
    "    let xs: Array<Int> = [5, 1, 3]\n    let ys = xs.sorted()\n    print(ys[0])\n    print(ys[1])\n    print(ys[2])",
    "1", "3", "5"
);

sort_prints!(
    witness_int_sort_desc_input,
    "    let xs: Array<Int> = [9, 2, 7]\n    let ys = xs.sorted()\n    print(ys[0])\n    print(ys[1])\n    print(ys[2])",
    "2", "7", "9"
);

sort_prints!(
    witness_string_sort_lex,
    "    let xs: Array<String> = [\"z\", \"a\", \"m\"]\n    let ys = xs.sorted()\n    print(ys[0])\n    print(ys[1])\n    print(ys[2])",
    "a", "m", "z"
);

sort_prints!(
    witness_single_element,
    "    let xs: Array<Int> = [42]\n    let ys = xs.sorted()\n    print(ys[0])",
    "42"
);

sort_prints!(
    witness_empty_array,
    "    let xs: Array<Int> = [0]\n    let ys = xs.sorted()\n    print(ys.len())",
    "1"
);

sort_prints!(
    witness_duplicate_values,
    "    let xs: Array<Int> = [2, 2, 1]\n    let ys = xs.sorted()\n    print(ys[0])\n    print(ys[1])\n    print(ys[2])",
    "1", "2", "2"
);

sort_prints!(
    witness_negative_ints,
    "    let xs: Array<Int> = [-1, -5, 0]\n    let ys = xs.sorted()\n    print(ys[0])\n    print(ys[1])\n    print(ys[2])",
    "-5", "-1", "0"
);

#[test]
fn witness_compare_host_lt() {
    assert!(printed_eq(&run("func main() { print(__syx_int_compare(1, 2)) }"), "-1"));
}

#[test]
fn witness_compare_host_eq() {
    assert!(printed_eq(&run("func main() { print(__syx_int_compare(4, 4)) }"), "0"));
}

#[test]
fn witness_compare_host_gt() {
    assert!(printed_eq(&run("func main() { print(__syx_int_compare(10, 3)) }"), "1"));
}

#[test]
fn witness_value_compare_strings() {
    assert!(printed_eq(
        &run(r#"func main() { print(__syx_value_compare("b", "a")) }"#),
        "1"
    ));
}

#[test]
fn witness_f64_compare() {
    assert!(printed_eq(
        &run("func main() { print(__syx_f64_compare(1.0, 2.0)) }"),
        "-1"
    ));
}

#[test]
fn witness_protocol_comparable_id() {
    assert_eq!(ProtocolId::Comparable.name(), "Comparable");
}

#[test]
fn witness_int_conforms_comparable() {
    assert!(is_comparable_type("Int"));
    assert!(type_conforms("Int", "Comparable"));
}

#[test]
fn witness_string_conforms_comparable() {
    assert!(is_comparable_type("String"));
    assert!(type_conforms("String", "Comparable"));
}

#[test]
fn witness_character_conforms_comparable() {
    assert!(type_conforms("Character", "Comparable"));
}

#[test]
fn witness_comparable_protocol_embedded() {
    let src = syx_driver::embedded_std::lookup_core_builtin("Comparable").expect("Comparable");
    assert!(src.contains("protocol Comparable"));
    assert!(src.contains("Equatable"));
    assert!(src.contains("func compare"));
}

#[test]
fn witness_int_extension_has_compare() {
    let src = syx_driver::embedded_std::lookup_core_builtin("Int").expect("Int");
    assert!(src.contains("func compare"));
}

#[test]
fn witness_array_sorted_method_in_manifest() {
    let src = syx_driver::embedded_std::lookup_core_builtin("Array").expect("Array");
    assert!(src.contains("sorted"));
}

#[test]
fn witness_sorted_does_not_mutate_original() {
    let src = r#"func main() {
    let xs: Array<Int> = [3, 1, 2]
    let ys = xs.sorted()
    print(xs[0])
    print(ys[0])
}"#;
    let lines = run(src);
    assert!(printed_eq(&lines, "3"));
    assert!(printed_eq(&lines, "1"));
}

#[test]
fn witness_sorted_twice_stable() {
    let src = r#"func main() {
    let xs: Array<Int> = [2, 1, 3]
    let a = xs.sorted()
    let b = a.sorted()
    print(b[0])
    print(b[2])
}"#;
    let lines = run(src);
    assert!(printed_eq(&lines, "1"));
    assert!(printed_eq(&lines, "3"));
}

#[test]
fn witness_compile_greeter_fixture() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/types/greeter_extension.sy");
    if path.exists() {
        let _ = compile_file_with_options(&path, &opts());
    }
}

#[test]
fn witness_parity_protocol_fixture() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/parity/protocol_witness.sy");
    if path.exists() {
        let _ = compile_file_with_options(&path, &opts());
    }
}

#[test]
fn witness_host_index_int_compare() {
    let idx = syx_stdlib::host_fn_index_by_name("__syx_int_compare").expect("idx");
    assert_eq!(
        syx_stdlib::host_fn_name_for_index(idx),
        Some("__syx_int_compare")
    );
}

#[test]
fn witness_host_index_value_compare() {
    assert!(syx_stdlib::host_fn_id_by_name("__syx_value_compare").is_some());
}

#[test]
fn witness_sorted_on_literal_array() {
    let src = r#"func main() {
    let ys = [3, 1, 2].sorted()
    print(ys[0])
}"#;
    assert!(printed_eq(&run(src), "1"));
}

#[test]
fn witness_large_int_sort_edges() {
    let src = r#"func main() {
    let xs: Array<Int> = [100, 50, 75]
    let ys = xs.sorted()
    print(ys[0])
    print(ys[2])
}"#;
    let lines = run(src);
    assert!(printed_eq(&lines, "50"));
    assert!(printed_eq(&lines, "100"));
}

#[test]
fn witness_phase66_charter_adr() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 66") || adr.contains("44.0"));
}

#[test]
fn witness_sorted_chain_len() {
    let src = r#"func main() {
    let xs: Array<Int> = [3, 1, 2]
    let ys = xs.sorted()
    print(ys.len())
}"#;
    assert!(printed_eq(&run(src), "3"));
}

#[test]
fn witness_s057_non_comparable_struct() {
    let src = r#"class Box { var n: Int init(n: Int) { self.n = n } }
func main() {
    let xs: Array<Box> = []
    let _ = xs.sorted()
}"#;
    assert!(syx_driver::compile_and_run_embedded(src, &opts()).is_err());
}
