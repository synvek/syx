//! Phase 84: stdlib production hosts (spawn/wait, StringBuilder, Bytes, File.watch, Testing framework, Regex+, Channel.close).

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use regex::Regex;
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_runtime_core::{platform_int_value, value_description, Value};

static SB_NEXT_ID: AtomicI64 = AtomicI64::new(1);
static SB_TABLE: OnceLock<Mutex<HashMap<i64, String>>> = OnceLock::new();
static CHILD_NEXT_ID: AtomicI64 = AtomicI64::new(1);
static CHILD_TABLE: OnceLock<Mutex<HashMap<i64, Child>>> = OnceLock::new();
static BUILDER_NEXT_ID: AtomicI64 = AtomicI64::new(1);
static PROCESS_BUILDER_SESSIONS: OnceLock<Mutex<HashMap<i64, ProcessBuilderSession>>> =
    OnceLock::new();
static PIPE_NEXT_ID: AtomicI64 = AtomicI64::new(1);
static CHILD_PIPE_SESSIONS: OnceLock<Mutex<HashMap<i64, ChildPipeEnd>>> = OnceLock::new();
static TEST_REGISTRY: OnceLock<Mutex<Vec<TestEntry>>> = OnceLock::new();

struct ProcessBuilderSession {
    command: String,
    args: Vec<String>,
    cwd: Option<String>,
    env: Vec<(String, String)>,
}

enum ChildPipeEnd {
    Stdin(ChildStdin),
    Stdout(ChildStdout),
    Stderr(ChildStderr),
}

#[derive(Clone)]
struct TestEntry {
    name: String,
    body: Option<Value>,
}

fn test_registry() -> &'static Mutex<Vec<TestEntry>> {
    TEST_REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

/// One `Testing.test` closure after it ran inside the VM entry.
#[derive(Clone, Debug)]
pub struct RegisteredTestOutcome {
    pub name: String,
    pub failure: Option<String>,
}

thread_local! {
    static LAST_REGISTERED_OUTCOMES: RefCell<Vec<RegisteredTestOutcome>> = const { RefCell::new(Vec::new()) };
}

pub fn clear_registered_tests() {
    if let Ok(mut guard) = test_registry().lock() {
        guard.clear();
    }
}

pub fn take_registered_test_bodies() -> Vec<(String, Option<Value>)> {
    test_registry()
        .lock()
        .map(|mut guard| {
            std::mem::take(&mut *guard)
                .into_iter()
                .map(|entry| (entry.name, entry.body))
                .collect()
        })
        .unwrap_or_default()
}

pub fn store_registered_test_outcomes(outcomes: Vec<RegisteredTestOutcome>) {
    LAST_REGISTERED_OUTCOMES.with(|slot| *slot.borrow_mut() = outcomes);
}

pub fn take_registered_test_outcomes() -> Vec<RegisteredTestOutcome> {
    LAST_REGISTERED_OUTCOMES.with(|slot| std::mem::take(&mut *slot.borrow_mut()))
}

fn sb_table() -> &'static Mutex<HashMap<i64, String>> {
    SB_TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn child_table() -> &'static Mutex<HashMap<i64, Child>> {
    CHILD_TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn builder_sessions() -> &'static Mutex<HashMap<i64, ProcessBuilderSession>> {
    PROCESS_BUILDER_SESSIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn pipe_sessions() -> &'static Mutex<HashMap<i64, ChildPipeEnd>> {
    CHILD_PIPE_SESSIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Drop **untaken** piped stdio so wait/kill cannot block on a full unread pipe.
/// Taken ends live in `CHILD_PIPE_SESSIONS` (ADR-243 product Pipe API).
fn drop_child_pipes(child: &mut Child) {
    drop(child.stdin.take());
    drop(child.stdout.take());
    drop(child.stderr.take());
}

