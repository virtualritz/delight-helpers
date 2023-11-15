#![cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
use std::{env, path::PathBuf};
use delight_build::setup_lib3delight;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Setup linking/download of lib3delight.
    setup_lib3delight()?;

    // Auto-generate Rust bindings for delight.h.
    println!("cargo:rerun-if-changed=include/wrapper.h");

    let delight_include_path =
        PathBuf::from(&env::var("DELIGHT").expect("$DELIGHT is not set")).join("include");

    // Build bindings
    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .allowlist_type("Dl.*")
        .allowlist_function("Dl.*")
        .clang_arg(format!("-I{}", delight_include_path.display()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
