#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MAGCOLOREFFECT {
    pub transform: [f32; 25],
}
impl MAGCOLOREFFECT {}
impl ::core::default::Default for MAGCOLOREFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MAGCOLOREFFECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MAGCOLOREFFECT").field("transform", &self.transform).finish()
    }
}
impl ::core::cmp::PartialEq for MAGCOLOREFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
    }
}
impl ::core::cmp::Eq for MAGCOLOREFFECT {}
unsafe impl ::windows::core::Abi for MAGCOLOREFFECT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MAGIMAGEHEADER {
    pub width: u32,
    pub height: u32,
    pub format: ::windows::core::GUID,
    pub stride: u32,
    pub offset: u32,
    pub cbSize: usize,
}
impl MAGIMAGEHEADER {}
impl ::core::default::Default for MAGIMAGEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MAGIMAGEHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MAGIMAGEHEADER").field("width", &self.width).field("height", &self.height).field("format", &self.format).field("stride", &self.stride).field("offset", &self.offset).field("cbSize", &self.cbSize).finish()
    }
}
impl ::core::cmp::PartialEq for MAGIMAGEHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.format == other.format && self.stride == other.stride && self.offset == other.offset && self.cbSize == other.cbSize
    }
}
impl ::core::cmp::Eq for MAGIMAGEHEADER {}
unsafe impl ::windows::core::Abi for MAGIMAGEHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MAGTRANSFORM {
    pub v: [f32; 9],
}
impl MAGTRANSFORM {}
impl ::core::default::Default for MAGTRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MAGTRANSFORM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MAGTRANSFORM").field("v", &self.v).finish()
    }
}
impl ::core::cmp::PartialEq for MAGTRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}
impl ::core::cmp::Eq for MAGTRANSFORM {}
unsafe impl ::windows::core::Abi for MAGTRANSFORM {
    type Abi = Self;
}
pub const MS_CLIPAROUNDCURSOR: i32 = 2i32;
pub const MS_INVERTCOLORS: i32 = 4i32;
pub const MS_SHOWMAGNIFIEDCURSOR: i32 = 1i32;
pub const MW_FILTERMODE_EXCLUDE: u32 = 0u32;
pub const MW_FILTERMODE_INCLUDE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetColorEffect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagGetColorEffect(hwnd.into_param().abi(), ::core::mem::transmute(peffect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagGetFullscreenColorEffect(::core::mem::transmute(peffect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagGetFullscreenTransform(::core::mem::transmute(pmaglevel), ::core::mem::transmute(pxoffset), ::core::mem::transmute(pyoffset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn MagGetImageScalingCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> ::core::option::Option<MagImageScalingCallback> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetImageScalingCallback(hwnd: super::super::Foundation::HWND) -> ::core::option::Option<MagImageScalingCallback>;
        }
        ::core::mem::transmute(MagGetImageScalingCallback(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetInputTransform(pfenabled: *mut super::super::Foundation::BOOL, prectsource: *mut super::super::Foundation::RECT, prectdest: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetInputTransform(pfenabled: *mut super::super::Foundation::BOOL, prectsource: *mut super::super::Foundation::RECT, prectdest: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagGetInputTransform(::core::mem::transmute(pfenabled), ::core::mem::transmute(prectsource), ::core::mem::transmute(prectdest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetWindowFilterList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pdwfiltermode: *mut u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetWindowFilterList(hwnd: super::super::Foundation::HWND, pdwfiltermode: *mut u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> i32;
        }
        ::core::mem::transmute(MagGetWindowFilterList(hwnd.into_param().abi(), ::core::mem::transmute(pdwfiltermode), ::core::mem::transmute(count), ::core::mem::transmute(phwnd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetWindowSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, prect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetWindowSource(hwnd: super::super::Foundation::HWND, prect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagGetWindowSource(hwnd.into_param().abi(), ::core::mem::transmute(prect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetWindowTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagGetWindowTransform(hwnd.into_param().abi(), ::core::mem::transmute(ptransform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type MagImageScalingCallback = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, srcdata: *mut ::core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut ::core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: super::super::Foundation::RECT, clipped: super::super::Foundation::RECT, dirty: super::super::Graphics::Gdi::HRGN) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagInitialize() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagInitialize() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagInitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetColorEffect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagSetColorEffect(hwnd.into_param().abi(), ::core::mem::transmute(peffect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagSetFullscreenColorEffect(::core::mem::transmute(peffect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagSetFullscreenTransform(::core::mem::transmute(maglevel), ::core::mem::transmute(xoffset), ::core::mem::transmute(yoffset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn MagSetImageScalingCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, callback: ::core::option::Option<MagImageScalingCallback>) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetImageScalingCallback(hwnd: super::super::Foundation::HWND, callback: ::windows::core::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagSetImageScalingCallback(hwnd.into_param().abi(), ::core::mem::transmute(callback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetInputTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(fenabled: Param0, prectsource: *const super::super::Foundation::RECT, prectdest: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetInputTransform(fenabled: super::super::Foundation::BOOL, prectsource: *const super::super::Foundation::RECT, prectdest: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagSetInputTransform(fenabled.into_param().abi(), ::core::mem::transmute(prectsource), ::core::mem::transmute(prectdest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetWindowFilterList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, dwfiltermode: u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetWindowFilterList(hwnd: super::super::Foundation::HWND, dwfiltermode: u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagSetWindowFilterList(hwnd.into_param().abi(), ::core::mem::transmute(dwfiltermode), ::core::mem::transmute(count), ::core::mem::transmute(phwnd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetWindowSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::RECT>>(hwnd: Param0, rect: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetWindowSource(hwnd: super::super::Foundation::HWND, rect: super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagSetWindowSource(hwnd.into_param().abi(), rect.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetWindowTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagSetWindowTransform(hwnd.into_param().abi(), ::core::mem::transmute(ptransform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagShowSystemCursor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(fshowcursor: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagShowSystemCursor(fshowcursor: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagShowSystemCursor(fshowcursor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagUninitialize() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagUninitialize() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MagUninitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
