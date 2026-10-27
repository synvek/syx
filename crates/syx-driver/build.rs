//! Phase 44/45: regenerate Core numeric builtins + manifest from taxonomy.
//!
//! Hand-authored authority (do **not** overwrite from this script):
//! - `packages/core/builtins/{scalar,special,collections,text,reference,algebraic}/*.sy`
//!   except generated `numeric/*.sy` (from `_templates/`)
//! - `packages/core/protocols/*.sy`
//! - `packages/io/types/http_response.sy`
//! - `packages/internal/types.sy`
//!
//! Numeric bodies come from `packages/core/builtins/_templates/*.sy.in`.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn templates_dir() -> PathBuf {
    repo_root().join("packages/core/builtins/_templates")
}

fn write_if_changed(path: &Path, content: &str) {
    if path.exists() {
        if fs::read_to_string(path).ok().as_deref() == Some(content) {
            return;
        }
    } else if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create dir");
    }
    fs::write(path, content).expect("write generated stdlib file");
}

fn load_template(rel: &str) -> String {
    let path = templates_dir().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!("missing builtin template {}: {e}", path.display())
    })
}

fn render(template: &str, vars: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (key, value) in vars {
        let needle = format!("{{{{{key}}}}}");
        out = out.replace(&needle, value);
    }
    out
}

fn rerun_if_templates_changed() {
    let dir = templates_dir();
    println!("cargo:rerun-if-changed={}", dir.display());
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            println!("cargo:rerun-if-changed={}", path.display());
            if path.is_dir() {
                if let Ok(inner) = fs::read_dir(&path) {
                    for f in inner.flatten() {
                        println!("cargo:rerun-if-changed={}", f.path().display());
                    }
                }
            }
        }
    }
}

struct NumSpec {
    surface: &'static str,
    internal_id: &'static str,
    numeric_kind: &'static str,
    file: &'static str,
    bits: u32,
    signed: bool,
    wide: bool,
    alias_of: Option<&'static str>,
    delegate: Option<&'static str>,
}

const NUMERICS: &[NumSpec] = &[
    NumSpec { surface: "Int8", internal_id: "numeric.i8", numeric_kind: "I8", file: "int8", bits: 8, signed: true, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "Int16", internal_id: "numeric.i16", numeric_kind: "I16", file: "int16", bits: 16, signed: true, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "Int32", internal_id: "numeric.i32", numeric_kind: "I32", file: "int32", bits: 32, signed: true, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "Int64", internal_id: "numeric.i64", numeric_kind: "I64", file: "int64", bits: 64, signed: true, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "Int128", internal_id: "numeric.i128", numeric_kind: "I128", file: "int128", bits: 128, signed: true, wide: true, alias_of: None, delegate: None },
    NumSpec { surface: "UInt8", internal_id: "numeric.u8", numeric_kind: "U8", file: "uint8", bits: 8, signed: false, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "UInt16", internal_id: "numeric.u16", numeric_kind: "U16", file: "uint16", bits: 16, signed: false, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "UInt32", internal_id: "numeric.u32", numeric_kind: "U32", file: "uint32", bits: 32, signed: false, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "UInt64", internal_id: "numeric.u64", numeric_kind: "U64", file: "uint64", bits: 64, signed: false, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "UInt128", internal_id: "numeric.u128", numeric_kind: "U128", file: "uint128", bits: 128, signed: false, wide: true, alias_of: None, delegate: None },
    NumSpec { surface: "Int", internal_id: "numeric.int", numeric_kind: "Int", file: "int", bits: 64, signed: true, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "UInt", internal_id: "numeric.uint", numeric_kind: "UInt", file: "uint", bits: 64, signed: false, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "Float32", internal_id: "numeric.f32", numeric_kind: "F32", file: "float32", bits: 32, signed: false, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "Float64", internal_id: "numeric.f64", numeric_kind: "F64", file: "float64", bits: 64, signed: false, wide: false, alias_of: None, delegate: None },
    NumSpec { surface: "Float", internal_id: "numeric.f32", numeric_kind: "F32", file: "float", bits: 32, signed: false, wide: false, alias_of: Some("Float32"), delegate: Some("Float32") },
    NumSpec { surface: "Double", internal_id: "numeric.f64", numeric_kind: "F64", file: "double", bits: 64, signed: false, wide: false, alias_of: Some("Float64"), delegate: Some("Float64") },
];

