#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type AppCapability = *mut ::core::ffi::c_void;
pub type AppCapabilityAccessChangedEventArgs = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct AppCapabilityAccessStatus(pub i32);
impl AppCapabilityAccessStatus {
    pub const DeniedBySystem: Self = Self(0i32);
    pub const NotDeclaredByApp: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const UserPromptRequired: Self = Self(3i32);
    pub const Allowed: Self = Self(4i32);
}
impl ::core::marker::Copy for AppCapabilityAccessStatus {}
impl ::core::clone::Clone for AppCapabilityAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
