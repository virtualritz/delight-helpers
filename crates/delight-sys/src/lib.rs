#![allow(non_snake_case)]
#![allow(dead_code)]
//! Auto-generated Rust bindings for *Illumination Research*'s 3Delight
//! renderer's utility API.
//!
//! You should not need to use this crate directly except for two
//! reasons. You are likely either:
//!
//! * a masochist who wants to use the C-API directly from Rust.
//!
//! * Not happy with my high level Rust binding (see below) – consider
//!   opening an issue [here](https://github.com/virtualritz/delight-helpers/issues)
//!   instead.
//!
//! ## High Level Bindings
//!
//! There are high level Rust bindings for this API in the
//! [`delight` crate](https://crates.io/crates/delight/).
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
//! * `link_lib3delight` -- Statucally link against `lib3dlight`` during build.
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
use lazy_static::lazy_static;
use std::os::raw::{c_char, c_int};

// Crate features -----------------------------------------------------

#[cfg(not(feature = "link_lib3delight"))]
mod dynamic;
#[cfg(feature = "link_lib3delight")]
mod linked;

#[cfg(not(feature = "link_lib3delight"))]
use self::dynamic as api;
#[cfg(feature = "link_lib3delight")]
use self::linked as api;

// API initalization/on-demand loading of lib3delight -----------------
lazy_static! {
    pub static ref DL_API: api::ApiImpl = api::ApiImpl::new().expect("Could not load lib3delight");
}

pub trait Api {
    fn DlGetVersionString(&self) -> *const c_char;
    fn DlGetLibNameAndVersionString(&self) -> *const c_char;
    fn DlGetCopyrightString(&self) -> *const c_char;
    fn DlGetInstallRoot(&self) -> *const c_char;
    fn DlIsFreeLibrary(&self) -> c_int;
}
