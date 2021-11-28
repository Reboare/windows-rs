#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type RegisterLicenseKeyWithExpiration = unsafe extern "system" fn(licensekey: super::super::Foundation::PWSTR, validityindays: u32, status: *mut LicenseProtectionStatus) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type ValidateLicenseKeyProtection = unsafe extern "system" fn(licensekey: super::super::Foundation::PWSTR, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows_sys::core::HRESULT;
pub type LicenseProtectionStatus = i32;
pub const Success: LicenseProtectionStatus = 0i32;
pub const LicenseKeyNotFound: LicenseProtectionStatus = 1i32;
pub const LicenseKeyUnprotected: LicenseProtectionStatus = 2i32;
pub const LicenseKeyCorrupted: LicenseProtectionStatus = 3i32;
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = 4i32;
