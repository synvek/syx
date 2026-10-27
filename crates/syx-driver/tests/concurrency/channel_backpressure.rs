//! Channel backpressure deepen (trySend Full→false + length/capacity).
// was: m3_a04_backpressure

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a04_backpressure_try_send_full_false_and_length() {
    let src = r#"
func main() {
  let ch = __syx_channel_create(2)
  print(__syx_channel_capacity(ch))
  print(__syx_channel_length(ch))
  print(__syx_channel_try_send(ch, "A"))
  print(__syx_channel_try_send(ch, "B"))
  print(__syx_channel_length(ch))
  print(__syx_channel_try_send(ch, "C"))
  print(__syx_channel_try_recv(ch))
  print(__syx_channel_length(ch))
  print(__syx_channel_try_send(ch, "C"))
  print(__syx_channel_length(ch))
}
"#;
    let out = run(src);
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert!(
        lines.len() >= 10,
        "expected capacity/length/trySend/tryRecv sequence: {out}"
    );
    assert_eq!(lines[0], "2", "capacity must be 2: {out}");
    assert_eq!(lines[1], "0", "initial length must be 0: {out}");
    assert!(
        lines[2] == "true" || lines[2] == "True",
        "trySend A must succeed: {out}"
    );
    assert!(
        lines[3] == "true" || lines[3] == "True",
        "trySend B must succeed: {out}"
    );
    assert_eq!(lines[4], "2", "length after fill must be 2: {out}");
    assert!(
        lines[5] == "false" || lines[5] == "False",
        "trySend C when full must return false: {out}"
    );
    assert_eq!(lines[6], "A", "tryRecv must drain A: {out}");
    assert_eq!(lines[7], "1", "length after drain must be 1: {out}");
    assert!(
        lines[8] == "true" || lines[8] == "True",
        "trySend C after drain must succeed: {out}"
    );
    assert_eq!(lines[9], "2", "length after refill must be 2: {out}");
}

#[test]
fn m3_a04_backpressure_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a04_backpressure"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a04_backpressure = \"signed-ADR-267\""),
        "inventory must note signed-ADR-267"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-267")
            || guide.contains("m3_a04_backpressure")
            || guide.contains("backpressure"),
        "UsageGuide must document Channel backpressure deepen"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("ADR-267")
            && (roadmap.contains("backpressure")
                || roadmap.contains("m3_a04_backpressure")
                || roadmap.contains("trySend")),
        "roadmap must record M3 W8 backpressure"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W9")
            && (roadmap.contains("fairness")
                || roadmap.contains("ADR-268")
                || roadmap.contains("A04")
                || roadmap.contains("done")))
            || (roadmap.contains("W10")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-269")
                    || roadmap.contains("A04")
                    || roadmap.contains("done")))
            || (roadmap.contains("W11")
                && (roadmap.contains("A05")
                    || roadmap.contains("docs")
                    || roadmap.contains("ADR-270")
                    || roadmap.contains("done")))
            || (roadmap.contains("W12")
                && (roadmap.contains("closeout") || roadmap.contains("A05"))),
        "roadmap may advance W9/W10 done / W11 = A05"
    );
    let a04 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-A04 |"))
        .expect("M3-A04 row");
    assert!(
        a04.contains("| [~] |") || a04.contains("| [x] |"),
        "M3-A04 may be [~] or [x] after W10 closeout: {a04}"
    );
    let sync = include_str!("../../../../packages/sync/mod.sy");
    assert!(
        sync.contains("trySend")
            && sync.contains("__syx_channel_try_send")
            && sync.contains("count")
            && sync.contains("capacity"),
        "Sync.Channel trySend/length/capacity facade required"
    );
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("ADR-267")
            || book.contains("backpressure")
            || book.contains("trySend"),
        "18-concurrency must document backpressure thin nail"
    );
}
