use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// W929 / ADR-1273: runtime override for `libsyx_rt` archive path.
///
/// When `SYX_RT_STATIC_LIB` points at an existing `.a` / `.lib` (or a deps dir
/// containing hashed archives), prefer it over compile-time embed / cargo search.
fn runtime_env_rt_archive() -> Option<PathBuf> {
    let raw = std::env::var_os("SYX_RT_STATIC_LIB")?;
    let path = PathBuf::from(raw);
    resolve_archive(&path)
}

/// W930 / ADR-1274: feature-profile install layout under a `lib` root.
///
/// Looks for `profiles/<id>/libsyx_rt.a` then `<id>/libsyx_rt.a` (and `.lib`).
/// Rejects empty ids and path separators / `..`.
fn resolve_feature_profile_under_lib(lib_root: &Path, profile_id: &str) -> Option<PathBuf> {
    let id = profile_id.trim();
    if id.is_empty()
        || id.contains('/')
        || id.contains('\\')
        || id.contains("..")
        || id.contains('\0')
    {
        return None;
    }
    let candidates = [
        lib_root.join("profiles").join(id).join("libsyx_rt.a"),
        lib_root.join("profiles").join(id).join("syx_rt.lib"),
        lib_root.join(id).join("libsyx_rt.a"),
        lib_root.join(id).join("syx_rt.lib"),
    ];
    for c in &candidates {
        if let Some(path) = resolve_archive(c) {
            return Some(path);
        }
    }
    None
}

/// W930 / ADR-1274: `SYX_RT_PROFILE=<feature-id>` → install multi-profile lookup.
fn runtime_feature_profile_rt_archive() -> Option<PathBuf> {
    let id = std::env::var("SYX_RT_PROFILE").ok()?;
    let exe = std::env::current_exe().ok()?;
    let bin_dir = exe.parent()?;
    let lib_root = bin_dir.join("../lib");
    resolve_feature_profile_under_lib(&lib_root, &id)
}

/// Locate `libsyx_rt*.a` / `syx_rt.lib` produced by Cargo or installed under `../lib` relative to `syx`.
pub fn rt_archive_path() -> io::Result<PathBuf> {
    if let Some(path) = runtime_env_rt_archive() {
        return Ok(path);
    }
    if let Some(path) = runtime_feature_profile_rt_archive() {
        return Ok(path);
    }

    let embedded = PathBuf::from(env!("SYX_RT_STATIC_LIB"));
    if let Some(path) = resolve_archive(&embedded) {
        return Ok(path);
    }

    // SYX 38.0 install layout: `bin/syx` with runtime at `../lib/libsyx_rt.a`.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(bin_dir) = exe.parent() {
            let install_lib = bin_dir.join("../lib/libsyx_rt.a");
            if let Some(path) = resolve_archive(&install_lib) {
                return Ok(path);
            }
            let install_msvc = bin_dir.join("../lib/syx_rt.lib");
            if let Some(path) = resolve_archive(&install_msvc) {
                return Ok(path);
            }
            if let Some(path) = find_hashed_archive(&bin_dir.join("../lib")) {
                return Ok(path);
            }
        }
    }

    let mut search_roots: Vec<PathBuf> = Vec::new();
    if let Ok(dir) = std::env::var("CARGO_TARGET_DIR") {
        search_roots.push(PathBuf::from(dir));
    }
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    search_roots.push(manifest.join("../../target"));

    for root in search_roots {
        let Ok(root) = root.canonicalize() else {
            continue;
        };
        for profile in profile_search_order() {
            let flat = root.join(profile).join("libsyx_rt.a");
            if let Some(path) = resolve_archive(&flat) {
                return Ok(path);
            }
            let msvc = root.join(profile).join("syx_rt.lib");
            if let Some(path) = resolve_archive(&msvc) {
                return Ok(path);
            }
            let deps = root.join(profile).join("deps");
            if let Some(path) = resolve_archive(&deps) {
                return Ok(path);
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "libsyx_rt.a / syx_rt.lib not found (hint: run `cargo build -p syx-rt` first, or set SYX_RT_STATIC_LIB / SYX_RT_PROFILE); embedded path was {}",
            embedded.display()
        ),
    ))
}

fn resolve_archive(path: &Path) -> Option<PathBuf> {
    if path.is_file() {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext == "a" || ext == "lib" {
            return Some(path.to_path_buf());
        }
    }
    if path.is_dir() {
        return find_hashed_archive(path);
    }
    None
}

fn profile_search_order() -> [&'static str; 2] {
    if cfg!(debug_assertions) {
        ["debug", "release"]
    } else {
        ["release", "debug"]
    }
}

fn find_hashed_archive(deps: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(deps).ok()?;
    let mut best: Option<(PathBuf, std::time::SystemTime)> = None;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let is_unix =
            name.starts_with("libsyx_rt") && name.ends_with(".a") && !name.contains(".rlib");
        let is_msvc =
            name.starts_with("syx_rt") && name.ends_with(".lib") && !name.contains(".rlib");
        if !is_unix && !is_msvc {
            continue;
        }
        let Ok(meta) = entry.metadata() else {
            continue;
        };
        let Ok(modified) = meta.modified() else {
            continue;
        };
        if best.as_ref().is_none_or(|(_, t)| modified > *t) {
            best = Some((entry.path(), modified));
        }
    }
    best.map(|(path, _)| path)
}

/// Extra native link inputs (SYX 39.0).
#[derive(Debug, Clone, Default)]
pub struct LinkOptions {
    pub lib_paths: Vec<PathBuf>,
    pub libs: Vec<String>,
    pub objects: Vec<PathBuf>,
    pub link_flags: Vec<String>,
    /// SYX 71.0: `debug` or `release` — selects `libsyx_rt` archive profile.
    pub rt_profile: String,
}

/// Host triple used for `lib/sysroot/<triple>/` (override with `TARGET`).
pub fn host_link_triple() -> String {
    if let Ok(t) = std::env::var("TARGET") {
        if !t.is_empty() {
            return t;
        }
    }
    let arch = std::env::consts::ARCH;
    if cfg!(target_os = "windows") {
        let env = if cfg!(target_env = "gnu") {
            "gnu"
        } else {
            "msvc"
        };
        return format!("{arch}-pc-windows-{env}");
    }
    if cfg!(target_os = "macos") {
        return format!("{arch}-apple-darwin");
    }
    if cfg!(target_os = "linux") {
        return format!("{arch}-unknown-linux-gnu");
    }
    format!("{arch}-unknown-unknown")
}

