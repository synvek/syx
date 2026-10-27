//! Phase 44: builtin taxonomy (`BuiltinKind`) and manifest v2 entry metadata.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinKind {
    Numeric,
    Scalar,
    Bottom,
    Text,
    Collection,
    Reference,
    Protocol,
    Algebraic,
    HostType,
}

impl BuiltinKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Numeric => "numeric",
            Self::Scalar => "scalar",
            Self::Bottom => "bottom",
            Self::Text => "text",
            Self::Collection => "collection",
            Self::Reference => "reference",
            Self::Protocol => "protocol",
            Self::Algebraic => "algebraic",
            Self::HostType => "host_type",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "numeric" => Some(Self::Numeric),
            "scalar" => Some(Self::Scalar),
            "bottom" => Some(Self::Bottom),
            "text" => Some(Self::Text),
            "collection" => Some(Self::Collection),
            "reference" => Some(Self::Reference),
            "protocol" => Some(Self::Protocol),
            "algebraic" => Some(Self::Algebraic),
            "host_type" => Some(Self::HostType),
            _ => None,
        }
    }
}

/// Explicit witness protocols on value builtins (matches Core class conformances).
const VALUE_WITNESS: &[&str] = &["Equatable", "Hashable", "CustomStringConvertible"];
/// Protocol auto_conformance kind groups (manifest protocol entries).
const PROTO_KIND_GROUPS: &[&str] = &["numeric", "scalar", "text", "collection"];
const COMPARABLE_KIND_GROUPS: &[&str] = &["numeric", "text"];
const HOST_MIN_MAX: &[&str] = &["min", "max"];
const HOST_MIN_MAX_ABS: &[&str] = &["min", "max", "abs"];
const HOST_ARRAY_OPS: &[&str] = &["map", "filter", "reduce", "reverse", "contains", "joined", "count"];
const HOST_SET_OPS: &[&str] = &["count", "contains", "insert", "remove", "union", "intersection"];
const HOST_DICT_OPS: &[&str] = &["count", "contains", "containsKey", "keys", "values", "remove"];
const HOST_RESULT_OPS: &[&str] = &["ok", "err", "is_ok", "is_err"];
const HOST_BIGINT_OPS: &[&str] = &["from_string", "from_i128", "add", "sub", "mul", "cmp", "description"];

#[derive(Debug, Clone, Copy)]
pub struct ManifestEntry {
    pub surface: &'static str,
    pub kind: BuiltinKind,
    pub canonical: &'static str,
    pub alias_of: Option<&'static str>,
    pub internal_id: &'static str,
    pub core_path: &'static str,
    pub numeric_kind: Option<&'static str>,
    pub prelude: bool,
    pub importable: bool,
    pub owner_module: Option<&'static str>,
    pub auto_conformance: Option<&'static [&'static str]>,
    pub host_ops: Option<&'static [&'static str]>,
}

struct NumMeta {
    surface: &'static str,
    internal_id: &'static str,
    numeric_kind: &'static str,
    core_path: &'static str,
    bits: u32,
    signed: bool,
    wide: bool,
    alias_of: Option<&'static str>,
}

