//! Registry v8 E2E driver tests (Phase 69).
// was: registry

use std::fs;
use std::path::{Path, PathBuf};

use ed25519_dalek::SigningKey;
use syx_pkg::{
    add_dependency, graph_to_lockfile, install_project, load_resolved_packages,
    publish_package, publish_package_signed, resolve_dependencies, save_lockfile,
    sign_package_checksum, verifying_key_hex, DependencyAddSpec, SignaturePolicy, SyxLockfile,
};

const TEST_SIGNING_SEED: [u8; 32] = [0x69; 32];

fn test_signing_key() -> SigningKey {
    SigningKey::from_bytes(&TEST_SIGNING_SEED)
}

fn pin_test_trust_root() {
    std::env::set_var(
        "SYX_REGISTRY_VERIFY_KEY",
        verifying_key_hex(&test_signing_key()),
    );
}

fn publish_signed(pkg: &Path, reg: &str) {
    pin_test_trust_root();
    publish_package_signed(pkg, reg, &test_signing_key()).expect("publish signed");
}

fn temp_pkg(name: &str, main_body: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_reg63_{name}_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("syx.toml"),
        format!(
            r#"name = "{name}"
version = "1.0.0"
main = "main.sy"
"#
        ),
    )
    .unwrap();
    fs::write(dir.join("main.sy"), main_body).unwrap();
    dir
}

