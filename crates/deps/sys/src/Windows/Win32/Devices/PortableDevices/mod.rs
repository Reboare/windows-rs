#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type DMProcessConfigXMLFiltered = unsafe extern "system" fn(pszxmlin: super::super::Foundation::PWSTR, rgszallowedcspnodes: *const super::super::Foundation::PWSTR, dwnumallowedcspnodes: u32, pbstrxmlout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
pub const CLSID_WPD_NAMESPACE_EXTENSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 897084732, data2: 45173, data3: 18873, data4: [136, 221, 2, 152, 118, 225, 28, 1] };
pub type DELETE_OBJECT_OPTIONS = i32;
pub const PORTABLE_DEVICE_DELETE_NO_RECURSION: DELETE_OBJECT_OPTIONS = 0i32;
pub const PORTABLE_DEVICE_DELETE_WITH_RECURSION: DELETE_OBJECT_OPTIONS = 1i32;
pub type DEVICE_RADIO_STATE = i32;
pub const DRS_RADIO_ON: DEVICE_RADIO_STATE = 0i32;
pub const DRS_SW_RADIO_OFF: DEVICE_RADIO_STATE = 1i32;
pub const DRS_HW_RADIO_OFF: DEVICE_RADIO_STATE = 2i32;
pub const DRS_SW_HW_RADIO_OFF: DEVICE_RADIO_STATE = 3i32;
pub const DRS_HW_RADIO_ON_UNCONTROLLABLE: DEVICE_RADIO_STATE = 4i32;
pub const DRS_RADIO_INVALID: DEVICE_RADIO_STATE = 5i32;
pub const DRS_HW_RADIO_OFF_UNCONTROLLABLE: DEVICE_RADIO_STATE = 6i32;
pub const DRS_RADIO_MAX: DEVICE_RADIO_STATE = 6i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_MTPBTH_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3927062522,
        data2: 22685,
        data3: 17522,
        data4: [132, 228, 10, 190, 54, 253, 98, 239],
    },
    pid: 2u32,
};
pub const DEVSVCTYPE_ABSTRACT: u32 = 1u32;
pub const DEVSVCTYPE_DEFAULT: u32 = 0u32;
pub const DEVSVC_SERVICEINFO_VERSION: u32 = 100u32;
pub const ENUM_AnchorResults_AnchorStateInvalid: u32 = 1u32;
pub const ENUM_AnchorResults_AnchorStateNormal: u32 = 0u32;
pub const ENUM_AnchorResults_AnchorStateOld: u32 = 2u32;
pub const ENUM_AnchorResults_ItemStateChanged: u32 = 4u32;
pub const ENUM_AnchorResults_ItemStateCreated: u32 = 2u32;
pub const ENUM_AnchorResults_ItemStateDeleted: u32 = 1u32;
pub const ENUM_AnchorResults_ItemStateInvalid: u32 = 0u32;
pub const ENUM_AnchorResults_ItemStateUpdated: u32 = 3u32;
pub const ENUM_CalendarObj_BusyStatusBusy: u32 = 1u32;
pub const ENUM_CalendarObj_BusyStatusFree: u32 = 0u32;
pub const ENUM_CalendarObj_BusyStatusOutOfOffice: u32 = 2u32;
pub const ENUM_CalendarObj_BusyStatusTentative: u32 = 3u32;
pub const ENUM_DeviceMetadataObj_DefaultCABFalse: u32 = 0u32;
pub const ENUM_DeviceMetadataObj_DefaultCABTrue: u32 = 1u32;
pub const ENUM_MessageObj_PatternInstanceFirst: u32 = 1u32;
pub const ENUM_MessageObj_PatternInstanceFourth: u32 = 4u32;
pub const ENUM_MessageObj_PatternInstanceLast: u32 = 5u32;
pub const ENUM_MessageObj_PatternInstanceNone: u32 = 0u32;
pub const ENUM_MessageObj_PatternInstanceSecond: u32 = 2u32;
pub const ENUM_MessageObj_PatternInstanceThird: u32 = 3u32;
pub const ENUM_MessageObj_PatternTypeDaily: u32 = 1u32;
pub const ENUM_MessageObj_PatternTypeMonthly: u32 = 3u32;
pub const ENUM_MessageObj_PatternTypeWeekly: u32 = 2u32;
pub const ENUM_MessageObj_PatternTypeYearly: u32 = 4u32;
pub const ENUM_MessageObj_PriorityHighest: u32 = 2u32;
pub const ENUM_MessageObj_PriorityLowest: u32 = 0u32;
pub const ENUM_MessageObj_PriorityNormal: u32 = 1u32;
pub const ENUM_MessageObj_ReadFalse: u32 = 0u32;
pub const ENUM_MessageObj_ReadTrue: u32 = 255u32;
pub const ENUM_StatusSvc_ChargingActive: u32 = 1u32;
pub const ENUM_StatusSvc_ChargingInactive: u32 = 0u32;
pub const ENUM_StatusSvc_ChargingUnknown: u32 = 2u32;
pub const ENUM_StatusSvc_RoamingActive: u32 = 1u32;
pub const ENUM_StatusSvc_RoamingInactive: u32 = 0u32;
pub const ENUM_StatusSvc_RoamingUnknown: u32 = 2u32;
pub const ENUM_SyncSvc_SyncObjectReferencesDisabled: u32 = 0u32;
pub const ENUM_SyncSvc_SyncObjectReferencesEnabled: u32 = 255u32;
pub const ENUM_TaskObj_CompleteFalse: u32 = 0u32;
pub const ENUM_TaskObj_CompleteTrue: u32 = 255u32;
pub const E_WPD_DEVICE_ALREADY_OPENED: ::windows_sys::core::HRESULT = -2144731135i32;
pub const E_WPD_DEVICE_IS_HUNG: ::windows_sys::core::HRESULT = -2144731130i32;
pub const E_WPD_DEVICE_NOT_OPEN: ::windows_sys::core::HRESULT = -2144731134i32;
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_DEVICE: ::windows_sys::core::HRESULT = -2144731133i32;
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_SERVICE: ::windows_sys::core::HRESULT = -2144730934i32;
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_DEVICE: ::windows_sys::core::HRESULT = -2144731132i32;
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_SERVICE: ::windows_sys::core::HRESULT = -2144730933i32;
pub const E_WPD_OBJECT_NOT_COMMITED: ::windows_sys::core::HRESULT = -2144731131i32;
pub const E_WPD_SERVICE_ALREADY_OPENED: ::windows_sys::core::HRESULT = -2144730936i32;
pub const E_WPD_SERVICE_BAD_PARAMETER_ORDER: ::windows_sys::core::HRESULT = -2144730932i32;
pub const E_WPD_SERVICE_NOT_OPEN: ::windows_sys::core::HRESULT = -2144730935i32;
pub const E_WPD_SMS_INVALID_MESSAGE_BODY: ::windows_sys::core::HRESULT = -2144731035i32;
pub const E_WPD_SMS_INVALID_RECIPIENT: ::windows_sys::core::HRESULT = -2144731036i32;
pub const E_WPD_SMS_SERVICE_UNAVAILABLE: ::windows_sys::core::HRESULT = -2144731034i32;
pub const EnumBthMtpConnectors: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2706833737, data2: 58949, data3: 20291, data4: [139, 13, 64, 155, 6, 29, 178, 252] };
pub const FACILITY_WPD: u32 = 42u32;
pub const FLAG_MessageObj_DayOfWeekFriday: u32 = 32u32;
pub const FLAG_MessageObj_DayOfWeekMonday: u32 = 2u32;
pub const FLAG_MessageObj_DayOfWeekNone: u32 = 0u32;
pub const FLAG_MessageObj_DayOfWeekSaturday: u32 = 64u32;
pub const FLAG_MessageObj_DayOfWeekSunday: u32 = 1u32;
pub const FLAG_MessageObj_DayOfWeekThursday: u32 = 16u32;
pub const FLAG_MessageObj_DayOfWeekTuesday: u32 = 4u32;
pub const FLAG_MessageObj_DayOfWeekWednesday: u32 = 8u32;
pub const GUID_DEVINTERFACE_WPD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1791129720, data2: 42746, data3: 16725, data4: [186, 133, 249, 143, 73, 29, 79, 51] };
pub const GUID_DEVINTERFACE_WPD_PRIVATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3121377679,
    data2: 19949,
    data3: 18871,
    data4: [189, 211, 250, 190, 40, 102, 18, 17],
};
pub const GUID_DEVINTERFACE_WPD_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2666811264,
    data2: 15716,
    data3: 16966,
    data4: [166, 170, 32, 111, 50, 141, 30, 220],
};
pub type IConnectionRequestCallback = *mut ::core::ffi::c_void;
pub type IEnumPortableDeviceConnectors = *mut ::core::ffi::c_void;
pub type IEnumPortableDeviceObjectIDs = *mut ::core::ffi::c_void;
pub type IMediaRadioManager = *mut ::core::ffi::c_void;
pub type IMediaRadioManagerNotifySink = *mut ::core::ffi::c_void;
pub const IOCTL_WPD_MESSAGE_READWRITE_ACCESS: u32 = 4243720u32;
pub const IOCTL_WPD_MESSAGE_READ_ACCESS: u32 = 4210952u32;
pub type IPortableDevice = *mut ::core::ffi::c_void;
pub type IPortableDeviceCapabilities = *mut ::core::ffi::c_void;
pub type IPortableDeviceConnector = *mut ::core::ffi::c_void;
pub type IPortableDeviceContent = *mut ::core::ffi::c_void;
pub type IPortableDeviceContent2 = *mut ::core::ffi::c_void;
pub type IPortableDeviceDataStream = *mut ::core::ffi::c_void;
pub type IPortableDeviceDispatchFactory = *mut ::core::ffi::c_void;
pub type IPortableDeviceEventCallback = *mut ::core::ffi::c_void;
pub type IPortableDeviceKeyCollection = *mut ::core::ffi::c_void;
pub type IPortableDeviceManager = *mut ::core::ffi::c_void;
pub type IPortableDevicePropVariantCollection = *mut ::core::ffi::c_void;
pub type IPortableDeviceProperties = *mut ::core::ffi::c_void;
pub type IPortableDevicePropertiesBulk = *mut ::core::ffi::c_void;
pub type IPortableDevicePropertiesBulkCallback = *mut ::core::ffi::c_void;
pub type IPortableDeviceResources = *mut ::core::ffi::c_void;
pub type IPortableDeviceService = *mut ::core::ffi::c_void;
pub type IPortableDeviceServiceActivation = *mut ::core::ffi::c_void;
pub type IPortableDeviceServiceCapabilities = *mut ::core::ffi::c_void;
pub type IPortableDeviceServiceManager = *mut ::core::ffi::c_void;
pub type IPortableDeviceServiceMethodCallback = *mut ::core::ffi::c_void;
pub type IPortableDeviceServiceMethods = *mut ::core::ffi::c_void;
pub type IPortableDeviceServiceOpenCallback = *mut ::core::ffi::c_void;
pub type IPortableDeviceUnitsStream = *mut ::core::ffi::c_void;
pub type IPortableDeviceValues = *mut ::core::ffi::c_void;
pub type IPortableDeviceValuesCollection = *mut ::core::ffi::c_void;
pub type IPortableDeviceWebControl = *mut ::core::ffi::c_void;
pub type IRadioInstance = *mut ::core::ffi::c_void;
pub type IRadioInstanceCollection = *mut ::core::ffi::c_void;
pub type IWpdSerializer = *mut ::core::ffi::c_void;
pub const PortableDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921655237, data2: 15774, data3: 18647, data4: [152, 16, 134, 72, 72, 240, 244, 4] };
pub const PortableDeviceDispatchFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1126375987, data2: 33592, data3: 18008, data4: [174, 1, 11, 74, 232, 48, 182, 176] };
pub const PortableDeviceFTM: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4156556186,
    data2: 18274,
    data3: 18570,
    data4: [180, 179, 118, 14, 249, 161, 186, 155],
};
pub const PortableDeviceKeyCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3727491629,
    data2: 9344,
    data3: 17342,
    data4: [151, 240, 209, 250, 44, 249, 143, 79],
};
pub const PortableDeviceManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 183569644, data2: 11981, data3: 19346, data4: [149, 129, 52, 246, 174, 6, 55, 243] };
pub const PortableDevicePropVariantCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 145333807,
    data2: 28013,
    data3: 19328,
    data4: [175, 90, 186, 242, 188, 190, 76, 185],
};
pub const PortableDeviceService: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4015895746,
    data2: 37650,
    data3: 16940,
    data4: [145, 82, 65, 28, 217, 196, 221, 132],
};
pub const PortableDeviceServiceFTM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 373928276, data2: 51092, data3: 18810, data4: [155, 3, 243, 240, 18, 19, 2, 243] };
pub const PortableDeviceValues: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 202757379, data2: 53271, data3: 18382, data4: [144, 22, 123, 63, 151, 135, 33, 204] };
pub const PortableDeviceValuesCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 948048717, data2: 5327, data3: 16928, data4: [156, 180, 67, 95, 134, 216, 63, 96] };
pub const PortableDeviceWebControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 409849900,
    data2: 11756,
    data3: 16821,
    data4: [167, 212, 181, 144, 86, 250, 222, 81],
};
pub const RANGEMAX_MessageObj_PatternDayOfMonth: u32 = 31u32;
pub const RANGEMAX_MessageObj_PatternMonthOfYear: u32 = 12u32;
pub const RANGEMAX_StatusSvc_BatteryLife: u32 = 100u32;
pub const RANGEMAX_StatusSvc_MissedCalls: u32 = 255u32;
pub const RANGEMAX_StatusSvc_NewPictures: u32 = 65535u32;
pub const RANGEMAX_StatusSvc_SignalStrength: u32 = 4u32;
pub const RANGEMAX_StatusSvc_TextMessages: u32 = 255u32;
pub const RANGEMAX_StatusSvc_VoiceMail: u32 = 255u32;
pub const RANGEMIN_MessageObj_PatternDayOfMonth: u32 = 1u32;
pub const RANGEMIN_MessageObj_PatternMonthOfYear: u32 = 1u32;
pub const RANGEMIN_StatusSvc_BatteryLife: u32 = 0u32;
pub const RANGEMIN_StatusSvc_SignalStrength: u32 = 0u32;
pub const RANGESTEP_MessageObj_PatternDayOfMonth: u32 = 1u32;
pub const RANGESTEP_MessageObj_PatternMonthOfYear: u32 = 1u32;
pub const RANGESTEP_StatusSvc_BatteryLife: u32 = 1u32;
pub const RANGESTEP_StatusSvc_SignalStrength: u32 = 1u32;
pub type SMS_MESSAGE_TYPES = i32;
pub const SMS_TEXT_MESSAGE: SMS_MESSAGE_TYPES = 0i32;
pub const SMS_BINARY_MESSAGE: SMS_MESSAGE_TYPES = 1i32;
pub const SYNCSVC_FILTER_CALENDAR_WINDOW_WITH_RECURRENCE: u32 = 3u32;
pub const SYNCSVC_FILTER_CONTACTS_WITH_PHONE: u32 = 1u32;
pub const SYNCSVC_FILTER_NONE: u32 = 0u32;
pub const SYNCSVC_FILTER_TASK_ACTIVE: u32 = 2u32;
pub type SYSTEM_RADIO_STATE = i32;
pub const SRS_RADIO_ENABLED: SYSTEM_RADIO_STATE = 0i32;
pub const SRS_RADIO_DISABLED: SYSTEM_RADIO_STATE = 1i32;
pub const TYPE_AnchorSyncSvc: u32 = 1u32;
pub const TYPE_CalendarSvc: u32 = 0u32;
pub const TYPE_ContactsSvc: u32 = 0u32;
pub const TYPE_DeviceMetadataSvc: u32 = 0u32;
pub const TYPE_FullEnumSyncSvc: u32 = 1u32;
pub const TYPE_HintsSvc: u32 = 0u32;
pub const TYPE_MessageSvc: u32 = 0u32;
pub const TYPE_NotesSvc: u32 = 0u32;
pub const TYPE_RingtonesSvc: u32 = 0u32;
pub const TYPE_StatusSvc: u32 = 0u32;
pub const TYPE_TasksSvc: u32 = 0u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 886510601,
        data2: 19271,
        data3: 19840,
        data4: [170, 172, 58, 40, 164, 163, 179, 230],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 886510601,
        data2: 19271,
        data3: 19840,
        data4: [170, 172, 58, 40, 164, 163, 179, 230],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 886510601,
        data2: 19271,
        data3: 19840,
        data4: [170, 172, 58, 40, 164, 163, 179, 230],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 886510601,
        data2: 19271,
        data3: 19840,
        data4: [170, 172, 58, 40, 164, 163, 179, 230],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 886510601,
        data2: 19271,
        data3: 19840,
        data4: [170, 172, 58, 40, 164, 163, 179, 230],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_OPTIMAL_READ_BLOCK_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 886510601,
        data2: 19271,
        data3: 19840,
        data4: [170, 172, 58, 40, 164, 163, 179, 230],
    },
    pid: 7u32,
};
pub const WPDNSE_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 886510601,
    data2: 19271,
    data3: 19840,
    data4: [170, 172, 58, 40, 164, 163, 179, 230],
};
pub const WPDNSE_PROPSHEET_CONTENT_DETAILS: u32 = 32u32;
pub const WPDNSE_PROPSHEET_CONTENT_GENERAL: u32 = 4u32;
pub const WPDNSE_PROPSHEET_CONTENT_REFERENCES: u32 = 8u32;
pub const WPDNSE_PROPSHEET_CONTENT_RESOURCES: u32 = 16u32;
pub const WPDNSE_PROPSHEET_DEVICE_GENERAL: u32 = 1u32;
pub const WPDNSE_PROPSHEET_STORAGE_GENERAL: u32 = 2u32;
pub const WPD_API_OPTIONS_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 283462206, data2: 1325, data3: 18295, data4: [161, 60, 222, 118, 20, 190, 43, 196] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_API_OPTION_IOCTL_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 283462206, data2: 1325, data3: 18295, data4: [161, 60, 222, 118, 20, 190, 43, 196] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_API_OPTION_USE_CLEAR_DATA_STREAM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 283462206, data2: 1325, data3: 18295, data4: [161, 60, 222, 118, 20, 190, 43, 196] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_ACCEPTED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4187946243,
        data2: 17181,
        data3: 16600,
        data4: [161, 201, 78, 34, 13, 156, 136, 211],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_DECLINED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4187946243,
        data2: 17181,
        data3: 16600,
        data4: [161, 201, 78, 34, 13, 156, 136, 211],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4187946243,
        data2: 17181,
        data3: 16600,
        data4: [161, 201, 78, 34, 13, 156, 136, 211],
    },
    pid: 3u32,
};
pub const WPD_APPOINTMENT_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4187946243,
    data2: 17181,
    data3: 16600,
    data4: [161, 201, 78, 34, 13, 156, 136, 211],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_OPTIONAL_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4187946243,
        data2: 17181,
        data3: 16600,
        data4: [161, 201, 78, 34, 13, 156, 136, 211],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_REQUIRED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4187946243,
        data2: 17181,
        data3: 16600,
        data4: [161, 201, 78, 34, 13, 156, 136, 211],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4187946243,
        data2: 17181,
        data3: 16600,
        data4: [161, 201, 78, 34, 13, 156, 136, 211],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_TENTATIVE_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4187946243,
        data2: 17181,
        data3: 16600,
        data4: [161, 201, 78, 34, 13, 156, 136, 211],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4187946243,
        data2: 17181,
        data3: 16600,
        data4: [161, 201, 78, 34, 13, 156, 136, 211],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3005543786,
        data2: 56413,
        data3: 18149,
        data4: [182, 223, 210, 234, 65, 72, 136, 198],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BIT_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3005543786,
        data2: 56413,
        data3: 18149,
        data4: [182, 223, 210, 234, 65, 72, 136, 198],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BLOCK_ALIGNMENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3005543786,
        data2: 56413,
        data3: 18149,
        data4: [182, 223, 210, 234, 65, 72, 136, 198],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_CHANNEL_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3005543786,
        data2: 56413,
        data3: 18149,
        data4: [182, 223, 210, 234, 65, 72, 136, 198],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_FORMAT_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3005543786,
        data2: 56413,
        data3: 18149,
        data4: [182, 223, 210, 234, 65, 72, 136, 198],
    },
    pid: 11u32,
};
pub type WPD_BITRATE_TYPES = i32;
pub const WPD_BITRATE_TYPE_UNUSED: WPD_BITRATE_TYPES = 0i32;
pub const WPD_BITRATE_TYPE_DISCRETE: WPD_BITRATE_TYPES = 1i32;
pub const WPD_BITRATE_TYPE_VARIABLE: WPD_BITRATE_TYPES = 2i32;
pub const WPD_BITRATE_TYPE_FREE: WPD_BITRATE_TYPES = 3i32;
pub type WPD_CAPTURE_MODES = i32;
pub const WPD_CAPTURE_MODE_UNDEFINED: WPD_CAPTURE_MODES = 0i32;
pub const WPD_CAPTURE_MODE_NORMAL: WPD_CAPTURE_MODES = 1i32;
pub const WPD_CAPTURE_MODE_BURST: WPD_CAPTURE_MODES = 2i32;
pub const WPD_CAPTURE_MODE_TIMELAPSE: WPD_CAPTURE_MODES = 3i32;
pub const WPD_CATEGORY_CAPABILITIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] };
pub const WPD_CATEGORY_COMMON: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4030868124,
    data2: 24008,
    data3: 17472,
    data4: [181, 189, 93, 242, 136, 53, 101, 138],
};
pub const WPD_CATEGORY_DEVICE_HINTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 224377131,
    data2: 52038,
    data3: 19535,
    data4: [131, 67, 11, 195, 211, 241, 124, 132],
};
pub const WPD_CATEGORY_MEDIA_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1504981946,
    data2: 65092,
    data3: 19853,
    data4: [128, 140, 107, 203, 155, 15, 21, 232],
};
pub const WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] };
pub const WPD_CATEGORY_NETWORK_CONFIGURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2029635324,
    data2: 31160,
    data3: 18236,
    data4: [144, 96, 107, 210, 61, 208, 114, 196],
};
pub const WPD_CATEGORY_NULL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
pub const WPD_CATEGORY_OBJECT_ENUMERATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] };
pub const WPD_CATEGORY_OBJECT_MANAGEMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4011738077,
    data2: 43501,
    data3: 17217,
    data4: [139, 204, 24, 97, 146, 174, 160, 137],
};
pub const WPD_CATEGORY_OBJECT_PROPERTIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] };
pub const WPD_CATEGORY_OBJECT_PROPERTIES_BULK: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 298329309,
    data2: 1229,
    data3: 20046,
    data4: [140, 123, 246, 239, 183, 148, 216, 78],
};
pub const WPD_CATEGORY_OBJECT_RESOURCES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] };
pub const WPD_CATEGORY_SERVICE_CAPABILITIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] };
pub const WPD_CATEGORY_SERVICE_COMMON: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 841942813,
    data2: 14063,
    data3: 18303,
    data4: [180, 181, 111, 82, 215, 52, 186, 238],
};
pub const WPD_CATEGORY_SERVICE_METHODS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] };
pub const WPD_CATEGORY_SMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2948750694,
    data2: 65037,
    data3: 16660,
    data4: [144, 151, 151, 12, 147, 233, 32, 209],
};
pub const WPD_CATEGORY_STILL_IMAGE_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1338861954,
    data2: 8866,
    data3: 19205,
    data4: [164, 139, 98, 211, 139, 242, 123, 50],
};
pub const WPD_CATEGORY_STORAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3640199078,
    data2: 13516,
    data3: 17914,
    data4: [151, 251, 208, 7, 250, 71, 236, 148],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DEVICE_IDENTIFICATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1043699162,
        data2: 19825,
        data3: 18942,
        data4: [160, 180, 212, 64, 108, 58, 233, 63],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DONT_REGISTER_WPD_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1661599727,
        data2: 43132,
        data3: 19623,
        data4: [132, 52, 121, 117, 118, 228, 10, 150],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_MULTITRANSPORT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1043699162,
        data2: 19825,
        data3: 18942,
        data4: [160, 180, 212, 64, 108, 58, 233, 63],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_REGISTER_WPD_PRIVATE_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1661599727,
        data2: 43132,
        data3: 19623,
        data4: [132, 52, 121, 117, 118, 228, 10, 150],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SILENCE_AUTOPLAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1707172088, data2: 4967, data3: 19682, data4: [147, 157, 131, 16, 131, 159, 13, 48] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1661599727,
        data2: 43132,
        data3: 19623,
        data4: [132, 52, 121, 117, 118, 228, 10, 150],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_TRANSPORT_BANDWIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1043699162,
        data2: 19825,
        data3: 18942,
        data4: [160, 180, 212, 64, 108, 58, 233, 63],
    },
    pid: 4u32,
};
pub const WPD_CLASS_EXTENSION_OPTIONS_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1661599727,
    data2: 43132,
    data3: 19623,
    data4: [132, 52, 121, 117, 118, 228, 10, 150],
};
pub const WPD_CLASS_EXTENSION_OPTIONS_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1043699162,
    data2: 19825,
    data3: 18942,
    data4: [160, 180, 212, 64, 108, 58, 233, 63],
};
pub const WPD_CLASS_EXTENSION_OPTIONS_V3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1707172088, data2: 4967, data3: 19682, data4: [147, 157, 131, 16, 131, 159, 13, 48] };
pub const WPD_CLASS_EXTENSION_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 872090897,
    data2: 25763,
    data3: 20396,
    data4: [180, 199, 61, 254, 170, 153, 176, 81],
};
pub const WPD_CLASS_EXTENSION_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2131196341,
    data2: 64043,
    data3: 18278,
    data4: [156, 178, 247, 59, 163, 11, 103, 88],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_DESIRED_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_EVENT_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 11u32,
};
pub const WPD_CLIENT_INFORMATION_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MAJOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MANUAL_CLOSE_ON_DISCONNECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MINIMUM_RESULTS_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MINOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_REVISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_SECURITY_QUALITY_OF_SERVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_SHARE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_WMDRM_APPLICATION_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_WMDRM_APPLICATION_PRIVATE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] },
    pid: 6u32,
};
pub type WPD_COLOR_CORRECTED_STATUS_VALUES = i32;
pub const WPD_COLOR_CORRECTED_STATUS_NOT_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = 0i32;
pub const WPD_COLOR_CORRECTED_STATUS_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = 1i32;
pub const WPD_COLOR_CORRECTED_STATUS_SHOULD_NOT_BE_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    pub Command: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
    pub AccessType: u32,
    pub AccessProperty: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WPD_COMMAND_ACCESS_TYPES = i32;
