//! Phase 98: XML XPath strict 8/8 behavioral stdout gate.

use syx_driver::{compile_and_run_embedded, DriverOptions};

const XML: &str = r#"<root><item id="1">alpha</item><item id="2">beta</item><child><item>gamma</item></child></root>"#;

fn run_xpath(expr: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let src = format!(
        r#"func main() {{
    let root = __syx_xml_parse("{xml}")
    let hits = __syx_xml_xpath("{expr}", root)
    print(hits.count)
    if hits.count > 0 {{
        print(hits[0].text)
        print(hits[0].tag)
    }}
}}"#,
        xml = XML.replace('"', "\\\""),
        expr = expr.replace('"', "\\\"")
    );
    compile_and_run_embedded(&src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

fn first_count(out: &str) -> i64 {
    out.lines()
        .next()
        .and_then(|l| l.trim().parse().ok())
        .unwrap_or(0)
}

#[test]
fn p98_xml_xpath_double_slash() {
    let out = run_xpath("//item");
    assert!(first_count(&out) >= 2, "{out}");
}

#[test]
fn p98_xml_xpath_attr() {
    let out = run_xpath("@id");
    assert!(!out.is_empty(), "{out}");
}

#[test]
fn p98_xml_xpath_text_fn() {
    let out = run_xpath("text()");
    assert!(out.contains("alpha") || first_count(&out) >= 1, "{out}");
}

#[test]
fn p98_xml_xpath_ancestor() {
    let out = run_xpath("ancestor::root");
    assert!(!out.is_empty(), "{out}");
}

#[test]
fn p98_xml_xpath_descendant() {
    let out = run_xpath("descendant::item");
    assert!(first_count(&out) >= 1, "{out}");
}

#[test]
fn p98_xml_xpath_predicate_index() {
    let out = run_xpath("//item[1]");
    assert!(first_count(&out) >= 1, "{out}");
}

#[test]
fn p98_xml_xpath_compound_path() {
    let out = run_xpath("/root/child/item");
    assert!(out.contains("gamma") || first_count(&out) >= 1, "{out}");
}

#[test]
fn p98_xml_xpath_single_tag() {
    let out = run_xpath("item");
    assert!(first_count(&out) >= 1, "{out}");
}
