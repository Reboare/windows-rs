#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_UI_Controls")]
pub type CreateSecurityPage = unsafe extern "system" fn(psi: ISecurityInformation) -> super::super::super::UI::Controls::HPROPSHEETPAGE;
#[cfg(feature = "Win32_Foundation")]
pub type EditSecurity = unsafe extern "system" fn(hwndowner: super::super::super::Foundation::HWND, psi: ISecurityInformation) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type EditSecurityAdvanced = unsafe extern "system" fn(hwndowner: super::super::super::Foundation::HWND, psi: ISecurityInformation, usipage: SI_PAGE_TYPE) -> ::windows_sys::core::HRESULT;
pub const DOBJ_COND_NTACLS: i32 = 8i32;
pub const DOBJ_RES_CONT: i32 = 1i32;
pub const DOBJ_RES_ROOT: i32 = 2i32;
pub const DOBJ_RIBBON_LAUNCH: i32 = 16i32;
pub const DOBJ_VOL_NTACLS: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EFFPERM_RESULT_LIST {
    pub fEvaluated: super::super::super::Foundation::BOOLEAN,
    pub cObjectTypeListLength: u32,
    pub pObjectTypeList: *mut super::super::OBJECT_TYPE_LIST,
    pub pGrantedAccessList: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EFFPERM_RESULT_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EFFPERM_RESULT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IEffectivePermission = *mut ::core::ffi::c_void;
pub type IEffectivePermission2 = *mut ::core::ffi::c_void;
pub type ISecurityInformation = *mut ::core::ffi::c_void;
pub type ISecurityInformation2 = *mut ::core::ffi::c_void;
pub type ISecurityInformation3 = *mut ::core::ffi::c_void;
pub type ISecurityInformation4 = *mut ::core::ffi::c_void;
pub type ISecurityObjectTypeInfo = *mut ::core::ffi::c_void;
pub type SECURITY_INFO_PAGE_FLAGS = u32;
pub const SI_ADVANCED: SECURITY_INFO_PAGE_FLAGS = 16u32;
pub const SI_EDIT_AUDITS: SECURITY_INFO_PAGE_FLAGS = 2u32;
pub const SI_EDIT_PROPERTIES: SECURITY_INFO_PAGE_FLAGS = 128u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_OBJECT {
    pub pwszName: super::super::super::Foundation::PWSTR,
    pub pData: *mut ::core::ffi::c_void,
    pub cbData: u32,
    pub pData2: *mut ::core::ffi::c_void,
    pub cbData2: u32,
    pub Id: u32,
    pub fWellKnown: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SECURITY_OBJECT_ID_CENTRAL_ACCESS_RULE: u32 = 4u32;
pub const SECURITY_OBJECT_ID_CENTRAL_POLICY: u32 = 3u32;
pub const SECURITY_OBJECT_ID_OBJECT_SD: u32 = 1u32;
pub const SECURITY_OBJECT_ID_SHARE: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_INFO {
    pub pSid: super::super::super::Foundation::PSID,
    pub pwzCommonName: super::super::super::Foundation::PWSTR,
    pub pwzClass: super::super::super::Foundation::PWSTR,
    pub pwzUPN: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_INFO_LIST {
    pub cItems: u32,
    pub aSidInfo: [SID_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SI_ACCESS {
    pub pguid: *mut ::windows_sys::core::GUID,
    pub mask: u32,
    pub pszName: super::super::super::Foundation::PWSTR,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SI_ACCESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SI_ACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SI_ACCESS_CONTAINER: i32 = 262144i32;
pub const SI_ACCESS_GENERAL: i32 = 131072i32;
pub const SI_ACCESS_PROPERTY: i32 = 524288i32;
pub const SI_ACCESS_SPECIFIC: i32 = 65536i32;
pub const SI_CONTAINER: i32 = 4i32;
pub const SI_EDIT_OWNER: i32 = 1i32;
pub const SI_EDIT_PERMS: i32 = 0i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SI_INHERIT_TYPE {
    pub pguid: *mut ::windows_sys::core::GUID,
    pub dwFlags: super::super::ACE_FLAGS,
    pub pszName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SI_INHERIT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SI_INHERIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SI_NO_ACL_PROTECT: i32 = 512i32;
pub const SI_NO_TREE_APPLY: i32 = 1024i32;
pub const SI_OBJECT_GUID: i32 = 65536i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SI_OBJECT_INFO {
    pub dwFlags: SI_OBJECT_INFO_FLAGS,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub pszServerName: super::super::super::Foundation::PWSTR,
    pub pszObjectName: super::super::super::Foundation::PWSTR,
    pub pszPageTitle: super::super::super::Foundation::PWSTR,
    pub guidObjectType: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SI_OBJECT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SI_OBJECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SI_OBJECT_INFO_FLAGS = u32;
pub const SI_AUDITS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = 33554432u32;
pub const SI_DISABLE_DENY_ACE: SI_OBJECT_INFO_FLAGS = 2147483648u32;
pub const SI_EDIT_EFFECTIVE: SI_OBJECT_INFO_FLAGS = 131072u32;
pub const SI_ENABLE_CENTRAL_POLICY: SI_OBJECT_INFO_FLAGS = 1073741824u32;
pub const SI_ENABLE_EDIT_ATTRIBUTE_CONDITION: SI_OBJECT_INFO_FLAGS = 536870912u32;
pub const SI_MAY_WRITE: SI_OBJECT_INFO_FLAGS = 268435456u32;
pub const SI_NO_ADDITIONAL_PERMISSION: SI_OBJECT_INFO_FLAGS = 2097152u32;
pub const SI_OWNER_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = 67108864u32;
pub const SI_PERMS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = 16777216u32;
pub const SI_RESET_DACL: SI_OBJECT_INFO_FLAGS = 262144u32;
pub const SI_RESET_OWNER: SI_OBJECT_INFO_FLAGS = 1048576u32;
pub const SI_RESET_SACL: SI_OBJECT_INFO_FLAGS = 524288u32;
pub const SI_SCOPE_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = 134217728u32;
pub const SI_VIEW_ONLY: SI_OBJECT_INFO_FLAGS = 4194304u32;
pub const SI_OWNER_READONLY: i32 = 64i32;
pub const SI_OWNER_RECURSE: i32 = 256i32;
pub type SI_PAGE_ACTIVATED = i32;
pub const SI_SHOW_DEFAULT: SI_PAGE_ACTIVATED = 0i32;
pub const SI_SHOW_PERM_ACTIVATED: SI_PAGE_ACTIVATED = 1i32;
pub const SI_SHOW_AUDIT_ACTIVATED: SI_PAGE_ACTIVATED = 2i32;
pub const SI_SHOW_OWNER_ACTIVATED: SI_PAGE_ACTIVATED = 3i32;
pub const SI_SHOW_EFFECTIVE_ACTIVATED: SI_PAGE_ACTIVATED = 4i32;
pub const SI_SHOW_SHARE_ACTIVATED: SI_PAGE_ACTIVATED = 5i32;
pub const SI_SHOW_CENTRAL_POLICY_ACTIVATED: SI_PAGE_ACTIVATED = 6i32;
pub const SI_PAGE_TITLE: i32 = 2048i32;
pub type SI_PAGE_TYPE = i32;
pub const SI_PAGE_PERM: SI_PAGE_TYPE = 0i32;
pub const SI_PAGE_ADVPERM: SI_PAGE_TYPE = 1i32;
pub const SI_PAGE_AUDIT: SI_PAGE_TYPE = 2i32;
pub const SI_PAGE_OWNER: SI_PAGE_TYPE = 3i32;
pub const SI_PAGE_EFFECTIVE: SI_PAGE_TYPE = 4i32;
pub const SI_PAGE_TAKEOWNERSHIP: SI_PAGE_TYPE = 5i32;
pub const SI_PAGE_SHARE: SI_PAGE_TYPE = 6i32;
pub const SI_READONLY: i32 = 8i32;
pub const SI_RESET: i32 = 32i32;
pub const SI_RESET_DACL_TREE: i32 = 16384i32;
pub const SI_RESET_SACL_TREE: i32 = 32768i32;
pub const SI_SERVER_IS_DC: i32 = 4096i32;
