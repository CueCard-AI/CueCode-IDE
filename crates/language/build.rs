fn main() {
    let bundled = std::env::var("CUECODE_BUNDLE")
        .ok()
        .or_else(|| std::env::var("ZED_BUNDLE").ok());
    if let Some(bundled) = bundled {
        println!("cargo:rustc-env=CUECODE_BUNDLE={bundled}");
        println!("cargo:rustc-env=ZED_BUNDLE={bundled}");
    }
}
