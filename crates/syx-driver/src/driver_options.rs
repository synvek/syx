//! SYX driver options / module graph type hub.
//! Extracted in S2-I224 (zero semantics).
//! Hard constraint: no compile_*/native_build/cache_fingerprint imports.

use crate::driver_error_at;
use indexmap::IndexMap;
use std::path::{Path, PathBuf};
use syx_codegen::CompiledProgram;
use syx_diagnostics::{SourceFile, Span, SyxResult};
use syx_pkg::{InstalledPackage, SyxManifest};

#[derive(Debug, Clone)]
pub struct ModuleUnit {
    pub path: PathBuf,
    pub source: SourceFile,
    pub imported_modules: Vec<smol_str::SmolStr>,
    pub program: syx_codegen::CompiledProgram,
}

#[derive(Debug, Clone, Default)]
pub struct ModuleGraph {
    pub modules: IndexMap<PathBuf, ModuleUnit>,
    pub order: Vec<PathBuf>,
    /// Precomputed stdlib extension methods (from prelude snapshot). Empty → parse fallback.
    pub stdlib_extension_methods: indexmap::IndexSet<(smol_str::SmolStr, smol_str::SmolStr)>,
}

/// Native code generator backend selected by `--backend` / `SYX_BACKEND`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NativeBackend {
    #[default]
    Llvm,
}

impl NativeBackend {
    pub fn parse(value: &str) -> SyxResult<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "cranelift" | "cf" => Err(driver_error_at(
                "Cranelift backend removed in SYX 37.0 (Phase 59); use --backend llvm",
                None,
                Span::DUMMY,
                "B001",
            )),
            "llvm" => Ok(Self::Llvm),
            other => Err(driver_error_at(
                format!("unknown native backend '{other}' (expected 'llvm')"),
                None,
                Span::DUMMY,
                "B002",
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        "llvm"
    }

    /// Phase 59: LLVM is the sole production native backend.
    pub fn default_for_env() -> Self {
        Self::Llvm
    }
}

/// Resolve native backend from CLI flag, then `SYX_BACKEND`, else ADR-036/ADR-034 default.
pub fn resolve_native_backend(cli: Option<&str>) -> SyxResult<NativeBackend> {
    if let Some(value) = cli {
        return NativeBackend::parse(value);
    }
    if let Ok(value) = std::env::var("SYX_BACKEND") {
        return NativeBackend::parse(&value);
    }
    Ok(NativeBackend::Llvm)
}

/// Driver configuration for module resolution.
#[derive(Debug, Clone)]
pub struct DriverOptions {
    /// Extra directories to search for modules (`;`-separated on the CLI).
    pub module_dirs: Vec<PathBuf>,
    /// When true, embedded stdlib modules (`Math`, `IO`, …) take highest priority.
    pub enable_std: bool,
    /// Skip lockfile requirement (for single-file tests without dependencies).
    pub skip_install_check: bool,
    /// Resolved packages from syx.lock (set by compile_project).
    pub resolved_packages: Option<IndexMap<String, InstalledPackage>>,
    /// Enable strict Swift 6-style concurrency diagnostics (from `[concurrency] strict`).
    pub strict_concurrency: bool,
    /// Module default isolation (`MainActor` | `nonisolated`) from `syx.toml`.
    pub default_isolation: Option<String>,
    /// Link against a slim `syx-rt` build (`--no-default-features`) for smaller binaries.
    pub native_minimal: bool,
    /// Omit HTTP host functions from the linked `syx-rt` (`--no-http`).
    pub native_no_http: bool,
    /// Omit file I/O host functions from the linked `syx-rt` (`--no-file`).
    pub native_no_file: bool,
    /// Native codegen backend (`llvm` only in SYX 36.0+).
    pub native_backend: NativeBackend,
    /// Debug vs release artifact layout under `target/{debug,release}/`.
    pub build_profile: BuildProfile,
    /// WASM hybrid AOT emit (`--wasm --aot`).
    pub wasm_aot: bool,
    /// True while executing a build.syb task target (prevents recursive task execution).
    pub in_build_syx_task: bool,
    /// When true, read/write `target/cache/` (entry MIR + optional prelude disk snapshot).
    /// Default **false** so `run` / library / edge hosts have no filesystem side effects.
    /// CLI `syx build` enables this; `syx run --with-cache` opts in.
    pub incremental_cache: bool,
    /// When true, re-lower stdlib into unified MIR (needed for native/wasm emit).
    /// Default **true**; `syx run` sets **false** so interpret keeps snapshot bytecode
    /// (VM-image fastpath — skip prelude MIR rebuild).
    pub stdlib_mir: bool,
}

/// SYX 38.0 build profile for `target/debug` vs `target/release`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildProfile {
    #[default]
    Debug,
    Release,
}

