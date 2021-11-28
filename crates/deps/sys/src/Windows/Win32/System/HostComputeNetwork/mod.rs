#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type HcnCloseEndpoint = unsafe extern "system" fn(endpoint: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type HcnCloseGuestNetworkService = unsafe extern "system" fn(guestnetworkservice: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type HcnCloseLoadBalancer = unsafe extern "system" fn(loadbalancer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type HcnCloseNamespace = unsafe extern "system" fn(namespace: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type HcnCloseNetwork = unsafe extern "system" fn(network: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnCreateEndpoint = unsafe extern "system" fn(network: *const ::core::ffi::c_void, id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnCreateGuestNetworkService = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, guestnetworkservice: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnCreateLoadBalancer = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnCreateNamespace = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnCreateNetwork = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnDeleteEndpoint = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnDeleteGuestNetworkService = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnDeleteLoadBalancer = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnDeleteNamespace = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnDeleteNetwork = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnEnumerateEndpoints = unsafe extern "system" fn(query: super::super::Foundation::PWSTR, endpoints: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type HcnEnumerateGuestNetworkPortReservations = unsafe extern "system" fn(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnEnumerateLoadBalancers = unsafe extern "system" fn(query: super::super::Foundation::PWSTR, loadbalancer: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnEnumerateNamespaces = unsafe extern "system" fn(query: super::super::Foundation::PWSTR, namespaces: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnEnumerateNetworks = unsafe extern "system" fn(query: super::super::Foundation::PWSTR, networks: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type HcnFreeGuestNetworkPortReservations = unsafe extern "system" fn(portentries: *mut HCN_PORT_RANGE_ENTRY);
#[cfg(feature = "Win32_Foundation")]
pub type HcnModifyEndpoint = unsafe extern "system" fn(endpoint: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnModifyGuestNetworkService = unsafe extern "system" fn(guestnetworkservice: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnModifyLoadBalancer = unsafe extern "system" fn(loadbalancer: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnModifyNamespace = unsafe extern "system" fn(namespace: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnModifyNetwork = unsafe extern "system" fn(network: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnOpenEndpoint = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnOpenLoadBalancer = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnOpenNamespace = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnOpenNetwork = unsafe extern "system" fn(id: *const ::windows_sys::core::GUID, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnQueryEndpointProperties = unsafe extern "system" fn(endpoint: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnQueryLoadBalancerProperties = unsafe extern "system" fn(loadbalancer: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnQueryNamespaceProperties = unsafe extern "system" fn(namespace: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnQueryNetworkProperties = unsafe extern "system" fn(network: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnRegisterGuestNetworkServiceCallback = unsafe extern "system" fn(guestnetworkservice: *const ::core::ffi::c_void, callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnRegisterServiceCallback = unsafe extern "system" fn(callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnReleaseGuestNetworkServicePortReservationHandle = unsafe extern "system" fn(portreservationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnReserveGuestNetworkServicePort = unsafe extern "system" fn(guestnetworkservice: *const ::core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type HcnReserveGuestNetworkServicePortRange = unsafe extern "system" fn(guestnetworkservice: *const ::core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
pub type HcnUnregisterGuestNetworkServiceCallback = unsafe extern "system" fn(callbackhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type HcnUnregisterServiceCallback = unsafe extern "system" fn(callbackhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type HCN_NOTIFICATIONS = i32;
pub const HcnNotificationInvalid: HCN_NOTIFICATIONS = 0i32;
pub const HcnNotificationNetworkPreCreate: HCN_NOTIFICATIONS = 1i32;
pub const HcnNotificationNetworkCreate: HCN_NOTIFICATIONS = 2i32;
pub const HcnNotificationNetworkPreDelete: HCN_NOTIFICATIONS = 3i32;
pub const HcnNotificationNetworkDelete: HCN_NOTIFICATIONS = 4i32;
pub const HcnNotificationNamespaceCreate: HCN_NOTIFICATIONS = 5i32;
pub const HcnNotificationNamespaceDelete: HCN_NOTIFICATIONS = 6i32;
pub const HcnNotificationGuestNetworkServiceCreate: HCN_NOTIFICATIONS = 7i32;
pub const HcnNotificationGuestNetworkServiceDelete: HCN_NOTIFICATIONS = 8i32;
pub const HcnNotificationNetworkEndpointAttached: HCN_NOTIFICATIONS = 9i32;
pub const HcnNotificationNetworkEndpointDetached: HCN_NOTIFICATIONS = 16i32;
pub const HcnNotificationGuestNetworkServiceStateChanged: HCN_NOTIFICATIONS = 17i32;
pub const HcnNotificationGuestNetworkServiceInterfaceStateChanged: HCN_NOTIFICATIONS = 18i32;
pub const HcnNotificationServiceDisconnect: HCN_NOTIFICATIONS = 16777216i32;
pub const HcnNotificationFlagsReserved: HCN_NOTIFICATIONS = -268435456i32;
#[cfg(feature = "Win32_Foundation")]
pub type HCN_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows_sys::core::HRESULT, notificationdata: super::super::Foundation::PWSTR)>;
pub type HCN_PORT_ACCESS = i32;
pub const HCN_PORT_ACCESS_EXCLUSIVE: HCN_PORT_ACCESS = 1i32;
pub const HCN_PORT_ACCESS_SHARED: HCN_PORT_ACCESS = 2i32;
pub type HCN_PORT_PROTOCOL = i32;
pub const HCN_PORT_PROTOCOL_TCP: HCN_PORT_PROTOCOL = 1i32;
pub const HCN_PORT_PROTOCOL_UDP: HCN_PORT_PROTOCOL = 2i32;
pub const HCN_PORT_PROTOCOL_BOTH: HCN_PORT_PROTOCOL = 3i32;
#[repr(C)]
pub struct HCN_PORT_RANGE_ENTRY {
    pub OwningPartitionId: ::windows_sys::core::GUID,
    pub TargetPartitionId: ::windows_sys::core::GUID,
    pub Protocol: HCN_PORT_PROTOCOL,
    pub Priority: u64,
    pub ReservationType: u32,
    pub SharingFlags: u32,
    pub DeliveryMode: u32,
    pub StartingPort: u16,
    pub EndingPort: u16,
}
impl ::core::marker::Copy for HCN_PORT_RANGE_ENTRY {}
impl ::core::clone::Clone for HCN_PORT_RANGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HCN_PORT_RANGE_RESERVATION {
    pub startingPort: u16,
    pub endingPort: u16,
}
impl ::core::marker::Copy for HCN_PORT_RANGE_RESERVATION {}
impl ::core::clone::Clone for HCN_PORT_RANGE_RESERVATION {
    fn clone(&self) -> Self {
        *self
    }
}