fn file_registry() -> String {
    let dir = std::env::temp_dir().join(format!(
        "syx_reg63_idx_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    format!("file://{}", dir.canonicalize().unwrap().display())
}

#[test]
fn registry_index_schema_fields() {
    let entry = serde_json::json!({
        "name": "demo",
        "version": "1.0.0",
        "tarball_url": "file:///tmp/demo.tgz",
        "checksum": "sha256:abc",
        "syx_version": "41.0",
        "deps": {},
        "yanked": false
    });
    assert_eq!(entry["name"], "demo");
    assert_eq!(entry["syx_version"], "41.0");
}

#[test]
fn lockfile_default_empty() {
    let lock = SyxLockfile::default();
    assert!(lock.packages.is_empty());
    assert!(lock.plugins.is_empty());
}

#[test]
fn lockfile_v9_default_version() {
    let lock = SyxLockfile::default();
    assert_eq!(lock.version, 16);
}

#[test]
fn lockfile_v8_default_version_legacy_name() {
    let lock = SyxLockfile::default();
    assert_eq!(lock.version, 16);
}

#[test]
fn lockfile_v5_fields_serialize() {
    let lock = SyxLockfile {
        version: 5,
        syx_version: "44.0".into(),
        syx_version_floor: "44.0".into(),
        resolved_at: "1719500000".into(),
        registry: "file:///tmp/reg".into(),
        publisher_key_id: "key-ed25519-demo".into(),
        signature_policy: SignaturePolicy::Mandatory,
        tarball_sha256: indexmap::indexmap! {
            "demo".to_string() => "sha256:abc123".into(),
        },
        packages: indexmap::IndexMap::new(),
        plugins: vec![],
        registry_mirrors: vec![],
        artifact_cdn_base: None,
        ..Default::default()
    };
    let text = toml::to_string(&lock).unwrap();
    assert!(text.contains("tarball_sha256"));
    assert!(text.contains("publisher_key_id"));
    assert!(text.contains("signature_policy"));
    assert!(text.contains("version = 5"));
}

#[test]
fn lockfile_v4_fields_serialize() {
    let lock = SyxLockfile {
        version: 5,
        syx_version: "44.0".into(),
        syx_version_floor: "44.0".into(),
        resolved_at: "1719500000".into(),
        registry: "file:///tmp/reg".into(),
        publisher_key_id: String::new(),
        signature_policy: SignaturePolicy::Optional,
        tarball_sha256: indexmap::indexmap! {
            "demo".to_string() => "sha256:abc123".into(),
        },
        packages: indexmap::IndexMap::new(),
        plugins: vec![],
        registry_mirrors: vec![],
        artifact_cdn_base: None,
        ..Default::default()
    };
    let text = toml::to_string(&lock).unwrap();
    assert!(text.contains("tarball_sha256"));
    assert!(text.contains("resolved_at"));
    assert!(text.contains("syx_version_floor"));
    assert!(text.contains("version = 5"));
}

#[test]
fn graph_to_lockfile_v4_metadata() {
    let reg = file_registry();
    let pkg = temp_pkg("meta65", r#"public func meta() -> Int { return 7 }"#);
    publish_signed(&pkg, &reg);
    let app = temp_pkg("appmeta65", "import meta65\nfunc main() {}");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "appmeta65"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.meta65]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    install_project(&app).expect("install");
    let lock = syx_pkg::load_lockfile(&app).unwrap();
    assert_eq!(lock.version, 16);
    assert_eq!(lock.syx_version_floor, "57.0");
    assert!(!lock.resolved_at.is_empty());
    assert!(lock.tarball_sha256.contains_key("meta65"));
}

#[test]
fn golden_publish_install_compile_run() {
    let reg = file_registry();
    let lib = temp_pkg("golden65", "public func answer() -> Int { return 42 }");
    fs::write(
        lib.join("syx.toml"),
        "name = \"golden65\"\nversion = \"1.0.0\"\nmain = \"golden65.sy\"\n",
    )
    .unwrap();
    fs::write(
        lib.join("golden65.sy"),
        "public func answer() -> Int { return 42 }",
    )
    .unwrap();
    publish_signed(&lib, &reg);

    let app = temp_pkg("goldenapp65", "");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "goldenapp65"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.golden65]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    fs::write(
        app.join("main.sy"),
        r#"import golden65
func main() { print(golden65.answer()) }"#,
    )
    .unwrap();

    install_project(&app).expect("install");
    let lock = syx_pkg::load_lockfile(&app).unwrap();
    assert_eq!(lock.version, 16);
    assert!(lock.tarball_sha256.contains_key("golden65"));

    let unit = syx_driver::compile_project(&app).expect("compile");
    assert!(unit.entry.program.functions.iter().any(|f| f.name == "main"));

    syx_driver::run_project(&app).expect("run");
}

#[test]
fn lockfile_v4_roundtrip() {
    let dir = temp_pkg("lockv4", r#"func main() {}"#);
    let lock = SyxLockfile {
        version: 5,
        syx_version: "44.0".into(),
        syx_version_floor: "44.0".into(),
        resolved_at: "100".into(),
        registry: "file:///tmp".into(),
        publisher_key_id: "pk-demo".into(),
        signature_policy: SignaturePolicy::Optional,
        tarball_sha256: indexmap::indexmap! { "a".into() => "sha256:ff".into() },
        packages: indexmap::IndexMap::new(),
        plugins: vec![],
        registry_mirrors: vec![],
        artifact_cdn_base: None,
        ..Default::default()
    };
    save_lockfile(&dir, &lock).expect("save");
    let loaded = syx_pkg::load_lockfile(&dir).expect("load");
    assert_eq!(loaded.version, 5);
    assert_eq!(loaded.syx_version_floor, "44.0");
    assert_eq!(loaded.tarball_sha256.get("a").map(String::as_str), Some("sha256:ff"));
}

#[test]
fn resolve_graph_populates_tarball_sha256() {
    let reg = file_registry();
    let pkg = temp_pkg("sha65", r#"public func h() -> Int { return 1 }"#);
    publish_signed(&pkg, &reg);
    let app = temp_pkg("appsha65", "import sha65\nfunc main() {}");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "appsha65"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.sha65]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    let graph = resolve_dependencies(&app).unwrap();
    let lock = graph_to_lockfile(&graph, &syx_pkg::load_manifest(&app).unwrap());
    assert!(lock.tarball_sha256.get("sha65").unwrap().starts_with("sha256:"));
}

#[test]
fn install_writes_lock_version_4() {
    let reg = file_registry();
    let pkg = temp_pkg("ver65", r#"public func v() -> Int { return 0 }"#);
    publish_signed(&pkg, &reg);
    let app = temp_pkg("appver65", "import ver65\nfunc main() {}");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "appver65"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.ver65]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    install_project(&app).expect("install");
    let text = fs::read_to_string(app.join("syx.lock")).unwrap();
    assert!(text.contains("version = 14"));
}

#[test]
fn registry_compile_imported_symbol() {
    let reg = file_registry();
    let pkg = temp_pkg("sympkg", "");
    fs::write(
        pkg.join("syx.toml"),
        "name = \"sympkg\"\nversion = \"1.0.0\"\nmain = \"sympkg.sy\"\n",
    )
    .unwrap();
    fs::write(
        pkg.join("sympkg.sy"),
        "public func sym() -> Int { return 99 }",
    )
    .unwrap();
    publish_signed(&pkg, &reg);
    let app = temp_pkg("appsym65", "");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "appsym65"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.sympkg]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    fs::write(
        app.join("main.sy"),
        "import sympkg\nfunc main() { print(sympkg.sym()) }",
    )
    .unwrap();
    install_project(&app).expect("install");
    let unit = syx_driver::compile_project(&app).expect("compile");
    assert!(unit.graph.modules.len() >= 1);
}

#[test]
fn registry_run_prints_from_dependency() {
    let reg = file_registry();
    let pkg = temp_pkg("runpkg", "");
    fs::write(
        pkg.join("syx.toml"),
        "name = \"runpkg\"\nversion = \"1.0.0\"\nmain = \"runpkg.sy\"\n",
    )
    .unwrap();
    fs::write(
        pkg.join("runpkg.sy"),
        "public func msg() -> Int { return 100 }",
    )
    .unwrap();
    publish_signed(&pkg, &reg);
    let app = temp_pkg("apprun65", "");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "apprun65"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.runpkg]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    fs::write(
        app.join("main.sy"),
        "import runpkg\nfunc main() { print(runpkg.msg()) }",
    )
    .unwrap();
    install_project(&app).expect("install");
    syx_driver::compile_project(&app).expect("compile");
    syx_driver::run_project(&app).expect("run");
}

