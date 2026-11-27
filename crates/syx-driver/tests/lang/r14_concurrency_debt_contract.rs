//! R-14 Concurrency debt contract.
//! Nails: typed catch bindings, actor __actor_id on heap-v4 slots, doc honesty anchors.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn r14_typed_catch_bindings_in_concurrency_tests() {
    // Bare `catch (err)` binds String; reading `.code` needs `catch (err: Object)`.
    for rel in [
        "crates/syx-driver/tests/concurrency/task_tree.rs",
        "crates/syx-driver/tests/concurrency/task_deadline.rs",
        "crates/syx-driver/tests/concurrency/chaos_sampling.rs",
        "crates/syx-driver/tests/concurrency/actor_reentrancy.rs",
    ] {
        let src = std::fs::read_to_string(repo_path(rel)).unwrap();
        assert!(
            !src.contains("} catch (err) {"),
            "{rel} must not use a bare `catch (err)` when reading err.code"
        );
        assert!(
            src.contains("} catch (err: Object) {"),
            "{rel} must use `catch (err: Object)`"
        );
    }
}

#[test]
fn r14_actor_id_stamped_on_heap_slots() {
    let value = std::fs::read_to_string(repo_path("crates/syx-runtime-core/src/value.rs")).unwrap();
    let start = value
        .find("pub fn register_actor_id")
        .expect("register_actor_id must exist");
    let body = &value[start..];
    let body = &body[..body.find("\n    pub fn actor_id").unwrap_or(body.len())];
    assert!(
        body.contains("Value::HeapSlot { idx, kind }"),
        "register_actor_id must handle HeapSlot (heap-v4 actor instances)"
    );
    assert!(
        body.contains("__actor_id"),
        "register_actor_id must stamp __actor_id"
    );
}

#[test]
fn r14_concurrency_docs_honesty_anchors() {
    let book =
        std::fs::read_to_string(repo_path("books/language-guide/18-concurrency.md")).unwrap();
    assert!(
        book.contains("应用并发") || book.contains("主业务"),
        "18-concurrency must name the application concurrency tier"
    );
    assert!(
        book.contains("happens-before") || book.contains("内存序"),
        "18-concurrency must document memory ordering"
    );

    let guide = std::fs::read_to_string(repo_path("docs/9_UsageGuide.md")).unwrap();
    assert!(
        guide.contains("happens-before") || guide.contains("内存序") || guide.contains("ADR-287"),
        "UsageGuide must state memory ordering honesty"
    );
    assert!(
        guide.contains("高级运行时") || guide.contains("ADR-270"),
        "UsageGuide must state advanced runtime layering"
    );
    assert!(
        guide.contains("unsupported"),
        "UsageGuide must keep Atomic honesty (unsupported)"
    );
    assert!(
        !guide.contains("AtomicInt GA") && !guide.contains("Atomic 产品 GA"),
        "UsageGuide must not claim product Atomic GA"
    );
    assert!(
        guide.contains("catch (e: Object)") || guide.contains("catch (e: Object)"),
        "UsageGuide must document the typed catch binding for err.code"
    );
}

#[test]
fn r14_actor_surface_has_no_vacuous_assert() {
    let src = std::fs::read_to_string(repo_path(
        "crates/syx-driver/tests/concurrency/concurrency_surface.rs",
    ))
    .unwrap();
    assert!(
        !src.contains("if let Ok((code, lines)) = run_file_printed"),
        "actor fixtures must not silently skip assertions"
    );
}
