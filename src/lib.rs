//! android-properties is a rust wrapper for bionic property-related syscalls

#![deny(missing_docs, missing_debug_implementations, unused)]

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_void},
};

#[cfg(target_os = "android")]
use std::mem;

/// A struct representing android properties
///
/// This struct consists from a name-value pair
#[derive(Debug)]
pub struct AndroidProperty {
    /// Property name
    name: String,
    /// Property info pointer
    property_info: *const c_void,
}

impl AndroidProperty {
    /// Initializes and returns struct representing android properties
    pub fn new(name: &str) -> Self {
        AndroidProperty {
            name: name.to_string(),
            property_info: std::ptr::null(),
        }
    }

    /// Return property name
    pub fn name(&self) -> &str {
        &self.name
    }
}

struct ValuePair {
    name: String,
    value: String,
}

struct PropCallbackData {
    props: Vec<AndroidProperty>,
    read_callback_fn: SystemPropertyReadCallbackFn,
}

unsafe fn property_callback(cookie: *mut ValuePair, name: *const c_char, value: *const c_char, _serial: u32) {
    let cname = CStr::from_ptr(name);
    let cvalue = CStr::from_ptr(value);
    (*cookie).name = cname.to_str().unwrap().to_string();
    (*cookie).value = cvalue.to_str().unwrap().to_string();
}

unsafe fn foreach_property_callback(property_info: *const c_void, cookie: *mut PropCallbackData) {
    let mut result = Box::new(ValuePair {
        name: String::new(),
        value: String::new(),
    });
    ((*cookie).read_callback_fn)(property_info, property_callback, &mut *result);
    (*cookie).props.push(AndroidProperty {
        name: (*result).name,
        property_info,
    });
}

type Callback = unsafe fn(*mut ValuePair, *const c_char, *const c_char, u32);
type ForEachCallback = unsafe fn(*const c_void, *mut PropCallbackData);

type SystemPropertyGetFn = unsafe extern "C" fn(*const c_char, *mut c_char) -> c_int;
type SystemPropertySetFn = unsafe extern "C" fn(*const c_char, *const c_char) -> c_int;
type SystemPropertyFindFn = unsafe extern "C" fn(*const c_char) -> *const c_void;
type SystemPropertyReadCallbackFn = unsafe extern "C" fn(*const c_void, Callback, *mut ValuePair) -> *const c_void;
type SystemPropertyForEachFn = unsafe extern "C" fn(ForEachCallback, *mut PropCallbackData) -> c_int;

#[derive(Debug)]
/// An object that can retrieve and modify android properties
pub struct AndroidProperties {
    libc_so: *mut c_void,
    get_fn: Option<SystemPropertyGetFn>,
    set_fn: Option<SystemPropertySetFn>,
    find_fn: Option<SystemPropertyFindFn>,
    read_callback_fn: Option<SystemPropertyReadCallbackFn>,
    for_each_fn: Option<SystemPropertyForEachFn>,
}

impl AndroidProperties {
    #[cfg(not(target_os = "android"))]
    /// Create an entry point for accessing Android properties.
    pub fn new() -> Self {
        AndroidProperties {
            libc_so: std::ptr::null_mut(),
            set_fn: None,
            find_fn: None,
            read_callback_fn: None,
            for_each_fn: None,
            get_fn: None,
        }
    }

    #[cfg(target_os = "android")]
    /// Create an entry point for accessing Android properties.
    pub fn new() -> Self {
        let libc_name = CString::new("libc.so").unwrap();
        let libc_so = unsafe { libc::dlopen(libc_name.as_ptr(), libc::RTLD_NOLOAD) };

        let mut properties = AndroidProperties {
            libc_so,
            set_fn: None,
            find_fn: None,
            read_callback_fn: None,
            for_each_fn: None,
            get_fn: None,
        };

        if libc_so.is_null() {
            return properties;
        }


        unsafe fn load_fn(libc_so: *mut c_void, name: &str) -> Option<*const c_void> {
            let cname = CString::new(name).unwrap();
            let fn_ptr = libc::dlsym(libc_so, cname.as_ptr());

            if fn_ptr.is_null() {
                return None;
            }

            Some(fn_ptr)
        }

        unsafe {
            properties.read_callback_fn = load_fn(libc_so, "__system_property_read_callback")
                .map(|raw| mem::transmute::<*const c_void, SystemPropertyReadCallbackFn>(raw));

            properties.find_fn = load_fn(libc_so, "__system_property_find")
                .map(|raw| mem::transmute::<*const c_void, SystemPropertyFindFn>(raw));

            properties.set_fn = load_fn(libc_so, "__system_property_set")
                .map(|raw| mem::transmute::<*const c_void, SystemPropertySetFn>(raw));

            properties.for_each_fn = load_fn(libc_so, "__system_property_foreach")
                .map(|raw| mem::transmute::<*const c_void, SystemPropertyForEachFn>(raw));

            // Fallback for old versions of Android.
            if properties.read_callback_fn.is_none() || properties.find_fn.is_none() {
                properties.get_fn = load_fn(libc_so, "__system_property_get")
                    .map(|raw| mem::transmute::<*const c_void, SystemPropertyGetFn>(raw));
            }
        }

        properties
    }

