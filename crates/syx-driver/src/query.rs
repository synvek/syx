//! Module-level and decl-level query cache (Phase 8 AH1 → Phase 9 AM1 v2 → Phase 10 AR1 v3 → Phase 11 AW2 v4).
//!
//! Keys compile results by source hash + dependency tree fingerprint so
//! `compile_linked_with_options` can skip disk re-read on repeated hits.
//! DeclId-level entries support function-scoped invalidation for LSP incremental
//! semantic recheck. v3 adds parse / semantic layered storage and import-boundary
//! invalidation so `didChange` only re-checks the semantic subgraph of edited decls.
//! v4 propagates invalidation to importer modules when a `public` decl body changes.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use smol_str::SmolStr;
use syx_ast::{Decl, Module};

use super::{dependency_hashes_still_valid, dependency_tree_fingerprint, hash_source, ModuleUnit};

/// Stable key for a top-level declaration in a module file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclId {
    pub path: PathBuf,
    pub name: SmolStr,
}

impl DeclId {
    pub fn new(path: impl Into<PathBuf>, name: impl Into<SmolStr>) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
        }
    }
}

/// Cached parse tree for a module (v3 parse layer).
#[derive(Debug, Clone)]
struct ParseLayerEntry {
    source_hash: String,
    module: Module,
}

/// Per-decl semantic fingerprint (v3 semantic layer; full analysis still module-wide).
#[derive(Debug, Clone)]
struct SemanticDeclEntry {
    body_hash: String,
    diagnostic_count: usize,
}

/// In-memory mirror of the on-disk incremental cache (parse → semantic → compile).
#[derive(Debug, Default)]
pub struct ModuleQueryCache {
    entries: HashMap<String, CachedParseHit>,
    /// Decl body hash after last successful parse/semantic store.
    decl_parse: HashMap<String, String>,
    /// Decl name → other decl names referenced in its body (same module).
    decl_deps: HashMap<String, HashSet<String>>,
    /// Reverse edges for invalidation propagation.
    decl_dependents: HashMap<String, HashSet<String>>,
    /// Last seen source text per module path (LSP incremental diff).
    last_source: HashMap<String, String>,
    /// v3: parse tree keyed by module path.
    parse_layer: HashMap<String, ParseLayerEntry>,
    /// v3: semantic layer keyed by decl key (path|name).
    semantic_layer: HashMap<String, SemanticDeclEntry>,
    /// v3: hash of import declarations for import-boundary invalidation.
    import_boundary: HashMap<String, String>,
    /// v3: modules that import this path (reverse import boundary).
    import_dependents: HashMap<String, HashSet<String>>,
    /// v3: decl keys needing semantic recheck after last touch.
    pending_semantic: HashSet<String>,
}

#[derive(Debug, Clone)]
struct CachedParseHit {
    source_hash: String,
    dependency_tree_hash: String,
    unit: ModuleUnit,
}

impl ModuleQueryCache {
    pub fn cache_key(path: &Path, source_hash: &str, dependency_tree_hash: &str) -> String {
        format!("{}|{source_hash}|{dependency_tree_hash}", path.display())
    }

    fn decl_key(path: &Path, name: &str) -> String {
        format!("{}|{name}", path.display())
    }

    pub fn lookup(
        &self,
        path: &Path,
        source: &str,
        dependency_hashes: &[(String, String)],
    ) -> Option<ModuleUnit> {
        if !dependency_hashes_still_valid(dependency_hashes) {
            return None;
        }
        let source_hash = hash_source(source);
        let tree_hash = dependency_tree_fingerprint(dependency_hashes);
        let key = Self::cache_key(path, &source_hash, &tree_hash);
        self.entries.get(&key).and_then(|hit| {
            if hit.source_hash == source_hash && hit.dependency_tree_hash == tree_hash {
                Some(hit.unit.clone())
            } else {
                None
            }
        })
    }

    pub fn store(&mut self, unit: ModuleUnit, dependency_hashes: &[(String, String)]) {
        let source_hash = hash_source(&unit.source.contents);
        let tree_hash = dependency_tree_fingerprint(dependency_hashes);
        let key = Self::cache_key(&unit.path, &source_hash, &tree_hash);
        self.entries.insert(
            key,
            CachedParseHit {
                source_hash,
                dependency_tree_hash: tree_hash,
                unit: unit.clone(),
            },
        );
        self.index_decls_from_source(&unit.path, &unit.source.contents);
        self.last_source.insert(
            unit.path.display().to_string(),
            unit.source.contents.to_string(),
        );
    }

