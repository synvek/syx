use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use syx_pkg::{
    add_dependency, install_project, load_resolved_packages, publish_package_signed,
    verifying_key_hex, DependencyAddSpec,
};
use ed25519_dalek::SigningKey;

const TEST_SIGNING_SEED: [u8; 32] = [0x42; 32];

fn test_signing_key() -> SigningKey {
    SigningKey::from_bytes(&TEST_SIGNING_SEED)
}

fn pin_test_trust_root() {
    std::env::set_var(
        "SYX_REGISTRY_VERIFY_KEY",
        verifying_key_hex(&test_signing_key()),
    );
}

fn unpin_test_trust_root() {
    std::env::remove_var("SYX_REGISTRY_VERIFY_KEY");
}

fn publish_signed(root: &std::path::Path, registry: &str) {
    publish_package_signed(root, registry, &test_signing_key()).unwrap();
}

#[test]
fn path_dependency_install_writes_lock() {
    let root = tempfile::tempdir().unwrap();
    let math_dir = root.path().join("math_pkg");
    std::fs::create_dir_all(&math_dir).unwrap();
    std::fs::write(
        math_dir.join("syx.toml"),
        "name = \"helper\"\nversion = \"1.0.0\"\nmain = \"helper.sy\"\n",
    )
    .unwrap();
    std::fs::write(
        math_dir.join("helper.sy"),
        "public func add(a: Int, b: Int) -> Int { return a + b }",
    )
    .unwrap();

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        r#"name = "app"
version = "0.1.0"
main = "main.sy"

[dependencies.helper]
version = "1.0.0"
path = "../math_pkg"
"#,
    )
    .unwrap();
    std::fs::write(
        app.join("main.sy"),
        "import helper\nfunc main() { print(helper.add(1, 2)) }",
    )
    .unwrap();

    let report = install_project(&app).unwrap();
    assert_eq!(report.package_count, 1);
    assert!(app.join("syx.lock").exists());

    let packages = load_resolved_packages(&app).unwrap();
    assert!(packages.contains_key("helper"));
}

#[test]
fn add_dependency_updates_manifest_and_lock() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("syx.toml"),
        "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    std::fs::write(root.path().join("main.sy"), "func main() {}").unwrap();

    let dep = root.path().join("dep");
    std::fs::create_dir_all(&dep).unwrap();
    std::fs::write(
        dep.join("syx.toml"),
        "name = \"dep\"\nversion = \"1.0.0\"\nmain = \"dep.sy\"\n",
    )
    .unwrap();
    std::fs::write(dep.join("dep.sy"), "public func ok() -> Int { return 1 }").unwrap();

    add_dependency(root.path(), "dep", DependencyAddSpec::path("dep")).unwrap();
    assert!(root.path().join("syx.lock").exists());
}

#[test]
fn http_publish_requires_registry_token() {
    let root = tempfile::tempdir().unwrap();
    let pkg = root.path().join("pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"tokentest\"\nversion = \"0.0.1\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("main.sy"), "func main() { return 0 }").unwrap();
    std::env::remove_var("SYX_REGISTRY_TOKEN");
    let err = syx_pkg::publish_package_with_token(&pkg, "https://packages.sy.dev", None).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("P010") || msg.contains("SYX_REGISTRY_TOKEN"), "got: {msg}");
}

#[test]
fn registry_install_from_file_registry() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    assert!(registry_dir.is_dir());
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );
    let pkg = root.path().join("greet-pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"greet\"\nversion = \"1.0.0\"\nmain = \"greet.sy\"\n",
    )
    .unwrap();
    std::fs::write(
        pkg.join("greet.sy"),
        "public func hello() -> Int { return 99 }",
    )
    .unwrap();

    publish_signed(&pkg, &registry_url);

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "app"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{registry_url}"

[dependencies.greet]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "import greet\nfunc main() {}").unwrap();

    install_project(&app).unwrap();
    let packages = load_resolved_packages(&app).unwrap();
    assert!(packages.contains_key("greet"));
    assert!(packages["greet"].root.join("greet.sy").exists());
    unpin_test_trust_root();
}

