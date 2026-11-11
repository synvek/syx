//! S2-I151: write (`format_project`) shares `--check` SSOT — comments preserved on CST subset.

use std::fs;
use std::path::PathBuf;

use syx_driver::{format_check_path, format_project};

const FIXTURE: &str = "// ahead\nfunc main() {\n    // inside\n    print(1)\n}\n/* trail */\n";

#[test]
fn fmt_write_align_preserves_comments_and_matches_check() {
    let dir = tempfile_dir();
    let main = dir.join("main.sy");
    fs::write(&main, FIXTURE).expect("write fixture");
    fs::write(
        dir.join("syx.json"),
        r#"{"name":"i151-fmt","version":"0.0.1","main":"main.sy"}"#,
    )
    .expect("manifest");

    format_project(&dir).expect("format_project must succeed on CST subset");
    let after = fs::read_to_string(&main).expect("read back");
    assert!(
        after.contains("// ahead") && after.contains("// inside") && after.contains("/* trail */"),
        "write must preserve comments; got {after:?}"
    );
    assert!(
        after.contains("    print(1)") || after.contains("print(1)"),
        "body must remain; got {after:?}"
    );

    let report = format_check_path(&main).expect("check after write");
    assert_eq!(report.would_change, 0, "write then --check must be stable");
    assert_eq!(
        report.failed, 0,
        "check must not fail; failures={:?}",
        report.failures
    );
}

fn tempfile_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx-i151-fmt-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}
