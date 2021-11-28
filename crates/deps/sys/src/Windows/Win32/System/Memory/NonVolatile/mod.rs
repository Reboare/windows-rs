#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type RtlDrainNonVolatileFlush = unsafe extern "system" fn(nvtoken: *const ::core::ffi::c_void) -> u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type RtlFillNonVolatileMemory = unsafe extern "system" fn(nvtoken: *const ::core::ffi::c_void, nvdestination: *mut ::core::ffi::c_void, size: usize, value: u8, flags: u32) -> u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type RtlFlushNonVolatileMemory = unsafe extern "system" fn(nvtoken: *const ::core::ffi::c_void, nvbuffer: *const ::core::ffi::c_void, size: usize, flags: u32) -> u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type RtlFlushNonVolatileMemoryRanges = unsafe extern "system" fn(nvtoken: *const ::core::ffi::c_void, nvranges: *const NV_MEMORY_RANGE, numranges: usize, flags: u32) -> u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type RtlFreeNonVolatileToken = unsafe extern "system" fn(nvtoken: *const ::core::ffi::c_void) -> u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type RtlGetNonVolatileToken = unsafe extern "system" fn(nvbuffer: *const ::core::ffi::c_void, size: usize, nvtoken: *mut *mut ::core::ffi::c_void) -> u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type RtlWriteNonVolatileMemory = unsafe extern "system" fn(nvtoken: *const ::core::ffi::c_void, nvdestination: *mut ::core::ffi::c_void, source: *const ::core::ffi::c_void, size: usize, flags: u32) -> u32;
#[repr(C)]
pub struct NV_MEMORY_RANGE {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub Length: usize,
}
impl ::core::marker::Copy for NV_MEMORY_RANGE {}
impl ::core::clone::Clone for NV_MEMORY_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