/// Invoke registered closures on the active VM. `assert` / `assertEq` host `Err` counts as failure.
pub fn run_all_registered_tests() -> i32 {
    let bodies = take_registered_test_bodies();
    if bodies.is_empty() {
        return 0;
    }
    #[cfg(feature = "interpreter")]
    {
        let Some(program) =
            crate::program_ctx::with_program_ctx(|ctx| std::sync::Arc::clone(&ctx.program))
        else {
            return bodies.len() as i32;
        };
        return crate::program_ctx::with_active_vm(|vm| {
            let mut failed = 0i32;
            for (_name, body) in &bodies {
                if registered_body_error(vm, &program, body).is_some() {
                    failed += 1;
                }
            }
            failed
        })
        .unwrap_or(bodies.len() as i32);
    }
    #[cfg(not(feature = "interpreter"))]
    {
        let _ = bodies;
        0
    }
}

#[cfg(feature = "interpreter")]
pub(crate) fn registered_body_error(
    vm: &mut crate::VmInstance,
    program: &syx_codegen::CompiledProgram,
    body: &Option<Value>,
) -> Option<String> {
    let Some(Value::Closure { fn_index, captures }) = body else {
        return Some("registered test has no closure".into());
    };
    let Some(func) = program.functions.get(*fn_index as usize) else {
        return Some(format!("missing closure function {fn_index}"));
    };
    match vm.run_closure_with_captures(func, program, captures, &[]) {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
    }
}

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().map(|v| v.as_str()).unwrap_or_default()
}

fn bytes_from_value(v: &Value) -> Vec<u8> {
    if let Some(items) = Value::materialize_array(v.clone()) {
        return items
            .iter()
            .map(|x| match x {
                Value::U8(b) => *b,
                _ => x.as_i64().clamp(0, 255) as u8,
            })
            .collect();
    }
    v.as_str().into_bytes()
}

fn value_from_bytes(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|&b| Value::U8(b)).collect())
}

// --- Process spawn/wait + execOutput ---

pub fn host_process_exec_output(stack: &mut Vec<Value>) -> SyxResult<()> {
    let args_val = stack.pop().unwrap_or(Value::Array(vec![]));
    let command = pop_str(stack);
    let args = Value::materialize_array(args_val).unwrap_or_default();
    let cmd_args: Vec<String> = args.iter().map(|v| v.as_str()).collect();
    let output = Command::new(&command).args(&cmd_args).output();
    match output {
        Ok(out) => {
            let code = out.status.code().unwrap_or(-1);
            crate::stdlib_phase83_hosts::set_last_exit_code(code);
            stack.push(Value::Str(
                String::from_utf8_lossy(&out.stdout).into_owned(),
            ));
        }
        Err(e) => {
            crate::stdlib_phase83_hosts::set_last_exit_code(127);
            stack.push(Value::Str(format!("exec error: {e}")));
        }
    }
    Ok(())
}

pub fn host_process_spawn(stack: &mut Vec<Value>) -> SyxResult<()> {
    let args_val = stack.pop().unwrap_or(Value::Array(vec![]));
    let command = pop_str(stack);
    let args = Value::materialize_array(args_val).unwrap_or_default();
    let cmd_args: Vec<String> = args.iter().map(|v| v.as_str()).collect();
    match Command::new(&command)
        .args(&cmd_args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => {
            let id = CHILD_NEXT_ID.fetch_add(1, Ordering::Relaxed);
            child_table().lock().unwrap().insert(id, child);
            stack.push(platform_int_value(id));
        }
        Err(_) => stack.push(platform_int_value(-1)),
    }
    Ok(())
}

pub fn host_process_wait(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let code = child_table()
        .lock()
        .unwrap()
        .remove(&handle)
        .and_then(|mut child| child.wait().ok())
        .and_then(|s| s.code())
        .unwrap_or(-1);
    crate::stdlib_phase83_hosts::set_last_exit_code(code);
    stack.push(platform_int_value(code as i64));
    Ok(())
}