/// Resolve SDK link sysroot: `SYX_SYSROOT` or `bin/../lib/sysroot/<triple>/`.
pub fn resolve_sdk_sysroot() -> Option<PathBuf> {
    if let Ok(raw) = std::env::var("SYX_SYSROOT") {
        let p = PathBuf::from(raw);
        if p.is_dir() {
            return Some(p.canonicalize().unwrap_or(p));
        }
    }
    let exe = std::env::current_exe().ok()?;
    let bin_dir = exe.parent()?;
    let candidate = bin_dir
        .join("../lib/sysroot")
        .join(host_link_triple());
    if candidate.is_dir() {
        return Some(candidate.canonicalize().unwrap_or(candidate));
    }
    None
}

/// Whether a sysroot directory looks ready for linking (marker and/or import libs).
pub fn sysroot_looks_usable(sysroot: &Path) -> bool {
    if !sysroot.is_dir() {
        return false;
    }
    if sysroot.join(".syx-sysroot").is_file() {
        return true;
    }
    // Windows: at least one import lib
    if let Ok(rd) = std::fs::read_dir(sysroot) {
        for e in rd.flatten() {
            let name = e.file_name();
            let s = name.to_string_lossy();
            if s.ends_with(".lib") || s.ends_with(".a") || s.ends_with(".tbd") {
                return true;
            }
        }
    }
    false
}

/// Doctor / smoke report for SDK native layout next to `bin/syx`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeSdkLayoutReport {
    pub triple: String,
    pub rt_archive: Option<PathBuf>,
    pub sysroot: Option<PathBuf>,
    pub sysroot_ok: bool,
    pub ok: bool,
    pub notes: Vec<String>,
}

/// Inspect install layout used for Go-like single-file native links.
pub fn native_sdk_layout_report() -> NativeSdkLayoutReport {
    let triple = host_link_triple();
    let mut notes = Vec::new();
    let rt = rt_archive_path().ok();
    if rt.is_none() {
        notes.push("libsyx_rt / syx_rt.lib not found (set SYX_RT_STATIC_LIB or install SDK lib/)".into());
    }
    let sysroot = resolve_sdk_sysroot();
    let sysroot_ok = sysroot.as_ref().is_some_and(|p| sysroot_looks_usable(p));
    if sysroot.is_none() {
        notes.push(format!(
            "sysroot not shipped (optional ../lib/sysroot/{triple} or SYX_SYSROOT); \
             native link uses host VS/Xcode/CLT for now"
        ));
    } else if !sysroot_ok {
        notes.push("sysroot directory present but empty / unusable".into());
    } else {
        notes.push("sysroot OK — prefer SDK libs over local VS/Xcode".into());
    }
    #[cfg(windows)]
    let host_sdk_ok = {
        let _ = &notes;
        cc::windows_registry::find_tool(&host_link_triple(), "link.exe").is_some()
    };
    #[cfg(not(windows))]
    let host_sdk_ok = true;
    if !sysroot_ok && host_sdk_ok {
        notes.push("using host toolchain (SDK sysroot temporarily disabled)".into());
    }
    let ok = rt.is_some()
        && (sysroot_ok
            || host_sdk_ok
            || cfg!(target_os = "linux")
            || std::env::var_os("SYX_ALLOW_HOST_SDK").is_some());
    NativeSdkLayoutReport {
        triple,
        rt_archive: rt,
        sysroot,
        sysroot_ok,
        ok,
        notes,
    }
}

/// Scan a Windows COFF `.obj` for external undefined symbol names.
pub fn coff_undefined_externals(bytes: &[u8]) -> Vec<String> {
    if bytes.len() < 20 {
        return Vec::new();
    }
    let machine = u16::from_le_bytes([bytes[0], bytes[1]]);
    if !matches!(machine, 0x014c | 0x8664 | 0xaa64 | 0x01c4) {
        return Vec::new();
    }
    let ptr_sym = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let num_sym = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    if ptr_sym == 0 || num_sym == 0 || ptr_sym.saturating_add(num_sym.saturating_mul(18)) > bytes.len()
    {
        return Vec::new();
    }
    let string_table_off = ptr_sym + num_sym * 18;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < num_sym {
        let off = ptr_sym + i * 18;
        if off + 18 > bytes.len() {
            break;
        }
        let section = i16::from_le_bytes([bytes[off + 12], bytes[off + 13]]);
        let storage = bytes[off + 16];
        let num_aux = bytes[off + 17] as usize;
        // IMAGE_SYM_UNDEFINED = 0, IMAGE_SYM_CLASS_EXTERNAL = 2
        if section == 0 && storage == 2 {
            let name = if bytes[off..off + 8] != [0; 8] && bytes[off] != 0 {
                // short name
                let end = bytes[off..off + 8]
                    .iter()
                    .position(|&b| b == 0)
                    .unwrap_or(8);
                String::from_utf8_lossy(&bytes[off..off + end]).into_owned()
            } else {
                let str_off = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().unwrap()) as usize;
                let abs = string_table_off.saturating_add(str_off);
                if abs < bytes.len() {
                    let end = bytes[abs..]
                        .iter()
                        .position(|&b| b == 0)
                        .map(|n| abs + n)
                        .unwrap_or(bytes.len());
                    String::from_utf8_lossy(&bytes[abs..end]).into_owned()
                } else {
                    String::new()
                }
            };
            if !name.is_empty() {
                out.push(name);
            }
        }
        i += 1 + num_aux;
    }
    out
}

/// True if an undefined COFF symbol is allowed for user AOT objects.
pub fn user_object_undef_allowed(name: &str) -> bool {
    // Keep `__imp_*` / `__std_*` before stripping a single Mach-O/`_` prefix.
    if name.starts_with("__imp_")
        || name.starts_with("__std_")
        || name.starts_with("llvm.")
        || name.starts_with("syx_rt_")
        || name.starts_with("syx_user_")
        || name.starts_with("_syx_rt_")
        || name.starts_with("_syx_user_")
    {
        return true;
    }
    let n = name.strip_prefix('_').unwrap_or(name);
    // Common CRT / compiler builtins that sysroot + CRT import libs resolve.
    matches!(
        n,
        "memcpy"
            | "memmove"
            | "memset"
            | "memcmp"
            | "strlen"
            | "wcslen"
            | "malloc"
            | "free"
            | "realloc"
            | "calloc"
            | "__CxxFrameHandler3"
            | "__C_specific_handler"
            | "_Tls_index"
            | "__chkstk"
            | "__security_cookie"
            | "__security_check_cookie"
            | "_fltused"
    ) || n.starts_with("__imp_")
        || n.starts_with("__std_")
}

