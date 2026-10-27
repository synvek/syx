//! Comparable protocol + sorted witness tests .
// was: comparable

use syx_driver::{
    compile_and_run_embedded, compile_file_with_options, run_file_printed, DriverOptions,
};
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;
use syx_stdlib::{is_comparable_type, lookup_numeric_host, ProtocolId, type_conforms};

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

fn has_error(src: &str, code: &str) -> bool {
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = Parser::parse(file).unwrap();
    match SemanticAnalyzer::analyze(&module) {
        Err(d) => d.code.as_deref() == Some(code),
        Ok(a) => a.errors.iter().any(|e| e.code.as_deref() == Some(code)),
    }
}

#[test]
fn cache_v38_tag_in_driver() {
    let src = include_str!("../../src/lib.rs");
    assert!(src.contains("syx-cache-v38"));
}

#[test]
fn manifest_phase66() {
    let m: serde_json::Value =
        serde_json::from_str(syx_driver::embedded_std::internal_manifest_json()).unwrap();
    assert_eq!(m["phase"], 66);
    assert_eq!(m["version"], "44.0");
}

#[test]
fn protocol_id_comparable() {
    assert_eq!(ProtocolId::Comparable.name(), "Comparable");
    assert_eq!(ProtocolId::from_name("Comparable"), Some(ProtocolId::Comparable));
}

#[test]
fn int_is_comparable_via_manifest() {
    assert!(is_comparable_type("Int"));
    assert!(type_conforms("Int", "Comparable"));
}

#[test]
fn string_is_comparable_via_manifest() {
    assert!(is_comparable_type("String"));
    assert!(type_conforms("Character", "Comparable"));
}

#[test]
fn int_compare_host_registered() {
    assert!(lookup_numeric_host("__syx_int_compare").is_some());
}

#[test]
fn value_compare_host_registered() {
    assert!(syx_stdlib::host_fn_id_by_name("__syx_value_compare").is_some());
}

#[test]
fn host_index_i64_compare() {
    let idx = syx_stdlib::host_fn_index_by_name("__syx_i64_compare").expect("idx");
    assert_eq!(
        syx_stdlib::host_fn_name_for_index(idx),
        Some("__syx_i64_compare")
    );
}

#[test]
fn int_compare_less_via_host() {
    let src = r#"
func main() {
    print(__syx_int_compare(1, 3))
}
"#;
    assert!(printed_eq(&run(src), "-1"));
}

#[test]
fn int_compare_equal_via_host() {
    let src = r#"
func main() {
    print(__syx_int_compare(5, 5))
}
"#;
    assert!(printed_eq(&run(src), "0"));
}

#[test]
fn int_compare_greater_via_host() {
    let src = r#"
func main() {
    print(__syx_int_compare(9, 2))
}
"#;
    assert!(printed_eq(&run(src), "1"));
}

#[test]
fn string_lex_compare_via_host() {
    let src = r#"
func main() {
    print(__syx_value_compare("apple", "banana"))
}
"#;
    assert!(printed_eq(&run(src), "-1"));
}

#[test]
fn array_sorted_ints() {
    let src = r#"
func main() {
    let xs = [3, 1, 2]
    let ys = xs.sorted()
    print(ys[0])
    print(ys[1])
    print(ys[2])
}
"#;
    let lines = run(src);
    assert!(printed_eq(&lines, "1"));
    assert!(printed_eq(&lines, "2"));
    assert!(printed_eq(&lines, "3"));
}

#[test]
fn array_sorted_strings() {
    let src = r#"
func main() {
    let xs = ["c", "a", "b"]
    let ys = xs.sorted()
    print(ys[0])
    print(ys[1])
    print(ys[2])
}
"#;
    let lines = run(src);
    assert!(printed_eq(&lines, "a"));
    assert!(printed_eq(&lines, "b"));
    assert!(printed_eq(&lines, "c"));
}

#[test]
fn float_compare_via_host() {
    let src = r#"
func main() {
    print(__syx_f64_compare(1.5, 2.5))
}
"#;
    assert!(printed_eq(&run(src), "-1"));
}

#[test]
fn comparable_protocol_source_embedded() {
    let src = syx_driver::embedded_std::lookup_core_builtin("Comparable").expect("Comparable");
    assert!(src.contains("protocol Comparable"));
    assert!(src.contains("Equatable"));
    assert!(src.contains("func compare"));
    assert!(src.contains("Self"));
}

