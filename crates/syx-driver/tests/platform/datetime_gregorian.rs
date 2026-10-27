//! Gregorian / RFC3339 DateTime MVP behavioral gates.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn datetime_format_epoch_day_is_rfc3339() {
    // Wave A W19 / ADR-363 / A-03: format product exact (not contains soft).
    let out = run(r#"func main() { print(__syx_datetime_format(86400000, "iso")) }"#);
    assert_eq!(
        out, "1970-01-02T00:00:00.000Z",
        "datetime format epoch day must exact: {out}"
    );
}

#[test]
fn datetime_parse_roundtrip() {
    // Wave A W18 / ADR-362 / A-03: parse→ms→format product exact (not contains soft).
    let out = run(
        r#"func main() {
            let o = __syx_datetime_parse_iso8601("1970-01-02T00:00:00.000Z")
            print(__syx_keypath_get(o, "DateTime", "ms", false))
            print(__syx_datetime_format(86400000, "iso"))
        }"#,
    );
    assert_eq!(
        out, "86400000\n1970-01-02T00:00:00.000Z",
        "datetime parse roundtrip must exact: {out}"
    );
}

#[test]
fn datetime_leap_day() {
    let out = run(
        r#"func main() {
            let o = __syx_datetime_parse("2020-02-29")
            print(__syx_datetime_format(__syx_keypath_get(o, "DateTime", "ms", false), "%Y-%m-%d"))
        }"#,
    );
    assert_eq!(out.trim(), "2020-02-29");
}

#[test]
fn datetime_timezone_still_unsupported() {
    let err = compile_and_run_embedded(
        r#"func main() { print(__syx_datetime_timezone_offset("America/New_York")) }"#,
        &DriverOptions::default(),
    );
    assert!(err.is_err(), "timezoneOffset must fail closed");
}
