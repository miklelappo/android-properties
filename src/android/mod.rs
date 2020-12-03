use crate::AndroidProperty;
use anyhow::{anyhow, Result};

const PROPERTY_VALUE_MAX: usize = 92;

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
};

/// A struct representing property_info
///
/// This is defined inside bionic in bionic/libc/system_properties/include/system_properties/prop_info.h
/// The struct is not complete, as its memeber are never used inside Rust
#[derive(Debug)]
#[repr(C)]
pub struct prop_info {
    /// ID
    pub serial: u32,
    /// Current value
    pub value: [u8; PROPERTY_VALUE_MAX],
}

type Callback = unsafe fn(*mut AndroidProperty, *const c_char, *const c_char, u32);
type ForEachCallback = unsafe fn(*const prop_info, *mut Vec<AndroidProperty>);

unsafe fn property_callback(cookie: *mut AndroidProperty, name: *const c_char, value: *const c_char, _serial: u32) {
    let cname = CStr::from_ptr(name);
    let cvalue = CStr::from_ptr(value);
    (*cookie).name = cname.to_str().unwrap().to_string();
    (*cookie).value = cvalue.to_str().unwrap().to_string();
}

unsafe fn foreach_property_callback(pi: *const prop_info, cookie: *mut Vec<AndroidProperty>) {
    let mut result = Box::new(AndroidProperty {
        name: "".to_string(),
        value: "".to_string(),
    });
    __system_property_read_callback(pi, property_callback, &mut *result);
    (*cookie).push(*result);
}

extern "C" {
    fn __system_property_set(name: *const c_char, value: *const c_char) -> c_int;
    fn __system_property_find(name: *const c_char) -> *const prop_info;
    fn __system_property_read_callback(pi: *const prop_info, callback: Callback, cookie: *mut AndroidProperty);
    fn __system_property_foreach(callback: ForEachCallback, cookie: *mut Vec<AndroidProperty>) -> c_int;
}

#[cfg(feature = "bionic-deprecated")]
extern "C" {
    /* Deprecated. Use __system_property_read_callback instead. */
    fn __system_property_get(name: *const c_char, value: *mut c_char) -> c_int;
}

/// Set system property `name` to `value`, creating the system property if it doesn't already exist
pub fn plat_setprop(name: &str, value: &str) -> Result<()> {
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
#[cfg(not(feature = "bionic-deprecated"))]
pub fn plat_getprop(name: &str) -> Option<String> {
    let cname = CString::new(name).unwrap();
    let pi = unsafe { __system_property_find(cname.as_ptr()) };
    if pi == std::ptr::null() {
        return None;
    }
    let mut result = Box::new(AndroidProperty {
        name: "".to_string(),
        value: "".to_string(),
    });
    unsafe { __system_property_read_callback(pi, property_callback, &mut *result) };
    Some(result.value)
}

/// Retrieve a property with name `name`. Returns None if the operation fails.
#[cfg(feature = "bionic-deprecated")]
pub fn plat_getprop(name: &str) -> Option<String> {
    let cname = CString::new(name).unwrap();
    let cvalue = CString::new(Vec::with_capacity(PROPERTY_VALUE_MAX)).unwrap();
    let raw = cvalue.into_raw();
    let ret = unsafe { __system_property_get(cname.as_ptr(), raw) };
    match ret {
        len if len > 0 => unsafe { Some(String::from_raw_parts(raw as *mut u8, len as usize, PROPERTY_VALUE_MAX)) },
        _ => None,
    }
}

/// Returns an iterator to vector, which contains all properties present in a system
pub fn plat_prop_values() -> impl Iterator<Item = AndroidProperty> {
    let mut properties: Box<Vec<AndroidProperty>> = Box::new(Vec::new());
    unsafe {
        __system_property_foreach(foreach_property_callback, &mut *properties);
    }
    properties.into_iter()
}