    pub fn invalidate_path(&mut self, path: &Path) {
        let prefix = format!("{}|", path.display());
        let path_key = path.display().to_string();
        self.entries.retain(|k, _| !k.starts_with(&prefix));
        self.decl_parse.retain(|k, _| !k.starts_with(&prefix));
        self.decl_deps.retain(|k, _| !k.starts_with(&prefix));
        self.decl_dependents.retain(|k, _| !k.starts_with(&prefix));
        self.last_source.remove(&path_key);
        self.parse_layer.remove(&path_key);
        self.semantic_layer.retain(|k, _| !k.starts_with(&prefix));
        self.import_boundary.remove(&path_key);
        self.pending_semantic.retain(|k| !k.starts_with(&prefix));
        if let Some(deps) = self.import_dependents.remove(&path_key) {
            for dep in deps {
                // dependents will re-check import boundary on next touch
                let _ = dep;
            }
        }
    }

    pub fn invalidate_decl(&mut self, path: &Path, name: &str) {
        let key = Self::decl_key(path, name);
        self.decl_parse.remove(&key);
        self.semantic_layer.remove(&key);
        self.pending_semantic.insert(key.clone());
        if let Some(deps) = self.decl_dependents.get(&key).cloned() {
            for dependent in deps {
                let dep_key = Self::decl_key(path, &dependent);
                if let Some(set) = self.decl_deps.get_mut(&dep_key) {
                    set.remove(name);
                }
            }
        }
        self.decl_deps.remove(&key);
        self.decl_dependents.remove(&key);
    }

    /// Invalidate `name` and any decl that transitively depends on it.
    pub fn invalidate_decl_subgraph(&mut self, path: &Path, name: &str) {
        let mut work = vec![name.to_string()];
        let mut seen = HashSet::new();
        while let Some(n) = work.pop() {
            if !seen.insert(n.clone()) {
                continue;
            }
            self.invalidate_decl(path, &n);
            if let Some(deps) = self.decl_dependents.get(&Self::decl_key(path, &n)).cloned() {
                for d in deps {
                    work.push(d);
                }
            }
        }
    }

    pub fn decl_parse_hit(&self, path: &Path, name: &str, body_hash: &str) -> bool {
        self.decl_parse
            .get(&Self::decl_key(path, name))
            .is_some_and(|h| h == body_hash)
    }

    pub fn record_decl_parse(&mut self, path: &Path, name: &str, body_hash: String) {
        self.decl_parse
            .insert(Self::decl_key(path, name), body_hash);
    }

    pub fn touch_source(&mut self, path: &Path, new_source: &str) -> Vec<DeclId> {
        let path_key = path.display().to_string();
        let old = self.last_source.get(&path_key).cloned().unwrap_or_default();
        if old == new_source {
            return Vec::new();
        }
        self.last_source
            .insert(path_key.clone(), new_source.to_string());

        let new_import_hash = hash_import_boundary(new_source);
        let import_changed = self
            .import_boundary
            .get(&path_key)
            .is_some_and(|h| h != &new_import_hash)
            || (self.import_boundary.get(&path_key).is_none() && !old.is_empty());
        self.import_boundary
            .insert(path_key.clone(), new_import_hash);

        if import_changed && !old.is_empty() {
            self.invalidate_semantic_layer(path);
            if let Some(importers) = self.import_dependents.get(&path_key).cloned() {
                for importer in importers {
                    self.invalidate_semantic_layer(Path::new(&importer));
                }
            }
        }

        let source_hash = hash_source(new_source);
        if self
            .parse_layer
            .get(&path_key)
            .is_some_and(|e| e.source_hash == source_hash)
        {
            // parse layer hit — decl diff still drives semantic invalidation
        } else if let Ok(module) = syx_parser::Parser::parse(syx_diagnostics::SourceFile::new(
            path_key.clone(),
            new_source.to_string(),
        )) {
            self.parse_layer.insert(
                path_key.clone(),
                ParseLayerEntry {
                    source_hash,
                    module: module.clone(),
                },
            );
            self.index_decls_from_module(path, &module);
        }

        let changed = if old.is_empty() {
            Vec::new()
        } else {
            changed_decl_names_cached(self, path, &old, new_source)
        };
        for name in &changed {
            self.invalidate_decl_subgraph(path, name);
            self.invalidate_importers_of_public_decl(path, name, new_source);
        }
        changed
            .into_iter()
            .map(|name| DeclId::new(path, name))
            .collect()
    }

