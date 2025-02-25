#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type ISpiControllerProvider = *mut ::core::ffi::c_void;
pub type ISpiDeviceProvider = *mut ::core::ffi::c_void;
pub type ISpiProvider = *mut ::core::ffi::c_void;
pub type ProviderSpiConnectionSettings = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct ProviderSpiMode(pub i32);
impl ProviderSpiMode {
    pub const Mode0: Self = Self(0i32);
    pub const Mode1: Self = Self(1i32);
    pub const Mode2: Self = Self(2i32);
    pub const Mode3: Self = Self(3i32);
}
impl ::core::marker::Copy for ProviderSpiMode {}
impl ::core::clone::Clone for ProviderSpiMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProviderSpiSharingMode(pub i32);
impl ProviderSpiSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderSpiSharingMode {}
impl ::core::clone::Clone for ProviderSpiSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
