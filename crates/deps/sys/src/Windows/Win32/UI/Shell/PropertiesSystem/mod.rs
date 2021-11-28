#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type ClearPropVariantArray = unsafe extern "system" fn(rgpropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, cvars: u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type ClearVariantArray = unsafe extern "system" fn(pvars: *mut super::super::super::System::Com::VARIANT, cvars: u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromBooleanVector = unsafe extern "system" fn(prgf: *const super::super::super::Foundation::BOOL, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromBuffer = unsafe extern "system" fn(pv: *const ::core::ffi::c_void, cb: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromCLSID = unsafe extern "system" fn(clsid: *const ::windows_sys::core::GUID, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromDoubleVector = unsafe extern "system" fn(prgn: *const f64, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromFileTime = unsafe extern "system" fn(pftin: *const super::super::super::Foundation::FILETIME, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromFileTimeVector = unsafe extern "system" fn(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromGUIDAsString = unsafe extern "system" fn(guid: *const ::windows_sys::core::GUID, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromInt16Vector = unsafe extern "system" fn(prgn: *const i16, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromInt32Vector = unsafe extern "system" fn(prgn: *const i32, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromInt64Vector = unsafe extern "system" fn(prgn: *const i64, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromPropVariantVectorElem = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromResource = unsafe extern "system" fn(hinst: super::super::super::Foundation::HINSTANCE, id: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
pub type InitPropVariantFromStrRet = unsafe extern "system" fn(pstrret: *mut super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromStringAsVector = unsafe extern "system" fn(psz: super::super::super::Foundation::PWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromStringVector = unsafe extern "system" fn(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromUInt16Vector = unsafe extern "system" fn(prgn: *const u16, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromUInt32Vector = unsafe extern "system" fn(prgn: *const u32, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantFromUInt64Vector = unsafe extern "system" fn(prgn: *const u64, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type InitPropVariantVectorFromPropVariant = unsafe extern "system" fn(propvarsingle: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarvector: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromBooleanArray = unsafe extern "system" fn(prgf: *const super::super::super::Foundation::BOOL, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromBuffer = unsafe extern "system" fn(pv: *const ::core::ffi::c_void, cb: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromDoubleArray = unsafe extern "system" fn(prgn: *const f64, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromFileTime = unsafe extern "system" fn(pft: *const super::super::super::Foundation::FILETIME, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromFileTimeArray = unsafe extern "system" fn(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromGUIDAsString = unsafe extern "system" fn(guid: *const ::windows_sys::core::GUID, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromInt16Array = unsafe extern "system" fn(prgn: *const i16, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromInt32Array = unsafe extern "system" fn(prgn: *const i32, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromInt64Array = unsafe extern "system" fn(prgn: *const i64, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromResource = unsafe extern "system" fn(hinst: super::super::super::Foundation::HINSTANCE, id: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
pub type InitVariantFromStrRet = unsafe extern "system" fn(pstrret: *const super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromStringArray = unsafe extern "system" fn(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromUInt16Array = unsafe extern "system" fn(prgn: *const u16, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromUInt32Array = unsafe extern "system" fn(prgn: *const u32, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromUInt64Array = unsafe extern "system" fn(prgn: *const u64, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type InitVariantFromVariantArrayElem = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, ielem: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSCoerceToCanonicalValue = unsafe extern "system" fn(key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
pub type PSCreateAdapterFromPropertyStore = unsafe extern "system" fn(pps: IPropertyStore, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type PSCreateDelayedMultiplexPropertyStore = unsafe extern "system" fn(flags: GETPROPERTYSTOREFLAGS, pdpsf: IDelayedPropertyStoreFactory, rgstoreids: *const u32, cstores: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type PSCreateMemoryPropertyStore = unsafe extern "system" fn(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type PSCreateMultiplexPropertyStore = unsafe extern "system" fn(prgpunkstores: *const ::windows_sys::core::IUnknown, cstores: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSCreatePropertyChangeArray = unsafe extern "system" fn(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, cchanges: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type PSCreatePropertyStoreFromObject = unsafe extern "system" fn(punk: ::windows_sys::core::IUnknown, grfmode: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub type PSCreatePropertyStoreFromPropertySetStorage = unsafe extern "system" fn(ppss: super::super::super::System::Com::StructuredStorage::IPropertySetStorage, grfmode: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSCreateSimplePropertyChange = unsafe extern "system" fn(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type PSEnumeratePropertyDescriptions = unsafe extern "system" fn(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSFormatForDisplay = unsafe extern "system" fn(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSFormatForDisplayAlloc = unsafe extern "system" fn(key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSFormatPropertyValue = unsafe extern "system" fn(pps: IPropertyStore, ppd: IPropertyDescription, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSGetImageReferenceForValue = unsafe extern "system" fn(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSGetItemPropertyHandler = unsafe extern "system" fn(punkitem: ::windows_sys::core::IUnknown, freadwrite: super::super::super::Foundation::BOOL, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSGetItemPropertyHandlerWithCreateObject = unsafe extern "system" fn(punkitem: ::windows_sys::core::IUnknown, freadwrite: super::super::super::Foundation::BOOL, punkcreateobject: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSGetNameFromPropertyKey = unsafe extern "system" fn(propkey: *const PROPERTYKEY, ppszcanonicalname: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSGetNamedPropertyFromPropertyStorage = unsafe extern "system" fn(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: super::super::super::Foundation::PWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
pub type PSGetPropertyDescription = unsafe extern "system" fn(propkey: *const PROPERTYKEY, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSGetPropertyDescriptionByName = unsafe extern "system" fn(pszcanonicalname: super::super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSGetPropertyDescriptionListFromString = unsafe extern "system" fn(pszproplist: super::super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSGetPropertyFromPropertyStorage = unsafe extern "system" fn(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSGetPropertyKeyFromName = unsafe extern "system" fn(pszname: super::super::super::Foundation::PWSTR, ppropkey: *mut PROPERTYKEY) -> ::windows_sys::core::HRESULT;
pub type PSGetPropertySystem = unsafe extern "system" fn(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSGetPropertyValue = unsafe extern "system" fn(pps: IPropertyStore, ppd: IPropertyDescription, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSLookupPropertyHandlerCLSID = unsafe extern "system" fn(pszfilepath: super::super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_Delete = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadBOOL = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadBSTR = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadDWORD = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadGUID = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadInt = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut i32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadLONG = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut i32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadPOINTL = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::POINTL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadPOINTS = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::POINTS) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadPropertyKey = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut PROPERTYKEY) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadRECTL = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::RECTL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadSHORT = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut i16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadStr = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR, charactercount: i32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadStrAlloc = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadStream = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub type PSPropertyBag_ReadType = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, var: *mut super::super::super::System::Com::VARIANT, r#type: u16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadULONGLONG = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_ReadUnknown = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteBOOL = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteBSTR = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteDWORD = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteGUID = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteInt = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: i32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteLONG = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: i32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WritePOINTL = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::POINTL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WritePOINTS = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::POINTS) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WritePropertyKey = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const PROPERTYKEY) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteRECTL = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::RECTL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteSHORT = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: i16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteStr = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteStream = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteULONGLONG = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: u64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSPropertyBag_WriteUnknown = unsafe extern "system" fn(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSPropertyKeyFromString = unsafe extern "system" fn(pszstring: super::super::super::Foundation::PWSTR, pkey: *mut PROPERTYKEY) -> ::windows_sys::core::HRESULT;
pub type PSRefreshPropertySchema = unsafe extern "system" fn() -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSRegisterPropertySchema = unsafe extern "system" fn(pszpath: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PSSetPropertyValue = unsafe extern "system" fn(pps: IPropertyStore, ppd: IPropertyDescription, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSStringFromPropertyKey = unsafe extern "system" fn(pkey: *const PROPERTYKEY, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PSUnregisterPropertySchema = unsafe extern "system" fn(pszpath: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PifMgr_CloseProperties = unsafe extern "system" fn(hprops: super::super::super::Foundation::HANDLE, flopt: u32) -> super::super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type PifMgr_GetProperties = unsafe extern "system" fn(hprops: super::super::super::Foundation::HANDLE, pszgroup: super::super::super::Foundation::PSTR, lpprops: *mut ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PifMgr_OpenProperties = unsafe extern "system" fn(pszapp: super::super::super::Foundation::PWSTR, pszpif: super::super::super::Foundation::PWSTR, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type PifMgr_SetProperties = unsafe extern "system" fn(hprops: super::super::super::Foundation::HANDLE, pszgroup: super::super::super::Foundation::PSTR, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantChangeType = unsafe extern "system" fn(ppropvardest: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvarsrc: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantCompareEx = unsafe extern "system" fn(propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetBooleanElem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetDoubleElem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut f64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetElementCount = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetFileTimeElem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pftval: *mut super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetInt16Elem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetInt32Elem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetInt64Elem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetStringElem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, ppszval: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetUInt16Elem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetUInt32Elem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantGetUInt64Elem = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToBSTR = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pbstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToBoolean = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToBooleanVector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgf: *mut super::super::super::Foundation::BOOL, crgf: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToBooleanVectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToBooleanWithDefault = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToBuffer = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToDouble = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdblret: *mut f64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToDoubleVector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToDoubleVectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToDoubleWithDefault = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, dbldefault: f64) -> f64;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToFileTime = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToFileTimeVector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgft: *mut super::super::super::Foundation::FILETIME, crgft: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToFileTimeVectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToGUID = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt16 = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, piret: *mut i16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt16Vector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt16VectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt16WithDefault = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, idefault: i16) -> i16;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt32 = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, plret: *mut i32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt32Vector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt32VectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt32WithDefault = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ldefault: i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt64 = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pllret: *mut i64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt64Vector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt64VectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToInt64WithDefault = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, lldefault: i64) -> i64;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
pub type PropVariantToStrRet = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstrret: *mut super::Common::STRRET) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToString = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToStringAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszout: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToStringVector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToStringVectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToStringWithDefault = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pszdefault: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt16 = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiret: *mut u16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt16Vector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt16VectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt16WithDefault = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uidefault: u16) -> u16;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt32 = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pulret: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt32Vector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt32VectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt32WithDefault = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uldefault: u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt64 = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pullret: *mut u64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt64Vector = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt64VectorAlloc = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToUInt64WithDefault = unsafe extern "system" fn(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ulldefault: u64) -> u64;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub type PropVariantToVariant = unsafe extern "system" fn(ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type PropVariantToWinRTPropertyValue = unsafe extern "system" fn(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type SHAddDefaultPropertiesByExt = unsafe extern "system" fn(pszext: super::super::super::Foundation::PWSTR, ppropstore: IPropertyStore) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type SHGetPropertyStoreForWindow = unsafe extern "system" fn(hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_UI_Shell_Common")]
pub type SHGetPropertyStoreFromIDList = unsafe extern "system" fn(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type SHGetPropertyStoreFromParsingName = unsafe extern "system" fn(pszpath: super::super::super::Foundation::PWSTR, pbc: super::super::super::System::Com::IBindCtx, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub type SHPropStgCreate = unsafe extern "system" fn(psstg: super::super::super::System::Com::StructuredStorage::IPropertySetStorage, fmtid: *const ::windows_sys::core::GUID, pclsid: *const ::windows_sys::core::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut super::super::super::System::Com::StructuredStorage::IPropertyStorage, pucodepage: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type SHPropStgReadMultiple = unsafe extern "system" fn(pps: super::super::super::System::Com::StructuredStorage::IPropertyStorage, ucodepage: u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type SHPropStgWriteMultiple = unsafe extern "system" fn(pps: super::super::super::System::Com::StructuredStorage::IPropertyStorage, pucodepage: *mut u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantCompare = unsafe extern "system" fn(var1: *const super::super::super::System::Com::VARIANT, var2: *const super::super::super::System::Com::VARIANT) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetBooleanElem = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetDoubleElem = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut f64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetElementCount = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetInt16Elem = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut i16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetInt32Elem = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut i32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetInt64Elem = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut i64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetStringElem = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, ielem: u32, ppszval: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetUInt16Elem = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut u16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetUInt32Elem = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantGetUInt64Elem = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut u64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToBoolean = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToBooleanArray = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, prgf: *mut super::super::super::Foundation::BOOL, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToBooleanArrayAlloc = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToBooleanWithDefault = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToBuffer = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToDosDateTime = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToDouble = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pdblret: *mut f64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToDoubleArray = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToDoubleArrayAlloc = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToDoubleWithDefault = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, dbldefault: f64) -> f64;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToFileTime = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, stfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToGUID = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt16 = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, piret: *mut i16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt16Array = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt16ArrayAlloc = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt16WithDefault = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, idefault: i16) -> i16;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt32 = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, plret: *mut i32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt32Array = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt32ArrayAlloc = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt32WithDefault = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, ldefault: i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt64 = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pllret: *mut i64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt64Array = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt64ArrayAlloc = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToInt64WithDefault = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, lldefault: i64) -> i64;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub type VariantToPropVariant = unsafe extern "system" fn(pvar: *const super::super::super::System::Com::VARIANT, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
pub type VariantToStrRet = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pstrret: *mut super::Common::STRRET) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToString = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pszbuf: super::super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToStringAlloc = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, ppszbuf: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToStringArray = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToStringArrayAlloc = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToStringWithDefault = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pszdefault: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt16 = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, puiret: *mut u16) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt16Array = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt16ArrayAlloc = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt16WithDefault = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, uidefault: u16) -> u16;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt32 = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pulret: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt32Array = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt32ArrayAlloc = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt32WithDefault = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, uldefault: u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt64 = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, pullret: *mut u64) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt64Array = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt64ArrayAlloc = unsafe extern "system" fn(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type VariantToUInt64WithDefault = unsafe extern "system" fn(varin: *const super::super::super::System::Com::VARIANT, ulldefault: u64) -> u64;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub type WinRTPropertyValueToPropVariant = unsafe extern "system" fn(punkpropertyvalue: ::windows_sys::core::IUnknown, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
pub type DRAWPROGRESSFLAGS = i32;
pub const DPF_NONE: DRAWPROGRESSFLAGS = 0i32;
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = 1i32;
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = 2i32;
pub const DPF_ERROR: DRAWPROGRESSFLAGS = 4i32;
pub const DPF_WARNING: DRAWPROGRESSFLAGS = 8i32;
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = 16i32;
pub type GETPROPERTYSTOREFLAGS = i32;
pub const GPS_DEFAULT: GETPROPERTYSTOREFLAGS = 0i32;
pub const GPS_HANDLERPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 1i32;
pub const GPS_READWRITE: GETPROPERTYSTOREFLAGS = 2i32;
pub const GPS_TEMPORARY: GETPROPERTYSTOREFLAGS = 4i32;
pub const GPS_FASTPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 8i32;
pub const GPS_OPENSLOWITEM: GETPROPERTYSTOREFLAGS = 16i32;
pub const GPS_DELAYCREATION: GETPROPERTYSTOREFLAGS = 32i32;
pub const GPS_BESTEFFORT: GETPROPERTYSTOREFLAGS = 64i32;
pub const GPS_NO_OPLOCK: GETPROPERTYSTOREFLAGS = 128i32;
pub const GPS_PREFERQUERYPROPERTIES: GETPROPERTYSTOREFLAGS = 256i32;
pub const GPS_EXTRINSICPROPERTIES: GETPROPERTYSTOREFLAGS = 512i32;
pub const GPS_EXTRINSICPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 1024i32;
pub const GPS_VOLATILEPROPERTIES: GETPROPERTYSTOREFLAGS = 2048i32;
pub const GPS_VOLATILEPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 4096i32;
pub const GPS_MASK_VALID: GETPROPERTYSTOREFLAGS = 8191i32;
pub type ICreateObject = *mut ::core::ffi::c_void;
pub type IDelayedPropertyStoreFactory = *mut ::core::ffi::c_void;
pub type IInitializeWithFile = *mut ::core::ffi::c_void;
pub type IInitializeWithStream = *mut ::core::ffi::c_void;
pub type INamedPropertyStore = *mut ::core::ffi::c_void;
pub type IObjectWithPropertyKey = *mut ::core::ffi::c_void;
pub type IPersistSerializedPropStorage = *mut ::core::ffi::c_void;
pub type IPersistSerializedPropStorage2 = *mut ::core::ffi::c_void;
pub type IPropertyChange = *mut ::core::ffi::c_void;
pub type IPropertyChangeArray = *mut ::core::ffi::c_void;
pub type IPropertyDescription = *mut ::core::ffi::c_void;
pub type IPropertyDescription2 = *mut ::core::ffi::c_void;
pub type IPropertyDescriptionAliasInfo = *mut ::core::ffi::c_void;
pub type IPropertyDescriptionList = *mut ::core::ffi::c_void;
pub type IPropertyDescriptionRelatedPropertyInfo = *mut ::core::ffi::c_void;
pub type IPropertyDescriptionSearchInfo = *mut ::core::ffi::c_void;
pub type IPropertyEnumType = *mut ::core::ffi::c_void;
pub type IPropertyEnumType2 = *mut ::core::ffi::c_void;
pub type IPropertyEnumTypeList = *mut ::core::ffi::c_void;
pub type IPropertyStore = *mut ::core::ffi::c_void;
pub type IPropertyStoreCache = *mut ::core::ffi::c_void;
pub type IPropertyStoreCapabilities = *mut ::core::ffi::c_void;
pub type IPropertyStoreFactory = *mut ::core::ffi::c_void;
pub type IPropertySystem = *mut ::core::ffi::c_void;
pub type IPropertySystemChangeNotify = *mut ::core::ffi::c_void;
pub type IPropertyUI = *mut ::core::ffi::c_void;
pub const InMemoryPropertyStore: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2583879698,
    data2: 25347,
    data3: 19998,
    data4: [185, 161, 99, 15, 128, 37, 146, 197],
};
pub const InMemoryPropertyStoreMarshalByValue: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3570011693,
    data2: 28071,
    data3: 19317,
    data4: [169, 124, 95, 48, 111, 14, 174, 220],
};
pub type PDOPSTATUS = i32;
pub const PDOPS_RUNNING: PDOPSTATUS = 1i32;
pub const PDOPS_PAUSED: PDOPSTATUS = 2i32;
pub const PDOPS_CANCELLED: PDOPSTATUS = 3i32;
pub const PDOPS_STOPPED: PDOPSTATUS = 4i32;
pub const PDOPS_ERRORS: PDOPSTATUS = 5i32;
pub type PKA_FLAGS = i32;
pub const PKA_SET: PKA_FLAGS = 0i32;
pub const PKA_APPEND: PKA_FLAGS = 1i32;
pub const PKA_DELETE: PKA_FLAGS = 2i32;
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
pub type PLACEHOLDER_STATES = i32;
pub const PS_NONE: PLACEHOLDER_STATES = 0i32;
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = 1i32;
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = 2i32;
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = 4i32;
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = 8i32;
pub const PS_DEFAULT: PLACEHOLDER_STATES = 7i32;
pub const PS_ALL: PLACEHOLDER_STATES = 15i32;
pub type PROPDESC_AGGREGATION_TYPE = i32;
pub const PDAT_DEFAULT: PROPDESC_AGGREGATION_TYPE = 0i32;
pub const PDAT_FIRST: PROPDESC_AGGREGATION_TYPE = 1i32;
pub const PDAT_SUM: PROPDESC_AGGREGATION_TYPE = 2i32;
pub const PDAT_AVERAGE: PROPDESC_AGGREGATION_TYPE = 3i32;
pub const PDAT_DATERANGE: PROPDESC_AGGREGATION_TYPE = 4i32;
pub const PDAT_UNION: PROPDESC_AGGREGATION_TYPE = 5i32;
pub const PDAT_MAX: PROPDESC_AGGREGATION_TYPE = 6i32;
pub const PDAT_MIN: PROPDESC_AGGREGATION_TYPE = 7i32;
pub type PROPDESC_COLUMNINDEX_TYPE = i32;
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = 0i32;
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = 1i32;
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = 2i32;
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = 3i32;
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = 4i32;
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = 5i32;
pub type PROPDESC_CONDITION_TYPE = i32;
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = 0i32;
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = 1i32;
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = 2i32;
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = 3i32;
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = 4i32;
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = 5i32;
pub type PROPDESC_DISPLAYTYPE = i32;
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = 0i32;
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = 1i32;
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = 2i32;
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = 3i32;
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = 4i32;
pub type PROPDESC_ENUMFILTER = i32;
pub const PDEF_ALL: PROPDESC_ENUMFILTER = 0i32;
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = 1i32;
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = 2i32;
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = 3i32;
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = 4i32;
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = 5i32;
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = 6i32;
pub type PROPDESC_FORMAT_FLAGS = i32;
pub const PDFF_DEFAULT: PROPDESC_FORMAT_FLAGS = 0i32;
pub const PDFF_PREFIXNAME: PROPDESC_FORMAT_FLAGS = 1i32;
pub const PDFF_FILENAME: PROPDESC_FORMAT_FLAGS = 2i32;
pub const PDFF_ALWAYSKB: PROPDESC_FORMAT_FLAGS = 4i32;
pub const PDFF_RESERVED_RIGHTTOLEFT: PROPDESC_FORMAT_FLAGS = 8i32;
pub const PDFF_SHORTTIME: PROPDESC_FORMAT_FLAGS = 16i32;
pub const PDFF_LONGTIME: PROPDESC_FORMAT_FLAGS = 32i32;
pub const PDFF_HIDETIME: PROPDESC_FORMAT_FLAGS = 64i32;
pub const PDFF_SHORTDATE: PROPDESC_FORMAT_FLAGS = 128i32;
pub const PDFF_LONGDATE: PROPDESC_FORMAT_FLAGS = 256i32;
pub const PDFF_HIDEDATE: PROPDESC_FORMAT_FLAGS = 512i32;
pub const PDFF_RELATIVEDATE: PROPDESC_FORMAT_FLAGS = 1024i32;
pub const PDFF_USEEDITINVITATION: PROPDESC_FORMAT_FLAGS = 2048i32;
pub const PDFF_READONLY: PROPDESC_FORMAT_FLAGS = 4096i32;
pub const PDFF_NOAUTOREADINGORDER: PROPDESC_FORMAT_FLAGS = 8192i32;
pub type PROPDESC_GROUPING_RANGE = i32;
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = 0i32;
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = 1i32;
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = 2i32;
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = 3i32;
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = 4i32;
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = 5i32;
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = 6i32;
pub type PROPDESC_RELATIVEDESCRIPTION_TYPE = i32;
pub const PDRDT_GENERAL: PROPDESC_RELATIVEDESCRIPTION_TYPE = 0i32;
pub const PDRDT_DATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 1i32;
pub const PDRDT_SIZE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 2i32;
pub const PDRDT_COUNT: PROPDESC_RELATIVEDESCRIPTION_TYPE = 3i32;
pub const PDRDT_REVISION: PROPDESC_RELATIVEDESCRIPTION_TYPE = 4i32;
pub const PDRDT_LENGTH: PROPDESC_RELATIVEDESCRIPTION_TYPE = 5i32;
pub const PDRDT_DURATION: PROPDESC_RELATIVEDESCRIPTION_TYPE = 6i32;
pub const PDRDT_SPEED: PROPDESC_RELATIVEDESCRIPTION_TYPE = 7i32;
pub const PDRDT_RATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 8i32;
pub const PDRDT_RATING: PROPDESC_RELATIVEDESCRIPTION_TYPE = 9i32;
pub const PDRDT_PRIORITY: PROPDESC_RELATIVEDESCRIPTION_TYPE = 10i32;
pub type PROPDESC_SEARCHINFO_FLAGS = i32;
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = 0i32;
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = 1i32;
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = 2i32;
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = 4i32;
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = 8i32;
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = 16i32;
pub type PROPDESC_SORTDESCRIPTION = i32;
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = 0i32;
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = 1i32;
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = 2i32;
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = 3i32;
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = 4i32;
pub type PROPDESC_TYPE_FLAGS = i32;
pub const PDTF_DEFAULT: PROPDESC_TYPE_FLAGS = 0i32;
pub const PDTF_MULTIPLEVALUES: PROPDESC_TYPE_FLAGS = 1i32;
pub const PDTF_ISINNATE: PROPDESC_TYPE_FLAGS = 2i32;
pub const PDTF_ISGROUP: PROPDESC_TYPE_FLAGS = 4i32;
pub const PDTF_CANGROUPBY: PROPDESC_TYPE_FLAGS = 8i32;
pub const PDTF_CANSTACKBY: PROPDESC_TYPE_FLAGS = 16i32;
pub const PDTF_ISTREEPROPERTY: PROPDESC_TYPE_FLAGS = 32i32;
pub const PDTF_INCLUDEINFULLTEXTQUERY: PROPDESC_TYPE_FLAGS = 64i32;
pub const PDTF_ISVIEWABLE: PROPDESC_TYPE_FLAGS = 128i32;
pub const PDTF_ISQUERYABLE: PROPDESC_TYPE_FLAGS = 256i32;
pub const PDTF_CANBEPURGED: PROPDESC_TYPE_FLAGS = 512i32;
pub const PDTF_SEARCHRAWVALUE: PROPDESC_TYPE_FLAGS = 1024i32;
pub const PDTF_DONTCOERCEEMPTYSTRINGS: PROPDESC_TYPE_FLAGS = 2048i32;
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: PROPDESC_TYPE_FLAGS = 4096i32;
pub const PDTF_ISSYSTEMPROPERTY: PROPDESC_TYPE_FLAGS = -2147483648i32;
pub const PDTF_MASK_ALL: PROPDESC_TYPE_FLAGS = -2147475457i32;
pub type PROPDESC_VIEW_FLAGS = i32;
pub const PDVF_DEFAULT: PROPDESC_VIEW_FLAGS = 0i32;
pub const PDVF_CENTERALIGN: PROPDESC_VIEW_FLAGS = 1i32;
pub const PDVF_RIGHTALIGN: PROPDESC_VIEW_FLAGS = 2i32;
pub const PDVF_BEGINNEWGROUP: PROPDESC_VIEW_FLAGS = 4i32;
pub const PDVF_FILLAREA: PROPDESC_VIEW_FLAGS = 8i32;
pub const PDVF_SORTDESCENDING: PROPDESC_VIEW_FLAGS = 16i32;
pub const PDVF_SHOWONLYIFPRESENT: PROPDESC_VIEW_FLAGS = 32i32;
pub const PDVF_SHOWBYDEFAULT: PROPDESC_VIEW_FLAGS = 64i32;
pub const PDVF_SHOWINPRIMARYLIST: PROPDESC_VIEW_FLAGS = 128i32;
pub const PDVF_SHOWINSECONDARYLIST: PROPDESC_VIEW_FLAGS = 256i32;
pub const PDVF_HIDELABEL: PROPDESC_VIEW_FLAGS = 512i32;
pub const PDVF_HIDDEN: PROPDESC_VIEW_FLAGS = 2048i32;
pub const PDVF_CANWRAP: PROPDESC_VIEW_FLAGS = 4096i32;
pub const PDVF_MASK_ALL: PROPDESC_VIEW_FLAGS = 7167i32;
pub type PROPENUMTYPE = i32;
pub const PET_DISCRETEVALUE: PROPENUMTYPE = 0i32;
pub const PET_RANGEDVALUE: PROPENUMTYPE = 1i32;
pub const PET_DEFAULTVALUE: PROPENUMTYPE = 2i32;
pub const PET_ENDRANGE: PROPENUMTYPE = 3i32;
#[repr(C)]
pub struct PROPERTYKEY {
    pub fmtid: ::windows_sys::core::GUID,
    pub pid: u32,
}
impl ::core::marker::Copy for PROPERTYKEY {}
impl ::core::clone::Clone for PROPERTYKEY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROPERTYUI_FLAGS = i32;
pub const PUIF_DEFAULT: PROPERTYUI_FLAGS = 0i32;
pub const PUIF_RIGHTALIGN: PROPERTYUI_FLAGS = 1i32;
pub const PUIF_NOLABELININFOTIP: PROPERTYUI_FLAGS = 2i32;
pub type PROPERTYUI_FORMAT_FLAGS = i32;
pub const PUIFFDF_DEFAULT: PROPERTYUI_FORMAT_FLAGS = 0i32;
pub const PUIFFDF_RIGHTTOLEFT: PROPERTYUI_FORMAT_FLAGS = 1i32;
pub const PUIFFDF_SHORTFORMAT: PROPERTYUI_FORMAT_FLAGS = 2i32;
pub const PUIFFDF_NOTIME: PROPERTYUI_FORMAT_FLAGS = 4i32;
pub const PUIFFDF_FRIENDLYDATE: PROPERTYUI_FORMAT_FLAGS = 8i32;
pub type PROPERTYUI_NAME_FLAGS = i32;
pub const PUIFNF_DEFAULT: PROPERTYUI_NAME_FLAGS = 0i32;
pub const PUIFNF_MNEMONIC: PROPERTYUI_NAME_FLAGS = 1i32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [super::super::super::Foundation::CHAR; 30],
    pub achCmdLine: [super::super::super::Foundation::CHAR; 128],
    pub achWorkDir: [super::super::super::Foundation::CHAR; 64],
    pub wHotKey: u16,
    pub achIconFile: [super::super::super::Foundation::CHAR; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [super::super::super::Foundation::CHAR; 80],
    pub achPIFFile: [super::super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPPRG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPPRG {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROPVAR_CHANGE_FLAGS = i32;
pub const PVCHF_DEFAULT: PROPVAR_CHANGE_FLAGS = 0i32;
pub const PVCHF_NOVALUEPROP: PROPVAR_CHANGE_FLAGS = 1i32;
pub const PVCHF_ALPHABOOL: PROPVAR_CHANGE_FLAGS = 2i32;
pub const PVCHF_NOUSEROVERRIDE: PROPVAR_CHANGE_FLAGS = 4i32;
pub const PVCHF_LOCALBOOL: PROPVAR_CHANGE_FLAGS = 8i32;
pub const PVCHF_NOHEXSTRING: PROPVAR_CHANGE_FLAGS = 16i32;
pub type PROPVAR_COMPARE_FLAGS = i32;
pub const PVCF_DEFAULT: PROPVAR_COMPARE_FLAGS = 0i32;
pub const PVCF_TREATEMPTYASGREATERTHAN: PROPVAR_COMPARE_FLAGS = 1i32;
pub const PVCF_USESTRCMP: PROPVAR_COMPARE_FLAGS = 2i32;
pub const PVCF_USESTRCMPC: PROPVAR_COMPARE_FLAGS = 4i32;
pub const PVCF_USESTRCMPI: PROPVAR_COMPARE_FLAGS = 8i32;
pub const PVCF_USESTRCMPIC: PROPVAR_COMPARE_FLAGS = 16i32;
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: PROPVAR_COMPARE_FLAGS = 32i32;
pub type PROPVAR_COMPARE_UNIT = i32;
pub const PVCU_DEFAULT: PROPVAR_COMPARE_UNIT = 0i32;
pub const PVCU_SECOND: PROPVAR_COMPARE_UNIT = 1i32;
pub const PVCU_MINUTE: PROPVAR_COMPARE_UNIT = 2i32;
pub const PVCU_HOUR: PROPVAR_COMPARE_UNIT = 3i32;
pub const PVCU_DAY: PROPVAR_COMPARE_UNIT = 4i32;
pub const PVCU_MONTH: PROPVAR_COMPARE_UNIT = 5i32;
pub const PVCU_YEAR: PROPVAR_COMPARE_UNIT = 6i32;
pub type PSC_STATE = i32;
pub const PSC_NORMAL: PSC_STATE = 0i32;
pub const PSC_NOTINSOURCE: PSC_STATE = 1i32;
pub const PSC_DIRTY: PSC_STATE = 2i32;
pub const PSC_READONLY: PSC_STATE = 3i32;
pub type PSTIME_FLAGS = i32;
pub const PSTF_UTC: PSTIME_FLAGS = 0i32;
pub const PSTF_LOCAL: PSTIME_FLAGS = 1i32;
pub const PropertySystem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3096870789, data2: 22702, data3: 20294, data4: [159, 178, 93, 121, 4, 121, 143, 75] };
#[repr(C)]
pub struct SERIALIZEDPROPSTORAGE(pub u8);
pub type SYNC_ENGINE_STATE_FLAGS = i32;
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = 0i32;
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = 1i32;
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = 2i32;
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = 4i32;
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = 8i32;
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = 16i32;
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = 32i32;
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = 64i32;
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = 128i32;
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = 256i32;
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = 511i32;
pub type SYNC_TRANSFER_STATUS = i32;
pub const STS_NONE: SYNC_TRANSFER_STATUS = 0i32;
pub const STS_NEEDSUPLOAD: SYNC_TRANSFER_STATUS = 1i32;
pub const STS_NEEDSDOWNLOAD: SYNC_TRANSFER_STATUS = 2i32;
pub const STS_TRANSFERRING: SYNC_TRANSFER_STATUS = 4i32;
pub const STS_PAUSED: SYNC_TRANSFER_STATUS = 8i32;
pub const STS_HASERROR: SYNC_TRANSFER_STATUS = 16i32;
pub const STS_FETCHING_METADATA: SYNC_TRANSFER_STATUS = 32i32;
pub const STS_USER_REQUESTED_REFRESH: SYNC_TRANSFER_STATUS = 64i32;
pub const STS_HASWARNING: SYNC_TRANSFER_STATUS = 128i32;
pub const STS_EXCLUDED: SYNC_TRANSFER_STATUS = 256i32;
pub const STS_INCOMPLETE: SYNC_TRANSFER_STATUS = 512i32;
pub const STS_PLACEHOLDER_IFEMPTY: SYNC_TRANSFER_STATUS = 1024i32;
pub type _PERSIST_SPROPSTORE_FLAGS = i32;
pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = 0i32;
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = 1i32;
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = 2i32;