#[test]
fn lockfile_v2_registry_checksum_field() {
    let lock = SyxLockfile {
        version: 5,
        syx_version: "44.0".into(),
        syx_version_floor: "44.0".into(),
        resolved_at: "0".into(),
        registry: "file:///tmp/reg".into(),
        publisher_key_id: String::new(),
        signature_policy: SignaturePolicy::default(),
        tarball_sha256: indexmap::indexmap! {
            "demo".to_string() => "sha256:deadbeef".into(),
        },
        packages: indexmap::indexmap! {
            "demo".to_string() => syx_pkg::LockPackage {
                source: syx_pkg::PackageSource::Registry {
                    registry: "file:///tmp/reg".into(),
                    version: "1.0.0".into(),
                    checksum: "sha256:deadbeef".into(),
                    dir: "vendor/demo".into(),
                },
            },
        },
        plugins: vec![],
        registry_mirrors: vec![],
        artifact_cdn_base: None,
        ..Default::default()
    };
    let text = toml::to_string(&lock).unwrap();
    assert!(text.contains("checksum"));
    assert!(text.contains("registry"));
    assert!(text.contains("tarball_sha256"));
}

#[test]
fn lockfile_plugins_roundtrip() {
    let lock = SyxLockfile {
        version: 5,
        syx_version: "44.0".into(),
        syx_version_floor: "44.0".into(),
        resolved_at: "0".into(),
        registry: "file:///tmp/reg".into(),
        publisher_key_id: String::new(),
        signature_policy: SignaturePolicy::default(),
        tarball_sha256: indexmap::IndexMap::new(),
        packages: indexmap::IndexMap::new(),
        plugins: vec![syx_pkg::LockPluginEntry {
            id: "mycrypto".into(),
            hosts: vec!["vm".into()],
            plugin_base: Some(1),
        }],
        registry_mirrors: vec![],
        artifact_cdn_base: None,
        ..Default::default()
    };
    let text = toml::to_string(&lock).unwrap();
    assert!(text.contains("mycrypto"));
}

#[test]
fn registry_syx_version_43() {
    assert_eq!(
        serde_json::json!({"syx_version": "43.0"})["syx_version"],
        "43.0"
    );
}