impl BuildProfile {
    pub fn dir_name(self) -> &'static str {
        match self {
            BuildProfile::Debug => "debug",
            BuildProfile::Release => "release",
        }
    }

    pub fn is_release(self) -> bool {
        matches!(self, BuildProfile::Release)
    }

    pub fn cache_tag(self) -> &'static str {
        self.dir_name()
    }

    pub fn rt_profile_name(self) -> &'static str {
        self.dir_name()
    }

    pub fn clang_ir_args(self) -> &'static [&'static str] {
        if self.lto_enabled() {
            return &["-O2", "-DNDEBUG", "-flto=thin"];
        }
        match self {
            BuildProfile::Debug => &["-O0", "-g"],
            BuildProfile::Release => &["-O2", "-DNDEBUG"],
        }
    }

    pub fn lto_enabled(self) -> bool {
        matches!(self, BuildProfile::Release)
            && std::env::var("SYX_NATIVE_LTO")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
    }

    pub fn link_profile_args(self) -> Vec<&'static str> {
        let mut args = match self {
            BuildProfile::Debug => Vec::new(),
            BuildProfile::Release => {
                #[cfg(target_os = "macos")]
                {
                    vec!["-Wl,-dead_strip"]
                }
                #[cfg(target_os = "linux")]
                {
                    vec!["-Wl,--gc-sections"]
                }
                #[cfg(not(any(target_os = "macos", target_os = "linux")))]
                {
                    Vec::new()
                }
            }
        };
        if self.lto_enabled() {
            args.push("-flto");
        }
        args
    }

    pub fn from_release_flag(release: bool) -> Self {
        if release {
            BuildProfile::Release
        } else {
            BuildProfile::Debug
        }
    }
}

/// Project root for a path (directory or single `.sy` file).
pub fn project_root_for(path: &Path) -> PathBuf {
    if path.is_dir() {
        path.to_path_buf()
    } else {
        path.parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    }
}

pub fn project_target_root(project_root: &Path) -> PathBuf {
    project_root.join("target")
}

pub fn project_cache_dir(project_root: &Path) -> PathBuf {
    project_target_root(project_root).join("cache")
}

pub fn project_build_dir(project_root: &Path, profile: BuildProfile) -> PathBuf {
    project_target_root(project_root).join(profile.dir_name())
}

/// Host executable file name for native AOT (`app` / `{stem}`, plus `.exe` on Windows).
pub fn native_executable_file_name(stem: &str) -> String {
    if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.to_string()
    }
}

pub fn default_native_executable(project_root: &Path, profile: BuildProfile) -> PathBuf {
    project_build_dir(project_root, profile).join(native_executable_file_name("app"))
}

/// Default native executable path when building a single `.sy` file (stem-based, not `app`).
pub fn default_native_executable_for_source(source: &Path, profile: BuildProfile) -> PathBuf {
    let root = project_root_for(source);
    let stem = source.file_stem().and_then(|s| s.to_str()).unwrap_or("app");
    project_build_dir(&root, profile).join(native_executable_file_name(stem))
}

pub fn default_native_object(project_root: &Path, profile: BuildProfile) -> PathBuf {
    project_build_dir(project_root, profile).join("main.o")
}

/// Default native object path when building a single `.sy` file (`{stem}.o`).
pub fn default_native_object_for_source(source: &Path, profile: BuildProfile) -> PathBuf {
    let root = project_root_for(source);
    let stem = source
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("main");
    project_build_dir(&root, profile).join(format!("{stem}.o"))
}

impl Default for DriverOptions {
    fn default() -> Self {
        Self {
            module_dirs: Vec::new(),
            enable_std: true,
            skip_install_check: false,
            resolved_packages: None,
            strict_concurrency: false,
            default_isolation: None,
            native_minimal: false,
            native_no_http: false,
            native_no_file: false,
            native_backend: NativeBackend::default_for_env(),
            build_profile: BuildProfile::Debug,
            wasm_aot: syx_config::wasm_aot_enabled(),
            in_build_syx_task: false,
            incremental_cache: false,
            stdlib_mir: true,
        }
    }
}

/// Parse semicolon-separated module search paths.
pub fn parse_module_dirs(value: &str) -> Vec<PathBuf> {
    value
        .split(';')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(PathBuf::from)
        .collect()
}

#[derive(Debug, Clone)]
pub struct CompiledProject {
    pub manifest: SyxManifest,
    pub entry: ModuleUnit,
    pub graph: ModuleGraph,
}