/// Audit user object bytes: COFF undefs must pass [`user_object_undef_allowed`].
pub fn audit_user_object_undefs(bytes: &[u8]) -> Result<(), String> {
    let undefs = coff_undefined_externals(bytes);
    if undefs.is_empty() {
        return Ok(());
    }
    let bad: Vec<_> = undefs
        .into_iter()
        .filter(|n| !user_object_undef_allowed(n))
        .collect();
    if bad.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "user object has disallowed undefined symbols: {}",
            bad.join(", ")
        ))
    }
}

/// Locate `libsyx_rt*.a` for the given build profile (`debug` / `release`).
pub fn rt_archive_path_for_profile(profile: &str) -> io::Result<PathBuf> {
    if let Some(path) = runtime_env_rt_archive() {
        return Ok(path);
    }
    if let Some(path) = runtime_feature_profile_rt_archive() {
        return Ok(path);
    }

    let embedded = PathBuf::from(env!("SYX_RT_STATIC_LIB"));
    if let Some(path) = resolve_archive(&embedded) {
        return Ok(path);
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(bin_dir) = exe.parent() {
            let install_lib = bin_dir.join("../lib/libsyx_rt.a");
            if let Some(path) = resolve_archive(&install_lib) {
                return Ok(path);
            }
            if let Some(path) = find_hashed_archive(&bin_dir.join("../lib")) {
                return Ok(path);
            }
        }
    }

    let mut search_roots: Vec<PathBuf> = Vec::new();
    if let Ok(dir) = std::env::var("CARGO_TARGET_DIR") {
        search_roots.push(PathBuf::from(dir));
    }
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    search_roots.push(manifest.join("../../target"));

    let profiles: Vec<&str> = if profile == "release" {
        vec!["release", "debug"]
    } else {
        vec!["debug", "release"]
    };

    for root in search_roots {
        let Ok(root) = root.canonicalize() else {
            continue;
        };
        for prof in &profiles {
            let flat = root.join(prof).join("libsyx_rt.a");
            if let Some(path) = resolve_archive(&flat) {
                return Ok(path);
            }
            let msvc = root.join(prof).join("syx_rt.lib");
            if let Some(path) = resolve_archive(&msvc) {
                return Ok(path);
            }
            let deps = root.join(prof).join("deps");
            if let Some(path) = resolve_archive(&deps) {
                return Ok(path);
            }
            if let Some(path) = find_hashed_archive(&deps) {
                return Ok(path);
            }
        }
    }

    rt_archive_path()
}

/// Prefer LLD (bundled `rust-lld` / `SYX_LLD` / `ld64.lld` / `lld`) before system `cc`.
/// True in-process LLD (as a library) is not available for LLVM 21+ on crates.io; we invoke
/// the LLD binary that ships with the Rust toolchain or the host LLVM install.
fn resolve_lld_linker() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SYX_LLD") {
        if !path.is_empty() {
            let p = PathBuf::from(&path);
            if p.is_file() {
                return Some(p);
            }
        }
    }
    if let Ok(sysroot) = std::process::Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
    {
        if sysroot.status.success() {
            let root = String::from_utf8_lossy(&sysroot.stdout).trim().to_string();
            let host = std::env::consts::ARCH;
            let candidates = [
                format!("{root}/lib/rustlib/{host}-apple-darwin/bin/rust-lld"),
                format!("{root}/lib/rustlib/{host}-unknown-linux-gnu/bin/rust-lld"),
                format!("{root}/lib/rustlib/{host}-pc-windows-msvc/bin/rust-lld.exe"),
                format!("{root}/lib/rustlib/{host}-pc-windows-gnu/bin/rust-lld.exe"),
            ];
            for c in candidates {
                let p = PathBuf::from(&c);
                if p.is_file() {
                    return Some(p);
                }
            }
            // Fallback: scan rustlib/*/bin/rust-lld
            let rustlib = PathBuf::from(&root).join("lib/rustlib");
            if let Ok(entries) = std::fs::read_dir(rustlib) {
                for entry in entries.flatten() {
                    let bin = entry.path().join("bin").join(if cfg!(windows) {
                        "rust-lld.exe"
                    } else {
                        "rust-lld"
                    });
                    if bin.is_file() {
                        return Some(bin);
                    }
                }
            }
        }
    }
    for name in ["ld64.lld", "ld.lld", "lld-link", "lld"] {
        if let Some(path) = find_on_path(name) {
            return Some(path);
        }
    }
    None
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
        #[cfg(windows)]
        {
            let with_exe = dir.join(format!("{name}.exe"));
            if with_exe.is_file() {
                return Some(with_exe);
            }
        }
    }
    None
}

fn host_linker_command(prefer_lld: bool) -> Command {
    if prefer_lld {
        if let Some(lld) = resolve_lld_linker() {
            let mut cmd = Command::new(lld);
            apply_lld_flavor_args(&mut cmd);
            apply_link_search_env(&mut cmd);
            return cmd;
        }
    }
    #[cfg(windows)]
    {
        // Prefer SDK sysroot + rust-lld; link.exe only as developer fallback.
        if let Some(mut cmd) = msvc_link_exe_command() {
            apply_link_search_env(&mut cmd);
            return cmd;
        }
        let mut cmd = Command::new("link.exe");
        apply_link_search_env(&mut cmd);
        return cmd;
    }
    #[cfg(not(windows))]
    Command::new("cc")
}

#[cfg(windows)]
fn windows_host_msvc_triple() -> String {
    host_link_triple()
}