#[test]
fn int_extension_declares_compare() {
    let src = syx_driver::embedded_std::lookup_core_builtin("Int").expect("Int");
    assert!(src.contains("func compare"));
    assert!(src.contains("__syx_int_compare"));
}

#[test]
fn comparable_compile_smoke() {
    compile_file_with_options(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/types/greeter_extension.sy"),
        &opts(),
    )
    .expect("compile fixture");
}

#[test]
fn sorted_non_comparable_element_s057() {
    assert!(has_error(
        r#"
class Box {
    var label: String
    init(label: String) { self.label = label }
}
func main() {
    let xs = [Box(label: "a")]
    let ys = xs.sorted()
}
"#,
        "S057"
    ));
}

#[test]
fn struct_with_compare_witness_sorted() {
    let src = r#"
class Score: Comparable {
    var n: Int
    init(n: Int) { self.n = n }
    func compare(other: Score) -> Int {
        return self.n - other.n
    }
}
func main() {
    let xs = [Score(n: 3), Score(n: 1), Score(n: 2)]
    let ys = xs.sorted()
    print(ys[0].n)
    print(ys[1].n)
    print(ys[2].n)
}
"#;
    let lines = run(src);
    assert!(printed_eq(&lines, "1"));
    assert!(printed_eq(&lines, "2"));
    assert!(printed_eq(&lines, "3"));
}

#[test]
fn parity_comparable_sort_fixture() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/parity/comparable_sort.sy");
    let (_, lines) = run_file_printed(&path, &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "1"));
    assert!(lines.iter().any(|l| l.trim() == "2"));
    assert!(lines.iter().any(|l| l.trim() == "3"));
}

#[test]
fn sorted_single_int() {
    let src = r#"
func main() {
    let xs = [42]
    let ys = xs.sorted()
    print(ys[0])
}
"#;
    assert!(printed_eq(&run(src), "42"));
}

#[test]
fn sorted_descending_ints_reversed() {
    let src = r#"
func main() {
    let xs = [5, 4, 3, 2, 1]
    let ys = xs.sorted()
    print(ys[0])
    print(ys[4])
}
"#;
    let lines = run(src);
    assert!(printed_eq(&lines, "1"));
    assert!(printed_eq(&lines, "5"));
}

#[test]
fn bigint_compare_host_smoke() {
    let src = r#"
func main() {
    print(__syx_value_compare("1", "2"))
}
"#;
    assert!(printed_eq(&run(src), "-1"));
}

#[test]
fn character_is_comparable() {
    assert!(is_comparable_type("Character"));
}

#[test]
fn float64_is_comparable() {
    assert!(is_comparable_type("Float64"));
}

macro_rules! int_sorted_case {
    ($name:ident, $input:expr, $first:expr, $last:expr) => {
        #[test]
        fn $name() {
            let input = $input;
            let last_idx = input.split(',').count() - 1;
            let src = format!(
                "func main() {{\n    let ys = [{input}].sorted()\n    print(ys[0])\n    print(ys[{last_idx}])\n}}"
            );
            let lines = run(&src);
            assert!(printed_eq(&lines, $first));
            assert!(printed_eq(&lines, $last));
        }
    };
}

int_sorted_case!(sorted_pair_2_1, "2, 1", "1", "2");
int_sorted_case!(sorted_triple_9_3_6, "9, 3, 6", "3", "9");

#[test]
fn sorted_strings_lex_order() {
    let src = r#"
func main() {
    let ys = ["zebra", "apple", "mango"].sorted()
    print(ys[0])
    print(ys[2])
}
"#;
    let lines = run(src);
    assert!(printed_eq(&lines, "apple"));
    assert!(printed_eq(&lines, "zebra"));
}

#[test]
fn array_sorted_does_not_mutate_original() {
    let src = r#"
func main() {
    let xs = [3, 1, 2]
    let _ = xs.sorted()
    print(xs[0])
}
"#;
    assert!(printed_eq(&run(src), "3"));
}

#[test]
fn nested_sorted_arrays() {
    let src = r#"
func main() {
    let a = [2, 1].sorted()
    let b = [4, 3].sorted()
    print(a[0] + b[0])
}
"#;
    assert!(printed_eq(&run(src), "4"));
}
