//! Atomic / memory ordering docs thin start.
// was: m3_c03_memory_ordering


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