/// M2 W37 / ADR-181: live `CHILD_TABLE` count for EC-M2-3 Process leak nails (test/ops only).
pub fn process_child_live_count() -> usize {
    child_table().lock().map(|g| g.len()).unwrap_or(0)
}

/// Test cleanup: drop a live child handle by id without going through Process.wait.
pub fn process_child_force_close(id: i64) -> bool {
    child_table()
        .lock()
        .ok()
        .and_then(|mut g| g.remove(&id))
        .is_some()
}

/// M2 W109 / ADR-253: live `CHILD_PIPE_SESSIONS` count (test/ops only).
pub fn child_pipe_live_count() -> usize {
    pipe_sessions().lock().map(|g| g.len()).unwrap_or(0)
}

/// Test cleanup: drop a live ChildPipe session by id.
pub fn child_pipe_force_close(id: i64) -> bool {
    pipe_sessions()
        .lock()
        .ok()
        .and_then(|mut g| g.remove(&id))
        .is_some()
}

// --- ADR-242 / M2-S6-01: ProcessBuilder + Child kill/waitTimeout ---

/// `ProcessBuilder.create(command, args) -> Int`
pub fn host_process_builder_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let args_val = stack.pop().unwrap_or(Value::Array(vec![]));
    let command = pop_str(stack);
    let args = Value::materialize_array(args_val).unwrap_or_default();
    let cmd_args: Vec<String> = args.iter().map(|v| v.as_str()).collect();
    if command.is_empty() {
        return Err(SyxDiagnostic::error("ProcessBuilder create: empty command").with_code("R003"));
    }
    let id = BUILDER_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    builder_sessions().lock().unwrap().insert(
        id,
        ProcessBuilderSession {
            command,
            args: cmd_args,
            cwd: None,
            env: Vec::new(),
        },
    );
    stack.push(platform_int_value(id));
    Ok(())
}

/// `ProcessBuilder.cwd(builder, path)`
pub fn host_process_builder_cwd(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = pop_str(stack);
    let builder = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let mut sessions = builder_sessions().lock().unwrap();
    let Some(session) = sessions.get_mut(&builder) else {
        return Err(
            SyxDiagnostic::error(format!("ProcessBuilder invalid handle {builder}"))
                .with_code("R003"),
        );
    };
    session.cwd = if path.is_empty() { None } else { Some(path) };
    stack.push(Value::Nil);
    Ok(())
}

/// `ProcessBuilder.env(builder, key, value)` — overlay; inherit rest of parent env.
pub fn host_process_builder_env(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = pop_str(stack);
    let key = pop_str(stack);
    let builder = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let mut sessions = builder_sessions().lock().unwrap();
    let Some(session) = sessions.get_mut(&builder) else {
        return Err(
            SyxDiagnostic::error(format!("ProcessBuilder invalid handle {builder}"))
                .with_code("R003"),
        );
    };
    if key.is_empty() {
        return Err(SyxDiagnostic::error("ProcessBuilder env: empty key").with_code("R003"));
    }
    if let Some((_, v)) = session.env.iter_mut().find(|(k, _)| k == &key) {
        *v = value;
    } else {
        session.env.push((key, value));
    }
    stack.push(Value::Nil);
    Ok(())
}