    /// v3: lookup cached parse tree when source hash matches.
    pub fn lookup_parse(&self, path: &Path, source: &str) -> Option<Module> {
        let path_key = path.display().to_string();
        let source_hash = hash_source(source);
        self.parse_layer.get(&path_key).and_then(|e| {
            if e.source_hash == source_hash {
                Some(e.module.clone())
            } else {
                None
            }
        })
    }

    /// v3: store parse layer after external parse.
    pub fn store_parse(&mut self, path: &Path, source: &str, module: &Module) {
        let path_key = path.display().to_string();
        self.parse_layer.insert(
            path_key,
            ParseLayerEntry {
                source_hash: hash_source(source),
                module: module.clone(),
            },
        );
        self.index_decls_from_module(path, module);
        self.import_boundary
            .insert(path.display().to_string(), hash_import_boundary(source));
    }

    /// v3: record semantic snapshot for a decl after successful analysis.
    pub fn record_semantic_decl(
        &mut self,
        path: &Path,
        name: &str,
        body_hash: &str,
        diagnostic_count: usize,
    ) {
        let key = Self::decl_key(path, name);
        self.semantic_layer.insert(
            key.clone(),
            SemanticDeclEntry {
                body_hash: body_hash.to_string(),
                diagnostic_count,
            },
        );
        self.pending_semantic.remove(&key);
    }

    /// v3: true when semantic layer matches current decl body hash.
    pub fn semantic_decl_hit(&self, path: &Path, name: &str, body_hash: &str) -> bool {
        self.semantic_layer
            .get(&Self::decl_key(path, name))
            .is_some_and(|e| e.body_hash == body_hash)
    }

    /// v3: decl names still needing semantic recheck (invalidated subgraph).
    pub fn pending_semantic_decls(&self, path: &Path) -> Vec<DeclId> {
        let prefix = format!("{}|", path.display());
        self.pending_semantic
            .iter()
            .filter(|k| k.starts_with(&prefix))
            .filter_map(|k| k.strip_prefix(&prefix).map(|name| DeclId::new(path, name)))
            .collect()
    }

    /// v3: register that `importer` depends on `imported` module path for boundary invalidation.
    pub fn register_import_edge(&mut self, importer: &Path, imported: &Path) {
        self.import_dependents
            .entry(imported.display().to_string())
            .or_default()
            .insert(importer.display().to_string());
    }

    fn invalidate_semantic_layer(&mut self, path: &Path) {
        let prefix = format!("{}|", path.display());
        self.semantic_layer.retain(|k, _| !k.starts_with(&prefix));
        for key in self.decl_parse.keys().filter(|k| k.starts_with(&prefix)) {
            self.pending_semantic.insert(key.clone());
        }
    }

    /// v4: when a `public` decl body changes, invalidate semantic layers of importing modules.
    fn invalidate_importers_of_public_decl(&mut self, path: &Path, name: &str, source: &str) {
        let Ok(module) = syx_parser::Parser::parse(syx_diagnostics::SourceFile::new(
            path.display().to_string(),
            source.to_string(),
        )) else {
            return;
        };
        let is_public = module.decls.iter().any(|decl| match decl {
            syx_ast::Decl::Function {
                name: n,
                access: syx_ast::AccessLevel::Public,
                ..
            }
            | syx_ast::Decl::VarGlobal {
                name: n,
                access: syx_ast::AccessLevel::Public,
                ..
            }
            | syx_ast::Decl::DataClass {
                name: n,
                access: syx_ast::AccessLevel::Public,
                ..
            } => n.as_str() == name,
            _ => false,
        });
        if !is_public {
            return;
        }
        let path_key = path.display().to_string();
        if let Some(importers) = self.import_dependents.get(&path_key).cloned() {
            for importer in importers {
                self.invalidate_semantic_layer(Path::new(&importer));
            }
        }
    }

