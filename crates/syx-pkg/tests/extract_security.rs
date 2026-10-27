//! Wave C W674–W675 / ADR-1018–1019 / C-03: tar traversal + symlink escape + vendor-slot
//! cache poison must fail (P013).

use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::symlink as os_symlink;

use flate2::write::GzEncoder;
use flate2::Compression;
use tar::{Builder, EntryType, Header};

fn gzip_tar_with<F>(f: F) -> Vec<u8>
where
    F: FnOnce(&mut Builder<&mut GzEncoder<Vec<u8>>>),
{
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    {
        let mut builder = Builder::new(&mut encoder);
        f(&mut builder);
        builder.finish().expect("finish tar");
    }
    encoder.finish().expect("finish gzip")
}

fn append_regular(builder: &mut Builder<&mut GzEncoder<Vec<u8>>>, path: &str, data: &[u8]) {
    let mut header = Header::new_gnu();
    header.set_path(path).unwrap();
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder.append(&header, data).unwrap();
}

/// Craft a path the `tar` Builder API would refuse (`..`), by writing name bytes directly.
fn append_traversal_path(builder: &mut Builder<&mut GzEncoder<Vec<u8>>>, path: &str, data: &[u8]) {
    let mut header = Header::new_gnu();
    {
        let name = &mut header.as_old_mut().name;
        name.fill(0);
        let bytes = path.as_bytes();
        assert!(bytes.len() < name.len(), "path too long for ustar name");
        name[..bytes.len()].copy_from_slice(bytes);
    }
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder.append(&header, data).unwrap();
}

fn append_symlink(builder: &mut Builder<&mut GzEncoder<Vec<u8>>>, path: &str, target: &str) {
    let mut header = Header::new_gnu();
    header.set_entry_type(EntryType::Symlink);
    header.set_path(path).unwrap();
    header.set_link_name(target).unwrap();
    header.set_size(0);
    header.set_mode(0o777);
    header.set_cksum();
    builder.append(&header, std::io::empty()).unwrap();
}

#[test]
fn extract_rejects_parent_dir_traversal() {
    let bytes = gzip_tar_with(|b| {
        append_traversal_path(b, "../escape.txt", b"pwned");
    });
    let root = tempfile::tempdir().unwrap();
    let dest = root.path().join("vendor");
    let outside = root.path().join("escape.txt");
    let err = syx_pkg::extract_tar_gz(&bytes, &dest).expect_err("traversal must fail");
    let msg = format!("{err}");
    assert!(msg.contains("P013"), "got: {msg}");
    assert!(
        !outside.exists(),
        "escape file must not appear outside dest"
    );
    assert!(
        !dest.join("escape.txt").exists(),
        "escape must not land inside dest either via strip"
    );
}

#[test]
fn extract_rejects_symlink_escape() {
    let bytes = gzip_tar_with(|b| {
        append_symlink(b, "evil_link", "/tmp/syx-c03-escape-target");
    });
    let root = tempfile::tempdir().unwrap();
    let dest = root.path().join("vendor");
    let err = syx_pkg::extract_tar_gz(&bytes, &dest).expect_err("symlink escape must fail");
    let msg = format!("{err}");
    assert!(msg.contains("P013"), "got: {msg}");
    assert!(
        !dest.join("evil_link").exists(),
        "escaping symlink must not be created"
    );
}

#[test]
fn extract_rejects_relative_symlink_escape() {
    let bytes = gzip_tar_with(|b| {
        append_symlink(b, "nested/evil_link", "../../outside.txt");
    });
    let root = tempfile::tempdir().unwrap();
    let dest = root.path().join("vendor");
    let outside = root.path().join("outside.txt");
    let err = syx_pkg::extract_tar_gz(&bytes, &dest).expect_err("relative symlink escape must fail");
    let msg = format!("{err}");
    assert!(msg.contains("P013"), "got: {msg}");
    assert!(!outside.exists());
    assert!(!dest.join("nested/evil_link").exists());
}

#[test]
fn extract_allows_safe_tarball() {
    let bytes = gzip_tar_with(|b| {
        append_regular(b, "syx.toml", b"name = \"safe\"\nversion = \"1.0.0\"\n");
        append_regular(b, "src/main.sy", b"func main() { return 0 }\n");
        append_symlink(b, "alias.sy", "src/main.sy");
    });
    let root = tempfile::tempdir().unwrap();
    let dest = root.path().join("vendor");
    syx_pkg::extract_tar_gz(&bytes, &dest).expect("safe tarball must extract");
    assert!(dest.join("syx.toml").is_file());
    assert!(dest.join("src/main.sy").is_file());
    assert!(Path::new(&dest.join("alias.sy")).exists());
}

#[cfg(unix)]
#[test]
fn extract_rejects_symlink_dest_poison() {
    let root = tempfile::tempdir().unwrap();
    let victim = root.path().join("victim-1.0.0");
    std::fs::create_dir_all(&victim).unwrap();
    std::fs::write(victim.join("marker"), b"clean").unwrap();
    let dest = root.path().join("evil-pkg-1.0.0");
    os_symlink(&victim, &dest).unwrap();

    let bytes = gzip_tar_with(|b| {
        append_regular(b, "pwn.txt", b"hacked");
    });
    let err = syx_pkg::extract_tar_gz(&bytes, &dest).expect_err("symlink dest must fail");
    let msg = format!("{err}");
    assert!(msg.contains("P013"), "got: {msg}");
    assert_eq!(
        std::fs::read(victim.join("marker")).unwrap(),
        b"clean",
        "sibling victim must not be written through"
    );
    assert!(!victim.join("pwn.txt").exists());
}

#[test]
fn extract_rejects_sibling_path_as_poison() {
    let bytes = gzip_tar_with(|b| {
        append_traversal_path(b, "../victim-1.0.0/marker", b"poisoned");
    });
    let root = tempfile::tempdir().unwrap();
    let victim = root.path().join("victim-1.0.0");
    std::fs::create_dir_all(&victim).unwrap();
    std::fs::write(victim.join("marker"), b"clean").unwrap();
    let dest = root.path().join("evil-pkg-1.0.0");
    let err = syx_pkg::extract_tar_gz(&bytes, &dest).expect_err("sibling path poison must fail");
    let msg = format!("{err}");
    assert!(msg.contains("P013"), "got: {msg}");
    assert_eq!(std::fs::read(victim.join("marker")).unwrap(), b"clean");
}

#[cfg(unix)]
#[test]
fn prepare_rejects_symlink_vendor_slot() {
    let root = tempfile::tempdir().unwrap();
    let victim = root.path().join("victim-1.0.0");
    std::fs::create_dir_all(&victim).unwrap();
    std::fs::write(victim.join("marker"), b"clean").unwrap();
    let dest = root.path().join("evil-pkg-1.0.0");
    os_symlink(&victim, &dest).unwrap();

    let err = syx_pkg::prepare_vendor_extract_dest(&dest).expect_err("symlink slot must fail");
    let msg = format!("{err}");
    assert!(msg.contains("P013"), "got: {msg}");
    assert_eq!(std::fs::read(victim.join("marker")).unwrap(), b"clean");
}

#[test]
fn prepare_wipes_existing_vendor_dir() {
    let root = tempfile::tempdir().unwrap();
    let dest = root.path().join("pkg-1.0.0");
    std::fs::create_dir_all(&dest).unwrap();
    std::fs::write(dest.join("stale.txt"), b"old").unwrap();
    syx_pkg::prepare_vendor_extract_dest(&dest).expect("wipe must succeed");
    assert!(!dest.exists(), "vendor slot must be removed after wipe");
}
