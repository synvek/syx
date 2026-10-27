//! One-shot corpus normalize for W684–W816 (run with --ignored).
//! Writes dry-run output for every .sy under repo root that formats successfully
//! and would_change.

use std::fs;
use std::path::{Path, PathBuf};

use syx_diagnostics::SourceFile;
use syx_fmt::format_dry_run;

fn collect_sy(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if matches!(name, "target" | ".git" | ".sy" | "node_modules") {
                continue;
            }
            collect_sy(&p, out);
        } else if p.extension().and_then(|e| e.to_str()) == Some("sy") {
            out.push(p);
        }
    }
}

#[test]
#[ignore = "W816 one-shot: cargo test -p syx-fmt --test normalize_would_change -- --ignored"]
fn normalize_would_change_corpus() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut files = Vec::new();
    collect_sy(&root, &mut files);
    files.sort();
    let mut wrote = 0usize;
    let mut skipped = 0usize;
    let mut failed = 0usize;
    for file in files {
        let Ok(text) = fs::read_to_string(&file) else {
            failed += 1;
            continue;
        };
        let source = SourceFile::new(file.display().to_string(), text);
        match format_dry_run(&source) {
            Ok(report) if report.would_change => {
                fs::write(&file, &report.formatted).expect("write");
                wrote += 1;
                eprintln!("wrote: {}", file.display());
            }
            Ok(_) => {
                skipped += 1;
            }
            Err(_) => {
                failed += 1;
            }
        }
    }
    eprintln!("normalize: wrote={wrote} unchanged={skipped} failed={failed}");
    assert!(wrote > 0 || skipped > 0, "expected some .sy files");
}