#[test]
fn git_dependency_install_from_in_tree_fixture() {
    if !syx_pkg::git_available() {
        eprintln!("git not available, skipping git fixture test");
        return;
    }
    use std::process::Command;

    let root = tempfile::tempdir().unwrap();
    let src = root.path().join("demo-src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("syx.toml"),
        "name = \"demo\"\nversion = \"0.1.0\"\nmain = \"demo.sy\"\n",
    )
    .unwrap();
    std::fs::write(
        src.join("demo.sy"),
        "public func greet() -> Int { return 7 }",
    )
    .unwrap();
    Command::new("git")
        .args(["init"])
        .current_dir(&src)
        .status()
        .expect("git init");
    Command::new("git")
        .args(["add", "."])
        .current_dir(&src)
        .status()
        .expect("git add");
    Command::new("git")
        .args(["-c", "user.email=syx@test", "-c", "user.name=syx", "commit", "-m", "init"])
        .current_dir(&src)
        .status()
        .expect("git commit");
    let bare = root.path().join("demo.git");
    Command::new("git")
        .args(["clone", "--bare", src.to_str().unwrap(), bare.to_str().unwrap()])
        .status()
        .expect("git clone --bare");
    let git_url = format!("file://{}", bare.canonicalize().unwrap().to_string_lossy());

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "app"
version = "0.1.0"
main = "main.sy"

[dependencies.demo]
version = "0.1.0"
git = "{git_url}"
rev = "HEAD"
"#
        ),
    )
    .unwrap();
    std::fs::write(
        app.join("main.sy"),
        "import demo\nfunc main() { print(demo.greet()) }",
    )
    .unwrap();
    install_project(&app).expect("git dependency install");
    assert!(app.join("syx.lock").exists());
    let packages = load_resolved_packages(&app).unwrap();
    assert!(packages.contains_key("demo"));
}

fn start_mock_registry_http(registry_dir: &std::path::Path) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let root = registry_dir.to_path_buf();
    thread::spawn(move || {
        // Enough accepts for multi-package diamond installs (index + download × N).
        for _ in 0..64 {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 4096];
                let n = stream.read(&mut buf).unwrap_or(0);
                let req = String::from_utf8_lossy(&buf[..n]);
                let first = req.lines().next().unwrap_or("");
                let body = if first.contains("/download") {
                    let parts: Vec<_> = first.split_whitespace().nth(1).unwrap_or("").split('/').collect();
                    let name = parts.get(3).copied().unwrap_or("");
                    let version = parts.get(4).copied().unwrap_or("");
                    std::fs::read(
                        root.join("v1/packages")
                            .join(name)
                            .join(version)
                            .join("download"),
                    )
                    .unwrap_or_default()
                } else if first.contains("/v1/packages/") {
                    let parts: Vec<_> = first.split_whitespace().nth(1).unwrap_or("").split('/').collect();
                    let name = parts.last().copied().unwrap_or("");
                    std::fs::read_to_string(
                        root.join("v1/packages").join(name).join("index.json"),
                    )
                    .unwrap_or_else(|_| "{}".into())
                    .into_bytes()
                } else {
                    b"ok".to_vec()
                };
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(resp.as_bytes());
                let _ = stream.write_all(&body);
            }
        }
    });
    thread::sleep(Duration::from_millis(50));
    format!("http://{addr}")
}

#[test]
fn remote_registry_install_from_mock_http() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let file_registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );
    let pkg = root.path().join("remote-pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"remote\"\nversion = \"1.0.0\"\nmain = \"remote.sy\"\n",
    )
    .unwrap();
    std::fs::write(
        pkg.join("remote.sy"),
        "public func ping() -> Int { return 55 }",
    )
    .unwrap();
    publish_signed(&pkg, &file_registry_url);

    let http_url = start_mock_registry_http(&registry_dir);
    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "app"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{http_url}"

[dependencies.remote]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "import remote\nfunc main() {}").unwrap();
    install_project(&app).expect("remote HTTP registry install");
    let packages = load_resolved_packages(&app).unwrap();
    assert!(packages.contains_key("remote"));
    unpin_test_trust_root();
}

#[test]
fn staging_registry_env_publish_install_cycle() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("staging-registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let staging_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );
    std::env::set_var("SYX_REGISTRY_STAGING", &staging_url);

    let pkg = root.path().join("staging-pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"stagingdemo\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(
        pkg.join("lib.sy"),
        "public func ping() -> Int { return 7 }",
    )
    .unwrap();
    publish_signed(&pkg, &staging_url);

    let app = root.path().join("consumer");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        "name = \"consumer\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.stagingdemo]\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    std::fs::write(
        app.join("main.sy"),
        "import stagingdemo\nfunc main() { print(stagingdemo.ping()) }",
    )
    .unwrap();
    install_project(&app).expect("staging install via SYX_REGISTRY_STAGING default");
    std::env::remove_var("SYX_REGISTRY_STAGING");
    unpin_test_trust_root();
}

#[test]
fn r001_semver_conflict_rejects_incompatible_path_version() {
    let root = tempfile::tempdir().unwrap();
    let lib = root.path().join("lib");
    std::fs::create_dir_all(&lib).unwrap();
    std::fs::write(
        lib.join("syx.toml"),
        "name = \"lib\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(lib.join("lib.sy"), "public func ok() -> Int { return 1 }").unwrap();

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        r#"name = "app"
version = "0.1.0"
main = "main.sy"

[dependencies.lib]
version = "^2.0.0"
path = "../lib"
"#,
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "import lib\nfunc main() { print(lib.ok()) }").unwrap();

    let err = install_project(&app).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("R001"),
        "expected R001 semver conflict, got: {msg}"
    );
}

