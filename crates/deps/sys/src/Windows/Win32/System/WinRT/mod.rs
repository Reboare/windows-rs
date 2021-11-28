#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_System_WinRT_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Win32_System_WinRT_Composition")]
pub mod Composition;
#[cfg(feature = "Win32_System_WinRT_CoreInputView")]
pub mod CoreInputView;
#[cfg(feature = "Win32_System_WinRT_Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Win32_System_WinRT_Display")]
pub mod Display;
#[cfg(feature = "Win32_System_WinRT_Graphics")]
pub mod Graphics;
#[cfg(feature = "Win32_System_WinRT_Holographic")]
pub mod Holographic;
#[cfg(feature = "Win32_System_WinRT_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_System_WinRT_ML")]
pub mod ML;
#[cfg(feature = "Win32_System_WinRT_Media")]
pub mod Media;
#[cfg(feature = "Win32_System_WinRT_Pdf")]
pub mod Pdf;
#[cfg(feature = "Win32_System_WinRT_Printing")]
pub mod Printing;
#[cfg(feature = "Win32_System_WinRT_Shell")]
pub mod Shell;
#[cfg(feature = "Win32_System_WinRT_Storage")]
pub mod Storage;
#[cfg(feature = "Win32_System_WinRT_Xaml")]
pub mod Xaml;
pub type CoDecodeProxy = unsafe extern "system" fn(dwclientpid: u32, ui64proxyaddress: u64, pserverinformation: *mut ServerInformation) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "System")]
pub type CreateDispatcherQueueController = unsafe extern "system" fn(options: DispatcherQueueOptions, dispatcherqueuecontroller: *mut super::super::super::System::DispatcherQueueController) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CreateRandomAccessStreamOnFile = unsafe extern "system" fn(filepath: super::super::Foundation::PWSTR, accessmode: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_System_Com")]
pub type CreateRandomAccessStreamOverStream = unsafe extern "system" fn(stream: super::Com::IStream, options: BSOS_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type CreateStreamOverRandomAccessStream = unsafe extern "system" fn(randomaccessstream: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type GetRestrictedErrorInfo = unsafe extern "system" fn(pprestrictederrorinfo: *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
pub type HSTRING_UserFree = unsafe extern "system" fn(param0: *const u32, param1: *const ::windows_sys::core::HSTRING);
pub type HSTRING_UserFree64 = unsafe extern "system" fn(param0: *const u32, param1: *const ::windows_sys::core::HSTRING);
pub type HSTRING_UserMarshal = unsafe extern "system" fn(param0: *const u32, param1: *mut u8, param2: *const ::windows_sys::core::HSTRING) -> *mut u8;
pub type HSTRING_UserMarshal64 = unsafe extern "system" fn(param0: *const u32, param1: *mut u8, param2: *const ::windows_sys::core::HSTRING) -> *mut u8;
pub type HSTRING_UserSize = unsafe extern "system" fn(param0: *const u32, param1: u32, param2: *const ::windows_sys::core::HSTRING) -> u32;
pub type HSTRING_UserSize64 = unsafe extern "system" fn(param0: *const u32, param1: u32, param2: *const ::windows_sys::core::HSTRING) -> u32;
pub type HSTRING_UserUnmarshal = unsafe extern "system" fn(param0: *const u32, param1: *const u8, param2: *mut ::windows_sys::core::HSTRING) -> *mut u8;
pub type HSTRING_UserUnmarshal64 = unsafe extern "system" fn(param0: *const u32, param1: *const u8, param2: *mut ::windows_sys::core::HSTRING) -> *mut u8;
#[cfg(feature = "Win32_Foundation")]
pub type IsErrorPropagationEnabled = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
pub type MetaDataGetDispenser = unsafe extern "system" fn(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type RoActivateInstance = unsafe extern "system" fn(activatableclassid: ::windows_sys::core::HSTRING, instance: *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
pub type RoCaptureErrorContext = unsafe extern "system" fn(hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
pub type RoClearError = unsafe extern "system" fn();
pub type RoFailFastWithErrorContext = unsafe extern "system" fn(hrerror: ::windows_sys::core::HRESULT);
pub type RoFreeParameterizedTypeExtra = unsafe extern "system" fn(extra: ROPARAMIIDHANDLE);
pub type RoGetActivationFactory = unsafe extern "system" fn(activatableclassid: ::windows_sys::core::HSTRING, iid: *const ::windows_sys::core::GUID, factory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type RoGetAgileReference = unsafe extern "system" fn(options: AgileReferenceOptions, riid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, ppagilereference: *mut IAgileReference) -> ::windows_sys::core::HRESULT;
pub type RoGetApartmentIdentifier = unsafe extern "system" fn(apartmentidentifier: *mut u64) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_System_Com_Marshal")]
pub type RoGetBufferMarshaler = unsafe extern "system" fn(buffermarshaler: *mut super::Com::Marshal::IMarshal) -> ::windows_sys::core::HRESULT;
pub type RoGetErrorReportingFlags = unsafe extern "system" fn(pflags: *mut u32) -> ::windows_sys::core::HRESULT;
pub type RoGetMatchingRestrictedErrorInfo = unsafe extern "system" fn(hrin: ::windows_sys::core::HRESULT, pprestrictederrorinfo: *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type RoGetParameterizedTypeInstanceIID = unsafe extern "system" fn(nameelementcount: u32, nameelements: *const super::super::Foundation::PWSTR, metadatalocator: IRoMetaDataLocator, iid: *mut ::windows_sys::core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows_sys::core::HRESULT;
pub type RoGetServerActivatableClasses = unsafe extern "system" fn(servername: ::windows_sys::core::HSTRING, activatableclassids: *mut *mut ::windows_sys::core::HSTRING, count: *mut u32) -> ::windows_sys::core::HRESULT;
pub type RoInitialize = unsafe extern "system" fn(inittype: RO_INIT_TYPE) -> ::windows_sys::core::HRESULT;
pub type RoInspectCapturedStackBackTrace = unsafe extern "system" fn(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows_sys::core::HRESULT;
pub type RoInspectThreadErrorInfo = unsafe extern "system" fn(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, targeterrorinfoaddress: *mut usize) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type RoOriginateError = unsafe extern "system" fn(error: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type RoOriginateErrorW = unsafe extern "system" fn(error: ::windows_sys::core::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type RoOriginateLanguageException = unsafe extern "system" fn(error: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING, languageexception: ::windows_sys::core::IUnknown) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type RoParameterizedTypeExtraGetTypeSignature = unsafe extern "system" fn(extra: ROPARAMIIDHANDLE) -> super::super::Foundation::PSTR;
pub type RoRegisterActivationFactories = unsafe extern "system" fn(activatableclassids: *const ::windows_sys::core::HSTRING, activationfactorycallbacks: *const isize, count: u32, cookie: *mut isize) -> ::windows_sys::core::HRESULT;
pub type RoRegisterForApartmentShutdown = unsafe extern "system" fn(callbackobject: IApartmentShutdown, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_sys::core::HRESULT;
pub type RoReportFailedDelegate = unsafe extern "system" fn(punkdelegate: ::windows_sys::core::IUnknown, prestrictederrorinfo: IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
pub type RoReportUnhandledError = unsafe extern "system" fn(prestrictederrorinfo: IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type RoResolveRestrictedErrorInfoReference = unsafe extern "system" fn(reference: super::super::Foundation::PWSTR, pprestrictederrorinfo: *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
pub type RoRevokeActivationFactories = unsafe extern "system" fn(cookie: isize);
pub type RoSetErrorReportingFlags = unsafe extern "system" fn(flags: u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type RoTransformError = unsafe extern "system" fn(olderror: ::windows_sys::core::HRESULT, newerror: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type RoTransformErrorW = unsafe extern "system" fn(olderror: ::windows_sys::core::HRESULT, newerror: ::windows_sys::core::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
pub type RoUninitialize = unsafe extern "system" fn();
pub type RoUnregisterForApartmentShutdown = unsafe extern "system" fn(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_sys::core::HRESULT;
pub type SetRestrictedErrorInfo = unsafe extern "system" fn(prestrictederrorinfo: IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
pub type WindowsCompareStringOrdinal = unsafe extern "system" fn(string1: ::windows_sys::core::HSTRING, string2: ::windows_sys::core::HSTRING, result: *mut i32) -> ::windows_sys::core::HRESULT;
pub type WindowsConcatString = unsafe extern "system" fn(string1: ::windows_sys::core::HSTRING, string2: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type WindowsCreateString = unsafe extern "system" fn(sourcestring: super::super::Foundation::PWSTR, length: u32, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type WindowsCreateStringReference = unsafe extern "system" fn(sourcestring: super::super::Foundation::PWSTR, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
pub type WindowsDeleteString = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
pub type WindowsDeleteStringBuffer = unsafe extern "system" fn(bufferhandle: HSTRING_BUFFER) -> ::windows_sys::core::HRESULT;
pub type WindowsDuplicateString = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
pub type WindowsGetStringLen = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type WindowsGetStringRawBuffer = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING, length: *mut u32) -> super::super::Foundation::PWSTR;
pub type WindowsInspectString = unsafe extern "system" fn(targethstring: usize, machine: u16, callback: PINSPECT_HSTRING_CALLBACK, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows_sys::core::HRESULT;
pub type WindowsInspectString2 = unsafe extern "system" fn(targethstring: u64, machine: u16, callback: PINSPECT_HSTRING_CALLBACK2, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type WindowsIsStringEmpty = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
pub type WindowsPreallocateStringBuffer = unsafe extern "system" fn(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows_sys::core::HRESULT;
pub type WindowsPromoteStringBuffer = unsafe extern "system" fn(bufferhandle: HSTRING_BUFFER, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
pub type WindowsReplaceString = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING, stringreplaced: ::windows_sys::core::HSTRING, stringreplacewith: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type WindowsStringHasEmbeddedNull = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING, hasembednull: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
pub type WindowsSubstring = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING, startindex: u32, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
pub type WindowsSubstringWithSpecifiedLength = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING, startindex: u32, length: u32, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
pub type WindowsTrimStringEnd = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING, trimstring: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
pub type WindowsTrimStringStart = unsafe extern "system" fn(string: ::windows_sys::core::HSTRING, trimstring: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
pub type ACTIVATIONTYPE = i32;
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = 0i32;
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = 1i32;
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = 2i32;
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = 4i32;
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = 8i32;
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = 16i32;
pub type APARTMENT_SHUTDOWN_REGISTRATION_COOKIE = isize;
pub type AgileReferenceOptions = i32;
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = 0i32;
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = 1i32;
pub type BSOS_OPTIONS = i32;
pub const BSOS_DEFAULT: BSOS_OPTIONS = 0i32;
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = 1i32;
pub type CASTING_CONNECTION_ERROR_STATUS = i32;
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = 0i32;
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = 1i32;
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = 2i32;
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = 3i32;
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = 4i32;
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = 5i32;
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = 6i32;
pub type CASTING_CONNECTION_STATE = i32;
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = 0i32;
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = 1i32;
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = 2i32;
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = 3i32;
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = 4i32;
pub const CastingSourceInfo_Property_CastingTypes: &'static str = "CastingTypes";
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: &'static str = "PreferredSourceUriScheme";
pub const CastingSourceInfo_Property_ProtectedMedia: &'static str = "ProtectedMedia";
pub type DISPATCHERQUEUE_THREAD_APARTMENTTYPE = i32;
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = 0i32;
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = 1i32;
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = 2i32;
pub type DISPATCHERQUEUE_THREAD_TYPE = i32;
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = 1i32;
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = 2i32;
#[repr(C)]
pub struct DispatcherQueueOptions {
    pub dwSize: u32,
    pub threadType: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}
impl ::core::marker::Copy for DispatcherQueueOptions {}
impl ::core::clone::Clone for DispatcherQueueOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct EventRegistrationToken {
    pub value: i64,
}
impl ::core::marker::Copy for EventRegistrationToken {}
impl ::core::clone::Clone for EventRegistrationToken {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HSTRING_BUFFER = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HSTRING_HEADER {
    pub Reserved: HSTRING_HEADER_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSTRING_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSTRING_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union HSTRING_HEADER_0 {
    pub Reserved1: *mut ::core::ffi::c_void,
    pub Reserved2: [super::super::Foundation::CHAR; 24],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSTRING_HEADER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSTRING_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IAccountsSettingsPaneInterop = *mut ::core::ffi::c_void;
pub type IActivationFactory = *mut ::core::ffi::c_void;
pub type IAgileReference = *mut ::core::ffi::c_void;
pub type IApartmentShutdown = *mut ::core::ffi::c_void;
pub type IAppServiceConnectionExtendedExecution = *mut ::core::ffi::c_void;
pub type IBufferByteAccess = *mut ::core::ffi::c_void;
pub type ICastingController = *mut ::core::ffi::c_void;
pub type ICastingEventHandler = *mut ::core::ffi::c_void;
pub type ICastingSourceInfo = *mut ::core::ffi::c_void;
pub type ICorrelationVectorInformation = *mut ::core::ffi::c_void;
pub type ICorrelationVectorSource = *mut ::core::ffi::c_void;
pub type IDragDropManagerInterop = *mut ::core::ffi::c_void;
pub type IHolographicSpaceInterop = *mut ::core::ffi::c_void;
pub type IInputPaneInterop = *mut ::core::ffi::c_void;
pub type ILanguageExceptionErrorInfo = *mut ::core::ffi::c_void;
pub type ILanguageExceptionErrorInfo2 = *mut ::core::ffi::c_void;
pub type ILanguageExceptionStackBackTrace = *mut ::core::ffi::c_void;
pub type ILanguageExceptionTransform = *mut ::core::ffi::c_void;
pub type IMemoryBufferByteAccess = *mut ::core::ffi::c_void;
pub type IMessageDispatcher = *mut ::core::ffi::c_void;
pub type IPlayToManagerInterop = *mut ::core::ffi::c_void;
pub type IRestrictedErrorInfo = *mut ::core::ffi::c_void;
pub type IRoMetaDataLocator = *mut ::core::ffi::c_void;
pub type IRoSimpleMetaDataBuilder = *mut ::core::ffi::c_void;
pub type IShareWindowCommandEventArgsInterop = *mut ::core::ffi::c_void;
pub type IShareWindowCommandSourceInterop = *mut ::core::ffi::c_void;
pub type ISpatialInteractionManagerInterop = *mut ::core::ffi::c_void;
pub type ISystemMediaTransportControlsInterop = *mut ::core::ffi::c_void;
pub type IUIViewSettingsInterop = *mut ::core::ffi::c_void;
pub type IUserActivityInterop = *mut ::core::ffi::c_void;
pub type IUserActivityRequestManagerInterop = *mut ::core::ffi::c_void;
pub type IUserActivitySourceHostInterop = *mut ::core::ffi::c_void;
pub type IUserConsentVerifierInterop = *mut ::core::ffi::c_void;
pub type IWeakReference = *mut ::core::ffi::c_void;
pub type IWeakReferenceSource = *mut ::core::ffi::c_void;
pub type IWebAuthenticationCoreManagerInterop = *mut ::core::ffi::c_void;
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
pub type PINSPECT_HSTRING_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT>;
pub type PINSPECT_HSTRING_CALLBACK2 = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: u64, length: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT>;
pub type PINSPECT_MEMORY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT>;
pub type ROPARAMIIDHANDLE = isize;
pub type RO_ERROR_REPORTING_FLAGS = u32;
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = 0u32;
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 1u32;
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 2u32;
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = 4u32;
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = 8u32;
pub type RO_INIT_TYPE = i32;
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = 0i32;
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = 1i32;
#[repr(C)]
pub struct ServerInformation {
    pub dwServerPid: u32,
    pub dwServerTid: u32,
    pub ui64ServerAddress: u64,
}
impl ::core::marker::Copy for ServerInformation {}
impl ::core::clone::Clone for ServerInformation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TrustLevel = i32;
pub const BaseTrust: TrustLevel = 0i32;
pub const PartialTrust: TrustLevel = 1i32;
pub const FullTrust: TrustLevel = 2i32;
#[repr(C)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