/// `ProcessBuilder.spawn(builder) -> Int` — child handle in `CHILD_TABLE`.
pub fn host_process_builder_spawn(stack: &mut Vec<Value>) -> SyxResult<()> {
    let builder = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let session = {
        let sessions = builder_sessions().lock().unwrap();
        sessions.get(&builder).map(|s| ProcessBuilderSession {
            command: s.command.clone(),
            args: s.args.clone(),
            cwd: s.cwd.clone(),
            env: s.env.clone(),
        })
    };
    let Some(session) = session else {
        return Err(
            SyxDiagnostic::error(format!("ProcessBuilder invalid handle {builder}"))
                .with_code("R003"),
        );
    };
    let mut cmd = Command::new(&session.command);
    // ADR-243: stdin piped for ChildPipe.write; stdout/stderr piped for readChunk.
    cmd.args(&session.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(cwd) = &session.cwd {
        cmd.current_dir(cwd);
    }
    for (k, v) in &session.env {
        cmd.env(k, v);
    }
    let child = cmd.spawn().map_err(|e| {
        SyxDiagnostic::error(format!("ProcessBuilder spawn failed: {e}")).with_code("R003")
    })?;
    let id = CHILD_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    child_table().lock().unwrap().insert(id, child);
    stack.push(platform_int_value(id));
    Ok(())
}

/// `Child.wait(handle) -> Int` — remove + wait; fail if already gone.
pub fn host_child_wait(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let mut child = child_table()
        .lock()
        .unwrap()
        .remove(&handle)
        .ok_or_else(|| {
            SyxDiagnostic::error(format!("Child invalid handle {handle}")).with_code("R003")
        })?;
    drop_child_pipes(&mut child);
    let status = child
        .wait()
        .map_err(|e| SyxDiagnostic::error(format!("Child wait failed: {e}")).with_code("R003"))?;
    let code = status.code().unwrap_or(-1);
    crate::stdlib_phase83_hosts::set_last_exit_code(code);
    stack.push(platform_int_value(code as i64));
    Ok(())
}

/// `Child.kill(handle)` — kill + remove; fail if already gone.
pub fn host_child_kill(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let mut child = child_table()
        .lock()
        .unwrap()
        .remove(&handle)
        .ok_or_else(|| {
            SyxDiagnostic::error(format!("Child invalid handle {handle}")).with_code("R003")
        })?;
    let _ = child.kill();
    drop_child_pipes(&mut child);
    let _ = child.wait();
    stack.push(Value::Nil);
    Ok(())
}

/// ADR-244 / M2-S6-03: `Child.signal(handle, name)` — send TERM/INT/KILL **without** removing handle.
pub fn host_child_signal(stack: &mut Vec<Value>) -> SyxResult<()> {
    let raw_name = pop_str(stack);
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let mut name = raw_name.trim().to_ascii_uppercase();
    if let Some(stripped) = name.strip_prefix("SIG") {
        name = stripped.to_string();
    }
    let mut guard = child_table().lock().unwrap();
    let Some(child) = guard.get_mut(&handle) else {
        return Err(
            SyxDiagnostic::error(format!("Child invalid handle {handle}")).with_code("R003"),
        );
    };

    #[cfg(unix)]
    {
        let sig = match name.as_str() {
            "TERM" => libc::SIGTERM,
            "INT" => libc::SIGINT,
            "KILL" => libc::SIGKILL,
            _ => {
                return Err(SyxDiagnostic::error(format!(
                    "Child.signal unsupported name '{raw_name}' (use TERM/INT/KILL)"
                ))
                .with_code("R003"));
            }
        };
        let pid = child.id() as libc::pid_t;
        let rc = unsafe { libc::kill(pid, sig) };
        if rc != 0 {
            let err = std::io::Error::last_os_error();
            return Err(
                SyxDiagnostic::error(format!("Child.signal failed: {err}")).with_code("R003")
            );
        }
    }

    #[cfg(windows)]
    {
        match name.as_str() {
            "KILL" => {
                child.kill().map_err(|e| {
                    SyxDiagnostic::error(format!("Child.signal KILL failed: {e}")).with_code("R003")
                })?;
            }
            "TERM" | "INT" => {
                return Err(SyxDiagnostic::error(format!(
                    "Child.signal '{raw_name}' unsupported on Windows (use KILL)"
                ))
                .with_code("R003"));
            }
            _ => {
                return Err(SyxDiagnostic::error(format!(
                    "Child.signal unsupported name '{raw_name}' (use KILL on Windows)"
                ))
                .with_code("R003"));
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = (child, name, raw_name);
        return Err(
            SyxDiagnostic::error("Child.signal unsupported on this platform").with_code("R003"),
        );
    }

    stack.push(Value::Nil);
    Ok(())
}

/// `Child.waitTimeout(handle, timeoutMs) -> Int` — try_wait loop; on timeout kill + Err.
pub fn host_child_wait_timeout(stack: &mut Vec<Value>) -> SyxResult<()> {
    let timeout_ms = stack.pop().unwrap_or(Value::I64(0)).as_i64().max(0) as u64;
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    loop {
        let mut guard = child_table().lock().unwrap();
        let Some(child) = guard.get_mut(&handle) else {
            return Err(
                SyxDiagnostic::error(format!("Child invalid handle {handle}")).with_code("R003"),
            );
        };
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut child = guard.remove(&handle).expect("handle present");
                drop(guard);
                drop_child_pipes(&mut child);
                let code = status.code().unwrap_or(-1);
                crate::stdlib_phase83_hosts::set_last_exit_code(code);
                stack.push(platform_int_value(code as i64));
                return Ok(());
            }
            Ok(None) => {
                if Instant::now() >= deadline {
                    let mut child = guard.remove(&handle).expect("handle present");
                    drop(guard);
                    let _ = child.kill();
                    drop_child_pipes(&mut child);
                    let _ = child.wait();
                    return Err(SyxDiagnostic::error(format!(
                        "Child waitTimeout exceeded ({timeout_ms}ms) for handle {handle}"
                    ))
                    .with_code("R003"));
                }
                drop(guard);
                thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                let _ = guard.remove(&handle);
                return Err(
                    SyxDiagnostic::error(format!("Child waitTimeout failed: {e}"))
                        .with_code("R003"),
                );
            }
        }
    }
}

// --- ADR-243 / M2-S6-02b: ChildPipe take + read/write/close ---

fn insert_pipe(end: ChildPipeEnd) -> i64 {
    let id = PIPE_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    pipe_sessions().lock().unwrap().insert(id, end);
    id
}

/// `Child.stdout(handle) -> Int` — take-once into pipe session.
pub fn host_child_stdout(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let mut guard = child_table().lock().unwrap();
    let Some(child) = guard.get_mut(&handle) else {
        return Err(
            SyxDiagnostic::error(format!("Child invalid handle {handle}")).with_code("R003"),
        );
    };
    let out = child.stdout.take().ok_or_else(|| {
        SyxDiagnostic::error(format!("Child stdout already taken for handle {handle}"))
            .with_code("R003")
    })?;
    drop(guard);
    stack.push(platform_int_value(insert_pipe(ChildPipeEnd::Stdout(out))));
    Ok(())
}

/// `Child.stderr(handle) -> Int`
pub fn host_child_stderr(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let mut guard = child_table().lock().unwrap();
    let Some(child) = guard.get_mut(&handle) else {
        return Err(
            SyxDiagnostic::error(format!("Child invalid handle {handle}")).with_code("R003"),
        );
    };
    let err = child.stderr.take().ok_or_else(|| {
        SyxDiagnostic::error(format!("Child stderr already taken for handle {handle}"))
            .with_code("R003")
    })?;
    drop(guard);
    stack.push(platform_int_value(insert_pipe(ChildPipeEnd::Stderr(err))));
    Ok(())
}

/// `Child.stdin(handle) -> Int`
pub fn host_child_stdin(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let mut guard = child_table().lock().unwrap();
    let Some(child) = guard.get_mut(&handle) else {
        return Err(
            SyxDiagnostic::error(format!("Child invalid handle {handle}")).with_code("R003"),
        );
    };
    let inp = child.stdin.take().ok_or_else(|| {
        SyxDiagnostic::error(format!("Child stdin already taken for handle {handle}"))
            .with_code("R003")
    })?;
    drop(guard);
    stack.push(platform_int_value(insert_pipe(ChildPipeEnd::Stdin(inp))));
    Ok(())
}

/// `ChildPipe.readChunk(pipe, maxBytes) -> Array<UInt8>` (empty = EOF).
pub fn host_child_pipe_read_chunk(stack: &mut Vec<Value>) -> SyxResult<()> {
    let max_bytes = stack.pop().unwrap_or(Value::I32(4096)).as_i64().max(0) as usize;
    let pipe = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let mut sessions = pipe_sessions().lock().unwrap();
    let Some(end) = sessions.get_mut(&pipe) else {
        return Err(
            SyxDiagnostic::error(format!("ChildPipe invalid handle {pipe}")).with_code("R003"),
        );
    };
    if max_bytes == 0 {
        stack.push(Value::Array(vec![]));
        return Ok(());
    }
    let mut buf = vec![0u8; max_bytes];
    let n = match end {
        ChildPipeEnd::Stdout(r) => r.read(&mut buf),
        ChildPipeEnd::Stderr(r) => r.read(&mut buf),
        ChildPipeEnd::Stdin(_) => {
            return Err(
                SyxDiagnostic::error("ChildPipe readChunk: stdin is write-only").with_code("R003"),
            );
        }
    }
    .map_err(|e| SyxDiagnostic::error(format!("ChildPipe read failed: {e}")).with_code("R001"))?;
    buf.truncate(n);
    stack.push(Value::Array(buf.into_iter().map(Value::U8).collect()));
    Ok(())
}

/// `ChildPipe.write(pipe, bytes) -> Int` — stdin only.
pub fn host_child_pipe_write(stack: &mut Vec<Value>) -> SyxResult<()> {
    let bytes_val = stack.pop().unwrap_or(Value::Array(vec![]));
    let pipe = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let bytes = bytes_from_value(&bytes_val);
    let mut sessions = pipe_sessions().lock().unwrap();
    let Some(end) = sessions.get_mut(&pipe) else {
        return Err(
            SyxDiagnostic::error(format!("ChildPipe invalid handle {pipe}")).with_code("R003"),
        );
    };
    let n = match end {
        ChildPipeEnd::Stdin(w) => w.write(&bytes).and_then(|n| {
            w.flush()?;
            Ok(n)
        }),
        ChildPipeEnd::Stdout(_) | ChildPipeEnd::Stderr(_) => {
            return Err(
                SyxDiagnostic::error("ChildPipe write: stdout/stderr are read-only")
                    .with_code("R003"),
            );
        }
    }
    .map_err(|e| SyxDiagnostic::error(format!("ChildPipe write failed: {e}")).with_code("R001"))?;
    stack.push(platform_int_value(n as i64));
    Ok(())
}

/// `ChildPipe.close(pipe)` — remove session; further ops fail.
pub fn host_child_pipe_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let pipe = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let removed = pipe_sessions().lock().unwrap().remove(&pipe).is_some();
    if !removed {
        return Err(
            SyxDiagnostic::error(format!("ChildPipe invalid handle {pipe}")).with_code("R003"),
        );
    }
    stack.push(Value::Nil);
    Ok(())
}