pub const WPD_COMMAND_ACCESS_READ: WPD_COMMAND_ACCESS_TYPES = 1i32;
pub const WPD_COMMAND_ACCESS_READWRITE: WPD_COMMAND_ACCESS_TYPES = 3i32;
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_STGM_ACCESS: WPD_COMMAND_ACCESS_TYPES = 4i32;
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_FILE_ACCESS: WPD_COMMAND_ACCESS_TYPES = 8i32;
pub const WPD_COMMAND_ACCESS_FROM_ATTRIBUTE_WITH_METHOD_ACCESS: WPD_COMMAND_ACCESS_TYPES = 16i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_FIXED_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_REGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2131196341,
        data2: 64043,
        data3: 18278,
        data4: [156, 178, 247, 59, 163, 11, 103, 88],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_UNREGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2131196341,
        data2: 64043,
        data3: 18278,
        data4: [156, 178, 247, 59, 163, 11, 103, 88],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_WRITE_DEVICE_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 872090897,
        data2: 25763,
        data3: 20396,
        data4: [180, 199, 61, 254, 170, 153, 176, 81],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMIT_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2029635324,
        data2: 31160,
        data3: 18236,
        data4: [144, 96, 107, 210, 61, 208, 114, 196],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_GET_OBJECT_IDS_FROM_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_RESET_DEVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_SAVE_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_DEVICE_HINTS_GET_CONTENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 224377131,
        data2: 52038,
        data3: 19535,
        data4: [131, 67, 11, 195, 211, 241, 124, 132],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_GENERATE_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2029635324,
        data2: 31160,
        data3: 18236,
        data4: [144, 96, 107, 210, 61, 208, 114, 196],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_PAUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1504981946,
        data2: 65092,
        data3: 19853,
        data4: [128, 140, 107, 203, 155, 15, 21, 232],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1504981946,
        data2: 65092,
        data3: 19853,
        data4: [128, 140, 107, 203, 155, 15, 21, 232],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_STOP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1504981946,
        data2: 65092,
        data3: 19853,
        data4: [128, 140, 107, 203, 155, 15, 21, 232],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_READ_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_WRITE_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_END_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_FIND_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_START_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COMMIT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COPY_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_ONLY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_DELETE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_MOVE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_REVERT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_UPDATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_WRITE_OBJECT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ALL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_SET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CLOSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_COMMIT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CREATE_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_OPEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_REVERT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK_IN_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_PROCESS_WIRELESS_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2029635324,
        data2: 31160,
        data3: 18236,
        data4: [144, 96, 107, 210, 61, 208, 114, 196],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS_BY_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_COMMON_GET_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 841942813,
        data2: 14063,
        data3: 18303,
        data4: [180, 181, 111, 82, 215, 52, 186, 238],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_CANCEL_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_END_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_START_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SMS_SEND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2948750694,
        data2: 65037,
        data3: 16660,
        data4: [144, 151, 151, 12, 147, 233, 32, 209],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STILL_IMAGE_CAPTURE_INITIATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1338861954,
        data2: 8866,
        data3: 19205,
        data4: [164, 139, 98, 211, 139, 242, 123, 50],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STORAGE_EJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3640199078,
        data2: 13516,
        data3: 17914,
        data4: [151, 251, 208, 7, 250, 71, 236, 148],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STORAGE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3640199078,
        data2: 13516,
        data3: 17914,
        data4: [151, 251, 208, 7, 250, 71, 236, 148],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_BODY_TEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_END_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_NOTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] },
    pid: 7u32,
};
pub const WPD_COMMON_INFORMATION_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_PRIORITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_START_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_SUBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ANNIVERSARY_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 62u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ASSISTANT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 61u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BIRTHDATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 57u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 45u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 40u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 41u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 50u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_CHILDREN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 60u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 54u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 51u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 52u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 53u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MIDDLE_NAMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MOBILE_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 42u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MOBILE_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 43u32,
};
pub const WPD_CONTACT_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4225039787,
    data2: 39037,
    data3: 18295,
    data4: [179, 249, 114, 97, 133, 169, 49, 43],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_EMAILS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_PHONES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 47u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PAGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 46u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 44u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 39u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 49u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 55u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PREFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_EMAIL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 58u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 48u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_RINGTONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 63u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ROLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 56u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_SPOUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 59u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_SUFFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4225039787,
        data2: 39037,
        data3: 18295,
        data4: [179, 249, 114, 97, 133, 169, 49, 43],
    },
    pid: 7u32,
};
pub const WPD_CONTENT_TYPE_ALL: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2162258130,
    data2: 4181,
    data3: 19006,
    data4: [185, 82, 130, 204, 79, 138, 134, 137],
};
pub const WPD_CONTENT_TYPE_APPOINTMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 267191822, data2: 34707, data3: 19230, data4: [144, 201, 72, 172, 56, 154, 198, 49] };
pub const WPD_CONTENT_TYPE_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1255327838,
    data2: 24109,
    data3: 17893,
    data4: [136, 100, 79, 34, 158, 60, 108, 240],
};
pub const WPD_CONTENT_TYPE_AUDIO_ALBUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2853729150,
    data2: 20489,
    data3: 18682,
    data4: [174, 33, 133, 242, 67, 131, 180, 230],
};
pub const WPD_CONTENT_TYPE_CALENDAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2717735271, data2: 24611, data3: 18848, data4: [157, 241, 248, 6, 11, 231, 81, 176] };
pub const WPD_CONTENT_TYPE_CERTIFICATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3694687976,
    data2: 43336,
    data3: 16480,
    data4: [144, 80, 203, 215, 126, 138, 61, 135],
};
pub const WPD_CONTENT_TYPE_CONTACT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3938091795,
    data2: 17701,
    data3: 18183,
    data4: [159, 14, 135, 198, 128, 142, 148, 53],
};
pub const WPD_CONTENT_TYPE_CONTACT_GROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 879462706, data2: 19510, data3: 16600, data4: [148, 21, 24, 40, 41, 31, 157, 233] };
pub const WPD_CONTENT_TYPE_DOCUMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1745542994,
    data2: 38154,
    data3: 16449,
    data4: [155, 65, 101, 227, 147, 100, 129, 85],
};
pub const WPD_CONTENT_TYPE_EMAIL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2151154762, data2: 32337, data3: 20367, data4: [136, 61, 29, 6, 35, 209, 69, 51] };
pub const WPD_CONTENT_TYPE_FOLDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 669180818, data2: 41233, data3: 18656, data4: [171, 12, 225, 119, 5, 160, 95, 133] };
pub const WPD_CONTENT_TYPE_FUNCTIONAL_OBJECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2582446432, data2: 6143, data3: 19524, data4: [157, 152, 29, 122, 111, 148, 25, 33] };
pub const WPD_CONTENT_TYPE_GENERIC_FILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 8773798, data2: 36148, data3: 17879, data4: [188, 92, 68, 126, 89, 199, 61, 72] };
pub const WPD_CONTENT_TYPE_GENERIC_MESSAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3893275384, data2: 45787, data3: 16691, data4: [182, 126, 27, 239, 75, 74, 110, 95] };
pub const WPD_CONTENT_TYPE_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4011919317, data2: 42282, data3: 16963, data4: [162, 107, 98, 212, 23, 109, 118, 3] };
pub const WPD_CONTENT_TYPE_IMAGE_ALBUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1970876744, data2: 5621, data3: 18992, data4: [168, 19, 84, 237, 138, 55, 226, 38] };
pub const WPD_CONTENT_TYPE_MEDIA_CAST: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1586017228,
    data2: 15973,
    data3: 20066,
    data4: [191, 255, 34, 148, 149, 37, 58, 176],
};
pub const WPD_CONTENT_TYPE_MEMO: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2631012047,
    data2: 15184,
    data3: 16719,
    data4: [166, 65, 228, 115, 255, 228, 87, 81],
};
pub const WPD_CONTENT_TYPE_MIXED_CONTENT_ALBUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15778732, data2: 42387, data3: 18860, data4: [146, 25, 36, 171, 202, 90, 37, 99] };
pub const WPD_CONTENT_TYPE_NETWORK_ASSOCIATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 52275182, data2: 6344, data3: 16901, data4: [132, 126, 137, 161, 18, 97, 208, 243] };
pub const WPD_CONTENT_TYPE_PLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 439613412, data2: 44819, data3: 18677, data4: [153, 78, 119, 54, 157, 254, 4, 163] };
pub const WPD_CONTENT_TYPE_PROGRAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3530160490,
    data2: 9340,
    data3: 19455,
    data4: [152, 251, 151, 243, 196, 146, 32, 230],
};
pub const WPD_CONTENT_TYPE_SECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2182121973, data2: 7569, data3: 19913, data4: [190, 60, 187, 177, 179, 91, 24, 206] };
pub const WPD_CONTENT_TYPE_TASK: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1663381292,
    data2: 34943,
    data3: 19638,
    data4: [177, 172, 210, 152, 85, 220, 239, 108],
};
pub const WPD_CONTENT_TYPE_TELEVISION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1621191119,
    data2: 62126,
    data3: 20001,
    data4: [147, 117, 150, 119, 241, 28, 28, 110],
};
pub const WPD_CONTENT_TYPE_UNSPECIFIED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 685298462, data2: 9372, data3: 17742, data4: [170, 188, 52, 136, 49, 104, 230, 52] };
pub const WPD_CONTENT_TYPE_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2455875644,
    data2: 15736,
    data3: 17689,
    data4: [133, 227, 2, 197, 225, 245, 11, 185],
};
pub const WPD_CONTENT_TYPE_VIDEO_ALBUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 19598775, data2: 54465, data3: 17878, data4: [176, 129, 148, 184, 119, 121, 97, 79] };
pub const WPD_CONTENT_TYPE_WIRELESS_PROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 195823370,
    data2: 40799,
    data3: 19876,
    data4: [168, 246, 61, 228, 77, 104, 253, 108],
};
pub const WPD_CONTROL_FUNCTION_GENERIC_MESSAGE: u32 = 66u32;
pub type WPD_CROPPED_STATUS_VALUES = i32;
pub const WPD_CROPPED_STATUS_NOT_CROPPED: WPD_CROPPED_STATUS_VALUES = 0i32;
pub const WPD_CROPPED_STATUS_CROPPED: WPD_CROPPED_STATUS_VALUES = 1i32;
pub const WPD_CROPPED_STATUS_SHOULD_NOT_BE_CROPPED: WPD_CROPPED_STATUS_VALUES = 2i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_EDP_IDENTITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1814792076,
        data2: 49900,
        data3: 18701,
        data4: [180, 37, 215, 167, 94, 35, 229, 237],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FIRMWARE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FUNCTIONAL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1178457698,
        data2: 32708,
        data3: 17041,
        data4: [145, 28, 127, 76, 156, 202, 151, 153],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MODEL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1178457698,
        data2: 32708,
        data3: 17041,
        data4: [145, 28, 127, 76, 156, 202, 151, 153],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_NETWORK_IDENTIFIER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_POWER_LEVEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_POWER_SOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 5u32,
};
pub const WPD_DEVICE_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 651466650,
    data2: 58947,
    data3: 17958,
    data4: [158, 43, 115, 109, 192, 201, 47, 220],
};
pub const WPD_DEVICE_PROPERTIES_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1178457698,
    data2: 32708,
    data3: 17041,
    data4: [145, 28, 127, 76, 156, 202, 151, 153],
};
pub const WPD_DEVICE_PROPERTIES_V3: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1814792076,
    data2: 49900,
    data3: 18701,
    data4: [180, 37, 215, 167, 94, 35, 229, 237],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_PROTOCOL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTED_DRM_SCHEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTED_FORMATS_ARE_ORDERED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTS_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SYNC_PARTNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_TRANSPORT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1178457698,
        data2: 32708,
        data3: 17041,
        data4: [145, 28, 127, 76, 156, 202, 151, 153],
    },
    pid: 4u32,
};
pub type WPD_DEVICE_TRANSPORTS = i32;
pub const WPD_DEVICE_TRANSPORT_UNSPECIFIED: WPD_DEVICE_TRANSPORTS = 0i32;
pub const WPD_DEVICE_TRANSPORT_USB: WPD_DEVICE_TRANSPORTS = 1i32;
pub const WPD_DEVICE_TRANSPORT_IP: WPD_DEVICE_TRANSPORTS = 2i32;
pub const WPD_DEVICE_TRANSPORT_BLUETOOTH: WPD_DEVICE_TRANSPORTS = 3i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 651466650,
        data2: 58947,
        data3: 17958,
        data4: [158, 43, 115, 109, 192, 201, 47, 220],
    },
    pid: 15u32,
};
pub type WPD_DEVICE_TYPES = i32;
pub const WPD_DEVICE_TYPE_GENERIC: WPD_DEVICE_TYPES = 0i32;
pub const WPD_DEVICE_TYPE_CAMERA: WPD_DEVICE_TYPES = 1i32;
pub const WPD_DEVICE_TYPE_MEDIA_PLAYER: WPD_DEVICE_TYPES = 2i32;
pub const WPD_DEVICE_TYPE_PHONE: WPD_DEVICE_TYPES = 3i32;
pub const WPD_DEVICE_TYPE_VIDEO: WPD_DEVICE_TYPES = 4i32;
pub const WPD_DEVICE_TYPE_PERSONAL_INFORMATION_MANAGER: WPD_DEVICE_TYPES = 5i32;
pub const WPD_DEVICE_TYPE_AUDIO_RECORDER: WPD_DEVICE_TYPES = 6i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_USE_DEVICE_STAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1178457698,
        data2: 32708,
        data3: 17041,
        data4: [145, 28, 127, 76, 156, 202, 151, 153],
    },
    pid: 5u32,
};
pub const WPD_DOCUMENT_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 185664003, data2: 60309, data3: 20226, data4: [147, 224, 151, 198, 49, 73, 58, 213] };
pub type WPD_EFFECT_MODES = i32;
pub const WPD_EFFECT_MODE_UNDEFINED: WPD_EFFECT_MODES = 0i32;
pub const WPD_EFFECT_MODE_COLOR: WPD_EFFECT_MODES = 1i32;
pub const WPD_EFFECT_MODE_BLACK_AND_WHITE: WPD_EFFECT_MODES = 2i32;
pub const WPD_EFFECT_MODE_SEPIA: WPD_EFFECT_MODES = 3i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_BCC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_CC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_HAS_ATTACHMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_HAS_BEEN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] },
    pid: 7u32,
};
pub const WPD_EMAIL_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_RECEIVED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_SENDER_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_TO_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] },
    pid: 2u32,
};
pub const WPD_EVENT_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 281634168,
    data2: 11905,
    data3: 16657,
    data4: [173, 222, 224, 140, 166, 19, 143, 109],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 281634168,
        data2: 11905,
        data3: 16657,
        data4: [173, 222, 224, 140, 166, 19, 143, 109],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 281634168,
        data2: 11905,
        data3: 16657,
        data4: [173, 222, 224, 140, 166, 19, 143, 109],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 281634168,
        data2: 11905,
        data3: 16657,
        data4: [173, 222, 224, 140, 166, 19, 143, 109],
    },
    pid: 3u32,
};
pub const WPD_EVENT_DEVICE_CAPABILITIES_UPDATED: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 914905761,
    data2: 52564,
    data3: 19882,
    data4: [179, 208, 175, 179, 224, 63, 89, 153],
};
pub const WPD_EVENT_DEVICE_REMOVED: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3838560795,
    data2: 26904,
    data3: 18617,
    data4: [133, 238, 2, 190, 124, 133, 10, 249],
};
pub const WPD_EVENT_DEVICE_RESET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2002112339, data2: 49645, data3: 17651, data4: [181, 162, 69, 30, 44, 55, 107, 39] };
pub const WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 22328, data3: 20466, data4: [132, 69, 190, 49, 38, 105, 16, 89] };
pub const WPD_EVENT_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 732095498, data2: 27468, data3: 17045, data4: [187, 67, 38, 50, 43, 153, 174, 178] };
pub const WPD_EVENT_OBJECT_ADDED: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2804341397,
    data2: 57863,
    data3: 19202,
    data4: [141, 68, 190, 242, 232, 108, 191, 252],
};
pub const WPD_EVENT_OBJECT_REMOVED: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3196234632,
    data2: 42284,
    data3: 18467,
    data4: [150, 229, 208, 39, 38, 113, 252, 56],
};
pub const WPD_EVENT_OBJECT_TRANSFER_REQUESTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2367070369, data2: 62150, data3: 16858, data4: [143, 25, 94, 83, 114, 26, 219, 242] };
pub const WPD_EVENT_OBJECT_UPDATED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 340109145, data2: 11777, data3: 18525, data4: [159, 39, 255, 7, 218, 230, 151, 171] };
pub const WPD_EVENT_OPTIONS_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3017333463, data2: 41825, data3: 19331, data4: [138, 72, 91, 2, 206, 16, 113, 59] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_OPTION_IS_AUTOPLAY_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3017333463, data2: 41825, data3: 19331, data4: [138, 72, 91, 2, 206, 16, 113, 59] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_OPTION_IS_BROADCAST_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3017333463, data2: 41825, data3: 19331, data4: [138, 72, 91, 2, 206, 16, 113, 59] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_CHILD_HIERARCHY_CHANGED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_EVENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OBJECT_CREATION_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OBJECT_PARENT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OPERATION_PROGRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OPERATION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_PNP_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1384151946,
        data2: 18708,
        data3: 17187,
        data4: [155, 154, 116, 246, 84, 178, 184, 70],
    },
    pid: 2u32,
};
pub const WPD_EVENT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] };
pub const WPD_EVENT_PROPERTIES_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1384151946,
    data2: 18708,
    data3: 17187,
    data4: [155, 154, 116, 246, 84, 178, 184, 70],
};
pub const WPD_EVENT_SERVICE_METHOD_COMPLETE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2318661112, data2: 2764, data3: 19867, data4: [156, 196, 17, 45, 53, 59, 134, 202] };
pub const WPD_EVENT_STORAGE_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 931291499, data2: 8892, data3: 17524, data4: [162, 81, 48, 112, 248, 211, 136, 87] };
pub type WPD_EXPOSURE_METERING_MODES = i32;
pub const WPD_EXPOSURE_METERING_MODE_UNDEFINED: WPD_EXPOSURE_METERING_MODES = 0i32;
pub const WPD_EXPOSURE_METERING_MODE_AVERAGE: WPD_EXPOSURE_METERING_MODES = 1i32;
pub const WPD_EXPOSURE_METERING_MODE_CENTER_WEIGHTED_AVERAGE: WPD_EXPOSURE_METERING_MODES = 2i32;
pub const WPD_EXPOSURE_METERING_MODE_MULTI_SPOT: WPD_EXPOSURE_METERING_MODES = 3i32;
pub const WPD_EXPOSURE_METERING_MODE_CENTER_SPOT: WPD_EXPOSURE_METERING_MODES = 4i32;
pub type WPD_EXPOSURE_PROGRAM_MODES = i32;
pub const WPD_EXPOSURE_PROGRAM_MODE_UNDEFINED: WPD_EXPOSURE_PROGRAM_MODES = 0i32;
pub const WPD_EXPOSURE_PROGRAM_MODE_MANUAL: WPD_EXPOSURE_PROGRAM_MODES = 1i32;
pub const WPD_EXPOSURE_PROGRAM_MODE_AUTO: WPD_EXPOSURE_PROGRAM_MODES = 2i32;
pub const WPD_EXPOSURE_PROGRAM_MODE_APERTURE_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = 3i32;
pub const WPD_EXPOSURE_PROGRAM_MODE_SHUTTER_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = 4i32;
pub const WPD_EXPOSURE_PROGRAM_MODE_CREATIVE: WPD_EXPOSURE_PROGRAM_MODES = 5i32;
pub const WPD_EXPOSURE_PROGRAM_MODE_ACTION: WPD_EXPOSURE_PROGRAM_MODES = 6i32;
pub const WPD_EXPOSURE_PROGRAM_MODE_PORTRAIT: WPD_EXPOSURE_PROGRAM_MODES = 7i32;
pub type WPD_FLASH_MODES = i32;
pub const WPD_FLASH_MODE_UNDEFINED: WPD_FLASH_MODES = 0i32;
pub const WPD_FLASH_MODE_AUTO: WPD_FLASH_MODES = 1i32;
pub const WPD_FLASH_MODE_OFF: WPD_FLASH_MODES = 2i32;
pub const WPD_FLASH_MODE_FILL: WPD_FLASH_MODES = 3i32;
pub const WPD_FLASH_MODE_RED_EYE_AUTO: WPD_FLASH_MODES = 4i32;
pub const WPD_FLASH_MODE_RED_EYE_FILL: WPD_FLASH_MODES = 5i32;
pub const WPD_FLASH_MODE_EXTERNAL_SYNC: WPD_FLASH_MODES = 6i32;
pub type WPD_FOCUS_METERING_MODES = i32;
pub const WPD_FOCUS_METERING_MODE_UNDEFINED: WPD_FOCUS_METERING_MODES = 0i32;
pub const WPD_FOCUS_METERING_MODE_CENTER_SPOT: WPD_FOCUS_METERING_MODES = 1i32;
pub const WPD_FOCUS_METERING_MODE_MULTI_SPOT: WPD_FOCUS_METERING_MODES = 2i32;
pub type WPD_FOCUS_MODES = i32;
pub const WPD_FOCUS_UNDEFINED: WPD_FOCUS_MODES = 0i32;
pub const WPD_FOCUS_MANUAL: WPD_FOCUS_MODES = 1i32;
pub const WPD_FOCUS_AUTOMATIC: WPD_FOCUS_MODES = 2i32;
pub const WPD_FOCUS_AUTOMATIC_MACRO: WPD_FOCUS_MODES = 3i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FOLDER_CONTENT_TYPES_ALLOWED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2124053183, data2: 58728, data3: 19252, data4: [170, 47, 19, 187, 18, 171, 23, 125] },
    pid: 2u32,
};
pub const WPD_FOLDER_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2124053183, data2: 58728, data3: 19252, data4: [170, 47, 19, 187, 18, 171, 23, 125] };
pub const WPD_FORMAT_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2694848512, data2: 48303, data3: 19432, data4: [179, 245, 35, 63, 35, 28, 245, 143] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FORMAT_ATTRIBUTE_MIMETYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2694848512, data2: 48303, data3: 19432, data4: [179, 245, 35, 63, 35, 28, 245, 143] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FORMAT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2694848512, data2: 48303, data3: 19432, data4: [179, 245, 35, 63, 35, 28, 245, 143] },
    pid: 2u32,
};
pub const WPD_FUNCTIONAL_CATEGORY_ALL: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 764044562,
    data2: 42828,
    data3: 17550,
    data4: [186, 138, 244, 172, 7, 196, 147, 153],
};
pub const WPD_FUNCTIONAL_CATEGORY_AUDIO_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1059723545,
    data2: 51138,
    data3: 18944,
    data4: [133, 93, 245, 124, 240, 109, 235, 187],
};
pub const WPD_FUNCTIONAL_CATEGORY_DEVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 149571179, data2: 58276, data3: 17206, data4: [161, 243, 164, 77, 43, 92, 67, 140] };
pub const WPD_FUNCTIONAL_CATEGORY_NETWORK_CONFIGURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1224006514, data2: 31850, data3: 19120, data4: [158, 26, 71, 14, 60, 219, 242, 106] };
pub const WPD_FUNCTIONAL_CATEGORY_RENDERING_INFORMATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 140512164, data2: 42938, data3: 18945, data4: [171, 14, 0, 101, 208, 163, 86, 211] };
pub const WPD_FUNCTIONAL_CATEGORY_SMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4497585, data2: 49641, data3: 19197, data4: [179, 88, 166, 44, 97, 23, 201, 207] };
pub const WPD_FUNCTIONAL_CATEGORY_STILL_IMAGE_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1631363879,
    data2: 43923,
    data3: 18688,
    data4: [180, 250, 137, 91, 181, 135, 75, 121],
};
pub const WPD_FUNCTIONAL_CATEGORY_STORAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 602954684, data2: 5598, data3: 19498, data4: [165, 91, 169, 175, 92, 228, 18, 239] };
pub const WPD_FUNCTIONAL_CATEGORY_VIDEO_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3795738475,
    data2: 29251,
    data3: 17322,
    data4: [141, 241, 14, 179, 217, 104, 169, 24],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FUNCTIONAL_OBJECT_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2399481235,
        data2: 43978,
        data3: 20421,
        data4: [165, 172, 176, 29, 244, 219, 229, 152],
    },
    pid: 2u32,
};
pub const WPD_FUNCTIONAL_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2399481235,
    data2: 43978,
    data3: 20421,
    data4: [165, 172, 176, 29, 244, 219, 229, 152],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_BITDEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1674987784,
        data2: 40865,
        data3: 18335,
        data4: [133, 186, 153, 82, 33, 100, 71, 219],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_COLOR_CORRECTED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1674987784,
        data2: 40865,
        data3: 18335,
        data4: [133, 186, 153, 82, 33, 100, 71, 219],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_CROPPED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1674987784,
        data2: 40865,
        data3: 18335,
        data4: [133, 186, 153, 82, 33, 100, 71, 219],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1674987784,
        data2: 40865,
        data3: 18335,
        data4: [133, 186, 153, 82, 33, 100, 71, 219],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1674987784,
        data2: 40865,
        data3: 18335,
        data4: [133, 186, 153, 82, 33, 100, 71, 219],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1674987784,
        data2: 40865,
        data3: 18335,
        data4: [133, 186, 153, 82, 33, 100, 71, 219],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_HORIZONTAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1674987784,
        data2: 40865,
        data3: 18335,
        data4: [133, 186, 153, 82, 33, 100, 71, 219],
    },
    pid: 9u32,
};
pub const WPD_IMAGE_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1674987784,
    data2: 40865,
    data3: 18335,
    data4: [133, 186, 153, 82, 33, 100, 71, 219],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_VERTICAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1674987784,
        data2: 40865,
        data3: 18335,
        data4: [133, 186, 153, 82, 33, 100, 71, 219],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ALBUM_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_AUDIO_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 49u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BITRATE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BUY_NOW: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BYTE_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_COMPOSER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_COPYRIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DESTINATION_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DURATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_LAST_ACCESSED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_LAST_BUILD_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_MANAGING_EDITOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_META_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_OBJECT_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_PARENTAL_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 9u32,
};
pub const WPD_MEDIA_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 785955333,
    data2: 2771,
    data3: 17116,
    data4: [176, 208, 188, 149, 172, 57, 106, 200],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_RELEASE_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SAMPLE_RATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SKIP_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SOURCE_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_STAR_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUBSCRIPTION_CONTENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUB_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 39u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUB_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TIME_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TIME_TO_LIVE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TOTAL_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_USER_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_USE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_WEBMASTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 785955333,
        data2: 2771,
        data3: 17116,
        data4: [176, 208, 188, 149, 172, 57, 106, 200],
    },
    pid: 22u32,
};
pub const WPD_MEMO_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1610349691, data2: 29827, data3: 16813, data4: [175, 185, 218, 63, 78, 89, 43, 141] };
pub type WPD_META_GENRES = i32;
pub const WPD_META_GENRE_UNUSED: WPD_META_GENRES = 0i32;
pub const WPD_META_GENRE_GENERIC_MUSIC_AUDIO_FILE: WPD_META_GENRES = 1i32;
pub const WPD_META_GENRE_GENERIC_NON_MUSIC_AUDIO_FILE: WPD_META_GENRES = 17i32;
pub const WPD_META_GENRE_SPOKEN_WORD_AUDIO_BOOK_FILES: WPD_META_GENRES = 18i32;
pub const WPD_META_GENRE_SPOKEN_WORD_FILES_NON_AUDIO_BOOK: WPD_META_GENRES = 19i32;
pub const WPD_META_GENRE_SPOKEN_WORD_NEWS: WPD_META_GENRES = 20i32;
pub const WPD_META_GENRE_SPOKEN_WORD_TALK_SHOWS: WPD_META_GENRES = 21i32;
pub const WPD_META_GENRE_GENERIC_VIDEO_FILE: WPD_META_GENRES = 33i32;
pub const WPD_META_GENRE_NEWS_VIDEO_FILE: WPD_META_GENRES = 34i32;
pub const WPD_META_GENRE_MUSIC_VIDEO_FILE: WPD_META_GENRES = 35i32;
pub const WPD_META_GENRE_HOME_VIDEO_FILE: WPD_META_GENRES = 36i32;
pub const WPD_META_GENRE_FEATURE_FILM_VIDEO_FILE: WPD_META_GENRES = 37i32;
pub const WPD_META_GENRE_TELEVISION_VIDEO_FILE: WPD_META_GENRES = 38i32;
pub const WPD_META_GENRE_TRAINING_EDUCATIONAL_VIDEO_FILE: WPD_META_GENRES = 39i32;
pub const WPD_META_GENRE_PHOTO_MONTAGE_VIDEO_FILE: WPD_META_GENRES = 40i32;
pub const WPD_META_GENRE_GENERIC_NON_AUDIO_NON_VIDEO: WPD_META_GENRES = 48i32;
pub const WPD_META_GENRE_AUDIO_PODCAST: WPD_META_GENRES = 64i32;
pub const WPD_META_GENRE_VIDEO_PODCAST: WPD_META_GENRES = 65i32;
pub const WPD_META_GENRE_MIXED_PODCAST: WPD_META_GENRES = 66i32;
pub const WPD_METHOD_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_ASSOCIATED_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_ALBUM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3005543786,
        data2: 56413,
        data3: 18149,
        data4: [182, 223, 210, 234, 65, 72, 136, 198],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_LYRICS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3005543786,
        data2: 56413,
        data3: 18149,
        data4: [182, 223, 210, 234, 65, 72, 136, 198],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_MOOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3005543786,
        data2: 56413,
        data3: 18149,
        data4: [182, 223, 210, 234, 65, 72, 136, 198],
    },
    pid: 8u32,
};
pub const WPD_MUSIC_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3005543786,
    data2: 56413,
    data3: 18149,
    data4: [182, 223, 210, 234, 65, 72, 136, 198],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_TRACK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3005543786,
        data2: 56413,
        data3: 18149,
        data4: [182, 223, 210, 234, 65, 72, 136, 198],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_NETWORK_ASSOCIATION_HOST_NETWORK_IDENTIFIERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3838393375, data2: 45571, data3: 17393, data4: [161, 0, 90, 7, 209, 27, 2, 116] },
    pid: 2u32,
};
pub const WPD_NETWORK_ASSOCIATION_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3838393375, data2: 45571, data3: 17393, data4: [161, 0, 90, 7, 209, 27, 2, 116] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_NETWORK_ASSOCIATION_X509V3SEQUENCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3838393375, data2: 45571, data3: 17393, data4: [161, 0, 90, 7, 209, 27, 2, 116] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_BACK_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CONTAINER_FUNCTIONAL_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_AUTHORED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_CREATED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_MODIFIED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 6u32,
};
pub const WPD_OBJECT_FORMAT_3G2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3112501248,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_3G2A: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 437329965, data2: 34649, data3: 20020, data4: [186, 94, 177, 33, 16, 135, 238, 228] };
pub const WPD_OBJECT_FORMAT_3GP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3112435712,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_3GPA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3843499824, data2: 63857, data3: 16879, data4: [161, 11, 34, 113, 160, 1, 157, 122] };
pub const WPD_OBJECT_FORMAT_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3103981568,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3145793536,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT_GROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3120955392,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_ABSTRACT_MEDIA_CAST: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3121283072,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_AIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805765120,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_ALL: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3254136498,
    data2: 19379,
    data3: 18332,
    data4: [156, 250, 5, 181, 243, 165, 123, 34],
};
pub const WPD_OBJECT_FORMAT_AMR: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3104309248,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_ASF: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 806092800,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_ASXPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3121807360,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_ATSCTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3112632320,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_AUDIBLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3104047104,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_AVCHD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3112566784,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_AVI: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805961728,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_BMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 939786240,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_CIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 939851776,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_DPOF: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805699584,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_DVBTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3112697856,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_EXECUTABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805502976,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_EXIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 939589632,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_FLAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3104178176,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_FLASHPIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 939720704,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_GIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 939982848,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_HTML: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805634048,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_ICALENDAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3187867648,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_ICON: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 124924653, data2: 4140, data3: 17976, data4: [156, 34, 131, 241, 66, 191, 200, 34] };
pub const WPD_OBJECT_FORMAT_JFIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 940048384,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_JP2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 940507136,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_JPEGXR: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3087269888,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_JPX: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 940572672,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_M3UPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3121676288,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_M4A: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 816555948, data2: 28669, data3: 19491, data4: [163, 89, 62, 155, 82, 243, 241, 200] };
pub const WPD_OBJECT_FORMAT_MHT_COMPILED_HTML: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3129212928,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MICROSOFT_EXCEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3129278464,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MICROSOFT_POWERPOINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3129344000,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MICROSOFT_WFC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2969829376,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MICROSOFT_WORD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3129147392,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MKV: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3113222144,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MP2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3112370176,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MP3: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805896192,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MP4: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3112304640,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MPEG: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 806027264,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_MPLPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3121741824,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_NETWORK_ASSOCIATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2969698304,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_OGG: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3103916032,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_PCD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 940113920,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_PICT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 940179456,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_PLSPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3121872896,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_PNG: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 940244992,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_PROPERTIES_ONLY: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805371904,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_QCELP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3104243712,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_SCRIPT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805437440,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_TEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805568512,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_TIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 940376064,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_TIFFEP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 939655168,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_TIFFIT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 940441600,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_UNSPECIFIED: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805306368,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_VCALENDAR1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3187802112,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_VCARD2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3145859072,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_VCARD3: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3145924608,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_WAVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 805830656,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_WBMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3087204352,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_WINDOWSIMAGEFORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3095461888,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_WMA: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3103850496,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_WMV: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3112239104,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_WPLPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3121610752,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_X509V3CERTIFICATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2969763840,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
pub const WPD_OBJECT_FORMAT_XML: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3129081856,
    data2: 44652,
    data3: 18436,
    data4: [152, 186, 197, 123, 70, 150, 95, 231],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_GENERATE_THUMBNAIL_FROM_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_HINT_LOCATION_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ISHIDDEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ISSYSTEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_IS_DRM_PROTECTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_KEYWORDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_LANGUAGE_LOCALE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ORIGINAL_FILE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 5u32,
};
pub const WPD_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4016785677,
    data2: 23768,
    data3: 17274,
    data4: [175, 252, 218, 139, 96, 238, 74, 60],
};
pub const WPD_OBJECT_PROPERTIES_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 57920829,
    data2: 19014,
    data3: 16599,
    data4: [180, 216, 115, 232, 218, 116, 231, 117],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SUPPORTED_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 57920829,
        data2: 19014,
        data3: 16599,
        data4: [180, 216, 115, 232, 218, 116, 231, 117],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SYNC_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4016785677,
        data2: 23768,
        data3: 17274,
        data4: [175, 252, 218, 139, 96, 238, 74, 60],
    },
    pid: 16u32,
};
pub type WPD_OPERATION_STATES = i32;
pub const WPD_OPERATION_STATE_UNSPECIFIED: WPD_OPERATION_STATES = 0i32;
pub const WPD_OPERATION_STATE_STARTED: WPD_OPERATION_STATES = 1i32;
pub const WPD_OPERATION_STATE_RUNNING: WPD_OPERATION_STATES = 2i32;
pub const WPD_OPERATION_STATE_PAUSED: WPD_OPERATION_STATES = 3i32;
pub const WPD_OPERATION_STATE_CANCELLED: WPD_OPERATION_STATES = 4i32;
pub const WPD_OPERATION_STATE_FINISHED: WPD_OPERATION_STATES = 5i32;
pub const WPD_OPERATION_STATE_ABORTED: WPD_OPERATION_STATES = 6i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_MANAGEMENT_RECURSIVE_DELETE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 5001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_NO_INPUT_BUFFER_ON_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 5003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_READ_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 5001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_WRITE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 5002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_SMS_BINARY_MESSAGE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2948750694,
        data2: 65037,
        data3: 16660,
        data4: [144, 151, 151, 12, 147, 233, 32, 209],
    },
    pid: 5001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_VALID_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 5001u32,
};
pub const WPD_PARAMETER_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3867561431,
    data2: 62245,
    data3: 17898,
    data4: [161, 213, 151, 207, 115, 182, 202, 88],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_ORDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_USAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3867561431,
        data2: 62245,
        data3: 17898,
        data4: [161, 213, 151, 207, 115, 182, 202, 88],
    },
    pid: 12u32,
};
pub type WPD_PARAMETER_USAGE_TYPES = i32;
pub const WPD_PARAMETER_USAGE_RETURN: WPD_PARAMETER_USAGE_TYPES = 0i32;
pub const WPD_PARAMETER_USAGE_IN: WPD_PARAMETER_USAGE_TYPES = 1i32;
pub const WPD_PARAMETER_USAGE_OUT: WPD_PARAMETER_USAGE_TYPES = 2i32;
pub const WPD_PARAMETER_USAGE_INOUT: WPD_PARAMETER_USAGE_TYPES = 3i32;
pub type WPD_POWER_SOURCES = i32;
pub const WPD_POWER_SOURCE_BATTERY: WPD_POWER_SOURCES = 0i32;
pub const WPD_POWER_SOURCE_EXTERNAL: WPD_POWER_SOURCES = 1i32;
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_DEVICE_PROPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1297371224,
    data2: 35072,
    data3: 16563,
    data4: [143, 29, 220, 36, 110, 30, 131, 112],
};
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_OBJECT_PROPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1297371224,
    data2: 20430,
    data3: 17784,
    data4: [149, 200, 134, 152, 169, 188, 15, 73],
};
pub const WPD_PROPERTY_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2876851160,
    data2: 25394,
    data3: 17503,
    data4: [160, 13, 141, 94, 241, 233, 111, 55],
};
pub const WPD_PROPERTY_ATTRIBUTES_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1570611552,
    data2: 29870,
    data3: 17356,
    data4: [133, 169, 254, 85, 90, 128, 121, 142],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_FAST_PROPERTY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1570611552,
        data2: 29870,
        data3: 17356,
        data4: [133, 169, 254, 85, 90, 128, 121, 142],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2876851160,
        data2: 25394,
        data3: 17503,
        data4: [160, 13, 141, 94, 241, 233, 111, 55],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1570611552,
        data2: 29870,
        data3: 17356,
        data4: [133, 169, 254, 85, 90, 128, 121, 142],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] },
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 872090897,
        data2: 25763,
        data3: 20396,
        data4: [180, 199, 61, 254, 170, 153, 176, 81],
    },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 872090897,
        data2: 25763,
        data3: 20396,
        data4: [180, 199, 61, 254, 170, 153, 176, 81],
    },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2131196341,
        data2: 64043,
        data3: 18278,
        data4: [156, 178, 247, 59, 163, 11, 103, 88],
    },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2131196341,
        data2: 64043,
        data3: 18278,
        data4: [156, 178, 247, 59, 163, 11, 103, 88],
    },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_REGISTRATION_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2131196341,
        data2: 64043,
        data3: 18278,
        data4: [156, 178, 247, 59, 163, 11, 103, 88],
    },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_ACTIVITY_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_TARGET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_DRIVER_ERROR_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4030868124,
        data2: 24008,
        data3: 17472,
        data4: [181, 189, 93, 242, 136, 53, 101, 138],
    },
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_LOCATIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 224377131,
        data2: 52038,
        data3: 19535,
        data4: [131, 67, 11, 195, 211, 241, 124, 132],
    },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 224377131,
        data2: 52038,
        data3: 19535,
        data4: [131, 67, 11, 195, 211, 241, 124, 132],
    },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_EVENT_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1297371224,
        data2: 61320,
        data3: 20045,
        data4: [149, 195, 79, 50, 127, 114, 138, 150],
    },
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] },
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_NULL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_FILTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_NUM_OBJECTS_REQUESTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] },
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_COPY_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CREATION_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DESTINATION_FOLDER_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_MOVE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_UPDATE_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4011738077,
        data2: 43501,
        data3: 17217,
        data4: [139, 204, 24, 97, 146, 174, 160, 137],
    },
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PARENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 298329309,
        data2: 1229,
        data3: 20046,
        data4: [140, 123, 246, 239, 183, 148, 216, 78],
    },
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] },
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_ACCESS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_POSITION_FROM_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_ORIGIN_FLAG: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_STREAM_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SUPPORTS_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] },
    pid: 1015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_PUBLIC_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2029635324,
        data2: 31160,
        data3: 18236,
        data4: [144, 96, 107, 210, 61, 208, 114, 196],
    },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1018u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1019u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITANCE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1017u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] },
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_PARAMETER_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_RESULT_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 841942813,
        data2: 14063,
        data3: 18303,
        data4: [180, 181, 111, 82, 215, 52, 186, 238],
    },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_BINARY_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2948750694,
        data2: 65037,
        data3: 16660,
        data4: [144, 151, 151, 12, 147, 233, 32, 209],
    },
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_MESSAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2948750694,
        data2: 65037,
        data3: 16660,
        data4: [144, 151, 151, 12, 147, 233, 32, 209],
    },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_RECIPIENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2948750694,
        data2: 65037,
        data3: 16660,
        data4: [144, 151, 151, 12, 147, 233, 32, 209],
    },
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_TEXT_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 2948750694,
        data2: 65037,
        data3: 16660,
        data4: [144, 151, 151, 12, 147, 233, 32, 209],
    },
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_STORAGE_DESTINATION_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3640199078,
        data2: 13516,
        data3: 17914,
        data4: [151, 251, 208, 7, 250, 71, 236, 148],
    },
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_STORAGE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3640199078,
        data2: 13516,
        data3: 17914,
        data4: [151, 251, 208, 7, 250, 71, 236, 148],
    },
    pid: 1001u32,
};
pub const WPD_RENDERING_INFORMATION_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3309110175,
    data2: 60963,
    data3: 18993,
    data4: [133, 144, 118, 57, 135, 152, 112, 180],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3309110175,
        data2: 60963,
        data3: 18993,
        data4: [133, 144, 118, 57, 135, 152, 112, 180],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_CREATABLE_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3309110175,
        data2: 60963,
        data3: 18993,
        data4: [133, 144, 118, 57, 135, 152, 112, 180],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3309110175,
        data2: 60963,
        data3: 18993,
        data4: [133, 144, 118, 57, 135, 152, 112, 180],
    },
    pid: 3u32,
};
pub type WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = i32;
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_OBJECT: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = 0i32;
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_RESOURCE: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = 1i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 4029326164, data2: 8960, data3: 20013, data4: [161, 185, 59, 103, 48, 247, 250, 33] },
    pid: 0u32,
};
pub const WPD_RESOURCE_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 515307012,
    data2: 37496,
    data3: 17055,
    data4: [147, 204, 91, 184, 192, 102, 86, 182],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 515307012,
        data2: 37496,
        data3: 17055,
        data4: [147, 204, 91, 184, 192, 102, 86, 182],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 515307012,
        data2: 37496,
        data3: 17055,
        data4: [147, 204, 91, 184, 192, 102, 86, 182],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 515307012,
        data2: 37496,
        data3: 17055,
        data4: [147, 204, 91, 184, 192, 102, 86, 182],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 515307012,
        data2: 37496,
        data3: 17055,
        data4: [147, 204, 91, 184, 192, 102, 86, 182],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_READ_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 515307012,
        data2: 37496,
        data3: 17055,
        data4: [147, 204, 91, 184, 192, 102, 86, 182],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_WRITE_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 515307012,
        data2: 37496,
        data3: 17055,
        data4: [147, 204, 91, 184, 192, 102, 86, 182],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_RESOURCE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 515307012,
        data2: 37496,
        data3: 17055,
        data4: [147, 204, 91, 184, 192, 102, 86, 182],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_TOTAL_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 515307012,
        data2: 37496,
        data3: 17055,
        data4: [147, 204, 91, 184, 192, 102, 86, 182],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1002518914,
        data2: 34225,
        data3: 18656,
        data4: [149, 166, 141, 58, 208, 107, 225, 23],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_BRANDING_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3056841134,
        data2: 27823,
        data3: 19079,
        data4: [149, 137, 34, 222, 214, 221, 88, 153],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 743270403,
        data2: 33002,
        data3: 17792,
        data4: [175, 154, 91, 225, 162, 62, 221, 203],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_DEFAULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3894311358,
        data2: 13552,
        data3: 16831,
        data4: [181, 63, 241, 160, 106, 232, 120, 66],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_GENERIC: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3115971861, data2: 47728, data3: 17991, data4: [148, 220, 250, 73, 37, 233, 90, 7] },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 4053139160,
        data2: 43560,
        data3: 20195,
        data4: [177, 83, 225, 130, 221, 94, 220, 57],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3351513018,
        data2: 39162,
        data3: 18101,
        data4: [153, 96, 35, 254, 193, 36, 207, 222],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_VIDEO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3043421762, data2: 25448, data3: 17040, data4: [134, 98, 112, 24, 47, 183, 159, 32] },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1365966123,
        data2: 50766,
        data3: 17648,
        data4: [152, 220, 190, 225, 200, 143, 125, 102],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1365966123,
        data2: 50766,
        data3: 17648,
        data4: [152, 220, 190, 225, 200, 143, 125, 102],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_REFERENCED_OBJECT_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1365966123,
        data2: 50766,
        data3: 17648,
        data4: [152, 220, 190, 225, 200, 143, 125, 102],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 1365966123,
        data2: 50766,
        data3: 17648,
        data4: [152, 220, 190, 225, 200, 143, 125, 102],
    },
    pid: 4u32,
};
pub type WPD_SECTION_DATA_UNITS_VALUES = i32;
pub const WPD_SECTION_DATA_UNITS_BYTES: WPD_SECTION_DATA_UNITS_VALUES = 0i32;
pub const WPD_SECTION_DATA_UNITS_MILLISECONDS: WPD_SECTION_DATA_UNITS_VALUES = 1i32;
pub const WPD_SECTION_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1365966123,
    data2: 50766,
    data3: 17648,
    data4: [152, 220, 190, 225, 200, 143, 125, 102],
};
pub type WPD_SERVICE_INHERITANCE_TYPES = i32;
pub const WPD_SERVICE_INHERITANCE_IMPLEMENTATION: WPD_SERVICE_INHERITANCE_TYPES = 0i32;
pub const WPD_SERVICE_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1964009866, data2: 52052, data3: 18460, data4: [184, 219, 13, 117, 201, 63, 28, 6] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SERVICE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1964009866, data2: 52052, data3: 18460, data4: [184, 219, 13, 117, 201, 63, 28, 6] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_ENCODING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] },
    pid: 5u32,
};
pub type WPD_SMS_ENCODING_TYPES = i32;
pub const SMS_ENCODING_7_BIT: WPD_SMS_ENCODING_TYPES = 0i32;
pub const SMS_ENCODING_8_BIT: WPD_SMS_ENCODING_TYPES = 1i32;
pub const SMS_ENCODING_UTF_16: WPD_SMS_ENCODING_TYPES = 2i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_MAX_PAYLOAD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] },
    pid: 4u32,
};
pub const WPD_SMS_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_PROVIDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_TIMEOUT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_BURST_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_BURST_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAMERA_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAMERA_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_DELAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 18u32,
};
pub const WPD_STILL_IMAGE_CAPTURE_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_COMPRESSION_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CONTRAST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_DIGITAL_ZOOM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EFFECT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_BIAS_COMPENSATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_PROGRAM_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FLASH_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCAL_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_RGB_GAIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_SHARPNESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_TIMELAPSE_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_TIMELAPSE_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_UPLOAD_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_WHITE_BALANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_ACCESS_CAPABILITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 11u32,
};
pub type WPD_STORAGE_ACCESS_CAPABILITY_VALUES = i32;
pub const WPD_STORAGE_ACCESS_CAPABILITY_READWRITE: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = 0i32;
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITHOUT_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = 1i32;
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITH_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = 2i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_CAPACITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_CAPACITY_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FILE_SYSTEM_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FREE_SPACE_IN_BYTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FREE_SPACE_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_MAX_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 9u32,
};
pub const WPD_STORAGE_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] },
    pid: 2u32,
};
pub type WPD_STORAGE_TYPE_VALUES = i32;
pub const WPD_STORAGE_TYPE_UNDEFINED: WPD_STORAGE_TYPE_VALUES = 0i32;
pub const WPD_STORAGE_TYPE_FIXED_ROM: WPD_STORAGE_TYPE_VALUES = 1i32;
pub const WPD_STORAGE_TYPE_REMOVABLE_ROM: WPD_STORAGE_TYPE_VALUES = 2i32;
pub const WPD_STORAGE_TYPE_FIXED_RAM: WPD_STORAGE_TYPE_VALUES = 3i32;
pub const WPD_STORAGE_TYPE_REMOVABLE_RAM: WPD_STORAGE_TYPE_VALUES = 4i32;
pub type WPD_STREAM_UNITS = i32;
pub const WPD_STREAM_UNITS_BYTES: WPD_STREAM_UNITS = 0i32;
pub const WPD_STREAM_UNITS_FRAMES: WPD_STREAM_UNITS = 1i32;
pub const WPD_STREAM_UNITS_ROWS: WPD_STREAM_UNITS = 2i32;
pub const WPD_STREAM_UNITS_MILLISECONDS: WPD_STREAM_UNITS = 4i32;
pub const WPD_STREAM_UNITS_MICROSECONDS: WPD_STREAM_UNITS = 8i32;
pub const WPD_TASK_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3813992798,
    data2: 55456,
    data3: 17975,
    data4: [160, 58, 12, 178, 104, 56, 219, 199],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3813992798,
        data2: 55456,
        data3: 17975,
        data4: [160, 58, 12, 178, 104, 56, 219, 199],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_PERCENT_COMPLETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3813992798,
        data2: 55456,
        data3: 17975,
        data4: [160, 58, 12, 178, 104, 56, 219, 199],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_REMINDER_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3813992798,
        data2: 55456,
        data3: 17975,
        data4: [160, 58, 12, 178, 104, 56, 219, 199],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID {
        data1: 3813992798,
        data2: 55456,
        data3: 17975,
        data4: [160, 58, 12, 178, 104, 56, 219, 199],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_AUTHOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_CREDITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_FOURCC_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_FRAMERATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_KEY_FRAME_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 10u32,
};
pub const WPD_VIDEO_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_QUALITY_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_CHANNEL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_REPEAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_STATION_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_SCAN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] },
    pid: 12u32,
};
pub type WPD_VIDEO_SCAN_TYPES = i32;
pub const WPD_VIDEO_SCAN_TYPE_UNUSED: WPD_VIDEO_SCAN_TYPES = 0i32;
pub const WPD_VIDEO_SCAN_TYPE_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = 1i32;
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = 2i32;
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = 3i32;
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = 4i32;
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = 5i32;
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE: WPD_VIDEO_SCAN_TYPES = 6i32;
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE_AND_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = 7i32;
pub type WPD_WHITE_BALANCE_SETTINGS = i32;
pub const WPD_WHITE_BALANCE_UNDEFINED: WPD_WHITE_BALANCE_SETTINGS = 0i32;
pub const WPD_WHITE_BALANCE_MANUAL: WPD_WHITE_BALANCE_SETTINGS = 1i32;
pub const WPD_WHITE_BALANCE_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = 2i32;
pub const WPD_WHITE_BALANCE_ONE_PUSH_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = 3i32;
pub const WPD_WHITE_BALANCE_DAYLIGHT: WPD_WHITE_BALANCE_SETTINGS = 4i32;
pub const WPD_WHITE_BALANCE_FLORESCENT: WPD_WHITE_BALANCE_SETTINGS = 5i32;
pub const WPD_WHITE_BALANCE_TUNGSTEN: WPD_WHITE_BALANCE_SETTINGS = 6i32;
pub const WPD_WHITE_BALANCE_FLASH: WPD_WHITE_BALANCE_SETTINGS = 7i32;
pub type WpdAttributeForm = i32;
pub const WPD_PROPERTY_ATTRIBUTE_FORM_UNSPECIFIED: WpdAttributeForm = 0i32;
pub const WPD_PROPERTY_ATTRIBUTE_FORM_RANGE: WpdAttributeForm = 1i32;
pub const WPD_PROPERTY_ATTRIBUTE_FORM_ENUMERATION: WpdAttributeForm = 2i32;
pub const WPD_PROPERTY_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdAttributeForm = 3i32;
pub const WPD_PROPERTY_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdAttributeForm = 4i32;
pub type WpdParameterAttributeForm = i32;
pub const WPD_PARAMETER_ATTRIBUTE_FORM_UNSPECIFIED: WpdParameterAttributeForm = 0i32;
pub const WPD_PARAMETER_ATTRIBUTE_FORM_RANGE: WpdParameterAttributeForm = 1i32;
pub const WPD_PARAMETER_ATTRIBUTE_FORM_ENUMERATION: WpdParameterAttributeForm = 2i32;
pub const WPD_PARAMETER_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdParameterAttributeForm = 3i32;
pub const WPD_PARAMETER_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdParameterAttributeForm = 4i32;
pub const WpdSerializer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 194094923, data2: 44412, data3: 19101, data4: [181, 99, 41, 238, 249, 22, 113, 114] };
