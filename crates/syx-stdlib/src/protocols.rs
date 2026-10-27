use crate::builtin_kind::{lookup_manifest_entry, BuiltinKind, ManifestEntry};
use crate::numeric::from_type_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolId {
    Equatable,
    Hashable,
    Comparable,
    CustomStringConvertible,
    CustomDebugStringConvertible,
}

impl ProtocolId {
    pub fn name(self) -> &'static str {
        match self {
            Self::Equatable => "Equatable",
            Self::Hashable => "Hashable",
            Self::Comparable => "Comparable",
            Self::CustomStringConvertible => "CustomStringConvertible",
            Self::CustomDebugStringConvertible => "CustomDebugStringConvertible",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Equatable" => Some(Self::Equatable),
            "Hashable" => Some(Self::Hashable),
            "Comparable" => Some(Self::Comparable),
            "CustomStringConvertible" => Some(Self::CustomStringConvertible),
            "CustomDebugStringConvertible" => Some(Self::CustomDebugStringConvertible),
            _ => None,
        }
    }
}

fn base_type_name(type_name: &str) -> &str {
    type_name.split('<').next().unwrap_or(type_name)
}

fn entry_conforms_to_protocol(entry: &ManifestEntry, protocol: ProtocolId) -> bool {
    let pname = protocol.name();
    if let Some(ac) = entry.auto_conformance {
        if ac.iter().any(|p| *p == pname) {
            return true;
        }
    }
    if let Some(proto_entry) = lookup_manifest_entry(pname) {
        if let Some(groups) = proto_entry.auto_conformance {
            let kind = entry.kind.as_str();
            if groups.iter().any(|g| *g == kind) {
                return true;
            }
        }
    }
    false
}

fn builtin_conforms(type_name: &str, protocol: ProtocolId) -> bool {
    let type_name = base_type_name(type_name);
    if type_name.starts_with("SIMD") {
        return matches!(
            protocol,
            ProtocolId::Equatable | ProtocolId::Comparable
        );
    }
    match (type_name, protocol) {
        ("Hasher", ProtocolId::CustomStringConvertible) => return true,
        ("Void", _) => return false,
        _ => {}
    }
    let Some(entry) = lookup_manifest_entry(type_name) else {
        return false;
    };
    if entry_conforms_to_protocol(&entry, protocol) {
        return true;
    }
    if protocol == ProtocolId::CustomDebugStringConvertible {
        return matches!(
            entry.kind,
            BuiltinKind::Numeric
                | BuiltinKind::Scalar
                | BuiltinKind::Text
                | BuiltinKind::Collection
                | BuiltinKind::Reference
        );
    }
    false
}

pub fn type_conforms(type_name: &str, protocol: &str) -> bool {
    if protocol == "Codable" || protocol == "Decodable" || protocol == "Encodable" {
        return is_codable_type(type_name);
    }
    let Some(pid) = ProtocolId::from_name(protocol) else {
        return false;
    };
    builtin_conforms(type_name, pid)
}

pub fn is_codable_type(type_name: &str) -> bool {
    let type_name = base_type_name(type_name);
    if from_type_name(type_name).is_some() {
        return true;
    }
    matches!(
        type_name,
        "Bool" | "String" | "Character" | "Array" | "Set" | "Dictionary" | "Object"
    )
}

pub fn is_equatable_type(type_name: &str) -> bool {
    type_conforms(type_name, "Equatable")
}

pub fn is_hashable_type_name(type_name: &str) -> bool {
    type_conforms(type_name, "Hashable")
}

pub fn is_comparable_type(type_name: &str) -> bool {
    type_conforms(type_name, "Comparable")
}

pub fn is_custom_string_convertible(type_name: &str) -> bool {
    type_conforms(type_name, "CustomStringConvertible")
}

pub fn is_custom_debug_string_convertible(type_name: &str) -> bool {
    type_conforms(type_name, "CustomDebugStringConvertible")
}

pub fn known_protocol(name: &str) -> bool {
    ProtocolId::from_name(name).is_some()
        || matches!(
            name,
            "Iterator"
                | "Sequence"
                | "Codable"
                | "Decodable"
                | "Encodable"
                | "Encoder"
                | "Decoder"
                | "Container"
                | "Collection"
        )
}

/// Phase 18: protocol-associated type names for generic where-clause validation.
pub fn protocol_associated_types(protocol: &str) -> Option<&'static [&'static str]> {
    match protocol {
        "Iterator" => Some(&["Element"]),
        "Sequence" => Some(&["Element"]),
        "Container" => Some(&["Item"]),
        "Collection" => Some(&["Element", "Index"]),
        _ => None,
    }
}
