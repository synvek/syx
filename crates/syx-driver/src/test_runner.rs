//! SYX driver test runner face (`syx test`).
//! Extracted in S2-I219 (zero semantics).

use crate::{
    collect_sy_files, compile_linked_with_options, driver_error_at, ensure_vm_plugin_dispatch,
    run_compiled_program_with_strict, run_file_with_options, run_project_with_options,
    DriverOptions,
};
use serde::Serialize;
use std::path::{Path, PathBuf};
use syx_ast::Decl;
use syx_diagnostics::{SourceFile, Span, SyxDiagnostic, SyxResult};
use syx_parser::Parser;

/// Per-case status for `syx test` reports (W932 / ADR-1276).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TestCaseStatus {
    Passed,
    Failed,
    Skipped,
}

/// One discovered / executed test case.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TestCaseResult {
    pub name: String,
    pub status: TestCaseStatus,
    /// Failure message for JSON / JUnit (`--format json|junit`); omitted when passed/skipped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<String>,
}

/// Aggregated test run report (Phase 84 + W932 cases).
#[derive(Debug, Clone, Default, Serialize)]
pub struct TestReport {
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub cases: Vec<TestCaseResult>,
}

impl TestReport {
    /// Serialize as JSON object for `--format json` (W932).
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize as JUnit XML for `--format junit` (W933 / ADR-1277).
    ///
    /// Single `<testsuite name="syx">` with `tests` / `failures` / `skipped` / `passed`
    /// attributes aligned to report counters; each case is a `<testcase>`.
    pub fn to_junit_xml_string(&self) -> String {
        fn xml_escape(s: &str) -> String {
            let mut out = String::with_capacity(s.len());
            for c in s.chars() {
                match c {
                    '&' => out.push_str("&amp;"),
                    '<' => out.push_str("&lt;"),
                    '>' => out.push_str("&gt;"),
                    '"' => out.push_str("&quot;"),
                    '\'' => out.push_str("&apos;"),
                    _ => out.push(c),
                }
            }
            out
        }
        let total = self.passed + self.failed + self.skipped;
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
            "<testsuite name=\"syx\" tests=\"{}\" failures=\"{}\" skipped=\"{}\" passed=\"{}\">\n",
            total, self.failed, self.skipped, self.passed
        ));
        for case in &self.cases {
            let name = xml_escape(&case.name);
            match case.status {
                TestCaseStatus::Passed => {
                    xml.push_str(&format!(
                        "  <testcase name=\"{name}\" classname=\"syx\"/>\n"
                    ));
                }
                TestCaseStatus::Failed => {
                    let msg = case.failure.as_deref().map(xml_escape).unwrap_or_default();
                    xml.push_str(&format!(
                        "  <testcase name=\"{name}\" classname=\"syx\"><failure>{msg}</failure></testcase>\n"
                    ));
                }
                TestCaseStatus::Skipped => {
                    xml.push_str(&format!(
                        "  <testcase name=\"{name}\" classname=\"syx\"><skipped/></testcase>\n"
                    ));
                }
            }
        }
        xml.push_str("</testsuite>\n");
        xml
    }
}

/// Output format for `syx test` (W932 JSON / W933 JUnit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TestOutputFormat {
    #[default]
    Human,
    Json,
    Tap,
    /// JUnit XML report (W933 / ADR-1277).
    Junit,
}

impl TestOutputFormat {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "human" => Some(Self::Human),
            "json" => Some(Self::Json),
            "tap" => Some(Self::Tap),
            "junit" => Some(Self::Junit),
            _ => None,
        }
    }
}

/// Run project tests (`tests/**/*.sy`) or the main entry when no test directory exists.
pub fn test_project(root: &Path, options: &DriverOptions) -> SyxResult<TestReport> {
    test_project_with_filter(root, options, None)
}

/// Run tests with optional name filter (Phase 84). TAP when `SYX_TEST_TAP=1` (human default).
pub fn test_project_with_filter(
    root: &Path,
    options: &DriverOptions,
    filter: Option<&str>,
) -> SyxResult<TestReport> {
    let tap_env = std::env::var("SYX_TEST_TAP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let format = if tap_env {
        TestOutputFormat::Tap
    } else {
        TestOutputFormat::Human
    };
    test_project_with_format(root, options, filter, format)
}

