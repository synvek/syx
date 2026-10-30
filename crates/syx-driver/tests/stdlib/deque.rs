//! 03 Deque&lt;Int&gt; MVP nail.
// was: m2_s1_03_deque

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn run(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => r.printed.join("\n"),
        Err(e) => panic!("compile/run failed: {e:?}\n---src---\n{src}"),
    }
}

#[test]
fn m2_s1_03_push_back_pop_front_fifo() {
    let out = run(
        r#"
func main() {
  var d = Deque.new()
  Deque.pushBack(items: d, item: 1)
  Deque.pushBack(items: d, item: 2)
  Deque.pushBack(items: d, item: 3)
  print(Deque.count(items: d))
  print(Deque.popFront(items: d))
  print(Deque.popFront(items: d))
  print(Deque.popFront(items: d))
  print(Deque.count(items: d))
}
"#,
    );
    // Wave A W77 / ADR-421 / A-03: Deque pushBack/popFront FIFO product exact.
    assert_eq!(out, "3\n1\n2\n3\n0", "Deque FIFO must exact: {out}");
}

#[test]
fn m2_s1_03_push_front_pop_back_and_mixed() {
    let out = run(
        r#"
func main() {
  var d = Deque.new()
  Deque.pushFront(items: d, item: 2)
  Deque.pushFront(items: d, item: 1)
  Deque.pushBack(items: d, item: 3)
  print(Deque.popFront(items: d))
  print(Deque.popBack(items: d))
  print(Deque.popFront(items: d))
  print(Deque.isEmpty(items: d))
}
"#,
    );
    // Wave A W79 / ADR-423 / A-03: Deque pushFront/popBack mixed product exact.
    assert_eq!(out, "1\n3\n2\ntrue", "Deque mixed must exact: {out}");
}

#[test]
fn m2_s1_03_empty_pop_nil() {
    let out = run(
        r#"
func main() {
  var d = Deque.new()
  print(Deque.isEmpty(items: d))
  print(Deque.popFront(items: d))
  print(Deque.popBack(items: d))
  print(Deque.count(items: d))
}
"#,
    );
    // Wave A W77 / ADR-421 / A-03: Deque empty pop → nil product exact.
    assert_eq!(out, "true\nnil\nnil\n0", "Deque empty pop nil must exact: {out}");
}

#[test]
fn m2_s1_03_var_writeback_persists() {
    let out = run(
        r#"
func main() {
  var d = Deque.new()
  Deque.pushBack(items: d, item: 10)
  Deque.pushBack(items: d, item: 20)
  print(Deque.count(items: d))
  print(Deque.popFront(items: d))
  print(Deque.popFront(items: d))
}
"#,
    );
    // Wave A W79 / ADR-423 / A-03: Deque var writeback persists product exact.
    assert_eq!(out, "2\n10\n20", "Deque writeback must exact: {out}");
}

#[test]
fn m2_s1_03_docs_and_capability() {
    let host = include_str!("../../../syx-vm/src/script_hosts.rs");
    assert!(
        host.contains("host_deque_new")
            && host.contains("HeapSlotKind::Deque")
            && host.contains("ADR-248"),
        "host must allocate HeapSlotKind::Deque"
    );
    let facade = include_str!("../../../../packages/core/builtins/collections/deque.sy");
    assert!(
        facade.contains("__syx_deque_new")
            && facade.contains("pushBack")
            && facade.contains("popFront"),
        "deque.sy facade required"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "collections.deque");
    assert!(
        block.contains("status = \"implemented\""),
        "collections.deque must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s1_03_deque") && block.contains("ADR-248"),
        "capability must cite nail + ADR-248: {block}"
    );
    assert!(
        block.contains("Heap") || block.contains("lazy"),
        "notes must freeze Heap/lazy: {block}"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("Deque.new") || guide.contains("Deque（ADR-248"),
        "UsageGuide must document Deque"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s1_03_deque_tier = \"signed-ADR-248\""),
        "inventory must note signed-ADR-248"
    );
}