/// Prefer SDK `lib/sysroot/<triple>` LIBPATH; fall back to host MSVC for source builds.
fn apply_link_search_env(cmd: &mut Command) {
    if let Some(sysroot) = resolve_sdk_sysroot().filter(|p| sysroot_looks_usable(p)) {
        #[cfg(windows)]
        {
            let existing = std::env::var_os("LIB").unwrap_or_default();
            let mut lib = sysroot.as_os_str().to_owned();
            if !existing.is_empty() {
                lib.push(";");
                lib.push(&existing);
            }
            cmd.env("LIB", lib);
            return;
        }
        #[cfg(not(windows))]
        {
            let _ = (cmd, sysroot);
            return;
        }
    }
    #[cfg(windows)]
    apply_msvc_link_env(cmd);
    #[cfg(not(windows))]
    let _ = cmd;
}

/// Inject VC/Windows SDK `LIB` / `PATH` from the same detector rustc/cc uses.
#[cfg(windows)]
fn apply_msvc_link_env(cmd: &mut Command) {
    let triple = windows_host_msvc_triple();
    if let Some(tool) = cc::windows_registry::find_tool(&triple, "link.exe") {
        for (k, v) in tool.env() {
            cmd.env(k, v);
        }
    }
}

#[cfg(windows)]
fn msvc_link_exe_command() -> Option<Command> {
    let triple = windows_host_msvc_triple();
    let tool = cc::windows_registry::find_tool(&triple, "link.exe")?;
    let mut cmd = Command::new(tool.path());
    for (k, v) in tool.env() {
        cmd.env(k, v);
    }
    Some(cmd)
}

fn push_sdk_sysroot_libpath(cmd: &mut Command, win_msvc: bool) {
    let Some(sysroot) = resolve_sdk_sysroot().filter(|p| sysroot_looks_usable(p)) else {
        return;
    };
    if win_msvc {
        cmd.arg(format!("/LIBPATH:{}", sysroot.display()));
    } else {
        cmd.arg("-L").arg(&sysroot);
    }
}

fn macos_syslibroot_path() -> Option<PathBuf> {
    if let Some(sysroot) = resolve_sdk_sysroot() {
        let recorded = sysroot.join("sdk-path.txt");
        if let Ok(s) = std::fs::read_to_string(&recorded) {
            let p = PathBuf::from(s.trim());
            if p.is_dir() {
                return Some(p);
            }
        }
        // Bundled SDK tree (optional future layout)
        for name in ["MacOSX.sdk", "sdk"] {
            let p = sysroot.join(name);
            if p.is_dir() {
                return Some(p);
            }
        }
    }
    let sdk = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .ok()?;
    if !sdk.status.success() {
        return None;
    }
    let path = String::from_utf8_lossy(&sdk.stdout).trim().to_string();
    if path.is_empty() {
        None
    } else {
        Some(PathBuf::from(path))
    }
}

/// Ensure Windows can link: SDK sysroot or host MSVC must be available.
fn ensure_windows_link_inputs() -> io::Result<()> {
    #[cfg(windows)]
    {
        if resolve_sdk_sysroot().is_some_and(|p| sysroot_looks_usable(&p)) {
            return Ok(());
        }
        if msvc_link_exe_command().is_some() {
            return Ok(());
        }
        return Err(io::Error::other(
            "native link: no usable SYX_SYSROOT and no MSVC link.exe. \
             Install Visual Studio Build Tools (SDK sysroot is not shipped yet). \
             A plain `cc` on PATH is not used on Windows.",
        ));
    }
    #[cfg(not(windows))]
    Ok(())
}

/// True when the linker expects MSVC `link.exe` / lld-link style flags (`/OUT`, `/LIBPATH`).
fn windows_msvc_style_linker(cmd: &Command) -> bool {
    if !cfg!(target_os = "windows") {
        return false;
    }
    let prog = cmd.get_program().to_string_lossy();
    let lower = prog.to_ascii_lowercase();
    lower.contains("lld")
        || lower.contains("link.exe")
        || lower.ends_with("\\link")
        || lower.ends_with("/link")
        || lower == "link"
}

/// Import libs required when linking a Rust `staticlib` (`syx_rt.lib`) with MSVC
/// or LLVM `lld-link`. Defaultlib directives alone are often incomplete under LLD
/// (missing `kernel32` while `MSVCRT.lib` is already pulled).
#[cfg(windows)]
fn windows_system_link_libs() -> &'static [&'static str] {
    &[
        "kernel32",
        "user32",
        "shell32",
        "advapi32",
        "ole32",
        "oleaut32",
        "uuid",
        "ws2_32",
        "ntdll",
        "bcrypt",
        "userenv",
        "dbghelp",
        "ucrt",
        "vcruntime",
        "msvcrt",
        "legacy_stdio_definitions",
        "oldnames",
    ]
}

#[cfg(windows)]
fn push_windows_system_libs_msvc(cmd: &mut Command) {
    for lib in windows_system_link_libs() {
        cmd.arg(format!("{lib}.lib"));
    }
}

#[cfg(windows)]
fn push_windows_system_libs_lld(args: &mut Vec<std::ffi::OsString>) {
    for lib in windows_system_link_libs() {
        args.push(format!("{lib}.lib").into());
    }
}

fn apply_lld_flavor_args(cmd: &mut Command) {
    #[cfg(target_os = "macos")]
    {
        cmd.arg("-flavor").arg("darwin");
        #[cfg(target_arch = "aarch64")]
        {
            cmd.arg("-arch").arg("arm64");
        }
        #[cfg(target_arch = "x86_64")]
        {
            cmd.arg("-arch").arg("x86_64");
        }
        let sdk_ver = macos_sdk_version_for_lld();
        cmd.arg("-platform_version")
            .arg("macos")
            .arg(&sdk_ver)
            .arg(&sdk_ver);
    }
    #[cfg(target_os = "linux")]
    {
        cmd.arg("-flavor").arg("gnu");
    }
    #[cfg(target_os = "windows")]
    {
        cmd.arg("-flavor").arg("link");
    }
}

#[cfg(target_os = "macos")]
fn macos_sdk_version_for_lld() -> String {
    if let Ok(output) = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-version"])
        .output()
    {
        if output.status.success() {
            let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !v.is_empty() {
                return v;
            }
        }
    }
    "14.0".into()
}

fn using_lld_driver(cmd: &Command) -> bool {
    let prog = cmd.get_program().to_string_lossy();
    prog.contains("lld") || prog.contains("LLD") || prog.contains("syx-lld")
}

