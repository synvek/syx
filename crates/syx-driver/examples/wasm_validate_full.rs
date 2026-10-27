use std::fs;

fn main() {
    std::env::set_var("SYX_WASM_SIMD128", "1");
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    std::env::remove_var("SYX_WASM_MIR_EXEC_ONLY");
    let mut cfg = syx_config::runtime_config();
    cfg.enable_wasm_simd128 = true;
    syx_config::install_runtime_config(cfg);
    syx_host_plugin::register_echo_plugin_in_process();
    let d = std::env::temp_dir().join("syx_wasm_validate_full");
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    let src = r#"func main() {
    let v: SIMD2<Float64> = SIMD2.fromArray([2.0f64, 3.0f64])
    let w: SIMD2<Float64> = SIMD2.fromArray([4.0f64, 5.0f64])
    print(SIMD2.reduceAdd(SIMD2.mulF64(v, w)))
}"#;
    fs::write(d.join("main.sy"), src).unwrap();
    fs::write(d.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let mut o = syx_driver::DriverOptions::default();
    o.wasm_aot = true;
    let path = d.join("main.sy");
    let bytes = syx_driver::emit_wasm_bytes(&path, &o).expect("emit wasm");
    fs::write("/tmp/syx_wasm_full.wasm", &bytes).unwrap();
    println!("emitted {} bytes", bytes.len());
    match syx_wasm_run::run_wasm_capture(&bytes, "main", None) {
        Ok(r) => println!("wasmtime ok exit={} lines={:?}", r.exit_code, r.printed),
        Err(e) => {
            eprintln!("wasmtime run: {e:?}");
            std::process::exit(1);
        }
    }
}
