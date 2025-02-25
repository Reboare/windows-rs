#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
pub struct ACT_AUTHORIZATION_STATE {
    pub ulState: u32,
}
impl ::core::marker::Copy for ACT_AUTHORIZATION_STATE {}
impl ::core::clone::Clone for ACT_AUTHORIZATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ACT_AUTHORIZATION_STATE_VALUE = i32;
pub const ACT_UNAUTHORIZED: ACT_AUTHORIZATION_STATE_VALUE = 0i32;
pub const ACT_AUTHORIZED: ACT_AUTHORIZATION_STATE_VALUE = 1i32;
pub const ACT_AUTHORIZE_ON_RESUME: u32 = 1u32;
pub const ACT_AUTHORIZE_ON_SESSION_UNLOCK: u32 = 2u32;
pub const ACT_UNAUTHORIZE_ON_SESSION_LOCK: u32 = 2u32;
pub const ACT_UNAUTHORIZE_ON_SUSPEND: u32 = 1u32;
pub const APPUSERMODEL_STARTPINOPTION_DEFAULT: u32 = 0u32;
pub const APPUSERMODEL_STARTPINOPTION_NOPINONINSTALL: u32 = 1u32;
pub const APPUSERMODEL_STARTPINOPTION_USERPINNED: u32 = 2u32;
pub const AUDIO_CHANNELCOUNT_MONO: u32 = 1u32;
pub const AUDIO_CHANNELCOUNT_STEREO: u32 = 2u32;
pub const BLUETOOTH_ADDRESS_TYPE_PUBLIC: u32 = 0u32;
pub const BLUETOOTH_ADDRESS_TYPE_RANDOM: u32 = 1u32;
pub const BLUETOOTH_CACHED_MODE_UNCACHED: u32 = 1u32;
pub const BLUETOOTH_CACHE_MODE_CACHED: u32 = 0u32;
pub const CERT_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY: u32 = 2u32;
pub const CERT_CAPABILITY_CERTIFICATE_SUPPORT: u32 = 4u32;
pub const CERT_CAPABILITY_HASH_ALG: u32 = 1u32;
pub const CERT_CAPABILITY_OPTIONAL_FEATURES: u32 = 5u32;
pub const CERT_CAPABILITY_SIGNATURE_ALG: u32 = 3u32;
pub const CERT_MAX_CAPABILITY: u32 = 255u32;
pub const CERT_TYPE_ASCh: u32 = 3u32;
pub const CERT_TYPE_ASCm: u32 = 1u32;
pub const CERT_TYPE_EMPTY: u32 = 0u32;
pub const CERT_TYPE_HCh: u32 = 4u32;
pub const CERT_TYPE_PCp: u32 = 2u32;
pub const CERT_TYPE_SIGNER: u32 = 6u32;
pub const CERT_VALIDATION_POLICY_BASIC: u32 = 2u32;
pub const CERT_VALIDATION_POLICY_EXTENDED: u32 = 3u32;
pub const CERT_VALIDATION_POLICY_NONE: u32 = 1u32;
pub const CERT_VALIDATION_POLICY_RESERVED: u32 = 0u32;
pub const CREATOROPENWITHUIOPTION_HIDDEN: u32 = 0u32;
pub const CREATOROPENWITHUIOPTION_VISIBLE: u32 = 1u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATED: u32 = 3u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATION_DENIED: u32 = 2147483649u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_DEVICE_ERROR: u32 = 2147483650u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_NOT_AUTHENTICATED: u32 = 2u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_NO_AUTHENTICATION_REQUIRED: u32 = 1u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_UNKNOWN: u32 = 0u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 4002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_CERTIFICATE_EXTENSION_PARSING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 4005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_HASH_ALGS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 4001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_RENDER_USER_DATA_UNUSABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 4004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_SIGNING_ALGS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 4003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_ADMIN_CERTIFICATE_AUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_CREATE_CERTIFICATE_REQUEST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 108u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_DEVICE_CERTIFICATE_AUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_ACT_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 113u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 106u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 112u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 111u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 114u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_HOST_CERTIFICATE_AUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_INITIALIZE_TO_MANUFACTURER_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_SET_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 107u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_UNAUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 110u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_AUTHORIZE_ACT_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 203u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_CHANGE_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 209u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_CONFIG_ADMINISTRATOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 206u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_CREATE_USER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 207u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_DELETE_USER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 208u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_INITIALIZE_USER_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 210u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_QUERY_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 205u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_START_INITIALIZE_TO_MANUFACTURER_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 211u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_UNAUTHORIZE_ACT_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 204u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_SILO_ENUMERATE_SILOS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_SILO_GET_AUTHENTICATION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_SILO_IS_AUTHENTICATION_SILO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 6u32,
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    pub CurrentAdminFailures: u8,
    pub CurrentUserFailures: u8,
    pub TotalUserAuthenticationCount: u32,
    pub TotalAdminAuthenticationCount: u32,
    pub FipsCompliant: super::super::Foundation::BOOL,
    pub SecurityIDAvailable: super::super::Foundation::BOOL,
    pub InitializeInProgress: super::super::Foundation::BOOL,
    pub ITMSArmed: super::super::Foundation::BOOL,
    pub ITMSArmable: super::super::Foundation::BOOL,
    pub UserCreated: super::super::Foundation::BOOL,
    pub ResetOnPORDefault: super::super::Foundation::BOOL,
    pub ResetOnPORCurrent: super::super::Foundation::BOOL,
    pub MaxAdminFailures: u8,
    pub MaxUserFailures: u8,
    pub TimeToCompleteInitialization: u32,
    pub TimeRemainingToCompleteInitialization: u32,
    pub MinTimeToAuthenticate: u32,
    pub MaxAdminPasswordSize: u8,
    pub MinAdminPasswordSize: u8,
    pub MaxAdminHintSize: u8,
    pub MaxUserPasswordSize: u8,
    pub MinUserPasswordSize: u8,
    pub MaxUserHintSize: u8,
    pub MaxUserNameSize: u8,
    pub MaxSiloNameSize: u8,
    pub MaxChallengeSize: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_ADMIN_HINT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_AUTHENTICATION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_ACT_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_CAPABILITY_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_REQUEST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_IS_AUTHENTICATION_SILO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_MAX_AUTH_FAILURES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_MAX_CERTIFICATE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD_INDICATOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_OF_TYPE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_OLD_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_PASSWORD_INDICATOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_PASSWORD_SILO_INFO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_QUERY_SILO_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2017u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_QUERY_SILO_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_SECURITY_IDENTIFIER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_SIGNER_CERTIFICATE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_SILO_FRIENDLYNAME_SPECIFIED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_SILO_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_STORED_CERTIFICATE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_TEMPORARY_UNAUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_USER_HINT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_USER_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 2010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_VALIDATION_POLICY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2435088742,
        data2: 47154,
        data3: 19156,
        data4: [186, 164, 124, 160, 182, 178, 121, 140],
    },
    pid: 3005u32,
};
pub const ES_AUTHN_ERROR_END: u32 = 1279u32;
pub const ES_AUTHN_ERROR_START: u32 = 1024u32;
pub const ES_GENERAL_ERROR_END: u32 = 1023u32;
pub const ES_GENERAL_ERROR_START: u32 = 512u32;
pub const ES_PW_SILO_ERROR_END: u32 = 4607u32;
pub const ES_PW_SILO_ERROR_START: u32 = 4352u32;
pub const ES_RESERVED_COM_ERROR_END: u32 = 511u32;
pub const ES_RESERVED_COM_ERROR_START: u32 = 0u32;
pub const ES_RESERVED_SILO_ERROR_END: u32 = 4095u32;
pub const ES_RESERVED_SILO_ERROR_START: u32 = 1280u32;
pub const ES_RESERVED_SILO_SPECIFIC_ERROR_END: u32 = 49151u32;
pub const ES_RESERVED_SILO_SPECIFIC_ERROR_START: u32 = 4608u32;
pub const ES_VENDOR_ERROR_END: u32 = 65535u32;
pub const ES_VENDOR_ERROR_START: u32 = 49152u32;
pub const EnhancedStorageACT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2936498709,
    data2: 11982,
    data3: 19156,
    data4: [187, 33, 41, 240, 64, 225, 118, 216],
};
pub const EnhancedStorageSilo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3408208396, data2: 30407, data3: 20462, data4: [132, 43, 243, 56, 60, 208, 34, 188] };
pub const EnhancedStorageSiloAction: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2288855517,
    data2: 46342,
    data3: 18027,
    data4: [159, 191, 180, 79, 243, 131, 251, 63],
};
pub const EnumEnhancedStorageACT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4270068883,
    data2: 33628,
    data3: 20387,
    data4: [182, 204, 180, 178, 212, 113, 152, 72],
};
pub const FACILITY_ENHANCED_STORAGE: u32 = 4u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_COMPLETE: u32 = 2u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_COMPLETE_PINNED: u32 = 3u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_EXCLUDED: u32 = 4u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_FOLDER_EMPTY: u32 = 5u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_NOTAVAILABLEOFFLINE: u32 = 0u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_PARTIAL: u32 = 1u32;
pub const FLAGSTATUS_COMPLETED: i32 = 1i32;
pub const FLAGSTATUS_FOLLOWUP: i32 = 2i32;
pub const FLAGSTATUS_NOTFLAGGED: i32 = 0i32;
pub const GUID_DEVINTERFACE_ENHANCED_STORAGE_SILO: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 949483172,
    data2: 64821,
    data3: 19400,
    data4: [160, 183, 93, 187, 163, 106, 218, 250],
};
pub type IEnhancedStorageACT = *mut ::core::ffi::c_void;
pub type IEnhancedStorageACT2 = *mut ::core::ffi::c_void;
pub type IEnhancedStorageACT3 = *mut ::core::ffi::c_void;
pub type IEnhancedStorageSilo = *mut ::core::ffi::c_void;
pub type IEnhancedStorageSiloAction = *mut ::core::ffi::c_void;
pub type IEnumEnhancedStorageACT = *mut ::core::ffi::c_void;
pub const IMPORTANCE_HIGH_MAX: i32 = 5i32;
pub const IMPORTANCE_HIGH_MIN: i32 = 5i32;
pub const IMPORTANCE_HIGH_SET: i32 = 5i32;
pub const IMPORTANCE_LOW_MAX: i32 = 1i32;
pub const IMPORTANCE_LOW_MIN: i32 = 0i32;
pub const IMPORTANCE_LOW_SET: i32 = 1i32;
pub const IMPORTANCE_NORMAL_MAX: i32 = 4i32;
pub const IMPORTANCE_NORMAL_MIN: i32 = 2i32;
pub const IMPORTANCE_NORMAL_SET: i32 = 3i32;
pub const ISDEFAULTSAVE_BOTH: u32 = 3u32;
pub const ISDEFAULTSAVE_NONE: u32 = 0u32;
pub const ISDEFAULTSAVE_NONOWNER: u32 = 2u32;
pub const ISDEFAULTSAVE_OWNER: u32 = 1u32;
pub const LINK_STATUS_BROKEN: i32 = 2i32;
pub const LINK_STATUS_RESOLVED: i32 = 1i32;
pub const OFFLINEAVAILABILITY_ALWAYS_AVAILABLE: u32 = 2u32;
pub const OFFLINEAVAILABILITY_AVAILABLE: u32 = 1u32;
pub const OFFLINEAVAILABILITY_NOT_AVAILABLE: u32 = 0u32;
pub const OFFLINESTATUS_OFFLINE: u32 = 1u32;
pub const OFFLINESTATUS_OFFLINE_ERROR: u32 = 4u32;
pub const OFFLINESTATUS_OFFLINE_FORCED: u32 = 2u32;
pub const OFFLINESTATUS_OFFLINE_ITEM_VERSION_CONFLICT: u32 = 5u32;
pub const OFFLINESTATUS_OFFLINE_SLOW: u32 = 3u32;
pub const OFFLINESTATUS_OFFLINE_SUSPENDED: u32 = 6u32;
pub const OFFLINESTATUS_ONLINE: u32 = 0u32;
pub const PHOTO_CONTRAST_HARD: u32 = 2u32;
pub const PHOTO_CONTRAST_NORMAL: u32 = 0u32;
pub const PHOTO_CONTRAST_SOFT: u32 = 1u32;
pub const PHOTO_EXPOSUREPROGRAM_ACTION: u32 = 6u32;
pub const PHOTO_EXPOSUREPROGRAM_APERTURE: u32 = 3u32;
pub const PHOTO_EXPOSUREPROGRAM_CREATIVE: u32 = 5u32;
pub const PHOTO_EXPOSUREPROGRAM_LANDSCAPE: u32 = 8u32;
pub const PHOTO_EXPOSUREPROGRAM_MANUAL: u32 = 1u32;
pub const PHOTO_EXPOSUREPROGRAM_NORMAL: u32 = 2u32;
pub const PHOTO_EXPOSUREPROGRAM_PORTRAIT: u32 = 7u32;
pub const PHOTO_EXPOSUREPROGRAM_SHUTTER: u32 = 4u32;
pub const PHOTO_EXPOSUREPROGRAM_UNKNOWN: u32 = 0u32;
pub const PHOTO_FLASH_FLASH: u32 = 1u32;
pub const PHOTO_FLASH_FLASH_AUTO: u32 = 25u32;
pub const PHOTO_FLASH_FLASH_AUTO_NORETURNLIGHT: u32 = 29u32;
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE: u32 = 89u32;
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE_NORETURNLIGHT: u32 = 93u32;
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE_RETURNLIGHT: u32 = 95u32;
pub const PHOTO_FLASH_FLASH_AUTO_RETURNLIGHT: u32 = 31u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY: u32 = 9u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_NORETURNLIGHT: u32 = 13u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE: u32 = 73u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_NORETURNLIGHT: u32 = 77u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_RETURNLIGHT: u32 = 79u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_RETURNLIGHT: u32 = 15u32;
pub const PHOTO_FLASH_FLASH_REDEYE: u32 = 65u32;
pub const PHOTO_FLASH_FLASH_REDEYE_NORETURNLIGHT: u32 = 69u32;
pub const PHOTO_FLASH_FLASH_REDEYE_RETURNLIGHT: u32 = 71u32;
pub const PHOTO_FLASH_NOFUNCTION: u32 = 32u32;
pub const PHOTO_FLASH_NONE: u32 = 0u32;
pub const PHOTO_FLASH_NONE_AUTO: u32 = 24u32;
pub const PHOTO_FLASH_NONE_COMPULSORY: u32 = 16u32;
pub const PHOTO_FLASH_WITHOUTSTROBE: u32 = 5u32;
pub const PHOTO_FLASH_WITHSTROBE: u32 = 7u32;
pub const PHOTO_GAINCONTROL_HIGHGAINDOWN: f64 = 4f64;
pub const PHOTO_GAINCONTROL_HIGHGAINUP: f64 = 2f64;
pub const PHOTO_GAINCONTROL_LOWGAINDOWN: f64 = 3f64;
pub const PHOTO_GAINCONTROL_LOWGAINUP: f64 = 1f64;
pub const PHOTO_GAINCONTROL_NONE: f64 = 0f64;
pub const PHOTO_LIGHTSOURCE_D55: u32 = 20u32;
pub const PHOTO_LIGHTSOURCE_D65: u32 = 21u32;
pub const PHOTO_LIGHTSOURCE_D75: u32 = 22u32;
pub const PHOTO_LIGHTSOURCE_DAYLIGHT: u32 = 1u32;
pub const PHOTO_LIGHTSOURCE_FLUORESCENT: u32 = 2u32;
pub const PHOTO_LIGHTSOURCE_STANDARD_A: u32 = 17u32;
pub const PHOTO_LIGHTSOURCE_STANDARD_B: u32 = 18u32;
pub const PHOTO_LIGHTSOURCE_STANDARD_C: u32 = 19u32;
pub const PHOTO_LIGHTSOURCE_TUNGSTEN: u32 = 3u32;
pub const PHOTO_LIGHTSOURCE_UNKNOWN: u32 = 0u32;
pub const PHOTO_PROGRAMMODE_ACTION: u32 = 6u32;
pub const PHOTO_PROGRAMMODE_APERTURE: u32 = 3u32;
pub const PHOTO_PROGRAMMODE_CREATIVE: u32 = 5u32;
pub const PHOTO_PROGRAMMODE_LANDSCAPE: u32 = 8u32;
pub const PHOTO_PROGRAMMODE_MANUAL: u32 = 1u32;
pub const PHOTO_PROGRAMMODE_NORMAL: u32 = 2u32;
pub const PHOTO_PROGRAMMODE_NOTDEFINED: u32 = 0u32;
pub const PHOTO_PROGRAMMODE_PORTRAIT: u32 = 7u32;
pub const PHOTO_PROGRAMMODE_SHUTTER: u32 = 4u32;
pub const PHOTO_SATURATION_HIGH: u32 = 2u32;
pub const PHOTO_SATURATION_LOW: u32 = 1u32;
pub const PHOTO_SATURATION_NORMAL: u32 = 0u32;
pub const PHOTO_SHARPNESS_HARD: u32 = 2u32;
pub const PHOTO_SHARPNESS_NORMAL: u32 = 0u32;
pub const PHOTO_SHARPNESS_SOFT: u32 = 1u32;
pub const PHOTO_WHITEBALANCE_AUTO: u32 = 0u32;
pub const PHOTO_WHITEBALANCE_MANUAL: u32 = 1u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AcquisitionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1705609333,
        data2: 15488,
        data3: 16555,
        data4: [171, 188, 239, 218, 247, 125, 190, 226],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3229303193,
        data2: 57823,
        data3: 17555,
        data4: [177, 225, 222, 89, 70, 251, 88, 248],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_CountryCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3229303193,
        data2: 57823,
        data3: 17555,
        data4: [177, 225, 222, 89, 70, 251, 88, 248],
    },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3229303193,
        data2: 57823,
        data3: 17555,
        data4: [177, 225, 222, 89, 70, 251, 88, 248],
    },
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_RegionCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3229303193,
        data2: 57823,
        data3: 17555,
        data4: [177, 225, 222, 89, 70, 251, 88, 248],
    },
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_Town: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3229303193,
        data2: 57823,
        data3: 17555,
        data4: [177, 225, 222, 89, 70, 251, 88, 248],
    },
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_ExcludeFromShowInNewInstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_IsDestListSeparator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_IsDualMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_PreventPinning: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_RelaunchCommand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_RelaunchDisplayNameResource: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_RelaunchIconResource: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_SettingsCommand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_StartPinOption: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_ToastActivatorCLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_UninstallCommand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_VisualElementsManifestHintPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2672568405,
        data2: 40825,
        data3: 19257,
        data4: [168, 208, 225, 212, 45, 225, 213, 243],
    },
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppZoneIdentifier: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1345126059,
        data2: 18411,
        data3: 17820,
        data4: [185, 96, 230, 216, 114, 143, 119, 1],
    },
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ApplicationDefinedProperties: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3451896167, data2: 13182, data3: 16856, data4: [175, 124, 140, 9, 32, 84, 41, 199] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ApplicationName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_ChannelCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179216, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_Compression: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179216, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_EncodingBitrate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179216, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_Format: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179216, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_IsVariableBitRate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867291630,
        data2: 35863,
        data3: 19810,
        data4: [130, 60, 142, 156, 252, 189, 29, 92],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_PeakValue: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 628745680, data2: 4374, data3: 16516, data4: [189, 154, 155, 79, 124, 180, 223, 94] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_SampleRate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179216, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_SampleSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179216, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_StreamName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179216, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_StreamNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179216, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Author: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CachedFileUpdaterContentIdForConflictResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 114u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CachedFileUpdaterContentIdForStream: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 113u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_Duration: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 691839834, data2: 2474, data3: 19922, data4: [177, 128, 31, 226, 69, 114, 138, 82] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_IsOnline: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3220083017, data2: 58338, data3: 18855, data4: [168, 98, 192, 89, 136, 20, 92, 236] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_IsRecurring: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 828087437,
        data2: 32937,
        data3: 20217,
        data4: [174, 22, 142, 116, 109, 165, 29, 112],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_Location: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4129762584,
        data2: 52940,
        data3: 16561,
        data4: [178, 106, 57, 17, 113, 122, 167, 189],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_OptionalAttendeeAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3579555418,
        data2: 14482,
        data3: 16762,
        data4: [166, 73, 198, 172, 90, 170, 234, 179],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_OptionalAttendeeNames: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 155358727,
        data2: 22573,
        data3: 17279,
        data4: [132, 195, 222, 147, 162, 178, 76, 60],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_OrganizerAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1951171138, data2: 19957, data3: 17772, data4: [171, 158, 1, 78, 251, 144, 33, 227] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_OrganizerName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2863030521,
        data2: 39013,
        data3: 17806,
        data4: [180, 132, 1, 188, 127, 227, 151, 62],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_ReminderTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1929141156,
        data2: 9465,
        data3: 16401,
        data4: [159, 63, 173, 210, 122, 250, 216, 24],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_RequiredAttendeeAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 195548867,
        data2: 22157,
        data3: 16729,
        data4: [171, 145, 120, 26, 145, 251, 113, 229],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_RequiredAttendeeNames: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3006984971,
        data2: 62802,
        data3: 17796,
        data4: [147, 108, 203, 147, 229, 205, 162, 159],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_Resources: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16091704, data2: 50507, data3: 19520, data4: [134, 150, 151, 35, 89, 128, 234, 225] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_ResponseStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 411836305,
        data2: 15424,
        data3: 16690,
        data4: [158, 197, 216, 176, 59, 114, 168, 162],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_ShowTimeAs: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1542690516,
        data2: 24242,
        data3: 18031,
        data4: [189, 233, 47, 179, 242, 54, 29, 110],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_ShowTimeAsText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1406818255,
        data2: 25280,
        data3: 17860,
        data4: [129, 222, 118, 16, 188, 239, 215, 245],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Capacity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2601995061, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Comment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_AccountName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_DateItemExpires: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1115701420,
        data2: 41335,
        data3: 19594,
        data4: [151, 96, 246, 247, 97, 34, 127, 154],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_Direction: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2387808304,
        data2: 47456,
        data3: 17222,
        data4: [174, 13, 102, 188, 154, 134, 251, 148],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_FollowupIconIndex: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2208707710,
        data2: 28644,
        data3: 20288,
        data4: [186, 156, 196, 134, 82, 64, 209, 244],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_HeaderItem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3385020292, data2: 8769, data3: 17409, data4: [182, 7, 189, 32, 237, 117, 174, 127] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_PolicyTag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3960160657,
        data2: 43787,
        data3: 19558,
        data4: [144, 182, 198, 99, 124, 222, 187, 171],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_SecurityFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2249827510,
        data2: 40781,
        data3: 17449,
        data4: [140, 15, 185, 150, 202, 89, 227, 53],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_Suffix: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2155570490, data2: 40593, data3: 17391, data4: [143, 151, 17, 206, 4, 238, 32, 197] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_TaskStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3189404358,
        data2: 39453,
        data3: 18103,
        data4: [175, 231, 175, 175, 140, 239, 73, 153],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_TaskStatusText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2792637559, data2: 49719, data3: 18267, data4: [160, 117, 84, 243, 68, 152, 41, 42] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Company: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ComputerName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Computer_DecoratedFreeSpace: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2601995061, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AccountPictureDynamicVideo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 193703960,
        data2: 10021,
        data3: 19268,
        data4: [146, 186, 121, 51, 174, 178, 221, 231],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AccountPictureLarge: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 193703960,
        data2: 10021,
        data3: 19268,
        data4: [146, 186, 121, 51, 174, 178, 221, 231],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AccountPictureSmall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 193703960,
        data2: 10021,
        data3: 19268,
        data4: [146, 186, 121, 51, 174, 178, 221, 231],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Anniversary: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2597698267,
        data2: 52903,
        data3: 17520,
        data4: [160, 61, 184, 78, 81, 185, 148, 158],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AssistantName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3440389276,
        data2: 21824,
        data3: 19080,
        data4: [166, 246, 100, 228, 152, 28, 140, 209],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AssistantTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2593334349,
        data2: 42925,
        data3: 20472,
        data4: [155, 153, 69, 238, 76, 192, 154, 246],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Birthday: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 47u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1930409693,
        data2: 53116,
        data3: 17003,
        data4: [160, 63, 189, 22, 108, 201, 238, 36],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 119u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 117u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 120u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 118u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 116u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 124u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 122u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 125u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 123u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 121u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 129u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 127u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 130u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 128u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 126u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressCity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1076582708,
        data2: 60506,
        data3: 18627,
        data4: [147, 230, 133, 232, 106, 45, 147, 78],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressCountry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2964878100,
        data2: 64758,
        data3: 20459,
        data4: [141, 255, 165, 13, 166, 175, 86, 28],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressPostOfficeBox: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3159257550, data2: 6137, data3: 18645, data4: [190, 233, 2, 29, 240, 234, 84, 9] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressPostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3788808350,
        data2: 55128,
        data3: 19665,
        data4: [182, 236, 52, 168, 181, 167, 63, 128],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1148156031, data2: 4292, data3: 16843, data4: [166, 196, 77, 3, 67, 85, 21, 151] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressStreet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3721479695,
        data2: 49343,
        data3: 17747,
        data4: [140, 228, 16, 67, 60, 144, 143, 176],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessEmailAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4067542617,
        data2: 32350,
        data3: 18207,
        data4: [186, 37, 127, 119, 178, 134, 248, 54],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessFaxNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2448422643,
        data2: 11815,
        data3: 17098,
        data4: [147, 62, 124, 153, 159, 190, 49, 11],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessHomePage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1446054176,
        data2: 9361,
        data3: 18713,
        data4: [153, 206, 234, 219, 6, 250, 253, 178],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1779819936,
        data2: 2590,
        data3: 19671,
        data4: [187, 140, 210, 241, 176, 201, 41, 188],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_CallbackTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3209941443,
        data2: 18912,
        data3: 20351,
        data4: [133, 103, 90, 130, 29, 138, 197, 66],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_CarTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2413587946,
        data2: 47401,
        data3: 16683,
        data4: [186, 144, 57, 122, 37, 116, 101, 254],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3564279556,
        data2: 36593,
        data3: 17391,
        data4: [144, 36, 43, 211, 129, 24, 127, 213],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_CompanyMainTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2240406657,
        data2: 24640,
        data3: 18237,
        data4: [177, 113, 127, 168, 156, 39, 8, 237],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_ConnectedServiceDisplayName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 968326991,
        data2: 41220,
        data3: 18531,
        data4: [179, 149, 45, 178, 173, 143, 123, 193],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_ConnectedServiceIdentities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2163482296,
        data2: 44996,
        data3: 16904,
        data4: [170, 95, 204, 226, 26, 98, 114, 129],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_ConnectedServiceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3049802910,
        data2: 22823,
        data3: 18101,
        data4: [163, 204, 147, 60, 33, 183, 132, 105],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_ConnectedServiceSupportedActions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2711599017, data2: 587, data3: 17265, data4: [168, 191, 77, 41, 195, 228, 233, 201] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DataSuppliers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2522923651,
        data2: 64570,
        data3: 18952,
        data4: [160, 150, 238, 211, 170, 196, 109, 162],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Department: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4238308102, data2: 65423, data3: 19785, data4: [159, 182, 63, 254, 92, 9, 81, 236] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DisplayBusinessPhoneNumbers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 910174426, data2: 55445, data3: 16894, data4: [165, 132, 48, 43, 27, 183, 10, 118] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DisplayHomePhoneNumbers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1349041375, data2: 54935, data3: 19845, data4: [140, 83, 31, 28, 218, 176, 23, 99] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DisplayMobilePhoneNumbers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2628830040,
        data2: 40314,
        data3: 18097,
        data4: [180, 102, 220, 198, 241, 163, 217, 61],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DisplayOtherPhoneNumbers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 50894963, data2: 36584, data3: 16785, data4: [189, 96, 211, 31, 114, 183, 144, 11] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4177166243,
        data2: 53547,
        data3: 18309,
        data4: [138, 78, 105, 26, 148, 247, 163, 231],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailAddress2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 949375075,
        data2: 60872,
        data3: 17000,
        data4: [132, 145, 183, 114, 49, 114, 207, 41],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailAddress3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1682782132,
        data2: 57779,
        data3: 19373,
        data4: [176, 153, 126, 124, 4, 150, 106, 202],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2228810551,
        data2: 38941,
        data3: 17587,
        data4: [150, 21, 199, 89, 109, 186, 23, 227],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3429846820,
        data2: 24707,
        data3: 19412,
        data4: [135, 84, 103, 77, 13, 232, 122, 184],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_FileAsName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4053944999,
        data2: 40103,
        data3: 16630,
        data4: [137, 236, 151, 222, 249, 255, 232, 219],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_FirstName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 345471044, data2: 27465, data3: 19117, data4: [167, 20, 164, 81, 59, 246, 4, 96] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_FullName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1667141713,
        data2: 20645,
        data3: 19362,
        data4: [185, 219, 78, 208, 86, 199, 114, 150],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Gender: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1015869016, data2: 54512, data3: 19705, data4: [183, 86, 78, 93, 36, 68, 123, 205] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_GenderValue: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1015869016, data2: 54512, data3: 19705, data4: [183, 86, 78, 93, 36, 68, 123, 205] },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Hobbies: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1573004607,
        data2: 24081,
        data3: 19167,
        data4: [156, 254, 145, 13, 208, 30, 62, 112],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2566488916, data2: 24954, data3: 18104, data4: [133, 96, 91, 27, 100, 191, 31, 137] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 109u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 107u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 110u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 108u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 106u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 114u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 112u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 115u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 113u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 111u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressCity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 65u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressCountry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 145119905,
        data2: 62665,
        data3: 17373,
        data4: [157, 223, 163, 61, 142, 126, 173, 133],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressPostOfficeBox: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2074043289, data2: 2623, data3: 19218, data4: [137, 189, 74, 220, 81, 201, 24, 175] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressPostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2331820400,
        data2: 35398,
        data3: 19283,
        data4: [158, 238, 144, 186, 231, 21, 30, 98],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3365544912,
        data2: 32109,
        data3: 20152,
        data4: [135, 212, 119, 106, 130, 212, 147, 229],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressStreet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 182382944, data2: 56127, data3: 17160, data4: [154, 33, 6, 35, 123, 22, 250, 42] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeEmailAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1456017053,
        data2: 40262,
        data3: 18787,
        data4: [136, 111, 46, 28, 217, 166, 148, 239],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeFaxNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1712194774, data2: 33195, data3: 18807, data4: [160, 159, 130, 49, 49, 19, 171, 38] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_IMAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3599613322, data2: 13172, data3: 19329, data4: [153, 114, 62, 195, 6, 130, 219, 61] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Initials: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4091081741, data2: 20683, data3: 17570, data4: [151, 24, 64, 203, 145, 25, 73, 93] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JA_CompanyNamePhonetic: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2306553492, data2: 65182, data3: 17382, data4: [128, 102, 38, 15, 89, 12, 1, 0] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JA_FirstNamePhonetic: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2306553492, data2: 65182, data3: 17382, data4: [128, 102, 38, 15, 89, 12, 1, 0] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JA_LastNamePhonetic: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2306553492, data2: 65182, data3: 17382, data4: [128, 102, 38, 15, 89, 12, 1, 0] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1CompanyAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 120u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1CompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1Department: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 106u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1Manager: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1OfficeLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1Title: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1YomiCompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2CompanyAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 121u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2CompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 108u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2Department: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 113u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2Manager: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 112u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2OfficeLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 110u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2Title: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 109u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2YomiCompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 107u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3CompanyAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 123u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3CompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 115u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3Department: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 119u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3Manager: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 118u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3OfficeLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 117u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3Title: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 116u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3YomiCompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 114u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Label: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2544938377, data2: 57161, data3: 18892, data4: [131, 78, 102, 9, 116, 253, 117, 91] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_LastName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2402710016,
        data2: 49776,
        data3: 17788,
        data4: [177, 212, 224, 124, 91, 205, 144, 199],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_MailingAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3232505962,
        data2: 33406,
        data3: 18000,
        data4: [149, 174, 119, 226, 187, 116, 252, 201],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_MiddleName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 71u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_MobileTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_NickName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 74u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OfficeLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1350656506,
        data2: 12603,
        data3: 17365,
        data4: [131, 161, 193, 172, 207, 104, 98, 44],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 134u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 132u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 135u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 133u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 131u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 139u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 137u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 140u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 138u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 136u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 144u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 142u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 145u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 143u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2813785494, data2: 54904, data3: 19393, data4: [176, 95, 2, 3, 210, 126, 138, 161] },
    pid: 141u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressCity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1852320035,
        data2: 32635,
        data3: 20236,
        data4: [163, 55, 207, 202, 41, 102, 135, 191],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressCountry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2400613736,
        data2: 2734,
        data3: 17186,
        data4: [142, 217, 96, 85, 183, 176, 227, 152],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressPostOfficeBox: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2334583361, data2: 1423, data3: 17398, data4: [174, 204, 64, 53, 104, 28, 233, 119] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressPostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2512803521,
        data2: 10943,
        data3: 16712,
        data4: [158, 211, 158, 198, 2, 227, 183, 205],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1907587030,
        data2: 58736,
        data3: 16991,
        data4: [161, 112, 128, 159, 174, 115, 229, 78],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressStreet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4288030217, data2: 47062, data3: 18841, data4: [134, 45, 149, 24, 13, 82, 154, 234] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherEmailAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 299250539,
        data2: 14532,
        data3: 20169,
        data4: [132, 214, 235, 56, 208, 177, 80, 175],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PagerTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3593489921,
        data2: 63733,
        data3: 20293,
        data4: [139, 21, 208, 36, 166, 41, 103, 137],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PersonalTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 69u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PhoneNumbersCanonical: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494040225, data2: 37502, data3: 16565, data4: [165, 3, 110, 219, 212, 42, 81, 126] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Prefix: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 75u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressCity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3370816752, data2: 43491, data3: 18793, data4: [169, 75, 156, 98, 169, 83, 36, 224] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressCountry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3846011293, data2: 3903, data3: 18030, data4: [178, 255, 116, 99, 74, 60, 183, 164] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressPostOfficeBox: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3730764743,
        data2: 18145,
        data3: 18510,
        data4: [153, 153, 98, 197, 48, 131, 148, 193],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressPostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 414962725, data2: 60669, data3: 18159, data4: [182, 18, 123, 74, 96, 52, 237, 160] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4044844542, data2: 28984, data3: 17984, data4: [139, 76, 174, 55, 93, 199, 10, 109] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressStreet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1673681696,
        data2: 38590,
        data3: 18575,
        data4: [135, 136, 192, 156, 64, 122, 216, 18],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryEmailAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 48u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Profession: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1919463253,
        data2: 7396,
        data3: 20334,
        data4: [164, 31, 182, 228, 239, 16, 228, 169],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_SpouseName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2636384438,
        data2: 12647,
        data3: 16939,
        data4: [130, 176, 245, 131, 183, 167, 207, 227],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Suffix: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 393070140, data2: 9864, data3: 20105, data4: [129, 67, 163, 71, 128, 15, 37, 233] },
    pid: 73u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_TTYTDDTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2867948460, data2: 11093, data3: 17894, data4: [159, 109, 65, 94, 185, 73, 16, 223] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_TelexNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3310635324, data2: 49655, data3: 16577, data4: [167, 108, 239, 140, 6, 20, 0, 62] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_WebPage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Webpage2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 124u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Webpage3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 16137688, data2: 8893, data3: 19037, data4: [186, 52, 92, 176, 185, 189, 203, 3] },
    pid: 125u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContainedItems: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContentId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 132u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContentStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContentType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContentUri: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 131u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Copyright: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CreatorAppId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3270116462, data2: 828, data3: 20113, data4: [189, 91, 212, 148, 47, 107, 190, 73] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CreatorOpenWithUIOptions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3270116462, data2: 828, data3: 20113, data4: [189, 91, 212, 148, 47, 107, 190, 73] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_DatePlayExpires: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2930514404,
        data2: 35246,
        data3: 17672,
        data4: [185, 183, 187, 134, 122, 190, 226, 237],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_DatePlayStarts: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2930514404,
        data2: 35246,
        data3: 17672,
        data4: [185, 183, 187, 134, 122, 190, 226, 237],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_Description: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2930514404,
        data2: 35246,
        data3: 17672,
        data4: [185, 183, 187, 134, 122, 190, 226, 237],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_IsDisabled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2930514404,
        data2: 35246,
        data3: 17672,
        data4: [185, 183, 187, 134, 122, 190, 226, 237],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_IsProtected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2930514404,
        data2: 35246,
        data3: 17672,
        data4: [185, 183, 187, 134, 122, 190, 226, 237],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_PlayCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2930514404,
        data2: 35246,
        data3: 17672,
        data4: [185, 183, 187, 134, 122, 190, 226, 237],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DataObjectFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 511812600, data2: 41743, data3: 16967, data4: [185, 238, 29, 3, 104, 169, 66, 92] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateAccessed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateAcquired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 750430453, data2: 55327, data3: 18378, data4: [177, 122, 248, 216, 34, 48, 1, 49] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateArchived: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1140381623, data2: 42052, data3: 20359, data4: [147, 131, 82, 39, 28, 155, 145, 92] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateCompleted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1929033601,
        data2: 44250,
        data3: 17381,
        data4: [177, 85, 178, 67, 79, 133, 230, 120],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateCreated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateImported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 18258u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateModified: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DefaultSaveLocationDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1568061055,
        data2: 39741,
        data3: 17595,
        data4: [182, 174, 37, 218, 79, 99, 138, 103],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DescriptionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_DeviceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_Flags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_LastConnectedTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_ModelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_ProductId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_ProductVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_ServiceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_VendorId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_VendorIdSource: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_IsReadOnly: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3421733648, data2: 18967, data3: 17168, data4: [161, 235, 36, 127, 11, 103, 89, 59] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_ProductId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3421733648, data2: 18967, data3: 17168, data4: [161, 235, 36, 127, 11, 103, 89, 59] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_UsageId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3421733648, data2: 18967, data3: 17168, data4: [161, 235, 36, 127, 11, 103, 89, 59] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_UsagePage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3421733648, data2: 18967, data3: 17168, data4: [161, 235, 36, 127, 11, 103, 89, 59] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_VendorId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3421733648, data2: 18967, data3: 17168, data4: [161, 235, 36, 127, 11, 103, 89, 59] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_VersionNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3421733648, data2: 18967, data3: 17168, data4: [161, 235, 36, 127, 11, 103, 89, 59] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterDriverDirectory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2222745310,
        data2: 47318,
        data3: 19193,
        data4: [171, 195, 111, 79, 146, 107, 192, 57],
    },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterDriverName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2948886896,
        data2: 5365,
        data3: 18828,
        data4: [143, 48, 176, 209, 155, 228, 73, 198],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterEnumerationFlag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2684830369,
        data2: 52620,
        data3: 19255,
        data4: [149, 171, 112, 117, 85, 135, 118, 122],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 175867119, data2: 3111, data3: 17983, data4: [132, 239, 6, 197, 7, 0, 1, 190] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterPortName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4006066017,
        data2: 28564,
        data3: 16817,
        data4: [148, 159, 199, 41, 114, 13, 209, 60],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Proximity_SupportsNfc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4214768333, data2: 40490, data3: 20355, data4: [143, 204, 75, 7, 97, 19, 154, 233] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Serial_PortName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1282142556,
        data2: 19459,
        data3: 19116,
        data4: [145, 245, 100, 192, 248, 82, 188, 244],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Serial_UsbProductId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1282142556,
        data2: 19459,
        data3: 19116,
        data4: [145, 245, 100, 192, 248, 82, 188, 244],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Serial_UsbVendorId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1282142556,
        data2: 19459,
        data3: 19116,
        data4: [145, 245, 100, 192, 248, 82, 188, 244],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_DeviceInterfaceClasses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2514560949, data2: 31180, data3: 20099, data4: [156, 158, 132, 34, 24, 123, 62, 14] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2514560949, data2: 31180, data3: 20099, data4: [156, 158, 132, 34, 24, 123, 62, 14] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbProductId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2514560949, data2: 31180, data3: 20099, data4: [156, 158, 132, 34, 24, 123, 62, 14] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbProtocol: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2514560949, data2: 31180, data3: 20099, data4: [156, 158, 132, 34, 24, 123, 62, 14] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbSubClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2514560949, data2: 31180, data3: 20099, data4: [156, 158, 132, 34, 24, 123, 62, 14] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbVendorId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2514560949, data2: 31180, data3: 20099, data4: [156, 158, 132, 34, 24, 123, 62, 14] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PrinterURL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 189330266, data2: 48750, data3: 20247, data4: [177, 8, 60, 64, 115, 209, 102, 154] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_CanPair: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_Categories: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_DialProtocol_InstalledApplications: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_IsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_IsPresent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_ModelIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_ProtocolIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 196746974, data2: 30054, data3: 20295, data4: [144, 236, 37, 252, 86, 124, 237, 42] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportedUriSchemes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsAudio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsCapturing: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsImages: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsLimitedDiscovery: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsNetworking: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsObjectTransfer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsPositioning: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsRendering: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsTelephony: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsVideo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1794465093,
        data2: 14555,
        data3: 17557,
        data4: [172, 176, 212, 114, 138, 59, 131, 20],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_AepId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3384885673,
        data2: 6988,
        data3: 20247,
        data4: [169, 209, 242, 152, 83, 140, 173, 184],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_Bluetooth_CacheMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2537828638,
        data2: 31057,
        data3: 19246,
        data4: [182, 240, 236, 178, 147, 202, 193, 25],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_Bluetooth_ServiceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2744756935,
        data2: 49765,
        data3: 18254,
        data4: [176, 115, 255, 206, 87, 114, 23, 22],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_Bluetooth_TargetDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2537828638,
        data2: 31057,
        data3: 19246,
        data4: [182, 240, 236, 178, 147, 202, 193, 25],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1903314774,
        data2: 15988,
        data3: 17458,
        data4: [155, 89, 231, 178, 246, 104, 165, 147],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1903314774,
        data2: 15988,
        data3: 17458,
        data4: [155, 89, 231, 178, 246, 104, 165, 147],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_IoT_ServiceInterfaces: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2044284546,
        data2: 19833,
        data3: 17834,
        data4: [130, 26, 116, 133, 139, 78, 76, 166],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ParentAepIsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3384885673,
        data2: 6988,
        data3: 20247,
        data4: [169, 209, 242, 152, 83, 140, 173, 184],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ProtocolId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3384885673,
        data2: 6988,
        data3: 20247,
        data4: [169, 209, 242, 152, 83, 140, 173, 184],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ServiceClassId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1903314774,
        data2: 15988,
        data3: 17458,
        data4: [155, 89, 231, 178, 246, 104, 165, 147],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ServiceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3384885673,
        data2: 6988,
        data3: 20247,
        data4: [169, 209, 242, 152, 83, 140, 173, 184],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_AepId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 992796678,
        data2: 24161,
        data3: 20446,
        data4: [186, 184, 155, 138, 172, 155, 38, 223],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Major: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Minor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Audio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Capturing: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Information: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_LimitedDiscovery: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Networking: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_ObjectXfer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Positioning: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Rendering: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Telephony: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1606235341,
        data2: 22042,
        data3: 16686,
        data4: [186, 152, 71, 138, 107, 15, 239, 29],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_LastSeenTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735477131, data2: 35819, data3: 18645, data4: [135, 224, 108, 218, 52, 40, 4, 10] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_AddressType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2573136048,
        data2: 32435,
        data3: 19083,
        data4: [185, 206, 6, 139, 179, 244, 175, 105],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_Appearance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2573136048,
        data2: 32435,
        data3: 19083,
        data4: [185, 206, 6, 139, 179, 244, 175, 105],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_Appearance_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2573136048,
        data2: 32435,
        data3: 19083,
        data4: [185, 206, 6, 139, 179, 244, 175, 105],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_Appearance_Subcategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2573136048,
        data2: 32435,
        data3: 19083,
        data4: [185, 206, 6, 139, 179, 244, 175, 105],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_IsConnectable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2573136048,
        data2: 32435,
        data3: 19083,
        data4: [185, 206, 6, 139, 179, 244, 175, 105],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_CanPair: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3888380713,
        data2: 51879,
        data3: 20295,
        data4: [140, 139, 190, 89, 179, 48, 212, 197],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2740557483,
        data2: 4559,
        data3: 18741,
        data4: [139, 97, 166, 118, 16, 129, 236, 223],
    },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3888380713,
        data2: 51879,
        data3: 20295,
        data4: [140, 139, 190, 89, 179, 48, 212, 197],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_DeviceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2740557483,
        data2: 4559,
        data3: 18741,
        data4: [139, 97, 166, 118, 16, 129, 236, 223],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2740557483,
        data2: 4559,
        data3: 18741,
        data4: [139, 97, 166, 118, 16, 129, 236, 223],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_IsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2740557483,
        data2: 4559,
        data3: 18741,
        data4: [139, 97, 166, 118, 16, 129, 236, 223],
    },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_IsPresent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2740557483,
        data2: 4559,
        data3: 18741,
        data4: [139, 97, 166, 118, 16, 129, 236, 223],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2740557483,
        data2: 4559,
        data3: 18741,
        data4: [139, 97, 166, 118, 16, 129, 236, 223],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_ModelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2740557483,
        data2: 4559,
        data3: 18741,
        data4: [139, 97, 166, 118, 16, 129, 236, 223],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2740557483,
        data2: 4559,
        data3: 18741,
        data4: [139, 97, 166, 118, 16, 129, 236, 223],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_PointOfService_ConnectionTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3569312179,
        data2: 17454,
        data3: 19162,
        data4: [136, 45, 250, 123, 112, 200, 50, 217],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_ProtocolId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 992796678,
        data2: 24161,
        data3: 20446,
        data4: [186, 184, 155, 138, 172, 155, 38, 223],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_SignalStrength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2740557483,
        data2: 4559,
        data3: 18741,
        data4: [139, 97, 166, 118, 16, 129, 236, 223],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AppPackageFamilyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1361274243, data2: 3146, data3: 20456, data4: [184, 31, 22, 106, 236, 19, 245, 16] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_Microphone_IsFarField: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2302915443,
        data2: 14476,
        data3: 17301,
        data4: [181, 87, 188, 109, 186, 255, 175, 219],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2302915443,
        data2: 14476,
        data3: 17301,
        data4: [181, 87, 188, 109, 186, 255, 175, 219],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2302915443,
        data2: 14476,
        data3: 17301,
        data4: [181, 87, 188, 109, 186, 255, 175, 219],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_Microphone_SignalToNoiseRatioInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2302915443,
        data2: 14476,
        data3: 17301,
        data4: [181, 87, 188, 109, 186, 255, 175, 219],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_RawProcessingSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2302915443,
        data2: 14476,
        data3: 17301,
        data4: [181, 87, 188, 109, 186, 255, 175, 219],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_SpeechProcessingSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4213041252, data2: 57453, data3: 18420, data4: [130, 166, 138, 10, 239, 68, 73, 60] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_BatteryLife: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_BatteryPlusCharging: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_BatteryPlusChargingText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 91u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_CategoryGroup: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 94u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_CategoryIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 90u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_CategoryPlural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 92u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ChallengeAep: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 125055326,
        data2: 46868,
        data3: 18668,
        data4: [141, 232, 129, 37, 192, 119, 172, 17],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ChargingState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_CompatibleIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Connected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 55u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2357121542,
        data2: 16266,
        data3: 18471,
        data4: [179, 171, 174, 158, 31, 174, 252, 108],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DefaultTooltip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2282713250,
        data2: 24706,
        data3: 18348,
        data4: [138, 171, 167, 57, 209, 163, 0, 195],
    },
    pid: 153u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DevObjectType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 325533506,
        data2: 41942,
        data3: 18934,
        data4: [180, 218, 174, 70, 224, 197, 35, 124],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceCapabilities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceCharacteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceDescription1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 81u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceDescription2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 82u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceHasProblem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1410045054,
        data2: 35648,
        data3: 17852,
        data4: [168, 162, 106, 11, 137, 76, 189, 162],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceInstanceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 256u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceManufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DialProtocol_InstalledApplications: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1749404786,
        data2: 7025,
        data3: 18627,
        data4: [175, 134, 176, 145, 113, 161, 155, 20],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DiscoveryMethod: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 52u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_Domain: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_FullName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_HostName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_InstanceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_NetworkAdapterId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_PortNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_Priority: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_ServiceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_TextAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_Ttl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_Weight: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3212427435, data2: 47988, data3: 19694, data4: [176, 112, 71, 11, 90, 226, 2, 234] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12288u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_FunctionPaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3498955968,
        data2: 15006,
        data3: 17966,
        data4: [130, 144, 123, 99, 107, 37, 118, 185],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_GlyphIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1361274243, data2: 3146, data3: 20456, data4: [184, 31, 22, 106, 236, 19, 245, 16] },
    pid: 123u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_HardwareIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 57u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_InLocalMachineContainer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2357121542,
        data2: 16266,
        data3: 18471,
        data4: [179, 171, 174, 158, 31, 174, 252, 108],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_InterfaceClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 40784238, data2: 47124, data3: 16715, data4: [131, 205, 133, 109, 111, 239, 72, 34] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_InterfaceEnabled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 40784238, data2: 47124, data3: 16715, data4: [131, 205, 133, 109, 111, 239, 72, 34] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_InterfacePaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3498955968,
        data2: 15006,
        data3: 17966,
        data4: [130, 144, 123, 99, 107, 37, 118, 185],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IpAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12297u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IsDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 86u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IsNetworkConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 85u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IsShared: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 84u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IsSoftwareInstalling: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2212127526, data2: 38822, data3: 16520, data4: [148, 83, 161, 146, 63, 87, 59, 41] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_LaunchDeviceStageFromExplorer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 77u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_LocalMachine: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 70u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_LocationPaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8192u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_MetadataPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 71u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_MicrophoneArray_Geometry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2709692066,
        data2: 10219,
        data3: 17822,
        data4: [147, 93, 178, 250, 215, 176, 119, 98],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_MissedCalls: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ModelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2161647270, data2: 29811, data3: 19212, data4: [130, 22, 239, 193, 26, 44, 76, 139] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8194u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ModelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8195u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NetworkName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NetworkType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NetworkedTooltip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2282713250,
        data2: 24706,
        data3: 18348,
        data4: [138, 171, 167, 57, 209, 163, 0, 195],
    },
    pid: 152u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NewPictures: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NotWorkingProperly: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 83u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notification: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 108022540,
        data2: 59440,
        data3: 19585,
        data4: [145, 120, 145, 228, 233, 90, 128, 160],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NotificationStore: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 108022540,
        data2: 59440,
        data3: 19585,
        data4: [145, 120, 145, 228, 233, 90, 128, 160],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_LowBattery: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3300949803, data2: 34084, data3: 20070, data4: [174, 58, 166, 35, 95, 16, 59, 235] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_MissedCall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1712648008,
        data2: 20222,
        data3: 17444,
        data4: [158, 218, 199, 159, 64, 78, 223, 62],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_NewMessage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 736699914, data2: 8210, data3: 18242, data4: [165, 85, 244, 27, 99, 139, 125, 203] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_NewVoicemail: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1498846550,
        data2: 2568,
        data3: 16914,
        data4: [149, 185, 250, 226, 173, 100, 19, 219],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_StorageFull: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2699038433,
        data2: 61639,
        data3: 19777,
        data4: [184, 231, 38, 167, 189, 141, 56, 176],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_StorageFullLinkText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2699038433,
        data2: 61639,
        data3: 19777,
        data4: [184, 231, 38, 167, 189, 141, 56, 176],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Paired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 56u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Panel_PanelGroup: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2377948294,
        data2: 38825,
        data3: 19455,
        data4: [155, 198, 191, 233, 93, 62, 109, 173],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Panel_PanelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2377948294,
        data2: 38825,
        data3: 19455,
        data4: [155, 198, 191, 233, 93, 62, 109, 173],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Parent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PhoneLineTransportDevice_Connected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2932813800,
        data2: 7424,
        data3: 20462,
        data4: [138, 109, 167, 13, 113, 155, 119, 43],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PhysicalDeviceLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1410045054,
        data2: 35648,
        data3: 17852,
        data4: [168, 162, 106, 11, 137, 76, 189, 162],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PlaybackPositionPercent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 909368921,
        data2: 26661,
        data3: 17281,
        data4: [164, 155, 159, 107, 161, 58, 20, 113],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PlaybackState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 909368921,
        data2: 26661,
        data3: 17281,
        data4: [164, 155, 159, 107, 161, 58, 20, 113],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PlaybackTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 909368921,
        data2: 26661,
        data3: 17281,
        data4: [164, 155, 159, 107, 161, 58, 20, 113],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Present: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1410045054,
        data2: 35648,
        data3: 17852,
        data4: [168, 162, 106, 11, 137, 76, 189, 162],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PresentationUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8198u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PrimaryCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3498955968,
        data2: 15006,
        data3: 17966,
        data4: [130, 144, 123, 99, 107, 37, 118, 185],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_RemainingDuration: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 909368921,
        data2: 26661,
        data3: 17281,
        data4: [164, 155, 159, 107, 161, 58, 20, 113],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_RestrictedInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 40784238, data2: 47124, data3: 16715, data4: [131, 205, 133, 109, 111, 239, 72, 34] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Roaming: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SafeRemovalRequired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2950264384,
        data2: 34467,
        data3: 16912,
        data4: [182, 124, 40, 156, 65, 170, 190, 85],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SchematicName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 40784238, data2: 47124, data3: 16715, data4: [131, 205, 133, 109, 111, 239, 72, 34] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 16384u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ServiceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 16385u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SharedTooltip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2282713250,
        data2: 24706,
        data3: 18348,
        data4: [138, 171, 167, 57, 209, 163, 0, 195],
    },
    pid: 151u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SignalStrength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SmartCards_ReaderKind: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3602233475,
        data2: 6333,
        data3: 19277,
        data4: [178, 236, 158, 56, 175, 254, 218, 130],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3498955968,
        data2: 15006,
        data3: 17966,
        data4: [130, 144, 123, 99, 107, 37, 118, 185],
    },
    pid: 259u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Status1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3498955968,
        data2: 15006,
        data3: 17966,
        data4: [130, 144, 123, 99, 107, 37, 118, 185],
    },
    pid: 257u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Status2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3498955968,
        data2: 15006,
        data3: 17966,
        data4: [130, 144, 123, 99, 107, 37, 118, 185],
    },
    pid: 258u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_StorageCapacity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_StorageFreeSpace: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_StorageFreeSpacePercent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_TextMessages: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Voicemail: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1238179702, data2: 22054, data3: 19223, data4: [164, 232, 24, 180, 170, 26, 34, 19] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_AdvertisementId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_RequestServiceInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_ServiceConfigMethods: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_ServiceInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_ServiceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_DeviceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_GroupId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_InformationElements: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_InterfaceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_IsLegacyDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_IsMiracastLcpSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_IsVisible: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_MiracastVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_Services: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_SupportedChannelList: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFi_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4010895339,
        data2: 52220,
        data3: 17217,
        data4: [165, 104, 167, 201, 26, 104, 152, 44],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiaDeviceType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1809653702, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WinPhone8CameraFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3082081820,
        data2: 23140,
        data3: 16775,
        data4: [165, 46, 177, 83, 159, 53, 144, 153],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Wwan_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4279330795,
        data2: 52220,
        data3: 17217,
        data4: [165, 104, 167, 201, 26, 104, 152, 44],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_ByteCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_CharacterCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_ClientID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 661486512, data2: 23348, data3: 20400, data4: [170, 75, 21, 142, 209, 42, 24, 9] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Contributor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4080275806, data2: 55835, data3: 17673, data4: [155, 61, 17, 149, 4, 220, 122, 187] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_DateCreated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_DatePrinted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_DateSaved: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Division: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 503340774,
        data2: 48935,
        data3: 17035,
        data4: [176, 28, 121, 103, 106, 205, 40, 112],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_DocumentID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3767010760,
        data2: 58261,
        data3: 16607,
        data4: [128, 210, 84, 240, 214, 196, 49, 84],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_HiddenSlideCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_LastAuthor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_LineCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Manager: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_MultimediaClipCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_NoteCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_PageCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_ParagraphCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_PresentationFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_RevisionNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_SlideCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Template: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_TotalEditingTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_WordCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DueDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1065644725,
        data2: 57519,
        data3: 19890,
        data4: [128, 113, 197, 63, 231, 106, 231, 206],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EdgeGesture_DisableTouchWhenFullscreen: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 852375730,
        data2: 11418,
        data3: 16817,
        data4: [155, 197, 179, 120, 67, 148, 170, 68],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EndDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3344935429,
        data2: 38653,
        data3: 18919,
        data4: [156, 180, 159, 96, 16, 130, 213, 83],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ExpandoProperties: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1872891366, data2: 53532, data3: 19869, data4: [161, 84, 100, 49, 118, 40, 193, 45] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileAllocationSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 217021779, data2: 64100, data3: 4561, data4: [162, 3, 0, 0, 248, 31, 237, 238] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3841002044,
        data2: 18918,
        data3: 16477,
        data4: [130, 136, 162, 59, 212, 238, 170, 108],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileFRN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1104108256,
        data2: 63322,
        data3: 18438,
        data4: [189, 135, 89, 199, 217, 36, 142, 185],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileOfflineAvailabilityStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileOwner: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2601995060, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FilePlaceholderStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3002710486,
        data2: 65220,
        data3: 19925,
        data4: [148, 215, 137, 87, 72, 140, 128, 123],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 217021779, data2: 64100, data3: 4561, data4: [162, 3, 0, 0, 248, 31, 237, 238] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FindData: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FlagColor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1742705886, data2: 3239, data3: 19823, data4: [183, 146, 5, 58, 62, 79, 3, 207] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FlagColorText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1173022535,
        data2: 36394,
        data3: 16558,
        data4: [140, 191, 202, 82, 171, 166, 21, 42],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FlagStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FlagStatusText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3696557358, data2: 6301, data3: 18545, data4: [170, 1, 8, 194, 245, 122, 74, 188] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FolderKind: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FolderNameDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FreeSpace: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2601995061, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FullText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 507439168, data2: 48171, data3: 18284, data4: [130, 55, 42, 205, 26, 131, 155, 34] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Altitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2189351759,
        data2: 23411,
        data3: 17575,
        data4: [137, 29, 253, 255, 171, 234, 53, 202],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_AltitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2016685515,
        data2: 58200,
        data3: 16709,
        data4: [174, 154, 107, 254, 78, 15, 159, 81],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_AltitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 766320311,
        data2: 33133,
        data3: 16595,
        data4: [158, 195, 201, 119, 59, 226, 170, 222],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_AltitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1185702557, data2: 30186, data3: 17685, data4: [134, 127, 109, 196, 50, 28, 88, 68] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_AreaInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2536387390,
        data2: 44158,
        data3: 18929,
        data4: [138, 223, 167, 13, 7, 169, 188, 171],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DOP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 217643778, data2: 6199, data3: 17137, data4: [166, 151, 167, 1, 122, 162, 137, 185] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DOPDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2696844485, data2: 20666, data3: 18555, data4: [189, 53, 6, 84, 190, 136, 129, 237] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DOPNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1192651542,
        data2: 13903,
        data3: 19104,
        data4: [159, 49, 226, 171, 61, 244, 73, 195],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Date: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 906151954, data2: 3899, data3: 17904, data4: [133, 173, 96, 52, 104, 214, 148, 35] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestBearing: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3329051452,
        data2: 59528,
        data3: 18380,
        data4: [185, 159, 157, 202, 62, 227, 77, 234],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestBearingDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2059203832,
        data2: 31807,
        data3: 18824,
        data4: [172, 145, 141, 44, 46, 151, 236, 165],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestBearingNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3124436393,
        data2: 34542,
        data3: 19293,
        data4: [162, 164, 162, 113, 164, 41, 240, 207],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestBearingRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2595767187,
        data2: 10767,
        data3: 19317,
        data4: [187, 34, 114, 121, 120, 105, 119, 203],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestDistance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2839457284, data2: 26628, data3: 20260, data4: [172, 129, 9, 178, 102, 69, 33, 24] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestDistanceDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2613234075,
        data2: 44145,
        data3: 16679,
        data4: [157, 28, 37, 150, 208, 215, 220, 183],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestDistanceNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 735725530, data2: 2246, data3: 20449, data4: [128, 188, 167, 47, 197, 23, 197, 208] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestDistanceRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3981308627,
        data2: 34453,
        data3: 17675,
        data4: [133, 111, 245, 193, 197, 58, 203, 102],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLatitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2635955397,
        data2: 23609,
        data3: 17692,
        data4: [134, 179, 146, 142, 45, 24, 204, 71],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLatitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 976691858,
        data2: 32714,
        data3: 18855,
        data4: [153, 213, 228, 123, 178, 212, 231, 171],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLatitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3975460598,
        data2: 54694,
        data3: 17212,
        data4: [187, 146, 64, 118, 101, 15, 200, 144],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLatitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3467124921, data2: 52833, data3: 18565, data4: [161, 40, 0, 93, 144, 135, 193, 146] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLongitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1202283105,
        data2: 52044,
        data3: 18439,
        data4: [138, 211, 64, 185, 217, 219, 198, 188],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLongitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1113418213,
        data2: 18605,
        data3: 18688,
        data4: [141, 128, 110, 182, 184, 208, 172, 134],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLongitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2737111682,
        data2: 64365,
        data3: 18645,
        data4: [154, 137, 219, 202, 206, 117, 204, 207],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLongitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 405544614, data2: 31772, data3: 16515, data4: [171, 75, 172, 108, 159, 78, 209, 40] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Differential: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2868178469,
        data2: 48443,
        data3: 19927,
        data4: [191, 196, 71, 247, 123, 176, 15, 109],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ImgDirection: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 373767313,
        data2: 53271,
        data3: 20185,
        data4: [186, 77, 182, 186, 165, 93, 188, 248],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ImgDirectionDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 280118677, data2: 16802, data3: 20000, data4: [147, 194, 87, 97, 193, 57, 95, 50] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ImgDirectionNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3696785351, data2: 8799, data3: 17911, data4: [186, 199, 232, 19, 52, 182, 19, 10] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ImgDirectionRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2762646967,
        data2: 6864,
        data3: 17503,
        data4: [129, 26, 15, 143, 110, 103, 246, 181],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Latitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2267533311,
        data2: 18536,
        data3: 20166,
        data4: [173, 91, 129, 185, 133, 33, 209, 171],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LatitudeDecimal: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 257281506, data2: 20297, data3: 17677, data4: [146, 193, 220, 209, 99, 1, 177, 183] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LatitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 384185582, data2: 11263, data3: 18811, data4: [189, 138, 67, 65, 173, 57, 238, 185] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LatitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2111482577,
        data2: 52424,
        data3: 16814,
        data4: [183, 80, 178, 203, 128, 49, 174, 162],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LatitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 43778642,
        data2: 23430,
        data3: 18119,
        data4: [172, 160, 39, 105, 255, 200, 227, 212],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Longitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3301235634,
        data2: 46483,
        data3: 18027,
        data4: [187, 218, 208, 61, 39, 213, 228, 58],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LongitudeDecimal: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1182384565, data2: 33869, data3: 17808, data4: [186, 245, 243, 34, 35, 31, 27, 129] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LongitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3194885996,
        data2: 17716,
        data3: 19756,
        data4: [172, 229, 49, 222, 218, 193, 96, 107],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LongitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 45151881, data2: 43284, data3: 20037, data4: [130, 29, 29, 218, 69, 46, 210, 196] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LongitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 870117931, data2: 10453, data3: 17996, data4: [128, 53, 30, 233, 239, 210, 82, 120] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_MapDatum: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 748870374,
        data2: 60892,
        data3: 16509,
        data4: [190, 241, 119, 57, 66, 171, 250, 149],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_MeasureMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2685791581, data2: 43754, data3: 19800, data4: [138, 134, 60, 88, 105, 32, 234, 11] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ProcessingMethod: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1507106401, data2: 33807, data3: 19113, data4: [169, 57, 226, 9, 155, 127, 99, 153] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Satellites: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1182721397, data2: 7973, data3: 17751, data4: [173, 78, 184, 181, 139, 13, 156, 21] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Speed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3663530082, data2: 28278, data3: 19995, data4: [186, 189, 112, 2, 27, 210, 84, 148] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_SpeedDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2098343258, data2: 44638, data3: 17205, data4: [136, 65, 215, 30, 124, 231, 47, 83] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_SpeedNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2898906685, data2: 49683, data3: 18754, data4: [139, 72, 109, 8, 32, 242, 28, 109] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_SpeedRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3975673033,
        data2: 21583,
        data3: 19821,
        data4: [157, 152, 138, 215, 154, 218, 244, 83],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 307532276, data2: 33167, data3: 18098, data4: [145, 181, 213, 55, 117, 54, 23, 178] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Track: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1992333635,
        data2: 31795,
        data3: 18915,
        data4: [158, 126, 205, 186, 135, 44, 250, 218],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_TrackDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3369177612, data2: 502, data3: 16576, data4: [172, 134, 47, 58, 74, 208, 7, 112] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_TrackNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1881745140, data2: 17574, data3: 17377, data4: [174, 113, 69, 98, 113, 22, 137, 59] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_TrackRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 903603966,
        data2: 17603,
        data3: 17408,
        data4: [170, 174, 210, 199, 153, 196, 7, 232],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_VersionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 577785252,
        data2: 50866,
        data3: 19097,
        data4: [142, 86, 241, 109, 248, 201, 37, 153],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_HighKeywords: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_History_SelectionCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 484497084,
        data2: 21356,
        data3: 17920,
        data4: [176, 221, 126, 12, 102, 179, 80, 213],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_History_TargetUrlHostName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 484497084,
        data2: 21356,
        data3: 17920,
        data4: [176, 221, 126, 12, 102, 179, 80, 213],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_History_VisitCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1556031367, data2: 18639, data3: 16904, data4: [185, 14, 238, 94, 93, 66, 2, 148] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2725202684, data2: 29510, data3: 17049, data4: [190, 71, 235, 26, 230, 19, 19, 159] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IdentityProvider_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3111059323, data2: 13770, data3: 18997, data4: [134, 7, 41, 227, 165, 76, 70, 234] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IdentityProvider_Picture: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 606410351,
        data2: 22082,
        data3: 18532,
        data4: [153, 47, 152, 253, 152, 242, 148, 195],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_Blob: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2352714660, data2: 47853, data3: 6787, data4: [154, 50, 16, 46, 227, 19, 246, 235] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_DisplayName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2103984073,
        data2: 53589,
        data3: 17832,
        data4: [187, 31, 137, 209, 155, 203, 121, 47],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_InternetSid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1835883849, data2: 9821, data3: 18056, data4: [159, 78, 31, 221, 51, 231, 204, 131] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_IsMeIdentity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2752546568,
        data2: 2527,
        data3: 17271,
        data4: [157, 252, 109, 153, 152, 109, 90, 103],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_KeyProviderContext: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2725202684, data2: 29510, data3: 17049, data4: [190, 71, 235, 26, 230, 19, 19, 159] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_KeyProviderName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2725202684, data2: 29510, data3: 17049, data4: [190, 71, 235, 26, 230, 19, 19, 159] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_LogonStatusString: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4052610547, data2: 13183, data3: 17088, data4: [158, 3, 206, 224, 135, 8, 168, 195] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_PrimaryEmailAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4240533539,
        data2: 47853,
        data3: 20260,
        data4: [155, 50, 160, 152, 33, 23, 247, 250],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_PrimarySid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 723222558,
        data2: 49345,
        data3: 18823,
        data4: [158, 197, 114, 250, 137, 129, 71, 135],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_ProviderData: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2829536146, data2: 13851, data3: 20122, data4: [183, 34, 124, 74, 115, 48, 163, 18] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_ProviderID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1957158473, data2: 64017, data3: 19773, data4: [160, 6, 219, 126, 8, 103, 89, 22] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_QualifiedUserName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3662810705, data2: 62697, data3: 18233, data4: [172, 130, 2, 224, 169, 92, 144, 48] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_UniqueID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3848258480,
        data2: 11104,
        data3: 16928,
        data4: [145, 142, 178, 30, 139, 241, 96, 22],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_UserName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3291620611,
        data2: 30922,
        data3: 18886,
        data4: [154, 204, 166, 142, 42, 253, 123, 107],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ImageParsingName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3614772960,
        data2: 50852,
        data3: 18668,
        data4: [181, 62, 184, 123, 82, 230, 208, 115],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_BitDepth: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179215, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_ColorSpace: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 40961u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_CompressedBitsPerPixel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 910913449, data2: 14251, data3: 18474, data4: [190, 43, 174, 2, 246, 13, 67, 24] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_CompressedBitsPerPixelDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 529024225, data2: 9389, data3: 17672, data4: [157, 253, 83, 38, 164, 21, 206, 2] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_CompressedBitsPerPixelNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3524948296, data2: 54060, data3: 17956, data4: [137, 0, 39, 114, 16, 247, 156, 15] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_Compression: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 259u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_CompressionText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1057547887, data2: 12100, data3: 19385, data4: [166, 130, 172, 53, 210, 86, 35, 34] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_Dimensions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179215, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_HorizontalResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179215, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_HorizontalSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179215, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_ImageID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 282770949, data2: 12970, data3: 19497, data4: [191, 26, 99, 226, 210, 32, 88, 127] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_ResolutionUnit: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 431300518, data2: 8082, data3: 19036, data4: [171, 72, 125, 240, 171, 214, 116, 68] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_VerticalResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179215, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_VerticalSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179215, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Importance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ImportanceText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2746390417, data2: 30483, data3: 19997, data4: [187, 64, 23, 219, 133, 240, 24, 49] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_InfoTipText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_InternalName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 217021779, data2: 64100, data3: 4561, data4: [162, 3, 0, 0, 248, 31, 237, 238] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsAttachment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4064232028, data2: 29089, data3: 20392, data4: [146, 47, 103, 142, 164, 166, 4, 8] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsDefaultNonOwnerSaveLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1568061055,
        data2: 39741,
        data3: 17595,
        data4: [182, 174, 37, 218, 79, 99, 138, 103],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsDefaultSaveLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1568061055,
        data2: 39741,
        data3: 17595,
        data4: [182, 174, 37, 218, 79, 99, 138, 103],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsDeleted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1557815240,
        data2: 13294,
        data3: 20467,
        data4: [144, 148, 174, 123, 216, 134, 140, 77],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsEncrypted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2430984526,
        data2: 25739,
        data3: 18470,
        data4: [178, 170, 172, 175, 121, 14, 53, 19],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsFlagged: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1571309413,
        data2: 58367,
        data3: 17016,
        data4: [134, 176, 162, 121, 103, 251, 221, 3],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsFlaggedComplete: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2800967890, data2: 22009, data3: 18654, data4: [185, 9, 98, 14, 9, 10, 100, 124] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsIncomplete: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 879528913,
        data2: 11882,
        data3: 19525,
        data4: [137, 164, 97, 183, 142, 142, 112, 15],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsLocationSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1568061055,
        data2: 39741,
        data3: 17595,
        data4: [182, 174, 37, 218, 79, 99, 138, 103],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsPinnedToNameSpaceTree: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1568061055,
        data2: 39741,
        data3: 17595,
        data4: [182, 174, 37, 218, 79, 99, 138, 103],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsRead: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsSearchOnlyItem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1568061055,
        data2: 39741,
        data3: 17595,
        data4: [182, 174, 37, 218, 79, 99, 138, 103],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsSendToTarget: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsShared: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4018687067,
        data2: 11262,
        data3: 16827,
        data4: [170, 229, 118, 238, 223, 79, 153, 2],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemAuthors: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3500166922, data2: 17962, data3: 18596, data4: [187, 47, 55, 6, 232, 141, 189, 125] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemClassType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 75913389, data2: 11704, data3: 16804, data4: [187, 182, 172, 30, 241, 32, 126, 177] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4158354612,
        data2: 17031,
        data3: 16643,
        data4: [175, 186, 241, 177, 61, 205, 117, 207],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemFolderNameDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemFolderPathDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemFolderPathDisplayNarrow: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3669831917, data2: 67, data3: 18313, data4: [167, 248, 208, 19, 164, 115, 102, 34] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1804443764,
        data2: 15196,
        data3: 17340,
        data4: [136, 111, 10, 44, 220, 224, 11, 111],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemNameDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemNameDisplayWithoutExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemNamePrefix: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3610329073,
        data2: 42874,
        data3: 16412,
        data4: [140, 153, 61, 189, 214, 138, 221, 54],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemNameSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemParticipants: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3570444822,
        data2: 39240,
        data3: 16804,
        data4: [170, 133, 217, 127, 249, 100, 105, 147],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemPathDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemPathDisplayNarrow: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemTypeText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1231625360, data2: 32279, data3: 4122, data4: [169, 28, 8, 0, 43, 46, 205, 169] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Journal_Contacts: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3735537708,
        data2: 7561,
        data3: 19046,
        data4: [148, 39, 164, 227, 222, 186, 188, 177],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Journal_EntryType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2512302588,
        data2: 12909,
        data3: 17988,
        data4: [179, 150, 205, 62, 217, 14, 109, 223],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Keywords: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Kind: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 507439168, data2: 48171, data3: 18284, data4: [130, 55, 42, 205, 26, 131, 155, 34] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_KindText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4031508373,
        data2: 50565,
        data3: 16791,
        data4: [162, 183, 223, 70, 253, 201, 238, 109],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Language: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3587036418, data2: 11932, data3: 4123, data4: [147, 151, 8, 0, 43, 44, 249, 174] },
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LastSyncError: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 107u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LastSyncWarning: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 128u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LastWriterPackageFamilyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1345126059,
        data2: 18411,
        data3: 17820,
        data4: [185, 96, 230, 216, 114, 143, 119, 1],
    },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LayoutPattern_ContentViewModeForBrowse: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 500u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LayoutPattern_ContentViewModeForSearch: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 501u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LibraryLocationsCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2424739527,
        data2: 36743,
        data3: 17650,
        data4: [128, 237, 168, 193, 198, 137, 69, 117],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_Arguments: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1131357799, data2: 5346, data3: 20459, data4: [179, 10, 20, 108, 83, 181, 182, 116] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_Comment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3115627516, data2: 11089, data3: 19010, data4: [181, 216, 50, 65, 70, 175, 207, 37] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_DateVisited: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1556031367, data2: 18639, data3: 16904, data4: [185, 14, 238, 94, 93, 66, 2, 148] },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_Description: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1556031367, data2: 18639, data3: 16904, data4: [185, 14, 238, 94, 93, 66, 2, 148] },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_FeedItemLocalId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2318375417,
        data2: 15415,
        data3: 18013,
        data4: [168, 215, 105, 119, 122, 36, 109, 12],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3115627516, data2: 11089, data3: 19010, data4: [181, 216, 50, 65, 70, 175, 207, 37] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2055042804,
        data2: 46640,
        data3: 19415,
        data4: [149, 255, 55, 204, 81, 169, 117, 201],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetParsingPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3115627516, data2: 11089, data3: 19010, data4: [181, 216, 50, 65, 70, 175, 207, 37] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetSFGAOFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3115627516, data2: 11089, data3: 19010, data4: [181, 216, 50, 65, 70, 175, 207, 37] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetSFGAOFlagsStrings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3600031873, data2: 54587, data3: 17469, data4: [173, 71, 94, 5, 157, 156, 210, 122] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1556031367, data2: 18639, data3: 16904, data4: [185, 14, 238, 94, 93, 66, 2, 148] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetUrlHostName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2318375417,
        data2: 15415,
        data3: 18013,
        data4: [168, 215, 105, 119, 122, 36, 109, 12],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetUrlPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2318375417,
        data2: 15415,
        data3: 18013,
        data4: [168, 215, 105, 119, 122, 36, 109, 12],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LowKeywords: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MIMEType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 191095632, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_AuthorUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_AverageLevel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 166581686, data2: 45825, data3: 17349, data4: [153, 144, 208, 3, 2, 239, 253, 70] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ClassPrimaryID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ClassSecondaryID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_CollectionGroupID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_CollectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ContentDistributor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ContentID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_CreatorApplication: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_CreatorApplicationVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_DVDID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_DateEncoded: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 776692749, data2: 20505, data3: 18136, data4: [136, 129, 85, 65, 76, 197, 202, 160] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_DateReleased: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3728854057,
        data2: 26993,
        data3: 17040,
        data4: [180, 114, 245, 159, 46, 47, 49, 226],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_DlnaProfileID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3483573061, data2: 21085, data3: 18840, data4: [187, 68, 63, 125, 129, 84, 47, 164] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Duration: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179216, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_EncodedBy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_EncodingSettings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_EpisodeNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_FrameCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179215, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_MCDI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_MetadataContentProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Producer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_PromotionUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ProtectionType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ProviderRating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 39u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ProviderStyle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 40u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Publisher: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_SeasonNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_SeriesName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 42u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_SubTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_SubscriptionContentId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2599136890,
        data2: 38468,
        data3: 18557,
        data4: [169, 44, 101, 117, 133, 237, 117, 26],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ThumbnailLargePath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 47u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ThumbnailLargeUri: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 48u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ThumbnailSmallPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 49u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ThumbnailSmallUri: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 50u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_UniqueFileIdentifier: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_UserNoAutoInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 41u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_UserWebUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Writer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Year: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MediumKeywords: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_AttachmentContents: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 826523516,
        data2: 32936,
        data3: 18516,
        data4: [136, 128, 226, 228, 1, 137, 189, 208],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_AttachmentNames: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_BccAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_BccName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_CcAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_CcName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ConversationID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3700392125, data2: 44830, data3: 17033, data4: [133, 182, 61, 252, 27, 73, 57, 146] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ConversationIndex: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3700392125, data2: 44830, data3: 17033, data4: [133, 182, 61, 252, 27, 73, 57, 146] },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_DateReceived: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_DateSent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_Flags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2821562087, data2: 51815, data3: 17170, data4: [150, 94, 34, 107, 206, 168, 80, 35] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_FromAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_FromName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_HasAttachments: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2619330420,
        data2: 11671,
        data3: 16826,
        data4: [180, 174, 203, 46, 54, 97, 166, 228],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_IsFwdOrReply: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2593898632, data2: 20333, data3: 18078, data4: [153, 25, 231, 5, 65, 32, 64, 249] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_MessageClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3449738328, data2: 2254, data3: 16783, data4: [167, 14, 249, 18, 199, 187, 156, 92] },
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_Participants: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 446408197, data2: 36476, data3: 19729, data4: [173, 125, 165, 10, 218, 24, 186, 27] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ProofInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2425942844, data2: 39549, data3: 18600, data4: [141, 229, 46, 18, 39, 166, 78, 145] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_SenderAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 199346407, data2: 6529, data3: 18038, data4: [174, 20, 253, 215, 143, 5, 166, 231] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_SenderName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 228859130, data2: 53796, data3: 18968, data4: [174, 47, 89, 97, 88, 219, 75, 58] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_Store: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ToAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ToDoFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 528837279, data2: 26880, data3: 19130, data4: [149, 5, 45, 95, 27, 77, 102, 203] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ToDoTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3167521340,
        data2: 36079,
        data3: 17125,
        data4: [155, 28, 198, 144, 121, 57, 139, 199],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ToName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3823130700,
        data2: 46984,
        data3: 19034,
        data4: [187, 32, 127, 90, 68, 201, 172, 221],
    },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MileageInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4260905840, data2: 794, data3: 19165, data4: [158, 145, 13, 119, 95, 28, 102, 5] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumArtist: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumArtistSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4059935919, data2: 63372, data3: 18028, data4: [187, 5, 86, 233, 45, 176, 184, 236] },
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumTitleSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 334200828,
        data2: 60553,
        data3: 17222,
        data4: [177, 157, 204, 198, 241, 120, 66, 35],
    },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Artist: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_ArtistSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3739954613,
        data2: 1686,
        data3: 19680,
        data4: [148, 254, 160, 31, 119, 164, 95, 181],
    },
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_BeatsPerMinute: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Composer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_ComposerSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 12329123, data2: 48456, data3: 16517, data4: [135, 44, 168, 141, 119, 245, 9, 126] },
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Conductor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_ContentGroupDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_DiscNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1795060791,
        data2: 39885,
        data3: 18887,
        data4: [128, 254, 74, 92, 101, 250, 88, 116],
    },
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_DisplayArtist: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4245825875,
        data2: 64147,
        data3: 20215,
        data4: [146, 195, 4, 201, 70, 178, 247, 200],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Genre: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_InitialKey: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_IsCompilation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3293173195,
        data2: 40612,
        data3: 18441,
        data4: [130, 232, 175, 157, 89, 222, 214, 209],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Lyrics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Mood: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 39u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_PartOfSet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Period: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_SynchronizedLyrics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1797405546,
        data2: 5678,
        data3: 19113,
        data4: [179, 159, 5, 214, 120, 252, 109, 119],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_TrackNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_NamespaceCLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Note_Color: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1198967546, data2: 48356, data3: 19633, data4: [162, 62, 38, 94, 118, 216, 235, 17] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Note_ColorText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1186261214, data2: 52658, data3: 17421, data4: [136, 92, 22, 88, 235, 101, 185, 20] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Null: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_OfflineAvailability: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2839972022,
        data2: 32159,
        data3: 17776,
        data4: [166, 72, 227, 223, 192, 171, 43, 63],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_OfflineStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1831110799,
        data2: 18200,
        data3: 19418,
        data4: [175, 237, 234, 15, 180, 56, 108, 216],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_OriginalFileName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 217021779, data2: 64100, data3: 4561, data4: [162, 3, 0, 0, 248, 31, 237, 238] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_OwnerSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1568061055,
        data2: 39741,
        data3: 17595,
        data4: [182, 174, 37, 218, 79, 99, 138, 103],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParentalRating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParentalRatingReason: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 278416906,
        data2: 63986,
        data3: 17185,
        data4: [183, 239, 186, 241, 149, 175, 67, 25],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParentalRatingsOrganization: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2818443328, data2: 4932, data3: 18160, data4: [141, 55, 82, 237, 113, 42, 75, 249] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParsingBindContext: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3753484365, data2: 13871, data3: 19619, data4: [179, 11, 2, 84, 177, 123, 91, 132] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParsingName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParsingPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PerceivedType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PercentFull: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2601995061, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Aperture: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 37378u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ApertureDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3785991051, data2: 26245, data3: 18109, data4: [135, 94, 87, 13, 199, 173, 115, 32] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ApertureNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 53996780, data2: 14843, data3: 17793, data4: [160, 189, 76, 76, 197, 30, 153, 20] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Brightness: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 443554806, data2: 18316, data3: 17249, data4: [131, 171, 55, 1, 187, 5, 60, 88] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_BrightnessDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1857972550,
        data2: 8993,
        data3: 17418,
        data4: [144, 240, 192, 67, 239, 211, 36, 118],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_BrightnessNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2658996623,
        data2: 45844,
        data3: 17824,
        data4: [140, 251, 214, 84, 185, 23, 201, 233],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_CameraManufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 271u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_CameraModel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 272u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_CameraSerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 273u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Contrast: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 712530857, data2: 36131, data3: 19949, data4: [130, 230, 96, 163, 80, 200, 106, 16] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ContrastText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1507715570,
        data2: 21075,
        data3: 16618,
        data4: [154, 139, 71, 158, 150, 198, 36, 154],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_DateTaken: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 36867u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_DigitalZoom: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4166776896,
        data2: 43301,
        data3: 19394,
        data4: [176, 196, 142, 54, 181, 152, 103, 158],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_DigitalZoomDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1952165646,
        data2: 58817,
        data3: 19707,
        data4: [138, 27, 208, 49, 160, 165, 35, 147],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_DigitalZoomNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 382449956,
        data2: 25856,
        data3: 18235,
        data4: [165, 190, 241, 89, 155, 203, 228, 19],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_EXIFVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3546248250, data2: 60206, data3: 18418, data4: [162, 134, 132, 65, 50, 203, 20, 39] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Event: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 18248u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureBias: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 37380u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureBiasDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2871025232, data2: 1207, data3: 17948, data4: [161, 140, 47, 35, 56, 54, 230, 39] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureBiasNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1938551428,
        data2: 7559,
        data3: 16907,
        data4: [146, 207, 88, 52, 191, 110, 249, 237],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureIndex: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2524666616,
        data2: 39258,
        data3: 18157,
        data4: [158, 17, 53, 179, 197, 185, 120, 45],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureIndexDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2467377033, data2: 49803, data3: 18735, data4: [138, 157, 75, 226, 6, 44, 238, 138] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureIndexNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3454914352,
        data2: 35097,
        data3: 17631,
        data4: [143, 76, 78, 178, 255, 219, 141, 137],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureProgram: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 34850u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureProgramText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4274426039,
        data2: 24368,
        data3: 17990,
        data4: [174, 71, 76, 170, 251, 168, 132, 163],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 33434u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureTimeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1441367447, data2: 44310, data3: 17120, data4: [182, 36, 33, 89, 154, 25, 152, 56] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureTimeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 629032162, data2: 36913, data3: 17187, data4: [172, 56, 133, 197, 82, 135, 27, 46] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 33437u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FNumberDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3911853206,
        data2: 8763,
        data3: 17507,
        data4: [164, 227, 48, 234, 187, 167, 157, 128],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FNumberNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 462910346, data2: 65020, data3: 17967, data4: [157, 147, 25, 87, 224, 139, 233, 12] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Flash: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 37385u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashEnergy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 41483u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashEnergyDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3619036272,
        data2: 25379,
        data3: 18893,
        data4: [165, 252, 200, 66, 119, 22, 44, 151],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashEnergyNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4239211837,
        data2: 2136,
        data3: 16399,
        data4: [170, 163, 47, 102, 204, 226, 166, 188],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashManufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2864379593,
        data2: 57541,
        data3: 18201,
        data4: [133, 133, 87, 177, 3, 229, 132, 254],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashModel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4270046005,
        data2: 19738,
        data3: 17122,
        data4: [145, 107, 6, 243, 225, 175, 113, 158],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1804298486, data2: 8203, data3: 18410, data4: [141, 37, 216, 5, 15, 87, 51, 159] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalLength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 37386u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalLengthDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 811320853, data2: 56481, data3: 17573, data4: [159, 212, 16, 192, 186, 121, 65, 46] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalLengthInFilm: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2699511305, data2: 47181, data3: 20297, data4: [184, 96, 70, 43, 217, 151, 31, 152] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalLengthNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2003528507, data2: 7741, data3: 19212, data4: [154, 14, 143, 186, 242, 168, 73, 42] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneXResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3485502871,
        data2: 50935,
        data3: 17540,
        data4: [137, 221, 235, 239, 67, 86, 254, 118],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneXResolutionDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 154399733,
        data2: 18310,
        data3: 20294,
        data4: [168, 232, 214, 77, 211, 127, 165, 33],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneXResolutionNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3704295599, data2: 46306, data3: 19336, data4: [149, 249, 3, 27, 77, 90, 180, 144] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneYResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1342170320,
        data2: 37199,
        data3: 19140,
        data4: [141, 111, 201, 198, 29, 225, 105, 177],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneYResolutionDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 492927398, data2: 43126, data3: 16433, data4: [176, 19, 51, 71, 178, 182, 77, 200] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneYResolutionNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2732933573,
        data2: 17472,
        data3: 19368,
        data4: [134, 126, 117, 207, 192, 104, 40, 205],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_GainControl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4197468041, data2: 199, data3: 19840, data4: [144, 74, 30, 77, 204, 114, 101, 170] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_GainControlDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1116098045,
        data2: 40356,
        data3: 20343,
        data4: [189, 237, 74, 173, 123, 37, 103, 53],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_GainControlNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2391723900,
        data2: 47032,
        data3: 20152,
        data4: [166, 63, 14, 231, 21, 201, 111, 158],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_GainControlText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3227662514, data2: 3065, data3: 17017, data4: [167, 35, 37, 133, 103, 21, 203, 157] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ISOSpeed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 34855u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_LensManufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3873295095,
        data2: 10693,
        data3: 20234,
        data4: [154, 104, 209, 148, 18, 236, 112, 144],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_LensModel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3777459478,
        data2: 11103,
        data3: 18537,
        data4: [137, 177, 46, 88, 91, 211, 139, 122],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_LightSource: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 37384u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MakerNote: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4197462867,
        data2: 46681,
        data3: 16466,
        data4: [133, 233, 188, 172, 121, 84, 155, 132],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MakerNoteOffset: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2168406308, data2: 13542, data3: 19735, data4: [171, 62, 107, 31, 60, 34, 71, 161] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MaxAperture: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 150394818, data2: 58354, data3: 17660, data4: [175, 30, 90, 165, 200, 26, 45, 62] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MaxApertureDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3346474196,
        data2: 24607,
        data3: 18117,
        data4: [155, 137, 197, 63, 147, 188, 235, 119],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MaxApertureNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3238519185,
        data2: 42073,
        data3: 17605,
        data4: [154, 230, 185, 82, 173, 75, 144, 109],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MeteringMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 37383u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MeteringModeText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4129881484,
        data2: 31656,
        data3: 18010,
        data4: [166, 91, 197, 170, 121, 38, 58, 158],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Orientation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 274u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_OrientationText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2850691388,
        data2: 50449,
        data3: 18826,
        data4: [160, 107, 88, 226, 119, 109, 204, 40],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_PeopleNames: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3895499630, data2: 2124, data3: 18868, data4: [177, 252, 144, 168, 3, 49, 182, 56] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_PhotometricInterpretation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 873961201,
        data2: 7673,
        data3: 19228,
        data4: [165, 100, 145, 189, 239, 164, 56, 119],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_PhotometricInterpretationText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2182363094, data2: 40619, data3: 18277, data4: [165, 137, 59, 28, 187, 210, 42, 97] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ProgramMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1830911853, data2: 16234, data3: 18469, data4: [180, 112, 95, 3, 202, 47, 190, 155] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ProgramModeText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2145626663, data2: 9800, data3: 17139, data4: [137, 176, 69, 78, 92, 177, 80, 195] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_RelatedSoundFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 831155013, data2: 2175, data3: 19906, data4: [184, 204, 5, 53, 149, 81, 252, 158] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Saturation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1227060005,
        data2: 43354,
        data3: 20327,
        data4: [178, 17, 129, 107, 45, 69, 210, 224],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SaturationText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1632078856,
        data2: 46592,
        data3: 19076,
        data4: [187, 228, 233, 156, 69, 240, 160, 114],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Sharpness: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4234770139,
        data2: 33609,
        data3: 18800,
        data4: [174, 151, 179, 197, 49, 106, 8, 240],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SharpnessText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1374437191, data2: 56656, data3: 16925, data4: [135, 105, 51, 79, 80, 66, 75, 30] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ShutterSpeed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 37377u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ShutterSpeedDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3778906485,
        data2: 33223,
        data3: 18760,
        data4: [174, 63, 55, 202, 225, 30, 143, 247],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ShutterSpeedNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 384450626, data2: 55028, data3: 19402, data4: [131, 73, 124, 120, 211, 15, 179, 51] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SubjectDistance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 37382u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SubjectDistanceDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 209980040,
        data2: 45123,
        data3: 18029,
        data4: [151, 102, 212, 178, 109, 163, 250, 119],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SubjectDistanceNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2331285020,
        data2: 62758,
        data3: 17381,
        data4: [170, 129, 219, 118, 130, 25, 23, 141],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_TagViewAggregate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3088249181, data2: 49880, data3: 19391, data4: [186, 205, 121, 116, 67, 70, 17, 63] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_TranscodedForSync: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2593045365, data2: 25688, data3: 20098, data4: [186, 203, 53, 192, 9, 91, 3, 187] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_WhiteBalance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3996990858,
        data2: 21377,
        data3: 19706,
        data4: [177, 59, 170, 246, 107, 95, 78, 201],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_WhiteBalanceText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1664530782,
        data2: 51111,
        data3: 17005,
        data4: [134, 253, 122, 227, 211, 156, 132, 180],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Priority: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2619330420,
        data2: 11671,
        data3: 16826,
        data4: [180, 174, 203, 46, 54, 97, 166, 228],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PriorityText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3649825163,
        data2: 47211,
        data3: 16533,
        data4: [191, 82, 157, 35, 178, 224, 167, 82],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Project: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 967309602,
        data2: 18300,
        data3: 18654,
        data4: [139, 200, 178, 132, 65, 227, 66, 227],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Advanced: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2416590907, data2: 2427, data3: 19349, data4: [138, 226, 7, 31, 218, 238, 177, 24] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Audio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 671405161,
        data2: 30863,
        data3: 18602,
        data4: [133, 112, 113, 185, 193, 135, 225, 56],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Calendar: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2574504629, data2: 49112, data3: 17290, data4: [186, 148, 83, 73, 178, 147, 24, 26] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Camera: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3724598834, data2: 21630, data3: 18817, data4: [173, 75, 84, 47, 46, 144, 7, 216] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Contact: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3751239635, data2: 9482, data3: 16388, data4: [133, 143, 52, 226, 154, 62, 55, 170] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Content: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3503993018, data2: 13962, data3: 16464, data4: [168, 130, 108, 1, 15, 209, 154, 79] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Description: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2305405557,
        data2: 38005,
        data3: 19968,
        data4: [168, 135, 255, 147, 184, 180, 30, 68],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_FileSystem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3819426497, data2: 33020, data3: 19264, data4: [143, 52, 48, 234, 17, 27, 220, 46] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_GPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4084284122,
        data2: 37091,
        data3: 19985,
        data4: [170, 229, 253, 193, 118, 133, 185, 190],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_General: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3425703472, data2: 45458, data3: 19490, data4: [179, 114, 159, 76, 109, 51, 142, 7] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Image: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3815312007,
        data2: 4008,
        data3: 18986,
        data4: [154, 159, 252, 232, 130, 112, 85, 172],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Media: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1636248823, data2: 27486, data3: 19275, data4: [172, 45, 89, 218, 132, 69, 146, 72] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_MediaAdvanced: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2287575684,
        data2: 56958,
        data3: 17986,
        data4: [153, 186, 212, 49, 208, 68, 177, 236],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Message: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2144806301,
        data2: 5812,
        data3: 16693,
        data4: [159, 151, 124, 150, 236, 210, 250, 158],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Music: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1759338644, data2: 29206, data3: 16625, data4: [160, 41, 67, 254, 113, 39, 4, 63] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Origin: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 630772475,
        data2: 21865,
        data3: 17255,
        data4: [149, 223, 92, 211, 161, 119, 225, 165],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_PhotoAdvanced: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 213040986, data2: 40679, data3: 19078, data4: [130, 34, 240, 30, 7, 253, 173, 175] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_RecordedTV: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3887280696,
        data2: 25988,
        data3: 16752,
        data4: [165, 192, 172, 37, 239, 217, 218, 86],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Video: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3200125216,
        data2: 30321,
        data3: 19540,
        data4: [163, 235, 73, 253, 223, 193, 145, 238],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_ConflictPrompt: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_ContentViewModeForBrowse: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_ContentViewModeForSearch: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_ExtendedTileInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_FileOperationPrompt: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_FullDetails: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_InfoTip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_NonPersonal: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1238436127,
        data2: 2094,
        data3: 18751,
        data4: [178, 63, 210, 48, 138, 169, 102, 140],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_PreviewDetails: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_PreviewTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_QuickTip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_TileInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3381938721, data2: 41990, data3: 18686, data4: [130, 37, 174, 199, 226, 76, 33, 27] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_XPDetailsPanel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4062663808,
        data2: 63362,
        data3: 17041,
        data4: [189, 148, 241, 54, 147, 81, 58, 236],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ProviderItemID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4062026049,
        data2: 33264,
        data3: 18202,
        data4: [173, 238, 78, 116, 180, 146, 23, 237],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Rating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RatingText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2417589415,
        data2: 64911,
        data3: 20108,
        data4: [157, 163, 181, 126, 30, 96, 146, 149],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_ChannelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_Credits: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_DateContentExpires: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_EpisodeName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsATSCContent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsClosedCaptioningAvailable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsDTVContent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsHDContent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsRepeatBroadcast: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsSAP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_NetworkAffiliation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 743688211, data2: 64355, data3: 20002, data4: [161, 171, 11, 51, 28, 161, 226, 115] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_OriginalBroadcastDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1183121047, data2: 34661, data3: 18498, data4: [156, 19, 240, 6, 68, 123, 23, 140] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_ProgramDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_RecordingTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2772926305,
        data2: 31362,
        data3: 20170,
        data4: [157, 222, 152, 182, 155, 36, 121, 179],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_StationCallSign: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1836355042, data2: 36152, data3: 19651, data4: [172, 96, 240, 9, 176, 87, 197, 87] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_StationName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 458504679,
        data2: 60321,
        data3: 19192,
        data4: [189, 215, 122, 241, 212, 84, 148, 147],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RemoteConflictingFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 115u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFGAOFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_AutoSummary: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1443641024, data2: 20538, data3: 4559, data4: [186, 161, 0, 0, 76, 117, 42, 154] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_ContainerHash: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3169772163,
        data2: 13791,
        data3: 19795,
        data4: [130, 106, 243, 106, 62, 239, 198, 190],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_Contents: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_EntryID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1231625360, data2: 32279, data3: 4122, data4: [169, 28, 8, 0, 43, 46, 205, 169] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_ExtendedProperties: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2063840582, data2: 64079, data3: 19026, data4: [162, 254, 3, 213, 49, 30, 88, 101] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_GatherTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 191095632, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_HitCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1231625360, data2: 32279, data3: 4122, data4: [169, 28, 8, 0, 43, 46, 205, 169] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_IsClosedDirectory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 191095619, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_IsFullyContained: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 191095619, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_QueryFocusedSummary: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1443641024, data2: 20538, data3: 4559, data4: [186, 161, 0, 0, 76, 117, 42, 154] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_QueryFocusedSummaryWithFallback: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1443641024, data2: 20538, data3: 4559, data4: [186, 161, 0, 0, 76, 117, 42, 154] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_QueryPropertyHits: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1231625360, data2: 32279, data3: 4122, data4: [169, 28, 8, 0, 43, 46, 205, 169] },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_Rank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1231625360, data2: 32279, data3: 4122, data4: [169, 28, 8, 0, 43, 46, 205, 169] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_Store: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2691273395,
        data2: 36015,
        data3: 20183,
        data4: [165, 71, 178, 89, 227, 42, 201, 252],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_UrlToIndex: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 191095619, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_UrlToIndexWithModificationTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 191095619, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Security_AllowedEnterpriseDataProtectionIdentities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 953430912,
        data2: 54296,
        data3: 18480,
        data4: [132, 213, 70, 147, 90, 129, 197, 198],
    },
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Security_EncryptionOwners: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1599799146,
        data2: 14309,
        data3: 18304,
        data4: [151, 234, 128, 199, 86, 92, 245, 53],
    },
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Security_EncryptionOwnersDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3730971535, data2: 57637, data3: 17315, data4: [163, 45, 86, 101, 68, 109, 99, 42] },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sensitivity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4174640812, data2: 18548, data3: 17099, data4: [190, 89, 171, 69, 75, 48, 113, 106] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SensitivityText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3502764116,
        data2: 16242,
        data3: 18213,
        data4: [133, 39, 18, 154, 87, 124, 178, 105],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ShareUserRating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SharedWith: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4018687067,
        data2: 11262,
        data3: 16827,
        data4: [170, 229, 118, 238, 223, 79, 153, 2],
    },
    pid: 200u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SharingStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4018687067,
        data2: 11262,
        data3: 16827,
        data4: [170, 229, 118, 238, 223, 79, 153, 2],
    },
    pid: 300u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Shell_OmitFromView: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3728024972,
        data2: 50837,
        data3: 19644,
        data4: [185, 130, 56, 176, 173, 36, 206, 208],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Shell_SFGAOFlagsStrings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3600031873, data2: 54587, data3: 17469, data4: [173, 71, 94, 5, 157, 156, 210, 122] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SimpleRating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2694776910, data2: 44353, data3: 18591, data4: [128, 118, 170, 91, 227, 8, 43, 202] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Size: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SoftwareUsed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] },
    pid: 305u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Software_DateLastUsed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2216578960,
        data2: 65369,
        data3: 19734,
        data4: [137, 71, 232, 27, 191, 250, 179, 109],
    },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Software_ProductName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 217021779, data2: 64100, data3: 4561, data4: [162, 3, 0, 0, 248, 31, 237, 238] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SourceItem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1720508325, data2: 31259, data3: 17187, data4: [174, 75, 229, 39, 57, 58, 29, 129] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SourcePackageFamilyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4289633719, data2: 7309, data3: 17407, data4: [129, 140, 132, 64, 58, 163, 115, 45] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StartDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1224568520,
        data2: 35346,
        data3: 19679,
        data4: [160, 62, 78, 197, 165, 17, 237, 222],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 136353, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StatusBarSelectedItemCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 651962492, data2: 28221, data3: 19411, data4: [178, 176, 106, 38, 186, 46, 52, 109] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StatusBarViewItemCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 651962492, data2: 28221, data3: 19411, data4: [178, 176, 106, 38, 186, 46, 52, 109] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderCallerVersionInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3002710486,
        data2: 65220,
        data3: 19925,
        data4: [148, 215, 137, 87, 72, 140, 128, 123],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderError: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 109u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileChecksum: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3002710486,
        data2: 65220,
        data3: 19925,
        data4: [148, 215, 137, 87, 72, 140, 128, 123],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3002710486,
        data2: 65220,
        data3: 19925,
        data4: [148, 215, 137, 87, 72, 140, 128, 123],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileHasConflict: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3002710486,
        data2: 65220,
        data3: 19925,
        data4: [148, 215, 137, 87, 72, 140, 128, 123],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileIdentifier: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3002710486,
        data2: 65220,
        data3: 19925,
        data4: [148, 215, 137, 87, 72, 140, 128, 123],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileRemoteUri: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 112u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3002710486,
        data2: 65220,
        data3: 19925,
        data4: [148, 215, 137, 87, 72, 140, 128, 123],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileVersionWaterline: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3002710486,
        data2: 65220,
        data3: 19925,
        data4: [148, 215, 137, 87, 72, 140, 128, 123],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 108u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderShareStatuses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 111u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderSharingStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 117u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 110u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Storage_Portable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Storage_RemovableMedia: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Storage_SystemCritical: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Subject: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_Album: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 208908609,
        data2: 14806,
        data3: 18003,
        data4: [166, 131, 202, 178, 145, 234, 249, 91],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_AlbumID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 208908609,
        data2: 14806,
        data3: 18003,
        data4: [166, 131, 202, 178, 145, 234, 249, 91],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_Location: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 208908609,
        data2: 14806,
        data3: 18003,
        data4: [166, 131, 202, 178, 145, 234, 249, 91],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_Person: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 208908609,
        data2: 14806,
        data3: 18003,
        data4: [166, 131, 202, 178, 145, 234, 249, 91],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_ResourceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 208908609,
        data2: 14806,
        data3: 18003,
        data4: [166, 131, 202, 178, 145, 234, 249, 91],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_Tag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 208908609,
        data2: 14806,
        data3: 18003,
        data4: [166, 131, 202, 178, 145, 234, 249, 91],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SyncTransferStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_Comments: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2077578046,
        data2: 44821,
        data3: 17627,
        data4: [184, 200, 189, 102, 36, 225, 208, 50],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ConflictDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3461398873,
        data2: 12216,
        data3: 16893,
        data4: [190, 104, 211, 224, 66, 226, 116, 188],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ConflictFirstLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3461398873,
        data2: 12216,
        data3: 16893,
        data4: [190, 104, 211, 224, 66, 226, 116, 188],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ConflictSecondLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3461398873,
        data2: 12216,
        data3: 16893,
        data4: [190, 104, 211, 224, 66, 226, 116, 188],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerCollectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2077578046,
        data2: 44821,
        data3: 17627,
        data4: [184, 200, 189, 102, 36, 225, 208, 50],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2077578046,
        data2: 44821,
        data3: 17627,
        data4: [184, 200, 189, 102, 36, 225, 208, 50],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3461398873,
        data2: 12216,
        data3: 16893,
        data4: [190, 104, 211, 224, 66, 226, 116, 188],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2077578046,
        data2: 44821,
        data3: 17627,
        data4: [184, 200, 189, 102, 36, 225, 208, 50],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerTypeLabel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2077578046,
        data2: 44821,
        data3: 17627,
        data4: [184, 200, 189, 102, 36, 225, 208, 50],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ItemID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2077578046,
        data2: 44821,
        data3: 17627,
        data4: [184, 200, 189, 102, 36, 225, 208, 50],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ItemName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3461398873,
        data2: 12216,
        data3: 16893,
        data4: [190, 104, 211, 224, 66, 226, 116, 188],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ProgressPercentage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2077578046,
        data2: 44821,
        data3: 17627,
        data4: [184, 200, 189, 102, 36, 225, 208, 50],
    },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_State: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2077578046,
        data2: 44821,
        data3: 17627,
        data4: [184, 200, 189, 102, 36, 225, 208, 50],
    },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2077578046,
        data2: 44821,
        data3: 17627,
        data4: [184, 200, 189, 102, 36, 225, 208, 50],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Task_BillingInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3548205766, data2: 9756, data3: 17155, data4: [130, 179, 8, 185, 38, 172, 111, 18] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Task_CompletionStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 139299338,
        data2: 59093,
        data3: 16606,
        data4: [191, 31, 200, 130, 14, 124, 135, 124],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Task_Owner: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 147311711,
        data2: 24818,
        data3: 17556,
        data4: [173, 117, 85, 227, 224, 181, 173, 208],
    },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Thumbnail: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ThumbnailCacheId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1147999921, data2: 36269, data3: 18544, data4: [167, 72, 64, 46, 164, 61, 120, 140] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ThumbnailStream: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Title: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4070540768, data2: 20473, data3: 4200, data4: [171, 145, 8, 0, 43, 39, 179, 217] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TitleSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4042758221,
        data2: 8750,
        data3: 19154,
        data4: [130, 171, 29, 216, 234, 64, 229, 126],
    },
    pid: 300u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TotalFileSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Trademarks: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 217021779, data2: 64100, data3: 4561, data4: [162, 3, 0, 0, 248, 31, 237, 238] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TransferOrder: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 106u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TransferPosition: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TransferSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4243583315,
        data2: 59449,
        data3: 19699,
        data4: [169, 231, 234, 34, 131, 32, 148, 184],
    },
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_Compression: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_Director: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179218, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_EncodingBitrate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_FourCC: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 44u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_FrameHeight: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_FrameRate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_FrameWidth: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_HorizontalAspectRatio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 42u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_IsSpherical: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_IsStereo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 98u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_Orientation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 99u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_SampleSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_StreamName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_StreamNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_TotalBitrate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 43u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_TranscodedForSync: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 46u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_VerticalAspectRatio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1682179217, data2: 19595, data3: 4561, data4: [139, 112, 8, 0, 54, 177, 26, 3] },
    pid: 45u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_VolumeId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1147999921, data2: 36269, data3: 18544, data4: [167, 72, 64, 46, 164, 61, 120, 140] },
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Volume_FileSystem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2601995061, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Volume_IsMappedDrive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 345770857, data2: 11309, data3: 18684, data4: [128, 143, 211, 24, 215, 140, 70, 54] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Volume_IsRoot: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2601995061, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ZoneIdentifier: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1345126059,
        data2: 18411,
        data3: 17820,
        data4: [185, 96, 230, 216, 114, 143, 119, 1],
    },
    pid: 100u32,
};
pub const PLAYBACKSTATE_NOMEDIA: u32 = 7u32;
pub const PLAYBACKSTATE_PAUSED: u32 = 4u32;
pub const PLAYBACKSTATE_PLAYING: u32 = 2u32;
pub const PLAYBACKSTATE_RECORDING: u32 = 6u32;
pub const PLAYBACKSTATE_RECORDINGPAUSED: u32 = 5u32;
pub const PLAYBACKSTATE_STOPPED: u32 = 1u32;
pub const PLAYBACKSTATE_TRANSITIONING: u32 = 3u32;
pub const PLAYBACKSTATE_UNKNOWN: u32 = 0u32;
pub const RATING_FIVE_STARS_MAX: u32 = 99u32;
pub const RATING_FIVE_STARS_MIN: u32 = 88u32;
pub const RATING_FIVE_STARS_SET: u32 = 99u32;
pub const RATING_FOUR_STARS_MAX: u32 = 87u32;
pub const RATING_FOUR_STARS_MIN: u32 = 63u32;
pub const RATING_FOUR_STARS_SET: u32 = 75u32;
pub const RATING_ONE_STAR_MAX: u32 = 12u32;
pub const RATING_ONE_STAR_MIN: u32 = 1u32;
pub const RATING_ONE_STAR_SET: u32 = 1u32;
pub const RATING_THREE_STARS_MAX: u32 = 62u32;
pub const RATING_THREE_STARS_MIN: u32 = 38u32;
pub const RATING_THREE_STARS_SET: u32 = 50u32;
pub const RATING_TWO_STARS_MAX: u32 = 37u32;
pub const RATING_TWO_STARS_MIN: u32 = 13u32;
pub const RATING_TWO_STARS_SET: u32 = 25u32;
pub const SHARINGSTATUS_NOTSHARED: u32 = 0u32;
pub const SHARINGSTATUS_PRIVATE: u32 = 2u32;
pub const SHARINGSTATUS_SHARED: u32 = 1u32;
#[repr(C)]
pub struct SILO_INFO {
    pub ulSTID: u32,
    pub SpecificationMajor: u8,
    pub SpecificationMinor: u8,
    pub ImplementationMajor: u8,
    pub ImplementationMinor: u8,
    pub r#type: u8,
    pub capabilities: u8,
}
impl ::core::marker::Copy for SILO_INFO {}
impl ::core::clone::Clone for SILO_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const STORAGE_PROVIDER_SHARINGSTATUS_NOTSHARED: u32 = 0u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_PRIVATE: u32 = 2u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC: u32 = 3u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_COOWNED: u32 = 7u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_OWNED: u32 = 6u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED: u32 = 1u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED_COOWNED: u32 = 5u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED_OWNED: u32 = 4u32;
pub const SYNC_HANDLERTYPE_COMPUTERS: u32 = 5u32;
pub const SYNC_HANDLERTYPE_DEVICES: u32 = 2u32;
pub const SYNC_HANDLERTYPE_FOLDERS: u32 = 3u32;
pub const SYNC_HANDLERTYPE_OTHER: u32 = 0u32;
pub const SYNC_HANDLERTYPE_PROGRAMS: u32 = 1u32;
pub const SYNC_HANDLERTYPE_WEBSERVICES: u32 = 4u32;
pub const SYNC_STATE_ERROR: u32 = 3u32;
pub const SYNC_STATE_IDLE: u32 = 2u32;
pub const SYNC_STATE_NOTSETUP: u32 = 0u32;
pub const SYNC_STATE_PENDING: u32 = 4u32;
pub const SYNC_STATE_SYNCING: u32 = 5u32;
pub const SYNC_STATE_SYNCNOTRUN: u32 = 1u32;
pub const WPD_CATEGORY_ENHANCED_STORAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2435088742,
    data2: 47154,
    data3: 19156,
    data4: [186, 164, 124, 160, 182, 178, 121, 140],
};
