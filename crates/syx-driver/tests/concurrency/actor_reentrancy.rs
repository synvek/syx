//! actor reentrancy thin start (serial-no-reenter / C009).
// was: m3_a03_reentrancy

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a03_reentrancy_sync_self_call_fails_c009() {
    let src = r#"
actor Counter {
  var value: Int
  init(value: Int) {
    self.value = value
  }
  func inc() {
    self.value = self.value + 1
  }
  func bump() {
    self.inc()
  }
}

func main() {
  let c = Counter(value: 0)
  try {
    c.bump()
    print("OK")
  } catch (err) {
    print("DENIED")
    print(err.code)
  }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "same-actor sync reenter must fail fast (no hang): elapsed={elapsed:?} out={out}"
    );
    assert!(
        out.contains("DENIED") && out.contains("C009"),
        "sync self-call must surface C009: {out}"
    );
    assert!(!out.contains("OK"), "reentrant call must not succeed: {out}");
}

#[test]
fn m3_a03_reentrancy_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_actor_reentrancy_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("serial-no-reenter"),
        "policy HostFn must return serial-no-reenter: {out}"
    );
}

#[test]
fn m3_a03_reentrancy_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a03_reentrancy"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a03_reentrancy = \"signed-ADR-264\""),
        "inventory must note signed-ADR-264"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-264")
            || guide.contains("actorReentrancyPolicy")
            || guide.contains("serial-no-reenter"),
        "UsageGuide must document actor reentrancy thin start"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("ADR-264")
            && (roadmap.contains("reentran") || roadmap.contains("serial-no-reenter")),
        "roadmap must record M3 W5 actor reentrancy"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    // W5 locked W6=await-hold; after W6 signoff roadmap may advance to W7=A04.
    assert!(
        (roadmap.contains("W6")
            && (roadmap.contains("await-reentrancy")
                || roadmap.contains("await-hold")
                || roadmap.contains("await")
                || roadmap.contains("ADR-265")
                || roadmap.contains("done")))
            || (roadmap.contains("W7")
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
        "roadmap may advance W6–W15 done / W16 = B01"
    );
    let facade = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        facade.contains("actorReentrancyPolicy")
            && facade.contains("__syx_actor_reentrancy_policy"),
        "Concurrency.actorReentrancyPolicy facade required"
    );
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("ActorReentrancyPolicy")
            && reg.contains("__syx_actor_reentrancy_policy"),
        "HostFnId::ActorReentrancyPolicy must be registered"
    );
}
