//! Atomic / memory ordering docs thin start.
// was: m3_c03_memory_ordering

#[test]
fn m3_c03_memory_ordering_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_c03_memory_ordering"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_c03_memory_ordering = \"signed-ADR-287\""),
        "inventory must note signed-ADR-287"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-287")
            || guide.contains("m3_c03_memory_ordering")
            || guide.contains("happens-before"),
        "UsageGuide must document M3-C03 memory ordering"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let c03 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-C03 |"))
        .expect("M3-C03 row");
    assert!(
        c03.contains("| [~] |") || c03.contains("| [x] |"),
        "M3-C03 may be [~] or [x] after later windows: {c03}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W30")
            && (roadmap.contains("C04")
                || roadmap.contains("race")
                || roadmap.contains("stress")
                || roadmap.contains("thin start")))
            || (roadmap.contains("W29")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-288")
                    || roadmap.contains("C03")))
            || (roadmap.contains("W28") && roadmap.contains("ADR-287")),
        "roadmap may advance W28–W29 done / W30 = C04 thin start"
    );
}

#[test]
fn m3_c03_memory_ordering_docs_honesty() {
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("happens-before") || book.contains("内存序"),
        "18-concurrency must document memory ordering / happens-before"
    );
    assert!(
        book.contains("Mutex")
            && (book.contains("Semaphore") || book.contains("Channel")),
        "18-concurrency must tie sync primitives to ordering boundary"
    );
    assert!(
        book.contains("ADR-287") || book.contains("Atomic"),
        "18-concurrency must reference ADR-287 / Atomic honesty"
    );
    assert!(
        !book.contains("AtomicInt GA")
            && !book.contains("Wasm atomics GA")
            && !book.contains("memory_order 产品 GA"),
        "must not claim product Atomic / Wasm atomics GA"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("happens-before")
            || guide.contains("内存序")
            || guide.contains("ADR-287"),
        "UsageGuide must state memory ordering honesty"
    );
    assert!(
        !guide.contains("AtomicInt GA")
            && !guide.contains("Atomic 产品 GA")
            && guide.contains("unsupported"),
        "UsageGuide must keep Atomic honesty (unsupported / no product GA claim)"
    );
    let caps = include_str!("../../../../books/shared/host-fn-capabilities.md");
    assert!(
        caps.contains("WasmAtomicAddI32") && caps.contains("unsupported"),
        "host-fn capabilities must keep WasmAtomicAddI32 unsupported"
    );
}
