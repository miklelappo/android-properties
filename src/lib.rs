use anyhow::{anyhow, Result};

#[cfg(target_os = "android")]
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
    ptr,
};

const PROPERTY_VALUE_MAX: usize = 92;

pub struct AndroidProperty {
    pub name: String,
    pub value: String,
}

#[repr(C)]
pub struct prop_info {
    pub serial: u32,
    pub value: [u8; PROPERTY_VALUE_MAX],
    pub name: [u8; PROPERTY_VALUE_MAX],
}

#[cfg(target_os = "android")]
type Callback = fn(*mut AndroidProperty, *const c_char, *const c_char, u32);
#[cfg(target_os = "android")]
type ForEachCallback = fn(*const prop_info, *mut Vec<AndroidProperty>);

#[cfg(target_os = "android")]
pub fn property_callback(cookie: *mut AndroidProperty, name: *const c_char, value: *const c_char, _serial: u32) {
    let cname = unsafe { CStr::from_ptr(name) };
    let cvalue = unsafe { CStr::from_ptr(value) };
    unsafe { (*cookie).name = cname.to_str().unwrap().to_string() };
    unsafe { (*cookie).value = cvalue.to_str().unwrap().to_string() };
}

#[cfg(target_os = "android")]
pub fn foreach_property_callback(pi: *const prop_info, cookie: *mut Vec<AndroidProperty>) {
    let mut result = Box::new(AndroidProperty {name: "".to_string(), value: "".to_string()});
    unsafe { __system_property_read_callback(pi, property_callback, &mut *result) };
    unsafe {
        (*cookie).push(*result)
    };
}

#[cfg(target_os = "android")]
extern "C" {
    fn __system_property_set(name: *const c_char, value: *const c_char) -> c_int;
    fn __system_property_find(name: *const c_char) -> *const prop_info;
    fn __system_property_read_callback(pi: *const prop_info, callback: Callback, cookie: *mut AndroidProperty);
    fn __system_property_foreach(callback: ForEachCallback, cookie: *mut Vec<AndroidProperty>) -> c_int;
}

#[cfg(target_os = "android")]
#[cfg(feature = "bionic-deprecated")]
extern "C" {
    /* Deprecated. Use __system_property_read_callback instead. */
    fn __system_property_get(name: *const c_char, value: *mut c_char) -> c_int;
}

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

#[cfg(target_os = "android")]
#[cfg(not(feature = "bionic-deprecated"))]
pub fn getprop(name: &str) -> Option<String> {
    let cname = CString::new(name).unwrap();
    let pi = unsafe { __system_property_find(cname.as_ptr()) };
    if pi == ptr::null() {
        return None;
    }
    let mut result = Box::new(AndroidProperty {name: "".to_string(), value: "".to_string()});
    unsafe { __system_property_read_callback(pi, property_callback, &mut *result) };
    Some(result.value)
}

/// Retrieve a property with name `name`. Returns None if the operation fails.
#[cfg(target_os = "android")]
#[cfg(feature = "bionic-deprecated")]
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

#[cfg(target_os = "android")]
pub fn prop_values() -> impl Iterator<Item = AndroidProperty> {
    let mut properties: Box<Vec<AndroidProperty>> = Box::new(Vec::new());
    unsafe {
        __system_property_foreach(foreach_property_callback, &mut *properties);
    }
    properties.into_iter()
}
