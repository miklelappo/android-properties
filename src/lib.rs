use anyhow::Result;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(not(target_os = "android"))]
pub mod mock;

pub struct AndroidProperty {
    pub name: String,
    pub value: String,
}
pub fn getprop(name: &str) -> Option<String> {
    #[cfg(target_os = "android")]
    return crate::android::plat_getprop(name);

    #[cfg(not(target_os = "android"))]
    return crate::mock::plat_getprop(name);
}

pub fn setprop(name: &str, value: &str) -> Result<()> {
    #[cfg(target_os = "android")]
    return crate::android::plat_setprop(name, value);

    #[cfg(not(target_os = "android"))]
    return crate::mock::plat_setprop(name, value);
}

pub fn prop_values() -> impl Iterator<Item = AndroidProperty> {
    #[cfg(target_os = "android")]
    return crate::android::plat_prop_values();

    #[cfg(not(target_os = "android"))]
    return crate::mock::plat_prop_values();
}