/// Protocols declared on comparable value builtins (matches manifest `auto_conformance`).
const COMPARABLE_VALUE_PROTOCOLS: &str = "Equatable, Hashable, CustomStringConvertible, Comparable";

fn host_slug(surface: &str) -> Option<&'static str> {
    Some(match surface {
        "Int8" => "i8",
        "Int16" => "i16",
        "Int32" => "i32",
        "Int64" => "i64",
        "Int128" => "i128",
        "UInt8" => "u8",
        "UInt16" => "u16",
        "UInt32" => "u32",
        "UInt64" => "u64",
        "UInt128" => "u128",
        "Int" => "int",
        "UInt" => "uint",
        "Float32" => "f32",
        "Float64" => "f64",
        _ => return None,
    })
}

fn numeric_host_ops(spec: &NumSpec) -> Vec<&'static str> {
    // Surface no longer exposes binary min/max (use Math.min / __syx_*_min, or Type.min/max bounds).
    // HostFn table still registers __syx_*_min/max for Math / clamp / linalg.
    let is_float = spec.surface.contains("Float") || spec.surface == "Double";
    let mut ops = Vec::new();
    if spec.signed || is_float {
        ops.push("abs");
    }
    if spec.surface == "Int" {
        ops.push("add");
    }
    ops
}

fn value_witness_extension(ty: &str, slug: Option<&str>) -> String {
    let compare = match slug {
        Some(slug) => render(
            &load_template("fragments/compare.sy.in"),
            &[("TY", ty), ("SLUG", slug)],
        ),
        None => String::new(),
    };
    render(
        &load_template("value_witness.sy.in"),
        &[("TY", ty), ("COMPARE", &compare)],
    )
}

/// Zero literal for `zero()` / `sign()` (contextual typing fills platform `UInt`).
fn numeric_zero_lit(spec: &NumSpec) -> &'static str {
    let is_float = spec.surface.contains("Float") || spec.surface == "Double";
    if is_float {
        return if spec.bits == 32 { "0.0f32" } else { "0.0" };
    }
    match (spec.bits, spec.signed, spec.surface) {
        (_, _, "Int") | (_, _, "UInt") => "0",
        (8, true, _) => "0i8",
        (16, true, _) => "0i16",
        (32, true, _) => "0i32",
        (64, true, _) => "0i64",
        (128, true, _) => "0i128",
        (8, false, _) => "0u8",
        (16, false, _) => "0u16",
        (32, false, _) => "0u32",
        (64, false, _) => "0u64",
        (128, false, _) => "0u128",
        _ => "0",
    }
}

