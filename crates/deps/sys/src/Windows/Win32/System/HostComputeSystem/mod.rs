#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type HcsAttachLayerStorageFilter = unsafe extern "system" fn(layerpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type HcsCancelOperation = unsafe extern "system" fn(operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
pub type HcsCloseComputeSystem = unsafe extern "system" fn(computesystem: HCS_SYSTEM);
pub type HcsCloseOperation = unsafe extern "system" fn(operation: HCS_OPERATION);
pub type HcsCloseProcess = unsafe extern "system" fn(process: HCS_PROCESS);
#[cfg(feature = "Win32_Foundation")]
pub type HcsCrashComputeSystem = unsafe extern "system" fn(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type HcsCreateComputeSystem = unsafe extern "system" fn(id: super::super::Foundation::PWSTR, configuration: super::super::Foundation::PWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsCreateComputeSystemInNamespace = unsafe extern "system" fn(idnamespace: super::super::Foundation::PWSTR, id: super::super::Foundation::PWSTR, configuration: super::super::Foundation::PWSTR, operation: HCS_OPERATION, options: *const HCS_CREATE_OPTIONS, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsCreateEmptyGuestStateFile = unsafe extern "system" fn(gueststatefilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsCreateEmptyRuntimeStateFile = unsafe extern "system" fn(runtimestatefilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type HcsCreateOperation = unsafe extern "system" fn(context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> HCS_OPERATION;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type HcsCreateProcess = unsafe extern "system" fn(computesystem: HCS_SYSTEM, processparameters: super::super::Foundation::PWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, process: *mut HCS_PROCESS) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsDestroyLayer = unsafe extern "system" fn(layerpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsDetachLayerStorageFilter = unsafe extern "system" fn(layerpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsEnumerateComputeSystems = unsafe extern "system" fn(query: super::super::Foundation::PWSTR, operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsEnumerateComputeSystemsInNamespace = unsafe extern "system" fn(idnamespace: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsExportLayer = unsafe extern "system" fn(layerpath: super::super::Foundation::PWSTR, exportfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsExportLegacyWritableLayer = unsafe extern "system" fn(writablelayermountpath: super::super::Foundation::PWSTR, writablelayerfolderpath: super::super::Foundation::PWSTR, exportfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsFormatWritableLayerVhd = unsafe extern "system" fn(vhdhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
pub type HcsGetComputeSystemFromOperation = unsafe extern "system" fn(operation: HCS_OPERATION) -> HCS_SYSTEM;
#[cfg(feature = "Win32_Foundation")]
pub type HcsGetComputeSystemProperties = unsafe extern "system" fn(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, propertyquery: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsGetLayerVhdMountPath = unsafe extern "system" fn(vhdhandle: super::super::Foundation::HANDLE, mountpath: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type HcsGetOperationContext = unsafe extern "system" fn(operation: HCS_OPERATION) -> *mut ::core::ffi::c_void;
pub type HcsGetOperationId = unsafe extern "system" fn(operation: HCS_OPERATION) -> u64;
#[cfg(feature = "Win32_Foundation")]
pub type HcsGetOperationResult = unsafe extern "system" fn(operation: HCS_OPERATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsGetOperationResultAndProcessInfo = unsafe extern "system" fn(operation: HCS_OPERATION, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type HcsGetOperationType = unsafe extern "system" fn(operation: HCS_OPERATION) -> HCS_OPERATION_TYPE;
pub type HcsGetProcessFromOperation = unsafe extern "system" fn(operation: HCS_OPERATION) -> HCS_PROCESS;
pub type HcsGetProcessInfo = unsafe extern "system" fn(process: HCS_PROCESS, operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsGetProcessProperties = unsafe extern "system" fn(process: HCS_PROCESS, operation: HCS_OPERATION, propertyquery: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsGetProcessorCompatibilityFromSavedState = unsafe extern "system" fn(runtimefilename: super::super::Foundation::PWSTR, processorfeaturesstring: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsGetServiceProperties = unsafe extern "system" fn(propertyquery: super::super::Foundation::PWSTR, result: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsGrantVmAccess = unsafe extern "system" fn(vmid: super::super::Foundation::PWSTR, filepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsGrantVmGroupAccess = unsafe extern "system" fn(filepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsImportLayer = unsafe extern "system" fn(layerpath: super::super::Foundation::PWSTR, sourcefolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsInitializeLegacyWritableLayer = unsafe extern "system" fn(writablelayermountpath: super::super::Foundation::PWSTR, writablelayerfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsInitializeWritableLayer = unsafe extern "system" fn(writablelayerpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsModifyComputeSystem = unsafe extern "system" fn(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, configuration: super::super::Foundation::PWSTR, identity: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsModifyProcess = unsafe extern "system" fn(process: HCS_PROCESS, operation: HCS_OPERATION, settings: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsModifyServiceSettings = unsafe extern "system" fn(settings: super::super::Foundation::PWSTR, result: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsOpenComputeSystem = unsafe extern "system" fn(id: super::super::Foundation::PWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsOpenComputeSystemInNamespace = unsafe extern "system" fn(idnamespace: super::super::Foundation::PWSTR, id: super::super::Foundation::PWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
pub type HcsOpenProcess = unsafe extern "system" fn(computesystem: HCS_SYSTEM, processid: u32, requestedaccess: u32, process: *mut HCS_PROCESS) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsPauseComputeSystem = unsafe extern "system" fn(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsResumeComputeSystem = unsafe extern "system" fn(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsRevokeVmAccess = unsafe extern "system" fn(vmid: super::super::Foundation::PWSTR, filepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsRevokeVmGroupAccess = unsafe extern "system" fn(filepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsSaveComputeSystem = unsafe extern "system" fn(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsSetComputeSystemCallback = unsafe extern "system" fn(computesystem: HCS_SYSTEM, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows_sys::core::HRESULT;
pub type HcsSetOperationCallback = unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> ::windows_sys::core::HRESULT;
pub type HcsSetOperationContext = unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsSetProcessCallback = unsafe extern "system" fn(process: HCS_PROCESS, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsSetupBaseOSLayer = unsafe extern "system" fn(layerpath: super::super::Foundation::PWSTR, vhdhandle: super::super::Foundation::HANDLE, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsSetupBaseOSVolume = unsafe extern "system" fn(layerpath: super::super::Foundation::PWSTR, volumepath: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsShutDownComputeSystem = unsafe extern "system" fn(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsSignalProcess = unsafe extern "system" fn(process: HCS_PROCESS, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsStartComputeSystem = unsafe extern "system" fn(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsSubmitWerReport = unsafe extern "system" fn(settings: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsTerminateComputeSystem = unsafe extern "system" fn(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsTerminateProcess = unsafe extern "system" fn(process: HCS_PROCESS, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsWaitForComputeSystemExit = unsafe extern "system" fn(computesystem: HCS_SYSTEM, timeoutms: u32, result: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsWaitForOperationResult = unsafe extern "system" fn(operation: HCS_OPERATION, timeoutms: u32, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsWaitForOperationResultAndProcessInfo = unsafe extern "system" fn(operation: HCS_OPERATION, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcsWaitForProcessExit = unsafe extern "system" fn(computesystem: HCS_PROCESS, timeoutms: u32, result: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type HCS_CREATE_OPTIONS = i32;
pub const HcsCreateOptions_1: HCS_CREATE_OPTIONS = 65536i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct HCS_CREATE_OPTIONS_1 {
    pub Version: HCS_CREATE_OPTIONS,
    pub UserToken: super::super::Foundation::HANDLE,
    pub SecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    pub CallbackOptions: HCS_EVENT_OPTIONS,
    pub CallbackContext: *mut ::core::ffi::c_void,
    pub Callback: HCS_EVENT_CALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for HCS_CREATE_OPTIONS_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for HCS_CREATE_OPTIONS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HCS_EVENT {
    pub Type: HCS_EVENT_TYPE,
    pub EventData: super::super::Foundation::PWSTR,
    pub Operation: HCS_OPERATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HCS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HCS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type HCS_EVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(event: *const HCS_EVENT, context: *const ::core::ffi::c_void)>;
pub type HCS_EVENT_OPTIONS = u32;
pub const HcsEventOptionNone: HCS_EVENT_OPTIONS = 0u32;
pub const HcsEventOptionEnableOperationCallbacks: HCS_EVENT_OPTIONS = 1u32;
pub type HCS_EVENT_TYPE = i32;
pub const HcsEventInvalid: HCS_EVENT_TYPE = 0i32;
pub const HcsEventSystemExited: HCS_EVENT_TYPE = 1i32;
pub const HcsEventSystemCrashInitiated: HCS_EVENT_TYPE = 2i32;
pub const HcsEventSystemCrashReport: HCS_EVENT_TYPE = 3i32;
pub const HcsEventSystemRdpEnhancedModeStateChanged: HCS_EVENT_TYPE = 4i32;
pub const HcsEventSystemSiloJobCreated: HCS_EVENT_TYPE = 5i32;
pub const HcsEventSystemGuestConnectionClosed: HCS_EVENT_TYPE = 6i32;
pub const HcsEventProcessExited: HCS_EVENT_TYPE = 65536i32;
pub const HcsEventOperationCallback: HCS_EVENT_TYPE = 16777216i32;
pub const HcsEventServiceDisconnect: HCS_EVENT_TYPE = 33554432i32;
pub type HCS_NOTIFICATIONS = i32;
pub const HcsNotificationInvalid: HCS_NOTIFICATIONS = 0i32;
pub const HcsNotificationSystemExited: HCS_NOTIFICATIONS = 1i32;
pub const HcsNotificationSystemCreateCompleted: HCS_NOTIFICATIONS = 2i32;
pub const HcsNotificationSystemStartCompleted: HCS_NOTIFICATIONS = 3i32;
pub const HcsNotificationSystemPauseCompleted: HCS_NOTIFICATIONS = 4i32;
pub const HcsNotificationSystemResumeCompleted: HCS_NOTIFICATIONS = 5i32;
pub const HcsNotificationSystemCrashReport: HCS_NOTIFICATIONS = 6i32;
pub const HcsNotificationSystemSiloJobCreated: HCS_NOTIFICATIONS = 7i32;
pub const HcsNotificationSystemSaveCompleted: HCS_NOTIFICATIONS = 8i32;
pub const HcsNotificationSystemRdpEnhancedModeStateChanged: HCS_NOTIFICATIONS = 9i32;
pub const HcsNotificationSystemShutdownFailed: HCS_NOTIFICATIONS = 10i32;
pub const HcsNotificationSystemShutdownCompleted: HCS_NOTIFICATIONS = 10i32;
pub const HcsNotificationSystemGetPropertiesCompleted: HCS_NOTIFICATIONS = 11i32;
pub const HcsNotificationSystemModifyCompleted: HCS_NOTIFICATIONS = 12i32;
pub const HcsNotificationSystemCrashInitiated: HCS_NOTIFICATIONS = 13i32;
pub const HcsNotificationSystemGuestConnectionClosed: HCS_NOTIFICATIONS = 14i32;
pub const HcsNotificationSystemOperationCompletion: HCS_NOTIFICATIONS = 15i32;
pub const HcsNotificationSystemPassThru: HCS_NOTIFICATIONS = 16i32;
pub const HcsNotificationProcessExited: HCS_NOTIFICATIONS = 65536i32;
pub const HcsNotificationServiceDisconnect: HCS_NOTIFICATIONS = 16777216i32;
pub const HcsNotificationFlagsReserved: HCS_NOTIFICATIONS = -268435456i32;
#[cfg(feature = "Win32_Foundation")]
pub type HCS_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows_sys::core::HRESULT, notificationdata: super::super::Foundation::PWSTR)>;
pub type HCS_NOTIFICATION_FLAGS = i32;
pub const HcsNotificationFlagSuccess: HCS_NOTIFICATION_FLAGS = 0i32;
pub const HcsNotificationFlagFailure: HCS_NOTIFICATION_FLAGS = -2147483648i32;
pub type HCS_OPERATION = isize;
pub type HCS_OPERATION_COMPLETION = ::core::option::Option<unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::core::ffi::c_void)>;
pub type HCS_OPERATION_TYPE = i32;
pub const HcsOperationTypeNone: HCS_OPERATION_TYPE = -1i32;
pub const HcsOperationTypeEnumerate: HCS_OPERATION_TYPE = 0i32;
pub const HcsOperationTypeCreate: HCS_OPERATION_TYPE = 1i32;
pub const HcsOperationTypeStart: HCS_OPERATION_TYPE = 2i32;
pub const HcsOperationTypeShutdown: HCS_OPERATION_TYPE = 3i32;
pub const HcsOperationTypePause: HCS_OPERATION_TYPE = 4i32;
pub const HcsOperationTypeResume: HCS_OPERATION_TYPE = 5i32;
pub const HcsOperationTypeSave: HCS_OPERATION_TYPE = 6i32;
pub const HcsOperationTypeTerminate: HCS_OPERATION_TYPE = 7i32;
pub const HcsOperationTypeModify: HCS_OPERATION_TYPE = 8i32;
pub const HcsOperationTypeGetProperties: HCS_OPERATION_TYPE = 9i32;
pub const HcsOperationTypeCreateProcess: HCS_OPERATION_TYPE = 10i32;
pub const HcsOperationTypeSignalProcess: HCS_OPERATION_TYPE = 11i32;
pub const HcsOperationTypeGetProcessInfo: HCS_OPERATION_TYPE = 12i32;
pub const HcsOperationTypeGetProcessProperties: HCS_OPERATION_TYPE = 13i32;
pub const HcsOperationTypeModifyProcess: HCS_OPERATION_TYPE = 14i32;
pub const HcsOperationTypeCrash: HCS_OPERATION_TYPE = 15i32;
pub type HCS_PROCESS = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HCS_PROCESS_INFORMATION {
    pub ProcessId: u32,
    pub Reserved: u32,
    pub StdInput: super::super::Foundation::HANDLE,
    pub StdOutput: super::super::Foundation::HANDLE,
    pub StdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HCS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HCS_PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HCS_SYSTEM = isize;
