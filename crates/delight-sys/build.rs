#![cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "link_lib3delight")]
    {
        use std::{env, path::PathBuf};

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

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Could not write bindings.");

        // FIXME: add linking to lib3delight
    }

    Ok(())
}