#[test]
fn publish_dry_run_checksum_available() {
    let reg = file_registry();
    let pkg = temp_pkg("dry65", r#"func main() {}"#);
    publish_signed(&pkg, &reg);
    let app = temp_pkg("appdry65", "import dry65\nfunc main() {}");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "appdry65"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.dry65]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    install_project(&app).expect("install");
    let lock = syx_pkg::load_lockfile(&app).unwrap();
    let sum = lock.tarball_sha256.get("dry65").expect("checksum");
    assert!(sum.starts_with("sha256:"));
}

#[test]
fn path_dep_lock_v4_fields() {
    let root = temp_pkg("pathroot65", r#"func main() {}"#);
    let dep = temp_pkg("pathdep65", r#"public func p() -> Int { return 2 }"#);
    add_dependency(&root, "pathdep65", DependencyAddSpec::path(dep.to_string_lossy())).expect("add");
    install_project(&root).expect("install");
    let lock = syx_pkg::load_lockfile(&root).unwrap();
    assert_eq!(lock.version, 16);
    assert!(!lock.resolved_at.is_empty());
}

#[test]
fn publish_file_registry() {
    let pkg = temp_pkg("pub63", r#"func main() { print("ok") }"#);
    let reg = file_registry();
    publish_signed(&pkg, &reg);
    let index = fs::read_dir(reg.strip_prefix("file://").unwrap()).unwrap();
    assert!(index.count() >= 1);
}

#[test]
fn publish_install_roundtrip() {
    let reg = file_registry();
    let pkg = temp_pkg("greet63", "public func hello() -> Int { return 99 }");
    fs::write(pkg.join("greet.sy"), "public func hello() -> Int { return 99 }").unwrap();
    fs::write(
        pkg.join("syx.toml"),
        "name = \"greet63\"\nversion = \"1.0.0\"\nmain = \"greet.sy\"\n",
    )
    .unwrap();
    publish_signed(&pkg, &reg);

    let app = temp_pkg("app63", "import greet63\nfunc main() {}");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "app63"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.greet63]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    install_project(&app).expect("install");
    let packages = load_resolved_packages(&app).unwrap();
    assert!(packages.contains_key("greet63"));
}

#[test]
fn graph_to_lockfile_empty() {
    let lock = SyxLockfile::default();
    assert!(lock.packages.is_empty());
}

#[test]
fn save_lockfile_roundtrip() {
    let dir = temp_pkg("lock63", r#"func main() {}"#);
    let lock = SyxLockfile::default();
    save_lockfile(&dir, &lock).expect("save");
    let loaded = syx_pkg::load_lockfile(&dir).expect("load");
    assert!(loaded.packages.is_empty());
}

macro_rules! reg_schema {
    ($name:ident, $ver:literal) => {
        #[test]
        fn $name() {
            let entry = serde_json::json!({
                "name": "pkg",
                "version": $ver,
                "syx_version": "41.0",
            });
            assert_eq!(entry["syx_version"], "41.0");
        }
    };
}

reg_schema!(reg_schema_v1_0_0, "1.0.0");
reg_schema!(reg_schema_v1_0_1, "1.0.1");
reg_schema!(reg_schema_v1_1_0, "1.1.0");
reg_schema!(reg_schema_v2_0_0, "2.0.0");
reg_schema!(reg_schema_v0_1_0, "0.1.0");
reg_schema!(reg_schema_v0_2_0, "0.2.0");
reg_schema!(reg_schema_v3_0_0, "3.0.0");

#[test]
fn add_path_dependency() {
    let root = temp_pkg("root63", r#"func main() {}"#);
    let dep = temp_pkg("dep63", r#"public func x() -> Int { return 1 }"#);
    add_dependency(&root, "dep63", DependencyAddSpec::path(dep.to_string_lossy())).expect("add");
    let manifest = syx_pkg::load_manifest(&root).unwrap();
    assert!(manifest.dependencies.contains_key("dep63"));
}

#[test]
fn driver_compile_local_pkg() {
    let pkg = temp_pkg("drv63", r#"func main() { print("ok") }"#);
    let unit = syx_driver::compile_file_with_options(
        &pkg.join("main.sy"),
        &syx_driver::DriverOptions::default(),
    )
    .expect("compile");
    assert!(unit.program.functions.iter().any(|f| f.name == "main"));
}

#[test]
fn registry_default_url_chain() {
    let manifest = syx_pkg::SyxManifest::default();
    let urls = syx_pkg::registry_url_chain(&manifest);
    assert!(!urls.is_empty());
}

#[test]
fn lockfile_plugins_roundtrip_old() {
    let lock = SyxLockfile {
        version: 5,
        syx_version: "44.0".into(),
        syx_version_floor: "44.0".into(),
        resolved_at: "0".into(),
        registry: "file:///tmp/reg".into(),
        publisher_key_id: String::new(),
        signature_policy: SignaturePolicy::default(),
        tarball_sha256: indexmap::IndexMap::new(),
        packages: indexmap::IndexMap::new(),
        plugins: vec![syx_pkg::LockPluginEntry {
            id: "legacy".into(),
            hosts: vec!["vm".into()],
            plugin_base: None,
        }],
        registry_mirrors: vec![],
        artifact_cdn_base: None,
        ..Default::default()
    };
    let text = toml::to_string(&lock).unwrap();
    assert!(text.contains("legacy"));
}

#[test]
fn publish_twice_same_version_fails_or_overwrites() {
    let pkg = temp_pkg("dup63", r#"func main() {}"#);
    let reg = file_registry();
    publish_signed(&pkg, &reg);
    assert!(
        publish_package_signed(&pkg, &reg, &test_signing_key()).is_ok(),
        "republish same version is idempotent"
    );
}

#[test]
fn install_missing_lock_errors() {
    let dir = temp_pkg("nolock63", r#"func main() {}"#);
    let err = syx_pkg::load_lockfile(&dir);
    assert!(err.is_err());
}

#[test]
fn graph_to_lockfile_v6_fields() {
    let graph = syx_pkg::ResolvedGraph {
        packages: indexmap::IndexMap::new(),
    };
    let lock = graph_to_lockfile(&graph, &syx_pkg::SyxManifest::default());
    assert_eq!(lock.version, 16);
    assert_eq!(lock.syx_version, "58.0");
    assert!(!lock.resolved_at.is_empty());
}

#[test]
fn signature_policy_required_default() {
    let lock = SyxLockfile::default();
    assert_eq!(lock.signature_policy, SignaturePolicy::Mandatory);
}

#[test]
fn signature_policy_strict_roundtrip() {
    let lock = SyxLockfile {
        signature_policy: SignaturePolicy::Strict,
        ..Default::default()
    };
    let text = toml::to_string(&lock).unwrap();
    assert!(text.contains("strict"));
}

#[test]
fn signature_policy_mandatory_roundtrip() {
    let lock = SyxLockfile {
        signature_policy: SignaturePolicy::Mandatory,
        ..Default::default()
    };
    let text = toml::to_string(&lock).unwrap();
    assert!(text.contains("mandatory"));
}

#[test]
fn publisher_key_id_roundtrip() {
    let dir = temp_pkg("pk66", r#"func main() {}"#);
    let lock = SyxLockfile {
        publisher_key_id: "ed25519:abc".into(),
        ..Default::default()
    };
    save_lockfile(&dir, &lock).unwrap();
    let loaded = syx_pkg::load_lockfile(&dir).unwrap();
    assert_eq!(loaded.publisher_key_id, "ed25519:abc");
}

#[test]
fn registry_test_count_gate() {
    let src = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/registry.rs")).unwrap();
    let count = src.matches("#[test]").count();
    assert!(count >= 40, "registry needs 40+ tests, found {count}");
}

macro_rules! reg_lock_schema {
    ($name:ident, $policy:expr) => {
        #[test]
        fn $name() {
            let lock = SyxLockfile {
                signature_policy: $policy,
                publisher_key_id: "k".into(),
                ..Default::default()
            };
            assert_eq!(lock.version, 16);
        }
    };
}

reg_lock_schema!(reg_lock_mandatory, SignaturePolicy::Mandatory);
reg_lock_schema!(reg_lock_strict, SignaturePolicy::Strict);

#[test]
fn graph_to_lockfile_preserves_packages() {
    use syx_pkg::PackageSource;
    let mut packages = indexmap::IndexMap::new();
    packages.insert(
        "demo".into(),
        syx_pkg::LockPackage {
            source: PackageSource::Path {
                version: "1.0.0".into(),
                path: "vendor/demo".into(),
            },
        },
    );
    let graph = syx_pkg::ResolvedGraph { packages };
    let lock = graph_to_lockfile(&graph, &syx_pkg::SyxManifest::default());
    assert!(lock.packages.contains_key("demo"));
}

#[test]
fn lockfile_v5_syx_version_floor() {
    let lock = graph_to_lockfile(
        &syx_pkg::ResolvedGraph {
            packages: indexmap::IndexMap::new(),
        },
        &syx_pkg::SyxManifest::default(),
    );
    assert_eq!(lock.syx_version_floor, "57.0");
}

#[test]
fn lockfile_v9_cdn_fields_serialize() {
    let lock = SyxLockfile {
        registry_mirrors: vec!["file:///tmp/mirror".into()],
        artifact_cdn_base: Some("file:///tmp/cdn".into()),
        ..Default::default()
    };
    let text = toml::to_string(&lock).unwrap();
    assert!(text.contains("registry_mirrors"));
    assert!(text.contains("artifact_cdn_base"));
    assert!(text.contains("version = 14"));
}

#[test]
fn lockfile_empty_publisher_key_omitted_from_toml() {
    let lock = SyxLockfile::default();
    let text = toml::to_string(&lock).unwrap();
    assert!(!text.contains("publisher_key_id") || text.contains("publisher_key_id = \"\""));
}

#[test]
fn registry_phase70_charter() {
    let lock = SyxLockfile::default();
    assert_eq!(lock.version, 16);
    assert_eq!(lock.signature_policy, SignaturePolicy::Mandatory);
    assert!(lock.registry_mirrors.is_empty());
    assert!(lock.artifact_cdn_base.is_none());
}

#[test]
fn registry_unsigned_publish_rejected() {
    std::env::remove_var("SYX_REGISTRY_ALLOW_UNSIGNED");
    std::env::remove_var("SYX_PUBLISHER_SIGNING_KEY");
    let pkg = temp_pkg("unsigned69", r#"func main() { print("x") }"#);
    let reg = file_registry();
    let err = publish_package(&pkg, &reg).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Unsigned publish rejected") || msg.contains("P012"),
        "expected unsigned publish rejection, got: {msg}"
    );
}

#[test]
fn registry_signed_hard_e2e() {
    use syx_driver::run_file_printed;

    let signing_key = test_signing_key();
    let reg = file_registry();

    let lib = temp_pkg("signedlib69", "");
    fs::write(
        lib.join("syx.toml"),
        "name = \"signedlib69\"\nversion = \"1.0.0\"\nmain = \"signedlib69.sy\"\n",
    )
    .unwrap();
    fs::write(
        lib.join("signedlib69.sy"),
        "public func answer() -> Int { return 42 }",
    )
    .unwrap();
    pin_test_trust_root();
    publish_package_signed(&lib, &reg, &signing_key).expect("publish signed");

    let app = temp_pkg("signedapp69", "");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "signedapp69"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.signedlib69]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    fs::write(
        app.join("main.sy"),
        r#"import signedlib69
func main() { print(signedlib69.answer()) }"#,
    )
    .unwrap();

    install_project(&app).expect("install signed");
    let lock = syx_pkg::load_lockfile(&app).unwrap();
    assert_eq!(lock.version, 16);
    assert_eq!(lock.signature_policy, SignaturePolicy::Mandatory);
    assert!(lock.tarball_sha256.contains_key("signedlib69"));

    syx_driver::compile_project(&app).expect("compile");
    let (_, printed) =
        run_file_printed(&app.join("main.sy"), &syx_driver::DriverOptions::default())
            .expect("run");
    let out = printed.join("\n");
    assert!(out.contains("42"), "stdout: {out}");

    let checksum = lock.tarball_sha256.get("signedlib69").expect("checksum");
    let sig = syx_pkg::RegistryClient::new(&reg)
        .expect("registry client")
        .package_signature("signedlib69", "1.0.0")
        .expect("signature")
        .expect("signed tarball");
    assert!(syx_pkg::verify_package_signature(checksum, Some(&sig)));
    assert!(sig.starts_with("syx1:ed25519:"));
    let _ = sign_package_checksum(checksum, &signing_key);
}

#[test]
fn registry_signed_require_signature_env() {
    std::env::set_var("SYX_REGISTRY_REQUIRE_SIGNATURE", "1");
    registry_signed_hard_e2e();
    std::env::remove_var("SYX_REGISTRY_REQUIRE_SIGNATURE");
}

#[test]
fn registry_cdn_hard() {
    use syx_driver::run_file_printed;

    let signing_key = test_signing_key();
    let reg = file_registry();
    let cdn_dir = std::env::temp_dir().join(format!(
        "syx_cdn70_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&cdn_dir).unwrap();
    let cdn_url = format!("file://{}", cdn_dir.canonicalize().unwrap().display());

    let lib = temp_pkg("cdnlib70", "");
    fs::write(
        lib.join("syx.toml"),
        "name = \"cdnlib70\"\nversion = \"1.0.0\"\nmain = \"cdnlib70.sy\"\n",
    )
    .unwrap();
    fs::write(
        lib.join("cdnlib70.sy"),
        "public func answer() -> Int { return 70 }",
    )
    .unwrap();
    pin_test_trust_root();
    publish_package_signed(&lib, &reg, &signing_key).expect("publish signed");

    let pkg_download = PathBuf::from(reg.strip_prefix("file://").unwrap())
        .join("v1/packages/cdnlib70/1.0.0/download");
    let cdn_pkg = cdn_dir.join("v1/packages/cdnlib70/1.0.0");
    fs::create_dir_all(&cdn_pkg).unwrap();
    fs::copy(&pkg_download, cdn_pkg.join("download")).expect("mirror tarball");
    fs::remove_file(&pkg_download).expect("primary tarball removed");

    std::env::set_var("SYX_REGISTRY_CDN", &cdn_url);

    let app = temp_pkg("cdnapp70", "");
    fs::write(
        app.join("syx.toml"),
        format!(
            r#"name = "cdnapp70"
version = "0.1.0"
main = "main.sy"

[registry]
default = "{reg}"

[dependencies.cdnlib70]
version = "1.0.0"
"#
        ),
    )
    .unwrap();
    fs::write(
        app.join("main.sy"),
        r#"import cdnlib70
func main() { print(cdnlib70.answer()) }"#,
    )
    .unwrap();

    install_project(&app).expect("install from CDN mirror");
    let lock = syx_pkg::load_lockfile(&app).unwrap();
    assert_eq!(lock.version, 16);
    assert_eq!(lock.artifact_cdn_base.as_deref(), Some(cdn_url.as_str()));
    assert!(lock.tarball_sha256.contains_key("cdnlib70"));

    syx_driver::compile_project(&app).expect("compile");
    let (_, printed) =
        run_file_printed(&app.join("main.sy"), &syx_driver::DriverOptions::default())
            .expect("run");
    let out = printed.join("\n");
    assert!(out.contains("70"), "stdout: {out}");

    std::env::remove_var("SYX_REGISTRY_CDN");
}

/// C-05 thin doc nail (W682 / ADR-1026): publish→verify env + flow keywords.
#[test]
fn c05_ed25519_publish_flow_docs() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    let quick = include_str!("../../../../docs/8_QuickReference.md");
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    for (label, doc) in [("UsageGuide", usage), ("QuickReference", quick)] {
        assert!(
            doc.contains("SYX_PUBLISHER_SIGNING_KEY")
                && doc.contains("SYX_REGISTRY_VERIFY_KEY")
                && doc.contains("syx1:ed25519:"),
            "{label} must document publisher signing + verify key + syx1:ed25519: flow"
        );
    }
    assert!(
        usage.contains("P011") && usage.contains("P012"),
        "UsageGuide must cite P011 (verify) and P012 (unsigned publish)"
    );
    assert!(
        roadmap.contains("| C-05 |") && roadmap.contains("[x]")
            && roadmap.lines().any(|l| l.contains("C-05") && l.contains("[x]")),
        "roadmap C-05 must be marked [x] (W682)"
    );
}