    fn index_decls_from_source(&mut self, path: &Path, source: &str) {
        let Ok(module) = syx_parser::Parser::parse(syx_diagnostics::SourceFile::new(
            path.display().to_string(),
            source.to_string(),
        )) else {
            return;
        };
        self.index_decls_from_module(path, &module);
    }

    fn index_decls_from_module(&mut self, path: &Path, module: &Module) {
        let mut deps_map: HashMap<String, HashSet<String>> = HashMap::new();
        let mut rev_map: HashMap<String, HashSet<String>> = HashMap::new();
        for decl in &module.decls {
            let Some(name) = decl_top_level_name(decl) else {
                continue;
            };
            let body_hash = hash_decl_body(decl);
            self.record_decl_parse(path, name, body_hash);
            let refs = decl_referenced_names(decl);
            deps_map.insert(name.to_string(), refs.clone());
            for r in refs {
                rev_map.entry(r).or_default().insert(name.to_string());
            }
        }
        for (name, deps) in deps_map {
            self.decl_deps.insert(Self::decl_key(path, &name), deps);
        }
        for (name, dependents) in rev_map {
            self.decl_dependents
                .insert(Self::decl_key(path, &name), dependents);
        }
    }
}

static MODULE_QUERY_CACHE: OnceLock<Mutex<ModuleQueryCache>> = OnceLock::new();

pub fn global_query_cache() -> &'static Mutex<ModuleQueryCache> {
    MODULE_QUERY_CACHE.get_or_init(|| Mutex::new(ModuleQueryCache::default()))
}

pub fn lookup_cached(
    path: &Path,
    source: &str,
    dependency_hashes: &[(String, String)],
) -> Option<ModuleUnit> {
    global_query_cache()
        .lock()
        .ok()
        .and_then(|cache| cache.lookup(path, source, dependency_hashes))
}

pub fn store_cached(unit: ModuleUnit, dependency_hashes: &[(String, String)]) {
    if let Ok(mut cache) = global_query_cache().lock() {
        cache.store(unit, dependency_hashes);
    }
}

pub fn invalidate_path(path: &Path) {
    if let Ok(mut cache) = global_query_cache().lock() {
        cache.invalidate_path(path);
    }
}

/// LSP/driver hook: diff source, invalidate affected decl subgraph, return changed decls.
pub fn on_source_changed(path: &Path, new_source: &str) -> Vec<DeclId> {
    global_query_cache()
        .lock()
        .ok()
        .map(|mut cache| cache.touch_source(path, new_source))
        .unwrap_or_default()
}

/// v3: parse layer lookup (returns cached AST when source hash matches).
pub fn lookup_parse(path: &Path, source: &str) -> Option<Module> {
    global_query_cache()
        .lock()
        .ok()
        .and_then(|cache| cache.lookup_parse(path, source))
}

/// v3: store parse layer after parse (LSP/driver shared cache).
pub fn store_parse(path: &Path, source: &str, module: &Module) {
    if let Ok(mut cache) = global_query_cache().lock() {
        cache.store_parse(path, source, module);
    }
}

/// v3: record per-decl semantic snapshot after analysis.
pub fn record_semantic_decl(path: &Path, name: &str, body_hash: &str, diagnostic_count: usize) {
    if let Ok(mut cache) = global_query_cache().lock() {
        cache.record_semantic_decl(path, name, body_hash, diagnostic_count);
    }
}

/// v3: decls still pending semantic recheck for LSP incremental path.
pub fn pending_semantic_decls(path: &Path) -> Vec<DeclId> {
    global_query_cache()
        .lock()
        .ok()
        .map(|cache| cache.pending_semantic_decls(path))
        .unwrap_or_default()
}

/// v3: register cross-module import edge for boundary invalidation.
pub fn register_import_edge(importer: &Path, imported: &Path) {
    if let Ok(mut cache) = global_query_cache().lock() {
        cache.register_import_edge(importer, imported);
    }
}

pub fn decls_for_module(path: &Path, source: &str) -> Vec<DeclId> {
    let Ok(module) = syx_parser::Parser::parse(syx_diagnostics::SourceFile::new(
        path.display().to_string(),
        source.to_string(),
    )) else {
        return Vec::new();
    };
    module
        .decls
        .iter()
        .filter_map(decl_top_level_name)
        .map(|name| DeclId::new(path, name))
        .collect()
}

