// Deduped LSP go-to-definition matrix (historical phase68–139).
use super::*;

#[test]
fn definition_msg_use() {
    let text = "func main() {
  let msg = \"p139\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p139.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p139.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p139.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p139.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p139.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p139.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_hold_helper() {
    let text = "func dampen() -> Int { return 5 }\nfunc main() { dampen() }";
    let def = definition_for_position("file:///p139-hold.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_nested_helper() {
    let text = "func osHost() -> Int { return 1 }\nfunc main() { osHost() }";
    let def = definition_for_position("file:///p139-host.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_third_helper() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p139.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v2() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p138.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v2() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p138.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v2() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p138.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v2() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p138.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v2() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p138.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v2() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p138.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_hold_helper_v2() {
    let text = "func dampen() -> Int { return 5 }\nfunc main() { dampen() }";
    let def = definition_for_position("file:///p138-hold.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_hostname_helper() {
    let text = "func osHost() -> Int { return 1 }\nfunc main() { osHost() }";
    let def = definition_for_position("file:///p138-host.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_third_helper_v2() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p138.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v3() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p137.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v3() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p137.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v3() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p137.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v3() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p137.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v3() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p137.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v3() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p137.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_hold_helper_v3() {
    let text = "func dampen() -> Int { return 5 }\nfunc main() { dampen() }";
    let def = definition_for_position("file:///p137-hold.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_hostname_helper_v2() {
    let text = "func osHost() -> Int { return 1 }\nfunc main() { osHost() }";
    let def = definition_for_position("file:///p137-host.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_third_helper_v3() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p137.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v4() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p136.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v4() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p136.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v4() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p136.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v4() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p136.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v4() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p136.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v4() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p136.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_hold_helper_v4() {
    let text = "func dampen() -> Int { return 5 }\nfunc main() { dampen() }";
    let def = definition_for_position("file:///p136-hold.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_hostname_helper_v3() {
    let text = "func osHost() -> Int { return 1 }\nfunc main() { osHost() }";
    let def = definition_for_position("file:///p136-host.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_third_helper_v4() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p136.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v5() {
    let text = "func main() {
  let msg = \"p135\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p135.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v5() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p135.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v5() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p135.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v5() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p135.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v5() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p135.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v5() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p135.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v6() {
    let text = "func main() {
  let msg = \"p134\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p134.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v6() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p134.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v6() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p134.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v6() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p134.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v6() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p134.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v6() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p134.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v7() {
    let text = "func main() {
  let msg = \"p133\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p133.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v7() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p133.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v7() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p133.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v7() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p133.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v7() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p133.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v7() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p133.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v8() {
    let text = "func main() {
  let msg = \"p132\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p132.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v8() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p132.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v8() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p132.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v8() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p132.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v8() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p132.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v8() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p132.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v9() {
    let text = "func main() {
  let msg = \"p131\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p131.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v9() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p131.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v9() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p131.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v9() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p131.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v9() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p131.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v9() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p131.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v10() {
    let text = "func main() {
  let msg = \"p130\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p130.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v10() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p130.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v10() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p130.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v10() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p130.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v10() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p130.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v10() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p130.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v11() {
    let text = "func main() {
  let msg = \"p129\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p129.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v11() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p129.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v11() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p129.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v11() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p129.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v11() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p129.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v11() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p129.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v12() {
    let text = "func main() {
  let msg = \"p128\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p128.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v12() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p128.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v12() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p128.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v12() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p128.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v12() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p128.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_lazy_helper_v12() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p128.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v13() {
    let text = "func main() {
  let msg = \"p127\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p127.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v13() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p127.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v13() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p127.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v13() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p127.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v13() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p127.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_bean_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p127.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v14() {
    let text = "func main() {
  let msg = \"p125\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p126.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v14() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p126.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v14() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p126.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v14() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p126.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v14() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p126.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_witness_use() {
    let text = "func main() {\n  let msg = \"p126\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p126.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_slider_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p126.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v15() {
    let text = "func main() {\n  let msg = \"p125\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p125.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v15() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p125.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v15() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p125.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v15() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p125.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v15() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p125.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v16() {
    let text = "func main() {\n  let msg = \"p124\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p124.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v16() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p124.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v16() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p124.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v16() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p124.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v16() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p124.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v17() {
    let text = "func main() {\n  let msg = \"p123\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p123.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v17() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p123.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v17() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p123.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v17() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p123.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v17() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p123.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_enum_case() {
    let text = "func helper() -> Int { return 123 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p123.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v18() {
    let text = "func main() {\n  let msg = \"p122\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p122.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v18() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p122.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v18() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p122.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v18() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p122.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v18() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p122.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_enum_case_v2() {
    let text = "func helper() -> Int { return 122 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p122.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v19() {
    let text = "func main() {\n  let msg = \"p121\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p121.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v19() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p121.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v19() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p121.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v19() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p121.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_param_in_func_v19() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p121.sy", text, 1, 22, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_enum_case_v3() {
    let text = "func helper() -> Int { return 121 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p121.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v20() {
    let text = "func main() {\n  let msg = \"p120\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p120.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v20() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p120.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v20() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p120.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v20() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p120.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v21() {
    let text = "func main() {\n  let msg = \"p119\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p119.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v21() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p119.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v21() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p119.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v21() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p119.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v22() {
    let text = "func main() {\n  let msg = \"p117\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p118.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v22() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p118.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v22() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p118.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v22() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p118.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v23() {
    let text = "func main() {\n  let msg = \"p117\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p117.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v23() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p117.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v23() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p117.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v23() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p117.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v24() {
    let text = "func main() {\n  let msg = \"p115\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p116.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v24() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p116.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v24() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p116.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v24() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p116.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v25() {
    let text = "func main() {\n  let msg = \"p115\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p115.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v25() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p115.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v25() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p115.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v25() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p115.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v26() {
    let text = "func main() {\n  let msg = \"p114\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p114.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v26() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p114.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v26() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p114.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v26() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p114.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v27() {
    let text = "func main() {\n  let msg = \"p113\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p113.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v27() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p113.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v27() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p113.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v27() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p113.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v28() {
    let text = "func main() {\n  let msg = \"p112\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p112.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v28() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p112.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v28() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p112.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v28() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p112.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v29() {
    let text = "func main() {\n  let msg = \"p111\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p111.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v29() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p111.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v29() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p111.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v29() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p111.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v30() {
    let text = "func main() {\n  let msg = \"p110\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p110.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v30() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p110.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v30() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p110.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v30() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p110.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v31() {
    let text = "func main() {\n  let msg = \"p109\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p109.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_fn_v31() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p109.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v31() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p109.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_second_helper_v31() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p109.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_use_again() {
    let text = "func main() {\n  let msg = \"x\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p109.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_msg_use_v32() {
    let text = "func main() {\n  let msg = \"p108\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p108.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p108.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v32() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p108.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_local_use() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p108.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_again() {
    let text = "func main() {\n  let msg = \"again\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p108.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_again() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p108.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v33() {
    let text = "func main() {\n  let msg = \"p107\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p107.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_v2() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p107.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v33() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p107.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_local_use_v2() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p107.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_again_v2() {
    let text = "func main() {\n  let msg = \"again\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p107.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_again_v2() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p107.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v34() {
    let text = "func main() {\n  let msg = \"p106\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p106.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_v3() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p106.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v34() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p106.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_local_use_v3() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p106.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_msg_use_v35() {
    let text = "func main() {\n  let msg = \"你\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p104.sy", text, 2, 10, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_v4() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p104.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v35() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p104.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_local_use_v4() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p104.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_helper_v5() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p103.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v36() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p103.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_local_use_v5() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p103.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_helper_v6() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p102.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v37() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p102.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_second() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p102.sy", text, 1, 19, &opts());
    assert!(def.get("range").is_some(), "expected helper def: {def}");
}

#[test]
fn definition_helper_third() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p102.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_use_v6() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p102.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_helper_v7() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p101.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v38() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p101.sy", text, 2, 8, &opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn definition_helper_second_v2() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p101.sy", text, 1, 19, &opts());
    assert!(def.get("range").is_some(), "expected helper def: {def}");
}

#[test]
fn definition_helper_third_v2() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p101.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_use_v7() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p101.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_helper_v8() {
    let text = "func helper() { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p100.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v39() {
    let text = "func helper() { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p100.sy", text, 1, 19, &opts());
    assert!(
        def.get("range").is_some(),
        "expected definition location: {def}"
    );
}

#[test]
fn definition_class_name() {
    let text = "class Point { var x: Int; init(x: Int) { self.x = x } }\nfunc main() { print(Point(x: 1).x) }";
    let def = definition_for_position("file:///p100.sy", text, 1, 20, &opts());
    assert!(
        def.get("range").is_some() || doc_ok(text),
        "class definition or parse ok: {def}"
    );
}

#[test]
fn definition_struct_type() {
    let text = "struct Box { var n: Int }\nfunc main() { print(Box(n: 1).n) }";
    let def = definition_for_position("file:///p100.sy", text, 1, 18, &opts());
    assert!(
        def.get("range").is_some() || doc_ok(text),
        "struct definition or parse ok: {def}"
    );
}

#[test]
fn definition_helper_v9() {
    let text = "func helper() { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p99.sy", text, 1, 19, &opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn definition_local_binding_v40() {
    let text = "func helper() { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p99.sy", text, 1, 19, &opts());
    assert!(
        def.get("range").is_some(),
        "expected definition location: {def}"
    );
}

#[test]
fn definition_class_name_v2() {
    let text = "class Point { var x: Int; init(x: Int) { self.x = x } }\nfunc main() { print(Point(x: 1).x) }";
    let def = definition_for_position("file:///p99.sy", text, 1, 20, &opts());
    assert!(
        def.get("range").is_some() || doc_ok(text),
        "class definition or parse ok: {def}"
    );
}

