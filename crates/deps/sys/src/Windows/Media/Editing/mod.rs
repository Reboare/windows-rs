#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type BackgroundAudioTrack = *mut ::core::ffi::c_void;
pub type EmbeddedAudioTrack = *mut ::core::ffi::c_void;
pub type MediaClip = *mut ::core::ffi::c_void;
pub type MediaComposition = *mut ::core::ffi::c_void;
pub type MediaOverlay = *mut ::core::ffi::c_void;
pub type MediaOverlayLayer = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct MediaTrimmingPreference(pub i32);
impl MediaTrimmingPreference {
    pub const Fast: Self = Self(0i32);
    pub const Precise: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaTrimmingPreference {}
impl ::core::clone::Clone for MediaTrimmingPreference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoFramePrecision(pub i32);
impl VideoFramePrecision {
    pub const NearestFrame: Self = Self(0i32);
    pub const NearestKeyFrame: Self = Self(1i32);
}
impl ::core::marker::Copy for VideoFramePrecision {}
impl ::core::clone::Clone for VideoFramePrecision {
    fn clone(&self) -> Self {
        *self
    }
}