const NUMERICS: &[NumMeta] = &[
    NumMeta { surface: "Int8", internal_id: "numeric.i8", numeric_kind: "I8", core_path: "core/builtins/numeric/int8.sy", bits: 8, signed: true, wide: false, alias_of: None },
    NumMeta { surface: "Int16", internal_id: "numeric.i16", numeric_kind: "I16", core_path: "core/builtins/numeric/int16.sy", bits: 16, signed: true, wide: false, alias_of: None },
    NumMeta { surface: "Int32", internal_id: "numeric.i32", numeric_kind: "I32", core_path: "core/builtins/numeric/int32.sy", bits: 32, signed: true, wide: false, alias_of: None },
    NumMeta { surface: "Int64", internal_id: "numeric.i64", numeric_kind: "I64", core_path: "core/builtins/numeric/int64.sy", bits: 64, signed: true, wide: false, alias_of: None },
    NumMeta { surface: "Int128", internal_id: "numeric.i128", numeric_kind: "I128", core_path: "core/builtins/numeric/int128.sy", bits: 128, signed: true, wide: true, alias_of: None },
    NumMeta { surface: "UInt8", internal_id: "numeric.u8", numeric_kind: "U8", core_path: "core/builtins/numeric/uint8.sy", bits: 8, signed: false, wide: false, alias_of: None },
    NumMeta { surface: "UInt16", internal_id: "numeric.u16", numeric_kind: "U16", core_path: "core/builtins/numeric/uint16.sy", bits: 16, signed: false, wide: false, alias_of: None },
    NumMeta { surface: "UInt32", internal_id: "numeric.u32", numeric_kind: "U32", core_path: "core/builtins/numeric/uint32.sy", bits: 32, signed: false, wide: false, alias_of: None },
    NumMeta { surface: "UInt64", internal_id: "numeric.u64", numeric_kind: "U64", core_path: "core/builtins/numeric/uint64.sy", bits: 64, signed: false, wide: false, alias_of: None },
    NumMeta { surface: "UInt128", internal_id: "numeric.u128", numeric_kind: "U128", core_path: "core/builtins/numeric/uint128.sy", bits: 128, signed: false, wide: true, alias_of: None },
    NumMeta { surface: "Int", internal_id: "numeric.int", numeric_kind: "Int", core_path: "core/builtins/numeric/int.sy", bits: 64, signed: true, wide: false, alias_of: None },
    NumMeta { surface: "UInt", internal_id: "numeric.uint", numeric_kind: "UInt", core_path: "core/builtins/numeric/uint.sy", bits: 64, signed: false, wide: false, alias_of: None },
    NumMeta { surface: "Float32", internal_id: "numeric.f32", numeric_kind: "F32", core_path: "core/builtins/numeric/float32.sy", bits: 32, signed: false, wide: false, alias_of: None },
    NumMeta { surface: "Float64", internal_id: "numeric.f64", numeric_kind: "F64", core_path: "core/builtins/numeric/float64.sy", bits: 64, signed: false, wide: false, alias_of: None },
    NumMeta { surface: "Float", internal_id: "numeric.f32", numeric_kind: "F32", core_path: "core/builtins/numeric/float.sy", bits: 32, signed: false, wide: false, alias_of: Some("Float32") },
    NumMeta { surface: "Double", internal_id: "numeric.f64", numeric_kind: "F64", core_path: "core/builtins/numeric/double.sy", bits: 64, signed: false, wide: false, alias_of: Some("Float64") },
];

