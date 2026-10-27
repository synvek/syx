//! Phase 69: browser prelude aligned with driver protocol graph + Iterator `next() -> T?`.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir.join("../..");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("browser_prelude.sy");

    let manifest = root.join("packages/internal/manifest.json");
    println!("cargo:rerun-if-changed={}", manifest.display());
    for proto in [
        "equatable.sy",
        "hashable.sy",
        "comparable.sy",
        "custom_string_convertible.sy",
    ] {
        println!(
            "cargo:rerun-if-changed={}",
            root.join("packages/core/protocols").join(proto).display()
        );
    }

    let mut prelude = String::from("// Generated browser unified prelude (Phase 69)\n");
    prelude.push_str(
        "public protocol Equatable {\n}\n\n\
         public protocol Hashable {\n}\n\n\
         public protocol Comparable: Equatable {\n    func compare(other: Self) -> Int { }\n}\n\n\
         public protocol CustomStringConvertible {\n}\n\n\
         protocol Iterator {\n    func next() -> Int? { }\n}\n\n\
         class Point: Comparable {\n    var x: Int\n    init(x: Int) { self.x = x }\n    func compare(other: Point) -> Int {\n        return self.x - other.x\n    }\n}\n",
    );

    fs::write(&out_path, prelude).expect("write browser_prelude.sy");
}
