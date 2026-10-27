mod bigint_hosts;
mod builtin_kind;
mod collection;
mod collection_hosts;
mod numeric;
mod numeric_hosts;
mod protocols;
mod register_hosts;
mod registry;
mod result_hosts;

pub use bigint_hosts::{
    bigint_host_names, lookup_bigint_host, BigIntHostEntry, BigIntHostOp, BIGINT_HOST_TABLE,
};
pub use collection_hosts::{
    collection_host_names, lookup_collection_host, CollectionHostEntry, CollectionHostOp,
    COLLECTION_HOST_TABLE,
};
pub use register_hosts::{
    all_registered_host_names, is_manifest_host_op, manifest_host_name, manifest_host_ops_parity,
};
pub use registry::{plugin_host_base_index, plugin_host_return_type, register_compile_plugin_hosts};
pub use result_hosts::{
    lookup_result_host, result_host_names, ResultHostEntry, ResultHostOp, RESULT_HOST_TABLE,
};

pub use collection::COLLECTION_TYPE_NAMES;
pub use builtin_kind::{
    internal_id_for_surface, lookup_manifest_entry, manifest_entries, numeric_meta,
    prelude_module_names, retired_ops_module_names, BuiltinKind, ManifestEntry,
};
pub use protocols::{
    is_comparable_type, is_custom_debug_string_convertible, is_custom_string_convertible,
    is_equatable_type, is_hashable_type_name, known_protocol, protocol_associated_types,
    type_conforms, ProtocolId,
};
pub use numeric_hosts::{
    host_return_type_by_name, host_slug, lookup_numeric_host, numeric_host_names,
    NumericHostEntry, NumericHostOp, NUMERIC_HOST_TABLE,
};
pub use numeric::{
    bit_width, default_float_kind, default_integer_kind, float_literal_bits, from_suffix,
    from_type_name, int_range, is_float, is_implicit_convertible, is_integer, is_integer_narrowing,
    is_signed, is_unsigned, resolve_alias, type_name, type_name_with_alias, widen_for_binary,
    NumericKind,
    PLATFORM_INT, PLATFORM_UINT,
};
pub use registry::{
    host_fn_id_by_name, host_fn_index, host_fn_index_by_name, host_fn_name_for_index, host_fn_return_type,
    is_instance_property_member, lookup_array_method, lookup_dict_method, lookup_http_response_method,
    lookup_set_method, lookup_sql_row_method, lookup_string_method, lookup_type_method,
    lookup_float64_math_host, lookup_float32_math_host, method_from_index,
    method_id_call_host_index, method_param_label,
    method_return_type, HostFnId, MethodId, TypeMethod, ARRAY_METHODS, DICT_METHODS,
    HTTP_RESPONSE_METHODS, SET_METHODS, SQL_ROW_METHODS, STRING_METHODS, TYPE_METHODS,
};
use std::sync::LazyLock;

use indexmap::IndexMap;

pub struct StdlibEntry {
    pub name: &'static str,
    pub type_name: &'static str,
}

pub static BUILTIN_TYPES: LazyLock<IndexMap<&'static str, &'static str>> = LazyLock::new(|| {
    IndexMap::from([
        // Fixed-width integers
        ("Int8", "Int8"),
        ("Int16", "Int16"),
        ("Int32", "Int32"),
        ("Int64", "Int64"),
        ("Int128", "Int128"),
        ("UInt8", "UInt8"),
        ("UInt16", "UInt16"),
        ("UInt32", "UInt32"),
        ("UInt64", "UInt64"),
        ("UInt128", "UInt128"),
        // Platform integers
        ("Int", "Int"),
        ("UInt", "UInt"),
        // Floating point
        ("Float32", "Float32"),
        ("Float64", "Float64"),
        ("Float", "Float32"),
        ("Double", "Float64"),
        // Core types
        ("Bool", "Bool"),
        ("String", "String"),
        ("Void", "Void"),
        // Collections
        ("Array", "Array"),
        ("Set", "Set"),
        ("Dictionary", "Dictionary"),
        ("OrderedSet", "OrderedSet"),
        ("Deque", "Deque"),
        ("Object", "Object"),
        ("ObjectIdentifier", "ObjectIdentifier"),
        // Protocol names (documentation / semantic registration)
        ("Equatable", "Equatable"),
        ("Hashable", "Hashable"),
        ("CustomStringConvertible", "CustomStringConvertible"),
        ("Codable", "Codable"),
        ("Result", "Result"),
        ("BigInt", "BigInt"),
        // Host return / witness types (not importable modules)
        ("HTTPResponse", "HTTPResponse"),
    ])
});

pub static ITERATOR_PROTOCOL: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| vec!["makeIterator", "next"]);

pub fn register_builtins() -> Vec<StdlibEntry> {
    BUILTIN_TYPES
        .iter()
        .map(|(name, ty)| StdlibEntry { name, type_name: ty })
        .chain([
            StdlibEntry {
                name: "print",
                type_name: "Void",
            },
            StdlibEntry {
                name: "Iterator",
                type_name: "Iterator",
            },
            StdlibEntry {
                name: "readLine",
                type_name: "String",
            },
        ])
        .collect()
}

pub fn all_host_fn_names() -> Vec<&'static str> {
    HostFnId::all()
        .iter()
        .map(|id| id.name())
        .chain(numeric_host_names())
        .chain(collection_host_names())
        .chain(bigint_host_names())
        .collect()
}

/// Whether a type name is a builtin registered in the stdlib.
pub fn is_builtin_type(name: &str) -> bool {
    BUILTIN_TYPES.contains_key(name)
}

/// Phase 65: numeric builtin types that cannot declare operator overloads.
pub fn is_numeric_builtin_type(name: &str) -> bool {
    matches!(
        name,
        "Int" | "Int8" | "Int16" | "Int32" | "Int64" | "Int128"
            | "UInt" | "UInt8" | "UInt16" | "UInt32" | "UInt64" | "UInt128"
            | "Float" | "Float32" | "Float64" | "Double" | "BigInt" | "Bool"
    )
}

/// Phase 65: collection / text builtin types that cannot declare operator overloads.
pub fn is_collection_builtin_type(name: &str) -> bool {
    matches!(name, "Array" | "Set" | "Dictionary" | "String" | "Object")
}

/// Phase 65: scalar text builtins (operator S044).
pub fn is_text_builtin_type(name: &str) -> bool {
    matches!(name, "String" | "Character")
}

/// Phase 65: types that must not define `operator func` (S043–S048).
pub fn operator_overload_forbidden_type(name: &str) -> bool {
    is_numeric_builtin_type(name) || is_collection_builtin_type(name)
}

/// Phase 42: type names for `lib/internal/manifest.json` sync (single source: `BUILTIN_TYPES`).
pub fn manifest_type_names() -> Vec<&'static str> {
    BUILTIN_TYPES.keys().copied().collect()
}

/// Whether a type name satisfies Hashable for Set/Dictionary keys.
pub fn is_hashable_type(name: &str) -> bool {
    is_hashable_type_name(name)
}
