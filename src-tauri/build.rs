use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir)
            .join("dylib")
            .join("vosk-win64-0.3.45")
            .display()
    );
    tauri_build::build()
}
