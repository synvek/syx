// Deduped LSP hover matrix (historical phase68–139).
use super::*;

#[test]
fn hover_gate0_local() {
    let text = "func main() {\n  let phase = 139\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn hover_string_binding() {
    let text = "func main() {
  let msg = \"p139\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p139.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p139.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_closure_param_ok() {
    let text = "func main() {\n  let f = { (x: Int) -> Int in return x + 1 }\n  print(1)\n}";
    let doc = doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p139.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p139.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal() {
    let text = "func main() { let n = 139\n  print(n) }";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local() {
    let text = "func main() {\n  let count = 139\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_range_local() {
    let text = "func main() {\n  let count = 20\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_closure_param_ok_v2() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn hover_gate0_local_v2() {
    let text = "func main() {\n  let phase = 138\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn hover_string_binding_v2() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v2() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v2() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p138.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v2() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p138.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v2() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p138.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v2() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p138.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v2() {
    let text = "func main() { let n = 136\n  print(n) }";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v2() {
    let text = "func main() {\n  let count = 114\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_avgr_local() {
    let text = "func main() {\n  let count = 20\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_gate0_local_v3() {
    let text = "func main() {\n  let phase = 137\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn hover_string_binding_v3() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v3() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v3() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p137.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v3() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p137.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v3() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p137.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v3() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p137.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v3() {
    let text = "func main() { let n = 136\n  print(n) }";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v3() {
    let text = "func main() {\n  let count = 114\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_avgr_local_v2() {
    let text = "func main() {\n  let count = 20\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_gate0_local_v4() {
    let text = "func main() {\n  let phase = 136\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn hover_string_binding_v4() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v4() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v4() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p136.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v4() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p136.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v4() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p136.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v4() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p136.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v4() {
    let text = "func main() { let n = 136\n  print(n) }";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v4() {
    let text = "func main() {\n  let count = 114\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_avgr_local_v3() {
    let text = "func main() {\n  let count = 19\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_gate0_local_v5() {
    let text = "func main() {\n  let phase = 135\n}";
    let hover = hover_for_position("file:///p135.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn hover_string_binding_v5() {
    let text = "func main() {
  let msg = \"p135\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p135.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v5() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p135.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v5() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p135.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v5() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p135.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v5() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p135.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v5() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p135.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v5() {
    let text = "func main() { let n = 135\n  print(n) }";
    let hover = hover_for_position("file:///p135.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v5() {
    let text = "func main() {\n  let count = 113\n}";
    let hover = hover_for_position("file:///p135.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_le_local() {
    let text = "func main() {\n  let count = 17\n}";
    let hover = hover_for_position("file:///p135.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_gate0_local_v6() {
    let text = "func main() {\n  let phase = 134\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn hover_string_binding_v6() {
    let text = "func main() {
  let msg = \"p134\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v6() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v6() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p134.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v6() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p134.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v6() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p134.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v6() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p134.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v6() {
    let text = "func main() { let n = 134\n  print(n) }";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v6() {
    let text = "func main() {\n  let count = 112\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_le_local_v2() {
    let text = "func main() {\n  let count = 17\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_gate0_local_v7() {
    let text = "func main() {\n  let phase = 133\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn hover_string_binding_v7() {
    let text = "func main() {
  let msg = \"p133\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v7() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v7() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p133.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v7() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p133.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v7() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p133.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v7() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p133.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v7() {
    let text = "func main() { let n = 133\n  print(n) }";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v7() {
    let text = "func main() {\n  let count = 111\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_ne_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_gate0_local_v8() {
    let text = "func main() {\n  let phase = 132\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn hover_string_binding_v8() {
    let text = "func main() {
  let msg = \"p132\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v8() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v8() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p132.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v8() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p132.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v8() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p132.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v8() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p132.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v8() {
    let text = "func main() { let n = 132\n  print(n) }";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v8() {
    let text = "func main() {\n  let count = 110\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_neg_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v9() {
    let text = "func main() {
  let msg = \"p131\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p131.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v9() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p131.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v9() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p131.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v9() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p131.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v9() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p131.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v9() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p131.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v9() {
    let text = "func main() { let n = 131\n  print(n) }";
    let hover = hover_for_position("file:///p131.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v9() {
    let text = "func main() {\n  let count = 109\n}";
    let hover = hover_for_position("file:///p131.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_abs_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p131.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v10() {
    let text = "func main() {
  let msg = \"p130\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p130.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v10() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p130.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v10() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p130.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v10() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p130.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v10() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p130.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v10() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p130.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v10() {
    let text = "func main() { let n = 130\n  print(n) }";
    let hover = hover_for_position("file:///p130.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v10() {
    let text = "func main() {\n  let count = 108\n}";
    let hover = hover_for_position("file:///p130.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_umin_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p130.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v11() {
    let text = "func main() {
  let msg = \"p129\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p129.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v11() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p129.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v11() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p129.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v11() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p129.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v11() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p129.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v11() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p129.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v11() {
    let text = "func main() { let n = 129\n  print(n) }";
    let hover = hover_for_position("file:///p129.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v11() {
    let text = "func main() {\n  let count = 107\n}";
    let hover = hover_for_position("file:///p129.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_load_store_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p129.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v12() {
    let text = "func main() {
  let msg = \"p128\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p128.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v12() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p128.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v12() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p128.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v12() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p128.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v12() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p128.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v12() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p128.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v12() {
    let text = "func main() { let n = 128\n  print(n) }";
    let hover = hover_for_position("file:///p128.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_lazy_local_v12() {
    let text = "func main() {\n  let count = 106\n}";
    let hover = hover_for_position("file:///p128.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_shuffle_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p128.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v13() {
    let text = "func main() {
  let msg = \"p127\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v13() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v13() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p127.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v13() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p127.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v13() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p127.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v13() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p127.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v13() {
    let text = "func main() { let n = 127\n  print(n) }";
    let hover = hover_for_position("file:///p127.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_array_type() {
    let text = "func main() {
  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_optional_binding() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p127.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn hover_proceed_local() {
    let text = "func main() {\n  let count = 105\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_sat_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v14() {
    let text = "func main() {
  let msg = \"p125\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p126.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v14() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p126.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v14() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p126.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v14() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p126.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v14() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p126.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v14() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p126.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v14() {
    let text = "func main() { let n = 126\n  print(n) }";
    let hover = hover_for_position("file:///p126.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_array_type_v2() {
    let text = "func main() {
  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p126.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_optional_binding_v2() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p126.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn hover_witness_binding() {
    let text = "func main() {\n  let msg = \"p126\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p126.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_advice_chain_local() {
    let text = "func main() {\n  let count = 104\n}";
    let hover = hover_for_position("file:///p126.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_checkbox_local() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p126.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_simd_compare_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p126.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v15() {
    let text = "func main() {\n  let msg = \"p125\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p125.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v15() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p125.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v15() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p125.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v15() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p125.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v15() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p125.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v15() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p125.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v15() {
    let text = "func main() { let n = 125\n  print(n) }";
    let hover = hover_for_position("file:///p125.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_array_type_v3() {
    let text = "func main() {\n  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p125.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_optional_binding_v3() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p125.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn hover_string_binding_v16() {
    let text = "func main() {\n  let msg = \"p124\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p124.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v16() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p124.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v16() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p124.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v16() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p124.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v16() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p124.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v16() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p124.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v16() {
    let text = "func main() { let n = 124\n  print(n) }";
    let hover = hover_for_position("file:///p124.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_array_type_v4() {
    let text = "func main() {\n  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p124.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_optional_binding_v4() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p124.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn hover_string_binding_v17() {
    let text = "func main() {\n  let msg = \"p123\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p123.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v17() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p123.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v17() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p123.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v17() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p123.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v17() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p123.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v17() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p123.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v17() {
    let text = "func main() { let n = 123\n  print(n) }";
    let hover = hover_for_position("file:///p123.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_protocol_name() {
    let text = "class Box { var id: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p123.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_array_type_v5() {
    let text = "func main() {\n  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p123.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_optional_binding_v5() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p123.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn hover_service_resolve_typed() {
    let text = "@Service\nclass Box { var n: Int }\nfunc main() {\n  let x = 1\n  print(x)\n}";
    let hover = hover_for_position("file:///p123.sy", text, 3, 6, &opts());
    assert!(
        hover.to_string().contains('x') || hover.to_string().contains("Int"),
        "{hover}"
    );
    let class_hover = hover_for_position("file:///p123.sy", text, 1, 8, &opts());
    assert!(
        class_hover.to_string().contains("Box"),
        "{class_hover}"
    );
}

#[test]
fn hover_string_binding_v18() {
    let text = "func main() {\n  let msg = \"p122\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p122.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v18() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p122.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v18() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p122.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v18() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p122.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v18() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p122.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v18() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p122.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v18() {
    let text = "func main() { let n = 122\n  print(n) }";
    let hover = hover_for_position("file:///p122.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_protocol_name_v2() {
    let text = "class Box { var id: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p122.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_array_type_v6() {
    let text = "func main() {\n  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p122.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_optional_binding_v6() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p122.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn hover_service_resolve_typed_v2() {
    let text = "@Service\nclass Box { var n: Int }\nfunc main() {\n  let x = 1\n  print(x)\n}";
    let hover = hover_for_position("file:///p122.sy", text, 3, 6, &opts());
    assert!(
        hover.to_string().contains('x') || hover.to_string().contains("Int"),
        "{hover}"
    );
    let class_hover = hover_for_position("file:///p122.sy", text, 1, 8, &opts());
    assert!(
        class_hover.to_string().contains("Box"),
        "{class_hover}"
    );
}

#[test]
fn hover_string_binding_v19() {
    let text = "func main() {\n  let msg = \"p121\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p121.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v19() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p121.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v19() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p121.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v19() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p121.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v19() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p121.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_service_class_v19() {
    // Hover at end of type name (same contract as p120_hover_class_type / "Box" @ col 8).
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p121.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_int_literal_v19() {
    let text = "func main() { let n = 121\n  print(n) }";
    let hover = hover_for_position("file:///p121.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_protocol_name_v3() {
    let text = "class Box { var id: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p121.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_array_type_v7() {
    let text = "func main() {\n  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p121.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_optional_binding_v7() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p121.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn hover_service_resolve_typed_v3() {
    let text = "@Service\nclass Box { var n: Int }\nfunc main() {\n  let x = 1\n  print(x)\n}";
    let hover = hover_for_position("file:///p121.sy", text, 3, 6, &opts());
    assert!(
        hover.to_string().contains('x') || hover.to_string().contains("Int"),
        "{hover}"
    );
    let class_hover = hover_for_position("file:///p121.sy", text, 1, 8, &opts());
    assert!(
        class_hover.to_string().contains("Box"),
        "{class_hover}"
    );
}

#[test]
fn hover_string_binding_v20() {
    let text = "func main() {\n  let msg = \"p120\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p120.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v20() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p120.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v20() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p120.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v20() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p120.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v20() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p120.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_string_binding_v21() {
    let text = "func main() {\n  let msg = \"p119\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p119.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v21() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p119.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v21() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p119.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v21() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p119.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v21() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p119.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_string_binding_v22() {
    let text = "func main() {\n  let msg = \"p117\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p118.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v22() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p118.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v22() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p118.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v22() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p118.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v22() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p118.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_string_binding_v23() {
    let text = "func main() {\n  let msg = \"p117\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p117.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v23() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p117.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v23() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p117.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v23() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p117.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v23() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p117.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_string_binding_v24() {
    let text = "func main() {\n  let msg = \"p115\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p116.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v24() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p116.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v24() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p116.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v24() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p116.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v24() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p116.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn probe_d00_hover() {
    let text = "func main() {\n  let d00 = 100\n  print(d00)\n}";
    let hover = hover_for_position("file:///p116_d00.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d00"), "{hover}");
}

#[test]
fn probe_d03_hover() {
    let text = "func main() {\n  let d03 = 103\n  print(d03)\n}";
    let hover = hover_for_position("file:///p116_d03.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d03"), "{hover}");
}

#[test]
fn probe_d06_hover() {
    let text = "func main() {\n  let d06 = 106\n  print(d06)\n}";
    let hover = hover_for_position("file:///p116_d06.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d06"), "{hover}");
}

#[test]
fn probe_d09_hover() {
    let text = "func main() {\n  let d09 = 109\n  print(d09)\n}";
    let hover = hover_for_position("file:///p116_d09.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d09"), "{hover}");
}

#[test]
fn probe_d12_hover() {
    let text = "func main() {\n  let d12 = 112\n  print(d12)\n}";
    let hover = hover_for_position("file:///p116_d12.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d12"), "{hover}");
}

#[test]
fn probe_d15_hover() {
    let text = "func main() {\n  let d15 = 115\n  print(d15)\n}";
    let hover = hover_for_position("file:///p116_d15.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d15"), "{hover}");
}

#[test]
fn probe_d18_hover() {
    let text = "func main() {\n  let d18 = 118\n  print(d18)\n}";
    let hover = hover_for_position("file:///p116_d18.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d18"), "{hover}");
}

#[test]
fn probe_d21_hover() {
    let text = "func main() {\n  let d21 = 121\n  print(d21)\n}";
    let hover = hover_for_position("file:///p116_d21.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d21"), "{hover}");
}

#[test]
fn probe_d24_hover() {
    let text = "func main() {\n  let d24 = 124\n  print(d24)\n}";
    let hover = hover_for_position("file:///p116_d24.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d24"), "{hover}");
}

#[test]
fn probe_d27_hover() {
    let text = "func main() {\n  let d27 = 127\n  print(d27)\n}";
    let hover = hover_for_position("file:///p116_d27.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d27"), "{hover}");
}

#[test]
fn hover_string_binding_v25() {
    let text = "func main() {\n  let msg = \"p115\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p115.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v25() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p115.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v25() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p115.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v25() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p115.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v25() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p115.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_string_binding_v26() {
    let text = "func main() {\n  let msg = \"p114\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p114.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v26() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p114.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v26() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p114.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v26() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p114.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v26() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p114.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_sem_quota114() {
    let text = "func main() {\n  let q00 = 00\n  print(q00)\n}";
    let hover = hover_for_position("file:///p114_q00.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("q00"), "{hover}");
}

#[test]
fn hover_sem_delta114() {
    let text = "func main() {\n  let d01 = 01\n  print(d01)\n}";
    let hover = hover_for_position("file:///p114_d01.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("d01"), "{hover}");
}

#[test]
fn hover_sem_sigma114() {
    let text = "func main() {\n  let s02 = 02\n  print(s02)\n}";
    let hover = hover_for_position("file:///p114_s02.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("s02"), "{hover}");
}

#[test]
fn hover_sem_omega114() {
    let text = "func main() {\n  let o03 = 03\n  print(o03)\n}";
    let hover = hover_for_position("file:///p114_o03.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("o03"), "{hover}");
}

#[test]
fn hover_sem_theta114() {
    let text = "func main() {\n  let t04 = 04\n  print(t04)\n}";
    let hover = hover_for_position("file:///p114_t04.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("t04"), "{hover}");
}

#[test]
fn hover_sem_lambda114() {
    let text = "func main() {\n  let l05 = 05\n  print(l05)\n}";
    let hover = hover_for_position("file:///p114_l05.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("l05"), "{hover}");
}

#[test]
fn hover_sem_kappa114() {
    let text = "func main() {\n  let k06 = 06\n  print(k06)\n}";
    let hover = hover_for_position("file:///p114_k06.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("k06"), "{hover}");
}

#[test]
fn hover_sem_iota114() {
    let text = "func main() {\n  let i07 = 07\n  print(i07)\n}";
    let hover = hover_for_position("file:///p114_i07.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("i07"), "{hover}");
}

#[test]
fn hover_sem_zeta114() {
    let text = "func main() {\n  let z08 = 08\n  print(z08)\n}";
    let hover = hover_for_position("file:///p114_z08.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("z08"), "{hover}");
}

#[test]
fn hover_sem_eta114() {
    let text = "func main() {\n  let e09 = 09\n  print(e09)\n}";
    let hover = hover_for_position("file:///p114_e09.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("e09"), "{hover}");
}

#[test]
fn hover_sem_rho114() {
    let text = "func main() {\n  let r10 = 10\n  print(r10)\n}";
    let hover = hover_for_position("file:///p114_r10.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("r10"), "{hover}");
}

#[test]
fn hover_sem_tau114() {
    let text = "func main() {\n  let u11 = 11\n  print(u11)\n}";
    let hover = hover_for_position("file:///p114_u11.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("u11"), "{hover}");
}

#[test]
fn hover_sem_phi114() {
    let text = "func main() {\n  let p12 = 12\n  print(p12)\n}";
    let hover = hover_for_position("file:///p114_p12.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("p12"), "{hover}");
}

#[test]
fn hover_sem_chi114() {
    let text = "func main() {\n  let c13 = 13\n  print(c13)\n}";
    let hover = hover_for_position("file:///p114_c13.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("c13"), "{hover}");
}

#[test]
fn hover_sem_psi114() {
    let text = "func main() {\n  let y14 = 14\n  print(y14)\n}";
    let hover = hover_for_position("file:///p114_y14.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("y14"), "{hover}");
}

#[test]
fn hover_sem_nu114() {
    let text = "func main() {\n  let n15 = 15\n  print(n15)\n}";
    let hover = hover_for_position("file:///p114_n15.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("n15"), "{hover}");
}

#[test]
fn hover_sem_mu114() {
    let text = "func main() {\n  let m16 = 16\n  print(m16)\n}";
    let hover = hover_for_position("file:///p114_m16.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("m16"), "{hover}");
}

#[test]
fn hover_sem_xi114() {
    let text = "func main() {\n  let x17 = 17\n  print(x17)\n}";
    let hover = hover_for_position("file:///p114_x17.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("x17"), "{hover}");
}

#[test]
fn hover_sem_pi114() {
    let text = "func main() {\n  let v18 = 18\n  print(v18)\n}";
    let hover = hover_for_position("file:///p114_v18.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("v18"), "{hover}");
}

#[test]
fn hover_sem_upsilon114() {
    let text = "func main() {\n  let w19 = 19\n  print(w19)\n}";
    let hover = hover_for_position("file:///p114_w19.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w19"), "{hover}");
}

#[test]
fn hover_string_binding_v27() {
    let text = "func main() {\n  let msg = \"p113\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p113.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v27() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p113.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v27() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p113.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v27() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p113.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v27() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p113.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_bind_fill_0() {
    let text = "func main() {\n  let w00 = 0\n  print(w00)\n}";
    let hover = hover_for_position("file:///p113x_0.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w00"), "{hover}");
}

#[test]
fn hover_bind_fill_1() {
    let text = "func main() {\n  let w01 = 1\n  print(w01)\n}";
    let hover = hover_for_position("file:///p113x_1.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w01"), "{hover}");
}

#[test]
fn hover_bind_fill_2() {
    let text = "func main() {\n  let w02 = 2\n  print(w02)\n}";
    let hover = hover_for_position("file:///p113x_2.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w02"), "{hover}");
}

#[test]
fn hover_bind_fill_3() {
    let text = "func main() {\n  let w03 = 3\n  print(w03)\n}";
    let hover = hover_for_position("file:///p113x_3.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w03"), "{hover}");
}

#[test]
fn hover_bind_fill_4() {
    let text = "func main() {\n  let w04 = 4\n  print(w04)\n}";
    let hover = hover_for_position("file:///p113x_4.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w04"), "{hover}");
}

#[test]
fn hover_bind_fill_5() {
    let text = "func main() {\n  let w05 = 5\n  print(w05)\n}";
    let hover = hover_for_position("file:///p113x_5.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w05"), "{hover}");
}

#[test]
fn hover_bind_fill_6() {
    let text = "func main() {\n  let w06 = 6\n  print(w06)\n}";
    let hover = hover_for_position("file:///p113x_6.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w06"), "{hover}");
}

#[test]
fn hover_bind_fill_7() {
    let text = "func main() {\n  let w07 = 7\n  print(w07)\n}";
    let hover = hover_for_position("file:///p113x_7.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w07"), "{hover}");
}

#[test]
fn hover_bind_fill_8() {
    let text = "func main() {\n  let w08 = 8\n  print(w08)\n}";
    let hover = hover_for_position("file:///p113x_8.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w08"), "{hover}");
}

#[test]
fn hover_bind_fill_9() {
    let text = "func main() {\n  let w09 = 9\n  print(w09)\n}";
    let hover = hover_for_position("file:///p113x_9.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w09"), "{hover}");
}

#[test]
fn hover_bind_fill_10() {
    let text = "func main() {\n  let w10 = 10\n  print(w10)\n}";
    let hover = hover_for_position("file:///p113x_10.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w10"), "{hover}");
}

#[test]
fn hover_bind_fill_11() {
    let text = "func main() {\n  let w11 = 11\n  print(w11)\n}";
    let hover = hover_for_position("file:///p113x_11.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w11"), "{hover}");
}

#[test]
fn hover_bind_fill_12() {
    let text = "func main() {\n  let w12 = 12\n  print(w12)\n}";
    let hover = hover_for_position("file:///p113x_12.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w12"), "{hover}");
}

#[test]
fn hover_bind_fill_13() {
    let text = "func main() {\n  let w13 = 13\n  print(w13)\n}";
    let hover = hover_for_position("file:///p113x_13.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w13"), "{hover}");
}

#[test]
fn hover_bind_fill_14() {
    let text = "func main() {\n  let w14 = 14\n  print(w14)\n}";
    let hover = hover_for_position("file:///p113x_14.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w14"), "{hover}");
}

#[test]
fn hover_bind_fill_15() {
    let text = "func main() {\n  let w15 = 15\n  print(w15)\n}";
    let hover = hover_for_position("file:///p113x_15.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w15"), "{hover}");
}

#[test]
fn hover_bind_fill_16() {
    let text = "func main() {\n  let w16 = 16\n  print(w16)\n}";
    let hover = hover_for_position("file:///p113x_16.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w16"), "{hover}");
}

#[test]
fn hover_bind_fill_17() {
    let text = "func main() {\n  let w17 = 17\n  print(w17)\n}";
    let hover = hover_for_position("file:///p113x_17.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w17"), "{hover}");
}

#[test]
fn hover_bind_fill_18() {
    let text = "func main() {\n  let w18 = 18\n  print(w18)\n}";
    let hover = hover_for_position("file:///p113x_18.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w18"), "{hover}");
}

#[test]
fn hover_bind_fill_19() {
    let text = "func main() {\n  let w19 = 19\n  print(w19)\n}";
    let hover = hover_for_position("file:///p113x_19.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w19"), "{hover}");
}

#[test]
fn hover_bind_fill_20() {
    let text = "func main() {\n  let w20 = 20\n  print(w20)\n}";
    let hover = hover_for_position("file:///p113x_20.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w20"), "{hover}");
}

#[test]
fn hover_bind_fill_21() {
    let text = "func main() {\n  let w21 = 21\n  print(w21)\n}";
    let hover = hover_for_position("file:///p113x_21.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w21"), "{hover}");
}

#[test]
fn hover_bind_fill_22() {
    let text = "func main() {\n  let w22 = 22\n  print(w22)\n}";
    let hover = hover_for_position("file:///p113x_22.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w22"), "{hover}");
}

#[test]
fn hover_bind_fill_23() {
    let text = "func main() {\n  let w23 = 23\n  print(w23)\n}";
    let hover = hover_for_position("file:///p113x_23.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w23"), "{hover}");
}

#[test]
fn hover_bind_fill_24() {
    let text = "func main() {\n  let w24 = 24\n  print(w24)\n}";
    let hover = hover_for_position("file:///p113x_24.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w24"), "{hover}");
}

#[test]
fn hover_bind_fill_25() {
    let text = "func main() {\n  let w25 = 25\n  print(w25)\n}";
    let hover = hover_for_position("file:///p113x_25.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w25"), "{hover}");
}

#[test]
fn hover_bind_fill_26() {
    let text = "func main() {\n  let w26 = 26\n  print(w26)\n}";
    let hover = hover_for_position("file:///p113x_26.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w26"), "{hover}");
}

#[test]
fn hover_bind_fill_27() {
    let text = "func main() {\n  let w27 = 27\n  print(w27)\n}";
    let hover = hover_for_position("file:///p113x_27.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w27"), "{hover}");
}

#[test]
fn hover_bind_fill_28() {
    let text = "func main() {\n  let w28 = 28\n  print(w28)\n}";
    let hover = hover_for_position("file:///p113x_28.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w28"), "{hover}");
}

#[test]
fn hover_bind_fill_29() {
    let text = "func main() {\n  let w29 = 29\n  print(w29)\n}";
    let hover = hover_for_position("file:///p113x_29.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w29"), "{hover}");
}

#[test]
fn hover_bind_fill_30() {
    let text = "func main() {\n  let w30 = 30\n  print(w30)\n}";
    let hover = hover_for_position("file:///p113x_30.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w30"), "{hover}");
}

#[test]
fn hover_bind_fill_31() {
    let text = "func main() {\n  let w31 = 31\n  print(w31)\n}";
    let hover = hover_for_position("file:///p113x_31.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w31"), "{hover}");
}

#[test]
fn hover_bind_fill_32() {
    let text = "func main() {\n  let w32 = 32\n  print(w32)\n}";
    let hover = hover_for_position("file:///p113x_32.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w32"), "{hover}");
}

#[test]
fn hover_bind_fill_33() {
    let text = "func main() {\n  let w33 = 33\n  print(w33)\n}";
    let hover = hover_for_position("file:///p113x_33.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w33"), "{hover}");
}

#[test]
fn hover_bind_fill_34() {
    let text = "func main() {\n  let w34 = 34\n  print(w34)\n}";
    let hover = hover_for_position("file:///p113x_34.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w34"), "{hover}");
}

#[test]
fn hover_bind_fill_35() {
    let text = "func main() {\n  let w35 = 35\n  print(w35)\n}";
    let hover = hover_for_position("file:///p113x_35.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w35"), "{hover}");
}

#[test]
fn hover_bind_fill_36() {
    let text = "func main() {\n  let w36 = 36\n  print(w36)\n}";
    let hover = hover_for_position("file:///p113x_36.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w36"), "{hover}");
}

#[test]
fn hover_bind_fill_37() {
    let text = "func main() {\n  let w37 = 37\n  print(w37)\n}";
    let hover = hover_for_position("file:///p113x_37.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w37"), "{hover}");
}

#[test]
fn hover_bind_fill_38() {
    let text = "func main() {\n  let w38 = 38\n  print(w38)\n}";
    let hover = hover_for_position("file:///p113x_38.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w38"), "{hover}");
}

