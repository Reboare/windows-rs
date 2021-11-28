#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type CheckGamingPrivilegeSilently = unsafe extern "system" fn(privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CheckGamingPrivilegeSilentlyForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
pub type CheckGamingPrivilegeWithUI = unsafe extern "system" fn(privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, friendlymessage: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type CheckGamingPrivilegeWithUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, friendlymessage: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type GetExpandedResourceExclusiveCpuCount = unsafe extern "system" fn(exclusivecpucount: *mut u32) -> ::windows_sys::core::HRESULT;
pub type GetGamingDeviceModelInformation = unsafe extern "system" fn(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HasExpandedResources = unsafe extern "system" fn(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type ProcessPendingGameUI = unsafe extern "system" fn(waitforcompletion: super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
pub type ReleaseExclusiveCpuSets = unsafe extern "system" fn() -> ::windows_sys::core::HRESULT;
pub type ShowChangeFriendRelationshipUI = unsafe extern "system" fn(targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowChangeFriendRelationshipUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowCustomizeUserProfileUI = unsafe extern "system" fn(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowCustomizeUserProfileUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowFindFriendsUI = unsafe extern "system" fn(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowFindFriendsUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowGameInfoUI = unsafe extern "system" fn(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowGameInfoUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowGameInviteUI = unsafe extern "system" fn(serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowGameInviteUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowGameInviteUIWithContext = unsafe extern "system" fn(serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, customactivationcontext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowGameInviteUIWithContextForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, customactivationcontext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowPlayerPickerUI = unsafe extern "system" fn(promptdisplaytext: ::windows_sys::core::HSTRING, xuids: *const ::windows_sys::core::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_sys::core::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowPlayerPickerUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, promptdisplaytext: ::windows_sys::core::HSTRING, xuids: *const ::windows_sys::core::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_sys::core::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowProfileCardUI = unsafe extern "system" fn(targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowProfileCardUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowTitleAchievementsUI = unsafe extern "system" fn(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowTitleAchievementsUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowUserSettingsUI = unsafe extern "system" fn(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type ShowUserSettingsUIForUser = unsafe extern "system" fn(user: ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type TryCancelPendingGameUI = unsafe extern "system" fn() -> super::Foundation::BOOL;
pub type GAMESTATS_OPEN_RESULT = i32;
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = 0i32;
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = 1i32;
pub type GAMESTATS_OPEN_TYPE = i32;
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = 0i32;
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = 1i32;
pub type GAME_INSTALL_SCOPE = i32;
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = 1i32;
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = 2i32;
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = 3i32;
pub type GAMING_DEVICE_DEVICE_ID = i32;
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = 0i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = 1988865574i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = 712204761i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = 1523980231i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = 284675555i32;
#[repr(C)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
impl ::core::marker::Copy for GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::clone::Clone for GAMING_DEVICE_MODEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GAMING_DEVICE_VENDOR_ID = i32;
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = 0i32;
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = -1024700366i32;
pub const GameExplorer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2589895056, data2: 12340, data3: 19823, data4: [145, 40, 1, 243, 198, 16, 34, 188] };
pub const GameStatistics: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3687340588,
    data2: 49372,
    data3: 18785,
    data4: [182, 226, 210, 139, 98, 193, 26, 212],
};
pub type GameUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_sys::core::HRESULT, context: *const ::core::ffi::c_void)>;
pub type IGameExplorer = *mut ::core::ffi::c_void;
pub type IGameExplorer2 = *mut ::core::ffi::c_void;
pub type IGameStatistics = *mut ::core::ffi::c_void;
pub type IGameStatisticsMgr = *mut ::core::ffi::c_void;
pub type IXblIdpAuthManager = *mut ::core::ffi::c_void;
pub type IXblIdpAuthTokenResult = *mut ::core::ffi::c_void;
pub type IXblIdpAuthTokenResult2 = *mut ::core::ffi::c_void;
pub type KnownGamingPrivileges = i32;
pub const XPRIVILEGE_BROADCAST: KnownGamingPrivileges = 190i32;
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: KnownGamingPrivileges = 197i32;
pub const XPRIVILEGE_GAME_DVR: KnownGamingPrivileges = 198i32;
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: KnownGamingPrivileges = 199i32;
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: KnownGamingPrivileges = 203i32;
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: KnownGamingPrivileges = 205i32;
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: KnownGamingPrivileges = 206i32;
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: KnownGamingPrivileges = 207i32;
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: KnownGamingPrivileges = 208i32;
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: KnownGamingPrivileges = 209i32;
pub const XPRIVILEGE_SHARE_CONTENT: KnownGamingPrivileges = 211i32;
pub const XPRIVILEGE_PREMIUM_CONTENT: KnownGamingPrivileges = 214i32;
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: KnownGamingPrivileges = 219i32;
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: KnownGamingPrivileges = 220i32;
pub const XPRIVILEGE_PREMIUM_VIDEO: KnownGamingPrivileges = 224i32;
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: KnownGamingPrivileges = 235i32;
pub const XPRIVILEGE_PURCHASE_CONTENT: KnownGamingPrivileges = 245i32;
pub const XPRIVILEGE_USER_CREATED_CONTENT: KnownGamingPrivileges = 247i32;
pub const XPRIVILEGE_PROFILE_VIEWING: KnownGamingPrivileges = 249i32;
pub const XPRIVILEGE_COMMUNICATIONS: KnownGamingPrivileges = 252i32;
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: KnownGamingPrivileges = 254i32;
pub const XPRIVILEGE_ADD_FRIEND: KnownGamingPrivileges = 255i32;
pub type PlayerPickerUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_sys::core::HRESULT, context: *const ::core::ffi::c_void, selectedxuids: *const ::windows_sys::core::HSTRING, selectedxuidscount: usize)>;
pub type XBL_IDP_AUTH_TOKEN_STATUS = i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = 0i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = 1i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: XBL_IDP_AUTH_TOKEN_STATUS = 2i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: XBL_IDP_AUTH_TOKEN_STATUS = 3i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: XBL_IDP_AUTH_TOKEN_STATUS = 4i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: XBL_IDP_AUTH_TOKEN_STATUS = 5i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: XBL_IDP_AUTH_TOKEN_STATUS = 6i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: XBL_IDP_AUTH_TOKEN_STATUS = 7i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: XBL_IDP_AUTH_TOKEN_STATUS = -1i32;
pub const XblIdpAuthManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3458421579,
    data2: 22232,
    data3: 18808,
    data4: [134, 162, 126, 229, 112, 100, 4, 104],
};
pub const XblIdpAuthTokenResult: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2672374849,
    data2: 29770,
    data3: 16652,
    data4: [174, 43, 154, 34, 247, 199, 115, 31],
};
