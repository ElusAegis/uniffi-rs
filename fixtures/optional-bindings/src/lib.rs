uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg_attr(feature = "optional", uniffi::export)]
#[cfg(feature = "optional")]
pub fn optional_add(a: i32, b: i32) -> i32 {
    a + b
}