#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type UalInstrument = unsafe extern "system" fn(data: *const UAL_DATA_BLOB) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type UalRegisterProduct = unsafe extern "system" fn(wszproductname: super::super::Foundation::PWSTR, wszrolename: super::super::Foundation::PWSTR, wszguid: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type UalStart = unsafe extern "system" fn(data: *const UAL_DATA_BLOB) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type UalStop = unsafe extern "system" fn(data: *const UAL_DATA_BLOB) -> ::windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct UAL_DATA_BLOB {
    pub Size: u32,
    pub RoleGuid: ::windows_sys::core::GUID,
    pub TenantId: ::windows_sys::core::GUID,
    pub Address: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub UserName: [u16; 260],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for UAL_DATA_BLOB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for UAL_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
