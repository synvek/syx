//! todo 4 Object soft-close inventory.
//! Locks item-4 done-face SSOT + OUT list; points back to guide / object.sy / lang nails.
//! ≠ merge / true HashMap / Object generics / object_bag_ga "delivered"
//! ≠ HostFn+1; ≠ Wave flip; no new Object/Dictionary runtime.


#[test]
fn todo4_object_soft_close_floor_nails_indexed() {
    let object_sy = include_str!("../../../../packages/core/builtins/reference/object.sy");
    assert!(
        object_sy.contains("func valueForKey")
            && object_sy.contains("func put")
            && object_sy.contains("func containsKey")
            && object_sy.contains("func keys")
            && object_sy.contains("func values")
            && object_sy.contains("func remove")
            && object_sy.contains("var count")
            && object_sy.contains("var isEmpty"),
        "object.sy must keep todo-4 method surface"
    );

    let guide = include_str!("../../../../books/language-guide/05-collection-types.md");
    assert!(
        guide.contains("显式")
            && guide.contains("Object")
            && guide.contains("Dictionary")
            && (guide.contains("异构") || guide.contains("异质")),
        "language-guide must keep Dictionary-default vs explicit Object bag narrative"
    );

    let keys = include_str!("../lang/object_keys_type.rs");
    assert!(
        keys.contains("let o: Object") && keys.contains("keys"),
        "lang object_keys_type nail must remain"
    );

    let values = include_str!("../lang/object_values_type.rs");
    assert!(
        values.contains("let o: Object") && values.contains("values"),
        "lang object_values_type nail must remain"
    );

    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("todo4_object_soft_close_first_slice")
            && adr.contains("object_keys_type")
            && adr.contains("object_values_type"),
        "ADR-1599 must index this inventory nail + Object keys/values floor"
    );

    let matrix = include_str!("../../../../packages/internal/hostfn_qs1_matrix.toml");
    assert!(
        matrix.contains("schema_version = 1") || matrix.contains("schema_version=1"),
        "hostfn_qs1_matrix.toml must declare schema_version"
    );
}
