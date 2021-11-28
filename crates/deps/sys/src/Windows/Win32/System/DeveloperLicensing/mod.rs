#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type AcquireDeveloperLicense = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pexpiration: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CheckDeveloperLicense = unsafe extern "system" fn(pexpiration: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type RemoveDeveloperLicense = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