// --- File.watch (mock + CI) ---

pub fn host_file_watch(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = pop_str(stack);
    let mock = std::env::var("SYX_FILE_WATCH_MOCK")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if mock {
        stack.push(Value::Array(vec![Value::Str("watch:mock-change".into())]));
        return Ok(());
    }
    #[cfg(all(unix, feature = "file"))]
    {
        use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
        use std::path::Path;
        if Path::new(&path).exists() {
            let (tx, _rx) = std::sync::mpsc::channel();
            if let Ok(mut watcher) = RecommendedWatcher::new(tx, Config::default()) {
                let _ = watcher.watch(Path::new(&path), RecursiveMode::NonRecursive);
                stack.push(Value::Array(vec![Value::Str(format!("watch:os:{path}"))]));
                return Ok(());
            }
        }
    }
    stack.push(Value::Array(vec![]));
    Ok(())
}

// --- StringBuilder ---

pub fn host_string_builder_new(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = SB_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    sb_table().lock().unwrap().insert(id, String::new());
    stack.push(platform_int_value(id));
    Ok(())
}

pub fn host_string_builder_append(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = pop_str(stack);
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    if let Some(buf) = sb_table().lock().unwrap().get_mut(&id) {
        buf.push_str(&text);
    }
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_string_builder_to_string(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let s = sb_table()
        .lock()
        .unwrap()
        .get(&id)
        .cloned()
        .unwrap_or_default();
    stack.push(Value::Str(s));
    Ok(())
}

// --- Bytes ---

pub fn host_bytes_from_array(stack: &mut Vec<Value>) -> SyxResult<()> {
    let arr = stack.pop().unwrap_or(Value::Array(vec![]));
    let bytes = bytes_from_value(&arr);
    stack.push(value_from_bytes(&bytes));
    Ok(())
}

pub fn host_bytes_slice(stack: &mut Vec<Value>) -> SyxResult<()> {
    let end = stack.pop().unwrap_or(Value::I64(0)).as_i64() as usize;
    let start = stack.pop().unwrap_or(Value::I64(0)).as_i64() as usize;
    let arr = stack.pop().unwrap_or(Value::Array(vec![]));
    let bytes = bytes_from_value(&arr);
    let slice = bytes.get(start..end.min(bytes.len())).unwrap_or(&[]);
    stack.push(value_from_bytes(slice));
    Ok(())
}

pub fn host_bytes_concat(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Array(vec![]));
    let a = stack.pop().unwrap_or(Value::Array(vec![]));
    let mut out = bytes_from_value(&a);
    out.extend(bytes_from_value(&b));
    stack.push(value_from_bytes(&out));
    Ok(())
}