fn hash_import_boundary(source: &str) -> String {
    use sha2::{Digest, Sha256};
    let Ok(module) = syx_parser::Parser::parse(syx_diagnostics::SourceFile::new(
        "import.sy",
        source.to_string(),
    )) else {
        return hash_source(source);
    };
    let mut hasher = Sha256::new();
    for decl in &module.decls {
        if let Decl::Import { path, alias, .. } = decl {
            for seg in path {
                hasher.update(seg.as_bytes());
            }
            if let Some(a) = alias {
                hasher.update(a.as_bytes());
            }
        }
    }
    format!("{:x}", hasher.finalize())
}

fn decl_top_level_name(decl: &Decl) -> Option<&str> {
    match decl {
        Decl::Function { name, .. }
        | Decl::Class { name, .. }
        | Decl::Struct { name, .. }
        | Decl::Protocol { name, .. }
        | Decl::VarGlobal { name, .. }
        | Decl::DataClass { name, .. }
        | Decl::FinalClass { name, .. }
        | Decl::Actor { name, .. }
        | Decl::Enum { name, .. }
        | Decl::GlobalActor { name, .. } => Some(name.as_str()),
        Decl::Import { .. }
        | Decl::Extension { .. }
        | Decl::Init { .. }
        | Decl::Deinit { .. }
        | Decl::TypeAlias { .. }
        | Decl::MacroDecl { .. }
        | Decl::MacroSiteInvoke { .. }
        | Decl::Subscript { .. }
        | Decl::TopLevelStmt { .. }
        | Decl::OperatorFunc { .. } => None,
        Decl::Export { item, .. } => decl_top_level_name(item),
        Decl::ExportList { .. } => None,
    }
}

/// Top-level decl name for LSP/query indexing (v3).
pub fn decl_name(decl: &Decl) -> Option<&str> {
    decl_top_level_name(decl)
}

fn hash_decl_body(decl: &Decl) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(format!("{decl:?}").as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Hash of a decl body for semantic-layer keys (v3).
pub fn decl_body_hash(decl: &Decl) -> String {
    hash_decl_body(decl)
}

fn decl_referenced_names(decl: &Decl) -> HashSet<String> {
    let mut names = HashSet::new();
    match decl {
        Decl::Function { body, .. } => collect_stmt_refs(body, &mut names),
        Decl::VarGlobal {
            init: Some(expr), ..
        } => collect_expr_refs(expr, &mut names),
        _ => {}
    }
    names
}

fn collect_stmt_refs(stmts: &[syx_ast::Stmt], names: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            syx_ast::Stmt::Expr { expr, .. } => collect_expr_refs(expr, names),
            syx_ast::Stmt::VarDecl { init: Some(e), .. } => collect_expr_refs(e, names),
            syx_ast::Stmt::Return { value: Some(e), .. } => collect_expr_refs(e, names),
            syx_ast::Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                collect_expr_refs(condition, names);
                collect_stmt_refs(then_branch, names);
                if let Some(el) = else_branch {
                    collect_stmt_refs(el, names);
                }
            }
            syx_ast::Stmt::While {
                condition, body, ..
            } => {
                collect_expr_refs(condition, names);
                collect_stmt_refs(body, names);
            }
            syx_ast::Stmt::Block { stmts, .. } => collect_stmt_refs(stmts, names),
            _ => {}
        }
    }
}

fn collect_expr_refs(expr: &syx_ast::Expr, names: &mut HashSet<String>) {
    match expr {
        syx_ast::Expr::Ident { name, .. } => {
            names.insert(name.to_string());
        }
        syx_ast::Expr::Binary { left, right, .. } => {
            collect_expr_refs(left, names);
            collect_expr_refs(right, names);
        }
        syx_ast::Expr::Unary { expr, .. } => collect_expr_refs(expr, names),
        syx_ast::Expr::Call { callee, args, .. } => {
            collect_expr_refs(callee, names);
            for a in args {
                collect_expr_refs(&a.value, names);
            }
        }
        syx_ast::Expr::Member { object, .. } => collect_expr_refs(object, names),
        syx_ast::Expr::Index { object, index, .. } => {
            collect_expr_refs(object, names);
            collect_expr_refs(index, names);
        }
        syx_ast::Expr::Assign { value, .. } => collect_expr_refs(value, names),
        syx_ast::Expr::Group { expr, .. } => collect_expr_refs(expr, names),
        syx_ast::Expr::Array { elements, .. } => {
            for e in elements {
                collect_expr_refs(e, names);
            }
        }
        syx_ast::Expr::Await { expr, .. } => collect_expr_refs(expr, names),
        _ => {}
    }
}