/// Wave C W676 / C-01: diamond path deps must re-check VersionReq on already-resolved packages.
#[test]
fn r001_diamond_semver_conflict_rejects_incompatible_transitive() {
    let root = tempfile::tempdir().unwrap();

    let shared = root.path().join("shared");
    std::fs::create_dir_all(&shared).unwrap();
    std::fs::write(
        shared.join("syx.toml"),
        "name = \"shared\"\nversion = \"1.0.0\"\nmain = \"shared.sy\"\n",
    )
    .unwrap();
    std::fs::write(shared.join("shared.sy"), "public func ok() -> Int { return 1 }").unwrap();

    let pkg_a = root.path().join("pkg_a");
    std::fs::create_dir_all(&pkg_a).unwrap();
    std::fs::write(
        pkg_a.join("syx.toml"),
        r#"name = "pkg_a"
version = "1.0.0"
main = "a.sy"

[dependencies.shared]
version = "^1.0.0"
path = "../shared"
"#,
    )
    .unwrap();
    std::fs::write(pkg_a.join("a.sy"), "public func a() -> Int { return 1 }").unwrap();

    let pkg_b = root.path().join("pkg_b");
    std::fs::create_dir_all(&pkg_b).unwrap();
    std::fs::write(
        pkg_b.join("syx.toml"),
        r#"name = "pkg_b"
version = "1.0.0"
main = "b.sy"

[dependencies.shared]
version = "^2.0.0"
path = "../shared"
"#,
    )
    .unwrap();
    std::fs::write(pkg_b.join("b.sy"), "public func b() -> Int { return 2 }").unwrap();

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        r#"name = "app"
version = "0.1.0"
main = "main.sy"

[dependencies.pkg_a]
version = "^1.0.0"
path = "../pkg_a"

[dependencies.pkg_b]
version = "^1.0.0"
path = "../pkg_b"
"#,
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "func main() { return 0 }").unwrap();

    let err = install_project(&app).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("R001"),
        "expected R001 diamond transitive conflict, got: {msg}"
    );
}

/// Compatible second requirement on an already-resolved path dep still succeeds.
#[test]
fn r001_diamond_compatible_transitive_still_ok() {
    let root = tempfile::tempdir().unwrap();

    let shared = root.path().join("shared");
    std::fs::create_dir_all(&shared).unwrap();
    std::fs::write(
        shared.join("syx.toml"),
        "name = \"shared\"\nversion = \"1.5.0\"\nmain = \"shared.sy\"\n",
    )
    .unwrap();
    std::fs::write(shared.join("shared.sy"), "public func ok() -> Int { return 1 }").unwrap();

    let pkg_a = root.path().join("pkg_a");
    std::fs::create_dir_all(&pkg_a).unwrap();
    std::fs::write(
        pkg_a.join("syx.toml"),
        r#"name = "pkg_a"
version = "1.0.0"
main = "a.sy"

[dependencies.shared]
version = "^1.0.0"
path = "../shared"
"#,
    )
    .unwrap();
    std::fs::write(pkg_a.join("a.sy"), "public func a() -> Int { return 1 }").unwrap();

    let pkg_b = root.path().join("pkg_b");
    std::fs::create_dir_all(&pkg_b).unwrap();
    std::fs::write(
        pkg_b.join("syx.toml"),
        r#"name = "pkg_b"
version = "1.0.0"
main = "b.sy"

[dependencies.shared]
version = ">=1.5.0, <2.0.0"
path = "../shared"
"#,
    )
    .unwrap();
    std::fs::write(pkg_b.join("b.sy"), "public func b() -> Int { return 2 }").unwrap();

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        r#"name = "app"
version = "0.1.0"
main = "main.sy"

[dependencies.pkg_a]
version = "^1.0.0"
path = "../pkg_a"

[dependencies.pkg_b]
version = "^1.0.0"
path = "../pkg_b"
"#,
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "func main() { return 0 }").unwrap();

    install_project(&app).expect("compatible diamond transitive must install");
}