/// Run tests with explicit output format (W932 JSON / W933 JUnit).
///
/// - `Human`: fail-fast on first error (legacy).
/// - `Tap` / `Json` / `Junit`: continue collecting; `Tap` prints TAP lines; `Json`/`Junit`
///   return full report (caller prints). `Json`/`Junit` return `Ok` with `failed > 0` so CLI
///   can print then exit non-zero; `Tap` returns `Err` if any case failed after the run.
///
/// Discovery: top-level zero-arg `@Test` each become `relativePath.fnName`
/// (posix path under `tests/`, no `.sy`; recursive `tests/**/*.sy`).
/// JUnit-shaped hooks (PascalCase attrs, at most one each):
/// `@TestSetup` once → (`@TestBefore` → `@Test` → `@TestAfter`) × N → `@TestTeardown` once.
/// Accepts a project root (`tests/**/*.sy` or whole-project `main`) or a single `.sy` file.
/// Files with no `@Test` still run as a single whole-file entry (hooks ignored).
pub fn test_project_with_format(
    root: &Path,
    options: &DriverOptions,
    filter: Option<&str>,
    format: TestOutputFormat,
) -> SyxResult<TestReport> {
    let tap = matches!(format, TestOutputFormat::Tap);
    let structured = matches!(format, TestOutputFormat::Json | TestOutputFormat::Junit);
    let continue_on_fail = tap || structured;
    let mut report = TestReport::default();
    let tests_dir = root.join("tests");

    enum PlannedUnit {
        /// Whole-file entry (`main` / `@Main`); hooks ignored.
        File { name: String, path: PathBuf },
        /// One source file with `@Test` cases and optional hooks.
        AtTestFile {
            file_stem: String,
            path: PathBuf,
            /// `@TestSetup` — once before all cases.
            setup: Option<String>,
            /// `@TestBefore` — before each case.
            before: Option<String>,
            /// `@TestAfter` — after each case.
            after: Option<String>,
            /// `@TestTeardown` — once after all cases.
            teardown: Option<String>,
            /// `(case_name, fn_name)`
            tests: Vec<(String, String)>,
            /// Duplicate hooks etc.
            discover_error: Option<String>,
        },
    }

    let mut planned: Vec<PlannedUnit> = Vec::new();
    let mut tap_runnable = 0usize;
    let single_sy = root.is_file() && root.extension().and_then(|e| e.to_str()) == Some("sy");
    let use_tests_dir = !single_sy && tests_dir.is_dir();
    let use_project_main = !single_sy && !use_tests_dir;

    if root.is_file() && !single_sy {
        return Err(driver_error_at(
            format!(
                "syx test expects a project directory or a .sy file, got {}",
                root.display()
            ),
            None,
            Span::DUMMY,
            "D002",
        ));
    }

    let mut plan_one_sy =
        |p: PathBuf, case_base: String, report: &mut TestReport| -> SyxResult<()> {
            let disc = discover_file_test_plan(&p)?;
            if disc.tests.is_empty() {
                if let Some(f) = filter {
                    if !case_base.contains(f) {
                        report.skipped += 1;
                        report.cases.push(TestCaseResult {
                            name: case_base,
                            status: TestCaseStatus::Skipped,
                            failure: None,
                        });
                        return Ok(());
                    }
                }
                tap_runnable += 1;
                planned.push(PlannedUnit::File {
                    name: case_base,
                    path: p,
                });
            } else {
                let mut tests = Vec::new();
                for fn_name in disc.tests {
                    let case_name = format!("{case_base}.{fn_name}");
                    if let Some(f) = filter {
                        if !case_name.contains(f) && !case_base.contains(f) {
                            report.skipped += 1;
                            report.cases.push(TestCaseResult {
                                name: case_name,
                                status: TestCaseStatus::Skipped,
                                failure: None,
                            });
                            continue;
                        }
                    }
                    tests.push((case_name, fn_name));
                }
                if tests.is_empty() && disc.discover_error.is_none() {
                    return Ok(());
                }
                tap_runnable += if disc.discover_error.is_some() {
                    1
                } else {
                    tests.len()
                };
                planned.push(PlannedUnit::AtTestFile {
                    file_stem: case_base,
                    path: p,
                    setup: disc.setup,
                    before: disc.before,
                    after: disc.after,
                    teardown: disc.teardown,
                    tests,
                    discover_error: disc.discover_error,
                });
            }
            Ok(())
        };

    if single_sy {
        let base = root
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        plan_one_sy(root.to_path_buf(), base, &mut report)?;
    } else if use_tests_dir {
        let paths = collect_sy_files(&tests_dir)?;
        for p in paths {
            let base = test_file_case_base(&tests_dir, &p);
            plan_one_sy(p, base, &mut report)?;
        }
    }

    if tap {
        println!("TAP version 13");
        let runnable = if use_project_main { 1 } else { tap_runnable };
        println!("1..{runnable}");
    }

    let mut index = 0usize;
    if single_sy || use_tests_dir {
        for unit in planned {
            match unit {
                PlannedUnit::File { name, path } => match run_file_with_options(&path, options) {
                    Ok(()) => {
                        let outcomes = syx_vm::take_registered_test_outcomes();
                        if outcomes.is_empty() {
                            index += 1;
                            report.passed += 1;
                            report.cases.push(TestCaseResult {
                                name: name.clone(),
                                status: TestCaseStatus::Passed,
                                failure: None,
                            });
                            if tap {
                                println!("ok {index} - {name}");
                            }
                        } else {
                            let mut first_fail: Option<SyxDiagnostic> = None;
                            for outcome in outcomes {
                                index += 1;
                                let case_name = format!("{name}.{}", outcome.name);
                                if let Some(failure) = outcome.failure {
                                    report.failed += 1;
                                    report.cases.push(TestCaseResult {
                                        name: case_name.clone(),
                                        status: TestCaseStatus::Failed,
                                        failure: Some(failure.clone()),
                                    });
                                    if tap {
                                        println!("not ok {index} - {case_name} # {failure}");
                                    } else if !continue_on_fail && first_fail.is_none() {
                                        first_fail = Some(driver_error_at(
                                            failure,
                                            None,
                                            Span::DUMMY,
                                            "T001",
                                        ));
                                    }
                                } else {
                                    report.passed += 1;
                                    report.cases.push(TestCaseResult {
                                        name: case_name.clone(),
                                        status: TestCaseStatus::Passed,
                                        failure: None,
                                    });
                                    if tap {
                                        println!("ok {index} - {case_name}");
                                    }
                                }
                            }
                            if let Some(err) = first_fail {
                                return Err(err);
                            }
                        }
                    }
                    Err(e) => {
                        let _ = syx_vm::take_registered_test_outcomes();
                        index += 1;
                        let failure = e.to_string();
                        report.failed += 1;
                        report.cases.push(TestCaseResult {
                            name: name.clone(),
                            status: TestCaseStatus::Failed,
                            failure: Some(failure.clone()),
                        });
                        if tap {
                            println!("not ok {index} - {name} # {failure}");
                        } else if !continue_on_fail {
                            return Err(e);
                        }
                    }
                },
                PlannedUnit::AtTestFile {
                    file_stem,
                    path,
                    setup,
                    before,
                    after,
                    teardown,
                    tests,
                    discover_error,
                } => {
                    if let Some(err) = discover_error {
                        index += 1;
                        report.failed += 1;
                        report.cases.push(TestCaseResult {
                            name: file_stem.clone(),
                            status: TestCaseStatus::Failed,
                            failure: Some(err.clone()),
                        });
                        if tap {
                            println!("not ok {index} - {file_stem} # {err}");
                        } else if !continue_on_fail {
                            return Err(driver_error_at(err, None, Span::DUMMY, "T001"));
                        }
                        continue;
                    }

                    let mut setup_ok = true;
                    if let Some(ref setup_fn) = setup {
                        let setup_registry =
                            if let Err(e) = run_file_entry_with_options(&path, setup_fn, options) {
                                let _ = syx_vm::take_registered_test_outcomes();
                                Some(format!("@TestSetup ({setup_fn}): {e}"))
                            } else {
                                registered_test_failure_text()
                                    .map(|extra| format!("@TestSetup ({setup_fn}): {extra}"))
                            };
                        if let Some(failure) = setup_registry {
                            setup_ok = false;
                            for (case_name, _) in &tests {
                                index += 1;
                                report.failed += 1;
                                report.cases.push(TestCaseResult {
                                    name: case_name.clone(),
                                    status: TestCaseStatus::Failed,
                                    failure: Some(failure.clone()),
                                });
                                if tap {
                                    println!("not ok {index} - {case_name} # {failure}");
                                }
                            }
                            if !continue_on_fail {
                                return Err(driver_error_at(failure, None, Span::DUMMY, "T001"));
                            }
                        }
                    }

                    let mut ran_any_test = false;
                    let mut human_stop: Option<SyxDiagnostic> = None;
                    if setup_ok {
                        for (case_name, fn_name) in &tests {
                            if human_stop.is_some() {
                                break;
                            }
                            index += 1;
                            ran_any_test = true;

                            let mut case_failure: Option<String> = None;
                            if let Some(ref before_fn) = before {
                                if let Err(e) =
                                    run_file_entry_with_options(&path, before_fn, options)
                                {
                                    let _ = syx_vm::take_registered_test_outcomes();
                                    case_failure = Some(format!("@TestBefore ({before_fn}): {e}"));
                                } else if let Some(extra) = registered_test_failure_text() {
                                    case_failure =
                                        Some(format!("@TestBefore ({before_fn}): {extra}"));
                                }
                            }
                            if case_failure.is_none() {
                                if let Err(e) = run_file_entry_with_options(&path, fn_name, options)
                                {
                                    let _ = syx_vm::take_registered_test_outcomes();
                                    case_failure = Some(e.to_string());
                                } else if let Some(extra) = registered_test_failure_text() {
                                    case_failure = Some(extra);
                                }
                            }

                            // `@TestAfter` — per case, even if Before/Test failed.
                            let mut after_extra: Option<String> = None;
                            if let Some(ref after_fn) = after {
                                if let Err(e) =
                                    run_file_entry_with_options(&path, after_fn, options)
                                {
                                    let _ = syx_vm::take_registered_test_outcomes();
                                    let af = format!("@TestAfter ({after_fn}): {e}");
                                    if case_failure.is_some() {
                                        after_extra = Some(af);
                                    } else {
                                        case_failure = Some(af);
                                    }
                                } else if let Some(extra) = registered_test_failure_text() {
                                    let af = format!("@TestAfter ({after_fn}): {extra}");
                                    if case_failure.is_some() {
                                        after_extra = Some(af);
                                    } else {
                                        case_failure = Some(af);
                                    }
                                }
                            }

                            if let Some(failure) = case_failure {
                                report.failed += 1;
                                report.cases.push(TestCaseResult {
                                    name: case_name.clone(),
                                    status: TestCaseStatus::Failed,
                                    failure: Some(failure.clone()),
                                });
                                if tap {
                                    println!("not ok {index} - {case_name} # {failure}");
                                } else if !continue_on_fail {
                                    human_stop =
                                        Some(driver_error_at(failure, None, Span::DUMMY, "T001"));
                                }
                            } else {
                                report.passed += 1;
                                report.cases.push(TestCaseResult {
                                    name: case_name.clone(),
                                    status: TestCaseStatus::Passed,
                                    failure: None,
                                });
                                if tap {
                                    println!("ok {index} - {case_name}");
                                }
                            }

                            if let Some(af) = after_extra {
                                index += 1;
                                let af_name = format!("{case_name}.@TestAfter");
                                report.failed += 1;
                                report.cases.push(TestCaseResult {
                                    name: af_name.clone(),
                                    status: TestCaseStatus::Failed,
                                    failure: Some(af.clone()),
                                });
                                if tap {
                                    println!("not ok {index} - {af_name} # {af}");
                                } else if !continue_on_fail && human_stop.is_none() {
                                    human_stop =
                                        Some(driver_error_at(af, None, Span::DUMMY, "T001"));
                                }
                            }
                        }
                    }

                    // `@TestTeardown` — once after all cases (if setup ok and entered ≥1).
                    if setup_ok && ran_any_test {
                        if let Some(ref teardown_fn) = teardown {
                            let teardown_failure = if let Err(e) =
                                run_file_entry_with_options(&path, teardown_fn, options)
                            {
                                let _ = syx_vm::take_registered_test_outcomes();
                                Some(format!("@TestTeardown ({teardown_fn}): {e}"))
                            } else {
                                registered_test_failure_text()
                                    .map(|extra| format!("@TestTeardown ({teardown_fn}): {extra}"))
                            };
                            if let Some(failure) = teardown_failure {
                                let td_name = format!("{file_stem}.@TestTeardown");
                                index += 1;
                                report.failed += 1;
                                report.cases.push(TestCaseResult {
                                    name: td_name.clone(),
                                    status: TestCaseStatus::Failed,
                                    failure: Some(failure.clone()),
                                });
                                if tap {
                                    println!("not ok {index} - {td_name} # {failure}");
                                } else if !continue_on_fail && human_stop.is_none() {
                                    human_stop =
                                        Some(driver_error_at(failure, None, Span::DUMMY, "T001"));
                                }
                            }
                        }
                    }

                    if let Some(e) = human_stop {
                        return Err(e);
                    }
                }
            }
        }
    } else {
        index = 1;
        let name = "main".to_string();
        match run_project_with_options(root, options) {
            Ok(()) => {
                report.passed = 1;
                report.cases.push(TestCaseResult {
                    name: name.clone(),
                    status: TestCaseStatus::Passed,
                    failure: None,
                });
                if tap {
                    println!("ok 1 - main");
                }
            }
            Err(e) => {
                let failure = e.to_string();
                report.failed = 1;
                report.cases.push(TestCaseResult {
                    name,
                    status: TestCaseStatus::Failed,
                    failure: Some(failure.clone()),
                });
                if tap {
                    println!("not ok 1 - main # {failure}");
                } else if !continue_on_fail {
                    return Err(e);
                }
            }
        }
    }

    if report.failed > 0 && !structured {
        return Err(driver_error_at(
            format!("{} test(s) failed", report.failed),
            None,
            Span::DUMMY,
            "T001",
        ));
    }
    Ok(report)
}

