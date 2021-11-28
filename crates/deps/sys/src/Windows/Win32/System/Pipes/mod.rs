#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type CallNamedPipeA = unsafe extern "system" fn(lpnamedpipename: super::super::Foundation::PSTR, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CallNamedPipeW = unsafe extern "system" fn(lpnamedpipename: super::super::Foundation::PWSTR, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type ConnectNamedPipe = unsafe extern "system" fn(hnamedpipe: super::super::Foundation::HANDLE, lpoverlapped: *mut super::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
pub type CreateNamedPipeA = unsafe extern "system" fn(lpname: super::super::Foundation::PSTR, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
pub type CreateNamedPipeW = unsafe extern "system" fn(lpname: super::super::Foundation::PWSTR, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreatePipe = unsafe extern "system" fn(hreadpipe: *mut super::super::Foundation::HANDLE, hwritepipe: *mut super::super::Foundation::HANDLE, lppipeattributes: *const super::super::Security::SECURITY_ATTRIBUTES, nsize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DisconnectNamedPipe = unsafe extern "system" fn(hnamedpipe: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNamedPipeClientComputerNameA = unsafe extern "system" fn(pipe: super::super::Foundation::HANDLE, clientcomputername: super::super::Foundation::PSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNamedPipeClientComputerNameW = unsafe extern "system" fn(pipe: super::super::Foundation::HANDLE, clientcomputername: super::super::Foundation::PWSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNamedPipeClientProcessId = unsafe extern "system" fn(pipe: super::super::Foundation::HANDLE, clientprocessid: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNamedPipeClientSessionId = unsafe extern "system" fn(pipe: super::super::Foundation::HANDLE, clientsessionid: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNamedPipeHandleStateA = unsafe extern "system" fn(hnamedpipe: super::super::Foundation::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: super::super::Foundation::PSTR, nmaxusernamesize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNamedPipeHandleStateW = unsafe extern "system" fn(hnamedpipe: super::super::Foundation::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: super::super::Foundation::PWSTR, nmaxusernamesize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNamedPipeInfo = unsafe extern "system" fn(hnamedpipe: super::super::Foundation::HANDLE, lpflags: *mut NAMED_PIPE_MODE, lpoutbuffersize: *mut u32, lpinbuffersize: *mut u32, lpmaxinstances: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNamedPipeServerProcessId = unsafe extern "system" fn(pipe: super::super::Foundation::HANDLE, serverprocessid: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNamedPipeServerSessionId = unsafe extern "system" fn(pipe: super::super::Foundation::HANDLE, serversessionid: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ImpersonateNamedPipeClient = unsafe extern "system" fn(hnamedpipe: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PeekNamedPipe = unsafe extern "system" fn(hnamedpipe: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nbuffersize: u32, lpbytesread: *mut u32, lptotalbytesavail: *mut u32, lpbytesleftthismessage: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetNamedPipeHandleState = unsafe extern "system" fn(hnamedpipe: super::super::Foundation::HANDLE, lpmode: *const NAMED_PIPE_MODE, lpmaxcollectioncount: *const u32, lpcollectdatatimeout: *const u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type TransactNamedPipe = unsafe extern "system" fn(hnamedpipe: super::super::Foundation::HANDLE, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, lpoverlapped: *mut super::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type WaitNamedPipeA = unsafe extern "system" fn(lpnamedpipename: super::super::Foundation::PSTR, ntimeout: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type WaitNamedPipeW = unsafe extern "system" fn(lpnamedpipename: super::super::Foundation::PWSTR, ntimeout: u32) -> super::super::Foundation::BOOL;
pub type NAMED_PIPE_MODE = u32;
pub const PIPE_WAIT: NAMED_PIPE_MODE = 0u32;
pub const PIPE_NOWAIT: NAMED_PIPE_MODE = 1u32;
pub const PIPE_READMODE_BYTE: NAMED_PIPE_MODE = 0u32;
pub const PIPE_READMODE_MESSAGE: NAMED_PIPE_MODE = 2u32;
pub const PIPE_CLIENT_END: NAMED_PIPE_MODE = 0u32;
pub const PIPE_SERVER_END: NAMED_PIPE_MODE = 1u32;
pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = 0u32;
pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_MODE = 4u32;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 0u32;
pub const PIPE_REJECT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 8u32;
pub const NMPWAIT_NOWAIT: u32 = 1u32;
pub const NMPWAIT_USE_DEFAULT_WAIT: u32 = 0u32;
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295u32;
pub const PIPE_UNLIMITED_INSTANCES: u32 = 255u32;
