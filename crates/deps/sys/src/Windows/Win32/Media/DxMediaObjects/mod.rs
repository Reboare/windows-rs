#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type DMOEnum = unsafe extern "system" fn(guidcategory: *const ::windows_sys::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE, ppenum: *mut IEnumDMO) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DMOGetName = unsafe extern "system" fn(clsiddmo: *const ::windows_sys::core::GUID, szname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type DMOGetTypes = unsafe extern "system" fn(clsiddmo: *const ::windows_sys::core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DMORegister = unsafe extern "system" fn(szname: super::super::Foundation::PWSTR, clsiddmo: *const ::windows_sys::core::GUID, guidcategory: *const ::windows_sys::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows_sys::core::HRESULT;
pub type DMOUnregister = unsafe extern "system" fn(clsiddmo: *const ::windows_sys::core::GUID, guidcategory: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type MoCopyMediaType = unsafe extern "system" fn(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type MoCreateMediaType = unsafe extern "system" fn(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type MoDeleteMediaType = unsafe extern "system" fn(pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type MoDuplicateMediaType = unsafe extern "system" fn(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type MoFreeMediaType = unsafe extern "system" fn(pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type MoInitMediaType = unsafe extern "system" fn(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows_sys::core::HRESULT;
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3214294400, data2: 50521, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const DMOCATEGORY_AGC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3901528992, data2: 50519, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133857978, data2: 15881, data3: 18720, data4: [170, 95, 33, 152, 17, 20, 143, 9] };
pub const DMOCATEGORY_AUDIO_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1475533707, data2: 59067, data3: 17683, data4: [157, 67, 220, 210, 166, 89, 49, 37] };
pub const DMOCATEGORY_AUDIO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4083166015,
    data2: 1426,
    data3: 18655,
    data4: [164, 205, 103, 71, 33, 231, 235, 235],
};
pub const DMOCATEGORY_AUDIO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 869902177, data2: 37064, data3: 4560, data4: [189, 67, 0, 160, 201, 17, 206, 134] };
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3766456383,
    data2: 25341,
    data3: 20064,
    data4: [140, 221, 222, 167, 35, 102, 101, 181],
};
pub const DMOCATEGORY_VIDEO_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1248441410,
    data2: 10430,
    data3: 18833,
    data4: [150, 156, 181, 0, 173, 245, 216, 168],
};
pub const DMOCATEGORY_VIDEO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3650154004,
    data2: 30572,
    data3: 18211,
    data4: [190, 70, 61, 162, 245, 111, 16, 185],
};
pub const DMOCATEGORY_VIDEO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 869902176, data2: 37064, data3: 4560, data4: [189, 67, 0, 160, 201, 17, 206, 134] };
pub type DMO_ENUM_FLAGS = i32;
pub const DMO_ENUMF_INCLUDE_KEYED: DMO_ENUM_FLAGS = 1i32;
pub const DMO_E_INVALIDSTREAMINDEX: ::windows_sys::core::HRESULT = -2147220991i32;
pub const DMO_E_INVALIDTYPE: ::windows_sys::core::HRESULT = -2147220990i32;
pub const DMO_E_NOTACCEPTING: ::windows_sys::core::HRESULT = -2147220988i32;
pub const DMO_E_NO_MORE_ITEMS: ::windows_sys::core::HRESULT = -2147220986i32;
pub const DMO_E_TYPE_NOT_ACCEPTED: ::windows_sys::core::HRESULT = -2147220987i32;
pub const DMO_E_TYPE_NOT_SET: ::windows_sys::core::HRESULT = -2147220989i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DMO_MEDIA_TYPE {
    pub majortype: ::windows_sys::core::GUID,
    pub subtype: ::windows_sys::core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows_sys::core::GUID,
    pub pUnk: ::windows_sys::core::IUnknown,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DMO_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DMO_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DMO_OUTPUT_DATA_BUFFER {
    pub pBuffer: IMediaBuffer,
    pub dwStatus: u32,
    pub rtTimestamp: i64,
    pub rtTimelength: i64,
}
impl ::core::marker::Copy for DMO_OUTPUT_DATA_BUFFER {}
impl ::core::clone::Clone for DMO_OUTPUT_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: ::windows_sys::core::GUID,
    pub subtype: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DMO_PARTIAL_MEDIATYPE {}
impl ::core::clone::Clone for DMO_PARTIAL_MEDIATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DMO_REGISTER_FLAGS = i32;
pub const DMO_REGISTERF_IS_KEYED: DMO_REGISTER_FLAGS = 1i32;
pub type IDMOQualityControl = *mut ::core::ffi::c_void;
pub type IDMOVideoOutputOptimizations = *mut ::core::ffi::c_void;
pub type IEnumDMO = *mut ::core::ffi::c_void;
pub type IMediaBuffer = *mut ::core::ffi::c_void;
pub type IMediaObject = *mut ::core::ffi::c_void;
pub type IMediaObjectInPlace = *mut ::core::ffi::c_void;
pub type _DMO_INPLACE_PROCESS_FLAGS = i32;
pub const DMO_INPLACE_NORMAL: _DMO_INPLACE_PROCESS_FLAGS = 0i32;
pub const DMO_INPLACE_ZERO: _DMO_INPLACE_PROCESS_FLAGS = 1i32;
pub type _DMO_INPUT_DATA_BUFFER_FLAGS = i32;
pub const DMO_INPUT_DATA_BUFFERF_SYNCPOINT: _DMO_INPUT_DATA_BUFFER_FLAGS = 1i32;
pub const DMO_INPUT_DATA_BUFFERF_TIME: _DMO_INPUT_DATA_BUFFER_FLAGS = 2i32;
pub const DMO_INPUT_DATA_BUFFERF_TIMELENGTH: _DMO_INPUT_DATA_BUFFER_FLAGS = 4i32;
pub const DMO_INPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_INPUT_DATA_BUFFER_FLAGS = 8i32;
pub type _DMO_INPUT_STATUS_FLAGS = i32;
pub const DMO_INPUT_STATUSF_ACCEPT_DATA: _DMO_INPUT_STATUS_FLAGS = 1i32;
pub type _DMO_INPUT_STREAM_INFO_FLAGS = i32;
pub const DMO_INPUT_STREAMF_WHOLE_SAMPLES: _DMO_INPUT_STREAM_INFO_FLAGS = 1i32;
pub const DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_INPUT_STREAM_INFO_FLAGS = 2i32;
pub const DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_INPUT_STREAM_INFO_FLAGS = 4i32;
pub const DMO_INPUT_STREAMF_HOLDS_BUFFERS: _DMO_INPUT_STREAM_INFO_FLAGS = 8i32;
pub type _DMO_OUTPUT_DATA_BUFFER_FLAGS = i32;
pub const DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 1i32;
pub const DMO_OUTPUT_DATA_BUFFERF_TIME: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 2i32;
pub const DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 4i32;
pub const DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 8i32;
pub const DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 16777216i32;
pub type _DMO_OUTPUT_STREAM_INFO_FLAGS = i32;
pub const DMO_OUTPUT_STREAMF_WHOLE_SAMPLES: _DMO_OUTPUT_STREAM_INFO_FLAGS = 1i32;
pub const DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_OUTPUT_STREAM_INFO_FLAGS = 2i32;
pub const DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_OUTPUT_STREAM_INFO_FLAGS = 4i32;
pub const DMO_OUTPUT_STREAMF_DISCARDABLE: _DMO_OUTPUT_STREAM_INFO_FLAGS = 8i32;
pub const DMO_OUTPUT_STREAMF_OPTIONAL: _DMO_OUTPUT_STREAM_INFO_FLAGS = 16i32;
pub type _DMO_PROCESS_OUTPUT_FLAGS = i32;
pub const DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _DMO_PROCESS_OUTPUT_FLAGS = 1i32;
pub type _DMO_QUALITY_STATUS_FLAGS = i32;
pub const DMO_QUALITY_STATUS_ENABLED: _DMO_QUALITY_STATUS_FLAGS = 1i32;
pub type _DMO_SET_TYPE_FLAGS = i32;
pub const DMO_SET_TYPEF_TEST_ONLY: _DMO_SET_TYPE_FLAGS = 1i32;
pub const DMO_SET_TYPEF_CLEAR: _DMO_SET_TYPE_FLAGS = 2i32;
pub type _DMO_VIDEO_OUTPUT_STREAM_FLAGS = i32;
pub const DMO_VOSF_NEEDS_PREVIOUS_SAMPLE: _DMO_VIDEO_OUTPUT_STREAM_FLAGS = 1i32;
