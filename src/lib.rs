use anyhow::{anyhow, Result};

#[cfg(target_os = "android")]
use std::{ffi::CString, os::raw::c_char};

#[cfg(target_os = "android")]
extern "C" {
    fn __system_property_set(name: *const c_char, value: *const c_char) -> i32;
    fn __system_property_get(name: *const c_char, value: *mut c_char) -> i32;
}

const PROPERTY_VALUE_MAX: usize = 92;

/// Set a property with `name` to value `value`
#[cfg(target_os = "android")]
pub fn setprop(name: &str, value: &str) -> Result<()> {
    let cname = CString::new(name).unwrap();
    let cvalue = CString::new(value).unwrap();
    let ret = unsafe { __system_property_set(cname.as_ptr(), cvalue.as_ptr()) };
    if ret >= 0 {
        Ok(())
    } else {
        Err(anyhow!("Failed to set Android property \"{}\" to \"{}\"", name, value))
    }
}

/// Retrieve a property with name `name`. Returns None if the operation fails.
#[cfg(target_os = "android")]
pub fn getprop(name: &str) -> Option<String> {
    let cname = CString::new(name).unwrap();
    let cvalue = CString::new(Vec::with_capacity(PROPERTY_VALUE_MAX)).unwrap();
    let raw = cvalue.into_raw();
    let ret = unsafe { __system_property_get(cname.as_ptr(), raw) };
    match ret {
        len if len > 0 => unsafe { Some(String::from_raw_parts(raw as *mut u8, len as usize, PROPERTY_VALUE_MAX)) },
        _ => None,
    }
}

#[cfg(not(target_os = "android"))]
pub fn getprop(_name: &str) -> Option<String> {
    None
}

#[cfg(not(target_os = "android"))]
pub fn setprop(_name: &str, _value: &str) -> Result<()> {
    Err(anyhow!("Failed to set android property (OS not supported)"))
}
