#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
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
//!   opening an issue [here](https://github.com/virtualritz/delight/issues)
//!   instead.
//!
//! # High Level Bindings
//!
//! There are high level Rust bindings for this API in the
//! [`delight` crate](https://crates.io/crates/delight/).
//!
//! # Compile- vs. Runtime
//!
//! The crate builds as-is, with default features.
//!
//! However, at runtime this crate requires a library/renderer that
//! implements the ɴsɪ C-API to link against. Currently the only
//! renderer that does is [*3Delight*](https://www.3delight.com/).

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

#[cfg(not(feature = "manual_init"))]
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