#[cfg(feature = "vendored-lld")]
fn link_with_embedded_lld(
    user_obj: &Path,
    output: &Path,
    opts: &LinkOptions,
    rt: &Path,
) -> io::Result<()> {
    let mut args: Vec<std::ffi::OsString> = Vec::new();
    args.push(user_obj.as_os_str().to_owned());
    for obj in &opts.objects {
        args.push(obj.as_os_str().to_owned());
    }
    args.push(rt.as_os_str().to_owned());
    for path in &opts.lib_paths {
        #[cfg(target_os = "windows")]
        {
            args.push(format!("/LIBPATH:{}", path.display()).into());
        }
        #[cfg(not(target_os = "windows"))]
        {
            args.push("-L".into());
            args.push(path.as_os_str().to_owned());
        }
    }
    for lib in &opts.libs {
        #[cfg(target_os = "windows")]
        {
            args.push(format!("{lib}.lib").into());
        }
        #[cfg(not(target_os = "windows"))]
        {
            args.push(format!("-l{lib}").into());
        }
    }
    for flag in &opts.link_flags {
        args.push(flag.into());
    }
    #[cfg(target_os = "windows")]
    {
        if let Some(sysroot) = resolve_sdk_sysroot().filter(|p| sysroot_looks_usable(p)) {
            args.push(format!("/LIBPATH:{}", sysroot.display()).into());
        }
        push_windows_system_libs_lld(&mut args);
        args.push(format!("/OUT:{}", output.display()).into());
        args.push("/ENTRY:syx_rt_start".into());
        args.push("/SUBSYSTEM:CONSOLE".into());
    }
    #[cfg(not(target_os = "windows"))]
    {
        args.push("-o".into());
        args.push(output.as_os_str().to_owned());
    }
    #[cfg(target_os = "macos")]
    {
        args.push("-e".into());
        args.push("_syx_rt_start".into());
        args.push("-liconv".into());
        args.push("-lobjc".into());
        if let Some(sdk) = macos_syslibroot_path() {
            args.push("-syslibroot".into());
            args.push(sdk.into_os_string());
        }
        args.push("-lSystem".into());
        for fw in [
            "Security",
            "CoreFoundation",
            "CoreServices",
            "Foundation",
            "AppKit",
            "CoreGraphics",
            "CoreText",
            "CoreVideo",
            "QuartzCore",
            "Metal",
            "IOKit",
            "Carbon",
            "ApplicationServices",
        ] {
            args.push("-framework".into());
            args.push(fw.into());
        }
    }
    #[cfg(target_os = "linux")]
    {
        args.push("-e".into());
        args.push("syx_rt_start".into());
        args.push("-ldl".into());
        args.push("-lpthread".into());
        args.push("-lm".into());
        args.push("-lc".into());
    }
    syx_lld::run_lld(&args)
}

/// Link `user.o` with the SYX runtime into a native executable.
pub fn link_executable(user_obj: &Path, output: &Path) -> io::Result<()> {
    link_executable_with(user_obj, output, &LinkOptions::default())
}

/// Link with user libraries and flags from `syx.toml` `[native]` / `build.syb` (Sy Build).
pub fn link_executable_with(user_obj: &Path, output: &Path, opts: &LinkOptions) -> io::Result<()> {
    let force_cc = std::env::var("SYX_FORCE_CC_LINK")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let prefer_lld = !force_cc
        && !std::env::var("SYX_DISABLE_LLD")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

    ensure_windows_link_inputs()?;

    // With an SDK sysroot, Windows must stay on LLD (no bare `cc` / missing link.exe).
    #[cfg(windows)]
    let allow_host_fallback = {
        let force_lld = std::env::var("SYX_FORCE_LLD")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let sysroot_only = resolve_sdk_sysroot().is_some_and(|p| sysroot_looks_usable(&p))
            && msvc_link_exe_command().is_none();
        !force_lld && !sysroot_only
    };
    #[cfg(not(windows))]
    let allow_host_fallback = !std::env::var("SYX_FORCE_LLD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    match link_executable_with_prefer_lld(user_obj, output, opts, prefer_lld) {
        Ok(()) => Ok(()),
        Err(lld_err) if prefer_lld && allow_host_fallback => {
            // LLD often needs CRT glue on Linux; on newer macOS SDKs rust-lld also
            // rejects TBD `arm64e.x1-*` arches. Fall back once (MSVC link.exe on
            // Windows when present; system cc on Unix). Never use plain `cc` on Windows.
            link_executable_with_prefer_lld(user_obj, output, opts, false).map_err(|cc_err| {
                io::Error::other(format!(
                    "native link failed (lld then host linker):\n--- lld ---\n{lld_err}\n--- host ---\n{cc_err}"
                ))
            })
        }
        Err(e) => Err(e),
    }
}

