use crate::AndroidProperty;
use anyhow::{anyhow, Result};

/// Mock implementation for getprop
///
/// Always returns None
pub fn plat_getprop(_name: &str) -> Option<String> {
    None
}

/// Mock implementation for setprop
///
/// Always returns Err
pub fn plat_setprop(_name: &str, _value: &str) -> Result<()> {
    Err(anyhow!("Failed to set android property (OS not supported)"))
}

/// Mock implementation for prop_values
///
/// Always returns iterator to empty vector
pub fn plat_prop_values() -> impl Iterator<Item = AndroidProperty> {
    let properties: Box<Vec<AndroidProperty>> = Box::new(Vec::new());
    properties.into_iter()
}
