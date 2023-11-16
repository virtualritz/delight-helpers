#![cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
//! Utility function for crates depending on `lib3delight`.
//!
//! Strictly to be used from within `build.rs`.
//!
//! ## Compile- vs. Runtime
//!
//! The crate builds as-is, with default features.
//!
//! However, at runtime this crate requires a library/renderer that
//! implements the resp. C-API to link against. Currently the only
//! renderer that does is [*3Delight*](https://www.3delight.com/).
//!
//! ## Cargo Features
//!
//! * `download_lib3delight` -- Fetches the dynamic library version of
//!   3Delight for Linux, macOS or Windows. This can be used as a fallback, to
//!   build against, if you do not have the renderer installed on your system.
//!   But it is an old version of 3Delight and foremost a CI feature.
//!
//!   It is instead suggested that you download a 3Delight package for your
//!   platform & install it. This will set the `DELIGHT` environment variable
//!   that the build script is looking for to find a locally installed library
//!   to link against.
//!
//!   This will also install *3Delight Display* which you can render to,
//!   progressively -- useful for debugging.
//!
//!   The free version renders with up to 12 cores.
//!
//! * `link_lib3delight` -- Statically link against `lib3delight` during build.
//!
//!   This requires a 3Delight installation unless `download_lib3delight` is
//!   set. See also next section.
//!
//! ## Linking Style
//!
//! The 3Delight dynamic library (`lib3delight`) can either be linked to,
//! during build, or loaded at runtime.
//!
//! * By default `lib3deligh` is loaded at runtime. This has several
//!   advantages:
//!
//!   1. If you ship your application or library you can ship it without the
//!      library. It can still run and will print an informative error if the
//!      library cannot be loaded.
//!
//!   2. A user can install an updated version of the renderer and stuff will
//!      ‘just work’.
//!
//! * Dynamically link against `lib3delight`.
//!
//!   * The feature is called `link_lib3delight`. You should disable default
//!     features (they are not needed/used) in this case:
//!
//!     ```toml
//!     [dependencies]
//!     delight-sys = {
//!         version = "0.8",
//!         default-features = false,
//!         features = ["link_lib3delight"]
//!     }
//!     ```
//!
//!   * `lib3delight` becomes a dependency. If it cannot be found by the
//!     system's dynamic linker at runtime, your lib/app will not load/start.

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
