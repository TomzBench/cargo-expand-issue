use std::env;
fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("invalid manifest dir");
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_parse_expand(&["cargo-expand-issue"])
        .generate()
        .unwrap()
        .write_to_file("header.h");
}
