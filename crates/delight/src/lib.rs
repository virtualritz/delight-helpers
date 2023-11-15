//! Utility functions for working with library version of the
//! [*3Delight*](https://www.3delight.com) renderer, `lib3delight`.
use delight_sys::{Api, DL_API};
use std::{ffi::CStr, path::PathBuf};

/// Get the copyright string of the 3Delight library.
///
/// E.g. `Copyright (c) 1999-2023 The 3Delight Team.`.
pub fn copyright() -> String {
    unsafe { CStr::from_ptr(DL_API.DlGetCopyrightString()) }
        .to_string_lossy()
        .into_owned()
}

/// Get the name and version of the 3Delight library.
///
/// E.g. `3DeLight 1.1.1a (Jan 01 2000)`.
pub fn name_and_version() -> String {
    unsafe { CStr::from_ptr(DL_API.DlGetLibNameAndVersionString()) }
        .to_string_lossy()
        .into_owned()
}

/// Get the version of the 3Delight library.
///
/// E.g. `1.1.1a (Jan 01 2000)`.
pub fn version() -> String {
    unsafe { CStr::from_ptr(DL_API.DlGetVersionString()) }
        .to_string_lossy()
        .into_owned()
}

/// Get the path to the root of the 3Delight installation.
pub fn install_root() -> PathBuf {
    PathBuf::from(
        unsafe { CStr::from_ptr(DL_API.DlGetInstallRoot()) }
            .to_string_lossy()
            .into_owned(),
    )
}

/// Returns `true`` if the free version of the 3Delight library is being used.
pub fn is_free_library() -> bool {
    DL_API.DlIsFreeLibrary() != 0
}
