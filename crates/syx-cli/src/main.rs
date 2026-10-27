use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{self, Command};

use clap::{Parser, Subcommand};
use miette::IntoDiagnostic;
use syx_config::{self, NativeBackendChoice, SyxRuntimeConfig};
use syx_driver::{
    add_dependency, build_native_executable, build_native_executable_file,
    build_native_file_with_options, build_native_with_options,
    build_wasm_file_with_options, build_wasm_with_options, check_project_with_options,
    compile_file_with_options, compile_project_with_options, debug_dap, debug_interactive,
    default_emit_output, default_emit_stem, default_native_executable,
    default_native_executable_for_source, default_native_object, default_native_object_for_source,
    doc_project, emit_with_options, write_emit_all,
    aot_matrix::{print_doctor_summary, run_aot_matrix},
    gc_report::{print_gc_report, run_gc_report},
    inkwell_report::{print_inkwell_report, run_inkwell_report},
    cache_remote_report::{print_cache_remote_report, run_cache_remote_report},
    rt_exec_report::{print_rt_exec_report, run_rt_exec_report},
    heap_host_report::{print_heap_host_report, run_heap_host_report},
    farm_report::{print_farm_report, run_farm_report},
    reflect_report::{print_reflect_report, run_reflect_report},
    rt_runtime_report::{print_rt_runtime_report, run_rt_runtime_report},
    gc_relocate_report::{print_gc_relocate_report, run_gc_relocate_report},
    gc_generational_report::{print_gc_generational_report, run_gc_generational_report},
    native_profile_report::{print_native_profile_report, run_native_profile_report},
    gui_backend_report::{print_gui_backend_report, run_gui_backend_report},
    capabilities_report::{
        print_capabilities_report, print_ir_capabilities_report, run_capabilities_report,
        run_ir_capabilities_report,
    },
    farm_billing_report::{print_farm_billing_report, run_farm_billing_report},
    dist_actor_cluster_report::{print_dist_actor_cluster_report, run_dist_actor_cluster_report},
    dist_cdn_route_report::{print_dist_cdn_route_report, run_dist_cdn_route_report},
    heap_bridge_report::{print_heap_bridge_report, run_heap_bridge_report},
    heapslot_parity_report::{print_heapslot_parity_report, run_heapslot_parity_report},
    heap_bridge_prod_report::{print_heap_bridge_prod_report, run_heap_bridge_prod_report},
    rt_v7_report::{print_rt_v7_report, run_rt_v7_report},
    dist_failover_report::{print_dist_failover_report, run_dist_failover_report},
    hsm_rotation_report::{print_hsm_rotation_report, run_hsm_rotation_report},
    dict_runtime_report::{print_dict_runtime_report, run_dict_runtime_report},
    actor_ipc_report::{print_actor_ipc_report, run_actor_ipc_report},
    native_exec_op_report::{print_native_exec_op_report, run_native_exec_op_report},
    EmitFormat,
    format_project, format_check_path, install_project, lint_project, parse_module_dirs, publish_package_with_token,
    repl_eval,
    run_file_with_options, run_project_with_options, run_wasm_file,
    run_wasm_project_with_options, run_wasm_source_with_options, update_dependencies,
    BuildProfile, DependencyAddSpec, DriverOptions, NativeBackend, project_root_for,
    resolve_native_backend,
};

#[derive(Parser)]
#[command(name = "syx", about = "SYX compiler and toolchain", version)]
struct Cli {
    /// Extra module search directories, separated by `;`
    #[arg(long = "module-dir", global = true, value_name = "PATHS")]
    module_dir: Option<String>,

    /// Enable embedded standard library modules (math, io, …); use `--enable-std=false` to disable
    #[arg(long = "enable-std", global = true, default_value_t = true, action = clap::ArgAction::Set)]
    enable_std: bool,

    /// Native codegen backend for `build --native` (`llvm` only; Cranelift removed in SYX 36.0)
    #[arg(long = "backend", global = true, value_name = "BACKEND")]
    backend: Option<String>,

    /// Enable Cranelift JIT tier-2 (opt-in; default off)
    #[arg(long = "jit", global = true, action = clap::ArgAction::SetTrue)]
    jit: bool,

    /// Disable Cranelift JIT tier-2 (explicit off; default is already off)
    #[arg(long = "no-jit", global = true, action = clap::ArgAction::SetTrue, conflicts_with = "jit")]
    no_jit: bool,

    /// Enable JIT fastpath hot-loop compilation
    #[arg(long = "jit-fastpath", global = true, action = clap::ArgAction::SetTrue)]
    jit_fastpath: bool,

    /// Enable LLVM AOT native codegen path
    #[arg(long = "llvm-aot", global = true, action = clap::ArgAction::SetTrue)]
    llvm_aot: bool,

    /// WASM hybrid AOT emit (use with `--wasm`)
    #[arg(long = "aot", global = true, action = clap::ArgAction::SetTrue)]
    wasm_aot: bool,

    /// WASM 100% mir_exec fallback (debug; use with `--wasm`)
    #[arg(long = "mir-exec-only", global = true, action = clap::ArgAction::SetTrue)]
    wasm_mir_exec_only: bool,

    /// Enable embed sandbox isolation
    #[arg(long = "embed-sandbox", global = true, action = clap::ArgAction::SetTrue)]
    embed_sandbox: bool,

    /// Opt-in permission sandbox; un-granted domains fail closed
    #[arg(long = "permission-sandbox", global = true, action = clap::ArgAction::SetTrue)]
    permission_sandbox: bool,

