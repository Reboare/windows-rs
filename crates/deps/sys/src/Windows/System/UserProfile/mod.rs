#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
pub struct AccountPictureKind(pub i32);
impl AccountPictureKind {
    pub const SmallImage: Self = Self(0i32);
    pub const LargeImage: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for AccountPictureKind {}
impl ::core::clone::Clone for AccountPictureKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AdvertisingManagerForUser = *mut ::core::ffi::c_void;
pub type AssignedAccessSettings = *mut ::core::ffi::c_void;
pub type DiagnosticsSettings = *mut ::core::ffi::c_void;
pub type FirstSignInSettings = *mut ::core::ffi::c_void;
pub type GlobalizationPreferencesForUser = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct SetAccountPictureResult(pub i32);
impl SetAccountPictureResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const LargeOrDynamicError: Self = Self(2i32);
    pub const VideoFrameSizeError: Self = Self(3i32);
    pub const FileSizeError: Self = Self(4i32);
    pub const Failure: Self = Self(5i32);
}
impl ::core::marker::Copy for SetAccountPictureResult {}
impl ::core::clone::Clone for SetAccountPictureResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetImageFeedResult(pub i32);
impl SetImageFeedResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const UserCanceled: Self = Self(2i32);
}
impl ::core::marker::Copy for SetImageFeedResult {}
impl ::core::clone::Clone for SetImageFeedResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserProfilePersonalizationSettings = *mut ::core::ffi::c_void;
