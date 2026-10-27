//! Channel close thin start (real disconnect / drain / C010).
// was: m3_a04_channel_close

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a04_channel_close_drain_then_nil_and_send_c010() {
    let src = r#"
func main() {
  let ch = __syx_channel_create(4)
  __syx_channel_send(ch, "A")
  __syx_channel_close(ch)
  print(__syx_channel_try_recv(ch))
  print(__syx_channel_try_recv(ch))
  try {
    __syx_channel_send(ch, "B")
    print("SEND_OK")
  } catch (err) {
    print("SEND_CLOSED")
    print(err.code)
  }
}
"#;
    let out = run(src);
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert!(
        lines.len() >= 3,
        "expected drain A, Nil, SEND_CLOSED/C010: {out}"
    );
    assert_eq!(lines[0], "A", "must drain buffered value after close: {out}");
    assert!(
        lines[1] == "nil" || lines[1] == "Nil",
        "second recv after drain must be Nil: {out}"
    );
    assert!(
        out.contains("SEND_CLOSED") && out.contains("C010"),
        "post-close send must surface C010: {out}"
    );
    assert!(!out.contains("SEND_OK"), "post-close send must not succeed: {out}");
}

#[test]
fn m3_a04_channel_close_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a04_channel_close"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a04_channel_close = \"signed-ADR-266\""),
        "inventory must note signed-ADR-266"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-266")
            || guide.contains("m3_a04_channel_close")
            || guide.contains("Channel close"),
        "UsageGuide must document Channel close thin start"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("ADR-266")
            && (roadmap.contains("channel_close")
                || roadmap.contains("Channel close")
                || roadmap.contains("m3_a04")),
        "roadmap must record M3 W7 Channel close"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W8")
            && (roadmap.contains("backpressure")
                || roadmap.contains("ADR-267")
                || roadmap.contains("A04")
                || roadmap.contains("done")))
            || (roadmap.contains("W9")
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
        "roadmap may advance W8–W10 done / W11 = A05"
    );
    let sync = include_str!("../../../../packages/sync/mod.sy");
    assert!(
        sync.contains("close") && sync.contains("__syx_channel_close"),
        "Sync.Channel.close facade required"
    );
}
