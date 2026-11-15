//! Array index read/write check Int index and element type (facade stays Object).

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

/// Build + run native AOT. Returns `None` when the toolchain lacks `--features full-native` (L006).
fn try_run_native_exe(label: &str, src: &str) -> Option<(i32, String, String)> {
    syx_host_plugin::register_echo_plugin_in_process();
    if !cfg!(feature = "full-native") {
        eprintln!("skip native AOT ({label}): toolchain built without --features full-native");
        return None;
    }
    let dir = std::env::temp_dir().join(format!("syx_arr_oob_{}_{}", label, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let exe = dir.join("app");
    match build_native_executable(&dir, &opts(), &exe) {
        Ok(_) => {}
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("L006") || msg.contains("full-native") {
                eprintln!("skip native AOT ({label}): {msg}");
                return None;
            }
            panic!("native build failed: {e}");
        }
    }
    let output = Command::new(&exe).output().expect("run native exe");
    Some((
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    ))
}

fn compile_fail_code(src: &str) -> Option<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = match syx_parser::Parser::parse(file) {
        Ok(m) => m,
        Err(d) => return d.code.clone(),
    };
    match syx_semantic::SemanticAnalyzer::analyze(&module) {
        Err(d) => d.code.clone(),
        Ok(a) => a.errors.first().and_then(|e| e.code.clone()),
    }
}

#[test]
fn array_int_subscript_assign_and_read() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var a: Array<Int> = [1]
    a[0] = 2
    print(a[0])
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "2"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn array_float_subscript_unsuffixed_int_assign() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var a: Array<Float> = [0.0f32]
    a[0] = 2
    print(a[0])
}
"#,
        &opts(),
    )
    .expect("run");
    let printed = result.printed.join("\n");
    assert!(
        printed.lines().any(|l| {
            l.trim()
                .parse::<f64>()
                .ok()
                .is_some_and(|n| (n - 2.0).abs() < 1e-9)
        }),
        "printed={printed:?}"
    );
}

#[test]
fn array_subscript_wrong_value_string_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var a: Array<Float> = [0.0f32]
    a[0] = "x"
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn array_subscript_wrong_value_suffix_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var a: Array<Float> = [0.0f32]
    a[0] = 2i8
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn array_subscript_oob_read_is_r003() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
    var a: Array<Int> = [1]
    print(a[1])
}
"#,
        &opts(),
    )
    .expect_err("oob read");
    assert_eq!(err.code.as_deref(), Some("R003"));
    assert!(
        err.message.contains("out of bounds"),
        "message={}",
        err.message
    );
}

#[test]
fn array_subscript_oob_write_is_r003() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
    var a: Array<Int> = [1]
    a[1] = 9
}
"#,
        &opts(),
    )
    .expect_err("oob write");
    assert_eq!(err.code.as_deref(), Some("R003"));
}

#[test]
fn array_subscript_negative_index_read_is_r003() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
    var a: Array<Int> = [1]
    print(a[-1])
}
"#,
        &opts(),
    )
    .expect_err("negative index read");
    assert_eq!(err.code.as_deref(), Some("R003"));
    assert!(
        err.message.contains("out of bounds"),
        "message={}",
        err.message
    );
}

#[test]
fn array_subscript_negative_index_write_is_r003() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
    var a: Array<Int> = [1]
    a[-1] = 9
}
"#,
        &opts(),
    )
    .expect_err("negative index write");
    assert_eq!(err.code.as_deref(), Some("R003"));
}

#[test]
fn array_subscript_non_int_index_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var a: Array<Int> = [1]
    a["0"] = 1
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn array_subscript_oob_write_native_is_r003() {
    // LLVM AOT must not swallow mir-run ArraySet Err (was unwrap_or(0) → silent continue).
    // Soft-skip without `--features full-native` (honest L006 refuse on default CI).
    let Some((code, _stdout, stderr)) = try_run_native_exe(
        "oob_write",
        r#"
func main() {
    var a: Array<Int> = [1]
    a[1] = 9
    print(a.count)
}
"#,
    ) else {
        return;
    };
    assert_ne!(code, 0, "stderr={stderr}");
    assert!(
        stderr.contains("R003") && stderr.contains("out of bounds"),
        "stderr={stderr}"
    );
}

#[test]
fn array_subscript_oob_read_native_is_r003() {
    let Some((code, _stdout, stderr)) = try_run_native_exe(
        "oob_read",
        r#"
func main() {
    var a: Array<Int> = [1]
    print(a[1])
}
"#,
    ) else {
        return;
    };
    assert_ne!(code, 0, "stderr={stderr}");
    assert!(
        stderr.contains("R003") && stderr.contains("out of bounds"),
        "stderr={stderr}"
    );
}

#[test]
fn s1_i10_keypath_array_generic_and_callable_check() {
    // S1-I10: `\Array<Int>.[0]` / property KeyPath call / mid-path `?.` must check.
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let arr = [1, 2, 3]
    let kp = \Array<Int>.[0]
    print(kp(arr))
    print((\Array<Int>.[0])(arr))
}
"#
        ),
        None,
        "array KeyPath check"
    );
    assert_eq!(
        compile_fail_code(
            r#"
class Person {
    let name: String
    init(name: String) { self.name = name }
}
func main() {
    let p = Person(name: "Alice")
    let nameKp = \Person.name
    print(nameKp(p))
}
"#
        ),
        None,
        "property KeyPath callable check"
    );
    assert_eq!(
        compile_fail_code(
            r#"
class Box {
    let value: Int?
    init(value: Int?) { self.value = value }
}
func main() {
    let b = Box(value: 42)
    let vKp = \Box.value?.description
    print(vKp(b))
}
"#
        ),
        None,
        "optional mid-path KeyPath check"
    );
}
