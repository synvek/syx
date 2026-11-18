//! U-33…U-38: Bool / String / Int (+ unary Neg) / Double (+ unary Neg) /
//! Integer→Double / inherited class field-default pad on empty memberwise / init.
//! ≠ Call / codegen map / deep `-(1)` / `-(1.5)` peel / extension float /
//! Group peel / true implicit CallSuper.

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u33_struct_bool_string_int_defaults_run() {
    let path = write_temp(
        r#"
struct S {
    var b: Bool = true
    var s: String = "hi"
    var n: Int = 7
}
public func main() {
    let x = S()
    print(x.b)
    print(x.s)
    print(x.n)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("struct defaults must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["true", "hi", "7"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u33_class_empty_init_applies_defaults() {
    let path = write_temp(
        r#"
class Animal {
    var name: String = "anon"
    var ok: Bool = false
    var age: Int = 7
    init() {}
}
public func main() {
    let a = Animal()
    print(a.name)
    print(a.ok)
    print(a.age)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("class empty init defaults must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["anon", "false", "7"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u33_explicit_label_overrides_default() {
    let path = write_temp(
        r#"
struct Box {
    var n: Int = 1
}
public func main() {
    let x = Box(n: 3)
    print(x.n)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("explicit label must override default");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["3"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u33_missing_default_still_nil() {
    let path = write_temp(
        r#"
struct Hole {
    var s: String
}
public func main() {
    let x = Hole()
    print(x.s)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("missing default must still be nil");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["nil"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u33_counter_int_default_still_runs() {
    let path = write_temp(
        r#"
struct Counter {
    var value: Int = 0
    mutating func increment() {
        self.value = self.value + 1
    }
}
public func main() {
    var c = Counter()
    c.increment()
    print(c.value)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("Counter Int default must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u34_struct_unary_neg_int_default_runs() {
    let path = write_temp(
        r#"
struct S {
    var n: Int = -1
}
public func main() {
    let x = S()
    print(x.n)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("struct unary -1 default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["-1"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u34_class_unary_neg_int_default_runs() {
    let path = write_temp(
        r#"
class C {
    var n: Int = -1
    init() {}
}
public func main() {
    let c = C()
    print(c.n)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("class unary -1 default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["-1"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u35_struct_double_default_runs() {
    let path = write_temp(
        r#"
struct S {
    var x: Double = 1.5
}
public func main() {
    let s = S()
    print(s.x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("struct Double default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["1.5"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u35_class_double_default_runs() {
    let path = write_temp(
        r#"
class C {
    var x: Double = 1.5
    init() {}
}
public func main() {
    let c = C()
    print(c.x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("class Double default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["1.5"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u36_struct_unary_neg_double_default_runs() {
    let path = write_temp(
        r#"
struct S {
    var x: Double = -1.5
}
public func main() {
    let s = S()
    print(s.x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("struct unary -1.5 default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["-1.5"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u36_class_unary_neg_double_default_runs() {
    let path = write_temp(
        r#"
class C {
    var x: Double = -1.5
    init() {}
}
public func main() {
    let c = C()
    print(c.x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("class unary -1.5 default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["-1.5"]
    );
    let _ = std::fs::remove_file(&path);
}

/// `/ 2` distinguishes I32 (→0) from F64 (→0.5); bare `print` / `+ 0.5` do not.
#[test]
fn u37_struct_int_to_double_default_runs() {
    let path = write_temp(
        r#"
struct S {
    var x: Double = 1
}
public func main() {
    let s = S()
    print(s.x / 2)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("struct Int→Double default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["0.5"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u37_class_int_to_double_default_runs() {
    let path = write_temp(
        r#"
class C {
    var x: Double = 1
    init() {}
}
public func main() {
    let c = C()
    print(c.x / 2)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("class Int→Double default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["0.5"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u37_struct_neg_int_to_double_default_runs() {
    let path = write_temp(
        r#"
struct S {
    var x: Double = -1
}
public func main() {
    let s = S()
    print(s.x / 2)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("struct Neg Int→Double default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["-0.5"]
    );
    let _ = std::fs::remove_file(&path);
}

/// U-38: inherited stored literal via ancestor walk in lower_class_init; ≠ CallSuper.
#[test]
fn u38_inherited_field_default_on_subclass_init() {
    let path = write_temp(
        r#"
class Animal {
    var name: String = "anon"
}
class Dog: Animal {
    var breed: String
    init(breed: String) {
        self.breed = breed
    }
}
public func main() {
    let d = Dog(breed: "lab")
    print(d.name)
    print(d.breed)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("inherited field default must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["anon", "lab"]
    );
    let _ = std::fs::remove_file(&path);
}
