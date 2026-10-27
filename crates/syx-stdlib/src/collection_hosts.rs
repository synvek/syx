//! Phase 46/47: collection HostFn registry (`__syx_array_*`, `__syx_set_*`, `__syx_dict_*`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionHostOp {
    Map,
    Filter,
    Reduce,
    Sorted,
    Reverse,
    Contains,
    Joined,
    Len,
    Count,
    ContainsKey,
    Keys,
    Insert,
    Values,
    /// M0-B03: Dictionary.remove
    Remove,
    /// Object/Dictionary string-key get/set (returns value / updated bag).
    Get,
    Set,
    /// Set.union / Set.intersection (value_equals).
    Union,
    Intersection,
}

#[derive(Debug, Clone, Copy)]
pub struct CollectionHostEntry {
    pub name: &'static str,
    pub op: CollectionHostOp,
    pub return_surface: &'static str,
}

const fn ch(name: &'static str, op: CollectionHostOp, ret: &'static str) -> CollectionHostEntry {
    CollectionHostEntry {
        name,
        op,
        return_surface: ret,
    }
}

pub static COLLECTION_HOST_TABLE: &[CollectionHostEntry] = &[
    ch("__syx_array_map", CollectionHostOp::Map, "Array"),
    ch("__syx_array_filter", CollectionHostOp::Filter, "Array"),
    ch("__syx_array_reduce", CollectionHostOp::Reduce, "Unknown"),
    ch("__syx_array_sorted", CollectionHostOp::Sorted, "Array"),
    ch("__syx_array_reverse", CollectionHostOp::Reverse, "Array"),
    ch("__syx_array_contains", CollectionHostOp::Contains, "Bool"),
    ch("__syx_array_joined", CollectionHostOp::Joined, "String"),
    ch("__syx_array_len", CollectionHostOp::Len, "Int"),
    ch("__syx_set_count", CollectionHostOp::Count, "Int"),
    ch("__syx_set_contains", CollectionHostOp::Contains, "Bool"),
    ch("__syx_set_insert", CollectionHostOp::Insert, "Bool"),
    ch("__syx_set_remove", CollectionHostOp::Remove, "Bool"),
    ch("__syx_set_union", CollectionHostOp::Union, "Set"),
    ch("__syx_set_intersection", CollectionHostOp::Intersection, "Set"),
    ch("__syx_dict_count", CollectionHostOp::Count, "Int"),
    ch("__syx_dict_contains", CollectionHostOp::ContainsKey, "Bool"),
    ch("__syx_dict_keys", CollectionHostOp::Keys, "Array"),
    ch("__syx_dict_values", CollectionHostOp::Values, "Array"),
    ch("__syx_dict_remove", CollectionHostOp::Remove, "Bool"),
    ch("__syx_dict_get", CollectionHostOp::Get, "Object"),
    ch("__syx_dict_set", CollectionHostOp::Set, "Object"),
];

pub fn lookup_collection_host(name: &str) -> Option<&'static CollectionHostEntry> {
    COLLECTION_HOST_TABLE.iter().find(|e| e.name == name)
}

pub fn collection_host_names() -> impl Iterator<Item = &'static str> {
    COLLECTION_HOST_TABLE.iter().map(|e| e.name)
}
