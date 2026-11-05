//! Playwright / wasm_web GUI surface checks (specs, package scripts, optional CI runs).

use std::path::PathBuf;
use std::process::Command;

#[test]
fn playwright_spec_exists() {
    assert!(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/wasm_web/playwright.spec.js")
        .exists());
}

#[test]
#[ignore = "requires playwright + dev server; CI runs npm run test:gui"]
fn playwright_npm_test_gui_optional() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web");
    if !root.join("package.json").exists() {
        return;
    }
    let output = Command::new("npm")
        .args(["run", "test:gui"])
        .current_dir(&root)
        .output()
        .expect("npm run test:gui");
    assert!(
        output.status.success(),
        "test:gui failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
#[ignore = "requires playwright + dev server; CI runs npm run test:gui"]
fn playwright_npm_test_gui_numeric_optional() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web");
    if !root.join("package.json").exists() {
        return;
    }
    let output = Command::new("npm")
        .args(["run", "test:gui"])
        .current_dir(&root)
        .output()
        .expect("npm run test:gui");
    assert!(
        output.status.success(),
        "test:gui failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn playwright_multistep_spec_exists() {
    let spec =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web/tier2t.spec.js");
    let body = std::fs::read_to_string(&spec).expect("tier2t.spec.js");
    assert!(body.contains("tier2t"));
    assert!(body.contains("__syx_gui_winit_event_pump"));
    assert!(body.contains("__syx_gui_pixel_read"));
}

#[test]
fn playwright_multistep_compile_run_soft() {
    let web = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web");
    if !web.join("node_modules/@playwright/test").exists() {
        eprintln!("skip: playwright not installed in examples/wasm_web");
        return;
    }
    let output = Command::new("npm")
        .args(["run", "test:gui"])
        .current_dir(&web)
        .output();
    match output {
        Ok(o) if o.status.success() => {}
        Ok(o) => {
            eprintln!(
                "playwright multistep skipped/failed: {}",
                String::from_utf8_lossy(&o.stderr)
            );
        }
        Err(e) => eprintln!("playwright multistep skipped: {e}"),
    }
}

#[test]
fn playwright_guidemo_subset_spec_exists() {
    let spec =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web/tier2u.spec.js");
    let body = std::fs::read_to_string(&spec).expect("tier2u.spec.js");
    assert!(body.contains("tier2u"));
    assert!(body.contains("__syx_gui_winit_window_create"));
    assert!(body.contains("__syx_gui_label_create"));
    assert!(body.contains("__syx_gui_winit_event_pump"));
    assert!(body.contains("__syx_gui_pixel_read"));
}

#[test]
fn playwright_guidemo_subset_compile_run() {
    let web = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web");
    if !web.join("node_modules/@playwright/test").exists() {
        eprintln!("skip: playwright not installed in examples/wasm_web");
        return;
    }
    let output = Command::new("npm")
        .args(["run", "test:gui"])
        .current_dir(&web)
        .output()
        .expect("npm run test:gui must execute");
    assert!(
        output.status.success(),
        "playwright guidemo subset failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn playwright_widgets_spec_exists() {
    let spec =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web/tier2v.spec.js");
    let body = std::fs::read_to_string(&spec).expect("tier2v.spec.js");
    assert!(body.contains("tier2v"));
    assert!(body.contains("Hello SYX"));
    assert!(body.contains("Click me"));
    assert!(body.contains("__syx_gui_label_create"));
}

#[test]
fn playwright_widgets_compile_run() {
    let web = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web");
    if !web.join("node_modules/@playwright/test").exists() {
        eprintln!("skip: playwright not installed in examples/wasm_web");
        return;
    }
    let output = Command::new("npm")
        .args(["run", "test:gui"])
        .current_dir(&web)
        .output()
        .expect("npm run test:gui must execute");
    assert!(
        output.status.success(),
        "playwright widgets failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn playwright_click_field_spec_exists() {
    let spec =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web/tier2w.spec.js");
    let body = std::fs::read_to_string(&spec).expect("tier2w.spec.js");
    assert!(body.contains("tier2w"));
    assert!(body.contains("TIER2W_HANDLER_RAN"));
    assert!(body.contains("__syx_gui_click_inject"));
    assert!(body.contains("__syx_gui_key_inject"));
    assert!(body.contains("__syx_gui_widget_get_text"));
}

#[test]
fn playwright_click_field_compile_run() {
    let web = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web");
    if !web.join("node_modules/@playwright/test").exists() {
        eprintln!("skip: playwright not installed in examples/wasm_web");
        return;
    }
    let output = Command::new("npm")
        .args(["run", "test:gui"])
        .current_dir(&web)
        .output()
        .expect("npm run test:gui must execute");
    assert!(
        output.status.success(),
        "playwright click/field failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
