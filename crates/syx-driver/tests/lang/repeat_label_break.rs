//! U-11: `outer: repeat { break outer }` keeps label on MIR loop_stack.
//! Bounded probes only — body must break (no unbounded loop).

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u11_labeled_repeat_break_outer_runs() {
    let path = write_temp(
        r#"
public func main() {
    outer: repeat {
        print(1)
        break outer
    } while true
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("labeled repeat break outer must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u11_unlabeled_repeat_break_still_runs() {
    let path = write_temp(
        r#"
public func main() {
    repeat {
        break
    } while false
    print(1)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("unlabeled repeat break must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u11_labeled_while_break_still_runs() {
    let path = write_temp(
        r#"
public func main() {
    outer: while true {
        print(2)
        break outer
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("labeled while break must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["2"]);
    let _ = std::fs::remove_file(&path);
}