/// Wave C W677 / C-01: registry diamond must re-check VersionReq (same resolver path as path deps).
#[test]
fn r001_diamond_registry_semver_conflict_rejects_incompatible_transitive() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let file_registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );

    let shared = root.path().join("shared");
    std::fs::create_dir_all(&shared).unwrap();
    std::fs::write(
        shared.join("syx.toml"),
        "name = \"shared\"\nversion = \"1.0.0\"\nmain = \"shared.sy\"\n",
    )
    .unwrap();
    std::fs::write(shared.join("shared.sy"), "public func ok() -> Int { return 1 }").unwrap();
    publish_signed(&shared, &file_registry_url);

    let pkg_a = root.path().join("pkg_a");
    std::fs::create_dir_all(&pkg_a).unwrap();
    std::fs::write(
        pkg_a.join("syx.toml"),
        r#"name = "pkg_a"
version = "1.0.0"
main = "a.sy"

[dependencies.shared]
version = "^1.0.0"
"#,
    )
    .unwrap();
    std::fs::write(pkg_a.join("a.sy"), "public func a() -> Int { return 1 }").unwrap();
    publish_signed(&pkg_a, &file_registry_url);

    let pkg_b = root.path().join("pkg_b");
    std::fs::create_dir_all(&pkg_b).unwrap();
    std::fs::write(
        pkg_b.join("syx.toml"),
        r#"name = "pkg_b"
version = "1.0.0"
main = "b.sy"

[dependencies.shared]
version = "^2.0.0"
"#,
    )
    .unwrap();
    std::fs::write(pkg_b.join("b.sy"), "public func b() -> Int { return 2 }").unwrap();
    publish_signed(&pkg_b, &file_registry_url);

    let http_url = start_mock_registry_http(&registry_dir);
    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "app"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{http_url}"

[dependencies.pkg_a]
version = "1.0.0"

[dependencies.pkg_b]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "func main() { return 0 }").unwrap();

    let err = install_project(&app).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("R001"),
        "expected R001 registry diamond conflict, got: {msg}"
    );
    unpin_test_trust_root();
}

#[test]
fn r001_diamond_registry_compatible_transitive_still_ok() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let file_registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );

    let shared = root.path().join("shared");
    std::fs::create_dir_all(&shared).unwrap();
    std::fs::write(
        shared.join("syx.toml"),
        "name = \"shared\"\nversion = \"1.5.0\"\nmain = \"shared.sy\"\n",
    )
    .unwrap();
    std::fs::write(shared.join("shared.sy"), "public func ok() -> Int { return 1 }").unwrap();
    publish_signed(&shared, &file_registry_url);

    let pkg_a = root.path().join("pkg_a");
    std::fs::create_dir_all(&pkg_a).unwrap();
    std::fs::write(
        pkg_a.join("syx.toml"),
        r#"name = "pkg_a"
version = "1.0.0"
main = "a.sy"

[dependencies.shared]
version = "^1.0.0"
"#,
    )
    .unwrap();
    std::fs::write(pkg_a.join("a.sy"), "public func a() -> Int { return 1 }").unwrap();
    publish_signed(&pkg_a, &file_registry_url);

    let pkg_b = root.path().join("pkg_b");
    std::fs::create_dir_all(&pkg_b).unwrap();
    std::fs::write(
        pkg_b.join("syx.toml"),
        r#"name = "pkg_b"
version = "1.0.0"
main = "b.sy"

[dependencies.shared]
version = ">=1.5.0, <2.0.0"
"#,
    )
    .unwrap();
    std::fs::write(pkg_b.join("b.sy"), "public func b() -> Int { return 2 }").unwrap();
    publish_signed(&pkg_b, &file_registry_url);

    let http_url = start_mock_registry_http(&registry_dir);
    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "app"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{http_url}"

[dependencies.pkg_a]
version = "1.0.0"

[dependencies.pkg_b]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "func main() { return 0 }").unwrap();

    install_project(&app).expect("compatible registry diamond must install");
    unpin_test_trust_root();
}

#[test]
fn syx_registry_prod_url_documented_default() {
    use syx_pkg::{default_registry_url, SyxManifest};
    let manifest = SyxManifest::default();
    let url = default_registry_url(&manifest);
    assert_eq!(url, "https://packages.sy.dev");
    std::env::set_var("SYX_REGISTRY", "https://registry.example.com");
    let url2 = default_registry_url(&manifest);
    assert_eq!(url2, "https://registry.example.com");
    std::env::remove_var("SYX_REGISTRY");
}

#[test]
fn registry_prod_url_smoke() {
    use syx_pkg::{default_registry_url, SyxManifest};
    let manifest = SyxManifest::default();
    let url = default_registry_url(&manifest);
    assert!(url.starts_with("https://"));
    std::env::set_var("SYX_REGISTRY", "https://packages.sy.dev");
    assert_eq!(default_registry_url(&manifest), "https://packages.sy.dev");
    std::env::remove_var("SYX_REGISTRY");
}