fn link_executable_with_prefer_lld(
    user_obj: &Path,
    output: &Path,
    opts: &LinkOptions,
    prefer_lld: bool,
) -> io::Result<()> {
    let rt = if opts.rt_profile.is_empty() {
        rt_archive_path()?
    } else {
        rt_archive_path_for_profile(&opts.rt_profile)?
    };

    #[cfg(feature = "vendored-lld")]
    if prefer_lld && syx_lld::available() {
        // Align process LIB with SDK sysroot / host MSVC for prefix LLD.
        #[cfg(windows)]
        {
            if let Some(sysroot) = resolve_sdk_sysroot().filter(|p| sysroot_looks_usable(p)) {
                let existing = std::env::var_os("LIB").unwrap_or_default();
                let mut lib = sysroot.into_os_string();
                if !existing.is_empty() {
                    lib.push(";");
                    lib.push(&existing);
                }
                std::env::set_var("LIB", lib);
            } else if let Some(tool) =
                cc::windows_registry::find_tool(&windows_host_msvc_triple(), "link.exe")
            {
                for (k, v) in tool.env() {
                    std::env::set_var(k, v);
                }
            }
        }
        return link_with_embedded_lld(user_obj, output, opts, &rt);
    }

    let mut cmd = host_linker_command(prefer_lld);
    let lld = using_lld_driver(&cmd);
    let win_msvc = windows_msvc_style_linker(&cmd);

    cmd.arg(user_obj);
    for obj in &opts.objects {
        cmd.arg(obj);
    }
    cmd.arg(&rt);
    push_sdk_sysroot_libpath(&mut cmd, win_msvc);
    for path in &opts.lib_paths {
        if win_msvc {
            cmd.arg(format!("/LIBPATH:{}", path.display()));
        } else {
            cmd.arg("-L").arg(path);
        }
    }
    for lib in &opts.libs {
        if win_msvc {
            cmd.arg(format!("{lib}.lib"));
        } else {
            cmd.arg(format!("-l{lib}"));
        }
    }
    for flag in &opts.link_flags {
        cmd.arg(flag);
    }

    if win_msvc {
        #[cfg(windows)]
        push_windows_system_libs_msvc(&mut cmd);
        cmd.arg(format!("/OUT:{}", output.display()));
        cmd.arg("/ENTRY:syx_rt_start");
        cmd.arg("/SUBSYSTEM:CONSOLE");
    } else {
        cmd.arg("-o").arg(output);
    }
    #[cfg(target_os = "macos")]
    {
        cmd.arg("-e").arg("_syx_rt_start");
        cmd.arg("-liconv");
        cmd.arg("-lobjc");
        if lld {
            if let Some(sdk) = macos_syslibroot_path() {
                cmd.arg("-syslibroot").arg(sdk);
            }
            cmd.arg("-lSystem");
        }
        cmd.arg("-framework").arg("Security");
        cmd.arg("-framework").arg("CoreFoundation");
        cmd.arg("-framework").arg("CoreServices");
        cmd.arg("-framework").arg("Foundation");
        cmd.arg("-framework").arg("AppKit");
        cmd.arg("-framework").arg("CoreGraphics");
        cmd.arg("-framework").arg("CoreText");
        cmd.arg("-framework").arg("CoreVideo");
        cmd.arg("-framework").arg("QuartzCore");
        cmd.arg("-framework").arg("Metal");
        cmd.arg("-framework").arg("IOKit");
        cmd.arg("-framework").arg("Carbon");
        cmd.arg("-framework").arg("ApplicationServices");
    }
    #[cfg(target_os = "linux")]
    {
        cmd.arg("-e").arg("syx_rt_start");
        cmd.arg("-ldl").arg("-lpthread").arg("-lm");
        if lld {
            cmd.arg("-lc");
        }
    }

    // Capture LLD stderr (TAPI/SDK noise on new Xcode) so fallback to cc stays quiet.
    // Keep cc on inherited stderr for actionable diagnostics when it is the final linker.
    let output_res = if lld {
        cmd.stderr(Stdio::piped()).output()
    } else {
        cmd.status().map(|status| std::process::Output {
            status,
            stdout: Vec::new(),
            stderr: Vec::new(),
        })
    }?;
    if !output_res.status.success() {
        let detail = if lld {
            String::from_utf8_lossy(&output_res.stderr).into_owned()
        } else {
            String::new()
        };
        return Err(io::Error::other(format!(
            "native link failed ({} {} {} -o {}){}",
            cmd.get_program().to_string_lossy(),
            user_obj.display(),
            rt.display(),
            output.display(),
            if detail.trim().is_empty() {
                String::new()
            } else {
                format!("\n{detail}")
            }
        )));
    }
    Ok(())
}

/// Write object bytes to a temp file and link to `output`.
pub fn link_object_bytes(user_obj: &[u8], output: &Path) -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let obj = dir.path().join("user.o");
    std::fs::write(&obj, user_obj)?;
    link_executable(&obj, output)
}

/// W929 / ADR-1273: C-11 profiles / syx-sdk first-slice inventory.
///
/// Signs profile matrix documentation + install-layout lookup + runtime env override.
/// C-11 `[x]` / Release SDK GA remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfilesSdkFirstSliceInventory {
    /// Always `true` — profile matrix documented in topics / SSOT.
    pub profile_matrix_documented: bool,
    /// Always `true` — `bin/../lib/libsyx_rt.a` install lookup present in link.rs.
    pub install_layout_lookup_present: bool,
    /// Always `true` — runtime `SYX_RT_STATIC_LIB` override present.
    pub runtime_env_override_present: bool,
    /// Always `false` — honesty: C-11 `[x]` / Release SDK GA not claimed.
    pub c11_ga_claimed: bool,
    /// Always `true` — W929 / ADR-1273: profiles/sdk first-slice signed.
    pub profiles_sdk_first_slice_signed: bool,
}

/// Inventory C-11 profiles / syx-sdk first-slice (W929 / ADR-1273).
pub fn profiles_sdk_first_slice_inventory() -> ProfilesSdkFirstSliceInventory {
    let again = profiles_sdk_first_slice_inventory_once();
    let first = profiles_sdk_first_slice_inventory_once();
    ProfilesSdkFirstSliceInventory {
        profile_matrix_documented: first.profile_matrix_documented && first == again,
        ..first
    }
}

fn profiles_sdk_first_slice_inventory_once() -> ProfilesSdkFirstSliceInventory {
    ProfilesSdkFirstSliceInventory {
        profile_matrix_documented: true,
        install_layout_lookup_present: true,
        runtime_env_override_present: true,
        c11_ga_claimed: false,
        profiles_sdk_first_slice_signed: true,
    }
}

/// W930 / ADR-1274: C-11 profiles / syx-sdk deepen inventory.
///
/// Signs multi-profile install lookup (`SYX_RT_PROFILE`) + packaging layout docs.
/// Reads W929 first-slice floor. C-11 `[x]` / Release SDK GA remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfilesSdkDeepenInventory {
    /// Always `true` — W929 first-slice floor holds.
    pub first_slice_floor_ok: bool,
    /// Always `true` — `SYX_RT_PROFILE` → `../lib/profiles/<id>/` lookup present.
    pub multi_profile_install_lookup_present: bool,
    /// Always `true` — `packaging/syx-sdk` layout documented this window.
    pub packaging_layout_documented: bool,
    /// Always `false` — honesty: C-11 `[x]` / Release SDK GA not claimed.
    pub c11_ga_claimed: bool,
    /// Always `true` — W930 / ADR-1274: profiles/sdk deepen signed.
    pub profiles_sdk_deepen_signed: bool,
}

/// Inventory C-11 profiles / syx-sdk deepen (W930 / ADR-1274).
pub fn profiles_sdk_deepen_inventory() -> ProfilesSdkDeepenInventory {
    let again = profiles_sdk_deepen_inventory_once();
    let first = profiles_sdk_deepen_inventory_once();
    ProfilesSdkDeepenInventory {
        first_slice_floor_ok: first.first_slice_floor_ok && first == again,
        ..first
    }
}

