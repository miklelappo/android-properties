//! android-properties is a rust wrapper for bionic property-related syscalls

#![deny(
    missing_docs,
    missing_debug_implementations,
    unused
)]

use anyhow::Result;

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
    pub value: String,
}

/// Returns the property value if it exists
pub fn getprop(name: &str) -> Option<String> {
    #[cfg(target_os = "android")]
    return crate::android::plat_getprop(name);

    #[cfg(not(target_os = "android"))]
    return crate::mock::plat_getprop(name);
}

/// Sets the property value if it exists or creates new one with specified value
pub fn setprop(name: &str, value: &str) -> Result<()> {
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