#[test]
fn registry_mirror_env_used_when_no_manifest_default() {
    use syx_pkg::{default_registry_url, SyxManifest};
    let manifest = SyxManifest::default();
    std::env::remove_var("SYX_REGISTRY");
    std::env::set_var("SYX_REGISTRY_MIRROR", "https://mirror.example.com");
    let url = default_registry_url(&manifest);
    assert_eq!(url, "https://mirror.example.com");
    std::env::remove_var("SYX_REGISTRY_MIRROR");
}

#[test]
fn registry_cdn_failover_smoke() {
    use syx_pkg::{registry_url_chain, SyxManifest};
    let mut manifest = SyxManifest::default();
    manifest.registry.default = Some("https://primary.invalid".into());
    std::env::set_var("SYX_REGISTRY_STAGING", "https://staging.invalid");
    std::env::set_var("SYX_REGISTRY_CDN", "https://cdn.invalid");
    std::env::set_var("SYX_REGISTRY_MIRROR", "https://mirror.invalid");
    let chain = registry_url_chain(&manifest);
    assert!(chain.first().map(String::as_str) == Some("https://primary.invalid"));
    assert!(chain.iter().any(|u| u.contains("staging")));
    assert!(chain.iter().any(|u| u.contains("cdn")));
    assert!(chain.iter().any(|u| u.contains("mirror")));
    std::env::remove_var("SYX_REGISTRY_STAGING");
    std::env::remove_var("SYX_REGISTRY_CDN");
    std::env::remove_var("SYX_REGISTRY_MIRROR");
}

#[test]
fn registry_promote_staging_smoke() {
    use syx_pkg::{registry_url_chain, SyxManifest};
    let manifest = SyxManifest::default();
    std::env::set_var("SYX_REGISTRY_STAGING", "https://staging.example");
    std::env::set_var("SYX_REGISTRY_PROMOTE_STAGING", "1");
    let chain = registry_url_chain(&manifest);
    assert_eq!(chain.first().map(String::as_str), Some("https://staging.example"));
    std::env::remove_var("SYX_REGISTRY_STAGING");
    std::env::remove_var("SYX_REGISTRY_PROMOTE_STAGING");
}

#[test]
fn registry_cdn_publish_install_chain_e2e() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let staging_dir = root.path().join("staging-registry");
    let cdn_dir = root.path().join("cdn-registry");
    std::fs::create_dir_all(&staging_dir).unwrap();
    std::fs::create_dir_all(&cdn_dir).unwrap();
    let staging_url = format!(
        "file://{}",
        staging_dir.canonicalize().unwrap().to_string_lossy()
    );
    let cdn_url = format!(
        "file://{}",
        cdn_dir.canonicalize().unwrap().to_string_lossy()
    );
    std::env::set_var("SYX_REGISTRY_STAGING", &staging_url);
    std::env::set_var("SYX_REGISTRY_CDN", &cdn_url);

    let pkg = root.path().join("chain-pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"chaindemo\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 15 }").unwrap();
    publish_signed(&pkg, &staging_url);

    let app = root.path().join("consumer");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        "name = \"consumer\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.chaindemo]\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    std::fs::write(
        app.join("main.sy"),
        "import chaindemo\nfunc main() { print(chaindemo.ping()) }",
    )
    .unwrap();
    install_project(&app).expect("three-hop registry chain install");
    std::env::remove_var("SYX_REGISTRY_STAGING");
    std::env::remove_var("SYX_REGISTRY_CDN");
    unpin_test_trust_root();
}

#[test]
fn registry_signed_publish_smoke() {
    pin_test_trust_root();
    use syx_pkg::verify_package_signature;

    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("signed-registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );

    let pkg = root.path().join("signed-pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"signedemo\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 42 }").unwrap();
    publish_signed(&pkg, &registry_url);

    let checksum = "deadbeefcafebabe";
    std::env::set_var("SYX_REGISTRY_ALLOW_UNSIGNED", "1");
    assert!(verify_package_signature(checksum, Some("syx1:stub")));
    assert!(verify_package_signature(checksum, None));
    std::env::remove_var("SYX_REGISTRY_ALLOW_UNSIGNED");

    let app = root.path().join("consumer");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"signed-consumer\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies]\nsignedemo = {{ version = \"1.0.0\", registry = \"{registry_url}\" }}\n"
        ),
    )
    .unwrap();
    std::fs::write(
        app.join("main.sy"),
        "import signedemo\nfunc main() { print(signedemo.ping()) }",
    )
    .unwrap();
    install_project(&app).expect("signed registry install");
    unpin_test_trust_root();
}

