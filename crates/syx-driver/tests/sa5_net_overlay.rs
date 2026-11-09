//! SA-5: workspace Networking overlay must export Url/Http after SyxHost prepend.
use std::path::PathBuf;
use syx_driver::DriverOptions;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn workspace_networking_import_check() {
    let root = repo_root();
    let src = root.join("examples/net_only_test.sy");
    // recreate if missing
    if !src.is_file() {
        std::fs::write(
            &src,
            "import Networking\n\nfunc main() {\n    let built = Url.build(base: \"https://api.example.com\", path: \"/v1\", query: {\"q\": \"syx\"})\n    print(built)\n}\n",
        )
        .unwrap();
    }
    let opts = DriverOptions::default();
    let result = syx_driver::check_file(&src, &opts);
    match result {
        Ok(()) => {}
        Err(e) => panic!("check failed: {e:?}"),
    }
}