/// Body for type-bound `min` / `max` properties (Swift FixedWidthInteger style).
/// Float bounds omitted: scientific literals are lossy; need bit-pattern HostFn later.
enum NumericBoundBody {
    Expr(&'static str),
    /// Multi-statement getter body (no outer braces); used when wraparound needs locals.
    Block(&'static str),
}

fn numeric_bound_bodies(spec: &NumSpec) -> Option<(NumericBoundBody, NumericBoundBody)> {
    let is_float = spec.surface.contains("Float") || spec.surface == "Double";
    if is_float {
        return None;
    }
    Some(match (spec.bits, spec.signed, spec.surface) {
        (8, true, _) => (
            NumericBoundBody::Expr("-128i8"),
            NumericBoundBody::Expr("127i8"),
        ),
        (16, true, _) => (
            NumericBoundBody::Expr("-32768i16"),
            NumericBoundBody::Expr("32767i16"),
        ),
        (32, true, _) => (
            NumericBoundBody::Expr("-2147483648i32"),
            NumericBoundBody::Expr("2147483647i32"),
        ),
        (64, true, "Int") => (
            NumericBoundBody::Expr("-9223372036854775808"),
            NumericBoundBody::Expr("9223372036854775807"),
        ),
        (64, true, _) => (
            NumericBoundBody::Expr("-9223372036854775808i64"),
            NumericBoundBody::Expr("9223372036854775807i64"),
        ),
        // Const `1i128 << 127` currently lowers Shl as kind=Int (truncates); use locals.
        (128, true, _) => (
            NumericBoundBody::Block(
                "            let one: Int128 = 1i128\n            return one << 127\n",
            ),
            NumericBoundBody::Block(
                "            let one: Int128 = 1i128\n            let mn: Int128 = one << 127\n            return mn - 1i128\n",
            ),
        ),
        (8, false, _) => (NumericBoundBody::Expr("0u8"), NumericBoundBody::Expr("255u8")),
        (16, false, _) => (NumericBoundBody::Expr("0u16"), NumericBoundBody::Expr("65535u16")),
        (32, false, _) => (NumericBoundBody::Expr("0u32"), NumericBoundBody::Expr("4294967295u32")),
        (64, false, "UInt") => (
            NumericBoundBody::Expr("0"),
            // `as UInt` traps at runtime; wrap 0-1 with contextual UInt locals.
            NumericBoundBody::Block(
                "            let z: UInt = 0\n            let one: UInt = 1\n            return z - one\n",
            ),
        ),
        (64, false, _) => (
            NumericBoundBody::Expr("0u64"),
            NumericBoundBody::Expr("18446744073709551615u64"),
        ),
        (128, false, _) => (
            NumericBoundBody::Expr("0u128"),
            NumericBoundBody::Block(
                "            let z: UInt128 = 0u128\n            let one: UInt128 = 1u128\n            return z - one\n",
            ),
        ),
        _ => return None,
    })
}

fn emit_bound_getter(ty: &str, name: &str, doc: &str, body: NumericBoundBody) -> String {
    match body {
        // Simple constant → `static let` (desugars to `@static __get_`).
        NumericBoundBody::Expr(expr) => render(
            &load_template("fragments/bound_let.sy.in"),
            &[("DOC", doc), ("NAME", name), ("TY", ty), ("EXPR", expr)],
        ),
        // Multi-statement → `static var { get }` (same desugar / Member path).
        NumericBoundBody::Block(block) => render(
            &load_template("fragments/bound_get.sy.in"),
            &[("DOC", doc), ("NAME", name), ("TY", ty), ("BLOCK", block)],
        ),
    }
}

fn numeric_epsilon_lit(spec: &NumSpec) -> Option<&'static str> {
    match spec.surface {
        // 2^-23 / 2^-52 written as decimals (e-notation literals are lossy today).
        "Float32" | "Float" => Some("0.00000011920928955078125f32"),
        "Float64" | "Double" => Some("0.0000000000000002220446049250313"),
        _ => None,
    }
}

fn numeric_type_doc(spec: &NumSpec) -> String {
    if let Some(canon) = spec.alias_of {
        return format!(
            "Alias of `{canon}` ({bits}-bit). Prelude; methods forward to `{canon}`.",
            canon = canon,
            bits = spec.bits,
        );
    }
    let kind = if spec.surface.contains("Float") || spec.surface == "Double" {
        "IEEE-754 floating-point"
    } else if spec.signed {
        "signed integer"
    } else {
        "unsigned integer"
    };
    format!(
        "{bits}-bit {kind} (`{ty}`). Prelude; host ops via `__syx_{slug}_*`.",
        bits = spec.bits,
        kind = kind,
        ty = spec.surface,
        slug = host_slug(spec.surface).unwrap_or("?"),
    )
}

// Hand-authored authority under packages/core/builtins (Array/String/Set/Dictionary/
// Deque/Bool/Character/Void/globals/Object*/Result/BigInt/OrderedSet) and
// packages/core/protocols — do not regenerate those from Rust strings.

fn numeric_builtin(spec: &NumSpec) -> String {
    let host_surface = spec.delegate.unwrap_or(spec.surface);
    let slug = host_slug(host_surface).expect("canonical numeric surface");
    let doc = numeric_type_doc(spec);
    let witness = value_witness_extension(spec.surface, Some(slug));

    if let Some(delegate) = spec.delegate {
        return render(
            &load_template("numeric_alias.sy.in"),
            &[
                ("DOC", &doc),
                ("ID", spec.internal_id),
                ("TY", spec.surface),
                ("PROTOCOLS", COMPARABLE_VALUE_PROTOCOLS),
                ("DELEGATE", delegate),
                ("WITNESS", &witness),
            ],
        );
    }

    let ty = spec.surface;
    let is_float = ty.contains("Float") || ty == "Double";
    let zero = numeric_zero_lit(spec);

    let mut bounds = String::new();
    if let Some((min_b, max_b)) = numeric_bound_bodies(spec) {
        bounds.push_str(&emit_bound_getter(
            ty,
            "min",
            "Smallest representable value (type bound, not binary min).",
            min_b,
        ));
        bounds.push_str(&emit_bound_getter(
            ty,
            "max",
            "Largest representable value (type bound, not binary max).",
            max_b,
        ));
    }

    let epsilon = match numeric_epsilon_lit(spec) {
        Some(eps) => render(
            &load_template("fragments/epsilon.sy.in"),
            &[("TY", ty), ("EPS", eps)],
        ),
        None => String::new(),
    };

    let mut abs_sign = String::new();
    if spec.signed && !is_float {
        abs_sign.push_str(&render(
            &load_template("fragments/abs_signed.sy.in"),
            &[("TY", ty), ("SLUG", slug)],
        ));
        abs_sign.push_str(&render(
            &load_template("fragments/sign.sy.in"),
            &[("TY", ty), ("ZERO", zero)],
        ));
    }
    if is_float {
        abs_sign.push_str(&render(
            &load_template("fragments/abs_float.sy.in"),
            &[("TY", ty), ("SLUG", slug)],
        ));
    }

    let bits = spec.bits.to_string();
    let bytes = (spec.bits / 8).to_string();
    let widths_clamp = render(
        &load_template("fragments/widths_clamp.sy.in"),
        &[
            ("BITS", &bits),
            ("BYTES", &bytes),
            ("TY", ty),
            ("ZERO", zero),
            ("SLUG", slug),
        ],
    );

    let wide = if spec.wide {
        load_template("fragments/wide_leading_zeros.sy.in")
    } else {
        String::new()
    };

    render(
        &load_template("numeric_canonical.sy.in"),
        &[
            ("DOC", &doc),
            ("ID", spec.internal_id),
            ("TY", ty),
            ("PROTOCOLS", COMPARABLE_VALUE_PROTOCOLS),
            ("SLUG", slug),
            ("BOUNDS", &bounds),
            ("EPSILON", &epsilon),
            ("ABS_SIGN", &abs_sign),
            ("WIDTHS_CLAMP", &widths_clamp),
            ("WIDE", &wide),
            ("WITNESS", &witness),
        ],
    )
}

fn manifest_entry_json(
    surface: &str,
    kind: &str,
    canonical: &str,
    alias_of: Option<&str>,
    internal_id: &str,
    core_path: &str,
    numeric_kind: Option<&str>,
    prelude: bool,
    importable: bool,
    owner_module: Option<&str>,
    auto_conformance: Option<&[&str]>,
    host_ops: Option<&[&str]>,
) -> String {
    let alias = alias_of
        .map(|a| format!("\"{a}\""))
        .unwrap_or_else(|| "null".to_string());
    let nk = numeric_kind
        .map(|n| format!("\"{n}\""))
        .unwrap_or_else(|| "null".to_string());
    let owner = owner_module
        .map(|o| format!("\"{o}\""))
        .unwrap_or_else(|| "null".to_string());
    let auto = auto_conformance
        .map(|items| {
            let inner: Vec<String> = items.iter().map(|i| format!("\"{i}\"")).collect();
            format!("[{}]", inner.join(", "))
        })
        .unwrap_or_else(|| "null".to_string());
    let hops = host_ops
        .map(|items| {
            let inner: Vec<String> = items.iter().map(|i| format!("\"{i}\"")).collect();
            format!("[{}]", inner.join(", "))
        })
        .unwrap_or_else(|| "null".to_string());
    format!(
        "    {{\n\
           \"surface\": \"{surface}\",\n\
           \"kind\": \"{kind}\",\n\
           \"canonical\": \"{canonical}\",\n\
           \"alias_of\": {alias},\n\
           \"internal_id\": \"{internal_id}\",\n\
           \"core_path\": \"{core_path}\",\n\
           \"numeric_kind\": {nk},\n\
           \"prelude\": {prelude},\n\
           \"importable\": {importable},\n\
           \"owner_module\": {owner},\n\
           \"auto_conformance\": {auto},\n\
           \"host_ops\": {hops}\n\
         }}"
    )
}

fn manifest_json(prelude_modules: &[&str]) -> String {
    let mut entries: Vec<String> = Vec::new();
    for spec in NUMERICS {
        let canonical = spec.alias_of.unwrap_or(spec.surface);
        let ops: Vec<&str> = if spec.delegate.is_some() {
            vec![]
        } else {
            numeric_host_ops(spec)
        };
        let hops = if ops.is_empty() {
            None
        } else {
            Some(ops.as_slice())
        };
        entries.push(manifest_entry_json(
            spec.surface,
            "numeric",
            canonical,
            spec.alias_of,
            spec.internal_id,
            &format!("core/builtins/numeric/{}.sy", spec.file),
            Some(spec.numeric_kind),
            true,
            false,
            None,
            None,
            hops,
        ));
    }
    let proto_auto = Some(&["numeric", "scalar", "text", "collection"][..]);
    let value_auto = Some(&["Equatable", "Hashable", "CustomStringConvertible"][..]);
    let comparable_value_auto = Some(&["Equatable", "Hashable", "CustomStringConvertible", "Comparable"][..]);
    let comparable_auto = Some(&["numeric", "text"][..]);
    let character_auto = Some(&["Equatable", "Hashable", "CustomStringConvertible", "Comparable"][..]);
    entries.push(manifest_entry_json("Bool", "scalar", "Bool", None, "scalar.bool", "core/builtins/scalar/bool.sy", None, true, false, None, value_auto, None));
    entries.push(manifest_entry_json("Character", "scalar", "Character", None, "scalar.character", "core/builtins/scalar/character.sy", None, true, false, None, character_auto, None));
    entries.push(manifest_entry_json("Void", "bottom", "Void", None, "special.void", "core/builtins/special/void.sy", None, false, false, None, None, None));
    entries.push(manifest_entry_json("String", "text", "String", None, "text.string", "core/builtins/text/string.sy", None, true, false, None, comparable_value_auto, None));
    entries.push(manifest_entry_json("Array", "collection", "Array", None, "collection.array", "core/builtins/collections/array.sy", None, true, false, None, value_auto, Some(&["map", "filter", "reduce", "reverse", "contains", "joined", "count", "append", "sortBy", "binarySearch", "chunked", "zip", "flatMap"][..])));
    entries.push(manifest_entry_json("Set", "collection", "Set", None, "collection.set", "core/builtins/collections/set.sy", None, true, false, None, value_auto, Some(&["count", "contains", "insert", "remove"][..])));
    entries.push(manifest_entry_json("Dictionary", "collection", "Dictionary", None, "collection.dictionary", "core/builtins/collections/dictionary.sy", None, true, false, None, value_auto, Some(&["count", "contains", "keys", "values"][..])));
    entries.push(manifest_entry_json("OrderedSet", "collection", "OrderedSet", None, "collection.ordered_set", "core/builtins/collections/ordered_set.sy", None, true, false, None, None, Some(&["new", "insert", "contains"][..])));
    entries.push(manifest_entry_json("Deque", "collection", "Deque", None, "collection.deque", "core/builtins/collections/deque.sy", None, true, false, None, None, Some(&["new", "pushBack", "pushFront", "popBack", "popFront", "count", "isEmpty"][..])));
    entries.push(manifest_entry_json("Object", "reference", "Object", None, "reference.object", "core/builtins/reference/object.sy", None, false, false, None, value_auto, None));
    entries.push(manifest_entry_json("ObjectIdentifier", "reference", "ObjectIdentifier", None, "reference.object_identifier", "core/builtins/reference/object_identifier.sy", None, false, false, None, value_auto, None));
    entries.push(manifest_entry_json("Equatable", "protocol", "Equatable", None, "protocol.equatable", "core/protocols/equatable.sy", None, false, false, None, proto_auto, None));
    entries.push(manifest_entry_json("Hashable", "protocol", "Hashable", None, "protocol.hashable", "core/protocols/hashable.sy", None, false, false, None, proto_auto, None));
    entries.push(manifest_entry_json("Comparable", "protocol", "Comparable", None, "protocol.comparable", "core/protocols/comparable.sy", None, false, false, None, comparable_auto, None));
    entries.push(manifest_entry_json("CustomStringConvertible", "protocol", "CustomStringConvertible", None, "protocol.custom_string_convertible", "core/protocols/custom_string_convertible.sy", None, false, false, None, proto_auto, None));
    entries.push(manifest_entry_json("Encoder", "protocol", "Encoder", None, "protocol.encoder", "core/protocols/encoder.sy", None, false, false, None, None, None));
    entries.push(manifest_entry_json("Decoder", "protocol", "Decoder", None, "protocol.decoder", "core/protocols/decoder.sy", None, false, false, None, None, None));
    entries.push(manifest_entry_json("Encodable", "protocol", "Encodable", None, "protocol.encodable", "core/protocols/encodable.sy", None, false, false, None, None, None));
    entries.push(manifest_entry_json("Decodable", "protocol", "Decodable", None, "protocol.decodable", "core/protocols/decodable.sy", None, false, false, None, None, None));
    entries.push(manifest_entry_json("Codable", "protocol", "Codable", None, "protocol.codable", "core/protocols/codable.sy", None, false, false, None, None, None));
    entries.push(manifest_entry_json("Result", "algebraic", "Result", None, "algebraic.result", "core/builtins/algebraic/result.sy", None, true, false, None, value_auto, Some(&["ok", "err", "is_ok", "is_err"][..])));
    entries.push(manifest_entry_json("BigInt", "numeric", "BigInt", None, "numeric.bigint", "core/builtins/numeric/bigint.sy", Some("bigint"), true, false, None, comparable_value_auto, Some(&["from_string", "from_i128", "add", "sub", "mul", "cmp", "description"][..])));
    entries.push(manifest_entry_json("HTTPResponse", "host_type", "HTTPResponse", None, "host.http_response", "io/types/http_response.sy", None, false, false, Some("Net"), None, None));

    let prelude_json: String = prelude_modules
        .iter()
        .map(|t| format!("\"{t}\""))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "{{\n\
           \"phase\": 139,\n\
           \"version\": \"117.0\",\n\
           \"source\": \"crates/syx-driver/build.rs + builtin_kind\",\n\
           \"entries\": [\n{}\n  ],\n\
           \"prelude_modules\": [{prelude_json}],\n\
           \"importable_modules\": [\"IO\", \"Net\", \"Crypto\", \"Text\", \"DateTime\", \"Sync\", \"Process\", \"Testing\", \"Log\", \"Di\", \"UI\", \"Os\", \"Sql\", \"Linalg\"],\n\
           \"forbidden_imports\": [\"Internal\", \"_Builtin\", \"_Internal\"],\n\
           \"retired_modules\": [\"Int8Ops\", \"Text\", \"Arrays\", \"BoolOps\"]\n\
         }}\n",
        entries.join(",\n")
    )
}