fn changed_decl_names_cached(
    cache: &ModuleQueryCache,
    path: &Path,
    old_source: &str,
    new_source: &str,
) -> Vec<String> {
    let old_mod = cache.lookup_parse(path, old_source).or_else(|| {
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new(
            path.display().to_string(),
            old_source.to_string(),
        ))
        .ok()
    });
    let new_mod = cache.lookup_parse(path, new_source).or_else(|| {
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new(
            path.display().to_string(),
            new_source.to_string(),
        ))
        .ok()
    });
    match (old_mod, new_mod) {
        (Some(old_mod), Some(new_mod)) => {
            diff_decl_names(&old_mod, &new_mod, old_source, new_source)
        }
        _ => changed_decl_names(old_source, new_source),
    }
}

fn diff_decl_names(
    old_mod: &Module,
    new_mod: &Module,
    old_source: &str,
    new_source: &str,
) -> Vec<String> {
    let mut old_hashes: HashMap<String, String> = HashMap::new();
    for decl in &old_mod.decls {
        if let Some(name) = decl_top_level_name(decl) {
            old_hashes.insert(name.to_string(), hash_decl_body(decl));
        }
    }
    let mut changed = Vec::new();
    for decl in &new_mod.decls {
        if let Some(name) = decl_top_level_name(decl) {
            let new_hash = hash_decl_body(decl);
            if old_hashes.get(name) != Some(&new_hash) {
                changed.push(name.to_string());
            }
        }
    }
    for name in old_hashes.keys() {
        if !new_mod
            .decls
            .iter()
            .filter_map(decl_top_level_name)
            .any(|n| n == name)
        {
            changed.push(name.clone());
        }
    }
    if changed.is_empty() && old_source != new_source {
        changed.push("__module__".into());
    }
    changed
}

