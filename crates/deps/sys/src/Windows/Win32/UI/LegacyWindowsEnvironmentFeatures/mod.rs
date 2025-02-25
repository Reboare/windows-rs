#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const EVCCBF_LASTNOTIFICATION: u32 = 1u32;
pub const EVCF_DONTSHOWIFZERO: u32 = 16u32;
pub const EVCF_ENABLEBYDEFAULT: u32 = 2u32;
pub const EVCF_ENABLEBYDEFAULT_AUTO: u32 = 8u32;
pub const EVCF_HASSETTINGS: u32 = 1u32;
pub const EVCF_OUTOFDISKSPACE: u32 = 64u32;
pub const EVCF_REMOVEFROMLIST: u32 = 4u32;
pub const EVCF_SETTINGSMODE: u32 = 32u32;
pub const EVCF_SYSTEMAUTORUN: u32 = 256u32;
pub const EVCF_USERCONSENTOBTAINED: u32 = 128u32;
pub type IADesktopP2 = *mut ::core::ffi::c_void;
pub type IActiveDesktopP = *mut ::core::ffi::c_void;
pub type IBriefcaseInitiator = *mut ::core::ffi::c_void;
pub type IEmptyVolumeCache = *mut ::core::ffi::c_void;
pub type IEmptyVolumeCache2 = *mut ::core::ffi::c_void;
pub type IEmptyVolumeCacheCallBack = *mut ::core::ffi::c_void;
pub type IReconcilableObject = *mut ::core::ffi::c_void;
pub type IReconcileInitiator = *mut ::core::ffi::c_void;
pub const REC_E_ABORTED: ::windows_sys::core::HRESULT = -2147217408i32;
pub const REC_E_INEEDTODOTHEUPDATES: ::windows_sys::core::HRESULT = -2147217404i32;
pub const REC_E_NOCALLBACK: ::windows_sys::core::HRESULT = -2147217407i32;
pub const REC_E_NORESIDUES: ::windows_sys::core::HRESULT = -2147217406i32;
pub const REC_E_TOODIFFERENT: ::windows_sys::core::HRESULT = -2147217405i32;
pub const REC_S_IDIDTHEUPDATES: ::windows_sys::core::HRESULT = 266240i32;
pub const REC_S_NOTCOMPLETE: ::windows_sys::core::HRESULT = 266241i32;
pub const REC_S_NOTCOMPLETEBUTPROPAGATE: ::windows_sys::core::HRESULT = 266242i32;
pub const STATEBITS_FLAT: u32 = 1u32;
pub type _reconcilef = i32;
pub const RECONCILEF_MAYBOTHERUSER: _reconcilef = 1i32;
pub const RECONCILEF_FEEDBACKWINDOWVALID: _reconcilef = 2i32;
pub const RECONCILEF_NORESIDUESOK: _reconcilef = 4i32;
pub const RECONCILEF_OMITSELFRESIDUE: _reconcilef = 8i32;
pub const RECONCILEF_RESUMERECONCILIATION: _reconcilef = 16i32;
pub const RECONCILEF_YOUMAYDOTHEUPDATES: _reconcilef = 32i32;
pub const RECONCILEF_ONLYYOUWERECHANGED: _reconcilef = 64i32;
pub const ALL_RECONCILE_FLAGS: _reconcilef = 127i32;