#[test]
fn p20_registry_ed25519_staging_prod_dual_url() {
    pin_test_trust_root();
    use syx_pkg::{sign_package_checksum, verify_package_signature};

    let root = tempfile::tempdir().unwrap();
    let staging_dir = root.path().join("staging-registry");
    let prod_dir = root.path().join("prod-registry");
    std::fs::create_dir_all(&staging_dir).unwrap();
    std::fs::create_dir_all(&prod_dir).unwrap();
    let staging_url = format!(
        "file://{}",
        staging_dir.canonicalize().unwrap().to_string_lossy()
    );
    let prod_url = format!(
        "file://{}",
        prod_dir.canonicalize().unwrap().to_string_lossy()
    );
    std::env::set_var("SYX_REGISTRY_STAGING", &staging_url);
    std::env::set_var("SYX_REGISTRY", &prod_url);
    std::env::set_var("SYX_REGISTRY_PROMOTE_STAGING", "1");

    let checksum = "cafebabedeadbeef0123456789abcdef";

    let pkg = root.path().join("ed-pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"eddemo\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 21 }").unwrap();
    publish_signed(&pkg, &staging_url);

    let sig = sign_package_checksum(checksum, &test_signing_key());
    assert!(verify_package_signature(checksum, Some(&sig)));
    assert!(!verify_package_signature("wrongchecksum000", Some(&sig)));

    let app = root.path().join("consumer");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"ed-consumer\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies]\neddemo = {{ version = \"1.0.0\", registry = \"{staging_url}\" }}\n"
        ),
    )
    .unwrap();
    std::fs::write(
        app.join("main.sy"),
        "import eddemo\nfunc main() { print(eddemo.ping()) }",
    )
    .unwrap();
    install_project(&app).expect("staging→prod dual URL install");
    std::env::remove_var("SYX_REGISTRY_STAGING");
    std::env::remove_var("SYX_REGISTRY");
    std::env::remove_var("SYX_REGISTRY_PROMOTE_STAGING");
    unpin_test_trust_root();
}

#[test]
fn c02_trust_root_rejects_embedded_key_without_pin() {
    use syx_pkg::{sign_package_checksum, verify_package_signature};

    unpin_test_trust_root();
    let checksum = "deadbeefcafebabe0123456789abcdef";
    let sig = sign_package_checksum(checksum, &test_signing_key());
    assert!(
        !verify_package_signature(checksum, Some(&sig)),
        "embedded self-sign must fail without VERIFY_KEY pin"
    );

    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );
    let pkg = root.path().join("nopin-pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"nopin\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 1 }").unwrap();
    publish_signed(&pkg, &registry_url);

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.nopin]\nversion = \"1.0.0\"\nregistry = \"{registry_url}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "import nopin\nfunc main() {}").unwrap();
    unpin_test_trust_root();
    let err = install_project(&app).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("P011"), "got: {msg}");
}

#[test]
fn c02_trust_root_rejects_mismatched_pin() {
    use syx_pkg::{sign_package_checksum, verify_package_signature};

    let checksum = "deadbeefcafebabe0123456789abcdef";
    let sig = sign_package_checksum(checksum, &test_signing_key());
    let other = SigningKey::from_bytes(&[0x99u8; 32]);
    std::env::set_var("SYX_REGISTRY_VERIFY_KEY", verifying_key_hex(&other));
    assert!(
        !verify_package_signature(checksum, Some(&sig)),
        "mismatched pin must reject embedded key"
    );

    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );
    let pkg = root.path().join("badpin-pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"badpin\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 2 }").unwrap();
    publish_signed(&pkg, &registry_url);

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.badpin]\nversion = \"1.0.0\"\nregistry = \"{registry_url}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "import badpin\nfunc main() {}").unwrap();
    std::env::set_var("SYX_REGISTRY_VERIFY_KEY", verifying_key_hex(&other));
    let err = install_project(&app).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("P011"), "got: {msg}");
    unpin_test_trust_root();
}

#[test]
fn c02_trust_root_accepts_matching_pin() {
    use syx_pkg::{sign_package_checksum, verify_package_signature};

    pin_test_trust_root();
    let checksum = "deadbeefcafebabe0123456789abcdef";
    let sig = sign_package_checksum(checksum, &test_signing_key());
    assert!(verify_package_signature(checksum, Some(&sig)));
    assert!(!verify_package_signature("wrongchecksum000", Some(&sig)));

    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );
    let pkg = root.path().join("okpin-pkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"okpin\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 3 }").unwrap();
    publish_signed(&pkg, &registry_url);

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.okpin]\nversion = \"1.0.0\"\nregistry = \"{registry_url}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(
        app.join("main.sy"),
        "import okpin\nfunc main() { print(okpin.ping()) }",
    )
    .unwrap();
    install_project(&app).expect("matching trust-root pin must install");
    unpin_test_trust_root();
}

fn sig_only_from_embedded(embedded: &str) -> String {
    let rest = embedded
        .strip_prefix("syx1:ed25519:")
        .expect("ed25519 prefix");
    let (_pk, sig) = rest.split_once(':').expect("pk:sig");
    format!("syx1:ed25519:{sig}")
}