// --- Channel.close ---

pub fn host_channel_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-266 / M3-A04: real disconnect lives in channel_hosts.
    crate::channel_hosts::host_channel_close(stack)
}

// --- Regex extensions ---

pub fn host_regex_compile(stack: &mut Vec<Value>) -> SyxResult<()> {
    let pattern = pop_str(stack);
    let ok = Regex::new(&pattern).is_ok();
    stack.push(Value::Str(if ok {
        format!("regex:{pattern}")
    } else {
        "regex:invalid".into()
    }));
    Ok(())
}

pub fn host_regex_find_all(stack: &mut Vec<Value>) -> SyxResult<()> {
    let pattern = pop_str(stack);
    let text = pop_str(stack);
    let matches: Vec<Value> = Regex::new(&pattern)
        .ok()
        .map(|re| {
            re.find_iter(&text)
                .map(|m| Value::Str(m.as_str().to_string()))
                .collect()
        })
        .unwrap_or_default();
    stack.push(Value::Array(matches));
    Ok(())
}

pub fn host_regex_replace_all(stack: &mut Vec<Value>) -> SyxResult<()> {
    let replacement = pop_str(stack);
    let pattern = pop_str(stack);
    let text = pop_str(stack);
    let out = Regex::new(&pattern)
        .ok()
        .map(|re| re.replace_all(&text, replacement.as_str()).into_owned())
        .unwrap_or(text);
    stack.push(Value::Str(out));
    Ok(())
}

// --- Testing framework ---

pub fn host_testing_test(stack: &mut Vec<Value>) -> SyxResult<()> {
    let body = stack.pop().filter(|v| !matches!(v, Value::Nil));
    let name = pop_str(stack);
    test_registry()
        .lock()
        .unwrap()
        .push(TestEntry { name, body });
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_testing_expect_equal(stack: &mut Vec<Value>) -> SyxResult<()> {
    let msg = pop_str(stack);
    let expected = stack.pop().unwrap_or(Value::Nil);
    let actual = stack.pop().unwrap_or(Value::Nil);
    if !syx_runtime_core::value_equals(&actual, &expected) {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "expect failed: {msg} — expected {}, got {}",
            value_description(&expected),
            value_description(&actual)
        )));
    }
    stack.push(Value::Nil);
    Ok(())
}
