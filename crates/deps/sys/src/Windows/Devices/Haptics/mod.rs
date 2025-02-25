#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type SimpleHapticsController = *mut ::core::ffi::c_void;
pub type SimpleHapticsControllerFeedback = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct VibrationAccessStatus(pub i32);
impl VibrationAccessStatus {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const DeniedByEnergySaver: Self = Self(3i32);
}
impl ::core::marker::Copy for VibrationAccessStatus {}
impl ::core::clone::Clone for VibrationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VibrationDevice = *mut ::core::ffi::c_void;
