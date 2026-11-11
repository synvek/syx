//! R-5 Mutex/Semaphore census contract.
//! Nails: script green, public Mutex/Semaphore off Object, Int-typed.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r5_mutex_sem_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r5_mutex_sem_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    stdout
}

fn parse(stdout: &str, key: &str) -> i64 {
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix(key) {
            return rest
                .parse()
                .unwrap_or_else(|_| panic!("bad int for {key}: {rest}"));
        }
    }
    panic!("missing key {key} in census output:\n{stdout}");
}

#[test]
fn r5_mutex_sem_census_files_exist() {
    assert!(repo_path("scripts/r5_mutex_sem_census.sh").is_file());
    assert!(repo_path("docs/census/r-5-mutex-sem-census.md").is_file());
    let sync = std::fs::read_to_string(repo_path("packages/sync/mod.sy")).unwrap();
    assert!(
        sync.contains("func create() -> Int"),
        "Mutex.create must return Int"
    );
    assert!(
        sync.contains("func create(permits: Int) -> Int"),
        "Semaphore.create must return Int"
    );
    assert!(
        sync.contains("func lock(mutex: Int)"),
        "Mutex.lock must take Int"
    );
    assert!(
        sync.contains("func wait(sem: Int)"),
        "Semaphore.wait must take Int"
    );
    let mutex_slice = sync
        .split("public class Mutex")
        .nth(1)
        .and_then(|rest| rest.split("public class").next())
        .unwrap_or("");
    let sem_slice = sync
        .split("public class Semaphore")
        .nth(1)
        .and_then(|rest| rest.split("public class").next())
        .unwrap_or(sync.split("public class Semaphore").nth(1).unwrap_or(""));
    for (label, slice) in [("Mutex", mutex_slice), ("Semaphore", sem_slice)] {
        assert!(
            !slice.lines().any(|ln| ln.contains("func ") && ln.contains("Object")),
            "{label} methods must not spell Object"
        );
    }
}

#[test]
fn r5_mutex_sem_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r4_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "mutex_sem_public_object_residual="), 0);
    assert_eq!(parse(&stdout, "mutex_create_returns_int="), 1);
    assert_eq!(parse(&stdout, "semaphore_create_returns_int="), 1);
    assert_eq!(parse(&stdout, "mutex_sem_params_int_typed="), 1);
}