    /// Grant `net` domain under permission sandbox (TCP/HTTP HostFns)
    #[arg(long = "allow-net", global = true, action = clap::ArgAction::SetTrue)]
    allow_net: bool,

    /// Grant `fs` domain under permission sandbox (File Read/Write HostFns)
    #[arg(long = "allow-fs", global = true, action = clap::ArgAction::SetTrue)]
    allow_fs: bool,

    /// Grant `env` domain under permission sandbox (EnvGet / EnvSet)
    #[arg(long = "allow-env", global = true, action = clap::ArgAction::SetTrue)]
    allow_env: bool,

    /// Grant `process` domain under permission sandbox (ProcessExec / ProcessSpawn)
    #[arg(long = "allow-process", global = true, action = clap::ArgAction::SetTrue)]
    allow_process: bool,

    /// Grant `plugin` domain under permission sandbox (`__syx_plugin_*` Hosts)
    #[arg(long = "allow-plugin", global = true, action = clap::ArgAction::SetTrue)]
    allow_plugin: bool,

    /// Opt-in HostFn capability-table gate; unsupported calls report S026
    #[arg(long = "ir-capability-gate", global = true, action = clap::ArgAction::SetTrue)]
    ir_capability_gate: bool,

    /// Embed execution timeout in milliseconds
    #[arg(long = "embed-timeout-ms", global = true, value_name = "MS")]
    embed_timeout_ms: Option<u64>,

    /// Embed memory limit in megabytes
    #[arg(long = "embed-memory-mb", global = true, value_name = "MB")]
    embed_memory_mb: Option<u64>,

    /// Disable digest/HMAC Host functions (`import Crypto`)
    #[arg(long = "no-crypto", global = true, action = clap::ArgAction::SetTrue)]
    no_crypto: bool,

    /// Disable SIMD types and operations
    #[arg(long = "no-simd", global = true, action = clap::ArgAction::SetTrue)]
    no_simd: bool,

    /// Disable property wrapper parser/desugar
    #[arg(long = "no-property-wrapper", global = true, action = clap::ArgAction::SetTrue)]
    no_property_wrapper: bool,