struct FileTestPlan {
    tests: Vec<String>,
    setup: Option<String>,
    before: Option<String>,
    after: Option<String>,
    teardown: Option<String>,
    discover_error: Option<String>,
}

/// Relative path under `tests/` without `.sy`, posix separators (e.g. `nested/deep`).
fn test_file_case_base(tests_dir: &Path, file: &Path) -> String {
    let rel = file.strip_prefix(tests_dir).unwrap_or(file);
    let mut s = rel.to_string_lossy().replace('\\', "/");
    if let Some(stripped) = s.strip_suffix(".sy") {
        s = stripped.to_string();
    } else if let Some(stem) = file.file_stem().and_then(|x| x.to_str()) {
        s = stem.to_string();
    }
    s
}

/// Top-level zero-arg `@Test` / hooks (PascalCase attrs, source order).
fn discover_file_test_plan(path: &Path) -> SyxResult<FileTestPlan> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        driver_error_at(
            format!("Failed to read {}: {e}", path.display()),
            None,
            Span::DUMMY,
            "D001",
        )
    })?;
    let source = SourceFile::new(path.display().to_string(), text);
    let module = Parser::parse(source)?;
    let mut tests = Vec::new();
    let mut setup: Option<String> = None;
    let mut before: Option<String> = None;
    let mut after: Option<String> = None;
    let mut teardown: Option<String> = None;
    let mut discover_error = None;
    for decl in &module.decls {
        if let Decl::Function {
            attrs,
            name,
            params,
            ..
        } = decl
        {
            if !params.is_empty() {
                continue;
            }
            let has_test = attrs.iter().any(|a| a.name == "Test");
            let has_setup = attrs.iter().any(|a| a.name == "TestSetup");
            let has_before = attrs.iter().any(|a| a.name == "TestBefore");
            let has_after = attrs.iter().any(|a| a.name == "TestAfter");
            let has_teardown = attrs.iter().any(|a| a.name == "TestTeardown");
            if has_setup {
                if setup.is_some() {
                    discover_error = Some("multiple @TestSetup functions in one file".into());
                } else {
                    setup = Some(name.to_string());
                }
            }
            if has_before {
                if before.is_some() {
                    discover_error = Some("multiple @TestBefore functions in one file".into());
                } else {
                    before = Some(name.to_string());
                }
            }
            if has_after {
                if after.is_some() {
                    discover_error = Some("multiple @TestAfter functions in one file".into());
                } else {
                    after = Some(name.to_string());
                }
            }
            if has_teardown {
                if teardown.is_some() {
                    discover_error = Some("multiple @TestTeardown functions in one file".into());
                } else {
                    teardown = Some(name.to_string());
                }
            }
            if has_test {
                tests.push(name.to_string());
            }
        }
    }
    Ok(FileTestPlan {
        tests,
        setup,
        before,
        after,
        teardown,
        discover_error,
    })
}

fn registered_test_failure_text() -> Option<String> {
    let parts: Vec<String> = syx_vm::take_registered_test_outcomes()
        .into_iter()
        .filter_map(|outcome| {
            outcome
                .failure
                .map(|failure| format!("{}: {failure}", outcome.name))
        })
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("; "))
    }
}

/// Compile and run a specific entry function (used for `@test` cases).
fn run_file_entry_with_options(path: &Path, entry: &str, options: &DriverOptions) -> SyxResult<()> {
    let _ = syx_host_plugin::load_from_env();
    ensure_vm_plugin_dispatch();
    let mut unit = compile_linked_with_options(path, options)?;
    unit.program.entry_point = smol_str::SmolStr::new(entry);
    run_compiled_program_with_strict(unit.program, options.strict_concurrency)
}