fn remove_legacy_files(core: &Path, internal: &Path) {
    let legacy_ops = [
        "int8ops", "int16ops", "int32ops", "int64ops", "int128ops",
        "uint8ops", "uint16ops", "uint32ops", "uint64ops", "uint128ops",
        "intops", "uintops", "float32ops", "float64ops", "floatops", "doubleops",
        "boolops", "characterops", "text", "arrays", "string_ext",
    ];
    for name in legacy_ops {
        let p = core.join(format!("{name}.sy"));
        if p.exists() {
            let _ = fs::remove_file(p);
        }
    }
    for spec in NUMERICS {
        let p = internal.join(format!("{}.sy", spec.file));
        if p.exists() {
            let _ = fs::remove_file(p);
        }
    }
    let string_internal = internal.join("string.sy");
    if string_internal.exists() {
        let _ = fs::remove_file(string_internal);
    }
}

fn main() {
    let root = repo_root();
    let core = root.join("packages/core");
    let internal = root.join("packages/internal");
    let numeric_dir = core.join("builtins/numeric");

    println!("cargo:rerun-if-changed=build.rs");
    rerun_if_templates_changed();

    remove_legacy_files(&core, &internal);

    // Hand-authored (not written here): scalar/*, special/*, collections/* (incl. OrderedSet),
    // text/string.sy, reference/*, algebraic/result.sy, numeric/bigint.sy, protocols/*,
    // packages/io/types/http_response.sy, packages/internal/types.sy.

    let mut prelude: Vec<&'static str> = vec!["Math"];
    for spec in NUMERICS {
        prelude.push(spec.surface);
        let path = numeric_dir.join(format!("{}.sy", spec.file));
        write_if_changed(&path, &numeric_builtin(spec));
    }
    prelude.extend([
        "Bool",
        "Character",
        "String",
        "Array",
        "Set",
        "Dictionary",
        "OrderedSet",
        "Deque",
        "Result",
        "BigInt",
        "Hasher",
        "Random",
        "JSON",
        "Time",
        "Env",
        "Path",
        "Concurrency",
        "SIMD4",
    ]);

    write_if_changed(&internal.join("manifest.json"), &manifest_json(&prelude));
    build_echo_plugin_test_dylib();
}