    /// Retrieve a property with name `name`. Returns None if the operation fails.
    pub fn get_property(&self, name: &str) -> Option<String> {
        let cname = CString::new(name).unwrap();

        self.get_property_impl(cname.as_ptr(), std::ptr::null())
    }

    fn get_property_impl(&self, cname: *const c_char, mut info: *const c_void) -> Option<String> {
        // If available, use the recommended approach to accessing properties (Android L and onward).
        if let (Some(find_fn), Some(read_callback_fn)) = (self.find_fn, self.read_callback_fn) {
            if info.is_null() {
                info = unsafe { (find_fn)(cname) };
            }

            if info.is_null() {
                return None;
            }

            let mut result = Box::new(ValuePair {
                name: String::new(),
                value: String::new(),
            });

            unsafe {
                (read_callback_fn)(info, property_callback, &mut *result);
            }

            return Some((result).value);
        }

        // Fall back to the older approach.
        if let Some(get_fn) = self.get_fn {
            const PROPERTY_VALUE_MAX: usize = 92;
            let cvalue = CString::new(Vec::with_capacity(PROPERTY_VALUE_MAX)).unwrap();
            let raw = cvalue.into_raw();
            let ret = unsafe { (get_fn)(cname, raw) };
            match ret {
                len if len > 0 => unsafe { Some(String::from_raw_parts(raw as *mut u8, len as usize, PROPERTY_VALUE_MAX)) },
                _ => None,
            }    
        } else {
            None
        }
    }

    /// Set system property `name` to `value`, creating the system property if it doesn't already exist.
    pub fn set_property(&self, name: &str, value: &str) -> Result<(), String> {
        let cname = CString::new(name).unwrap();
        let cvalue = CString::new(value).unwrap();
        if let Some(set_fn) = self.set_fn {
            let ret = unsafe { (set_fn)(cname.as_ptr(), cvalue.as_ptr()) };
            if ret >= 0 {
                Ok(())
            } else {
                Err(format!("Failed to set Android property \"{}\" to \"{}\" (error code: {})", name, value, ret))
            }
        } else {
            Err(format!("Unable to set any Android preperty"))
        }
    }

    /// Retrieve a property. Returns None if the operation fails.
    pub fn get(&self, prop: &AndroidProperty) -> Option<String> {
        let cname = CString::new(&prop.name[..]).unwrap();
        let result = self.get_property_impl(cname.as_ptr(), prop.property_info);

        result
    }

    /// Set a property to `value`, creating the system property if it doesn't already exist.
    pub fn set(&self, prop: &AndroidProperty, value: &str) -> Result<(), String> {
        self.set_property(&prop.name, value)
    }


    /// Returns an iterator to vector, which contains all properties present in a system.
    pub fn property_values(&self) -> impl Iterator<Item = AndroidProperty> {
        if let (Some(for_each_fn), Some(read_callback_fn)) = (self.for_each_fn, self.read_callback_fn) {
            let mut property_data = PropCallbackData {
                props: Vec::new(),
                read_callback_fn,
            };

            unsafe {
                (for_each_fn)(foreach_property_callback, &mut property_data);
            }

            property_data.props.into_iter()
        } else {
            Vec::new().into_iter()
        }
    }
}

impl Drop for AndroidProperties {
    fn drop(&mut self) {
        if !self.libc_so.is_null() {
            unsafe {
                libc::dlclose(self.libc_so);
            }    
        }
    }
}

#[test]
fn simple() {
    let properties = AndroidProperties::new();

    let hello = "hello";

    properties.set_property(hello, "bonjour").unwrap();
    if let Some(val) = properties.get_property(hello) {
        assert_eq!(&val[..], "bonjour");
    }

    println!("Listing all properties:");
    for property in properties.property_values() {
        println!(" - {:?}: {:?}", property.name(), properties.get(&property));
    }
}