/// Static manifest entries for Phase 46 (must stay in sync with `build.rs` output).
pub fn manifest_entries() -> Vec<ManifestEntry> {
    let mut entries = Vec::new();
    for n in NUMERICS {
        let is_float = n.surface.contains("Float") || n.surface == "Double";
        entries.push(ManifestEntry {
            surface: n.surface,
            kind: BuiltinKind::Numeric,
            canonical: n.alias_of.unwrap_or(n.surface),
            alias_of: n.alias_of,
            internal_id: n.internal_id,
            core_path: n.core_path,
            numeric_kind: Some(n.numeric_kind),
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: None,
            host_ops: Some(if n.signed && !is_float {
                HOST_MIN_MAX_ABS
            } else {
                HOST_MIN_MAX
            }),
        });
    }
    entries.extend([
        ManifestEntry {
            surface: "Bool",
            kind: BuiltinKind::Scalar,
            canonical: "Bool",
            alias_of: None,
            internal_id: "scalar.bool",
            core_path: "core/builtins/scalar/bool.sy",
            numeric_kind: None,
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: Some(VALUE_WITNESS),
            host_ops: None,
        },
        ManifestEntry {
            surface: "Character",
            kind: BuiltinKind::Scalar,
            canonical: "Character",
            alias_of: None,
            internal_id: "scalar.character",
            core_path: "core/builtins/scalar/character.sy",
            numeric_kind: None,
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: Some(&["Equatable", "Hashable", "CustomStringConvertible", "Comparable"]),
            host_ops: None,
        },
        ManifestEntry {
            surface: "Void",
            kind: BuiltinKind::Bottom,
            canonical: "Void",
            alias_of: None,
            internal_id: "special.void",
            core_path: "core/builtins/special/void.sy",
            numeric_kind: None,
            prelude: false,
            importable: false,
            owner_module: None,
            auto_conformance: None,
            host_ops: None,
        },
        ManifestEntry {
            surface: "String",
            kind: BuiltinKind::Text,
            canonical: "String",
            alias_of: None,
            internal_id: "text.string",
            core_path: "core/builtins/text/string.sy",
            numeric_kind: None,
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: Some(&["Equatable", "Hashable", "CustomStringConvertible", "Comparable"]),
            host_ops: None,
        },
        ManifestEntry {
            surface: "Array",
            kind: BuiltinKind::Collection,
            canonical: "Array",
            alias_of: None,
            internal_id: "collection.array",
            core_path: "core/builtins/collections/array.sy",
            numeric_kind: None,
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: Some(VALUE_WITNESS),
            host_ops: Some(HOST_ARRAY_OPS),
        },
        ManifestEntry {
            surface: "OrderedSet",
            kind: BuiltinKind::Collection,
            canonical: "OrderedSet",
            alias_of: None,
            internal_id: "collection.ordered_set",
            core_path: "core/builtins/collections/ordered_set.sy",
            numeric_kind: None,
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: None,
            host_ops: None,
        },
        ManifestEntry {
            surface: "Deque",
            kind: BuiltinKind::Collection,
            canonical: "Deque",
            alias_of: None,
            internal_id: "collection.deque",
            core_path: "core/builtins/collections/deque.sy",
            numeric_kind: None,
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: None,
            // HostFn registry (not COLLECTION_HOST_TABLE) — parity gate skips None.
            host_ops: None,
        },
        ManifestEntry {
            surface: "Set",
            kind: BuiltinKind::Collection,
            canonical: "Set",
            alias_of: None,
            internal_id: "collection.set",
            core_path: "core/builtins/collections/set.sy",
            numeric_kind: None,
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: Some(VALUE_WITNESS),
            host_ops: Some(HOST_SET_OPS),
        },
        ManifestEntry {
            surface: "Dictionary",
            kind: BuiltinKind::Collection,
            canonical: "Dictionary",
            alias_of: None,
            internal_id: "collection.dictionary",
            core_path: "core/builtins/collections/dictionary.sy",
            numeric_kind: None,
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: Some(VALUE_WITNESS),
            host_ops: Some(HOST_DICT_OPS),
        },
        ManifestEntry {
            surface: "Object",
            kind: BuiltinKind::Reference,
            canonical: "Object",
            alias_of: None,
            internal_id: "reference.object",
            core_path: "core/builtins/reference/object.sy",
            numeric_kind: None,
            prelude: false,
            importable: false,
            owner_module: None,
            auto_conformance: Some(VALUE_WITNESS),
            host_ops: None,
        },
        ManifestEntry {
            surface: "ObjectIdentifier",
            kind: BuiltinKind::Reference,
            canonical: "ObjectIdentifier",
            alias_of: None,
            internal_id: "reference.object_identifier",
            core_path: "core/builtins/reference/object_identifier.sy",
            numeric_kind: None,
            prelude: false,
            importable: false,
            owner_module: None,
            auto_conformance: Some(VALUE_WITNESS),
            host_ops: None,
        },
        ManifestEntry {
            surface: "Equatable",
            kind: BuiltinKind::Protocol,
            canonical: "Equatable",
            alias_of: None,
            internal_id: "protocol.equatable",
            core_path: "core/protocols/equatable.sy",
            numeric_kind: None,
            prelude: false,
            importable: false,
            owner_module: None,
            auto_conformance: Some(PROTO_KIND_GROUPS),
            host_ops: None,
        },
        ManifestEntry {
            surface: "Hashable",
            kind: BuiltinKind::Protocol,
            canonical: "Hashable",
            alias_of: None,
            internal_id: "protocol.hashable",
            core_path: "core/protocols/hashable.sy",
            numeric_kind: None,
            prelude: false,
            importable: false,
            owner_module: None,
            auto_conformance: Some(PROTO_KIND_GROUPS),
            host_ops: None,
        },
        ManifestEntry {
            surface: "Comparable",
            kind: BuiltinKind::Protocol,
            canonical: "Comparable",
            alias_of: None,
            internal_id: "protocol.comparable",
            core_path: "core/protocols/comparable.sy",
            numeric_kind: None,
            prelude: false,
            importable: false,
            owner_module: None,
            auto_conformance: Some(COMPARABLE_KIND_GROUPS),
            host_ops: None,
        },
        ManifestEntry {
            surface: "CustomStringConvertible",
            kind: BuiltinKind::Protocol,
            canonical: "CustomStringConvertible",
            alias_of: None,
            internal_id: "protocol.custom_string_convertible",
            core_path: "core/protocols/custom_string_convertible.sy",
            numeric_kind: None,
            prelude: false,
            importable: false,
            owner_module: None,
            auto_conformance: Some(PROTO_KIND_GROUPS),
            host_ops: None,
        },
        ManifestEntry {
            surface: "Codable",
            kind: BuiltinKind::Protocol,
            canonical: "Codable",
            alias_of: None,
            internal_id: "protocol.codable",
            core_path: "core/protocols/codable.sy",
            numeric_kind: None,
            prelude: false,
            importable: false,
            owner_module: None,
            auto_conformance: None,
            host_ops: None,
        },
        ManifestEntry {
            surface: "Result",
            kind: BuiltinKind::Algebraic,
            canonical: "Result",
            alias_of: None,
            internal_id: "algebraic.result",
            core_path: "core/builtins/algebraic/result.sy",
            numeric_kind: None,
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: Some(VALUE_WITNESS),
            host_ops: Some(HOST_RESULT_OPS),
        },
        ManifestEntry {
            surface: "BigInt",
            kind: BuiltinKind::Numeric,
            canonical: "BigInt",
            alias_of: None,
            internal_id: "numeric.bigint",
            core_path: "core/builtins/numeric/bigint.sy",
            numeric_kind: Some("bigint"),
            prelude: true,
            importable: false,
            owner_module: None,
            auto_conformance: Some(&["Equatable", "Hashable", "CustomStringConvertible", "Comparable"]),
            host_ops: Some(HOST_BIGINT_OPS),
        },
        ManifestEntry {
            surface: "HTTPResponse",
            kind: BuiltinKind::HostType,
            canonical: "HTTPResponse",
            alias_of: None,
            internal_id: "host.http_response",
            core_path: "io/types/http_response.sy",
            numeric_kind: None,
            prelude: false,
            importable: false,
            owner_module: Some("Net"),
            auto_conformance: None,
            host_ops: None,
        },
    ]);
    entries
}

