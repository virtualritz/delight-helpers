#![cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
//! Utility function for crates depending on `lib3delight`.
//!
//! Strictly to be used from within `build.rs`.
//!
//! ## Cargo Features
//!
//! * `download_lib3delight` -- Downloads (an outdated version) of `lib3delight`
//!    from Dropbox and copies it to the build directory.
//!
//! * `link_lib3delight` -- Links against `lib3delight` during build.

/// Finds/downloads `lib3delight` and sets up linking.
///
/// To be used from `build.rs`. The version of `lib3delight` this downloads is
/// guaranteed to be outdated. This feature is there so e.g. CI builds succeed
/// without the need to install a full 3Delight package on the build host.
pub fn setup_lib3delight() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "download_lib3delight")]
    #[allow(unused_variables)]
    let lib_path = {
        use std::{
            fs::File,
            io::Write,
            path::{Path, PathBuf},
        };

        let lib_path = PathBuf::from(&std::env::var("OUT_DIR")?);

        eprintln!("Building against 3Delight 2.9.30");

        #[cfg(target_os = "windows")]
        let lib = "https://www.dropbox.com/s/9iavkggor0ecc1x/3Delight.dll";
        #[cfg(target_os = "macos")]
        let lib = "https://www.dropbox.com/s/7vle92kcqbbyn8o/lib3delight.dylib";
        #[cfg(target_os = "linux")]
        let lib = "https://www.dropbox.com/s/wfw6w6p41lqd8ko/lib3delight.so";

        let lib_path = lib_path.join(Path::new(lib).file_name().unwrap());

        eprintln!("lib:     {}", lib_path.display());

        if !lib_path.exists() {
            // Download the lib to build against.
            let lib_data = reqwest::blocking::get(lib.to_owned() + "?dl=1")
                .ok().ok_or("Failed to download lib3delight")?
                .bytes()
                .ok().ok_or("Failed to get data for lib3delight")?;

            File::create(lib_path.clone())?
                .write_all(&lib_data)?;
        }

        lib_path.parent().unwrap().to_path_buf()
    };

    #[cfg(not(feature = "download_lib3delight"))]
    #[allow(unused_variables)]
    let lib_path = if let Ok(dl_path) = std::env::var("DELIGHT") {
        eprintln!("Building against locally installed 3Delight @ {}", &dl_path);
        let lib_path = std::path::PathBuf::from(dl_path);

        #[cfg(target_os = "windows")]
        let lib_path = lib_path.join("bin");

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let lib_path = lib_path.join("lib");

        lib_path
    } else {
        std::path::PathBuf::new()
    };

    #[cfg(feature = "link_lib3delight")]
    {
        // Emit linker searchpath.
        if lib_path.exists() {
            println!("cargo:rustc-link-search={}", lib_path.display());
        }

        // Link to lib3delight.
        println!("cargo:rustc-link-lib=dylib=3delight");
    }

    Ok(())
}
