#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type McastApiCleanup = unsafe extern "system" fn();
pub type McastApiStartup = unsafe extern "system" fn(version: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type McastEnumerateScopes = unsafe extern "system" fn(addrfamily: u16, requery: super::super::Foundation::BOOL, pscopelist: *mut MCAST_SCOPE_ENTRY, pscopelen: *mut u32, pscopecount: *mut u32) -> u32;
pub type McastGenUID = unsafe extern "system" fn(prequestid: *mut MCAST_CLIENT_UID) -> u32;
pub type McastReleaseAddress = unsafe extern "system" fn(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, preleaserequest: *mut MCAST_LEASE_REQUEST) -> u32;
pub type McastRenewAddress = unsafe extern "system" fn(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, prenewrequest: *mut MCAST_LEASE_REQUEST, prenewresponse: *mut MCAST_LEASE_RESPONSE) -> u32;
pub type McastRequestAddress = unsafe extern "system" fn(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, pscopectx: *mut MCAST_SCOPE_CTX, paddrrequest: *mut MCAST_LEASE_REQUEST, paddrresponse: *mut MCAST_LEASE_RESPONSE) -> u32;
#[repr(C)]
pub union IPNG_ADDRESS {
    pub IpAddrV4: u32,
    pub IpAddrV6: [u8; 16],
}
impl ::core::marker::Copy for IPNG_ADDRESS {}
impl ::core::clone::Clone for IPNG_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MCAST_API_CURRENT_VERSION: i32 = 1i32;
pub const MCAST_API_VERSION_0: i32 = 0i32;
pub const MCAST_API_VERSION_1: i32 = 1i32;
pub const MCAST_CLIENT_ID_LEN: u32 = 17u32;
#[repr(C)]
pub struct MCAST_CLIENT_UID {
    pub ClientUID: *mut u8,
    pub ClientUIDLength: u32,
}
impl ::core::marker::Copy for MCAST_CLIENT_UID {}
impl ::core::clone::Clone for MCAST_CLIENT_UID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MCAST_LEASE_REQUEST {
    pub LeaseStartTime: i32,
    pub MaxLeaseStartTime: i32,
    pub LeaseDuration: u32,
    pub MinLeaseDuration: u32,
    pub ServerAddress: IPNG_ADDRESS,
    pub MinAddrCount: u16,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl ::core::marker::Copy for MCAST_LEASE_REQUEST {}
impl ::core::clone::Clone for MCAST_LEASE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MCAST_LEASE_RESPONSE {
    pub LeaseStartTime: i32,
    pub LeaseEndTime: i32,
    pub ServerAddress: IPNG_ADDRESS,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl ::core::marker::Copy for MCAST_LEASE_RESPONSE {}
impl ::core::clone::Clone for MCAST_LEASE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MCAST_SCOPE_CTX {
    pub ScopeID: IPNG_ADDRESS,
    pub Interface: IPNG_ADDRESS,
    pub ServerID: IPNG_ADDRESS,
}
impl ::core::marker::Copy for MCAST_SCOPE_CTX {}
impl ::core::clone::Clone for MCAST_SCOPE_CTX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MCAST_SCOPE_ENTRY {
    pub ScopeCtx: MCAST_SCOPE_CTX,
    pub LastAddr: IPNG_ADDRESS,
    pub TTL: u32,
    pub ScopeDesc: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCAST_SCOPE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCAST_SCOPE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
