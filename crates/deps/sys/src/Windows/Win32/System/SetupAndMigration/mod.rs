#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type OOBEComplete = unsafe extern "system" fn(isoobecomplete: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type RegisterWaitUntilOOBECompleted = unsafe extern "system" fn(oobecompletedcallback: OOBE_COMPLETED_CALLBACK, callbackcontext: *const ::core::ffi::c_void, waithandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type UnregisterWaitUntilOOBECompleted = unsafe extern "system" fn(waithandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type OOBE_COMPLETED_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *const ::core::ffi::c_void)>;
