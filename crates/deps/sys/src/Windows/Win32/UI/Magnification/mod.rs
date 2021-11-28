#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type MagGetColorEffect = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagGetFullscreenColorEffect = unsafe extern "system" fn(peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagGetFullscreenTransform = unsafe extern "system" fn(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type MagGetImageScalingCallback = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND) -> MagImageScalingCallback;
#[cfg(feature = "Win32_Foundation")]
pub type MagGetInputTransform = unsafe extern "system" fn(pfenabled: *mut super::super::Foundation::BOOL, prectsource: *mut super::super::Foundation::RECT, prectdest: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagGetWindowFilterList = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, pdwfiltermode: *mut u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type MagGetWindowSource = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, prect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagGetWindowTransform = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagInitialize = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagSetColorEffect = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagSetFullscreenColorEffect = unsafe extern "system" fn(peffect: *const MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagSetFullscreenTransform = unsafe extern "system" fn(maglevel: f32, xoffset: i32, yoffset: i32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type MagSetImageScalingCallback = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, callback: MagImageScalingCallback) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagSetInputTransform = unsafe extern "system" fn(fenabled: super::super::Foundation::BOOL, prectsource: *const super::super::Foundation::RECT, prectdest: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagSetWindowFilterList = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, dwfiltermode: u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagSetWindowSource = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, rect: super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagSetWindowTransform = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagShowSystemCursor = unsafe extern "system" fn(fshowcursor: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type MagUninitialize = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[repr(C)]
pub struct MAGCOLOREFFECT {
    pub transform: [f32; 25],
}
impl ::core::marker::Copy for MAGCOLOREFFECT {}
impl ::core::clone::Clone for MAGCOLOREFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MAGIMAGEHEADER {
    pub width: u32,
    pub height: u32,
    pub format: ::windows_sys::core::GUID,
    pub stride: u32,
    pub offset: u32,
    pub cbSize: usize,
}
impl ::core::marker::Copy for MAGIMAGEHEADER {}
impl ::core::clone::Clone for MAGIMAGEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MAGTRANSFORM {
    pub v: [f32; 9],
}
impl ::core::marker::Copy for MAGTRANSFORM {}
impl ::core::clone::Clone for MAGTRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MS_CLIPAROUNDCURSOR: i32 = 2i32;
pub const MS_INVERTCOLORS: i32 = 4i32;
pub const MS_SHOWMAGNIFIEDCURSOR: i32 = 1i32;
pub const MW_FILTERMODE_EXCLUDE: u32 = 0u32;
pub const MW_FILTERMODE_INCLUDE: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type MagImageScalingCallback = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, srcdata: *mut ::core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut ::core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: super::super::Foundation::RECT, clipped: super::super::Foundation::RECT, dirty: super::super::Graphics::Gdi::HRGN) -> super::super::Foundation::BOOL>;
