#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type CreateAppContainerProfile = unsafe extern "system" fn(pszappcontainername: super::super::Foundation::PWSTR, pszdisplayname: super::super::Foundation::PWSTR, pszdescription: super::super::Foundation::PWSTR, pcapabilities: *const super::SID_AND_ATTRIBUTES, dwcapabilitycount: u32, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DeleteAppContainerProfile = unsafe extern "system" fn(pszappcontainername: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DeriveAppContainerSidFromAppContainerName = unsafe extern "system" fn(pszappcontainername: super::super::Foundation::PWSTR, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName = unsafe extern "system" fn(psidappcontainersid: super::super::Foundation::PSID, pszrestrictedappcontainername: super::super::Foundation::PWSTR, ppsidrestrictedappcontainersid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type GetAppContainerFolderPath = unsafe extern "system" fn(pszappcontainersid: super::super::Foundation::PWSTR, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type GetAppContainerNamedObjectPath = unsafe extern "system" fn(token: super::super::Foundation::HANDLE, appcontainersid: super::super::Foundation::PSID, objectpathlength: u32, objectpath: super::super::Foundation::PWSTR, returnlength: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_System_Registry")]
pub type GetAppContainerRegistryLocation = unsafe extern "system" fn(desiredaccess: u32, phappcontainerkey: *mut super::super::System::Registry::HKEY) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IsProcessInIsolatedContainer = unsafe extern "system" fn(isprocessinisolatedcontainer: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IsProcessInIsolatedWindowsEnvironment = unsafe extern "system" fn(isprocessinisolatedwindowsenvironment: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IsProcessInWDAGContainer = unsafe extern "system" fn(reserved: *const ::core::ffi::c_void, isprocessinwdagcontainer: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
pub type IIsolatedAppLauncher = *mut ::core::ffi::c_void;
pub const IsolatedAppLauncher: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3162580016, data2: 59230, data3: 20433, data4: [150, 65, 31, 159, 30, 45, 154, 31] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolatedAppLauncherTelemetryParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolatedAppLauncherTelemetryParameters {
    fn clone(&self) -> Self {
        *self
    }
}
