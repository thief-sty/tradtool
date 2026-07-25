fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("wasm") {
        println!("cargo:rustc-link-lib=static=arborium_sysroot");
    }
}
