#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CDB_REPORT_BITS: u32 = 0u32;
pub const CDB_REPORT_BYTES: u32 = 1u32;
pub const COMDB_MAX_PORTS_ARBITRATED: u32 = 4096u32;
pub const COMDB_MIN_PORTS_ARBITRATED: u32 = 256u32;
#[inline]
pub unsafe fn ComDBClaimNextFreePort<'a, Param0: ::windows::core::IntoParam<'a, HCOMDB>>(hcomdb: Param0, comnumber: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBClaimNextFreePort(hcomdb: HCOMDB, comnumber: *mut u32) -> i32;
        }
        ::core::mem::transmute(ComDBClaimNextFreePort(hcomdb.into_param().abi(), ::core::mem::transmute(comnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ComDBClaimPort<'a, Param0: ::windows::core::IntoParam<'a, HCOMDB>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hcomdb: Param0, comnumber: u32, forceclaim: Param2, forced: *mut super::super::Foundation::BOOL) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBClaimPort(hcomdb: HCOMDB, comnumber: u32, forceclaim: super::super::Foundation::BOOL, forced: *mut super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(ComDBClaimPort(hcomdb.into_param().abi(), ::core::mem::transmute(comnumber), forceclaim.into_param().abi(), ::core::mem::transmute(forced)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBClose<'a, Param0: ::windows::core::IntoParam<'a, HCOMDB>>(hcomdb: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBClose(hcomdb: HCOMDB) -> i32;
        }
        ::core::mem::transmute(ComDBClose(hcomdb.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBGetCurrentPortUsage<'a, Param0: ::windows::core::IntoParam<'a, HCOMDB>>(hcomdb: Param0, buffer: *mut u8, buffersize: u32, reporttype: u32, maxportsreported: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBGetCurrentPortUsage(hcomdb: HCOMDB, buffer: *mut u8, buffersize: u32, reporttype: u32, maxportsreported: *mut u32) -> i32;
        }
        ::core::mem::transmute(ComDBGetCurrentPortUsage(hcomdb.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(reporttype), ::core::mem::transmute(maxportsreported)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBOpen(phcomdb: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBOpen(phcomdb: *mut isize) -> i32;
        }
        ::core::mem::transmute(ComDBOpen(::core::mem::transmute(phcomdb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBReleasePort<'a, Param0: ::windows::core::IntoParam<'a, HCOMDB>>(hcomdb: Param0, comnumber: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBReleasePort(hcomdb: HCOMDB, comnumber: u32) -> i32;
        }
        ::core::mem::transmute(ComDBReleasePort(hcomdb.into_param().abi(), ::core::mem::transmute(comnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBResizeDatabase<'a, Param0: ::windows::core::IntoParam<'a, HCOMDB>>(hcomdb: Param0, newsize: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBResizeDatabase(hcomdb: HCOMDB, newsize: u32) -> i32;
        }
        ::core::mem::transmute(ComDBResizeDatabase(hcomdb.into_param().abi(), ::core::mem::transmute(newsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HCOMDB(pub isize);
impl ::core::default::Default for HCOMDB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HCOMDB {}
unsafe impl ::windows::core::Abi for HCOMDB {
    type Abi = Self;
}