#[test]
fn c02_trust_root_sig_only_requires_pin() {
    use syx_pkg::{sign_package_checksum, verify_package_signature};

    let checksum = "deadbeefcafebabe0123456789abcdef";
    let embedded = sign_package_checksum(checksum, &test_signing_key());
    let sig_only = sig_only_from_embedded(&embedded);

    unpin_test_trust_root();
    assert!(
        !verify_package_signature(checksum, Some(&sig_only)),
        "sig-only must fail without VERIFY_KEY pin"
    );

    let other = SigningKey::from_bytes(&[0x99u8; 32]);
    std::env::set_var("SYX_REGISTRY_VERIFY_KEY", verifying_key_hex(&other));
    assert!(
        !verify_package_signature(checksum, Some(&sig_only)),
        "sig-only must fail with mismatched pin"
    );
    unpin_test_trust_root();
}

#[test]
fn c02_trust_root_sig_only_accepts_matching_pin() {
    use syx_pkg::{sign_package_checksum, verify_package_signature};

    pin_test_trust_root();
    let checksum = "deadbeefcafebabe0123456789abcdef";
    let embedded = sign_package_checksum(checksum, &test_signing_key());
    let sig_only = sig_only_from_embedded(&embedded);
    assert!(verify_package_signature(checksum, Some(&sig_only)));
    assert!(!verify_package_signature("wrongchecksum000", Some(&sig_only)));
    unpin_test_trust_root();
}

#[test]
fn c02_trust_root_pin_hex_case_insensitive() {
    use syx_pkg::{sign_package_checksum, verify_package_signature};

    let checksum = "deadbeefcafebabe0123456789abcdef";
    let sig = sign_package_checksum(checksum, &test_signing_key());
    let upper = verifying_key_hex(&test_signing_key()).to_ascii_uppercase();
    std::env::set_var("SYX_REGISTRY_VERIFY_KEY", &upper);
    assert!(
        verify_package_signature(checksum, Some(&sig)),
        "uppercase pin must accept lowercase embedded pk"
    );
    unpin_test_trust_root();
}

#[test]
fn c04_locked_install_rejects_missing_lock() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        r#"name = "app"
version = "0.1.0"
main = "main.sy"

[dependencies.lockdemo]
version = "1.0.0"
registry = "file:///tmp/syx-missing-registry-unused"
"#,
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "func main() {}").unwrap();
    assert!(!app.join("syx.lock").exists());
    std::env::set_var("SYX_INSTALL_LOCKED", "1");
    let err = install_project(&app).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("P014"), "got: {msg}");
    std::env::remove_var("SYX_INSTALL_LOCKED");
    unpin_test_trust_root();
}

#[test]
fn c04_locked_install_rejects_checksum_mismatch() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );

    let pkg = root.path().join("lockpkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"lockpkg\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 4 }").unwrap();
    publish_signed(&pkg, &registry_url);

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.lockpkg]\nversion = \"1.0.0\"\nregistry = \"{registry_url}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "import lockpkg\nfunc main() {}").unwrap();
    install_project(&app).expect("seed install");

    let lock_path = app.join("syx.lock");
    let lock_text = std::fs::read_to_string(&lock_path).unwrap();
    let old_checksum = lock_text
        .lines()
        .find_map(|l| {
            let t = l.trim();
            if t.starts_with("checksum") {
                t.split('=')
                    .nth(1)
                    .map(|v| v.trim().trim_matches('"').to_string())
            } else {
                None
            }
        })
        .expect("checksum field in syx.lock");
    let mutated = lock_text.replace(
        &old_checksum,
        "sha256:0000000000000000000000000000000000000000000000000000000000000000",
    );
    std::fs::write(&lock_path, &mutated).unwrap();

    std::env::set_var("SYX_INSTALL_LOCKED", "1");
    let err = install_project(&app).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("P003"), "got: {msg}");
    std::env::remove_var("SYX_INSTALL_LOCKED");
    unpin_test_trust_root();
}

