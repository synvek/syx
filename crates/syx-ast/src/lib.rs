use smol_str::SmolStr;
use syx_diagnostics::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessLevel {
    Public,
    Internal,
    FilePrivate,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpr {
    Named {
        name: SmolStr,
        generics: Vec<TypeExpr>,
    },
    Optional(Box<TypeExpr>),
    Array(Box<TypeExpr>),
    Dictionary {
        key: Box<TypeExpr>,
        value: Box<TypeExpr>,
    },
    Function {
        params: Vec<TypeExpr>,
        returns: Option<Box<TypeExpr>>,
    },
    /// Phase 139 L2: product tuple type `(T, U)` / `(x: Int, y: Int)`.
    Tuple {
        elements: Vec<(Option<SmolStr>, TypeExpr)>,
    },
    Reference(Box<TypeExpr>, bool, Option<SmolStr>),
    /// `some Protocol` — opaque result type (Phase 11).
    OpaqueSome(Box<TypeExpr>),
    /// `any Protocol` — existential type (Phase 11).
    OpaqueAny(Box<TypeExpr>),
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer {
        value: i128,
        suffix: Option<SmolStr>,
    },
    /// Decimal string for `n` suffix literals exceeding `i128` (Phase 48).
    BigInt(String),
    Double {
        value: f64,
        suffix: Option<SmolStr>,
    },
    String(String),
    Interpolated(Vec<StringPart>),
    Regex(String),
    /// Dictionary literal entries: `(key_expr, value_expr)`; keys lowered at compile time.
    Dict(Vec<(Expr, Expr)>),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    Text(String),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundAssignOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    IdenticalEq,
    IdenticalNe,
    Lt,
    LtEq,
    Gt,
    GtEq,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    And,
    Or,
    NilCoalesce,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
    Try,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal {
        span: Span,
        value: Literal,
    },
    Ident {
        span: Span,
        name: SmolStr,
    },
    /// Phase 65: `super` reference for superclass dispatch.
    Super {
        span: Span,
    },
    Binary {
        span: Span,
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    /// S1-I7: `cond ? then : else` ternary.
    Ternary {
        span: Span,
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    Unary {
        span: Span,
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        span: Span,
        callee: Box<Expr>,
        args: Vec<CallArg>,
    },
    Member {
        span: Span,
        object: Box<Expr>,
        name: SmolStr,
    },
    Index {
        span: Span,
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Assign {
        span: Span,
        name: SmolStr,
        value: Box<Expr>,
    },
    CompoundAssign {
        span: Span,
        name: SmolStr,
        op: CompoundAssignOp,
        value: Box<Expr>,
    },
    MemberAssign {
        span: Span,
        object: Box<Expr>,
        name: SmolStr,
        value: Box<Expr>,
    },
    IndexAssign {
        span: Span,
        object: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    Group {
        span: Span,
        expr: Box<Expr>,
    },
    /// Phase 139 L2: tuple literal `(a, b)` / `(x: 1, y: 2)`.
    Tuple {
        span: Span,
        elements: Vec<(Option<SmolStr>, Expr)>,
    },
    Array {
        span: Span,
        elements: Vec<Expr>,
    },
    Await {
        span: Span,
        expr: Box<Expr>,
    },
    OptionalChain {
        span: Span,
        object: Box<Expr>,
        name: SmolStr,
    },
    ForceUnwrap {
        span: Span,
        expr: Box<Expr>,
    },
    HalfOpenRange {
        span: Span,
        start: i64,
        end: i64,
    },
    ClosedRange {
        span: Span,
        start: i64,
        end: i64,
    },
    /// S1-I7: `0..<self` / `a...b` with expression bounds (for-in / values).
    RangeDyn {
        span: Span,
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
    },
    /// Enum/case pattern: `.ok` / `.success(let data)` / `.failure(code: let c, message: let m)`.
    /// Each binding is `(optional_label, name)`.
    VariantPattern {
        span: Span,
        variant: SmolStr,
        bindings: Vec<(Option<SmolStr>, SmolStr)>,
    },
    TuplePattern {
        span: Span,
        bindings: Vec<SmolStr>,
    },
    Closure {
        span: Span,
        params: Vec<Param>,
        /// Explicit `-> T` on Swift-style `{ (x: Int) -> T in ... }` (optional).
        return_type: Option<TypeExpr>,
        body: ClosureBody,
    },
    TaskBlock {
        span: Span,
        detached: bool,
        body: Vec<Stmt>,
    },
    Borrow {
        span: Span,
        expr: Box<Expr>,
        mutable: bool,
    },
    /// Phase 43: read-only key path literal (`\Person.name`, `\Person?.name`).
    KeyPath {
        span: Span,
        root_type: SmolStr,
        property: SmolStr,
        optional: bool,
    },
    /// Phase 43: optional cast (`expr as? Type`).
    OptionalCast {
        span: Span,
        expr: Box<Expr>,
        target: TypeExpr,
    },
    /// Phase 60: runtime type check (`expr is Type`) → `Bool`.
    Is {
        span: Span,
        expr: Box<Expr>,
        target: TypeExpr,
    },
    /// Phase 60: hard cast (`expr as Type`); fails at runtime when incompatible.
    Cast {
        span: Span,
        expr: Box<Expr>,
        target: TypeExpr,
    },
    /// Phase 86: type metatype literal (`Point.self`) for `reflect(T.self)`.
    TypeSelf {
        span: Span,
        type_name: SmolStr,
    },
    /// Phase 50 ADR-070 M2: freestanding macro invocation in expression position.
    MacroInvoke {
        span: Span,
        name: SmolStr,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClosureBody {
    Expr(Box<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    pub label: Option<SmolStr>,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub pattern: Expr,
    pub guard: Option<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr {
        span: Span,
        expr: Expr,
    },
    VarDecl {
        span: Span,
        mutable: bool,
        name: SmolStr,
        ty: Option<TypeExpr>,
        init: Option<Expr>,
    },
    Return {
        span: Span,
        value: Option<Expr>,
    },
    If {
        span: Span,
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        span: Span,
        condition: Expr,
        body: Vec<Stmt>,
    },
    ForIn {
        span: Span,
        name: SmolStr,
        iterable: Expr,
        body: Vec<Stmt>,
        /// `for await x in …` async sequence iteration (Phase 9 AL3).
        is_async_iter: bool,
    },
    Block {
        span: Span,
        stmts: Vec<Stmt>,
    },
    /// Phase 139: statement sequence in the same scope (tuple destructure desugar).
    Seq {
        span: Span,
        stmts: Vec<Stmt>,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Try {
        span: Span,
        body: Vec<Stmt>,
        catch_name: SmolStr,
        catch_type: Option<TypeExpr>,
        catch_body: Vec<Stmt>,
    },
    Switch {
        span: Span,
        subject: Expr,
        cases: Vec<SwitchCase>,
        default: Option<Vec<Stmt>>,
    },
    IfCase {
        span: Span,
        pattern: Expr,
        subject: Expr,
        body: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    IfLet {
        span: Span,
        name: SmolStr,
        pattern: Expr,
        body: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    GuardLet {
        span: Span,
        name: SmolStr,
        pattern: Expr,
        body: Vec<Stmt>,
        else_branch: Vec<Stmt>,
    },
    Print {
        span: Span,
        expr: Expr,
    },
    Throw {
        span: Span,
        expr: Expr,
    },
    RepeatWhile {
        span: Span,
        body: Vec<Stmt>,
        condition: Expr,
    },
    MacroInvoke {
        span: Span,
        name: SmolStr,
        args: Vec<Expr>,
    },
}

/// Phase 139 L6: external argument label at call sites.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamLabel {
    /// `name:` — call site uses the internal name as label.
    Same,
    /// `label name` — call site must use `label:`.
    Named(SmolStr),
    /// `_ name` — call site is positional (no label).
    Omitted,
}

impl ParamLabel {
    /// Label required at call sites, if any.
    pub fn call_site_label<'a>(&'a self, internal: &'a SmolStr) -> Option<&'a SmolStr> {
        match self {
            ParamLabel::Same => Some(internal),
            ParamLabel::Named(label) => Some(label),
            ParamLabel::Omitted => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: SmolStr,
    pub external_label: ParamLabel,
    pub ty: Option<TypeExpr>,
    pub default: Option<Expr>,
    /// Phase 10 AQ4: `isolated` parameter for actor isolation.
    pub isolated: bool,
    /// Phase 11 AV1: `sending` parameter ownership transfer at isolation boundary.
    pub sending: bool,
    /// Phase 120: parameter attributes (e.g. `@Inject` for Service DI ctor inject).
    pub attrs: Vec<Attribute>,
}

impl Param {
    pub fn call_site_label(&self) -> Option<&SmolStr> {
        self.external_label.call_site_label(&self.name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionEffect {
    Pure,
    Io,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeArg {
    Ident(SmolStr),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: SmolStr,
    pub args: Vec<AttributeArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WherePred {
    pub type_name: SmolStr,
    pub constraint: SmolStr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroImplementation {
    User {
        params: Vec<SmolStr>,
        body: Vec<Stmt>,
        /// Phase 56/58: `@freestanding(declaration)` body decls.
        decl_body: Vec<Decl>,
    },
    External {
        module: String,
        type_name: String,
    },
    /// Phase 54: `#builtInMacro("tracked_accessor")` etc.
    BuiltIn {
        id: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    Function {
        span: Span,
        access: AccessLevel,
        attrs: Vec<Attribute>,
        name: SmolStr,
        type_params: Vec<SmolStr>,
        params: Vec<Param>,
        return_type: Option<TypeExpr>,
        throws_type: Option<TypeExpr>,
        effect: Option<FunctionEffect>,
        where_clause: Vec<WherePred>,
        body: Vec<Stmt>,
        is_async: bool,
        /// Phase 64: marks intentional override of superclass method.
        is_override: bool,
        /// Phase 65: marks method as non-overridable in subclasses.
        is_final: bool,
    },
    /// Phase 64: user-defined operator overload (`operator func +(a: T, b: T) -> T`).
    OperatorFunc {
        span: Span,
        access: AccessLevel,
        attrs: Vec<Attribute>,
        op: SmolStr,
        params: Vec<Param>,
        return_type: Option<TypeExpr>,
        body: Vec<Stmt>,
    },
    Import {
        span: Span,
        path: Vec<SmolStr>,
        alias: Option<SmolStr>,
    },
    /// W480 / ADR-824: `export func …` wraps a Function declaration (EBNF ExportDecl).
    /// Semantic → `aot_export_roots` fill remains deferred (≠ this variant alone).
    Export {
        span: Span,
        item: Box<Decl>,
    },
    /// W483 / ADR-827: `export { Name, … }` module-level export list (EBNF ExportList).
    /// Names push into `MirModule.aot_export_roots` on lower (≠ auto-discovery).
    ExportList {
        span: Span,
        names: Vec<SmolStr>,
    },
    Class {
        span: Span,
        access: AccessLevel,
        attrs: Vec<Attribute>,
        name: SmolStr,
        /// S1-I6: `class Holder<T>` / `class Holder<T = Int>` type parameters.
        type_params: Vec<SmolStr>,
        /// S1-I6: default type args parallel to `type_params` (`T = Int` → `Some(Int)`).
        type_param_defaults: Vec<Option<TypeExpr>>,
        /// Phase 64: single inheritance superclass (`class Dog: Animal`).
        superclass: Option<SmolStr>,
        conformances: Vec<SmolStr>,
        members: Vec<Decl>,
    },
    Struct {
        span: Span,
        access: AccessLevel,
        name: SmolStr,
        /// S1-I3: `struct Box<T>` type parameters (semantic/mono may still be thin).
        type_params: Vec<SmolStr>,
        /// S1-I6: default type args parallel to `type_params`.
        type_param_defaults: Vec<Option<TypeExpr>>,
        /// Stored fields (`var`/`let name: Type [= init]`). Init expressions live on matching
        /// `VarGlobal` entries in `members` when present.
        fields: Vec<(SmolStr, TypeExpr)>,
        /// S1-02: methods / `init` / field decls (`VarGlobal`) / nested types inside the struct body.
        members: Vec<Decl>,
    },
    Protocol {
        span: Span,
        /// W687 / C-06: access was previously discarded at parse — required for honest fmt.
        access: AccessLevel,
        name: SmolStr,
        /// W687: `protocol Codable: Encodable, Decodable` inherited list.
        inherited: Vec<SmolStr>,
        /// Phase 20: `associatedtype Element: Constraint` declarations.
        associated_types: Vec<(SmolStr, TypeExpr)>,
        methods: Vec<Decl>,
    },
    VarGlobal {
        span: Span,
        access: AccessLevel,
        attrs: Vec<Attribute>,
        mutable: bool,
        name: SmolStr,
        ty: Option<TypeExpr>,
        init: Option<Expr>,
        /// Phase 64: property observer blocks on stored properties.
        will_set: Option<Vec<Stmt>>,
        did_set: Option<Vec<Stmt>>,
        /// Phase 134: class computed property `{ get { … } }` (mutually exclusive with init/observers).
        computed_getter: Option<Vec<Stmt>>,
        /// Phase 135: optional `{ set { … } }` body (`newValue` binding).
        computed_setter: Option<Vec<Stmt>>,
        /// Phase 135: `override var` on computed property in subclass.
        is_override: bool,
        /// Phase 134: class `lazy var` — first read runs initializer (non-thread-safe MVP).
        is_lazy: bool,
    },
    Extension {
        span: Span,
        type_name: SmolStr,
        conformances: Vec<SmolStr>,
        where_clause: Vec<WherePred>,
        methods: Vec<Decl>,
    },
    DataClass {
        span: Span,
        access: AccessLevel,
        attrs: Vec<Attribute>,
        name: SmolStr,
        type_params: Vec<SmolStr>,
        where_clause: Vec<WherePred>,
        fields: Vec<(SmolStr, TypeExpr)>,
    },
    FinalClass {
        span: Span,
        access: AccessLevel,
        name: SmolStr,
        type_params: Vec<SmolStr>,
        variants: Vec<(SmolStr, Vec<(SmolStr, TypeExpr)>)>,
    },
    Actor {
        span: Span,
        access: AccessLevel,
        name: SmolStr,
        /// S1-I5: `actor Holder<T>` type parameters (mono may still be thin).
        type_params: Vec<SmolStr>,
        /// S1-I6: default type args parallel to `type_params`.
        type_param_defaults: Vec<Option<TypeExpr>>,
        conformances: Vec<SmolStr>,
        members: Vec<Decl>,
    },
    /// `global actor MyActor { }` — custom GlobalActor declaration (Phase 10 AQ4).
    GlobalActor {
        span: Span,
        access: AccessLevel,
        name: SmolStr,
    },
    Enum {
        span: Span,
        access: AccessLevel,
        attrs: Vec<Attribute>,
        name: SmolStr,
        type_params: Vec<SmolStr>,
        cases: Vec<(SmolStr, Option<Expr>, Vec<(SmolStr, TypeExpr)>)>,
    },
    Init {
        span: Span,
        access: AccessLevel,
        params: Vec<Param>,
        body: Vec<Stmt>,
    },
    Deinit {
        span: Span,
        access: AccessLevel,
        body: Vec<Stmt>,
    },
    /// `typealias Foo = Bar` — Phase 40 GA.
    TypeAlias {
        span: Span,
        access: AccessLevel,
        name: SmolStr,
        ty: TypeExpr,
    },
    /// Phase 56: module-level `#name(args)` declaration macro invoke.
    MacroSiteInvoke {
        span: Span,
        name: SmolStr,
        args: Vec<Expr>,
    },
    /// Phase 60: executable statement at module top level (main file only).
    TopLevelStmt {
        span: Span,
        stmt: Stmt,
    },
    /// Phase 51: unified macro declaration (ADR-070 M4).
    MacroDecl {
        span: Span,
        access: AccessLevel,
        attrs: Vec<Attribute>,
        name: SmolStr,
        implementation: MacroImplementation,
    },
    /// `subscript(index: Int) -> T { get / set }` — Phase 40 MVP.
    Subscript {
        span: Span,
        type_name: SmolStr,
        params: Vec<Param>,
        return_type: TypeExpr,
        getter: Vec<Stmt>,
        setter: Option<Vec<Stmt>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: SmolStr,
    pub decls: Vec<Decl>,
}

pub trait ExprVisitor<T> {
    fn visit_literal(&mut self, value: &Literal) -> T;
    fn visit_ident(&mut self, name: &SmolStr) -> T;
    fn visit_binary(&mut self, left: &Expr, op: BinOp, right: &Expr) -> T;
    fn visit_unary(&mut self, op: UnaryOp, expr: &Expr) -> T;
    fn visit_call(&mut self, callee: &Expr, args: &[CallArg]) -> T;
    fn visit_member(&mut self, object: &Expr, name: &SmolStr) -> T;
    fn visit_index(&mut self, object: &Expr, index: &Expr) -> T;
    fn visit_assign(&mut self, name: &SmolStr, value: &Expr) -> T;
    fn visit_group(&mut self, expr: &Expr) -> T;
    fn visit_array(&mut self, elements: &[Expr]) -> T;

    fn walk_expr(&mut self, expr: &Expr) -> T {
        match expr {
            Expr::Literal { value, .. } => self.visit_literal(value),
            Expr::Ident { name, .. } => self.visit_ident(name),
            Expr::Super { .. } => self.visit_ident(&SmolStr::new("super")),
            Expr::Binary { left, op, right, .. } => self.visit_binary(left, *op, right),
            Expr::Ternary {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                let _ = self.walk_expr(condition);
                let _ = self.walk_expr(then_branch);
                self.walk_expr(else_branch)
            }
            Expr::Unary { op, expr, .. } => self.visit_unary(*op, expr),
            Expr::Call { callee, args, .. } => self.visit_call(callee, args),
            Expr::Member { object, name, .. } => self.visit_member(object, name),
            Expr::Index { object, index, .. } => self.visit_index(object, index),
            Expr::Assign { name, value, .. } => self.visit_assign(name, value),
            Expr::CompoundAssign { name, value, .. } => self.visit_assign(name, value),
            Expr::MemberAssign { object, name, value, .. } => {
                let _ = self.visit_member(object, name);
                self.visit_group(value)
            }
            Expr::IndexAssign { object, index, value, .. } => {
                let _ = self.visit_index(object, index);
                self.visit_group(value)
            }
            Expr::Group { expr, .. } => self.visit_group(expr),
            Expr::Tuple { elements, .. } => {
                for (_, el) in elements {
                    let _ = self.walk_expr(el);
                }
                self.visit_literal(&Literal::Nil)
            }
            Expr::Array { elements, .. } => self.visit_array(elements),
            Expr::Await { expr, .. } => self.visit_group(expr),
            Expr::OptionalChain { object, .. } => self.visit_group(object),
            Expr::ForceUnwrap { expr, .. } => self.visit_group(expr),
            Expr::RangeDyn { start, end, .. } => {
                let _ = self.walk_expr(start);
                self.walk_expr(end)
            }
            Expr::Closure { body, .. } => self.visit_group(match body {
                ClosureBody::Expr(e) => e,
                ClosureBody::Block(stmts) => {
                    for stmt in stmts {
                        let _ = stmt;
                    }
                    return self.visit_literal(&Literal::Nil);
                }
            }),
            Expr::TaskBlock { body, .. } => {
                for stmt in body {
                    let _ = stmt;
                }
                self.visit_literal(&Literal::Nil)
            }
            Expr::HalfOpenRange { .. }
            | Expr::ClosedRange { .. }
            | Expr::VariantPattern { .. }
            | Expr::TuplePattern { .. } => {
                self.visit_literal(&Literal::Nil)
            }
            Expr::Borrow { expr, .. } => self.visit_group(expr),
            Expr::KeyPath { .. } => self.visit_literal(&Literal::Nil),
            Expr::OptionalCast { expr, .. } | Expr::Is { expr, .. } | Expr::Cast { expr, .. } => {
                self.visit_group(expr)
            }
            Expr::TypeSelf { .. } => self.visit_literal(&Literal::Nil),
            Expr::MacroInvoke { args, .. } => {
                for a in args {
                    let _ = self.walk_expr(a);
                }
                self.visit_literal(&Literal::Nil)
            }
        }
    }
}

impl Decl {
    /// Module symbol name: alias if present, else last path segment.
    pub fn import_symbol(&self) -> Option<&SmolStr> {
        match self {
            Decl::Import { alias, path, .. } => {
                if let Some(a) = alias {
                    Some(a)
                } else {
                    path.last()
                }
            }
            _ => None,
        }
    }

    pub fn import_path_str(&self) -> Option<String> {
        match self {
            Decl::Import { path, .. } => Some(path.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(".")),
            _ => None,
        }
    }
}

/// Phase 134: `@static` on class/extension members.
pub fn var_global_is_static(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.name.as_str() == "static")
}

pub fn var_global_getter_method(field: &SmolStr) -> SmolStr {
    SmolStr::new(format!("__get_{field}"))
}

pub fn var_global_setter_method(field: &SmolStr) -> SmolStr {
    SmolStr::new(format!("__set_{field}"))
}

/// Base of a member chain (`Outer.Middle` → `Outer_Middle`).
pub fn member_chain_mangled_base(expr: &Expr) -> Option<SmolStr> {
    match expr {
        Expr::Ident { name, .. } => Some(name.clone()),
        Expr::Member { object, name, .. } => member_chain_mangled_base(object)
            .map(|base| SmolStr::new(format!("{base}_{name}"))),
        _ => None,
    }
}

/// Nested type constructor callee (`Outer.Inner()` / `Outer.Middle.Inner()` → `Outer_Inner` / `Outer_Middle_Inner`).
pub fn nested_type_ctor_mangled(callee: &Expr) -> Option<SmolStr> {
    match callee {
        Expr::Member {
            object,
            name,
            ..
        } if name
            .chars()
            .next()
            .is_some_and(|c| c.is_uppercase()) =>
        {
            member_chain_mangled_base(object).map(|base| SmolStr::new(format!("{base}_{name}")))
        }
        _ => None,
    }
}

/// Dotted qualified path for a nested type (`Outer_Middle` + `Inner` → `Outer.Middle.Inner`).
pub fn nested_type_dotted_path(outer_mangled: &str, inner: &str) -> SmolStr {
    if outer_mangled.is_empty() {
        SmolStr::new(inner)
    } else {
        SmolStr::new(format!("{}.{inner}", outer_mangled.replace('_', ".")))
    }
}

/// Split `{Class}__{method}` or `{Class}___get_{field}` / `{Class}___set_{field}` for virtual dispatch.
pub fn parse_mangled_instance_method(full: &str) -> Option<(SmolStr, SmolStr)> {
    if let Some((class, field)) = full.split_once("___get_") {
        return Some((
            SmolStr::new(class),
            SmolStr::new(format!("__get_{field}")),
        ));
    }
    if let Some((class, field)) = full.split_once("___set_") {
        return Some((
            SmolStr::new(class),
            SmolStr::new(format!("__set_{field}")),
        ));
    }
    full.rsplit_once('_').map(|(class, method)| (SmolStr::new(class), SmolStr::new(method)))
}

pub fn lazy_storage_field(field: &SmolStr) -> SmolStr {
    SmolStr::new(format!("__lazy_storage_{field}"))
}

pub fn static_var_global_name(type_name: &SmolStr, field: &SmolStr) -> SmolStr {
    SmolStr::new(format!("{type_name}_{field}"))
}

/// Stored instance field slot name, if any (excludes computed/static).
pub fn var_global_stored_field_name(member: &Decl) -> Option<SmolStr> {
    let Decl::VarGlobal {
        name,
        computed_getter,
        is_lazy,
        attrs,
        ..
    } = member
    else {
        return None;
    };
    if computed_getter.is_some() || var_global_is_static(attrs) {
        return None;
    }
    if *is_lazy {
        return Some(lazy_storage_field(name));
    }
    Some(name.clone())
}

pub fn var_global_is_computed(member: &Decl) -> bool {
    matches!(
        member,
        Decl::VarGlobal {
            computed_getter: Some(_),
            ..
        }
    )
}

pub fn var_global_is_lazy(member: &Decl) -> bool {
    matches!(member, Decl::VarGlobal { is_lazy: true, .. })
}
