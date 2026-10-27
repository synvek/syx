//! await-hold deepen (no mid-await mailbox interleaving).
// was: m3_a03_await_reentrancy

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a03_await_hold_peek_sees_after_not_before() {
    // hold(): BEFORE → sleep → AFTER. Concurrent peek while sleeping must wait for
    // the handler to finish (await-hold), so peek observes AFTER — not BEFORE
    // (which would indicate Swift-style mid-await mailbox reentry).
    // Task.detached closures cannot capture locals — pass actor via detach args.
    let src = r#"
actor Hold {
  var phase: String
  init(phase: String) {
    self.phase = phase
  }
  func hold() {
    self.phase = "BEFORE"
    __syx_task_sleep(200)
    self.phase = "AFTER"
  }
  func peek() -> String {
    return self.phase
  }
}

func kickHold(_ a: Hold) {
  a.hold()
}

func main() {
  let a = Hold(phase: "INIT")
  let t = __syx_task_detach("kickHold", [a])
  __syx_task_sleep(80)
  print(a.peek())
  __syx_task_join(t)
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "await-hold peek must not hang: elapsed={elapsed:?} out={out}"
    );
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert!(
        !lines.is_empty(),
        "expected peek print: {out}"
    );
    assert_eq!(
        lines[0], "AFTER",
        "peek during hold-sleep must serialize to AFTER (await-hold), not BEFORE: {out}"
    );
    assert!(
        !out.contains("BEFORE"),
        "must not observe mid-await BEFORE from reentrant peek: {out}"
    );
}

#[test]
fn m3_a03_await_hold_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_actor_reentrancy_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("serial-no-reenter") && out.contains("await-hold"),
        "policy must be serial-no-reenter;await-hold: {out}"
    );
}

#[test]
fn m3_a03_await_reentrancy_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a03_await_reentrancy"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a03_await_reentrancy = \"signed-ADR-265\""),
        "inventory must note signed-ADR-265"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-265")
            || guide.contains("await-hold")
            || guide.contains("m3_a03_await_reentrancy"),
        "UsageGuide must document await-hold deepen"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("ADR-265")
            && (roadmap.contains("await-hold") || roadmap.contains("await-reentrancy")),
        "roadmap must record M3 W6 await-hold"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    // W6 locked W7=A04; after W7–W9 signoff roadmap may advance to W10=closeout.
    assert!(
        (roadmap.contains("W7")
            && (roadmap.contains("A04")
                || roadmap.contains("Channel")
                || roadmap.contains("ADR-266")
                || roadmap.contains("done")))
            || (roadmap.contains("W8")
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
                && (roadmap.contains("closeout") || roadmap.contains("A05")))
            || (roadmap.contains("W13")
                && (roadmap.contains("closeout")
                    || roadmap.contains("A01")
                    || roadmap.contains("ADR-272")
                    || roadmap.contains("done")))
            || (roadmap.contains("W14")
                && (roadmap.contains("closeout")
                    || roadmap.contains("A02")
                    || roadmap.contains("ADR-273")
                    || roadmap.contains("done")))
            || (roadmap.contains("W15")
                && (roadmap.contains("closeout")
                    || roadmap.contains("A03")
                    || roadmap.contains("ADR-274")
                    || roadmap.contains("done")))
            || (roadmap.contains("W16")
                && (roadmap.contains("B01")
                    || roadmap.contains("reactor")
                    || roadmap.contains("blocking")
                    || roadmap.contains("M3-B01"))),
        "roadmap may advance W7–W15 done / W16 = B01"
    );
    let facade = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        facade.contains("actorReentrancyPolicy")
            && facade.contains("__syx_actor_reentrancy_policy"),
        "Concurrency.actorReentrancyPolicy facade required"
    );
}