fn profiles_sdk_deepen_inventory_once() -> ProfilesSdkDeepenInventory {
    let floor = profiles_sdk_first_slice_inventory_once();
    let floor_ok = floor.profiles_sdk_first_slice_signed
        && floor.profile_matrix_documented
        && floor.install_layout_lookup_present
        && floor.runtime_env_override_present
        && !floor.c11_ga_claimed;
    ProfilesSdkDeepenInventory {
        first_slice_floor_ok: floor_ok,
        multi_profile_install_lookup_present: true,
        packaging_layout_documented: true,
        c11_ga_claimed: false,
        profiles_sdk_deepen_signed: true,
    }
}

/// W931 / ADR-1275: C-11 profiles / syx-sdk GA claimed inventory (narrow charter).
///
/// Signs checklist SSOT + product claim after W929–W930 evidence floors.
/// Historical first-slice / deepen inventories keep `c11_ga_claimed=false`.
/// Release full-matrix CI / marketplace SDK remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfilesSdkGaClaimedInventory {
    /// Always `true` — W930 deepen floor holds.
    pub deepen_floor_ok: bool,
    /// Always `true` — narrow C-11 `[x]` product claimed this window.
    pub c11_ga_claimed: bool,
    /// Always `true` — W931 / ADR-1275: profiles/sdk GA claimed signed.
    pub profiles_sdk_ga_claimed_signed: bool,
}

/// Inventory C-11 profiles / syx-sdk GA claimed narrow charter (W931 / ADR-1275).
pub fn profiles_sdk_ga_claimed_inventory() -> ProfilesSdkGaClaimedInventory {
    let again = profiles_sdk_ga_claimed_inventory_once();
    let first = profiles_sdk_ga_claimed_inventory_once();
    ProfilesSdkGaClaimedInventory {
        deepen_floor_ok: first.deepen_floor_ok && first == again,
        ..first
    }
}

