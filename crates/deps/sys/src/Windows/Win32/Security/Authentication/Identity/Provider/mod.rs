#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type ACCOUNT_STATE = i32;
pub const NOT_CONNECTED: ACCOUNT_STATE = 0i32;
pub const CONNECTING: ACCOUNT_STATE = 1i32;
pub const CONNECT_COMPLETED: ACCOUNT_STATE = 2i32;
pub type AsyncIAssociatedIdentityProvider = *mut ::core::ffi::c_void;
pub type AsyncIConnectedIdentityProvider = *mut ::core::ffi::c_void;
pub type AsyncIIdentityAdvise = *mut ::core::ffi::c_void;
pub type AsyncIIdentityAuthentication = *mut ::core::ffi::c_void;
pub type AsyncIIdentityProvider = *mut ::core::ffi::c_void;
pub type AsyncIIdentityStore = *mut ::core::ffi::c_void;
pub type AsyncIIdentityStoreEx = *mut ::core::ffi::c_void;
pub const CIdentityProfileHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3975528262,
    data2: 58294,
    data3: 17562,
    data4: [181, 107, 67, 245, 143, 134, 120, 20],
};
pub const CoClassIdentityStore: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 819237446,
    data2: 53783,
    data3: 18015,
    data4: [176, 11, 172, 157, 221, 101, 46, 183],
};
pub type IAssociatedIdentityProvider = *mut ::core::ffi::c_void;
pub type IConnectedIdentityProvider = *mut ::core::ffi::c_void;
pub type IDENTITY_TYPE = i32;
pub const IDENTITIES_ALL: IDENTITY_TYPE = 0i32;
pub const IDENTITIES_ME_ONLY: IDENTITY_TYPE = 1i32;
pub type IDENTITY_URL = i32;
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: IDENTITY_URL = 0i32;
pub const IDENTITY_URL_SIGN_IN_WIZARD: IDENTITY_URL = 1i32;
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: IDENTITY_URL = 2i32;
pub const IDENTITY_URL_IFEXISTS_WIZARD: IDENTITY_URL = 3i32;
pub const IDENTITY_URL_ACCOUNT_SETTINGS: IDENTITY_URL = 4i32;
pub const IDENTITY_URL_RESTORE_WIZARD: IDENTITY_URL = 5i32;
pub const IDENTITY_URL_CONNECT_WIZARD: IDENTITY_URL = 6i32;
pub type IIdentityAdvise = *mut ::core::ffi::c_void;
pub type IIdentityAuthentication = *mut ::core::ffi::c_void;
pub type IIdentityProvider = *mut ::core::ffi::c_void;
pub type IIdentityStore = *mut ::core::ffi::c_void;
pub type IIdentityStoreEx = *mut ::core::ffi::c_void;
pub type IdentityUpdateEvent = u32;
pub const IDENTITY_ASSOCIATED: IdentityUpdateEvent = 1u32;
pub const IDENTITY_DISASSOCIATED: IdentityUpdateEvent = 2u32;
pub const IDENTITY_CREATED: IdentityUpdateEvent = 4u32;
pub const IDENTITY_IMPORTED: IdentityUpdateEvent = 8u32;
pub const IDENTITY_DELETED: IdentityUpdateEvent = 16u32;
pub const IDENTITY_PROPCHANGED: IdentityUpdateEvent = 32u32;
pub const IDENTITY_CONNECTED: IdentityUpdateEvent = 64u32;
pub const IDENTITY_DISCONNECTED: IdentityUpdateEvent = 128u32;
pub const OID_OAssociatedIdentityProviderObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2563089373,
    data2: 56168,
    data3: 20250,
    data4: [141, 43, 144, 121, 205, 254, 175, 97],
};