#[test]
fn c04_locked_install_reuses_pin() {
    use syx_pkg::{load_lockfile, PackageSource};

    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );

    let pkg = root.path().join("reusepkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"reusepkg\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 5 }").unwrap();
    publish_signed(&pkg, &registry_url);

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.reusepkg]\nversion = \"1.0.0\"\nregistry = \"{registry_url}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(
        app.join("main.sy"),
        "import reusepkg\nfunc main() { print(reusepkg.ping()) }",
    )
    .unwrap();
    install_project(&app).expect("seed install");
    let before = load_lockfile(&app).unwrap();
    let (ver_before, sum_before) = match &before.packages["reusepkg"].source {
        PackageSource::Registry {
            version, checksum, ..
        } => (version.clone(), checksum.clone()),
        other => panic!("expected registry pin, got {other:?}"),
    };

    std::env::set_var("SYX_INSTALL_LOCKED", "1");
    install_project(&app).expect("locked reinstall");
    std::env::remove_var("SYX_INSTALL_LOCKED");

    let after = load_lockfile(&app).unwrap();
    match &after.packages["reusepkg"].source {
        PackageSource::Registry {
            version, checksum, ..
        } => {
            assert_eq!(version, &ver_before);
            assert_eq!(checksum, &sum_before);
        }
        other => panic!("expected registry pin, got {other:?}"),
    }
    unpin_test_trust_root();
}

#[test]
fn c04_locked_install_does_not_rewrite_lock() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );

    let pkg = root.path().join("stablepkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"stablepkg\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 6 }").unwrap();
    publish_signed(&pkg, &registry_url);

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.stablepkg]\nversion = \"1.0.0\"\nregistry = \"{registry_url}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "import stablepkg\nfunc main() {}").unwrap();
    install_project(&app).expect("seed install");

    let lock_path = app.join("syx.lock");
    let before_bytes = std::fs::read(&lock_path).unwrap();

    std::env::set_var("SYX_INSTALL_LOCKED", "1");
    install_project(&app).expect("locked reinstall must keep lock bytes");
    std::env::remove_var("SYX_INSTALL_LOCKED");

    let after_bytes = std::fs::read(&lock_path).unwrap();
    assert_eq!(
        before_bytes, after_bytes,
        "SYX_INSTALL_LOCKED must not rewrite syx.lock"
    );
    unpin_test_trust_root();
}

#[test]
fn c04_locked_install_rejects_missing_package() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );

    let pkg = root.path().join("onlypkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"onlypkg\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 7 }").unwrap();
    publish_signed(&pkg, &registry_url);

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.onlypkg]\nversion = \"1.0.0\"\nregistry = \"{registry_url}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "import onlypkg\nfunc main() {}").unwrap();
    install_project(&app).expect("seed install");

    // Manifest now asks for a registry dep that is absent from syx.lock.
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.missingpkg]\nversion = \"1.0.0\"\nregistry = \"{registry_url}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "func main() {}").unwrap();

    std::env::set_var("SYX_INSTALL_LOCKED", "1");
    let err = install_project(&app).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("P014"), "got: {msg}");
    std::env::remove_var("SYX_INSTALL_LOCKED");
    unpin_test_trust_root();
}

#[test]
fn c04_locked_install_rejects_non_registry_pin() {
    pin_test_trust_root();
    let root = tempfile::tempdir().unwrap();
    let registry_dir = root.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    let registry_url = format!(
        "file://{}",
        registry_dir.canonicalize().unwrap().to_string_lossy()
    );

    let pkg = root.path().join("mixpkg");
    std::fs::create_dir_all(&pkg).unwrap();
    std::fs::write(
        pkg.join("syx.toml"),
        "name = \"mixpkg\"\nversion = \"1.0.0\"\nmain = \"lib.sy\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("lib.sy"), "public func ping() -> Int { return 8 }").unwrap();
    publish_signed(&pkg, &registry_url);

    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("syx.toml"),
        format!(
            "name = \"app\"\nversion = \"0.1.0\"\nmain = \"main.sy\"\n\n[dependencies.mixpkg]\nversion = \"1.0.0\"\nregistry = \"{registry_url}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(app.join("main.sy"), "import mixpkg\nfunc main() {}").unwrap();
    install_project(&app).expect("seed install");

    // Corrupt the lock pin into a path source for the same package name.
    let lock_path = app.join("syx.lock");
    let lock_text = std::fs::read_to_string(&lock_path).unwrap();
    let path_pin = r#"source = "path"
version = "1.0.0"
path = "../mixpkg"
"#;
    // Replace the registry source block under [packages.mixpkg] with a path pin.
    let mutated = {
        let marker = "[packages.mixpkg]";
        let Some(start) = lock_text.find(marker) else {
            panic!("packages.mixpkg missing");
        };
        let after = &lock_text[start + marker.len()..];
        let next = after.find("\n[").map(|i| start + marker.len() + i);
        let end = next.unwrap_or(lock_text.len());
        format!(
            "{}{}\n{}{}",
            &lock_text[..start + marker.len()],
            "\n",
            path_pin,
            &lock_text[end..]
        )
    };
    std::fs::write(&lock_path, &mutated).unwrap();

    std::env::set_var("SYX_INSTALL_LOCKED", "1");
    let err = install_project(&app).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("P014"), "got: {msg}");
    std::env::remove_var("SYX_INSTALL_LOCKED");
    unpin_test_trust_root();
}
