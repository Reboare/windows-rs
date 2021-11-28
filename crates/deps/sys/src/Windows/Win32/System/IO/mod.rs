#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type BindIoCompletionCallback = unsafe extern "system" fn(filehandle: super::super::Foundation::HANDLE, function: LPOVERLAPPED_COMPLETION_ROUTINE, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CancelIo = unsafe extern "system" fn(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CancelIoEx = unsafe extern "system" fn(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CancelSynchronousIo = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CreateIoCompletionPort = unsafe extern "system" fn(filehandle: super::super::Foundation::HANDLE, existingcompletionport: super::super::Foundation::HANDLE, completionkey: usize, numberofconcurrentthreads: u32) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type DeviceIoControl = unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, dwiocontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut OVERLAPPED) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetOverlappedResult = unsafe extern "system" fn(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetOverlappedResultEx = unsafe extern "system" fn(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetQueuedCompletionStatus = unsafe extern "system" fn(completionport: super::super::Foundation::HANDLE, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetQueuedCompletionStatusEx = unsafe extern "system" fn(completionport: super::super::Foundation::HANDLE, lpcompletionportentries: *mut OVERLAPPED_ENTRY, ulcount: u32, ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PostQueuedCompletionStatus = unsafe extern "system" fn(completionport: super::super::Foundation::HANDLE, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: *const OVERLAPPED) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPOVERLAPPED_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(dwerrorcode: u32, dwnumberofbytestransfered: u32, lpoverlapped: *mut OVERLAPPED)>;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OVERLAPPED_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OVERLAPPED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OVERLAPPED_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OVERLAPPED_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: usize,
    pub lpOverlapped: *mut OVERLAPPED,
    pub Internal: usize,
    pub dwNumberOfBytesTransferred: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OVERLAPPED_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OVERLAPPED_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
