fn main() {
    let bytes = std::fs::read("/tmp/p115.wasm").expect("read /tmp/p115.wasm");
    let mut v = wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all());
    for p in wasmparser::Parser::new(0).parse_all(&bytes) {
        match v.payload(&p.unwrap()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
    println!("valid {} bytes", bytes.len());
}