pub fn lookup_manifest_entry(surface: &str) -> Option<ManifestEntry> {
    manifest_entries()
        .into_iter()
        .find(|e| e.surface == surface)
}

pub fn prelude_module_names() -> Vec<&'static str> {
    let mut names = vec!["Math"];
    for e in manifest_entries() {
        if e.prelude && e.kind != BuiltinKind::Protocol {
            names.push(e.surface);
        }
    }
    names.extend([
        "Hasher", "Random", "JSON", "Time", "Env", "Path", "Concurrency", "SIMD4",
    ]);
    names
}

pub fn retired_ops_module_names() -> &'static [&'static str] {
    &[
        "Int8Ops", "Int16Ops", "Int32Ops", "Int64Ops", "Int128Ops",
        "UInt8Ops", "UInt16Ops", "UInt32Ops", "UInt64Ops", "UInt128Ops",
        "IntOps", "UIntOps",
        "Float32Ops", "Float64Ops", "FloatOps", "DoubleOps",
        "BoolOps", "CharacterOps", "Text", "Arrays",
    ]
}

pub fn numeric_meta(surface: &str) -> Option<(u32, bool, bool)> {
    NUMERICS
        .iter()
        .find(|n| n.surface == surface)
        .map(|n| (n.bits, n.signed, n.wide))
}

pub fn internal_id_for_surface(surface: &str) -> Option<&'static str> {
    lookup_manifest_entry(surface).map(|e| e.internal_id)
}
