use cbindgen::Language;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_no_includes()
        .with_header("#define FFI_LIB \"libtracingserver.so\"")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/debug/tracingserver.h");
}