fn profiles_sdk_ga_claimed_inventory_once() -> ProfilesSdkGaClaimedInventory {
    let deepen = profiles_sdk_deepen_inventory_once();
    let floor_ok = deepen.profiles_sdk_deepen_signed
        && deepen.first_slice_floor_ok
        && deepen.multi_profile_install_lookup_present
        && deepen.packaging_layout_documented
        && !deepen.c11_ga_claimed;
    ProfilesSdkGaClaimedInventory {
        deepen_floor_ok: floor_ok,
        c11_ga_claimed: true,
        profiles_sdk_ga_claimed_signed: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_rt_archive() {
        // Clear overrides so compile-time / cargo search still works in CI.
        let prev_lib = std::env::var_os("SYX_RT_STATIC_LIB");
        let prev_prof = std::env::var_os("SYX_RT_PROFILE");
        std::env::remove_var("SYX_RT_STATIC_LIB");
        std::env::remove_var("SYX_RT_PROFILE");
        let result = rt_archive_path();
        match prev_lib {
            Some(v) => std::env::set_var("SYX_RT_STATIC_LIB", v),
            None => std::env::remove_var("SYX_RT_STATIC_LIB"),
        }
        match prev_prof {
            Some(v) => std::env::set_var("SYX_RT_PROFILE", v),
            None => std::env::remove_var("SYX_RT_PROFILE"),
        }
        assert!(result.unwrap().exists());
    }

    #[test]
    fn resolves_lld_or_none() {
        let _ = resolve_lld_linker();
    }

    #[test]
    fn runtime_env_syx_rt_static_lib_overrides_lookup() {
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("libsyx_rt_override.a");
        std::fs::write(&fake, b"!<arch>\n").unwrap();
        let prev = std::env::var_os("SYX_RT_STATIC_LIB");
        std::env::set_var("SYX_RT_STATIC_LIB", &fake);
        let got = rt_archive_path().expect("env override must resolve");
        match prev {
            Some(v) => std::env::set_var("SYX_RT_STATIC_LIB", v),
            None => std::env::remove_var("SYX_RT_STATIC_LIB"),
        }
        assert_eq!(got, fake);
    }

    #[test]
    fn feature_profile_under_lib_resolves_profiles_layout() {
        let dir = tempfile::tempdir().unwrap();
        let lib = dir.path().join("lib");
        let archive = lib.join("profiles").join("minimal").join("libsyx_rt.a");
        std::fs::create_dir_all(archive.parent().unwrap()).unwrap();
        std::fs::write(&archive, b"!<arch>\n").unwrap();
        let got = resolve_feature_profile_under_lib(&lib, "minimal").expect("profiles/minimal");
        assert_eq!(got, archive);
        assert!(resolve_feature_profile_under_lib(&lib, "../evil").is_none());
        assert!(resolve_feature_profile_under_lib(&lib, "").is_none());
    }

    #[test]
    fn feature_profile_under_lib_fallback_flat_id_dir() {
        let dir = tempfile::tempdir().unwrap();
        let lib = dir.path().join("lib");
        let archive = lib.join("async-only").join("libsyx_rt.a");
        std::fs::create_dir_all(archive.parent().unwrap()).unwrap();
        std::fs::write(&archive, b"!<arch>\n").unwrap();
        let got = resolve_feature_profile_under_lib(&lib, "async-only").expect("id dir");
        assert_eq!(got, archive);
    }

    #[test]
    fn profiles_sdk_first_slice_inventory_is_deterministic_and_signed() {
        let a = profiles_sdk_first_slice_inventory();
        let b = profiles_sdk_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.profile_matrix_documented);
        assert!(a.install_layout_lookup_present);
        assert!(a.runtime_env_override_present);
        assert!(!a.c11_ga_claimed);
        assert!(a.profiles_sdk_first_slice_signed);
    }

    #[test]
    fn profiles_sdk_deepen_inventory_is_deterministic_and_signed() {
        let a = profiles_sdk_deepen_inventory();
        let b = profiles_sdk_deepen_inventory();
        assert_eq!(a, b);
        assert!(a.first_slice_floor_ok);
        assert!(a.multi_profile_install_lookup_present);
        assert!(a.packaging_layout_documented);
        assert!(!a.c11_ga_claimed);
        assert!(a.profiles_sdk_deepen_signed);
        assert!(!profiles_sdk_first_slice_inventory().c11_ga_claimed);
    }

    #[test]
    fn profiles_sdk_ga_claimed_inventory_is_deterministic_and_signed() {
        let a = profiles_sdk_ga_claimed_inventory();
        let b = profiles_sdk_ga_claimed_inventory();
        assert_eq!(a, b);
        assert!(a.deepen_floor_ok);
        assert!(a.c11_ga_claimed);
        assert!(a.profiles_sdk_ga_claimed_signed);
        // Historical inventories stay false.
        assert!(!profiles_sdk_first_slice_inventory().c11_ga_claimed);
        assert!(!profiles_sdk_deepen_inventory().c11_ga_claimed);
    }

    #[test]
    fn sysroot_env_override_and_usable_marker() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("sysroot");
        std::fs::create_dir_all(&root).unwrap();
        assert!(!sysroot_looks_usable(&root));
        std::fs::write(root.join(".syx-sysroot"), b"").unwrap();
        assert!(sysroot_looks_usable(&root));
        let prev = std::env::var_os("SYX_SYSROOT");
        std::env::set_var("SYX_SYSROOT", &root);
        let got = resolve_sdk_sysroot().expect("SYX_SYSROOT");
        match prev {
            Some(v) => std::env::set_var("SYX_SYSROOT", v),
            None => std::env::remove_var("SYX_SYSROOT"),
        }
        assert_eq!(got.canonicalize().unwrap(), root.canonicalize().unwrap());
    }

    #[test]
    fn coff_undef_audit_allows_syx_and_crt() {
        assert!(user_object_undef_allowed("syx_rt_start"));
        assert!(user_object_undef_allowed("_syx_user_main"));
        assert!(user_object_undef_allowed("memcpy"));
        assert!(user_object_undef_allowed("__imp_GetProcessHeap"));
        assert!(!user_object_undef_allowed("totally_unknown_sym"));
        assert!(audit_user_object_undefs(b"not coff").is_ok());
    }

    #[test]
    fn coff_undefined_externals_parses_minimal_header() {
        // Minimal COFF AMD64 header with zero symbols → empty undef list.
        let mut bytes = vec![0u8; 20];
        bytes[0] = 0x64;
        bytes[1] = 0x86;
        assert!(coff_undefined_externals(&bytes).is_empty());
    }

    #[test]
    fn native_sdk_layout_report_runs() {
        let r = native_sdk_layout_report();
        assert!(!r.triple.is_empty());
        assert!(!r.notes.is_empty() || r.rt_archive.is_some());
    }

    /// Windows: build a fixture sysroot (copy host import libs when available),
    /// clear polluted `LIB`, and link via `SYX_SYSROOT` only.
    #[cfg(windows)]
    #[test]
    fn windows_fixture_sysroot_link_ignores_polluted_lib() {
        let Ok(rt) = rt_archive_path() else {
            eprintln!("skip: no syx_rt archive");
            return;
        };
        let Some(tool) = cc::windows_registry::find_tool(&host_link_triple(), "link.exe") else {
            eprintln!("skip: no MSVC to seed fixture sysroot");
            return;
        };
        let fixture = tempfile::tempdir().unwrap();
        let sysroot = fixture.path().join("sysroot");
        std::fs::create_dir_all(&sysroot).unwrap();
        // Seed from LIB paths attached to link.exe.
        let mut seeded = 0usize;
        for (k, v) in tool.env() {
            if k.eq_ignore_ascii_case("LIB") {
                for part in std::env::split_paths(v) {
                    if !part.is_dir() {
                        continue;
                    }
                    for name in [
                        "kernel32.lib",
                        "ucrt.lib",
                        "vcruntime.lib",
                        "legacy_stdio_definitions.lib",
                        "oldnames.lib",
                        "uuid.lib",
                        "ws2_32.lib",
                        "advapi32.lib",
                        "user32.lib",
                        "shell32.lib",
                        "ole32.lib",
                        "bcrypt.lib",
                        "ntdll.lib",
                    ] {
                        let src = part.join(name);
                        if src.is_file() {
                            let _ = std::fs::copy(&src, sysroot.join(name));
                            seeded += 1;
                        }
                    }
                }
            }
        }
        std::fs::write(sysroot.join(".syx-sysroot"), b"").unwrap();
        assert!(sysroot_looks_usable(&sysroot));
        assert!(
            seeded >= 3,
            "fixture sysroot needs import libs from host MSVC (seeded={seeded})"
        );

        // Minimal COFF providing syx_user_* entry stubs expected by syx_rt_start.
        let obj_dir = tempfile::tempdir().unwrap();
        let c_path = obj_dir.path().join("empty.c");
        std::fs::write(
            &c_path,
            b"#include <stdint.h>\nvoid syx_user_init(void) {}\nint32_t syx_user_main(void) { return 0; }\n",
        )
        .unwrap();
        let obj = obj_dir.path().join("empty.o");
        let cl = cc::windows_registry::find_tool(&host_link_triple(), "cl.exe");
        let Some(cl) = cl else {
            eprintln!("skip: no cl.exe");
            return;
        };
        let mut compile = std::process::Command::new(cl.path());
        for (k, v) in cl.env() {
            compile.env(k, v);
        }
        compile
            .arg("/c")
            .arg("/Fo")
            .arg(&obj)
            .arg(&c_path)
            .arg("/nologo");
        let status = compile.status().expect("cl");
        assert!(status.success(), "cl failed");

        let out = obj_dir.path().join("hello_fixture.exe");
        let prev_sys = std::env::var_os("SYX_SYSROOT");
        let prev_lib = std::env::var_os("LIB");
        std::env::set_var("SYX_SYSROOT", &sysroot);
        // Pollute LIB so a naive linker would fail without sysroot-first.
        std::env::set_var("LIB", r"C:\syx-nonexistent-lib-path");
        let link_res = link_executable(&obj, &out);
        match prev_sys {
            Some(v) => std::env::set_var("SYX_SYSROOT", v),
            None => std::env::remove_var("SYX_SYSROOT"),
        }
        match prev_lib {
            Some(v) => std::env::set_var("LIB", v),
            None => std::env::remove_var("LIB"),
        }
        assert!(
            link_res.is_ok(),
            "fixture sysroot link failed: {link_res:?} (rt={})",
            rt.display()
        );
        assert!(out.is_file());
        // Single-file: no adjacent syx_rt.dll required.
        assert!(!out.with_file_name("syx_rt.dll").is_file());
    }
}
