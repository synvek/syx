//! R-11 SqlRow census contract.
//! Nails: script green, six accessors row → Dictionary.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r11_sql_row_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r11_sql_row_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
fn r11_sql_row_census_files_exist() {
    assert!(repo_path("scripts/r11_sql_row_census.sh").is_file());
    assert!(repo_path("docs/census/r-11-sql-row-census.md").is_file());
    let row = std::fs::read_to_string(repo_path("packages/data/sql_row.sy")).unwrap();
    assert!(
        row.contains("func getString(row: Dictionary,"),
        "SqlRow.getString row must be Dictionary"
    );
    assert!(
        row.contains("func columnNames(row: Dictionary)"),
        "SqlRow.columnNames row must be Dictionary"
    );
    assert!(
        !row
            .lines()
            .any(|ln| ln.contains("func ") && ln.contains("Object")),
        "SqlRow methods must not spell Object"
    );
    let host = std::fs::read_to_string(repo_path("packages/internal/system_host.sy")).unwrap();
    assert!(
        host.contains("func sql_row_get_string(row: Dictionary,"),
        "SyxHost.sql_row_get_string row must be Dictionary"
    );
    assert!(
        host.contains("func sql_row_column_names(row: Dictionary)"),
        "SyxHost.sql_row_column_names row must be Dictionary"
    );
}

#[test]
fn r11_sql_row_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "r10_hinge_still_green="), 1);
    assert_eq!(parse(&stdout, "sql_row_public_object_residual="), 0);
    assert_eq!(parse(&stdout, "sql_row_params_dictionary="), 1);
    assert_eq!(parse(&stdout, "sql_row_syxhost_dictionary="), 1);
}
