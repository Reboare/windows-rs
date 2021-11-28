#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateMailslotA = unsafe extern "system" fn(lpname: super::super::Foundation::PSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateMailslotW = unsafe extern "system" fn(lpname: super::super::Foundation::PWSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type GetMailslotInfo = unsafe extern "system" fn(hmailslot: super::super::Foundation::HANDLE, lpmaxmessagesize: *mut u32, lpnextsize: *mut u32, lpmessagecount: *mut u32, lpreadtimeout: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetMailslotInfo = unsafe extern "system" fn(hmailslot: super::super::Foundation::HANDLE, lreadtimeout: u32) -> super::super::Foundation::BOOL;
