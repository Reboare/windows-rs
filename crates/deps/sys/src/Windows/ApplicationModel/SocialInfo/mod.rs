#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_SocialInfo_Provider")]
pub mod Provider;
pub type SocialFeedChildItem = *mut ::core::ffi::c_void;
pub type SocialFeedContent = *mut ::core::ffi::c_void;
pub type SocialFeedItem = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct SocialFeedItemStyle(pub i32);
impl SocialFeedItemStyle {
    pub const Default: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
}
impl ::core::marker::Copy for SocialFeedItemStyle {}
impl ::core::clone::Clone for SocialFeedItemStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialFeedKind(pub i32);
impl SocialFeedKind {
    pub const HomeFeed: Self = Self(0i32);
    pub const ContactFeed: Self = Self(1i32);
    pub const Dashboard: Self = Self(2i32);
}
impl ::core::marker::Copy for SocialFeedKind {}
impl ::core::clone::Clone for SocialFeedKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SocialFeedSharedItem = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct SocialFeedUpdateMode(pub i32);
impl SocialFeedUpdateMode {
    pub const Append: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
impl ::core::marker::Copy for SocialFeedUpdateMode {}
impl ::core::clone::Clone for SocialFeedUpdateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialItemBadgeStyle(pub i32);
impl SocialItemBadgeStyle {
    pub const Hidden: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const VisibleWithCount: Self = Self(2i32);
}
impl ::core::marker::Copy for SocialItemBadgeStyle {}
impl ::core::clone::Clone for SocialItemBadgeStyle {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SocialItemThumbnail = *mut ::core::ffi::c_void;
pub type SocialUserInfo = *mut ::core::ffi::c_void;
