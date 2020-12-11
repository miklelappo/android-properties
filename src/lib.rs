//! android-properties is a rust wrapper for bionic property-related syscalls

#![deny(missing_docs, missing_debug_implementations, unused)]

use std::{fmt, os::raw::c_void};

#[cfg(target_os = "android")]
/// The implementation of property API for Android bionic-based systems
pub mod android;

#[cfg(not(target_os = "android"))]
/// The mock implementation of property API for non-Android based systems
pub mod mock;

/// A struct representing android properties
///
/// This struct consists from a name-value pair
#[derive(Debug)]
pub struct AndroidProperty {
    /// Property name
    pub name: String,
    /// Property value
    pub value: Option<String>,
    /// Property info pointer
    pub property_info: *const c_void,
}

impl AndroidProperty {
    /// Initializes and returns struct representing android properties
    pub fn new(name: String, value: Option<String>, property_info: Option<*const c_void>) -> Self {
        AndroidProperty {
            name: name,
            value: value,
            property_info: property_info.unwrap_or(std::ptr::null()),
        }
    }

    /// Refresh property value using property_info strucutre for optimisation
    /// if possible
    pub fn refresh(&mut self) -> Result<(), String> {
        #[cfg(not(target_os = "android"))]
        return Err("Not supported by platform".into());

        #[cfg(feature = "bionic-deprecated")]
        return Err("Not supported by deprecated bionic".into());

        #[cfg(target_os = "android")]
        #[cfg(not(feature = "bionic-deprecated"))]
        {
            crate::android::plat_refresh_prop(self);
            Ok(())
        }
    }
}

impl fmt::Display for AndroidProperty {
    // Output in format [<name>]: [<value>]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: [{}]", self.name, self.value.as_ref().unwrap_or(&"".to_string()))
    }
}

/// Returns the property value if it exists
pub fn getprop(name: &str) -> AndroidProperty {
    #[cfg(target_os = "android")]
    return crate::android::plat_getprop(name);

    #[cfg(not(target_os = "android"))]
    return crate::mock::plat_getprop(name);
}

/// Sets the property value if it exists or creates new one with specified value
pub fn setprop(name: &str, value: &str) -> Result<(), String> {
    #[cfg(target_os = "android")]
    return crate::android::plat_setprop(name, value);

    #[cfg(not(target_os = "android"))]
    return crate::mock::plat_setprop(name, value);
}

/// Returns an iterator to vector, which contains all properties present in a system
pub fn prop_values() -> impl Iterator<Item = AndroidProperty> {
    #[cfg(target_os = "android")]
    return crate::android::plat_prop_values();

    #[cfg(not(target_os = "android"))]
    return crate::mock::plat_prop_values();
}
