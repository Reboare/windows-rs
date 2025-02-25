#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Storage_Xps")]
pub type PTCloseProvider = unsafe extern "system" fn(hprovider: super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub type PTConvertDevModeToPrintTicket = unsafe extern "system" fn(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: super::super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub type PTConvertPrintTicketToDevMode = unsafe extern "system" fn(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: super::super::super::System::Com::IStream, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub type PTGetPrintCapabilities = unsafe extern "system" fn(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: super::super::super::System::Com::IStream, pcapabilities: super::super::super::System::Com::IStream, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub type PTGetPrintDeviceCapabilities = unsafe extern "system" fn(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: super::super::super::System::Com::IStream, pdevicecapabilities: super::super::super::System::Com::IStream, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub type PTGetPrintDeviceResources = unsafe extern "system" fn(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pszlocalename: super::super::super::Foundation::PWSTR, pprintticket: super::super::super::System::Com::IStream, pdeviceresources: super::super::super::System::Com::IStream, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub type PTMergeAndValidatePrintTicket = unsafe extern "system" fn(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pbaseticket: super::super::super::System::Com::IStream, pdeltaticket: super::super::super::System::Com::IStream, scope: EPrintTicketScope, presultticket: super::super::super::System::Com::IStream, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
pub type PTOpenProvider = unsafe extern "system" fn(pszprintername: super::super::super::Foundation::PWSTR, dwversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
pub type PTOpenProviderEx = unsafe extern "system" fn(pszprintername: super::super::super::Foundation::PWSTR, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PTQuerySchemaVersionSupport = unsafe extern "system" fn(pszprintername: super::super::super::Foundation::PWSTR, pmaxversion: *mut u32) -> ::windows_sys::core::HRESULT;
pub type PTReleaseMemory = unsafe extern "system" fn(pbuffer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type EDefaultDevmodeType = i32;
pub const kUserDefaultDevmode: EDefaultDevmodeType = 0i32;
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = 1i32;
pub type EPrintTicketScope = i32;
pub const kPTPageScope: EPrintTicketScope = 0i32;
pub const kPTDocumentScope: EPrintTicketScope = 1i32;
pub const kPTJobScope: EPrintTicketScope = 2i32;
pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797u32;
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796u32;
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798u32;
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795u32;
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1u32;
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146u32;
pub const S_PT_NO_CONFLICT: u32 = 262145u32;
