#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type CoGetInstanceFromFile = unsafe extern "system" fn(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: super::super::super::Foundation::PWSTR, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CoGetInstanceFromIStorage = unsafe extern "system" fn(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: super::CLSCTX, pstg: IStorage, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows_sys::core::HRESULT;
pub type CoGetInterfaceAndReleaseStream = unsafe extern "system" fn(pstm: super::IStream, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CreateILockBytesOnHGlobal = unsafe extern "system" fn(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, pplkbyt: *mut ILockBytes) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CreateStreamOnHGlobal = unsafe extern "system" fn(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, ppstm: *mut super::IStream) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FmtIdToPropStgName = unsafe extern "system" fn(pfmtid: *const ::windows_sys::core::GUID, oszname: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FreePropVariantArray = unsafe extern "system" fn(cvariants: u32, rgvars: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
pub type GetConvertStg = unsafe extern "system" fn(pstg: IStorage) -> ::windows_sys::core::HRESULT;
pub type GetHGlobalFromILockBytes = unsafe extern "system" fn(plkbyt: ILockBytes, phglobal: *mut isize) -> ::windows_sys::core::HRESULT;
pub type GetHGlobalFromStream = unsafe extern "system" fn(pstm: super::IStream, phglobal: *mut isize) -> ::windows_sys::core::HRESULT;
pub type OleConvertIStorageToOLESTREAM = unsafe extern "system" fn(pstg: IStorage, lpolestream: *mut OLESTREAM) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type OleConvertIStorageToOLESTREAMEx = unsafe extern "system" fn(pstg: IStorage, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut super::STGMEDIUM, polestm: *mut OLESTREAM) -> ::windows_sys::core::HRESULT;
pub type OleConvertOLESTREAMToIStorage = unsafe extern "system" fn(lpolestream: *mut OLESTREAM, pstg: IStorage, ptd: *const super::DVTARGETDEVICE) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type OleConvertOLESTREAMToIStorageEx = unsafe extern "system" fn(polestm: *mut OLESTREAM, pstg: IStorage, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PropStgNameToFmtId = unsafe extern "system" fn(oszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PropVariantClear = unsafe extern "system" fn(pvar: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PropVariantCopy = unsafe extern "system" fn(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows_sys::core::HRESULT;
pub type ReadClassStg = unsafe extern "system" fn(pstg: IStorage, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
pub type ReadClassStm = unsafe extern "system" fn(pstm: super::IStream, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type ReadFmtUserTypeStg = unsafe extern "system" fn(pstg: IStorage, pcf: *mut u16, lplpszusertype: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type SetConvertStg = unsafe extern "system" fn(pstg: IStorage, fconvert: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type StgConvertPropertyToVariant = unsafe extern "system" fn(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN;
#[cfg(feature = "Win32_Foundation")]
pub type StgConvertVariantToProperty = unsafe extern "system" fn(pvar: *const PROPVARIANT, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: super::super::super::Foundation::BOOLEAN, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE;
#[cfg(feature = "Win32_Foundation")]
pub type StgCreateDocfile = unsafe extern "system" fn(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
pub type StgCreateDocfileOnILockBytes = unsafe extern "system" fn(plkbyt: ILockBytes, grfmode: u32, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
pub type StgCreatePropSetStg = unsafe extern "system" fn(pstorage: IStorage, dwreserved: u32, pppropsetstg: *mut IPropertySetStorage) -> ::windows_sys::core::HRESULT;
pub type StgCreatePropStg = unsafe extern "system" fn(punk: ::windows_sys::core::IUnknown, fmtid: *const ::windows_sys::core::GUID, pclsid: *const ::windows_sys::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut IPropertyStorage) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type StgCreateStorageEx = unsafe extern "system" fn(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows_sys::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type StgDeserializePropVariant = unsafe extern "system" fn(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32, ppropvar: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type StgGetIFillLockBytesOnFile = unsafe extern "system" fn(pwcsname: super::super::super::Foundation::PWSTR, ppflb: *mut IFillLockBytes) -> ::windows_sys::core::HRESULT;
pub type StgGetIFillLockBytesOnILockBytes = unsafe extern "system" fn(pilb: ILockBytes, ppflb: *mut IFillLockBytes) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type StgIsStorageFile = unsafe extern "system" fn(pwcsname: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type StgIsStorageILockBytes = unsafe extern "system" fn(plkbyt: ILockBytes) -> ::windows_sys::core::HRESULT;
pub type StgOpenAsyncDocfileOnIFillLockBytes = unsafe extern "system" fn(pflb: IFillLockBytes, grfmode: u32, asyncflags: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type StgOpenLayoutDocfile = unsafe extern "system" fn(pwcsdfname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
pub type StgOpenPropStg = unsafe extern "system" fn(punk: ::windows_sys::core::IUnknown, fmtid: *const ::windows_sys::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut IPropertyStorage) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type StgOpenStorage = unsafe extern "system" fn(pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: IStorage, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type StgOpenStorageEx = unsafe extern "system" fn(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows_sys::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type StgOpenStorageOnILockBytes = unsafe extern "system" fn(plkbyt: ILockBytes, pstgpriority: IStorage, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
pub type StgPropertyLengthAsVariant = unsafe extern "system" fn(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type StgSerializePropVariant = unsafe extern "system" fn(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type StgSetTimes = unsafe extern "system" fn(lpszname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
pub type WriteClassStg = unsafe extern "system" fn(pstg: IStorage, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
pub type WriteClassStm = unsafe extern "system" fn(pstm: super::IStream, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type WriteFmtUserTypeStg = unsafe extern "system" fn(pstg: IStorage, cf: u16, lpszusertype: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BSTRBLOB {}
impl ::core::clone::Clone for BSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl ::core::marker::Copy for CABOOL {}
impl ::core::clone::Clone for CABOOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
impl ::core::marker::Copy for CABSTRBLOB {}
impl ::core::clone::Clone for CABSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CAC {
    pub cElems: u32,
    pub pElems: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
impl ::core::marker::Copy for CACLIPDATA {}
impl ::core::clone::Clone for CACLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CACLSID {}
impl ::core::clone::Clone for CACLSID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::CY,
}
impl ::core::marker::Copy for CACY {}
impl ::core::clone::Clone for CACY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl ::core::marker::Copy for CADATE {}
impl ::core::clone::Clone for CADATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl ::core::marker::Copy for CADBL {}
impl ::core::clone::Clone for CADBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAFILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl ::core::marker::Copy for CAFLT {}
impl ::core::clone::Clone for CAFLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl ::core::marker::Copy for CAH {}
impl ::core::clone::Clone for CAH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl ::core::marker::Copy for CAI {}
impl ::core::clone::Clone for CAI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl ::core::marker::Copy for CAL {}
impl ::core::clone::Clone for CAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALPSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALPSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALPWSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAPROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAPROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl ::core::marker::Copy for CASCODE {}
impl ::core::clone::Clone for CASCODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl ::core::marker::Copy for CAUB {}
impl ::core::clone::Clone for CAUB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl ::core::marker::Copy for CAUH {}
impl ::core::clone::Clone for CAUH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl ::core::marker::Copy for CAUI {}
impl ::core::clone::Clone for CAUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl ::core::marker::Copy for CAUL {}
impl ::core::clone::Clone for CAUL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
#[repr(C)]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
impl ::core::marker::Copy for CLIPDATA {}
impl ::core::clone::Clone for CLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CWCSTORAGENAME: u32 = 32u32;
pub type IDirectWriterLock = *mut ::core::ffi::c_void;
pub type IEnumSTATPROPSETSTG = *mut ::core::ffi::c_void;
pub type IEnumSTATPROPSTG = *mut ::core::ffi::c_void;
pub type IEnumSTATSTG = *mut ::core::ffi::c_void;
pub type IFillLockBytes = *mut ::core::ffi::c_void;
pub type ILayoutStorage = *mut ::core::ffi::c_void;
pub type ILockBytes = *mut ::core::ffi::c_void;
pub type IPersistStorage = *mut ::core::ffi::c_void;
pub type IPropertyBag = *mut ::core::ffi::c_void;
pub type IPropertyBag2 = *mut ::core::ffi::c_void;
pub type IPropertySetStorage = *mut ::core::ffi::c_void;
pub type IPropertyStorage = *mut ::core::ffi::c_void;
pub type IRootStorage = *mut ::core::ffi::c_void;
pub type IStorage = *mut ::core::ffi::c_void;
pub type LOCKTYPE = i32;
pub const LOCK_WRITE: LOCKTYPE = 1i32;
pub const LOCK_EXCLUSIVE: LOCKTYPE = 2i32;
pub const LOCK_ONLYONCE: LOCKTYPE = 4i32;
#[repr(C)]
pub struct OLESTREAM {
    pub lpstbl: *mut OLESTREAMVTBL,
}
impl ::core::marker::Copy for OLESTREAM {}
impl ::core::clone::Clone for OLESTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OLESTREAMVTBL {
    pub Get: isize,
    pub Put: isize,
}
impl ::core::marker::Copy for OLESTREAMVTBL {}
impl ::core::clone::Clone for OLESTREAMVTBL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PIDDI_THUMBNAIL: i32 = 2i32;
pub const PIDDSI_BYTECOUNT: u32 = 4u32;
pub const PIDDSI_CATEGORY: u32 = 2u32;
pub const PIDDSI_COMPANY: u32 = 15u32;
pub const PIDDSI_DOCPARTS: u32 = 13u32;
pub const PIDDSI_HEADINGPAIR: u32 = 12u32;
pub const PIDDSI_HIDDENCOUNT: u32 = 9u32;
pub const PIDDSI_LINECOUNT: u32 = 5u32;
pub const PIDDSI_LINKSDIRTY: u32 = 16u32;
pub const PIDDSI_MANAGER: u32 = 14u32;
pub const PIDDSI_MMCLIPCOUNT: u32 = 10u32;
pub const PIDDSI_NOTECOUNT: u32 = 8u32;
pub const PIDDSI_PARCOUNT: u32 = 6u32;
pub const PIDDSI_PRESFORMAT: u32 = 3u32;
pub const PIDDSI_SCALE: u32 = 11u32;
pub const PIDDSI_SLIDECOUNT: u32 = 7u32;
pub const PIDMSI_COPYRIGHT: i32 = 11i32;
pub const PIDMSI_EDITOR: i32 = 2i32;
pub const PIDMSI_OWNER: i32 = 8i32;
pub const PIDMSI_PRODUCTION: i32 = 10i32;
pub const PIDMSI_PROJECT: i32 = 6i32;
pub const PIDMSI_RATING: i32 = 9i32;
pub const PIDMSI_SEQUENCE_NO: i32 = 5i32;
pub const PIDMSI_SOURCE: i32 = 4i32;
pub const PIDMSI_STATUS: i32 = 7i32;
pub type PIDMSI_STATUS_VALUE = i32;
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = 0i32;
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = 1i32;
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = 2i32;
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = 3i32;
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = 4i32;
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = 5i32;
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = 6i32;
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = 7i32;
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = 8i32;
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = 32767i32;
pub const PIDMSI_SUPPLIER: i32 = 3i32;
pub const PIDSI_APPNAME: i32 = 18i32;
pub const PIDSI_AUTHOR: i32 = 4i32;
pub const PIDSI_CHARCOUNT: i32 = 16i32;
pub const PIDSI_COMMENTS: i32 = 6i32;
pub const PIDSI_CREATE_DTM: i32 = 12i32;
pub const PIDSI_DOC_SECURITY: i32 = 19i32;
pub const PIDSI_EDITTIME: i32 = 10i32;
pub const PIDSI_KEYWORDS: i32 = 5i32;
pub const PIDSI_LASTAUTHOR: i32 = 8i32;
pub const PIDSI_LASTPRINTED: i32 = 11i32;
pub const PIDSI_LASTSAVE_DTM: i32 = 13i32;
pub const PIDSI_PAGECOUNT: i32 = 14i32;
pub const PIDSI_REVNUMBER: i32 = 9i32;
pub const PIDSI_SUBJECT: i32 = 3i32;
pub const PIDSI_TEMPLATE: i32 = 7i32;
pub const PIDSI_THUMBNAIL: i32 = 17i32;
pub const PIDSI_TITLE: i32 = 2i32;
pub const PIDSI_WORDCOUNT: i32 = 15i32;
pub const PID_BEHAVIOR: u32 = 2147483651u32;
pub const PID_CODEPAGE: u32 = 1u32;
pub const PID_DICTIONARY: u32 = 0u32;
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
pub const PID_FIRST_USABLE: u32 = 2u32;
pub const PID_ILLEGAL: u32 = 4294967295u32;
pub const PID_LOCALE: u32 = 2147483648u32;
pub const PID_MAX_READONLY: u32 = 3221225471u32;
pub const PID_MIN_READONLY: u32 = 2147483648u32;
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
pub const PID_SECURITY: u32 = 2147483650u32;
#[repr(C)]
pub struct PMemoryAllocator(pub u8);
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: u16,
    pub cfType: u16,
    pub dwHint: u32,
    pub pstrName: super::super::super::Foundation::PWSTR,
    pub clsid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPBAG2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPBAG2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROPSETFLAG_ANSI: u32 = 2u32;
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPSPEC {
    pub ulKind: PROPSPEC_KIND,
    pub Anonymous: PROPSPEC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROPSPEC_KIND = u32;
pub const PRSPEC_LPWSTR: PROPSPEC_KIND = 0u32;
pub const PRSPEC_PROPID: PROPSPEC_KIND = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PROPVARIANT_0 {
    pub Anonymous: PROPVARIANT_0_0,
    pub decVal: super::super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PROPVARIANT_0_0_0 {
    pub cVal: super::super::super::Foundation::CHAR,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub __OBSOLETE__VARIANT_BOOL: i16,
    pub scode: i32,
    pub cyVal: super::CY,
    pub date: f64,
    pub filetime: super::super::super::Foundation::FILETIME,
    pub puuid: *mut ::windows_sys::core::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: super::super::super::Foundation::BSTR,
    pub bstrblobVal: BSTRBLOB,
    pub blob: super::BLOB,
    pub pszVal: super::super::super::Foundation::PSTR,
    pub pwszVal: super::super::super::Foundation::PWSTR,
    pub punkVal: ::windows_sys::core::IUnknown,
    pub pdispVal: super::IDispatch,
    pub pStream: super::IStream,
    pub pStorage: IStorage,
    pub pVersionedStream: *mut VERSIONEDSTREAM,
    pub parray: *mut super::SAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: super::super::super::Foundation::PSTR,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub pdecVal: *mut super::super::super::Foundation::DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut super::super::super::Foundation::BSTR,
    pub ppunkVal: *mut ::windows_sys::core::IUnknown,
    pub ppdispVal: *mut super::IDispatch,
    pub pparray: *mut *mut super::SAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PRSPEC_INVALID: u32 = 4294967295u32;
#[repr(C)]
pub struct RemSNB {
    pub ulCntStr: u32,
    pub ulCntChar: u32,
    pub rgString: [u16; 1],
}
impl ::core::marker::Copy for RemSNB {}
impl ::core::clone::Clone for RemSNB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERIALIZEDPROPERTYVALUE {
    pub dwType: u32,
    pub rgb: [u8; 1],
}
impl ::core::marker::Copy for SERIALIZEDPROPERTYVALUE {}
impl ::core::clone::Clone for SERIALIZEDPROPERTYVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type STATFLAG = i32;
pub const STATFLAG_DEFAULT: STATFLAG = 0i32;
pub const STATFLAG_NONAME: STATFLAG = 1i32;
pub const STATFLAG_NOOPEN: STATFLAG = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSETSTG {
    pub fmtid: ::windows_sys::core::GUID,
    pub clsid: ::windows_sys::core::GUID,
    pub grfFlags: u32,
    pub mtime: super::super::super::Foundation::FILETIME,
    pub ctime: super::super::super::Foundation::FILETIME,
    pub atime: super::super::super::Foundation::FILETIME,
    pub dwOSVersion: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATPROPSETSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATPROPSETSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSTG {
    pub lpwstrName: super::super::super::Foundation::PWSTR,
    pub propid: u32,
    pub vt: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATPROPSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATPROPSTG {
    fn clone(&self) -> Self {
        *self
    }
}
pub type STGC = i32;
pub const STGC_DEFAULT: STGC = 0i32;
pub const STGC_OVERWRITE: STGC = 1i32;
pub const STGC_ONLYIFCURRENT: STGC = 2i32;
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = 4i32;
pub const STGC_CONSOLIDATE: STGC = 8i32;
pub const STGFMT_ANY: u32 = 4u32;
pub const STGFMT_DOCFILE: u32 = 5u32;
pub const STGFMT_DOCUMENT: u32 = 0u32;
pub const STGFMT_FILE: u32 = 3u32;
pub const STGFMT_NATIVE: u32 = 1u32;
pub const STGFMT_STORAGE: u32 = 0u32;
pub type STGMOVE = i32;
pub const STGMOVE_MOVE: STGMOVE = 0i32;
pub const STGMOVE_COPY: STGMOVE = 1i32;
pub const STGMOVE_SHALLOWCOPY: STGMOVE = 2i32;
pub const STGM_CONVERT: i32 = 131072i32;
pub const STGM_CREATE: i32 = 4096i32;
pub const STGM_DELETEONRELEASE: i32 = 67108864i32;
pub const STGM_DIRECT: i32 = 0i32;
pub const STGM_DIRECT_SWMR: i32 = 4194304i32;
pub const STGM_FAILIFTHERE: i32 = 0i32;
pub const STGM_NOSCRATCH: i32 = 1048576i32;
pub const STGM_NOSNAPSHOT: i32 = 2097152i32;
pub const STGM_PRIORITY: i32 = 262144i32;
pub const STGM_READ: i32 = 0i32;
pub const STGM_READWRITE: i32 = 2i32;
pub const STGM_SHARE_DENY_NONE: i32 = 64i32;
pub const STGM_SHARE_DENY_READ: i32 = 48i32;
pub const STGM_SHARE_DENY_WRITE: i32 = 32i32;
pub const STGM_SHARE_EXCLUSIVE: i32 = 16i32;
pub const STGM_SIMPLE: i32 = 134217728i32;
pub const STGM_TRANSACTED: i32 = 65536i32;
pub const STGM_WRITE: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STGOPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STGOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[repr(C)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: ::windows_sys::core::GUID,
    pub pStream: super::IStream,
}
impl ::core::marker::Copy for VERSIONEDSTREAM {}
impl ::core::clone::Clone for VERSIONEDSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
