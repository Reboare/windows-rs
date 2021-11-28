#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type EnumDynamicTimeZoneInformation = unsafe extern "system" fn(dwindex: u32, lptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type FileTimeToSystemTime = unsafe extern "system" fn(lpfiletime: *const super::super::Foundation::FILETIME, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetDynamicTimeZoneInformation = unsafe extern "system" fn(ptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type GetDynamicTimeZoneInformationEffectiveYears = unsafe extern "system" fn(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear: *mut u32, lastyear: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type GetTimeZoneInformation = unsafe extern "system" fn(lptimezoneinformation: *mut TIME_ZONE_INFORMATION) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type GetTimeZoneInformationForYear = unsafe extern "system" fn(wyear: u16, pdtzi: *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi: *mut TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LocalFileTimeToLocalSystemTime = unsafe extern "system" fn(timezoneinformation: *const TIME_ZONE_INFORMATION, localfiletime: *const super::super::Foundation::FILETIME, localsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LocalSystemTimeToLocalFileTime = unsafe extern "system" fn(timezoneinformation: *const TIME_ZONE_INFORMATION, localsystemtime: *const super::super::Foundation::SYSTEMTIME, localfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetDynamicTimeZoneInformation = unsafe extern "system" fn(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetTimeZoneInformation = unsafe extern "system" fn(lptimezoneinformation: *const TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SystemTimeToFileTime = unsafe extern "system" fn(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SystemTimeToTzSpecificLocalTime = unsafe extern "system" fn(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SystemTimeToTzSpecificLocalTimeEx = unsafe extern "system" fn(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type TzSpecificLocalTimeToSystemTime = unsafe extern "system" fn(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type TzSpecificLocalTimeToSystemTimeEx = unsafe extern "system" fn(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DYNAMIC_TIME_ZONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TIME_ZONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TSF_Authenticated: u32 = 2u32;
pub const TSF_Hardware: u32 = 1u32;
pub const TSF_IPv6: u32 = 4u32;
pub const TSF_SignatureAuthenticated: u32 = 8u32;
