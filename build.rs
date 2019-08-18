use std::env;

fn main() {
    let bdir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-lib=dylib=hello");
    println!("cargo:rustc-link-search=native={}/src/c/bin",bdir);
}
