#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type AcquireSRWLockExclusive = unsafe extern "system" fn(srwlock: *mut RTL_SRWLOCK);
pub type AcquireSRWLockShared = unsafe extern "system" fn(srwlock: *mut RTL_SRWLOCK);
#[cfg(feature = "Win32_Foundation")]
pub type AddIntegrityLabelToBoundaryDescriptor = unsafe extern "system" fn(boundarydescriptor: *mut super::super::Foundation::HANDLE, integritylabel: super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type AddSIDToBoundaryDescriptor = unsafe extern "system" fn(boundarydescriptor: *mut super::super::Foundation::HANDLE, requiredsid: super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type AttachThreadInput = unsafe extern "system" fn(idattach: u32, idattachto: u32, fattach: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CallbackMayRunLong = unsafe extern "system" fn(pci: *mut TP_CALLBACK_INSTANCE) -> super::super::Foundation::BOOL;
pub type CancelThreadpoolIo = unsafe extern "system" fn(pio: *mut TP_IO);
#[cfg(feature = "Win32_Foundation")]
pub type CancelWaitableTimer = unsafe extern "system" fn(htimer: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ChangeTimerQueueTimer = unsafe extern "system" fn(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ClosePrivateNamespace = unsafe extern "system" fn(handle: NamespaceHandle, flags: u32) -> super::super::Foundation::BOOLEAN;
pub type CloseThreadpool = unsafe extern "system" fn(ptpp: PTP_POOL);
pub type CloseThreadpoolCleanupGroup = unsafe extern "system" fn(ptpcg: isize);
#[cfg(feature = "Win32_Foundation")]
pub type CloseThreadpoolCleanupGroupMembers = unsafe extern "system" fn(ptpcg: isize, fcancelpendingcallbacks: super::super::Foundation::BOOL, pvcleanupcontext: *mut ::core::ffi::c_void);
pub type CloseThreadpoolIo = unsafe extern "system" fn(pio: *mut TP_IO);
pub type CloseThreadpoolTimer = unsafe extern "system" fn(pti: *mut TP_TIMER);
pub type CloseThreadpoolWait = unsafe extern "system" fn(pwa: *mut TP_WAIT);
pub type CloseThreadpoolWork = unsafe extern "system" fn(pwk: *mut TP_WORK);
#[cfg(feature = "Win32_Foundation")]
pub type ConvertFiberToThread = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
pub type ConvertThreadToFiber = unsafe extern "system" fn(lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
pub type ConvertThreadToFiberEx = unsafe extern "system" fn(lpparameter: *const ::core::ffi::c_void, dwflags: u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type CreateBoundaryDescriptorA = unsafe extern "system" fn(name: super::super::Foundation::PSTR, flags: u32) -> BoundaryDescriptorHandle;
#[cfg(feature = "Win32_Foundation")]
pub type CreateBoundaryDescriptorW = unsafe extern "system" fn(name: super::super::Foundation::PWSTR, flags: u32) -> BoundaryDescriptorHandle;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateEventA = unsafe extern "system" fn(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, binitialstate: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateEventExA = unsafe extern "system" fn(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateEventExW = unsafe extern "system" fn(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PWSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateEventW = unsafe extern "system" fn(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, binitialstate: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
pub type CreateFiber = unsafe extern "system" fn(dwstacksize: usize, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
pub type CreateFiberEx = unsafe extern "system" fn(dwstackcommitsize: usize, dwstackreservesize: usize, dwflags: u32, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateMutexA = unsafe extern "system" fn(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateMutexExA = unsafe extern "system" fn(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateMutexExW = unsafe extern "system" fn(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateMutexW = unsafe extern "system" fn(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreatePrivateNamespaceA = unsafe extern "system" fn(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PSTR) -> NamespaceHandle;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreatePrivateNamespaceW = unsafe extern "system" fn(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PWSTR) -> NamespaceHandle;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateProcessA = unsafe extern "system" fn(
    lpapplicationname: super::super::Foundation::PSTR,
    lpcommandline: super::super::Foundation::PSTR,
    lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binherithandles: super::super::Foundation::BOOL,
    dwcreationflags: PROCESS_CREATION_FLAGS,
    lpenvironment: *const ::core::ffi::c_void,
    lpcurrentdirectory: super::super::Foundation::PSTR,
    lpstartupinfo: *const STARTUPINFOA,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateProcessAsUserA = unsafe extern "system" fn(
    htoken: super::super::Foundation::HANDLE,
    lpapplicationname: super::super::Foundation::PSTR,
    lpcommandline: super::super::Foundation::PSTR,
    lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binherithandles: super::super::Foundation::BOOL,
    dwcreationflags: u32,
    lpenvironment: *const ::core::ffi::c_void,
    lpcurrentdirectory: super::super::Foundation::PSTR,
    lpstartupinfo: *const STARTUPINFOA,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateProcessAsUserW = unsafe extern "system" fn(
    htoken: super::super::Foundation::HANDLE,
    lpapplicationname: super::super::Foundation::PWSTR,
    lpcommandline: super::super::Foundation::PWSTR,
    lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binherithandles: super::super::Foundation::BOOL,
    dwcreationflags: u32,
    lpenvironment: *const ::core::ffi::c_void,
    lpcurrentdirectory: super::super::Foundation::PWSTR,
    lpstartupinfo: *const STARTUPINFOW,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateProcessW = unsafe extern "system" fn(
    lpapplicationname: super::super::Foundation::PWSTR,
    lpcommandline: super::super::Foundation::PWSTR,
    lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binherithandles: super::super::Foundation::BOOL,
    dwcreationflags: PROCESS_CREATION_FLAGS,
    lpenvironment: *const ::core::ffi::c_void,
    lpcurrentdirectory: super::super::Foundation::PWSTR,
    lpstartupinfo: *const STARTUPINFOW,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CreateProcessWithLogonW = unsafe extern "system" fn(
    lpusername: super::super::Foundation::PWSTR,
    lpdomain: super::super::Foundation::PWSTR,
    lppassword: super::super::Foundation::PWSTR,
    dwlogonflags: CREATE_PROCESS_LOGON_FLAGS,
    lpapplicationname: super::super::Foundation::PWSTR,
    lpcommandline: super::super::Foundation::PWSTR,
    dwcreationflags: u32,
    lpenvironment: *const ::core::ffi::c_void,
    lpcurrentdirectory: super::super::Foundation::PWSTR,
    lpstartupinfo: *const STARTUPINFOW,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CreateProcessWithTokenW = unsafe extern "system" fn(htoken: super::super::Foundation::HANDLE, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: super::super::Foundation::PWSTR, lpcommandline: super::super::Foundation::PWSTR, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: super::super::Foundation::PWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateRemoteThread = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateRemoteThreadEx = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateSemaphoreA = unsafe extern "system" fn(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateSemaphoreExA = unsafe extern "system" fn(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateSemaphoreExW = unsafe extern "system" fn(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateSemaphoreW = unsafe extern "system" fn(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateThread = unsafe extern "system" fn(lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: THREAD_CREATION_FLAGS, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
pub type CreateThreadpool = unsafe extern "system" fn(reserved: *mut ::core::ffi::c_void) -> PTP_POOL;
pub type CreateThreadpoolCleanupGroup = unsafe extern "system" fn() -> isize;
#[cfg(feature = "Win32_Foundation")]
pub type CreateThreadpoolIo = unsafe extern "system" fn(fl: super::super::Foundation::HANDLE, pfnio: PTP_WIN32_IO_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_IO;
pub type CreateThreadpoolTimer = unsafe extern "system" fn(pfnti: PTP_TIMER_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_TIMER;
pub type CreateThreadpoolWait = unsafe extern "system" fn(pfnwa: PTP_WAIT_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WAIT;
pub type CreateThreadpoolWork = unsafe extern "system" fn(pfnwk: PTP_WORK_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WORK;
#[cfg(feature = "Win32_Foundation")]
pub type CreateTimerQueue = unsafe extern "system" fn() -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type CreateTimerQueueTimer = unsafe extern "system" fn(phnewtimer: *mut super::super::Foundation::HANDLE, timerqueue: super::super::Foundation::HANDLE, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CreateUmsCompletionList = unsafe extern "system" fn(umscompletionlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CreateUmsThreadContext = unsafe extern "system" fn(lpumsthread: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateWaitableTimerExW = unsafe extern "system" fn(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CreateWaitableTimerW = unsafe extern "system" fn(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
pub type DeleteBoundaryDescriptor = unsafe extern "system" fn(boundarydescriptor: BoundaryDescriptorHandle);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type DeleteCriticalSection = unsafe extern "system" fn(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
pub type DeleteFiber = unsafe extern "system" fn(lpfiber: *const ::core::ffi::c_void);
pub type DeleteProcThreadAttributeList = unsafe extern "system" fn(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST);
#[cfg(feature = "Win32_Foundation")]
pub type DeleteSynchronizationBarrier = unsafe extern "system" fn(lpbarrier: *mut RTL_BARRIER) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DeleteTimerQueue = unsafe extern "system" fn(timerqueue: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DeleteTimerQueueEx = unsafe extern "system" fn(timerqueue: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DeleteTimerQueueTimer = unsafe extern "system" fn(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DeleteUmsCompletionList = unsafe extern "system" fn(umscompletionlist: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DeleteUmsThreadContext = unsafe extern "system" fn(umsthread: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DequeueUmsCompletionListItems = unsafe extern "system" fn(umscompletionlist: *const ::core::ffi::c_void, waittimeout: u32, umsthreadlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type DisassociateCurrentThreadFromCallback = unsafe extern "system" fn(pci: *mut TP_CALLBACK_INSTANCE);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type EnterCriticalSection = unsafe extern "system" fn(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
#[cfg(feature = "Win32_Foundation")]
pub type EnterSynchronizationBarrier = unsafe extern "system" fn(lpbarrier: *mut RTL_BARRIER, dwflags: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type EnterUmsSchedulingMode = unsafe extern "system" fn(schedulerstartupinfo: *const UMS_SCHEDULER_STARTUP_INFO) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ExecuteUmsThread = unsafe extern "system" fn(umsthread: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type ExitProcess = unsafe extern "system" fn(uexitcode: u32);
pub type ExitThread = unsafe extern "system" fn(dwexitcode: u32);
pub type FlsAlloc = unsafe extern "system" fn(lpcallback: PFLS_CALLBACK_FUNCTION) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type FlsFree = unsafe extern "system" fn(dwflsindex: u32) -> super::super::Foundation::BOOL;
pub type FlsGetValue = unsafe extern "system" fn(dwflsindex: u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type FlsSetValue = unsafe extern "system" fn(dwflsindex: u32, lpflsdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type FlushProcessWriteBuffers = unsafe extern "system" fn();
#[cfg(feature = "Win32_Foundation")]
pub type FreeLibraryWhenCallbackReturns = unsafe extern "system" fn(pci: *mut TP_CALLBACK_INSTANCE, r#mod: super::super::Foundation::HINSTANCE);
pub type GetActiveProcessorCount = unsafe extern "system" fn(groupnumber: u16) -> u32;
pub type GetActiveProcessorGroupCount = unsafe extern "system" fn() -> u16;
#[cfg(feature = "Win32_Foundation")]
pub type GetCurrentProcess = unsafe extern "system" fn() -> super::super::Foundation::HANDLE;
pub type GetCurrentProcessId = unsafe extern "system" fn() -> u32;
pub type GetCurrentProcessorNumber = unsafe extern "system" fn() -> u32;
#[cfg(feature = "Win32_System_Kernel")]
pub type GetCurrentProcessorNumberEx = unsafe extern "system" fn(procnumber: *mut super::Kernel::PROCESSOR_NUMBER);
#[cfg(feature = "Win32_Foundation")]
pub type GetCurrentThread = unsafe extern "system" fn() -> super::super::Foundation::HANDLE;
pub type GetCurrentThreadId = unsafe extern "system" fn() -> u32;
pub type GetCurrentThreadStackLimits = unsafe extern "system" fn(lowlimit: *mut usize, highlimit: *mut usize);
pub type GetCurrentUmsThread = unsafe extern "system" fn() -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type GetExitCodeProcess = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, lpexitcode: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetExitCodeThread = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, lpexitcode: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetGuiResources = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, uiflags: GET_GUI_RESOURCES_FLAGS) -> u32;
pub type GetMachineTypeAttributes = unsafe extern "system" fn(machine: u16, machinetypeattributes: *mut MACHINE_ATTRIBUTES) -> ::windows_sys::core::HRESULT;
pub type GetMaximumProcessorCount = unsafe extern "system" fn(groupnumber: u16) -> u32;
pub type GetMaximumProcessorGroupCount = unsafe extern "system" fn() -> u16;
pub type GetNextUmsListItem = unsafe extern "system" fn(umscontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type GetNumaAvailableMemoryNode = unsafe extern "system" fn(node: u8, availablebytes: *mut u64) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNumaAvailableMemoryNodeEx = unsafe extern "system" fn(node: u16, availablebytes: *mut u64) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNumaHighestNodeNumber = unsafe extern "system" fn(highestnodenumber: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNumaNodeNumberFromHandle = unsafe extern "system" fn(hfile: super::super::Foundation::HANDLE, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNumaNodeProcessorMask = unsafe extern "system" fn(node: u8, processormask: *mut u64) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
pub type GetNumaNodeProcessorMask2 = unsafe extern "system" fn(nodenumber: u16, processormasks: *mut super::SystemInformation::GROUP_AFFINITY, processormaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
pub type GetNumaNodeProcessorMaskEx = unsafe extern "system" fn(node: u16, processormask: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNumaProcessorNode = unsafe extern "system" fn(processor: u8, nodenumber: *mut u8) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type GetNumaProcessorNodeEx = unsafe extern "system" fn(processor: *const super::Kernel::PROCESSOR_NUMBER, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNumaProximityNode = unsafe extern "system" fn(proximityid: u32, nodenumber: *mut u8) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetNumaProximityNodeEx = unsafe extern "system" fn(proximityid: u32, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetPriorityClass = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessAffinityMask = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, lpprocessaffinitymask: *mut usize, lpsystemaffinitymask: *mut usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessDEPPolicy = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, lpflags: *mut u32, lppermanent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
pub type GetProcessDefaultCpuSetMasks = unsafe extern "system" fn(process: super::super::Foundation::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessDefaultCpuSets = unsafe extern "system" fn(process: super::super::Foundation::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessGroupAffinity = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, groupcount: *mut u16, grouparray: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessHandleCount = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, pdwhandlecount: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessId = unsafe extern "system" fn(process: super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessIdOfThread = unsafe extern "system" fn(thread: super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessInformation = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *mut ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessIoCounters = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, lpiocounters: *mut IO_COUNTERS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessMitigationPolicy = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessPriorityBoost = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessShutdownParameters = unsafe extern "system" fn(lpdwlevel: *mut u32, lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessTimes = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
pub type GetProcessVersion = unsafe extern "system" fn(processid: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type GetProcessWorkingSetSize = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetStartupInfoA = unsafe extern "system" fn(lpstartupinfo: *mut STARTUPINFOA);
#[cfg(feature = "Win32_Foundation")]
pub type GetStartupInfoW = unsafe extern "system" fn(lpstartupinfo: *mut STARTUPINFOW);
#[cfg(feature = "Win32_Foundation")]
pub type GetSystemTimes = unsafe extern "system" fn(lpidletime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetThreadDescription = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, ppszthreaddescription: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
pub type GetThreadGroupAffinity = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, groupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetThreadIOPendingFlag = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, lpioispending: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetThreadId = unsafe extern "system" fn(thread: super::super::Foundation::HANDLE) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type GetThreadIdealProcessorEx = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetThreadInformation = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetThreadPriority = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type GetThreadPriorityBoost = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
pub type GetThreadSelectedCpuSetMasks = unsafe extern "system" fn(thread: super::super::Foundation::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetThreadSelectedCpuSets = unsafe extern "system" fn(thread: super::super::Foundation::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetThreadTimes = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetUmsCompletionListEvent = unsafe extern "system" fn(umscompletionlist: *const ::core::ffi::c_void, umscompletionevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GetUmsSystemThreadInformation = unsafe extern "system" fn(threadhandle: super::super::Foundation::HANDLE, systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type InitOnceBeginInitialize = unsafe extern "system" fn(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, fpending: *mut super::super::Foundation::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type InitOnceComplete = unsafe extern "system" fn(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, lpcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type InitOnceExecuteOnce = unsafe extern "system" fn(initonce: *mut RTL_RUN_ONCE, initfn: PINIT_ONCE_FN, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type InitOnceInitialize = unsafe extern "system" fn(initonce: *mut RTL_RUN_ONCE);
pub type InitializeConditionVariable = unsafe extern "system" fn(conditionvariable: *mut RTL_CONDITION_VARIABLE);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type InitializeCriticalSection = unsafe extern "system" fn(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type InitializeCriticalSectionAndSpinCount = unsafe extern "system" fn(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type InitializeCriticalSectionEx = unsafe extern "system" fn(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type InitializeProcThreadAttributeList = unsafe extern "system" fn(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount: u32, dwflags: u32, lpsize: *mut usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_System_Kernel")]
pub type InitializeSListHead = unsafe extern "system" fn(listhead: *mut super::Kernel::SLIST_HEADER);
pub type InitializeSRWLock = unsafe extern "system" fn(srwlock: *mut RTL_SRWLOCK);
#[cfg(feature = "Win32_Foundation")]
pub type InitializeSynchronizationBarrier = unsafe extern "system" fn(lpbarrier: *mut RTL_BARRIER, ltotalthreads: i32, lspincount: i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_System_Kernel")]
pub type InterlockedFlushSList = unsafe extern "system" fn(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
#[cfg(feature = "Win32_System_Kernel")]
pub type InterlockedPopEntrySList = unsafe extern "system" fn(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
#[cfg(feature = "Win32_System_Kernel")]
pub type InterlockedPushEntrySList = unsafe extern "system" fn(listhead: *mut super::Kernel::SLIST_HEADER, listentry: *mut super::Kernel::SLIST_ENTRY) -> *mut super::Kernel::SLIST_ENTRY;
#[cfg(feature = "Win32_System_Kernel")]
pub type InterlockedPushListSListEx = unsafe extern "system" fn(listhead: *mut super::Kernel::SLIST_HEADER, list: *mut super::Kernel::SLIST_ENTRY, listend: *mut super::Kernel::SLIST_ENTRY, count: u32) -> *mut super::Kernel::SLIST_ENTRY;
#[cfg(feature = "Win32_Foundation")]
pub type IsImmersiveProcess = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type IsProcessCritical = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, critical: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type IsProcessorFeaturePresent = unsafe extern "system" fn(processorfeature: PROCESSOR_FEATURE_ID) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type IsThreadAFiber = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type IsThreadpoolTimerSet = unsafe extern "system" fn(pti: *mut TP_TIMER) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type IsWow64Process = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, wow64process: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type IsWow64Process2 = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, pprocessmachine: *mut u16, pnativemachine: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type LeaveCriticalSection = unsafe extern "system" fn(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type LeaveCriticalSectionWhenCallbackReturns = unsafe extern "system" fn(pci: *mut TP_CALLBACK_INSTANCE, pcs: *mut RTL_CRITICAL_SECTION);
#[cfg(feature = "Win32_Foundation")]
pub type NtQueryInformationProcess = unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, processinformationclass: PROCESSINFOCLASS, processinformation: *mut ::core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
#[cfg(feature = "Win32_Foundation")]
pub type NtQueryInformationThread = unsafe extern "system" fn(threadhandle: super::super::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
#[cfg(feature = "Win32_Foundation")]
pub type NtSetInformationThread = unsafe extern "system" fn(threadhandle: super::super::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *const ::core::ffi::c_void, threadinformationlength: u32) -> super::super::Foundation::NTSTATUS;
#[cfg(feature = "Win32_Foundation")]
pub type OpenEventA = unsafe extern "system" fn(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type OpenEventW = unsafe extern "system" fn(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type OpenMutexW = unsafe extern "system" fn(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type OpenPrivateNamespaceA = unsafe extern "system" fn(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PSTR) -> NamespaceHandle;
#[cfg(feature = "Win32_Foundation")]
pub type OpenPrivateNamespaceW = unsafe extern "system" fn(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PWSTR) -> NamespaceHandle;
#[cfg(feature = "Win32_Foundation")]
pub type OpenProcess = unsafe extern "system" fn(dwdesiredaccess: PROCESS_ACCESS_RIGHTS, binherithandle: super::super::Foundation::BOOL, dwprocessid: u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type OpenProcessToken = unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type OpenSemaphoreW = unsafe extern "system" fn(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type OpenThread = unsafe extern "system" fn(dwdesiredaccess: THREAD_ACCESS_RIGHTS, binherithandle: super::super::Foundation::BOOL, dwthreadid: u32) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type OpenThreadToken = unsafe extern "system" fn(threadhandle: super::super::Foundation::HANDLE, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, openasself: super::super::Foundation::BOOL, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type OpenWaitableTimerW = unsafe extern "system" fn(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type PulseEvent = unsafe extern "system" fn(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_System_Kernel")]
pub type QueryDepthSList = unsafe extern "system" fn(listhead: *const super::Kernel::SLIST_HEADER) -> u16;
#[cfg(feature = "Win32_Foundation")]
pub type QueryFullProcessImageNameA = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: super::super::Foundation::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type QueryFullProcessImageNameW = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: super::super::Foundation::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type QueryProcessAffinityUpdateMode = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, lpdwflags: *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type QueryProtectedPolicy = unsafe extern "system" fn(policyguid: *const ::windows_sys::core::GUID, policyvalue: *mut usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type QueryThreadpoolStackInformation = unsafe extern "system" fn(ptpp: PTP_POOL, ptpsi: *mut TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type QueryUmsThreadInformation = unsafe extern "system" fn(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *mut ::core::ffi::c_void, umsthreadinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type QueueUserAPC = unsafe extern "system" fn(pfnapc: super::super::Foundation::PAPCFUNC, hthread: super::super::Foundation::HANDLE, dwdata: usize) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type QueueUserAPC2 = unsafe extern "system" fn(apcroutine: super::super::Foundation::PAPCFUNC, thread: super::super::Foundation::HANDLE, data: usize, flags: QUEUE_USER_APC_FLAGS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type QueueUserWorkItem = unsafe extern "system" fn(function: LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type RegisterWaitForSingleObject = unsafe extern "system" fn(phnewwaitobject: *mut super::super::Foundation::HANDLE, hobject: super::super::Foundation::HANDLE, callback: WAITORTIMERCALLBACK, context: *const ::core::ffi::c_void, dwmilliseconds: u32, dwflags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ReleaseMutex = unsafe extern "system" fn(hmutex: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ReleaseMutexWhenCallbackReturns = unsafe extern "system" fn(pci: *mut TP_CALLBACK_INSTANCE, r#mut: super::super::Foundation::HANDLE);
pub type ReleaseSRWLockExclusive = unsafe extern "system" fn(srwlock: *mut RTL_SRWLOCK);
pub type ReleaseSRWLockShared = unsafe extern "system" fn(srwlock: *mut RTL_SRWLOCK);
#[cfg(feature = "Win32_Foundation")]
pub type ReleaseSemaphore = unsafe extern "system" fn(hsemaphore: super::super::Foundation::HANDLE, lreleasecount: i32, lppreviouscount: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ReleaseSemaphoreWhenCallbackReturns = unsafe extern "system" fn(pci: *mut TP_CALLBACK_INSTANCE, sem: super::super::Foundation::HANDLE, crel: u32);
#[cfg(feature = "Win32_Foundation")]
pub type ResetEvent = unsafe extern "system" fn(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ResumeThread = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type SetCriticalSectionSpinCount = unsafe extern "system" fn(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetEvent = unsafe extern "system" fn(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetEventWhenCallbackReturns = unsafe extern "system" fn(pci: *mut TP_CALLBACK_INSTANCE, evt: super::super::Foundation::HANDLE);
#[cfg(feature = "Win32_Foundation")]
pub type SetPriorityClass = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, dwpriorityclass: PROCESS_CREATION_FLAGS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessAffinityMask = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, dwprocessaffinitymask: usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessAffinityUpdateMode = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessDEPPolicy = unsafe extern "system" fn(dwflags: PROCESS_DEP_FLAGS) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
pub type SetProcessDefaultCpuSetMasks = unsafe extern "system" fn(process: super::super::Foundation::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessDefaultCpuSets = unsafe extern "system" fn(process: super::super::Foundation::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessDynamicEHContinuationTargets = unsafe extern "system" fn(process: super::super::Foundation::HANDLE, numberoftargets: u16, targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessDynamicEnforcedCetCompatibleRanges = unsafe extern "system" fn(process: super::super::Foundation::HANDLE, numberofranges: u16, ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessInformation = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *const ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessMitigationPolicy = unsafe extern "system" fn(mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *const ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessPriorityBoost = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, bdisablepriorityboost: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessRestrictionExemption = unsafe extern "system" fn(fenableexemption: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessShutdownParameters = unsafe extern "system" fn(dwlevel: u32, dwflags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProcessWorkingSetSize = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetProtectedPolicy = unsafe extern "system" fn(policyguid: *const ::windows_sys::core::GUID, policyvalue: usize, oldpolicyvalue: *mut usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadAffinityMask = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, dwthreadaffinitymask: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadDescription = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, lpthreaddescription: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
pub type SetThreadGroupAffinity = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, groupaffinity: *const super::SystemInformation::GROUP_AFFINITY, previousgroupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadIdealProcessor = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, dwidealprocessor: u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type SetThreadIdealProcessorEx = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER, lppreviousidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadInformation = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *const ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadPriority = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, npriority: THREAD_PRIORITY) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadPriorityBoost = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, bdisablepriorityboost: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
pub type SetThreadSelectedCpuSetMasks = unsafe extern "system" fn(thread: super::super::Foundation::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadSelectedCpuSets = unsafe extern "system" fn(thread: super::super::Foundation::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadStackGuarantee = unsafe extern "system" fn(stacksizeinbytes: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadToken = unsafe extern "system" fn(thread: *const super::super::Foundation::HANDLE, token: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadpoolStackInformation = unsafe extern "system" fn(ptpp: PTP_POOL, ptpsi: *const TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL;
pub type SetThreadpoolThreadMaximum = unsafe extern "system" fn(ptpp: PTP_POOL, cthrdmost: u32);
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadpoolThreadMinimum = unsafe extern "system" fn(ptpp: PTP_POOL, cthrdmic: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadpoolTimer = unsafe extern "system" fn(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32);
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadpoolTimerEx = unsafe extern "system" fn(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadpoolWait = unsafe extern "system" fn(pwa: *mut TP_WAIT, h: super::super::Foundation::HANDLE, pfttimeout: *const super::super::Foundation::FILETIME);
#[cfg(feature = "Win32_Foundation")]
pub type SetThreadpoolWaitEx = unsafe extern "system" fn(pwa: *mut TP_WAIT, h: super::super::Foundation::HANDLE, pfttimeout: *const super::super::Foundation::FILETIME, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetTimerQueueTimer = unsafe extern "system" fn(timerqueue: super::super::Foundation::HANDLE, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, preferio: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type SetUmsThreadInformation = unsafe extern "system" fn(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *const ::core::ffi::c_void, umsthreadinformationlength: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetWaitableTimer = unsafe extern "system" fn(htimer: super::super::Foundation::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, fresume: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetWaitableTimerEx = unsafe extern "system" fn(htimer: super::super::Foundation::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, wakecontext: *const REASON_CONTEXT, tolerabledelay: u32) -> super::super::Foundation::BOOL;
pub type Sleep = unsafe extern "system" fn(dwmilliseconds: u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type SleepConditionVariableCS = unsafe extern "system" fn(conditionvariable: *mut RTL_CONDITION_VARIABLE, criticalsection: *mut RTL_CRITICAL_SECTION, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SleepConditionVariableSRW = unsafe extern "system" fn(conditionvariable: *mut RTL_CONDITION_VARIABLE, srwlock: *mut RTL_SRWLOCK, dwmilliseconds: u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SleepEx = unsafe extern "system" fn(dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
pub type StartThreadpoolIo = unsafe extern "system" fn(pio: *mut TP_IO);
pub type SubmitThreadpoolWork = unsafe extern "system" fn(pwk: *mut TP_WORK);
#[cfg(feature = "Win32_Foundation")]
pub type SuspendThread = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE) -> u32;
pub type SwitchToFiber = unsafe extern "system" fn(lpfiber: *const ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type SwitchToThread = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type TerminateProcess = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, uexitcode: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type TerminateThread = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE, dwexitcode: u32) -> super::super::Foundation::BOOL;
pub type TlsAlloc = unsafe extern "system" fn() -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type TlsFree = unsafe extern "system" fn(dwtlsindex: u32) -> super::super::Foundation::BOOL;
pub type TlsGetValue = unsafe extern "system" fn(dwtlsindex: u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type TlsSetValue = unsafe extern "system" fn(dwtlsindex: u32, lptlsvalue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type TryAcquireSRWLockExclusive = unsafe extern "system" fn(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN;
#[cfg(feature = "Win32_Foundation")]
pub type TryAcquireSRWLockShared = unsafe extern "system" fn(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type TryEnterCriticalSection = unsafe extern "system" fn(lpcriticalsection: *mut RTL_CRITICAL_SECTION) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type TrySubmitThreadpoolCallback = unsafe extern "system" fn(pfns: PTP_SIMPLE_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type UmsThreadYield = unsafe extern "system" fn(schedulerparam: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type UnregisterWait = unsafe extern "system" fn(waithandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type UnregisterWaitEx = unsafe extern "system" fn(waithandle: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type UpdateProcThreadAttribute = unsafe extern "system" fn(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwflags: u32, attribute: usize, lpvalue: *const ::core::ffi::c_void, cbsize: usize, lppreviousvalue: *mut ::core::ffi::c_void, lpreturnsize: *const usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type WaitForInputIdle = unsafe extern "system" fn(hprocess: super::super::Foundation::HANDLE, dwmilliseconds: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type WaitForMultipleObjects = unsafe extern "system" fn(ncount: u32, lphandles: *const super::super::Foundation::HANDLE, bwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type WaitForMultipleObjectsEx = unsafe extern "system" fn(ncount: u32, lphandles: *const super::super::Foundation::HANDLE, bwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type WaitForSingleObject = unsafe extern "system" fn(hhandle: super::super::Foundation::HANDLE, dwmilliseconds: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type WaitForSingleObjectEx = unsafe extern "system" fn(hhandle: super::super::Foundation::HANDLE, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type WaitForThreadpoolIoCallbacks = unsafe extern "system" fn(pio: *mut TP_IO, fcancelpendingcallbacks: super::super::Foundation::BOOL);
#[cfg(feature = "Win32_Foundation")]
pub type WaitForThreadpoolTimerCallbacks = unsafe extern "system" fn(pti: *mut TP_TIMER, fcancelpendingcallbacks: super::super::Foundation::BOOL);
#[cfg(feature = "Win32_Foundation")]
pub type WaitForThreadpoolWaitCallbacks = unsafe extern "system" fn(pwa: *mut TP_WAIT, fcancelpendingcallbacks: super::super::Foundation::BOOL);
#[cfg(feature = "Win32_Foundation")]
pub type WaitForThreadpoolWorkCallbacks = unsafe extern "system" fn(pwk: *mut TP_WORK, fcancelpendingcallbacks: super::super::Foundation::BOOL);
#[cfg(feature = "Win32_Foundation")]
pub type WaitOnAddress = unsafe extern "system" fn(address: *const ::core::ffi::c_void, compareaddress: *const ::core::ffi::c_void, addresssize: usize, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
pub type WakeAllConditionVariable = unsafe extern "system" fn(conditionvariable: *mut RTL_CONDITION_VARIABLE);
pub type WakeByAddressAll = unsafe extern "system" fn(address: *const ::core::ffi::c_void);
pub type WakeByAddressSingle = unsafe extern "system" fn(address: *const ::core::ffi::c_void);
pub type WakeConditionVariable = unsafe extern "system" fn(conditionvariable: *mut RTL_CONDITION_VARIABLE);
#[cfg(feature = "Win32_Foundation")]
pub type WinExec = unsafe extern "system" fn(lpcmdline: super::super::Foundation::PSTR, ucmdshow: u32) -> u32;
pub type Wow64SetThreadDefaultGuestMachine = unsafe extern "system" fn(machine: u16) -> u16;
#[cfg(feature = "Win32_Foundation")]
pub type Wow64SuspendThread = unsafe extern "system" fn(hthread: super::super::Foundation::HANDLE) -> u32;
#[repr(C)]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
impl ::core::marker::Copy for APP_MEMORY_INFORMATION {}
impl ::core::clone::Clone for APP_MEMORY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BoundaryDescriptorHandle = isize;
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
pub type CREATE_EVENT = u32;
pub const CREATE_EVENT_INITIAL_SET: CREATE_EVENT = 2u32;
pub const CREATE_EVENT_MANUAL_RESET: CREATE_EVENT = 1u32;
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
pub type CREATE_PROCESS_LOGON_FLAGS = u32;
pub const LOGON_WITH_PROFILE: CREATE_PROCESS_LOGON_FLAGS = 1u32;
pub const LOGON_NETCREDENTIALS_ONLY: CREATE_PROCESS_LOGON_FLAGS = 2u32;
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
pub type GET_GUI_RESOURCES_FLAGS = u32;
pub const GR_GDIOBJECTS: GET_GUI_RESOURCES_FLAGS = 0u32;
pub const GR_GDIOBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = 2u32;
pub const GR_USEROBJECTS: GET_GUI_RESOURCES_FLAGS = 1u32;
pub const GR_USEROBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = 4u32;
pub const INIT_ONCE_ASYNC: u32 = 2u32;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
#[repr(C)]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
impl ::core::marker::Copy for IO_COUNTERS {}
impl ::core::clone::Clone for IO_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LPFIBER_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpfiberparameter: *mut ::core::ffi::c_void)>;
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut ::core::ffi::c_void;
pub type LPTHREAD_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpthreadparameter: *mut ::core::ffi::c_void) -> u32>;
pub type MACHINE_ATTRIBUTES = u32;
pub const UserEnabled: MACHINE_ATTRIBUTES = 1u32;
pub const KernelEnabled: MACHINE_ATTRIBUTES = 2u32;
pub const Wow64Container: MACHINE_ATTRIBUTES = 4u32;
pub type MEMORY_PRIORITY = u32;
pub const MEMORY_PRIORITY_VERY_LOW: MEMORY_PRIORITY = 1u32;
pub const MEMORY_PRIORITY_LOW: MEMORY_PRIORITY = 2u32;
pub const MEMORY_PRIORITY_MEDIUM: MEMORY_PRIORITY = 3u32;
pub const MEMORY_PRIORITY_BELOW_NORMAL: MEMORY_PRIORITY = 4u32;
pub const MEMORY_PRIORITY_NORMAL: MEMORY_PRIORITY = 5u32;
#[repr(C)]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: MEMORY_PRIORITY,
}
impl ::core::marker::Copy for MEMORY_PRIORITY_INFORMATION {}
impl ::core::clone::Clone for MEMORY_PRIORITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MUTEX_MODIFY_STATE: u32 = 1u32;
pub type NamespaceHandle = isize;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut ::core::ffi::c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut ::core::ffi::c_void; 3],
    pub AtlThunkSListPtr: *mut ::core::ffi::c_void,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut ::core::ffi::c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut ::core::ffi::c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut ::core::ffi::c_void; 1],
    pub SessionId: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PEB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PEB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut ::core::ffi::c_void; 3],
    pub InMemoryOrderModuleList: super::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for PEB_LDR_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for PEB_LDR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PFLS_CALLBACK_FUNCTION = ::core::option::Option<unsafe extern "system" fn(lpflsdata: *const ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
pub type PINIT_ONCE_FN = ::core::option::Option<unsafe extern "system" fn(initonce: *mut RTL_RUN_ONCE, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub const PME_CURRENT_VERSION: u32 = 1u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
pub type POWER_REQUEST_CONTEXT_FLAGS = u32;
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: POWER_REQUEST_CONTEXT_FLAGS = 2u32;
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: POWER_REQUEST_CONTEXT_FLAGS = 1u32;
pub type PPS_POST_PROCESS_INIT_ROUTINE = ::core::option::Option<unsafe extern "system" fn()>;
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
pub type PROCESSINFOCLASS = i32;
pub const ProcessBasicInformation: PROCESSINFOCLASS = 0i32;
pub const ProcessDebugPort: PROCESSINFOCLASS = 7i32;
pub const ProcessWow64Information: PROCESSINFOCLASS = 26i32;
pub const ProcessImageFileName: PROCESSINFOCLASS = 27i32;
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = 29i32;
pub type PROCESSOR_FEATURE_ID = u32;
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: PROCESSOR_FEATURE_ID = 25u32;
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 24u32;
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: PROCESSOR_FEATURE_ID = 26u32;
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 27u32;
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: PROCESSOR_FEATURE_ID = 18u32;
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 7u32;
pub const PF_CHANNELS_ENABLED: PROCESSOR_FEATURE_ID = 16u32;
pub const PF_COMPARE_EXCHANGE_DOUBLE: PROCESSOR_FEATURE_ID = 2u32;
pub const PF_COMPARE_EXCHANGE128: PROCESSOR_FEATURE_ID = 14u32;
pub const PF_COMPARE64_EXCHANGE128: PROCESSOR_FEATURE_ID = 15u32;
pub const PF_FASTFAIL_AVAILABLE: PROCESSOR_FEATURE_ID = 23u32;
pub const PF_FLOATING_POINT_EMULATED: PROCESSOR_FEATURE_ID = 1u32;
pub const PF_FLOATING_POINT_PRECISION_ERRATA: PROCESSOR_FEATURE_ID = 0u32;
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 3u32;
pub const PF_NX_ENABLED: PROCESSOR_FEATURE_ID = 12u32;
pub const PF_PAE_ENABLED: PROCESSOR_FEATURE_ID = 9u32;
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 8u32;
pub const PF_RDWRFSGSBASE_AVAILABLE: PROCESSOR_FEATURE_ID = 22u32;
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: PROCESSOR_FEATURE_ID = 20u32;
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 13u32;
pub const PF_VIRT_FIRMWARE_ENABLED: PROCESSOR_FEATURE_ID = 21u32;
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 6u32;
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 10u32;
pub const PF_XSAVE_ENABLED: PROCESSOR_FEATURE_ID = 17u32;
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 29u32;
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 30u32;
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 31u32;
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 34u32;
pub type PROCESS_ACCESS_RIGHTS = u32;
pub const PROCESS_TERMINATE: PROCESS_ACCESS_RIGHTS = 1u32;
pub const PROCESS_CREATE_THREAD: PROCESS_ACCESS_RIGHTS = 2u32;
pub const PROCESS_SET_SESSIONID: PROCESS_ACCESS_RIGHTS = 4u32;
pub const PROCESS_VM_OPERATION: PROCESS_ACCESS_RIGHTS = 8u32;
pub const PROCESS_VM_READ: PROCESS_ACCESS_RIGHTS = 16u32;
pub const PROCESS_VM_WRITE: PROCESS_ACCESS_RIGHTS = 32u32;
pub const PROCESS_DUP_HANDLE: PROCESS_ACCESS_RIGHTS = 64u32;
pub const PROCESS_CREATE_PROCESS: PROCESS_ACCESS_RIGHTS = 128u32;
pub const PROCESS_SET_QUOTA: PROCESS_ACCESS_RIGHTS = 256u32;
pub const PROCESS_SET_INFORMATION: PROCESS_ACCESS_RIGHTS = 512u32;
pub const PROCESS_QUERY_INFORMATION: PROCESS_ACCESS_RIGHTS = 1024u32;
pub const PROCESS_SUSPEND_RESUME: PROCESS_ACCESS_RIGHTS = 2048u32;
pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = 4096u32;
pub const PROCESS_SET_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = 8192u32;
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = 2097151u32;
pub const PROCESS_DELETE: PROCESS_ACCESS_RIGHTS = 65536u32;
pub const PROCESS_READ_CONTROL: PROCESS_ACCESS_RIGHTS = 131072u32;
pub const PROCESS_WRITE_DAC: PROCESS_ACCESS_RIGHTS = 262144u32;
pub const PROCESS_WRITE_OWNER: PROCESS_ACCESS_RIGHTS = 524288u32;
pub const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = 1048576u32;
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: PROCESS_ACCESS_RIGHTS = 983040u32;
pub type PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = u32;
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = 0u32;
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PROCESS_BASIC_INFORMATION {
    pub Reserved1: *mut ::core::ffi::c_void,
    pub PebBaseAddress: *mut PEB,
    pub Reserved2: [*mut ::core::ffi::c_void; 2],
    pub UniqueProcessId: usize,
    pub Reserved3: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PROCESS_BASIC_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PROCESS_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROCESS_CREATION_FLAGS = u32;
pub const DEBUG_PROCESS: PROCESS_CREATION_FLAGS = 1u32;
pub const DEBUG_ONLY_THIS_PROCESS: PROCESS_CREATION_FLAGS = 2u32;
pub const CREATE_SUSPENDED: PROCESS_CREATION_FLAGS = 4u32;
pub const DETACHED_PROCESS: PROCESS_CREATION_FLAGS = 8u32;
pub const CREATE_NEW_CONSOLE: PROCESS_CREATION_FLAGS = 16u32;
pub const NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 32u32;
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 64u32;
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 128u32;
pub const REALTIME_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 256u32;
pub const CREATE_NEW_PROCESS_GROUP: PROCESS_CREATION_FLAGS = 512u32;
pub const CREATE_UNICODE_ENVIRONMENT: PROCESS_CREATION_FLAGS = 1024u32;
pub const CREATE_SEPARATE_WOW_VDM: PROCESS_CREATION_FLAGS = 2048u32;
pub const CREATE_SHARED_WOW_VDM: PROCESS_CREATION_FLAGS = 4096u32;
pub const CREATE_FORCEDOS: PROCESS_CREATION_FLAGS = 8192u32;
pub const BELOW_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 16384u32;
pub const ABOVE_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 32768u32;
pub const INHERIT_PARENT_AFFINITY: PROCESS_CREATION_FLAGS = 65536u32;
pub const INHERIT_CALLER_PRIORITY: PROCESS_CREATION_FLAGS = 131072u32;
pub const CREATE_PROTECTED_PROCESS: PROCESS_CREATION_FLAGS = 262144u32;
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = 524288u32;
pub const PROCESS_MODE_BACKGROUND_BEGIN: PROCESS_CREATION_FLAGS = 1048576u32;
pub const PROCESS_MODE_BACKGROUND_END: PROCESS_CREATION_FLAGS = 2097152u32;
pub const CREATE_SECURE_PROCESS: PROCESS_CREATION_FLAGS = 4194304u32;
pub const CREATE_BREAKAWAY_FROM_JOB: PROCESS_CREATION_FLAGS = 16777216u32;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: PROCESS_CREATION_FLAGS = 33554432u32;
pub const CREATE_DEFAULT_ERROR_MODE: PROCESS_CREATION_FLAGS = 67108864u32;
pub const CREATE_NO_WINDOW: PROCESS_CREATION_FLAGS = 134217728u32;
pub const PROFILE_USER: PROCESS_CREATION_FLAGS = 268435456u32;
pub const PROFILE_KERNEL: PROCESS_CREATION_FLAGS = 536870912u32;
pub const PROFILE_SERVER: PROCESS_CREATION_FLAGS = 1073741824u32;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: PROCESS_CREATION_FLAGS = 2147483648u32;
pub type PROCESS_DEP_FLAGS = u32;
pub const PROCESS_DEP_ENABLE: PROCESS_DEP_FLAGS = 1u32;
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: PROCESS_DEP_FLAGS = 2u32;
pub const PROCESS_DEP_NONE: PROCESS_DEP_FLAGS = 0u32;
#[repr(C)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub TargetAddress: usize,
    pub Flags: usize,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub NumberOfTargets: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub BaseAddress: usize,
    pub Size: usize,
    pub Flags: u32,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub NumberOfRanges: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROCESS_INFORMATION_CLASS = i32;
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = 0i32;
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = 1i32;
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = 2i32;
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = 3i32;
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = 4i32;
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = 5i32;
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = 6i32;
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = 7i32;
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = 8i32;
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = 9i32;
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = 10i32;
#[repr(C)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PROCESS_LEAP_SECOND_INFO {}
impl ::core::clone::Clone for PROCESS_LEAP_SECOND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
#[repr(C)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: u16,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
impl ::core::marker::Copy for PROCESS_MACHINE_INFORMATION {}
impl ::core::clone::Clone for PROCESS_MACHINE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: usize,
}
impl ::core::marker::Copy for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::core::clone::Clone for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROCESS_MEMORY_EXHAUSTION_TYPE = i32;
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = 0i32;
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = 1i32;
pub type PROCESS_MITIGATION_POLICY = i32;
pub const ProcessDEPPolicy: PROCESS_MITIGATION_POLICY = 0i32;
pub const ProcessASLRPolicy: PROCESS_MITIGATION_POLICY = 1i32;
pub const ProcessDynamicCodePolicy: PROCESS_MITIGATION_POLICY = 2i32;
pub const ProcessStrictHandleCheckPolicy: PROCESS_MITIGATION_POLICY = 3i32;
pub const ProcessSystemCallDisablePolicy: PROCESS_MITIGATION_POLICY = 4i32;
pub const ProcessMitigationOptionsMask: PROCESS_MITIGATION_POLICY = 5i32;
pub const ProcessExtensionPointDisablePolicy: PROCESS_MITIGATION_POLICY = 6i32;
pub const ProcessControlFlowGuardPolicy: PROCESS_MITIGATION_POLICY = 7i32;
pub const ProcessSignaturePolicy: PROCESS_MITIGATION_POLICY = 8i32;
pub const ProcessFontDisablePolicy: PROCESS_MITIGATION_POLICY = 9i32;
pub const ProcessImageLoadPolicy: PROCESS_MITIGATION_POLICY = 10i32;
pub const ProcessSystemCallFilterPolicy: PROCESS_MITIGATION_POLICY = 11i32;
pub const ProcessPayloadRestrictionPolicy: PROCESS_MITIGATION_POLICY = 12i32;
pub const ProcessChildProcessPolicy: PROCESS_MITIGATION_POLICY = 13i32;
pub const ProcessSideChannelIsolationPolicy: PROCESS_MITIGATION_POLICY = 14i32;
pub const ProcessUserShadowStackPolicy: PROCESS_MITIGATION_POLICY = 15i32;
pub const ProcessRedirectionTrustPolicy: PROCESS_MITIGATION_POLICY = 16i32;
pub const MaxProcessMitigationPolicy: PROCESS_MITIGATION_POLICY = 17i32;
pub type PROCESS_NAME_FORMAT = u32;
pub const PROCESS_NAME_WIN32: PROCESS_NAME_FORMAT = 0u32;
pub const PROCESS_NAME_NATIVE: PROCESS_NAME_FORMAT = 1u32;
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
#[repr(C)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for PROCESS_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for PROCESS_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROCESS_PROTECTION_LEVEL = u32;
pub const PROTECTION_LEVEL_WINTCB_LIGHT: PROCESS_PROTECTION_LEVEL = 0u32;
pub const PROTECTION_LEVEL_WINDOWS: PROCESS_PROTECTION_LEVEL = 1u32;
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: PROCESS_PROTECTION_LEVEL = 2u32;
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: PROCESS_PROTECTION_LEVEL = 3u32;
pub const PROTECTION_LEVEL_LSA_LIGHT: PROCESS_PROTECTION_LEVEL = 4u32;
pub const PROTECTION_LEVEL_WINTCB: PROCESS_PROTECTION_LEVEL = 5u32;
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: PROCESS_PROTECTION_LEVEL = 6u32;
pub const PROTECTION_LEVEL_AUTHENTICODE: PROCESS_PROTECTION_LEVEL = 7u32;
pub const PROTECTION_LEVEL_PPL_APP: PROCESS_PROTECTION_LEVEL = 8u32;
pub const PROTECTION_LEVEL_NONE: PROCESS_PROTECTION_LEVEL = 4294967294u32;
#[repr(C)]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: PROCESS_PROTECTION_LEVEL,
}
impl ::core::marker::Copy for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::core::clone::Clone for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
#[cfg(feature = "Win32_System_SystemServices")]
pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = ::core::option::Option<unsafe extern "system" fn(reason: super::SystemServices::RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const ::core::ffi::c_void)>;
pub type PTIMERAPCROUTINE = ::core::option::Option<unsafe extern "system" fn(lpargtocompletionroutine: *const ::core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32)>;
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(objectcontext: *mut ::core::ffi::c_void, cleanupcontext: *mut ::core::ffi::c_void)>;
pub type PTP_POOL = isize;
pub type PTP_SIMPLE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void)>;
pub type PTP_TIMER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, timer: *mut TP_TIMER)>;
pub type PTP_WAIT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, wait: *mut TP_WAIT, waitresult: u32)>;
pub type PTP_WIN32_IO_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, overlapped: *mut ::core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: *mut TP_IO)>;
pub type PTP_WORK_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, work: *mut TP_WORK)>;
pub type QUEUE_USER_APC_FLAGS = i32;
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = 0i32;
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT {
    pub Version: u32,
    pub Flags: POWER_REQUEST_CONTEXT_FLAGS,
    pub Reason: REASON_CONTEXT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union REASON_CONTEXT_0 {
    pub Detailed: REASON_CONTEXT_0_0,
    pub SimpleReasonString: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: super::super::Foundation::HINSTANCE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [usize; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
impl ::core::marker::Copy for RTL_BARRIER {}
impl ::core::clone::Clone for RTL_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_CONDITION_VARIABLE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_CONDITION_VARIABLE {}
impl ::core::clone::Clone for RTL_CONDITION_VARIABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION {
    pub DebugInfo: *mut RTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: super::super::Foundation::HANDLE,
    pub LockSemaphore: super::super::Foundation::HANDLE,
    pub SpinCount: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut RTL_CRITICAL_SECTION,
    pub ProcessLocksList: super::Kernel::LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub SpareWORD: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION_DEBUG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION_DEBUG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union RTL_RUN_ONCE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_RUN_ONCE {}
impl ::core::clone::Clone for RTL_RUN_ONCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_SRWLOCK {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_SRWLOCK {}
impl ::core::clone::Clone for RTL_SRWLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RTL_UMS_THREAD_INFO_CLASS = i32;
pub const UmsThreadInvalidInfoClass: RTL_UMS_THREAD_INFO_CLASS = 0i32;
pub const UmsThreadUserContext: RTL_UMS_THREAD_INFO_CLASS = 1i32;
pub const UmsThreadPriority: RTL_UMS_THREAD_INFO_CLASS = 2i32;
pub const UmsThreadAffinity: RTL_UMS_THREAD_INFO_CLASS = 3i32;
pub const UmsThreadTeb: RTL_UMS_THREAD_INFO_CLASS = 4i32;
pub const UmsThreadIsSuspended: RTL_UMS_THREAD_INFO_CLASS = 5i32;
pub const UmsThreadIsTerminated: RTL_UMS_THREAD_INFO_CLASS = 6i32;
pub const UmsThreadMaxInfoClass: RTL_UMS_THREAD_INFO_CLASS = 7i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut ::core::ffi::c_void; 10],
    pub ImagePathName: super::super::Foundation::UNICODE_STRING,
    pub CommandLine: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTL_USER_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTL_USER_PROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: super::super::Foundation::PSTR,
    pub lpDesktop: super::super::Foundation::PSTR,
    pub lpTitle: super::super::Foundation::PSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXA {
    pub StartupInfo: STARTUPINFOA,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXW {
    pub StartupInfo: STARTUPINFOW,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: super::super::Foundation::PWSTR,
    pub lpDesktop: super::super::Foundation::PWSTR,
    pub lpTitle: super::super::Foundation::PWSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type STARTUPINFOW_FLAGS = u32;
pub const STARTF_FORCEONFEEDBACK: STARTUPINFOW_FLAGS = 64u32;
pub const STARTF_FORCEOFFFEEDBACK: STARTUPINFOW_FLAGS = 128u32;
pub const STARTF_PREVENTPINNING: STARTUPINFOW_FLAGS = 8192u32;
pub const STARTF_RUNFULLSCREEN: STARTUPINFOW_FLAGS = 32u32;
pub const STARTF_TITLEISAPPID: STARTUPINFOW_FLAGS = 4096u32;
pub const STARTF_TITLEISLINKNAME: STARTUPINFOW_FLAGS = 2048u32;
pub const STARTF_UNTRUSTEDSOURCE: STARTUPINFOW_FLAGS = 32768u32;
pub const STARTF_USECOUNTCHARS: STARTUPINFOW_FLAGS = 8u32;
pub const STARTF_USEFILLATTRIBUTE: STARTUPINFOW_FLAGS = 16u32;
pub const STARTF_USEHOTKEY: STARTUPINFOW_FLAGS = 512u32;
pub const STARTF_USEPOSITION: STARTUPINFOW_FLAGS = 4u32;
pub const STARTF_USESHOWWINDOW: STARTUPINFOW_FLAGS = 1u32;
pub const STARTF_USESIZE: STARTUPINFOW_FLAGS = 2u32;
pub const STARTF_USESTDHANDLES: STARTUPINFOW_FLAGS = 256u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
pub type THREADINFOCLASS = i32;
pub const ThreadIsIoPending: THREADINFOCLASS = 16i32;
pub const ThreadNameInformation: THREADINFOCLASS = 38i32;
pub type THREAD_ACCESS_RIGHTS = u32;
pub const THREAD_TERMINATE: THREAD_ACCESS_RIGHTS = 1u32;
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = 2u32;
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = 8u32;
pub const THREAD_SET_CONTEXT: THREAD_ACCESS_RIGHTS = 16u32;
pub const THREAD_SET_INFORMATION: THREAD_ACCESS_RIGHTS = 32u32;
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = 64u32;
pub const THREAD_SET_THREAD_TOKEN: THREAD_ACCESS_RIGHTS = 128u32;
pub const THREAD_IMPERSONATE: THREAD_ACCESS_RIGHTS = 256u32;
pub const THREAD_DIRECT_IMPERSONATION: THREAD_ACCESS_RIGHTS = 512u32;
pub const THREAD_SET_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = 1024u32;
pub const THREAD_QUERY_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = 2048u32;
pub const THREAD_RESUME: THREAD_ACCESS_RIGHTS = 4096u32;
pub const THREAD_ALL_ACCESS: THREAD_ACCESS_RIGHTS = 2097151u32;
pub const THREAD_DELETE: THREAD_ACCESS_RIGHTS = 65536u32;
pub const THREAD_READ_CONTROL: THREAD_ACCESS_RIGHTS = 131072u32;
pub const THREAD_WRITE_DAC: THREAD_ACCESS_RIGHTS = 262144u32;
pub const THREAD_WRITE_OWNER: THREAD_ACCESS_RIGHTS = 524288u32;
pub const THREAD_SYNCHRONIZE: THREAD_ACCESS_RIGHTS = 1048576u32;
pub const THREAD_STANDARD_RIGHTS_REQUIRED: THREAD_ACCESS_RIGHTS = 983040u32;
pub type THREAD_CREATION_FLAGS = u32;
pub const THREAD_CREATE_RUN_IMMEDIATELY: THREAD_CREATION_FLAGS = 0u32;
pub const THREAD_CREATE_SUSPENDED: THREAD_CREATION_FLAGS = 4u32;
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: THREAD_CREATION_FLAGS = 65536u32;
pub type THREAD_INFORMATION_CLASS = i32;
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = 0i32;
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = 1i32;
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = 2i32;
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = 3i32;
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = 4i32;
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[repr(C)]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for THREAD_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for THREAD_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
pub type THREAD_PRIORITY = i32;
pub const THREAD_MODE_BACKGROUND_BEGIN: THREAD_PRIORITY = 65536i32;
pub const THREAD_MODE_BACKGROUND_END: THREAD_PRIORITY = 131072i32;
pub const THREAD_PRIORITY_ABOVE_NORMAL: THREAD_PRIORITY = 1i32;
pub const THREAD_PRIORITY_BELOW_NORMAL: THREAD_PRIORITY = -1i32;
pub const THREAD_PRIORITY_HIGHEST: THREAD_PRIORITY = 2i32;
pub const THREAD_PRIORITY_IDLE: THREAD_PRIORITY = -15i32;
pub const THREAD_PRIORITY_MIN: THREAD_PRIORITY = -2i32;
pub const THREAD_PRIORITY_LOWEST: THREAD_PRIORITY = -2i32;
pub const THREAD_PRIORITY_NORMAL: THREAD_PRIORITY = 0i32;
pub const THREAD_PRIORITY_TIME_CRITICAL: THREAD_PRIORITY = 15i32;
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: isize,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut ::core::ffi::c_void,
    pub ActivationContext: isize,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_1,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3_0(pub u8);
#[repr(C)]
pub union TP_CALLBACK_ENVIRON_V3_1 {
    pub Flags: u32,
    pub s: TP_CALLBACK_ENVIRON_V3_1_0,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3_1_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1_0 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_CALLBACK_INSTANCE(pub u8);
pub type TP_CALLBACK_PRIORITY = i32;
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = 0i32;
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = 1i32;
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = 2i32;
pub const TP_CALLBACK_PRIORITY_INVALID: TP_CALLBACK_PRIORITY = 3i32;
pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = 3i32;
#[repr(C)]
pub struct TP_IO(pub u8);
#[repr(C)]
pub struct TP_POOL_STACK_INFORMATION {
    pub StackReserve: usize,
    pub StackCommit: usize,
}
impl ::core::marker::Copy for TP_POOL_STACK_INFORMATION {}
impl ::core::clone::Clone for TP_POOL_STACK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_TIMER(pub u8);
#[repr(C)]
pub struct TP_WAIT(pub u8);
#[repr(C)]
pub struct TP_WORK(pub u8);
pub type TimerQueueHandle = isize;
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: *mut ::core::ffi::c_void,
    pub SchedulerProc: PRTL_UMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::marker::Copy for UMS_SCHEDULER_STARTUP_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::clone::Clone for UMS_SCHEDULER_STARTUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type WAITORTIMERCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::BOOLEAN)>;
pub const WAIT_ABANDONED: u32 = 128u32;
pub const WAIT_ABANDONED_0: u32 = 128u32;
pub const WAIT_IO_COMPLETION: u32 = 192u32;
pub const WAIT_OBJECT_0: u32 = 0u32;
pub type WORKER_THREAD_FLAGS = u32;
pub const WT_EXECUTEDEFAULT: WORKER_THREAD_FLAGS = 0u32;
pub const WT_EXECUTEINIOTHREAD: WORKER_THREAD_FLAGS = 1u32;
pub const WT_EXECUTEINPERSISTENTTHREAD: WORKER_THREAD_FLAGS = 128u32;
pub const WT_EXECUTEINWAITTHREAD: WORKER_THREAD_FLAGS = 4u32;
pub const WT_EXECUTELONGFUNCTION: WORKER_THREAD_FLAGS = 16u32;
pub const WT_EXECUTEONLYONCE: WORKER_THREAD_FLAGS = 8u32;
pub const WT_TRANSFER_IMPERSONATION: WORKER_THREAD_FLAGS = 256u32;
pub const WT_EXECUTEINTIMERTHREAD: WORKER_THREAD_FLAGS = 32u32;
