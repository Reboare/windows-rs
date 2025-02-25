#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type ProtectFileToEnterpriseIdentity = unsafe extern "system" fn(fileorfolderpath: super::super::Foundation::PWSTR, identity: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type SrpCloseThreadNetworkContext = unsafe extern "system" fn(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type SrpCreateThreadNetworkContext = unsafe extern "system" fn(enterpriseid: super::super::Foundation::PWSTR, threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_sys::core::HRESULT;
pub type SrpDisablePermissiveModeFileEncryption = unsafe extern "system" fn() -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Appx"))]
pub type SrpDoesPolicyAllowAppExecution = unsafe extern "system" fn(packageid: *const super::super::Storage::Packaging::Appx::PACKAGE_ID, isallowed: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type SrpEnablePermissiveModeFileEncryption = unsafe extern "system" fn(enterpriseid: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type SrpGetEnterpriseIds = unsafe extern "system" fn(tokenhandle: super::super::Foundation::HANDLE, numberofbytes: *mut u32, enterpriseids: *mut super::super::Foundation::PWSTR, enterpriseidcount: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type SrpGetEnterprisePolicy = unsafe extern "system" fn(tokenhandle: super::super::Foundation::HANDLE, policyflags: *mut ENTERPRISE_DATA_POLICIES) -> ::windows_sys::core::HRESULT;
pub type SrpHostingInitialize = unsafe extern "system" fn(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> ::windows_sys::core::HRESULT;
pub type SrpHostingTerminate = unsafe extern "system" fn(r#type: SRPHOSTING_TYPE);
#[cfg(feature = "Win32_Foundation")]
pub type SrpIsTokenService = unsafe extern "system" fn(tokenhandle: super::super::Foundation::HANDLE, istokenservice: *mut u8) -> super::super::Foundation::NTSTATUS;
#[cfg(feature = "Win32_Foundation")]
pub type SrpSetTokenEnterpriseId = unsafe extern "system" fn(tokenhandle: super::super::Foundation::HANDLE, enterpriseid: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type UnprotectFile = unsafe extern "system" fn(fileorfolderpath: super::super::Foundation::PWSTR, options: *const FILE_UNPROTECT_OPTIONS) -> ::windows_sys::core::HRESULT;
pub type ENTERPRISE_DATA_POLICIES = u32;
pub const ENTERPRISE_POLICY_NONE: ENTERPRISE_DATA_POLICIES = 0u32;
pub const ENTERPRISE_POLICY_ALLOWED: ENTERPRISE_DATA_POLICIES = 1u32;
pub const ENTERPRISE_POLICY_ENLIGHTENED: ENTERPRISE_DATA_POLICIES = 2u32;
pub const ENTERPRISE_POLICY_EXEMPT: ENTERPRISE_DATA_POLICIES = 4u32;
#[repr(C)]
pub struct FILE_UNPROTECT_OPTIONS {
    pub audit: bool,
}
impl ::core::marker::Copy for FILE_UNPROTECT_OPTIONS {}
impl ::core::clone::Clone for FILE_UNPROTECT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTHREAD_NETWORK_CONTEXT {
    pub ThreadId: u32,
    pub ThreadContext: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTHREAD_NETWORK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTHREAD_NETWORK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IProtectionPolicyManagerInterop = *mut ::core::ffi::c_void;
pub type IProtectionPolicyManagerInterop2 = *mut ::core::ffi::c_void;
pub type IProtectionPolicyManagerInterop3 = *mut ::core::ffi::c_void;
pub type SRPHOSTING_TYPE = i32;
pub const SRPHOSTING_TYPE_NONE: SRPHOSTING_TYPE = 0i32;
pub const SRPHOSTING_TYPE_WINHTTP: SRPHOSTING_TYPE = 1i32;
pub const SRPHOSTING_TYPE_WININET: SRPHOSTING_TYPE = 2i32;
pub type SRPHOSTING_VERSION = i32;
pub const SRPHOSTING_VERSION1: SRPHOSTING_VERSION = 1i32;
