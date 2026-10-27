//! DateTime HostFn deepen nails (IANA zones + calendar reject).

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => panic!("expected failure, got stdout={:?}", r.printed),
        Err(e) => format!("{e}"),
    }
}

#[test]
fn datetime_iana_asia_tokyo_offset() {
    // Second IANA zone beyond America/New_York: Asia/Tokyo is fixed JST = +540 minutes.
    let out = run(
        r#"
func main() {
  let winter = __syx_datetime_parse_iso8601("2024-01-15T12:00:00.000Z")
  let wMs = __syx_keypath_get(winter, "DateTime", "ms", false)
  print(__syx_datetime_timezone_offset(wMs, "Asia/Tokyo"))
}
"#,
    );
    assert_eq!(
        out.trim(),
        "540",
        "Asia/Tokyo winter offset must exact 540: {out}"
    );
}

#[test]
fn datetime_calendar_reject_2023_feb29() {
    // Gregorian invalid date (non-leap Feb 29) must fail-closed — not silent success.
    let err = run_err(
        r#"
func main() {
  print(__syx_datetime_parse("2023-02-29"))
}
"#,
    );
    assert!(
        err.contains("U002")
            && (err.contains("2023-02-29")
                || err.contains("invalid")
                || err.contains("expected RFC3339")
                || err.contains("YYYY-MM-DD")),
        "2023-02-29 must fail-closed with U002-style diagnostic: {err}"
    );
}

#[test]
fn datetime_iana_format_with_timezone_tokyo() {
    // W354: Tokyo formatWithTimezone complements W353 offset-only nail (NY format already exists).
    let out = run(
        r#"
func main() {
  let winter = __syx_datetime_parse_iso8601("2024-01-15T12:00:00.000Z")
  let wMs = __syx_keypath_get(winter, "DateTime", "ms", false)
  print(__syx_datetime_format_with_timezone(wMs, "iso", "Asia/Tokyo"))
}
"#,
    );
    assert_eq!(
        out.trim(),
        "2024-01-15T21:00:00.000+09:00",
        "Asia/Tokyo winter formatWithTimezone must exact: {out}"
    );
}

#[test]
fn datetime_calendar_reject_2023_apr31() {
    // Gregorian invalid month-day (Apr 31) must fail-closed — distinct from Feb29 nail.
    let err = run_err(
        r#"
func main() {
  print(__syx_datetime_parse("2023-04-31"))
}
"#,
    );
    assert!(
        err.contains("U002")
            && (err.contains("2023-04-31")
                || err.contains("invalid")
                || err.contains("expected RFC3339")
                || err.contains("YYYY-MM-DD")),
        "2023-04-31 must fail-closed with U002-style diagnostic: {err}"
    );
}

#[test]
fn datetime_iana_europe_london_winter_offset() {
    // W355: Europe/London winter (GMT) = 0 minutes — third IANA zone DST pair.
    let out = run(
        r#"
func main() {
  let winter = __syx_datetime_parse_iso8601("2024-01-15T12:00:00.000Z")
  let wMs = __syx_keypath_get(winter, "DateTime", "ms", false)
  print(__syx_datetime_timezone_offset(wMs, "Europe/London"))
}
"#,
    );
    assert_eq!(
        out.trim(),
        "0",
        "Europe/London winter offset must exact 0 (GMT): {out}"
    );
}

#[test]
fn datetime_iana_europe_london_summer_offset() {
    // W355: Europe/London summer (BST) = 60 minutes.
    let out = run(
        r#"
func main() {
  let summer = __syx_datetime_parse_iso8601("2024-07-15T12:00:00.000Z")
  let sMs = __syx_keypath_get(summer, "DateTime", "ms", false)
  print(__syx_datetime_timezone_offset(sMs, "Europe/London"))
}
"#,
    );
    assert_eq!(
        out.trim(),
        "60",
        "Europe/London summer offset must exact 60 (BST): {out}"
    );
}
