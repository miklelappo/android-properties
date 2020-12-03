use anyhow::{anyhow, Result};
use crate::AndroidProperty;

pub fn plat_getprop(_name: &str) -> Option<String> {
    None
}

pub fn plat_setprop(_name: &str, _value: &str) -> Result<()> {
    Err(anyhow!("Failed to set android property (OS not supported)"))
}

pub fn plat_prop_values() -> impl Iterator<Item = AndroidProperty> {
    let properties: Box<Vec<AndroidProperty>> = Box::new(Vec::new());
    properties.into_iter()
}
