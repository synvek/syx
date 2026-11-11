//! SYX semantic TypeId / Symbol / Scope face.
//! Extracted in S2-I236 (zero semantics).

use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_ast::AccessLevel;
use syx_stdlib::NumericKind;

/// Duplicate binding in the current scope (mapped to S001 by SemanticAnalyzer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeError {
    Duplicate(SmolStr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeId {
    Void,
    Numeric(NumericKind),
    Bool,
    String,
    Array(Box<TypeId>),
    Set(Box<TypeId>),
    Dictionary {
        key: Box<TypeId>,
        value: Box<TypeId>,
    },
    /// Phase 139 L2: product tuple `(T, U)` with optional element labels.
    Tuple {
        elements: Vec<(Option<SmolStr>, TypeId)>,
    },
    Optional(Box<TypeId>),
    Result {
        ok: Box<TypeId>,
        err: Box<TypeId>,
    },
    Named(SmolStr),
    GenericParam(SmolStr),
    GenericFn {
        name: SmolStr,
        type_params: Vec<SmolStr>,
    },
    Reference(Box<TypeId>),
    MutReference(Box<TypeId>),
    /// Opaque `some Protocol` witness (Phase 11).
    OpaqueSome(Box<TypeId>),
    /// Existential `any Protocol` (Phase 11).
    OpaqueAny(Box<TypeId>),
    /// Phase 38: fixed-width SIMD vector (`SIMD4<Int32>`, etc.).
    Simd {
        lanes: u8,
        element: NumericKind,
    },
    Unknown,
}

impl TypeId {
    pub fn platform_int() -> Self {
        // Keep the `Int` alias so diagnostics/hover say `Int`, not `Int64`/`Int32`.
        TypeId::Numeric(NumericKind::Int)
    }

    /// Build a numeric type id, preserving `Int`/`UInt` aliases for display.
    /// Width resolution happens at convertibility / codegen via `resolve_alias`.
    pub fn numeric(kind: NumericKind) -> Self {
        TypeId::Numeric(kind)
    }

    pub fn as_numeric_kind(&self) -> Option<NumericKind> {
        match self {
            TypeId::Numeric(k) => Some(*k),
            _ => None,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, TypeId::Numeric(_))
    }

    pub fn as_simd(&self) -> Option<(u8, NumericKind)> {
        match self {
            TypeId::Simd { lanes, element } => Some((*lanes, *element)),
            _ => None,
        }
    }

    /// Legacy convenience: platform Int.
    pub fn int() -> Self {
        Self::platform_int()
    }

    /// Legacy convenience: Float64.
    pub fn double() -> Self {
        TypeId::Numeric(NumericKind::F64)
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: SmolStr,
    pub ty: TypeId,
    pub mutable: bool,
    pub access: AccessLevel,
}

#[derive(Debug, Default)]
pub struct Scope {
    pub symbols: IndexMap<SmolStr, Symbol>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn define(&mut self, symbol: Symbol) -> Result<(), ScopeError> {
        if self.symbols.contains_key(&symbol.name) {
            return Err(ScopeError::Duplicate(symbol.name));
        }
        self.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup(name)))
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        if self.symbols.contains_key(name) {
            self.symbols.get_mut(name)
        } else {
            self.parent.as_mut().and_then(|p| p.lookup_mut(name))
        }
    }
}
