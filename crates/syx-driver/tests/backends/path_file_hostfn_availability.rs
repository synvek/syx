//! ADR-184 / M2 W40: EC-M2-4 Path/File availability first nail (matrix honesty).
// was: ec_m2_4_path_file_availability

#[test]
fn ec_m2_4_path_file_availability_host_fn_overrides() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "PathJoin",
        "PathNormalize",
        "PathExists",
        "PathExtension",
        "FileOpen",
        "FileClose",
        "FileRead",
        "FileWrite",
        "FileSeek",
        "FileMmapRead",
    ] {
        assert!(
            toml.contains(&format!("name = \"{name}\"")),
            "host_fn_capabilities.toml must override {name}"
        );
    }
    // Spot-check PathJoin block carries vm = "real".
    let path_join = toml
        .split("name = \"PathJoin\"")
        .nth(1)
        .expect("PathJoin");
    let block = path_join.split("[[host]]").next().unwrap_or(path_join);
    assert!(
        block.contains("vm = \"real\""),
        "PathJoin must declare vm=real: {block}"
    );
}

#[test]
fn ec_m2_4_path_file_availability_product_io_path() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"io.path\""),
        "capabilities.toml must declare io.path: {caps}"
    );
    assert!(
        caps.contains("ADR-184") || caps.contains("availability first nail"),
        "io.path notes must cite ADR-184"
    );
}

#[test]
fn ec_m2_4_path_file_availability_generated_md_not_unsupported() {
    let md = include_str!("../../../../books/shared/host-fn-capabilities.md");
    let path_join = md
        .lines()
        .find(|l| l.starts_with("| PathJoin |"))
        .expect("PathJoin row");
    assert!(
        !path_join.contains("| unsupported | unsupported |"),
        "PathJoin must not be all-unsupported: {path_join}"
    );
    assert!(
        path_join.contains("| real |"),
        "PathJoin must list a real backend: {path_join}"
    );

    let file_open = md
        .lines()
        .find(|l| l.starts_with("| FileOpen |"))
        .expect("FileOpen row");
    assert!(
        file_open.contains("| real |"),
        "FileOpen must list a real backend: {file_open}"
    );
}

#[test]
fn ec_m2_4_path_file_availability_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_path_file_availability_slice"),
        "inventory must contain ec_m2_4_path_file_availability_slice: {inv}"
    );
    assert!(
        inv.contains("ADR-184") || inv.contains("signed-ADR-184"),
        "inventory must note ADR-184: {inv}"
    );
}