    /// Disable user `#macro` param gensym hygiene
    #[arg(long = "no-macro-hygiene", global = true, action = clap::ArgAction::SetTrue)]
    no_macro_hygiene: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a SYX project or source file
    Build {
        path: Option<PathBuf>,
        #[arg(long)]
        release: bool,
        #[arg(long)]
        debug: bool,
        #[arg(long)]
        native: bool,
        #[arg(long)]
        minimal: bool,
        #[arg(long)]
        no_http: bool,
        #[arg(long)]
        no_file: bool,
        #[arg(long)]
        object: bool,
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
        #[arg(long)]
        wasm: bool,
    },
    /// Compile and run a SYX project or source file
    Run {
        path: Option<PathBuf>,
        #[arg(long)]
        wasm: bool,
        #[arg(long, default_value = "main")]
        invoke: String,
        /// Show real OS windows (`SYX_GUI_VISIBLE=1`)
        #[arg(long = "gui-visible")]
        gui_visible: bool,
    },
    /// Type-check without emitting artifacts
    Check { path: Option<PathBuf> },
    /// Run project tests
    Test {
        path: Option<PathBuf>,
        /// Run only tests whose filename contains this substring
        #[arg(long)]
        filter: Option<String>,
        /// Report format: human (default), json, tap, or junit
        #[arg(long, default_value = "human", value_parser = ["human", "json", "tap", "junit"])]
        format: String,
    },
    /// Run allowlisted Criterion / Instant benchmarks
    #[command(hide = true)]
    Bench {
        /// Criterion / Instant bench name (allowlisted)
        name: Option<String>,
        /// Print allowlisted bench names and exit (does not invoke cargo)
        #[arg(long)]
        list: bool,
    },
    /// Collect LLVM code coverage for native tests
    #[command(hide = true)]
    Cov,
    /// Run the syx_smoke fuzz target
    #[command(hide = true)]
    Fuzz,
    /// Run property-based smoke tests
    #[command(hide = true)]
    Prop,
    /// Install project packages into the local toolchain
    Install { path: Option<PathBuf> },
    /// Format `.sy` source files
    Format {
        path: Option<PathBuf>,
        /// Check formatting only; exit non-zero if files would change
        #[arg(long)]
        check: bool,
    },
    /// Lint `.sy` source files
    Lint { path: Option<PathBuf> },
    /// Start an interactive or DAP debugger session
    Debug {
        path: Option<PathBuf>,
        /// DAP JSON-lines protocol on stdin/stdout (for IDE adapters)
        #[arg(long)]
        dap: bool,
    },
    /// Start an interactive REPL
    Repl,
    /// Add a dependency to the project
    Add {
        name: String,
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        git: Option<String>,
        #[arg(long, default_value = "1.0")]
        version: String,
        #[arg(long)]
        rev: Option<String>,
        #[arg(long)]
        registry: Option<String>,
        #[arg(long)]
        project: Option<PathBuf>,
    },
    /// Update project dependencies
    Update { path: Option<PathBuf> },
    /// Publish a package to a registry
    Publish {
        path: Option<PathBuf>,
        #[arg(long)]
        registry: Option<String>,
        #[arg(long)]
        token: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
    /// Initialize a new SYX project from a template
    Init {
        path: PathBuf,
        #[arg(long, default_value = "native")]
        template: String,
    },
    /// Generate API documentation from a `.sy` file or directory of sources
    Doc { path: Option<PathBuf> },
    /// Emit bytecode / IR (SYXBC, MIR, WASM, WAT, LLVM, CLIF)
    Emit {
        path: Option<PathBuf>,
        /// Output format: syxbc, mir, wasm, wat, llvm, clif, or all
        #[arg(long)]
        format: String,
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
    },
    /// Language server (LSP over stdin/stdout)
    Lsp {
        /// Accepted for vscode-languageclient compatibility; LSP always uses stdio.
        #[arg(long, hide = true)]
        stdio: bool,
    },
    /// Manage host plugins
    Plugin {
        #[command(subcommand)]
        command: PluginCommands,
    },
    /// Run toolchain health checks
    Doctor {
        /// Print WASM AOT strict/full matrix tier breakdown
        #[arg(long)]
        aot_matrix: bool,
        /// Print memory / GC / ARC layout report
        #[arg(long)]
        gc_report: bool,
        /// Print native LLVM exec_op ratio per fixture
        #[arg(long)]
        native_exec_op_report: bool,
        /// Print inkwell / LLVM IR backend report
        #[arg(long)]
        inkwell_report: bool,
        /// Print distributed cache remote push/fetch report
        #[arg(long)]
        cache_remote_report: bool,
        /// Print RT-internal exec fallback count
        #[arg(long)]
        rt_exec_report: bool,
        /// Print heap host materialize report
        #[arg(long)]
        heap_host_report: bool,
        /// Print build farm queue report
        #[arg(long)]
        farm_report: bool,
        /// Print generic specialization reflect report
        #[arg(long)]
        reflect_report: bool,
        /// Print RT runtime exec after native run
        #[arg(long)]
        rt_runtime_report: bool,
        /// Print GC relocate report
        #[arg(long)]
        gc_relocate_report: bool,
        /// Print dict runtime stdout digest
        #[arg(long)]
        dict_runtime_report: bool,
        /// Print actor IPC smoke report
        #[arg(long)]
        actor_ipc_report: bool,
        /// Print native debug/release profile report
        #[arg(long)]
        native_profile_report: bool,
        /// Print GUI backend report
        #[arg(long)]
        gui_backend_report: bool,
        /// Print capability matrix from packages/internal/capabilities.toml
        #[arg(long)]
        capabilities: bool,
        /// Print MirOp/HostFn IR capability summary
        #[arg(long)]
        ir_capabilities: bool,
        /// Print generational GC report
        #[arg(long)]
        gc_generational_report: bool,
        /// Alias for `--gc-generational-report`
        #[arg(long)]
        gc_gen_report: bool,
        /// Print farm billing meter report
        #[arg(long)]
        farm_billing_report: bool,
        /// Print dist actor multi-region cluster report
        #[arg(long)]
        dist_actor_cluster_report: bool,
        /// Print heap cross-process bridge report
        #[arg(long)]
        heap_bridge_report: bool,
        /// Print HeapSlot write-back parity report
        #[arg(long)]
        heapslot_parity_report: bool,
        /// Print heap production bridge report
        #[arg(long)]
        heap_bridge_prod_report: bool,
        /// Print RT native stack report
        #[arg(long)]
        rt_v7_report: bool,
        /// Print dist cross-continent failover report
        #[arg(long)]
        dist_failover_report: bool,
        /// Print HSM key-rotation cron report
        #[arg(long)]
        hsm_rotation_report: bool,
        /// Print dist actor global CDN route report
        #[arg(long)]
        dist_cdn_route_report: bool,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    /// Verify plugin manifest or `.syxplugin` bundle signature
    Verify {
        path: PathBuf,
    },
    /// Build plugin cdylib and generate SYX wrapper module
    Build {
        path: PathBuf,
        #[arg(long, default_value = "generated")]
        out: PathBuf,
    },
    /// Install plugin into `.syx/plugins/`
    Install {
        path: PathBuf,
        #[arg(long, default_value = ".")]
        project: PathBuf,
    },
}

fn main() {
    // Windows MSVC: keep `syx_user_*` stubs linked for `syx_rt_start` exports.
    syx_driver::ensure_host_rt_link_stubs();
    let args: Vec<String> = std::env::args().collect();
    if syx_macro::is_macro_sandbox_child_argv(&args) {
        if let Err(e) = syx_macro::macro_sandbox_child_main() {
            eprintln!("{e}");
            process::exit(1);
        }
        return;
    }
    if let Err(err) = run() {
        eprintln!("{err:?}");
        process::exit(1);
    }
}

fn run() -> miette::Result<()> {
    let cli = Cli::parse();
    let native_backend = resolve_native_backend(cli.backend.as_deref()).into_diagnostic()?;
    let backend_choice = NativeBackendChoice::Llvm;
    let jit = if cli.no_jit {
        Some(false)
    } else if cli.jit {
        Some(true)
    } else {
        None
    };
    syx_config::install_runtime_config(SyxRuntimeConfig::from_cli(
        jit,
        if cli.jit_fastpath { Some(true) } else { None },
        cli.enable_std,
        if cli.no_crypto {
            Some(false)
        } else {
            None
        },
        if cli.no_simd {
            Some(false)
        } else {
            None
        },
        if cli.no_property_wrapper {
            Some(false)
        } else {
            None
        },
        if cli.no_macro_hygiene {
            Some(false)
        } else {
            None
        },
        if cli.embed_sandbox { Some(true) } else { None },
        cli.embed_timeout_ms,
        cli.embed_memory_mb,
        if cli.llvm_aot { Some(true) } else { None },
        if cli.wasm_aot { Some(true) } else { None },
        if cli.wasm_mir_exec_only { Some(true) } else { None },
        Some(backend_choice),
        if cli.permission_sandbox { Some(true) } else { None },
        if cli.allow_net { Some(true) } else { None },
        if cli.allow_fs { Some(true) } else { None },
        if cli.allow_env { Some(true) } else { None },
        if cli.allow_process { Some(true) } else { None },
        if cli.allow_plugin { Some(true) } else { None },
        if cli.ir_capability_gate { Some(true) } else { None },
    ));
    let options = driver_options(
        cli.module_dir.as_deref(),
        cli.enable_std,
        native_backend,
        BuildProfile::Debug,
    );
    match cli.command {
        Commands::Build {
            path,
            release,
            debug,
            native,
            minimal,
            no_http,
            no_file,
            object,
            output,
            wasm,
        } => {
            let path = resolve_path(path);
            let profile = if debug && release {
                return Err(miette::miette!("syx build: --debug and --release are mutually exclusive"));
            } else if release {
                BuildProfile::Release
            } else {
                BuildProfile::Debug
            };
            let project_root = project_root_for(&path);
            let mut options = driver_options(
                cli.module_dir.as_deref(),
                cli.enable_std,
                native_backend,
                profile,
            );
            options.native_minimal = minimal;
            options.native_no_http = no_http;
            options.native_no_file = no_file;
            if native {
                let backend = options.native_backend.as_str();
                if path.is_dir() {
                    if object {
                        let out = output.unwrap_or_else(|| default_native_object(&project_root, profile));
                        let bytes = build_native_with_options(&path, &options).into_diagnostic()?;
                        if let Some(parent) = out.parent() {
                            std::fs::create_dir_all(parent).into_diagnostic()?;
                        }
                        std::fs::write(&out, bytes.object).into_diagnostic()?;
                        println!("Built native object {} ({backend})", out.display());
                    } else {
                        let out = output.unwrap_or_else(|| default_native_executable(&project_root, profile));
                        if let Some(parent) = out.parent() {
                            std::fs::create_dir_all(parent).into_diagnostic()?;
                        }
                        let exe = build_native_executable(&path, &options, &out).into_diagnostic()?;
                        println!("Built native executable {} ({backend})", exe.display());
                    }
                } else if path.extension().and_then(|e| e.to_str()) == Some("sy") {
                    if object {
                        let out = output.unwrap_or_else(|| {
                            default_native_object_for_source(&path, profile)
                        });
                        if let Some(parent) = out.parent() {
                            std::fs::create_dir_all(parent).into_diagnostic()?;
                        }
                        let bytes =
                            build_native_file_with_options(&path, &options).into_diagnostic()?;
                        std::fs::write(&out, bytes.object).into_diagnostic()?;
                        println!("Built native object {} ({backend})", out.display());
                    } else {
                        let out = output.unwrap_or_else(|| {
                            default_native_executable_for_source(&path, profile)
                        });
                        if let Some(parent) = out.parent() {
                            std::fs::create_dir_all(parent).into_diagnostic()?;
                        }
                        let exe =
                            build_native_executable_file(&path, &options, &out).into_diagnostic()?;
                        println!("Built native executable {} ({backend})", exe.display());
                    }
                } else {
                    return Err(miette::miette!(
                        "syx build --native requires a project directory or .sy file"
                    ));
                }
            } else if cli.backend.is_some() {
                return Err(miette::miette!(
                    "syx build --backend requires --native (--backend llvm only applies to native builds)"
                ));
            } else if wasm {
                let (bytes, out) = if path.is_dir() {
                    (
                        build_wasm_with_options(&path, &options).into_diagnostic()?,
                        path.join("main.wasm"),
                    )
                } else if path.extension().and_then(|e| e.to_str()) == Some("sy") {
                    (
                        build_wasm_file_with_options(&path, &options).into_diagnostic()?,
                        path.with_extension("wasm"),
                    )
                } else {
                    return Err(miette::miette!(
                        "syx build --wasm requires a project directory or .sy file"
                    ));
                };
                std::fs::write(&out, bytes).into_diagnostic()?;
                println!("Built wasm {}", out.display());
            } else if path.is_dir() {
                let manifest = syx_pkg::load_manifest(&path).into_diagnostic()?;
                if let Some(plan) =
                    syx_driver::build_syx::load_build_plan(&path, &manifest).into_diagnostic()?
                {
                    if plan.tasks.iter().any(|t| t.target.is_some()) {
                        let mut manifest = manifest;
                        let executed = syx_driver::build_syx::execute_build_plan(
                            &plan,
                            &mut options,
                            &mut manifest,
                            &path,
                        )
                        .into_diagnostic()?;
                        println!(
                            "Built {} (tasks: {})",
                            path.display(),
                            executed.join(" → ")
                        );
                    } else {
                        compile_project_with_options(&path, &options).into_diagnostic()?;
                        println!("Built {}", path.display());
                    }
                } else {
                    compile_project_with_options(&path, &options).into_diagnostic()?;
                    println!("Built {}", path.display());
                }
            } else {
                compile_file_with_options(&path, &options).into_diagnostic()?;
                println!("Built {}", path.display());
            }
        }
        Commands::Run { path, wasm, invoke, gui_visible } => {
            if gui_visible {
                std::env::set_var("SYX_GUI_VISIBLE", "1");
                // Phase 100: default HOLD unless explicitly disabled (CI sets SYX_GUI_HOLD=0).
                if std::env::var_os("SYX_GUI_HOLD").is_none() {
                    std::env::set_var("SYX_GUI_HOLD", "1");
                }
                // Phase 126: SynUI desktop run shares the same opt-in as --gui-visible.
                if std::env::var_os("SYX_SYNUI_RUN").is_none() {
                    std::env::set_var("SYX_SYNUI_RUN", "1");
                }
                syx_driver::require_gui_dylib_if_visible();
                let _ = syx_driver::ensure_gui_loaded();
            }
            let path = resolve_path(path);
            if wasm {
                let exit_code = if path.is_dir() {
                    run_wasm_project_with_options(&path, &invoke, &options).into_diagnostic()?
                } else if path.extension().and_then(|e| e.to_str()) == Some("wasm") {
                    run_wasm_file(&path, &invoke).into_diagnostic()?
                } else if path.extension().and_then(|e| e.to_str()) == Some("sy") {
                    run_wasm_source_with_options(&path, &invoke, &options).into_diagnostic()?
                } else {
                    return Err(miette::miette!(
                        "syx run --wasm requires a project directory, .sy file, or .wasm file"
                    ));
                };
                process::exit(exit_code as i32);
            } else if path.is_dir() {
                run_project_with_options(&path, &options).into_diagnostic()?;
            } else {
                run_file_with_options(&path, &options).into_diagnostic()?;
            }
        }
        Commands::Check { path } => {
            let path = resolve_path(path);
            if path.is_dir() {
                check_project_with_options(&path, &options).into_diagnostic()?;
            } else {
                compile_file_with_options(&path, &options).into_diagnostic()?;
            }
            println!("Check passed for {}", path.display());
        }
        Commands::Test {
            path,
            filter,
            format,
        } => {
            let path = resolve_path(path);
            let mut fmt = syx_driver::TestOutputFormat::parse(&format).unwrap_or_default();
            // Preserve Phase 84 env: SYX_TEST_TAP=1 with default human → TAP.
            if matches!(fmt, syx_driver::TestOutputFormat::Human) {
                let tap_env = std::env::var("SYX_TEST_TAP")
                    .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                if tap_env {
                    fmt = syx_driver::TestOutputFormat::Tap;
                }
            }
            let report = syx_driver::test_project_with_format(
                &path,
                &options,
                filter.as_deref(),
                fmt,
            )
            .into_diagnostic()?;
            match fmt {
                syx_driver::TestOutputFormat::Json => {
                    println!("{}", report.to_json_string().into_diagnostic()?);
                    if report.failed > 0 {
                        process::exit(1);
                    }
                    if report.passed == 0 && report.skipped > 0 {
                        return Err(miette::miette!("no tests matched filter"));
                    }
                }
                syx_driver::TestOutputFormat::Junit => {
                    print!("{}", report.to_junit_xml_string());
                    if report.failed > 0 {
                        process::exit(1);
                    }
                    if report.passed == 0 && report.skipped > 0 {
                        return Err(miette::miette!("no tests matched filter"));
                    }
                }
                syx_driver::TestOutputFormat::Tap => {
                    // TAP already printed by driver.
                    if report.passed == 0 && report.skipped > 0 {
                        return Err(miette::miette!("no tests matched filter"));
                    }
                }
                syx_driver::TestOutputFormat::Human => {
                    println!(
                        "ok {} passed{}",
                        report.passed,
                        if report.skipped > 0 {
                            format!(", {} skipped", report.skipped)
                        } else {
                            String::new()
                        }
                    );
                    if report.passed == 0 && report.skipped > 0 {
                        return Err(miette::miette!("no tests matched filter"));
                    }
                }
            }
        }
        Commands::Bench { name, list } => {
            const ALLOWLIST: &[&str] = &[
                "path_join_criterion",
                "http_response_criterion",
                "sql_exec_criterion",
                "tcp_connect_criterion",
                "compile_bench",
            ];
            if list {
                for b in ALLOWLIST {
                    println!("{b}");
                }
            } else {
                let Some(name) = name.as_deref() else {
                    return Err(miette::miette!(
                        "usage: syx bench <name> | syx bench --list\n\
                         allowlisted: {}",
                        ALLOWLIST.join(", ")
                    ));
                };
                if !ALLOWLIST.contains(&name) {
                    return Err(miette::miette!(
                        "unknown bench `{name}`; use `syx bench --list`\n\
                         allowlisted: {}",
                        ALLOWLIST.join(", ")
                    ));
                }
                let status = Command::new("cargo")
                    .args(["bench", "-p", "syx-driver", "--bench", name])
                    .status()
                    .into_diagnostic()?;
                if !status.success() {
                    process::exit(status.code().unwrap_or(1));
                }
            }
        }
        Commands::Cov => {
            let status = match Command::new("cargo")
                .args([
                    "llvm-cov",
                    "-p",
                    "syx-driver",
                    "--test",
                    "native",
                    "--no-report",
                ])
                .status()
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(miette::miette!(
                        "failed to run `cargo llvm-cov` ({e}); install with:\n\
                         cargo install cargo-llvm-cov"
                    ));
                }
            };
            if !status.success() {
                // Common case: llvm-cov subcommand missing → cargo exits non-zero.
                eprintln!(
                    "note: `syx cov` requires cargo-llvm-cov; install with:\n\
                     cargo install cargo-llvm-cov"
                );
                process::exit(status.code().unwrap_or(1));
            }
        }
        Commands::Fuzz => {
            let status = match Command::new("cargo")
                .args(["fuzz", "run", "syx_smoke"])
                .status()
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(miette::miette!(
                        "failed to run `cargo fuzz` ({e}); install with:\n\
                         cargo install cargo-fuzz\n\
                         note: fuzz/ scaffold met (W948); CI fuzz first-slice met (W950; -runs=1)"
                    ));
                }
            };
            if !status.success() {
                eprintln!(
                    "note: `syx fuzz` requires cargo-fuzz + fuzz/ scaffold with target `syx_smoke`;\n\
                     install: cargo install cargo-fuzz\n\
                     scaffold met (W948); CI fuzz first-slice met (W950; -runs=1); unbounded fuzz GA unmet"
                );
                process::exit(status.code().unwrap_or(1));
            }
        }
        Commands::Prop => {
            let status = match Command::new("cargo")
                .args(["test", "-p", "syx-driver", "--test", "property_smoke"])
                .status()
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(miette::miette!(
                        "failed to run `cargo test` ({e})\n\
                         note: `syx prop` → `cargo test -p syx-driver --test property_smoke`"
                    ));
                }
            };
            if !status.success() {
                eprintln!(
                    "note: `syx prop` delegates to `cargo test -p syx-driver --test property_smoke`;\n\
                     property_smoke harness met (W947); CI property first-slice met (W949); proptestest GA still unmet"
                );
                process::exit(status.code().unwrap_or(1));
            }
        }
        Commands::Install { path } => {
            let path = resolve_path(path);
            let report = install_project(&path).into_diagnostic()?;
            println!(
                "Installed {} package(s) for {}",
                report.package_count,
                path.display()
            );
        }
        Commands::Format { path, check } => {
            let path = resolve_path(path);
            if check {
                let report = format_check_path(&path).into_diagnostic()?;
                println!(
                    "format --check: {} file(s); would_change={}; failed={}",
                    report.checked, report.would_change, report.failed
                );
                for (file, msg) in &report.failures {
                    println!("  {}: {}", file.display(), msg);
                }
                if !report.ok() {
                    process::exit(1);
                }
            } else {
                format_project(&path).into_diagnostic()?;
                println!("Formatted {}", path.display());
            }
        }
        Commands::Lint { path } => {
            let path = resolve_path(path);
            let issues = lint_project(&path).into_diagnostic()?;
            for issue in issues {
                println!("[{}] {}", issue.code, issue.message);
            }
        }
        Commands::Debug { path, dap } => {
            let path = resolve_path(path);
            if dap {
                debug_dap(&path, &options).into_diagnostic()?;
            } else {
                debug_interactive(&path, &options).into_diagnostic()?;
            }
        }
        Commands::Repl => {
            repl_loop()?;
        }
        Commands::Add {
            name,
            path,
            git,
            version,
            rev,
            registry,
            project,
        } => {
            let root = resolve_path(project);
            let spec = if let Some(git_url) = git {
                DependencyAddSpec::git(git_url, version, rev)
            } else if let Some(ref path) = path {
                DependencyAddSpec {
                    version,
                    path: Some(path.clone()),
                    git: None,
                    rev: None,
                    registry: registry.clone(),
                }
            } else if let Some(reg) = registry {
                DependencyAddSpec {
                    version,
                    path: None,
                    git: None,
                    rev: None,
                    registry: Some(reg),
                }
            } else {
                return Err(miette::miette!(
                    "syx add requires --path, --git, or --registry for dependency source"
                ));
            };
            add_dependency(&root, &name, spec).into_diagnostic()?;
            if let Some(p) = &path {
                println!("Added dependency {name} -> {p}");
            } else {
                println!("Added dependency {name} from git");
            }
        }
        Commands::Update { path } => {
            let path = resolve_path(path);
            update_dependencies(&path).into_diagnostic()?;
            println!("Updated dependencies for {}", path.display());
        }
        Commands::Publish { path, registry, token, dry_run } => {
            let path = resolve_path(path);
            let registry = registry
                .or_else(|| std::env::var("SYX_REGISTRY").ok())
                .unwrap_or_else(|| "https://packages.sy.dev".into());
            if dry_run {
                let manifest = syx_pkg::load_manifest(&path).into_diagnostic()?;
                println!(
                    "dry-run publish {}@{} -> {}",
                    manifest.name, manifest.version, registry
                );
            } else {
                publish_package_with_token(
                    &path,
                    Some(registry.as_str()),
                    token.as_deref(),
                )
                .into_diagnostic()?;
                println!("Published {}", path.display());
            }
        }
        Commands::Init { path, template } => {
            std::fs::create_dir_all(&path).into_diagnostic()?;
            let (main, build) = match template.as_str() {
                "wasm" => (
                    r#"func main() {
    print("wasm")
}
"#,
                    r#"build {
    compileOptions {
        optLevel = release
    }
    features {
        wasm_aot = true
    }
}
"#,
                ),
                "plugin" => (
                    r#"func main() {
    print("plugin")
}
"#,
                    r#"build {
    compileOptions {
        optLevel = release
    }
}
"#,
                ),
                "lib" => (
                    r#"public func hello() {
    print("lib")
}