fn changed_decl_names(old_source: &str, new_source: &str) -> Vec<String> {
    if old_source.is_empty() {
        return Vec::new();
    }
    let old_file = syx_diagnostics::SourceFile::new("old.sy", old_source.to_string());
    let new_file = syx_diagnostics::SourceFile::new("new.sy", new_source.to_string());
    let Ok(old_mod) = syx_parser::Parser::parse(old_file) else {
        return vec!["__module__".into()];
    };
    let Ok(new_mod) = syx_parser::Parser::parse(new_file) else {
        return vec!["__module__".into()];
    };
    diff_decl_names(&old_mod, &new_mod, old_source, new_source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn decl_invalidation_subgraph_propagates_to_callers() {
        let path = Path::new("/tmp/a.sy");
        let mut cache = ModuleQueryCache::default();
        let source = r#"
func helper() -> Int { return 1 }
func main() { let x = helper() }
"#;
        cache.index_decls_from_source(path, source);
        assert!(cache.decl_parse_hit(
            path,
            "helper",
            &hash_decl_body_from_source(source, "helper")
        ));
        cache.invalidate_decl_subgraph(path, "helper");
        assert!(!cache.decl_parse_hit(
            path,
            "helper",
            &hash_decl_body_from_source(source, "helper")
        ));
    }

    #[test]
    fn touch_source_detects_changed_function() {
        let path = Path::new("/tmp/touch.sy");
        let mut cache = ModuleQueryCache::default();
        let v1 = "func foo() -> Int { return 1 }\nfunc bar() -> Int { return 2 }\n";
        let v2 = "func foo() -> Int { return 9 }\nfunc bar() -> Int { return 2 }\n";
        cache.touch_source(path, v1);
        let changed = cache.touch_source(path, v2);
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].name.as_str(), "foo");
    }

    fn hash_decl_body_from_source(source: &str, name: &str) -> String {
        let module =
            syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", source.to_string()))
                .unwrap();
        module
            .decls
            .iter()
            .find(|d| decl_top_level_name(d) == Some(name))
            .map(hash_decl_body)
            .unwrap()
    }

    #[test]
    fn query_v3_parse_layer_hit_on_unchanged_source() {
        let path = Path::new("/tmp/v3_parse.sy");
        let source = "func foo() -> Int { return 1 }\n";
        let mut cache = ModuleQueryCache::default();
        cache.touch_source(path, source);
        assert!(cache.lookup_parse(path, source).is_some());
        let edited = "func foo() -> Int { return 2 }\n";
        cache.touch_source(path, edited);
        assert!(cache.lookup_parse(path, source).is_none());
        assert!(cache.lookup_parse(path, edited).is_some());
    }

    #[test]
    fn query_v3_import_boundary_invalidates_semantic() {
        let path = Path::new("/tmp/v3_import.sy");
        let mut cache = ModuleQueryCache::default();
        let v1 = "import Other\nfunc main() { }\n";
        let v2 = "import OtherExtra\nfunc main() { }\n";
        cache.touch_source(path, v1);
        cache.record_semantic_decl(path, "main", "abc", 0);
        assert!(cache.semantic_decl_hit(path, "main", "abc"));
        cache.touch_source(path, v2);
        assert!(!cache.semantic_decl_hit(path, "main", "abc"));
    }

    #[test]
    fn query_v5_rename_smoke_under_10ms() {
        use std::time::Instant;
        let path = Path::new("/tmp/v5_rename.sy");
        let mut cache = ModuleQueryCache::default();
        let source = "func helper() -> Int { return 0 }\nfunc main() { helper() }\n";
        cache.touch_source(path, source);
        let start = Instant::now();
        let _ = cache.touch_source(path, &source.replace("helper", "assist"));
        let elapsed = start.elapsed();
        if !cfg!(debug_assertions) {
            assert!(
                elapsed.as_millis() < 10,
                "rename invalidation took {:?} (target <10ms)",
                elapsed
            );
        }
    }

    #[test]
    fn query_v3_single_function_edit_smoke_under_50ms() {
        use std::time::Instant;
        let path = Path::new("/tmp/v3_bench.sy");
        let mut cache = ModuleQueryCache::default();
        let mut source = String::from("func helper() -> Int { return 0 }\nfunc main() {\n");
        for i in 0..300 {
            source.push_str(&format!("    let v{i} = {i}\n"));
        }
        source.push_str("    return helper()\n}\n");
        cache.touch_source(path, &source);
        let edited = source.replacen("return 0", "return 42", 1);
        let start = Instant::now();
        let changed = cache.touch_source(path, &edited);
        let elapsed = start.elapsed();
        assert!(
            changed.iter().any(|d| d.name.as_str() == "helper"),
            "expected helper in changed decls: {changed:?}"
        );
        let limit_ms = 50;
        if !cfg!(debug_assertions) {
            assert!(
                elapsed.as_millis() < limit_ms as u128,
                "single-function edit took {:?} (target <{limit_ms}ms)",
                elapsed
            );
        }
    }

    #[test]
    fn query_v4_cross_module_public_edit_smoke_under_80ms() {
        use std::time::Instant;
        let math = Path::new("/tmp/v4_math.sy");
        let app = Path::new("/tmp/v4_app.sy");
        let mut cache = ModuleQueryCache::default();
        let math_v1 = "public func add(a: Int, b: Int) -> Int { return a + b }\n";
        let math_v2 = "public func add(a: Int, b: Int) -> Int { return a + b + 1 }\n";
        let app_src = "import math\nfunc main() { let x = math.add(1, 2) }\n";
        cache.touch_source(math, math_v1);
        cache.touch_source(app, app_src);
        cache.register_import_edge(app, math);
        cache.record_semantic_decl(app, "main", "hash1", 0);
        assert!(cache.semantic_decl_hit(app, "main", "hash1"));
        let start = Instant::now();
        cache.touch_source(math, math_v2);
        let elapsed = start.elapsed();
        assert!(!cache.semantic_decl_hit(app, "main", "hash1"));
        let limit_ms = 80;
        if !cfg!(debug_assertions) {
            assert!(
                elapsed.as_millis() < limit_ms as u128,
                "cross-module invalidation took {:?} (target <{limit_ms}ms)",
                elapsed
            );
        }
    }
}
