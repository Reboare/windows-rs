#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type IInkCommitRequestHandler = *mut ::core::ffi::c_void;
pub type IInkD2DRenderer = *mut ::core::ffi::c_void;
pub type IInkD2DRenderer2 = *mut ::core::ffi::c_void;
pub type IInkDesktopHost = *mut ::core::ffi::c_void;
pub type IInkHostWorkItem = *mut ::core::ffi::c_void;
pub type IInkPresenterDesktop = *mut ::core::ffi::c_void;
pub type INK_HIGH_CONTRAST_ADJUSTMENT = i32;
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = 0i32;
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = 1i32;
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = 2i32;
pub const InkD2DRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1078257164, data2: 31489, data3: 18033, data4: [169, 124, 4, 224, 33, 10, 7, 165] };
pub const InkDesktopHost: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 103122086, data2: 63536, data3: 19420, data4: [164, 210, 10, 16, 171, 6, 43, 29] };