/// Phase 105: CI must have echo plugin dylib for unload hard-fail gate (not skip).
fn build_echo_plugin_test_dylib() {
    use std::process::Command;
    let root = repo_root();
    let fixture = root.join("crates/syx-host-plugin/tests/fixtures/echo_plugin");
    let include = root.join("crates/syx-host-plugin/include");
    let src = fixture.join("echo_plugin.c");
    if !src.is_file() {
        return;
    }
    let (out_name, mut cmd) = if cfg!(target_os = "macos") {
        let mut c = Command::new("xcrun");
        c.arg("--sdk").arg("macosx").arg("clang");
        ("libecho_plugin.dylib", c)
    } else if cfg!(target_os = "windows") {
        return;
    } else {
        ("libecho_plugin.so", Command::new("cc"))
    };
    let out = fixture.join(out_name);
    if out.is_file() {
        if let (Ok(src_m), Ok(out_m)) = (
            src.metadata().and_then(|m| m.modified()),
            out.metadata().and_then(|m| m.modified()),
        ) {
            if out_m >= src_m {
                return;
            }
        }
    }
    if cfg!(target_os = "macos") {
        cmd.arg("-dynamiclib");
    } else {
        cmd.arg("-shared").arg("-fPIC");
    }
    let status = cmd
        .arg("-I")
        .arg(&include)
        .arg(&src)
        .arg("-o")
        .arg(&out)
        .status()
        .expect("echo plugin dylib build");
    if !status.success() {
        eprintln!(
            "cargo:warning=echo plugin dylib build failed (plugin unload tests need a working C toolchain)"
        );
        return;
    }
    println!("cargo:rerun-if-changed={}", src.display());
    println!("cargo:rerun-if-changed={}", include.join("syx_plugin.h").display());
}
