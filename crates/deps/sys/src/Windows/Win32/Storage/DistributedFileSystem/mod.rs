#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsAdd = unsafe extern "system" fn(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, comment: super::super::Foundation::PWSTR, flags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsAddFtRoot = unsafe extern "system" fn(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, ftdfsname: super::super::Foundation::PWSTR, comment: super::super::Foundation::PWSTR, flags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsAddRootTarget = unsafe extern "system" fn(pdfspath: super::super::Foundation::PWSTR, ptargetpath: super::super::Foundation::PWSTR, majorversion: u32, pcomment: super::super::Foundation::PWSTR, flags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsAddStdRoot = unsafe extern "system" fn(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, comment: super::super::Foundation::PWSTR, flags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsEnum = unsafe extern "system" fn(dfsname: super::super::Foundation::PWSTR, level: u32, prefmaxlen: u32, buffer: *mut *mut u8, entriesread: *mut u32, resumehandle: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsGetClientInfo = unsafe extern "system" fn(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *mut *mut u8) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type NetDfsGetFtContainerSecurity = unsafe extern "system" fn(domainname: super::super::Foundation::PWSTR, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsGetInfo = unsafe extern "system" fn(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *mut *mut u8) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type NetDfsGetSecurity = unsafe extern "system" fn(dfsentrypath: super::super::Foundation::PWSTR, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type NetDfsGetStdContainerSecurity = unsafe extern "system" fn(machinename: super::super::Foundation::PWSTR, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsGetSupportedNamespaceVersion = unsafe extern "system" fn(origin: DFS_NAMESPACE_VERSION_ORIGIN, pname: super::super::Foundation::PWSTR, ppversioninfo: *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsMove = unsafe extern "system" fn(olddfsentrypath: super::super::Foundation::PWSTR, newdfsentrypath: super::super::Foundation::PWSTR, flags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsRemove = unsafe extern "system" fn(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsRemoveFtRoot = unsafe extern "system" fn(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, ftdfsname: super::super::Foundation::PWSTR, flags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsRemoveFtRootForced = unsafe extern "system" fn(domainname: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, ftdfsname: super::super::Foundation::PWSTR, flags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsRemoveRootTarget = unsafe extern "system" fn(pdfspath: super::super::Foundation::PWSTR, ptargetpath: super::super::Foundation::PWSTR, flags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsRemoveStdRoot = unsafe extern "system" fn(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, flags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsSetClientInfo = unsafe extern "system" fn(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *const u8) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type NetDfsSetFtContainerSecurity = unsafe extern "system" fn(domainname: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type NetDfsSetInfo = unsafe extern "system" fn(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *const u8) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type NetDfsSetSecurity = unsafe extern "system" fn(dfsentrypath: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type NetDfsSetStdContainerSecurity = unsafe extern "system" fn(machinename: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
pub const DFS_ADD_VOLUME: u32 = 1u32;
pub const DFS_FORCE_REMOVE: u32 = 2147483648u32;
#[repr(C)]
pub struct DFS_GET_PKT_ENTRY_STATE_ARG {
    pub DfsEntryPathLen: u16,
    pub ServerNameLen: u16,
    pub ShareNameLen: u16,
    pub Level: u32,
    pub Buffer: [u16; 1],
}
impl ::core::marker::Copy for DFS_GET_PKT_ENTRY_STATE_ARG {}
impl ::core::clone::Clone for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_1 {
    pub EntryPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_100 {
    pub Comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_100 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_100 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DFS_INFO_101 {
    pub State: u32,
}
impl ::core::marker::Copy for DFS_INFO_101 {}
impl ::core::clone::Clone for DFS_INFO_101 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DFS_INFO_102 {
    pub Timeout: u32,
}
impl ::core::marker::Copy for DFS_INFO_102 {}
impl ::core::clone::Clone for DFS_INFO_102 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DFS_INFO_103 {
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
impl ::core::marker::Copy for DFS_INFO_103 {}
impl ::core::clone::Clone for DFS_INFO_103 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DFS_INFO_104 {
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl ::core::marker::Copy for DFS_INFO_104 {}
impl ::core::clone::Clone for DFS_INFO_104 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_105 {
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_105 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_105 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DFS_INFO_106 {
    pub State: u32,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl ::core::marker::Copy for DFS_INFO_106 {}
impl ::core::clone::Clone for DFS_INFO_106 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct DFS_INFO_107 {
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for DFS_INFO_107 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for DFS_INFO_107 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct DFS_INFO_150 {
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for DFS_INFO_150 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for DFS_INFO_150 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct DFS_INFO_1_32 {
    pub EntryPath: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for DFS_INFO_1_32 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for DFS_INFO_1_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_2 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_200 {
    pub FtDfsName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_200 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_200 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct DFS_INFO_2_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for DFS_INFO_2_32 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for DFS_INFO_2_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_3 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_300 {
    pub Flags: u32,
    pub DfsName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_300 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_300 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct DFS_INFO_3_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for DFS_INFO_3_32 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for DFS_INFO_3_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_4 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows_sys::core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct DFS_INFO_4_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows_sys::core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for DFS_INFO_4_32 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for DFS_INFO_4_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_5 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows_sys::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DFS_INFO_50 {
    pub NamespaceMajorVersion: u32,
    pub NamespaceMinorVersion: u32,
    pub NamespaceCapabilities: u64,
}
impl ::core::marker::Copy for DFS_INFO_50 {}
impl ::core::clone::Clone for DFS_INFO_50 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_6 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows_sys::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DFS_INFO_7 {
    pub GenerationGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DFS_INFO_7 {}
impl ::core::clone::Clone for DFS_INFO_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct DFS_INFO_8 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows_sys::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for DFS_INFO_8 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for DFS_INFO_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct DFS_INFO_9 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows_sys::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for DFS_INFO_9 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for DFS_INFO_9 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DFS_MOVE_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
pub type DFS_NAMESPACE_VERSION_ORIGIN = i32;
pub const DFS_NAMESPACE_VERSION_ORIGIN_COMBINED: DFS_NAMESPACE_VERSION_ORIGIN = 0i32;
pub const DFS_NAMESPACE_VERSION_ORIGIN_SERVER: DFS_NAMESPACE_VERSION_ORIGIN = 1i32;
pub const DFS_NAMESPACE_VERSION_ORIGIN_DOMAIN: DFS_NAMESPACE_VERSION_ORIGIN = 2i32;
pub const DFS_PROPERTY_FLAG_ABDE: u32 = 32u32;
pub const DFS_PROPERTY_FLAG_CLUSTER_ENABLED: u32 = 16u32;
pub const DFS_PROPERTY_FLAG_INSITE_REFERRALS: u32 = 1u32;
pub const DFS_PROPERTY_FLAG_ROOT_SCALABILITY: u32 = 2u32;
pub const DFS_PROPERTY_FLAG_SITE_COSTING: u32 = 4u32;
pub const DFS_PROPERTY_FLAG_TARGET_FAILBACK: u32 = 8u32;
pub const DFS_RESTORE_VOLUME: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_SITELIST_INFO {
    pub cSites: u32,
    pub Site: [DFS_SITENAME_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_SITELIST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_SITELIST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_SITENAME_INFO {
    pub SiteFlags: u32,
    pub SiteName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_SITENAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_SITENAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DFS_SITE_PRIMARY: u32 = 1u32;
pub const DFS_STORAGE_FLAVOR_UNUSED2: u32 = 768u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_STORAGE_INFO {
    pub State: u32,
    pub ServerName: super::super::Foundation::PWSTR,
    pub ShareName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_STORAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_STORAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct DFS_STORAGE_INFO_0_32 {
    pub State: u32,
    pub ServerName: u32,
    pub ShareName: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for DFS_STORAGE_INFO_0_32 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for DFS_STORAGE_INFO_0_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_STORAGE_INFO_1 {
    pub State: u32,
    pub ServerName: super::super::Foundation::PWSTR,
    pub ShareName: super::super::Foundation::PWSTR,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_STORAGE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_STORAGE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DFS_STORAGE_STATES: u32 = 15u32;
pub const DFS_STORAGE_STATE_ACTIVE: u32 = 4u32;
pub const DFS_STORAGE_STATE_OFFLINE: u32 = 1u32;
pub const DFS_STORAGE_STATE_ONLINE: u32 = 2u32;
#[repr(C)]
pub struct DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    pub DomainDfsMajorVersion: u32,
    pub DomainDfsMinorVersion: u32,
    pub DomainDfsCapabilities: u64,
    pub StandaloneDfsMajorVersion: u32,
    pub StandaloneDfsMinorVersion: u32,
    pub StandaloneDfsCapabilities: u64,
}
impl ::core::marker::Copy for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {}
impl ::core::clone::Clone for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DFS_TARGET_PRIORITY {
    pub TargetPriorityClass: DFS_TARGET_PRIORITY_CLASS,
    pub TargetPriorityRank: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for DFS_TARGET_PRIORITY {}
impl ::core::clone::Clone for DFS_TARGET_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DFS_TARGET_PRIORITY_CLASS = i32;
pub const DfsInvalidPriorityClass: DFS_TARGET_PRIORITY_CLASS = -1i32;
pub const DfsSiteCostNormalPriorityClass: DFS_TARGET_PRIORITY_CLASS = 0i32;
pub const DfsGlobalHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = 1i32;
pub const DfsSiteCostHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = 2i32;
pub const DfsSiteCostLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = 3i32;
pub const DfsGlobalLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = 4i32;
pub const DFS_VOLUME_FLAVORS: u32 = 768u32;
pub const DFS_VOLUME_FLAVOR_AD_BLOB: u32 = 512u32;
pub const DFS_VOLUME_FLAVOR_STANDALONE: u32 = 256u32;
pub const DFS_VOLUME_FLAVOR_UNUSED1: u32 = 0u32;
pub const DFS_VOLUME_STATES: u32 = 15u32;
pub const DFS_VOLUME_STATE_FORCE_SYNC: u32 = 64u32;
pub const DFS_VOLUME_STATE_INCONSISTENT: u32 = 2u32;
pub const DFS_VOLUME_STATE_OFFLINE: u32 = 3u32;
pub const DFS_VOLUME_STATE_OK: u32 = 1u32;
pub const DFS_VOLUME_STATE_ONLINE: u32 = 4u32;
pub const DFS_VOLUME_STATE_RESYNCHRONIZE: u32 = 16u32;
pub const DFS_VOLUME_STATE_STANDBY: u32 = 32u32;
pub const FSCTL_DFS_BASE: u32 = 6u32;
pub const FSCTL_DFS_GET_PKT_ENTRY_STATE: u32 = 401340u32;
pub const NET_DFS_SETDC_FLAGS: u32 = 0u32;
pub const NET_DFS_SETDC_INITPKT: u32 = 2u32;
pub const NET_DFS_SETDC_TIMEOUT: u32 = 1u32;