func main() {
    hello()
}
"#,
                    r#"build {
    compileOptions {
        optLevel = release
    }
}
"#,
                ),
                _ => (
                    r#"func main() {
    print("native")
}
"#,
                    r#"build {
    compileOptions {
        optLevel = release
    }
}
"#,
                ),
            };
            std::fs::write(path.join("main.sy"), main).into_diagnostic()?;
            std::fs::write(path.join("build.syb"), build).into_diagnostic()?;
            // `.syb` = Sy Build config (declarative DSL), not Sy source (`.sy`).
            std::fs::write(
                path.join("syx.toml"),
                r#"name = "demo"
version = "0.1.0"
main = "main.sy"
build = "build.syb"
"#,
            )
            .into_diagnostic()?;
            println!("Initialized {} template at {}", template, path.display());
        }
        Commands::Doc { path } => {
            let path = resolve_path(path);
            let markdown = doc_project(&path).into_diagnostic()?;
            let doc_root = if path.is_file() {
                path.parent()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("."))
            } else {
                path.clone()
            };
            let out = doc_root.join("docs").join("api.md");
            if let Some(parent) = out.parent() {
                std::fs::create_dir_all(parent).into_diagnostic()?;
            }
            std::fs::write(&out, markdown).into_diagnostic()?;
            println!("Generated {}", out.display());
        }
        Commands::Emit { path, format, output } => {
            let path = resolve_path(path);
            let format = match EmitFormat::parse(&format) {
                Some(f) => f,
                None => {
                    return Err(miette::miette!(
                        "unknown emit format '{format}' (expected syxbc, mir, wasm, wat, llvm, clif, or all)"
                    ));
                }
            };
            let artifacts = emit_with_options(&path, format, &options).into_diagnostic()?;
            let stem = default_emit_stem(&path);
            if format == EmitFormat::All {
                let dir = output.unwrap_or_else(|| {
                    if path.is_dir() {
                        path.join("emit")
                    } else {
                        path.parent()
                            .map(PathBuf::from)
                            .unwrap_or_else(|| PathBuf::from("."))
                            .join("emit")
                    }
                });
                write_emit_all(&dir, &stem, &artifacts).into_diagnostic()?;
                println!("Emitted all formats to {}", dir.display());
            } else {
                let out = output.unwrap_or_else(|| default_emit_output(&path, format));
                match format {
                    EmitFormat::Syxbc => {
                        std::fs::write(&out, artifacts.syxbc.as_deref().unwrap_or(""))
                            .into_diagnostic()?;
                    }
                    EmitFormat::Mir => {
                        std::fs::write(&out, artifacts.mir.as_deref().unwrap_or(""))
                            .into_diagnostic()?;
                    }
                    EmitFormat::Wasm => {
                        std::fs::write(&out, artifacts.wasm.as_deref().unwrap_or(&[]))
                            .into_diagnostic()?;
                    }
                    EmitFormat::Wat => {
                        std::fs::write(&out, artifacts.wat.as_deref().unwrap_or(""))
                            .into_diagnostic()?;
                    }
                    EmitFormat::Llvm => {
                        std::fs::write(&out, artifacts.llvm.as_deref().unwrap_or(""))
                            .into_diagnostic()?;
                    }
                    EmitFormat::Clif => {
                        std::fs::write(&out, artifacts.clif.as_deref().unwrap_or(""))
                            .into_diagnostic()?;
                    }
                    EmitFormat::All => unreachable!(),
                }
                println!("Emitted {} ({})", out.display(), format_name(format));
            }
        }
        Commands::Plugin { command } => match command {
            PluginCommands::Verify { path } => {
                use syx_host_plugin::{verify_signature, verify_syxplugin_bundle, PluginManifest, SyxPluginBundle};
                if path.extension().and_then(|e| e.to_str()) == Some("syxplugin") {
                    let bundle = SyxPluginBundle::from_path(&path).map_err(|e| {
                        miette::miette!("plugin verify failed: {e}")
                    })?;
                    verify_syxplugin_bundle(&bundle).map_err(|e| miette::miette!("{e}"))?;
                } else {
                    let manifest = PluginManifest::from_path(&path).map_err(|e| {
                        miette::miette!("plugin verify failed: {e}")
                    })?;
                    verify_signature(&manifest).map_err(|e| miette::miette!("{e}"))?;
                }
                println!("plugin verify ok: {}", path.display());
            }
            PluginCommands::Build { path, out } => {
                let manifest_path = if path.is_dir() {
                    path.join("plugin.json")
                } else {
                    path.clone()
                };
                let manifest = syx_host_plugin::PluginManifest::from_path(&manifest_path)
                    .map_err(|e| miette::miette!("plugin build failed: {e}"))?;
                let out_dir = if out.is_absolute() {
                    out
                } else {
                    manifest_path.parent().unwrap_or(&path).join(&out)
                };
                let written = syx_host_plugin::write_codegen_module(&manifest, &out_dir)
                    .map_err(|e| miette::miette!("plugin codegen failed: {e}"))?;
                println!("plugin build ok: {}", written.display());
            }
            PluginCommands::Install { path, project } => {
                let id = syx_host_plugin::install_plugin(&path, &project)
                    .map_err(|e| miette::miette!("plugin install failed: {e}"))?;
                println!("plugin install ok: {id} -> {}/.syx/plugins/{id}", project.display());
            }
        }
        Commands::Lsp { stdio: _ } => {
            syx_lsp::run_stdio().into_diagnostic()?;
        }
        Commands::Doctor {
            aot_matrix,
            gc_report,
            native_exec_op_report,
            inkwell_report,
            cache_remote_report,
            rt_exec_report,
            heap_host_report,
            farm_report,
            reflect_report,
            rt_runtime_report,
            gc_relocate_report,
            dict_runtime_report,
            actor_ipc_report,
            gc_generational_report,
            gc_gen_report,
            native_profile_report,
            gui_backend_report,
            capabilities,
            ir_capabilities,
            farm_billing_report,
            dist_actor_cluster_report,
            heap_bridge_report,
            heapslot_parity_report,
            heap_bridge_prod_report,
            rt_v7_report,
            dist_failover_report,
            hsm_rotation_report,
            dist_cdn_route_report,
        } => {
            if aot_matrix {
                let report = run_aot_matrix(true);
                print_doctor_summary(&report);
            } else if gc_report {
                let report = run_gc_report();
                print_gc_report(&report);
            } else if native_exec_op_report {
                let report = run_native_exec_op_report();
                print_native_exec_op_report(&report);
            } else if inkwell_report {
                let report = run_inkwell_report();
                print_inkwell_report(&report);
            } else if cache_remote_report {
                let report = run_cache_remote_report();
                print_cache_remote_report(&report);
            } else if rt_exec_report {
                let report = run_rt_exec_report("main_sync");
                print_rt_exec_report(&report);
            } else if heap_host_report {
                let report = run_heap_host_report();
                print_heap_host_report(&report);
            } else if farm_report {
                let report = run_farm_report();
                print_farm_report(&report);
            } else if reflect_report {
                let report = run_reflect_report();
                print_reflect_report(&report);
                if !report.ok {
                    process::exit(1);
                }
            } else if rt_runtime_report {
                let path = std::path::PathBuf::from("examples/main_sync.sy");
                let report = run_rt_runtime_report(&path);
                print_rt_runtime_report(&report);
            } else if gc_relocate_report {
                let report = run_gc_relocate_report();
                print_gc_relocate_report(&report);
            } else if dict_runtime_report {
                let path = std::path::PathBuf::from(
                    "crates/syx-driver/tests/fixtures/memory/heap_dict_slot_unified.sy",
                );
                let report = run_dict_runtime_report(&path);
                print_dict_runtime_report(&report);
            } else if actor_ipc_report {
                let report = run_actor_ipc_report();
                print_actor_ipc_report(&report);
            } else if gc_generational_report || gc_gen_report {
                let report = run_gc_generational_report();
                print_gc_generational_report(&report);
            } else if native_profile_report {
                let report = run_native_profile_report();
                print_native_profile_report(&report);
            } else if gui_backend_report {
                let report = run_gui_backend_report();
                print_gui_backend_report(&report);
            } else if capabilities {
                let report = run_capabilities_report();
                print_capabilities_report(&report);
            } else if ir_capabilities {
                let report = run_ir_capabilities_report();
                print_ir_capabilities_report(&report);
            } else if farm_billing_report {
                let report = run_farm_billing_report();
                print_farm_billing_report(&report);
            } else if dist_actor_cluster_report {
                let report = run_dist_actor_cluster_report();
                print_dist_actor_cluster_report(&report);
            } else if heap_bridge_report {
                let report = run_heap_bridge_report();
                print_heap_bridge_report(&report);
            } else if heapslot_parity_report {
                let report = run_heapslot_parity_report();
                print_heapslot_parity_report(&report);
            } else if heap_bridge_prod_report {
                let report = run_heap_bridge_prod_report();
                print_heap_bridge_prod_report(&report);
            } else if rt_v7_report {
                let report = run_rt_v7_report();
                print_rt_v7_report(&report);
            } else if dist_failover_report {
                let report = run_dist_failover_report();
                print_dist_failover_report(&report);
            } else if hsm_rotation_report {
                let report = run_hsm_rotation_report();
                print_hsm_rotation_report(&report);
            } else if dist_cdn_route_report {
                let report = run_dist_cdn_route_report();
                print_dist_cdn_route_report(&report);
            } else {
                println!("syx doctor: use --aot-matrix, --gc-report, --native-exec-op-report, --inkwell-report, --cache-remote-report, --rt-exec-report, --heap-host-report, --farm-report, --reflect-report, --rt-runtime-report, --gc-relocate-report, --dict-runtime-report, --actor-ipc-report, --gc-generational-report, --farm-billing-report, --dist-actor-cluster-report, --heap-bridge-report, --heapslot-parity-report, --heap-bridge-prod-report, --rt-v7-report, --dist-failover-report, --hsm-rotation-report, --dist-cdn-route-report, --gui-backend-report, --capabilities, or --ir-capabilities");
            }
        }
    }
    Ok(())
}

