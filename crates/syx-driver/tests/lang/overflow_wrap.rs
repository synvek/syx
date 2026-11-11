//! S1-I25: default VM Int add wraps (S1-07 scoped wrap SSOT); must not trap.

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

#[test]
fn s1_i25_vm_int_max_plus_one_wraps() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/lang/int_add_wrap.sy");
    let (code, lines) = run_file_printed(&path, &DriverOptions::default())
        .expect("VM run must succeed (wrap, not trap)");
    assert_eq!(code, 0, "exit {code}: {lines:?}");
    let out = lines
        .iter()
        .map(|l| l.trim_end_matches(['\r', '\n']))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        out, "-9223372036854775808",
        "Int max+1 must wrap to Int.min on default VM; got {out:?}"
    );
}
