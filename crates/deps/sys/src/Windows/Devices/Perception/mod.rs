#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_Perception_Provider")]
pub mod Provider;
pub type PerceptionColorFrame = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameArrivedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameReader = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameSource = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameSourceAddedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameSourceRemovedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameSourceWatcher = *mut ::core::ffi::c_void;
pub type PerceptionControlSession = *mut ::core::ffi::c_void;
pub type PerceptionDepthCorrelatedCameraIntrinsics = *mut ::core::ffi::c_void;
pub type PerceptionDepthCorrelatedCoordinateMapper = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrame = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameArrivedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameReader = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameSource = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameSourceAddedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameSourceRemovedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameSourceWatcher = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct PerceptionFrameSourceAccessStatus(pub i32);
impl PerceptionFrameSourceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for PerceptionFrameSourceAccessStatus {}
impl ::core::clone::Clone for PerceptionFrameSourceAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PerceptionFrameSourcePropertiesChangedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionFrameSourcePropertyChangeResult = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertyChangeStatus(pub i32);
impl PerceptionFrameSourcePropertyChangeStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Accepted: Self = Self(1i32);
    pub const LostControl: Self = Self(2i32);
    pub const PropertyNotSupported: Self = Self(3i32);
    pub const PropertyReadOnly: Self = Self(4i32);
    pub const ValueOutOfRange: Self = Self(5i32);
}
impl ::core::marker::Copy for PerceptionFrameSourcePropertyChangeStatus {}
impl ::core::clone::Clone for PerceptionFrameSourcePropertyChangeStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PerceptionInfraredFrame = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameArrivedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameReader = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameSource = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameSourceAddedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameSourceRemovedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameSourceWatcher = *mut ::core::ffi::c_void;
pub type PerceptionVideoProfile = *mut ::core::ffi::c_void;