fn format_name(format: EmitFormat) -> &'static str {
    match format {
        EmitFormat::Syxbc => "syxbc",
        EmitFormat::Mir => "mir",
        EmitFormat::Wasm => "wasm",
        EmitFormat::Wat => "wat",
        EmitFormat::Llvm => "llvm",
        EmitFormat::Clif => "clif",
        EmitFormat::All => "all",
    }
}

fn repl_loop() -> miette::Result<()> {
    println!("SYX REPL (Ctrl-D to exit)");
    loop {
        print!("syx> ");
        io::stdout().flush().into_diagnostic()?;
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line).into_diagnostic()?;
        if n == 0 {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Err(err) = repl_eval(trimmed) {
            eprintln!("{err:?}");
        }
    }
    Ok(())
}

fn resolve_path(path: Option<PathBuf>) -> PathBuf {
    let path = path.unwrap_or_else(|| PathBuf::from("."));
    // Canonicalize so stdlib discovery (walk-up to monorepo `packages/`) and
    // module graph keys stay consistent. Relative `.` cannot walk up, which
    // made `import Log` resolve as Embedded during graph build but as an
    // on-disk `packages/log` path during semantic analysis → S002.
    path.canonicalize().unwrap_or(path)
}

fn driver_options(
    module_dir: Option<&str>,
    enable_std: bool,
    native_backend: NativeBackend,
    build_profile: BuildProfile,
) -> DriverOptions {
    DriverOptions {
        module_dirs: module_dir
            .map(parse_module_dirs)
            .unwrap_or_default(),
        enable_std,
        native_backend,
        build_profile,
        wasm_aot: (build_profile == BuildProfile::Release || syx_config::wasm_aot_enabled())
            && !syx_config::wasm_mir_exec_only(),
        ..DriverOptions::default()
    }
}
