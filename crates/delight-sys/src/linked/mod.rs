use crate::Api;
use std::os::raw::{c_char, c_int};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub type ApiImpl = LinkedApi;

#[derive(Debug)]
pub struct LinkedApi {}

impl LinkedApi {
    #[inline]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api = LinkedApi {};
        Ok(api)
    }
}

impl Api for LinkedApi {
    #[inline]
    fn DlGetVersionString(&self) -> *const c_char {
        unsafe { DlGetVersionString() }
    }

    #[inline]
    fn DlGetLibNameAndVersionString(&self) -> *const c_char {
        unsafe { DlGetLibNameAndVersionString() }
    }

    #[inline]
    fn DlGetCopyrightString(&self) -> *const c_char {
        unsafe { DlGetCopyrightString() }
    }

    #[inline]
    fn DlGetInstallRoot(&self) -> *const c_char {
        unsafe { DlGetInstallRoot() }
    }

    #[inline]
    fn DlIsFreeLibrary(&self) -> c_int {
        unsafe { DlIsFreeLibrary() }
    }
}
