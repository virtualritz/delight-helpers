use crate::*;
use dlopen2::wrapper::{Container, WrapperApi};
use std::{env, path::Path};

pub type ApiImpl = DynamicApi;

#[derive(WrapperApi)]
struct CApi {
    DlGetCopyrightString: extern "C" fn() -> *const c_char,
    DlGetInstallRoot: extern "C" fn() -> *const c_char,
    DlGetLibNameAndVersionString: extern "C" fn() -> *const c_char,
    DlGetVersionString: extern "C" fn() -> *const c_char,
    DlIsFreeLibrary: extern "C" fn() -> c_int,
}

pub struct DynamicApi {
    api: Container<CApi>,
}

#[cfg(target_os = "linux")]
static DELIGHT_APP_PATH: &str = "/usr/local/3delight/lib/lib3delight.so";

#[cfg(target_os = "macos")]
static DELIGHT_APP_PATH: &str = "/Applications/3Delight/lib/lib3delight.dylib";

#[cfg(target_os = "windows")]
static DELIGHT_APP_PATH: &str = "C:/%ProgramFiles%/3Delight/bin/3Delight.dll";

#[cfg(target_os = "linux")]
static DELIGHT_LIB: &str = "lib3delight.so";

#[cfg(target_os = "macos")]
static DELIGHT_LIB: &str = "lib3delight.dylib";

#[cfg(target_os = "windows")]
static DELIGHT_LIB: &str = "3Delight.dll";

impl DynamicApi {
    #[inline]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        match unsafe { Container::load(DELIGHT_APP_PATH) }
            .or_else(|_| unsafe { Container::load(DELIGHT_LIB) })
            .or_else(|_| match env::var("DELIGHT") {
                Err(e) => Err(Box::new(e) as _),
                Ok(delight) => unsafe {
                    #[cfg(any(target_os = "linux", target_os = "macos"))]
                    let path = Path::new(&delight).join("lib").join(DELIGHT_LIB);
                    #[cfg(target_os = "windows")]
                    let path = Path::new(&delight).join("bin").join(DELIGHT_LIB);

                    Container::load(path)
                }
                .map_err(|e| Box::new(e) as _),
            }) {
            Err(e) => Err(e),
            Ok(api) => {
                let api = DynamicApi { api };
                Ok(api)
            }
        }
    }
}

impl TryFrom<&Path> for DynamicApi {
    type Error = dlopen2::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        match unsafe { Container::load(path) } {
            Err(e) => Err(e),
            Ok(api) => {
                let api = DynamicApi { api };
                Ok(api)
            }
        }
    }
}

impl Api for DynamicApi {
    #[inline]
    fn DlGetCopyrightString(&self) -> *const c_char {
        self.api.DlGetCopyrightString()
    }

    #[inline]
    fn DlGetInstallRoot(&self) -> *const c_char {
        self.api.DlGetInstallRoot()
    }

    #[inline]
    fn DlGetLibNameAndVersionString(&self) -> *const c_char {
        self.api.DlGetLibNameAndVersionString()
    }

    #[inline]
    fn DlGetVersionString(&self) -> *const c_char {
        self.api.DlGetVersionString()
    }

    #[inline]
    fn DlIsFreeLibrary(&self) -> c_int {
        self.api.DlIsFreeLibrary()
    }
}