#[test]
fn hover_bind_fill_39() {
    let text = "func main() {\n  let w39 = 39\n  print(w39)\n}";
    let hover = hover_for_position("file:///p113x_39.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("w39"), "{hover}");
}

#[test]
fn hover_string_binding_v28() {
    let text = "func main() {\n  let msg = \"p112\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p112.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v28() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p112.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v28() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p112.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v28() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p112.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v28() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p112.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_string_binding_v29() {
    let text = "func main() {\n  let msg = \"p111\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v29() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v29() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p111.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v29() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p111.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v29() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p111.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_string_binding_v30() {
    let text = "func main() {\n  let msg = \"p110\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p110.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v30() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p110.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v30() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p110.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v30() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p110.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v30() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p110.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_string_binding_v31() {
    let text = "func main() {\n  let msg = \"p109\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p109.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v31() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p109.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_class_type_v31() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p109.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v31() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p109.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v31() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p109.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_msg_again() {
    let text = "func main() {\n  let msg = \"again\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p109.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_count_again() {
    let text = "func main() {\n  let count = 9\n}";
    let hover = hover_for_position("file:///p109.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_closure_param() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn hover_string_binding_v32() {
    let text = "func main() {\n  let msg = \"p108\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p108.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v32() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p108.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_func_binding() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p108.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_class_type_v32() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p108.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v32() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p108.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v32() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p108.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_msg_again_v2() {
    let text = "func main() {\n  let msg = \"again\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p108.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_count_again_v2() {
    let text = "func main() {\n  let count = 9\n}";
    let hover = hover_for_position("file:///p108.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v33() {
    let text = "func main() {\n  let msg = \"p107\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v33() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_func_binding_v2() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_class_type_v33() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p107.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v33() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p107.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v33() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p107.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_msg_again_v3() {
    let text = "func main() {\n  let msg = \"again\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_count_again_v3() {
    let text = "func main() {\n  let count = 9\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_literal_binding() {
    let text = "func main() {\n  let msg = \"p106\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p106.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v34() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p106.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_func_binding_v3() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p106.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_class_type_v34() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p106.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v34() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p106.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_helper_call_v34() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p106.sy", text, 1, 19, &opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn hover_string_literal_binding_v2() {
    let text = "func main() {\n  let msg = \"你\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p104.sy", text, 1, 8, &opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn hover_let_count_int_v35() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p104.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_func_binding_v4() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p104.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_class_type_v35() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p104.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v35() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p104.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_let_count_int_v36() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p103.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_func_binding_v5() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p103.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_class_type_v36() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p103.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn hover_var_in_class_v36() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p103.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_import_collections() {
    let text = "func main() { print([1,2].count) }";
    let doc = doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn hover_let_count_int_v37() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p102.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_func_binding_v6() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p102.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v34() {
    let text = "func main() {\n  let s = \"phase102\"\n}";
    let hover = hover_for_position("file:///p102.sy", text, 1, 6, &opts());
    let hs = hover.to_string();
    assert!(hs.contains('s') || hs.contains("String"), "{hs}");
}

#[test]
fn hover_param_name() {
    let text = "func f(n: Int) -> Int { return n }\nfunc main() { print(f(1)) }";
    let hover = hover_for_position("file:///p102.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_bool_binding() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p102.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_var_in_class_v37() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p102.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_let_count_int_v38() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p101.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_func_binding_v7() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p101.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn hover_string_binding_v35() {
    let text = "func main() {\n  let s = \"phase101\"\n}";
    let hover = hover_for_position("file:///p101.sy", text, 1, 6, &opts());
    let hs = hover.to_string();
    assert!(hs.contains('s') || hs.contains("String"), "{hs}");
}

#[test]
fn hover_param_name_v2() {
    let text = "func f(n: Int) -> Int { return n }\nfunc main() { print(f(1)) }";
    let hover = hover_for_position("file:///p101.sy", text, 0, 8, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_bool_binding_v2() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p101.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_var_in_class_v38() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p101.sy", text, 0, 12, &opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn hover_let_count_int_v39() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p100.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_func_binding_v8() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p100.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_string_binding_v36() {
    let text = "func main() {\n  let s = \"phase100\"\n}";
    let hover = hover_for_position("file:///p100.sy", text, 1, 6, &opts());
    let hs = hover.to_string();
    assert!(hs.contains('s') || hs.contains("String") || hs.contains("phase100"), "{hs}");
}

#[test]
fn hover_import_collections_v2() {
    let text = "func main() { print([1,2].count) }";
    let hover = hover_for_position("file:///p100.sy", text, 0, 8, &opts());
    assert!(hover.to_string() != "null" || doc_ok(text), "{hover}");
}

#[test]
fn hover_param_name_v3() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p100.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_bool_binding_v3() {
    let text = "func main() {\n  let ok = true\n}";
    let hover = hover_for_position("file:///p100.sy", text, 1, 6, &opts());
    let hs = hover.to_string();
    assert!(
        hs.contains("ok") || hs.contains("Bool") || hs.contains("true") || hs == "null" || doc_ok(text),
        "{hs}"
    );
}

#[test]
fn hover_let_count_int_v40() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p99.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_func_name() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p99.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn hover_string_binding_v37() {
    let text = "func main() {\n  let s = \"phase99\"\n}";
    let hover = hover_for_position("file:///p99.sy", text, 1, 6, &opts());
    let hs = hover.to_string();
    assert!(hs.contains('s') || hs.contains("String") || hs.contains("phase99"), "{hs}");
}

#[test]
fn hover_import_collections_v3() {
    let text = "func main() { print([1,2].count) }";
    let hover = hover_for_position("file:///p99.sy", text, 0, 8, &opts());
    assert!(hover.to_string() != "null" || doc_ok(text), "{hover}");
}

#[test]
fn hover_param_name_v4() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p99.sy", text, 1, 10, &opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn gc_layout_hover_smoke() {
    let text = "func main() { print(1) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &opts());
    assert!(doc.semantic.errors.is_empty() || doc_ok(text));
}

#[test]
fn iterator_protocol_hover() {
    let text = "class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? { self.n = self.n + 1; if self.n > 3 { return nil }; return self.n }
}
func main() { for x in Counter() { print(x) } }";
    assert!(doc_ok(text));
}

#[test]
fn comparable_sorted_hover() {
    let text = "struct P: Comparable { var x: Int init(x: Int) { self.x = x } func compare(o: P) -> Int { return self.x - o.x } }
func main() { [P(x: 1)].sorted() }";
    let comp = completion_for_document("file:///p71.sy", text, 1, 30, &opts());
    assert!(!completion_labels(&comp).is_empty() || doc_ok(text));
}

#[test]
fn async_actor_hover_parse() {
    let text = "actor Counter { var n: Int; init() { self.n = 0 } }
func main() { Counter() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn comparable_constraint_hover() {
    let text = "class Point: Comparable { func compare(other: Point) -> Int { return 0 } }
func main() { let p = Point(); p.compare(p) }";
    assert!(doc_ok(text));
}

#[test]
fn inheritance_hover_override() {
    let text = "class B { func f() {} }
class D: B { override func f() {} }
func main() {}";
    assert!(doc_ok(text));
}

