#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type RtlExtendCorrelationVector = unsafe extern "system" fn(correlationvector: *mut CORRELATION_VECTOR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type RtlIncrementCorrelationVector = unsafe extern "system" fn(correlationvector: *mut CORRELATION_VECTOR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type RtlInitializeCorrelationVector = unsafe extern "system" fn(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: *const ::windows_sys::core::GUID) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type RtlValidateCorrelationVector = unsafe extern "system" fn(vector: *const CORRELATION_VECTOR) -> u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CORRELATION_VECTOR {
    pub Version: super::super::Foundation::CHAR,
    pub Vector: [super::super::Foundation::CHAR; 129],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CORRELATION_VECTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CORRELATION_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RTL_CORRELATION_VECTOR_STRING_LENGTH: u32 = 129u32;
pub const RTL_CORRELATION_VECTOR_V1_LENGTH: u32 = 64u32;
pub const RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH: u32 = 16u32;
pub const RTL_CORRELATION_VECTOR_V2_LENGTH: u32 = 128u32;
pub const RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH: u32 = 22u32;
