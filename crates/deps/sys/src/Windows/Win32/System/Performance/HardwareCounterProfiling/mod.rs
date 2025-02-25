#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type DisableThreadProfiling = unsafe extern "system" fn(performancedatahandle: super::super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type EnableThreadProfiling = unsafe extern "system" fn(threadhandle: super::super::super::Foundation::HANDLE, flags: u32, hardwarecounters: u64, performancedatahandle: *mut super::super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type QueryThreadProfiling = unsafe extern "system" fn(threadhandle: super::super::super::Foundation::HANDLE, enabled: *mut super::super::super::Foundation::BOOLEAN) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type ReadThreadProfilingData = unsafe extern "system" fn(performancedatahandle: super::super::super::Foundation::HANDLE, flags: u32, performancedata: *mut PERFORMANCE_DATA) -> u32;
#[repr(C)]
pub struct HARDWARE_COUNTER_DATA {
    pub Type: HARDWARE_COUNTER_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
impl ::core::marker::Copy for HARDWARE_COUNTER_DATA {}
impl ::core::clone::Clone for HARDWARE_COUNTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HARDWARE_COUNTER_TYPE = i32;
pub const PMCCounter: HARDWARE_COUNTER_TYPE = 0i32;
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = 1i32;
#[repr(C)]
pub struct PERFORMANCE_DATA {
    pub Size: u16,
    pub Version: u8,
    pub HwCountersCount: u8,
    pub ContextSwitchCount: u32,
    pub WaitReasonBitMap: u64,
    pub CycleTime: u64,
    pub RetryCount: u32,
    pub Reserved: u32,
    pub HwCounters: [HARDWARE_COUNTER_DATA; 16],
}
impl ::core::marker::Copy for PERFORMANCE_DATA {}
impl ::core::clone::Clone for PERFORMANCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
