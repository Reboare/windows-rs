#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
pub type AllJoynAcceptBusConnection = unsafe extern "system" fn(serverbushandle: super::super::Foundation::HANDLE, abortevent: super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type AllJoynCloseBusHandle = unsafe extern "system" fn(bushandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type AllJoynConnectToBus = unsafe extern "system" fn(connectionspec: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type AllJoynCreateBus = unsafe extern "system" fn(outbuffersize: u32, inbuffersize: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type AllJoynEnumEvents = unsafe extern "system" fn(connectedbushandle: super::super::Foundation::HANDLE, eventtoreset: super::super::Foundation::HANDLE, eventtypes: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type AllJoynEventSelect = unsafe extern "system" fn(connectedbushandle: super::super::Foundation::HANDLE, eventhandle: super::super::Foundation::HANDLE, eventtypes: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type AllJoynReceiveFromBus = unsafe extern "system" fn(connectedbushandle: super::super::Foundation::HANDLE, buffer: *mut ::core::ffi::c_void, bytestoread: u32, bytestransferred: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type AllJoynSendToBus = unsafe extern "system" fn(connectedbushandle: super::super::Foundation::HANDLE, buffer: *const ::core::ffi::c_void, bytestowrite: u32, bytestransferred: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type QCC_StatusText = unsafe extern "system" fn(status: QStatus) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_create = unsafe extern "system" fn(defaultlanguage: super::super::Foundation::PSTR) -> alljoyn_aboutdata;
pub type alljoyn_aboutdata_create_empty = unsafe extern "system" fn() -> alljoyn_aboutdata;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_create_full = unsafe extern "system" fn(arg: alljoyn_msgarg, language: super::super::Foundation::PSTR) -> alljoyn_aboutdata;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_createfrommsgarg = unsafe extern "system" fn(data: alljoyn_aboutdata, arg: alljoyn_msgarg, language: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_createfromxml = unsafe extern "system" fn(data: alljoyn_aboutdata, aboutdataxml: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_aboutdata_destroy = unsafe extern "system" fn(data: alljoyn_aboutdata);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_getaboutdata = unsafe extern "system" fn(data: alljoyn_aboutdata, msgarg: alljoyn_msgarg, language: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_aboutdata_getajsoftwareversion = unsafe extern "system" fn(data: alljoyn_aboutdata, ajsoftwareversion: *mut *mut i8) -> QStatus;
pub type alljoyn_aboutdata_getannouncedaboutdata = unsafe extern "system" fn(data: alljoyn_aboutdata, msgarg: alljoyn_msgarg) -> QStatus;
pub type alljoyn_aboutdata_getappid = unsafe extern "system" fn(data: alljoyn_aboutdata, appid: *mut *mut u8, num: *mut usize) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_getappname = unsafe extern "system" fn(data: alljoyn_aboutdata, appname: *mut *mut i8, language: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_aboutdata_getdateofmanufacture = unsafe extern "system" fn(data: alljoyn_aboutdata, dateofmanufacture: *mut *mut i8) -> QStatus;
pub type alljoyn_aboutdata_getdefaultlanguage = unsafe extern "system" fn(data: alljoyn_aboutdata, defaultlanguage: *mut *mut i8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_getdescription = unsafe extern "system" fn(data: alljoyn_aboutdata, description: *mut *mut i8, language: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_aboutdata_getdeviceid = unsafe extern "system" fn(data: alljoyn_aboutdata, deviceid: *mut *mut i8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_getdevicename = unsafe extern "system" fn(data: alljoyn_aboutdata, devicename: *mut *mut i8, language: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_getfield = unsafe extern "system" fn(data: alljoyn_aboutdata, name: super::super::Foundation::PSTR, value: *mut alljoyn_msgarg, language: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_aboutdata_getfields = unsafe extern "system" fn(data: alljoyn_aboutdata, fields: *const *const i8, num_fields: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_getfieldsignature = unsafe extern "system" fn(data: alljoyn_aboutdata, fieldname: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
pub type alljoyn_aboutdata_gethardwareversion = unsafe extern "system" fn(data: alljoyn_aboutdata, hardwareversion: *mut *mut i8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_getmanufacturer = unsafe extern "system" fn(data: alljoyn_aboutdata, manufacturer: *mut *mut i8, language: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_aboutdata_getmodelnumber = unsafe extern "system" fn(data: alljoyn_aboutdata, modelnumber: *mut *mut i8) -> QStatus;
pub type alljoyn_aboutdata_getsoftwareversion = unsafe extern "system" fn(data: alljoyn_aboutdata, softwareversion: *mut *mut i8) -> QStatus;
pub type alljoyn_aboutdata_getsupportedlanguages = unsafe extern "system" fn(data: alljoyn_aboutdata, languagetags: *const *const i8, num: usize) -> usize;
pub type alljoyn_aboutdata_getsupporturl = unsafe extern "system" fn(data: alljoyn_aboutdata, supporturl: *mut *mut i8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_isfieldannounced = unsafe extern "system" fn(data: alljoyn_aboutdata, fieldname: super::super::Foundation::PSTR) -> u8;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_isfieldlocalized = unsafe extern "system" fn(data: alljoyn_aboutdata, fieldname: super::super::Foundation::PSTR) -> u8;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_isfieldrequired = unsafe extern "system" fn(data: alljoyn_aboutdata, fieldname: super::super::Foundation::PSTR) -> u8;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_isvalid = unsafe extern "system" fn(data: alljoyn_aboutdata, language: super::super::Foundation::PSTR) -> u8;
pub type alljoyn_aboutdata_setappid = unsafe extern "system" fn(data: alljoyn_aboutdata, appid: *const u8, num: usize) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setappid_fromstring = unsafe extern "system" fn(data: alljoyn_aboutdata, appid: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setappname = unsafe extern "system" fn(data: alljoyn_aboutdata, appname: super::super::Foundation::PSTR, language: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setdateofmanufacture = unsafe extern "system" fn(data: alljoyn_aboutdata, dateofmanufacture: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setdefaultlanguage = unsafe extern "system" fn(data: alljoyn_aboutdata, defaultlanguage: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setdescription = unsafe extern "system" fn(data: alljoyn_aboutdata, description: super::super::Foundation::PSTR, language: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setdeviceid = unsafe extern "system" fn(data: alljoyn_aboutdata, deviceid: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setdevicename = unsafe extern "system" fn(data: alljoyn_aboutdata, devicename: super::super::Foundation::PSTR, language: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setfield = unsafe extern "system" fn(data: alljoyn_aboutdata, name: super::super::Foundation::PSTR, value: alljoyn_msgarg, language: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_sethardwareversion = unsafe extern "system" fn(data: alljoyn_aboutdata, hardwareversion: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setmanufacturer = unsafe extern "system" fn(data: alljoyn_aboutdata, manufacturer: super::super::Foundation::PSTR, language: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setmodelnumber = unsafe extern "system" fn(data: alljoyn_aboutdata, modelnumber: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setsoftwareversion = unsafe extern "system" fn(data: alljoyn_aboutdata, softwareversion: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setsupportedlanguage = unsafe extern "system" fn(data: alljoyn_aboutdata, language: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdata_setsupporturl = unsafe extern "system" fn(data: alljoyn_aboutdata, supporturl: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdatalistener_create = unsafe extern "system" fn(callbacks: *const alljoyn_aboutdatalistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_aboutdatalistener;
pub type alljoyn_aboutdatalistener_destroy = unsafe extern "system" fn(listener: alljoyn_aboutdatalistener);
pub type alljoyn_abouticon_clear = unsafe extern "system" fn(icon: *mut _alljoyn_abouticon_handle);
pub type alljoyn_abouticon_create = unsafe extern "system" fn() -> *mut _alljoyn_abouticon_handle;
pub type alljoyn_abouticon_destroy = unsafe extern "system" fn(icon: *mut _alljoyn_abouticon_handle);
pub type alljoyn_abouticon_getcontent = unsafe extern "system" fn(icon: *mut _alljoyn_abouticon_handle, data: *const *const u8, size: *mut usize);
pub type alljoyn_abouticon_geturl = unsafe extern "system" fn(icon: *mut _alljoyn_abouticon_handle, r#type: *const *const i8, url: *const *const i8);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_abouticon_setcontent = unsafe extern "system" fn(icon: *mut _alljoyn_abouticon_handle, r#type: super::super::Foundation::PSTR, data: *mut u8, csize: usize, ownsdata: u8) -> QStatus;
pub type alljoyn_abouticon_setcontent_frommsgarg = unsafe extern "system" fn(icon: *mut _alljoyn_abouticon_handle, arg: alljoyn_msgarg) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_abouticon_seturl = unsafe extern "system" fn(icon: *mut _alljoyn_abouticon_handle, r#type: super::super::Foundation::PSTR, url: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_abouticonobj_create = unsafe extern "system" fn(bus: alljoyn_busattachment, icon: *mut _alljoyn_abouticon_handle) -> *mut _alljoyn_abouticonobj_handle;
pub type alljoyn_abouticonobj_destroy = unsafe extern "system" fn(icon: *mut _alljoyn_abouticonobj_handle);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_abouticonproxy_create = unsafe extern "system" fn(bus: alljoyn_busattachment, busname: super::super::Foundation::PSTR, sessionid: u32) -> *mut _alljoyn_abouticonproxy_handle;
pub type alljoyn_abouticonproxy_destroy = unsafe extern "system" fn(proxy: *mut _alljoyn_abouticonproxy_handle);
pub type alljoyn_abouticonproxy_geticon = unsafe extern "system" fn(proxy: *mut _alljoyn_abouticonproxy_handle, icon: *mut _alljoyn_abouticon_handle) -> QStatus;
pub type alljoyn_abouticonproxy_getversion = unsafe extern "system" fn(proxy: *mut _alljoyn_abouticonproxy_handle, version: *mut u16) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutlistener_create = unsafe extern "system" fn(callback: *const alljoyn_aboutlistener_callback, context: *const ::core::ffi::c_void) -> alljoyn_aboutlistener;
pub type alljoyn_aboutlistener_destroy = unsafe extern "system" fn(listener: alljoyn_aboutlistener);
pub type alljoyn_aboutobj_announce = unsafe extern "system" fn(obj: alljoyn_aboutobj, sessionport: u16, aboutdata: alljoyn_aboutdata) -> QStatus;
pub type alljoyn_aboutobj_announce_using_datalistener = unsafe extern "system" fn(obj: alljoyn_aboutobj, sessionport: u16, aboutlistener: alljoyn_aboutdatalistener) -> QStatus;
pub type alljoyn_aboutobj_create = unsafe extern "system" fn(bus: alljoyn_busattachment, isannounced: alljoyn_about_announceflag) -> alljoyn_aboutobj;
pub type alljoyn_aboutobj_destroy = unsafe extern "system" fn(obj: alljoyn_aboutobj);
pub type alljoyn_aboutobj_unannounce = unsafe extern "system" fn(obj: alljoyn_aboutobj) -> QStatus;
pub type alljoyn_aboutobjectdescription_clear = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription);
pub type alljoyn_aboutobjectdescription_create = unsafe extern "system" fn() -> alljoyn_aboutobjectdescription;
pub type alljoyn_aboutobjectdescription_create_full = unsafe extern "system" fn(arg: alljoyn_msgarg) -> alljoyn_aboutobjectdescription;
pub type alljoyn_aboutobjectdescription_createfrommsgarg = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription, arg: alljoyn_msgarg) -> QStatus;
pub type alljoyn_aboutobjectdescription_destroy = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutobjectdescription_getinterfacepaths = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription, interfacename: super::super::Foundation::PSTR, paths: *const *const i8, numpaths: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutobjectdescription_getinterfaces = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription, path: super::super::Foundation::PSTR, interfaces: *const *const i8, numinterfaces: usize) -> usize;
pub type alljoyn_aboutobjectdescription_getmsgarg = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription, msgarg: alljoyn_msgarg) -> QStatus;
pub type alljoyn_aboutobjectdescription_getpaths = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription, paths: *const *const i8, numpaths: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutobjectdescription_hasinterface = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription, interfacename: super::super::Foundation::PSTR) -> u8;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutobjectdescription_hasinterfaceatpath = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription, path: super::super::Foundation::PSTR, interfacename: super::super::Foundation::PSTR) -> u8;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutobjectdescription_haspath = unsafe extern "system" fn(description: alljoyn_aboutobjectdescription, path: super::super::Foundation::PSTR) -> u8;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutproxy_create = unsafe extern "system" fn(bus: alljoyn_busattachment, busname: super::super::Foundation::PSTR, sessionid: u32) -> alljoyn_aboutproxy;
pub type alljoyn_aboutproxy_destroy = unsafe extern "system" fn(proxy: alljoyn_aboutproxy);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutproxy_getaboutdata = unsafe extern "system" fn(proxy: alljoyn_aboutproxy, language: super::super::Foundation::PSTR, data: alljoyn_msgarg) -> QStatus;
pub type alljoyn_aboutproxy_getobjectdescription = unsafe extern "system" fn(proxy: alljoyn_aboutproxy, objectdesc: alljoyn_msgarg) -> QStatus;
pub type alljoyn_aboutproxy_getversion = unsafe extern "system" fn(proxy: alljoyn_aboutproxy, version: *mut u16) -> QStatus;
pub type alljoyn_applicationstatelistener_create = unsafe extern "system" fn(callbacks: *const alljoyn_applicationstatelistener_callbacks, context: *mut ::core::ffi::c_void) -> alljoyn_applicationstatelistener;
pub type alljoyn_applicationstatelistener_destroy = unsafe extern "system" fn(listener: alljoyn_applicationstatelistener);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_authlistener_create = unsafe extern "system" fn(callbacks: *const alljoyn_authlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_authlistener;
pub type alljoyn_authlistener_destroy = unsafe extern "system" fn(listener: alljoyn_authlistener);
pub type alljoyn_authlistener_requestcredentialsresponse = unsafe extern "system" fn(listener: alljoyn_authlistener, authcontext: *mut ::core::ffi::c_void, accept: i32, credentials: alljoyn_credentials) -> QStatus;
pub type alljoyn_authlistener_setsharedsecret = unsafe extern "system" fn(listener: alljoyn_authlistener, sharedsecret: *const u8, sharedsecretsize: usize) -> QStatus;
pub type alljoyn_authlistener_verifycredentialsresponse = unsafe extern "system" fn(listener: alljoyn_authlistener, authcontext: *mut ::core::ffi::c_void, accept: i32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_authlistenerasync_create = unsafe extern "system" fn(callbacks: *const alljoyn_authlistenerasync_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_authlistener;
pub type alljoyn_authlistenerasync_destroy = unsafe extern "system" fn(listener: alljoyn_authlistener);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_autopinger_adddestination = unsafe extern "system" fn(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR, destination: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_autopinger_addpinggroup = unsafe extern "system" fn(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR, listener: alljoyn_pinglistener, pinginterval: u32);
pub type alljoyn_autopinger_create = unsafe extern "system" fn(bus: alljoyn_busattachment) -> alljoyn_autopinger;
pub type alljoyn_autopinger_destroy = unsafe extern "system" fn(autopinger: alljoyn_autopinger);
pub type alljoyn_autopinger_pause = unsafe extern "system" fn(autopinger: alljoyn_autopinger);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_autopinger_removedestination = unsafe extern "system" fn(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR, destination: super::super::Foundation::PSTR, removeall: i32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_autopinger_removepinggroup = unsafe extern "system" fn(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR);
pub type alljoyn_autopinger_resume = unsafe extern "system" fn(autopinger: alljoyn_autopinger);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_autopinger_setpinginterval = unsafe extern "system" fn(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR, pinginterval: u32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_addlogonentry = unsafe extern "system" fn(bus: alljoyn_busattachment, authmechanism: super::super::Foundation::PSTR, username: super::super::Foundation::PSTR, password: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_addmatch = unsafe extern "system" fn(bus: alljoyn_busattachment, rule: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_advertisename = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, transports: u16) -> QStatus;
pub type alljoyn_busattachment_bindsessionport = unsafe extern "system" fn(bus: alljoyn_busattachment, sessionport: *mut u16, opts: alljoyn_sessionopts, listener: alljoyn_sessionportlistener) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_canceladvertisename = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, transports: u16) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_cancelfindadvertisedname = unsafe extern "system" fn(bus: alljoyn_busattachment, nameprefix: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_cancelfindadvertisednamebytransport = unsafe extern "system" fn(bus: alljoyn_busattachment, nameprefix: super::super::Foundation::PSTR, transports: u16) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_cancelwhoimplements_interface = unsafe extern "system" fn(bus: alljoyn_busattachment, implementsinterface: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_busattachment_cancelwhoimplements_interfaces = unsafe extern "system" fn(bus: alljoyn_busattachment, implementsinterfaces: *const *const i8, numberinterfaces: usize) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_clearkeys = unsafe extern "system" fn(bus: alljoyn_busattachment, guid: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_busattachment_clearkeystore = unsafe extern "system" fn(bus: alljoyn_busattachment);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_connect = unsafe extern "system" fn(bus: alljoyn_busattachment, connectspec: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_create = unsafe extern "system" fn(applicationname: super::super::Foundation::PSTR, allowremotemessages: i32) -> alljoyn_busattachment;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_create_concurrency = unsafe extern "system" fn(applicationname: super::super::Foundation::PSTR, allowremotemessages: i32, concurrency: u32) -> alljoyn_busattachment;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_createinterface = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, iface: *mut alljoyn_interfacedescription) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_createinterface_secure = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, iface: *mut alljoyn_interfacedescription, secpolicy: alljoyn_interfacedescription_securitypolicy) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_createinterfacesfromxml = unsafe extern "system" fn(bus: alljoyn_busattachment, xml: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_deletedefaultkeystore = unsafe extern "system" fn(applicationname: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_busattachment_deleteinterface = unsafe extern "system" fn(bus: alljoyn_busattachment, iface: alljoyn_interfacedescription) -> QStatus;
pub type alljoyn_busattachment_destroy = unsafe extern "system" fn(bus: alljoyn_busattachment);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_disconnect = unsafe extern "system" fn(bus: alljoyn_busattachment, unused: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_busattachment_enableconcurrentcallbacks = unsafe extern "system" fn(bus: alljoyn_busattachment);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_enablepeersecurity = unsafe extern "system" fn(bus: alljoyn_busattachment, authmechanisms: super::super::Foundation::PSTR, listener: alljoyn_authlistener, keystorefilename: super::super::Foundation::PSTR, isshared: i32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_enablepeersecuritywithpermissionconfigurationlistener = unsafe extern "system" fn(bus: alljoyn_busattachment, authmechanisms: super::super::Foundation::PSTR, authlistener: alljoyn_authlistener, keystorefilename: super::super::Foundation::PSTR, isshared: i32, permissionconfigurationlistener: alljoyn_permissionconfigurationlistener) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_findadvertisedname = unsafe extern "system" fn(bus: alljoyn_busattachment, nameprefix: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_findadvertisednamebytransport = unsafe extern "system" fn(bus: alljoyn_busattachment, nameprefix: super::super::Foundation::PSTR, transports: u16) -> QStatus;
pub type alljoyn_busattachment_getalljoyndebugobj = unsafe extern "system" fn(bus: alljoyn_busattachment) -> alljoyn_proxybusobject;
pub type alljoyn_busattachment_getalljoynproxyobj = unsafe extern "system" fn(bus: alljoyn_busattachment) -> alljoyn_proxybusobject;
pub type alljoyn_busattachment_getconcurrency = unsafe extern "system" fn(bus: alljoyn_busattachment) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_getconnectspec = unsafe extern "system" fn(bus: alljoyn_busattachment) -> super::super::Foundation::PSTR;
pub type alljoyn_busattachment_getdbusproxyobj = unsafe extern "system" fn(bus: alljoyn_busattachment) -> alljoyn_proxybusobject;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_getglobalguidstring = unsafe extern "system" fn(bus: alljoyn_busattachment) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_getinterface = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR) -> alljoyn_interfacedescription;
pub type alljoyn_busattachment_getinterfaces = unsafe extern "system" fn(bus: alljoyn_busattachment, ifaces: *const alljoyn_interfacedescription, numifaces: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_getkeyexpiration = unsafe extern "system" fn(bus: alljoyn_busattachment, guid: super::super::Foundation::PSTR, timeout: *mut u32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_getpeerguid = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, guid: super::super::Foundation::PSTR, guidsz: *mut usize) -> QStatus;
pub type alljoyn_busattachment_getpermissionconfigurator = unsafe extern "system" fn(bus: alljoyn_busattachment) -> alljoyn_permissionconfigurator;
pub type alljoyn_busattachment_gettimestamp = unsafe extern "system" fn() -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_getuniquename = unsafe extern "system" fn(bus: alljoyn_busattachment) -> super::super::Foundation::PSTR;
pub type alljoyn_busattachment_isconnected = unsafe extern "system" fn(bus: alljoyn_busattachment) -> i32;
pub type alljoyn_busattachment_ispeersecurityenabled = unsafe extern "system" fn(bus: alljoyn_busattachment) -> i32;
pub type alljoyn_busattachment_isstarted = unsafe extern "system" fn(bus: alljoyn_busattachment) -> i32;
pub type alljoyn_busattachment_isstopping = unsafe extern "system" fn(bus: alljoyn_busattachment) -> i32;
pub type alljoyn_busattachment_join = unsafe extern "system" fn(bus: alljoyn_busattachment) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_joinsession = unsafe extern "system" fn(bus: alljoyn_busattachment, sessionhost: super::super::Foundation::PSTR, sessionport: u16, listener: alljoyn_sessionlistener, sessionid: *mut u32, opts: alljoyn_sessionopts) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_joinsessionasync = unsafe extern "system" fn(bus: alljoyn_busattachment, sessionhost: super::super::Foundation::PSTR, sessionport: u16, listener: alljoyn_sessionlistener, opts: alljoyn_sessionopts, callback: alljoyn_busattachment_joinsessioncb_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
pub type alljoyn_busattachment_leavesession = unsafe extern "system" fn(bus: alljoyn_busattachment, sessionid: u32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_namehasowner = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, hasowner: *mut i32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_ping = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, timeout: u32) -> QStatus;
pub type alljoyn_busattachment_registeraboutlistener = unsafe extern "system" fn(bus: alljoyn_busattachment, aboutlistener: alljoyn_aboutlistener);
pub type alljoyn_busattachment_registerapplicationstatelistener = unsafe extern "system" fn(bus: alljoyn_busattachment, listener: alljoyn_applicationstatelistener) -> QStatus;
pub type alljoyn_busattachment_registerbuslistener = unsafe extern "system" fn(bus: alljoyn_busattachment, listener: alljoyn_buslistener);
pub type alljoyn_busattachment_registerbusobject = unsafe extern "system" fn(bus: alljoyn_busattachment, obj: alljoyn_busobject) -> QStatus;
pub type alljoyn_busattachment_registerbusobject_secure = unsafe extern "system" fn(bus: alljoyn_busattachment, obj: alljoyn_busobject) -> QStatus;
pub type alljoyn_busattachment_registerkeystorelistener = unsafe extern "system" fn(bus: alljoyn_busattachment, listener: alljoyn_keystorelistener) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_registersignalhandler = unsafe extern "system" fn(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, srcpath: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_registersignalhandlerwithrule = unsafe extern "system" fn(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, matchrule: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_releasename = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_busattachment_reloadkeystore = unsafe extern "system" fn(bus: alljoyn_busattachment) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_removematch = unsafe extern "system" fn(bus: alljoyn_busattachment, rule: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_removesessionmember = unsafe extern "system" fn(bus: alljoyn_busattachment, sessionid: u32, membername: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_requestname = unsafe extern "system" fn(bus: alljoyn_busattachment, requestedname: super::super::Foundation::PSTR, flags: u32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_secureconnection = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, forceauth: i32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_secureconnectionasync = unsafe extern "system" fn(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, forceauth: i32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_setdaemondebug = unsafe extern "system" fn(bus: alljoyn_busattachment, module: super::super::Foundation::PSTR, level: u32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_setkeyexpiration = unsafe extern "system" fn(bus: alljoyn_busattachment, guid: super::super::Foundation::PSTR, timeout: u32) -> QStatus;
pub type alljoyn_busattachment_setlinktimeout = unsafe extern "system" fn(bus: alljoyn_busattachment, sessionid: u32, linktimeout: *mut u32) -> QStatus;
pub type alljoyn_busattachment_setlinktimeoutasync = unsafe extern "system" fn(bus: alljoyn_busattachment, sessionid: u32, linktimeout: u32, callback: alljoyn_busattachment_setlinktimeoutcb_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
pub type alljoyn_busattachment_setsessionlistener = unsafe extern "system" fn(bus: alljoyn_busattachment, sessionid: u32, listener: alljoyn_sessionlistener) -> QStatus;
pub type alljoyn_busattachment_start = unsafe extern "system" fn(bus: alljoyn_busattachment) -> QStatus;
pub type alljoyn_busattachment_stop = unsafe extern "system" fn(bus: alljoyn_busattachment) -> QStatus;
pub type alljoyn_busattachment_unbindsessionport = unsafe extern "system" fn(bus: alljoyn_busattachment, sessionport: u16) -> QStatus;
pub type alljoyn_busattachment_unregisteraboutlistener = unsafe extern "system" fn(bus: alljoyn_busattachment, aboutlistener: alljoyn_aboutlistener);
pub type alljoyn_busattachment_unregisterallaboutlisteners = unsafe extern "system" fn(bus: alljoyn_busattachment);
pub type alljoyn_busattachment_unregisterallhandlers = unsafe extern "system" fn(bus: alljoyn_busattachment) -> QStatus;
pub type alljoyn_busattachment_unregisterapplicationstatelistener = unsafe extern "system" fn(bus: alljoyn_busattachment, listener: alljoyn_applicationstatelistener) -> QStatus;
pub type alljoyn_busattachment_unregisterbuslistener = unsafe extern "system" fn(bus: alljoyn_busattachment, listener: alljoyn_buslistener);
pub type alljoyn_busattachment_unregisterbusobject = unsafe extern "system" fn(bus: alljoyn_busattachment, object: alljoyn_busobject);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_unregistersignalhandler = unsafe extern "system" fn(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, srcpath: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_unregistersignalhandlerwithrule = unsafe extern "system" fn(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, matchrule: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busattachment_whoimplements_interface = unsafe extern "system" fn(bus: alljoyn_busattachment, implementsinterface: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_busattachment_whoimplements_interfaces = unsafe extern "system" fn(bus: alljoyn_busattachment, implementsinterfaces: *const *const i8, numberinterfaces: usize) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_buslistener_create = unsafe extern "system" fn(callbacks: *const alljoyn_buslistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_buslistener;
pub type alljoyn_buslistener_destroy = unsafe extern "system" fn(listener: alljoyn_buslistener);
pub type alljoyn_busobject_addinterface = unsafe extern "system" fn(bus: alljoyn_busobject, iface: alljoyn_interfacedescription) -> QStatus;
pub type alljoyn_busobject_addinterface_announced = unsafe extern "system" fn(bus: alljoyn_busobject, iface: alljoyn_interfacedescription) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_addmethodhandler = unsafe extern "system" fn(bus: alljoyn_busobject, member: alljoyn_interfacedescription_member, handler: alljoyn_messagereceiver_methodhandler_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_addmethodhandlers = unsafe extern "system" fn(bus: alljoyn_busobject, entries: *const alljoyn_busobject_methodentry, numentries: usize) -> QStatus;
pub type alljoyn_busobject_cancelsessionlessmessage = unsafe extern "system" fn(bus: alljoyn_busobject, msg: alljoyn_message) -> QStatus;
pub type alljoyn_busobject_cancelsessionlessmessage_serial = unsafe extern "system" fn(bus: alljoyn_busobject, serialnumber: u32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_create = unsafe extern "system" fn(path: super::super::Foundation::PSTR, isplaceholder: i32, callbacks_in: *const alljoyn_busobject_callbacks, context_in: *const ::core::ffi::c_void) -> alljoyn_busobject;
pub type alljoyn_busobject_destroy = unsafe extern "system" fn(bus: alljoyn_busobject);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_emitpropertieschanged = unsafe extern "system" fn(bus: alljoyn_busobject, ifcname: super::super::Foundation::PSTR, propnames: *const *const i8, numprops: usize, id: u32);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_emitpropertychanged = unsafe extern "system" fn(bus: alljoyn_busobject, ifcname: super::super::Foundation::PSTR, propname: super::super::Foundation::PSTR, val: alljoyn_msgarg, id: u32);
pub type alljoyn_busobject_getannouncedinterfacenames = unsafe extern "system" fn(bus: alljoyn_busobject, interfaces: *const *const i8, numinterfaces: usize) -> usize;
pub type alljoyn_busobject_getbusattachment = unsafe extern "system" fn(bus: alljoyn_busobject) -> alljoyn_busattachment;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_getname = unsafe extern "system" fn(bus: alljoyn_busobject, buffer: super::super::Foundation::PSTR, buffersz: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_getpath = unsafe extern "system" fn(bus: alljoyn_busobject) -> super::super::Foundation::PSTR;
pub type alljoyn_busobject_issecure = unsafe extern "system" fn(bus: alljoyn_busobject) -> i32;
pub type alljoyn_busobject_methodreply_args = unsafe extern "system" fn(bus: alljoyn_busobject, msg: alljoyn_message, args: alljoyn_msgarg, numargs: usize) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_methodreply_err = unsafe extern "system" fn(bus: alljoyn_busobject, msg: alljoyn_message, error: super::super::Foundation::PSTR, errormessage: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_busobject_methodreply_status = unsafe extern "system" fn(bus: alljoyn_busobject, msg: alljoyn_message, status: QStatus) -> QStatus;
pub type alljoyn_busobject_setannounceflag = unsafe extern "system" fn(bus: alljoyn_busobject, iface: alljoyn_interfacedescription, isannounced: alljoyn_about_announceflag) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_signal = unsafe extern "system" fn(bus: alljoyn_busobject, destination: super::super::Foundation::PSTR, sessionid: u32, signal: alljoyn_interfacedescription_member, args: alljoyn_msgarg, numargs: usize, timetolive: u16, flags: u8, msg: alljoyn_message) -> QStatus;
pub type alljoyn_credentials_clear = unsafe extern "system" fn(cred: alljoyn_credentials);
pub type alljoyn_credentials_create = unsafe extern "system" fn() -> alljoyn_credentials;
pub type alljoyn_credentials_destroy = unsafe extern "system" fn(cred: alljoyn_credentials);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_getcertchain = unsafe extern "system" fn(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
pub type alljoyn_credentials_getexpiration = unsafe extern "system" fn(cred: alljoyn_credentials) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_getlogonentry = unsafe extern "system" fn(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_getpassword = unsafe extern "system" fn(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_getprivateKey = unsafe extern "system" fn(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_getusername = unsafe extern "system" fn(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
pub type alljoyn_credentials_isset = unsafe extern "system" fn(cred: alljoyn_credentials, creds: u16) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_setcertchain = unsafe extern "system" fn(cred: alljoyn_credentials, certchain: super::super::Foundation::PSTR);
pub type alljoyn_credentials_setexpiration = unsafe extern "system" fn(cred: alljoyn_credentials, expiration: u32);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_setlogonentry = unsafe extern "system" fn(cred: alljoyn_credentials, logonentry: super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_setpassword = unsafe extern "system" fn(cred: alljoyn_credentials, pwd: super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_setprivatekey = unsafe extern "system" fn(cred: alljoyn_credentials, pk: super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_credentials_setusername = unsafe extern "system" fn(cred: alljoyn_credentials, username: super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_getbuildinfo = unsafe extern "system" fn() -> super::super::Foundation::PSTR;
pub type alljoyn_getnumericversion = unsafe extern "system" fn() -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_getversion = unsafe extern "system" fn() -> super::super::Foundation::PSTR;
pub type alljoyn_init = unsafe extern "system" fn() -> QStatus;
pub type alljoyn_interfacedescription_activate = unsafe extern "system" fn(iface: alljoyn_interfacedescription);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_addannotation = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_addargannotation = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, argname: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_addmember = unsafe extern "system" fn(iface: alljoyn_interfacedescription, r#type: alljoyn_messagetype, name: super::super::Foundation::PSTR, inputsig: super::super::Foundation::PSTR, outsig: super::super::Foundation::PSTR, argnames: super::super::Foundation::PSTR, annotation: u8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_addmemberannotation = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_addmethod = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, inputsig: super::super::Foundation::PSTR, outsig: super::super::Foundation::PSTR, argnames: super::super::Foundation::PSTR, annotation: u8, accessperms: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_addproperty = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, signature: super::super::Foundation::PSTR, access: u8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_addpropertyannotation = unsafe extern "system" fn(iface: alljoyn_interfacedescription, property: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_addsignal = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, sig: super::super::Foundation::PSTR, argnames: super::super::Foundation::PSTR, annotation: u8, accessperms: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_interfacedescription_eql = unsafe extern "system" fn(one: alljoyn_interfacedescription, other: alljoyn_interfacedescription) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getannotation = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getannotationatindex = unsafe extern "system" fn(iface: alljoyn_interfacedescription, index: usize, name: super::super::Foundation::PSTR, name_size: *mut usize, value: super::super::Foundation::PSTR, value_size: *mut usize);
pub type alljoyn_interfacedescription_getannotationscount = unsafe extern "system" fn(iface: alljoyn_interfacedescription) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getargdescriptionforlanguage = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, arg: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, maxlanguagelength: usize, languagetag: super::super::Foundation::PSTR) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getdescriptionforlanguage = unsafe extern "system" fn(iface: alljoyn_interfacedescription, description: super::super::Foundation::PSTR, maxlanguagelength: usize, languagetag: super::super::Foundation::PSTR) -> usize;
pub type alljoyn_interfacedescription_getdescriptionlanguages = unsafe extern "system" fn(iface: alljoyn_interfacedescription, languages: *const *const i8, size: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getdescriptionlanguages2 = unsafe extern "system" fn(iface: alljoyn_interfacedescription, languages: super::super::Foundation::PSTR, languagessize: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getdescriptiontranslationcallback = unsafe extern "system" fn(iface: alljoyn_interfacedescription) -> alljoyn_interfacedescription_translation_callback_ptr;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getmember = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, member: *mut alljoyn_interfacedescription_member) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getmemberannotation = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getmemberargannotation = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, argname: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getmemberdescriptionforlanguage = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, maxlanguagelength: usize, languagetag: super::super::Foundation::PSTR) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getmembers = unsafe extern "system" fn(iface: alljoyn_interfacedescription, members: *mut alljoyn_interfacedescription_member, nummembers: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getmethod = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, member: *mut alljoyn_interfacedescription_member) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getname = unsafe extern "system" fn(iface: alljoyn_interfacedescription) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getproperties = unsafe extern "system" fn(iface: alljoyn_interfacedescription, props: *mut alljoyn_interfacedescription_property, numprops: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getproperty = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, property: *mut alljoyn_interfacedescription_property) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getpropertyannotation = unsafe extern "system" fn(iface: alljoyn_interfacedescription, property: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, str_size: *mut usize) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getpropertydescriptionforlanguage = unsafe extern "system" fn(iface: alljoyn_interfacedescription, property: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, maxlanguagelength: usize, languagetag: super::super::Foundation::PSTR) -> usize;
pub type alljoyn_interfacedescription_getsecuritypolicy = unsafe extern "system" fn(iface: alljoyn_interfacedescription) -> alljoyn_interfacedescription_securitypolicy;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_getsignal = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, member: *mut alljoyn_interfacedescription_member) -> i32;
pub type alljoyn_interfacedescription_hasdescription = unsafe extern "system" fn(iface: alljoyn_interfacedescription) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_hasmember = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, insig: super::super::Foundation::PSTR, outsig: super::super::Foundation::PSTR) -> i32;
pub type alljoyn_interfacedescription_hasproperties = unsafe extern "system" fn(iface: alljoyn_interfacedescription) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_hasproperty = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_introspect = unsafe extern "system" fn(iface: alljoyn_interfacedescription, str: super::super::Foundation::PSTR, buf: usize, indent: usize) -> usize;
pub type alljoyn_interfacedescription_issecure = unsafe extern "system" fn(iface: alljoyn_interfacedescription) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_member_eql = unsafe extern "system" fn(one: alljoyn_interfacedescription_member, other: alljoyn_interfacedescription_member) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_member_getannotation = unsafe extern "system" fn(member: alljoyn_interfacedescription_member, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_member_getannotationatindex = unsafe extern "system" fn(member: alljoyn_interfacedescription_member, index: usize, name: super::super::Foundation::PSTR, name_size: *mut usize, value: super::super::Foundation::PSTR, value_size: *mut usize);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_member_getannotationscount = unsafe extern "system" fn(member: alljoyn_interfacedescription_member) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_member_getargannotation = unsafe extern "system" fn(member: alljoyn_interfacedescription_member, argname: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_member_getargannotationatindex = unsafe extern "system" fn(member: alljoyn_interfacedescription_member, argname: super::super::Foundation::PSTR, index: usize, name: super::super::Foundation::PSTR, name_size: *mut usize, value: super::super::Foundation::PSTR, value_size: *mut usize);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_member_getargannotationscount = unsafe extern "system" fn(member: alljoyn_interfacedescription_member, argname: super::super::Foundation::PSTR) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_property_eql = unsafe extern "system" fn(one: alljoyn_interfacedescription_property, other: alljoyn_interfacedescription_property) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_property_getannotation = unsafe extern "system" fn(property: alljoyn_interfacedescription_property, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_property_getannotationatindex = unsafe extern "system" fn(property: alljoyn_interfacedescription_property, index: usize, name: super::super::Foundation::PSTR, name_size: *mut usize, value: super::super::Foundation::PSTR, value_size: *mut usize);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_property_getannotationscount = unsafe extern "system" fn(property: alljoyn_interfacedescription_property) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setargdescription = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, argname: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setargdescriptionforlanguage = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, arg: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, languagetag: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setdescription = unsafe extern "system" fn(iface: alljoyn_interfacedescription, description: super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setdescriptionforlanguage = unsafe extern "system" fn(iface: alljoyn_interfacedescription, description: super::super::Foundation::PSTR, languagetag: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setdescriptionlanguage = unsafe extern "system" fn(iface: alljoyn_interfacedescription, language: super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setdescriptiontranslationcallback = unsafe extern "system" fn(iface: alljoyn_interfacedescription, translationcallback: alljoyn_interfacedescription_translation_callback_ptr);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setmemberdescription = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setmemberdescriptionforlanguage = unsafe extern "system" fn(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, languagetag: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setpropertydescription = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_setpropertydescriptionforlanguage = unsafe extern "system" fn(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, languagetag: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_keystorelistener_create = unsafe extern "system" fn(callbacks: *const alljoyn_keystorelistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_keystorelistener;
pub type alljoyn_keystorelistener_destroy = unsafe extern "system" fn(listener: alljoyn_keystorelistener);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_keystorelistener_getkeys = unsafe extern "system" fn(listener: alljoyn_keystorelistener, keystore: alljoyn_keystore, sink: super::super::Foundation::PSTR, sink_sz: *mut usize) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_keystorelistener_putkeys = unsafe extern "system" fn(listener: alljoyn_keystorelistener, keystore: alljoyn_keystore, source: super::super::Foundation::PSTR, password: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_keystorelistener_with_synchronization_create = unsafe extern "system" fn(callbacks: *const alljoyn_keystorelistener_with_synchronization_callbacks, context: *mut ::core::ffi::c_void) -> alljoyn_keystorelistener;
pub type alljoyn_message_create = unsafe extern "system" fn(bus: alljoyn_busattachment) -> alljoyn_message;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_description = unsafe extern "system" fn(msg: alljoyn_message, str: super::super::Foundation::PSTR, buf: usize) -> usize;
pub type alljoyn_message_destroy = unsafe extern "system" fn(msg: alljoyn_message);
pub type alljoyn_message_eql = unsafe extern "system" fn(one: alljoyn_message, other: alljoyn_message) -> i32;
pub type alljoyn_message_getarg = unsafe extern "system" fn(msg: alljoyn_message, argn: usize) -> alljoyn_msgarg;
pub type alljoyn_message_getargs = unsafe extern "system" fn(msg: alljoyn_message, numargs: *mut usize, args: *mut alljoyn_msgarg);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_getauthmechanism = unsafe extern "system" fn(msg: alljoyn_message) -> super::super::Foundation::PSTR;
pub type alljoyn_message_getcallserial = unsafe extern "system" fn(msg: alljoyn_message) -> u32;
pub type alljoyn_message_getcompressiontoken = unsafe extern "system" fn(msg: alljoyn_message) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_getdestination = unsafe extern "system" fn(msg: alljoyn_message) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_geterrorname = unsafe extern "system" fn(msg: alljoyn_message, errormessage: super::super::Foundation::PSTR, errormessage_size: *mut usize) -> super::super::Foundation::PSTR;
pub type alljoyn_message_getflags = unsafe extern "system" fn(msg: alljoyn_message) -> u8;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_getinterface = unsafe extern "system" fn(msg: alljoyn_message) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_getmembername = unsafe extern "system" fn(msg: alljoyn_message) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_getobjectpath = unsafe extern "system" fn(msg: alljoyn_message) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_getreceiveendpointname = unsafe extern "system" fn(msg: alljoyn_message) -> super::super::Foundation::PSTR;
pub type alljoyn_message_getreplyserial = unsafe extern "system" fn(msg: alljoyn_message) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_getsender = unsafe extern "system" fn(msg: alljoyn_message) -> super::super::Foundation::PSTR;
pub type alljoyn_message_getsessionid = unsafe extern "system" fn(msg: alljoyn_message) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_getsignature = unsafe extern "system" fn(msg: alljoyn_message) -> super::super::Foundation::PSTR;
pub type alljoyn_message_gettimestamp = unsafe extern "system" fn(msg: alljoyn_message) -> u32;
pub type alljoyn_message_gettype = unsafe extern "system" fn(msg: alljoyn_message) -> alljoyn_messagetype;
pub type alljoyn_message_isbroadcastsignal = unsafe extern "system" fn(msg: alljoyn_message) -> i32;
pub type alljoyn_message_isencrypted = unsafe extern "system" fn(msg: alljoyn_message) -> i32;
pub type alljoyn_message_isexpired = unsafe extern "system" fn(msg: alljoyn_message, tillexpirems: *mut u32) -> i32;
pub type alljoyn_message_isglobalbroadcast = unsafe extern "system" fn(msg: alljoyn_message) -> i32;
pub type alljoyn_message_issessionless = unsafe extern "system" fn(msg: alljoyn_message) -> i32;
pub type alljoyn_message_isunreliable = unsafe extern "system" fn(msg: alljoyn_message) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_parseargs = unsafe extern "system" fn(msg: alljoyn_message, signature: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_message_setendianess = unsafe extern "system" fn(endian: i8);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_message_tostring = unsafe extern "system" fn(msg: alljoyn_message, str: super::super::Foundation::PSTR, buf: usize) -> usize;
pub type alljoyn_msgarg_array_create = unsafe extern "system" fn(size: usize) -> alljoyn_msgarg;
pub type alljoyn_msgarg_array_element = unsafe extern "system" fn(arg: alljoyn_msgarg, index: usize) -> alljoyn_msgarg;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_array_get = unsafe extern "system" fn(args: alljoyn_msgarg, numargs: usize, signature: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_array_set = unsafe extern "system" fn(args: alljoyn_msgarg, numargs: *mut usize, signature: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_array_set_offset = unsafe extern "system" fn(args: alljoyn_msgarg, argoffset: usize, numargs: *mut usize, signature: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_array_signature = unsafe extern "system" fn(values: alljoyn_msgarg, numvalues: usize, str: super::super::Foundation::PSTR, buf: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_array_tostring = unsafe extern "system" fn(args: alljoyn_msgarg, numargs: usize, str: super::super::Foundation::PSTR, buf: usize, indent: usize) -> usize;
pub type alljoyn_msgarg_clear = unsafe extern "system" fn(arg: alljoyn_msgarg);
pub type alljoyn_msgarg_clone = unsafe extern "system" fn(destination: alljoyn_msgarg, source: alljoyn_msgarg);
pub type alljoyn_msgarg_copy = unsafe extern "system" fn(source: alljoyn_msgarg) -> alljoyn_msgarg;
pub type alljoyn_msgarg_create = unsafe extern "system" fn() -> alljoyn_msgarg;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_create_and_set = unsafe extern "system" fn(signature: super::super::Foundation::PSTR) -> alljoyn_msgarg;
pub type alljoyn_msgarg_destroy = unsafe extern "system" fn(arg: alljoyn_msgarg);
pub type alljoyn_msgarg_equal = unsafe extern "system" fn(lhv: alljoyn_msgarg, rhv: alljoyn_msgarg) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_get = unsafe extern "system" fn(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_msgarg_get_array_element = unsafe extern "system" fn(arg: alljoyn_msgarg, index: usize, element: *mut alljoyn_msgarg);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_get_array_elementsignature = unsafe extern "system" fn(arg: alljoyn_msgarg, index: usize) -> super::super::Foundation::PSTR;
pub type alljoyn_msgarg_get_array_numberofelements = unsafe extern "system" fn(arg: alljoyn_msgarg) -> usize;
pub type alljoyn_msgarg_get_bool = unsafe extern "system" fn(arg: alljoyn_msgarg, b: *mut i32) -> QStatus;
pub type alljoyn_msgarg_get_bool_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: *mut usize, ab: *mut i32) -> QStatus;
pub type alljoyn_msgarg_get_double = unsafe extern "system" fn(arg: alljoyn_msgarg, d: *mut f64) -> QStatus;
pub type alljoyn_msgarg_get_double_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: *mut usize, ad: *mut f64) -> QStatus;
pub type alljoyn_msgarg_get_int16 = unsafe extern "system" fn(arg: alljoyn_msgarg, n: *mut i16) -> QStatus;
pub type alljoyn_msgarg_get_int16_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: *mut usize, an: *mut i16) -> QStatus;
pub type alljoyn_msgarg_get_int32 = unsafe extern "system" fn(arg: alljoyn_msgarg, i: *mut i32) -> QStatus;
pub type alljoyn_msgarg_get_int32_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: *mut usize, ai: *mut i32) -> QStatus;
pub type alljoyn_msgarg_get_int64 = unsafe extern "system" fn(arg: alljoyn_msgarg, x: *mut i64) -> QStatus;
pub type alljoyn_msgarg_get_int64_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: *mut usize, ax: *mut i64) -> QStatus;
pub type alljoyn_msgarg_get_objectpath = unsafe extern "system" fn(arg: alljoyn_msgarg, o: *mut *mut i8) -> QStatus;
pub type alljoyn_msgarg_get_signature = unsafe extern "system" fn(arg: alljoyn_msgarg, g: *mut *mut i8) -> QStatus;
pub type alljoyn_msgarg_get_string = unsafe extern "system" fn(arg: alljoyn_msgarg, s: *mut *mut i8) -> QStatus;
pub type alljoyn_msgarg_get_uint16 = unsafe extern "system" fn(arg: alljoyn_msgarg, q: *mut u16) -> QStatus;
pub type alljoyn_msgarg_get_uint16_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: *mut usize, aq: *mut u16) -> QStatus;
pub type alljoyn_msgarg_get_uint32 = unsafe extern "system" fn(arg: alljoyn_msgarg, u: *mut u32) -> QStatus;
pub type alljoyn_msgarg_get_uint32_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: *mut usize, au: *mut u32) -> QStatus;
pub type alljoyn_msgarg_get_uint64 = unsafe extern "system" fn(arg: alljoyn_msgarg, t: *mut u64) -> QStatus;
pub type alljoyn_msgarg_get_uint64_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: *mut usize, at: *mut u64) -> QStatus;
pub type alljoyn_msgarg_get_uint8 = unsafe extern "system" fn(arg: alljoyn_msgarg, y: *mut u8) -> QStatus;
pub type alljoyn_msgarg_get_uint8_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: *mut usize, ay: *mut u8) -> QStatus;
pub type alljoyn_msgarg_get_variant = unsafe extern "system" fn(arg: alljoyn_msgarg, v: alljoyn_msgarg) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_get_variant_array = unsafe extern "system" fn(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR, length: *mut usize, av: *mut alljoyn_msgarg) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_getdictelement = unsafe extern "system" fn(arg: alljoyn_msgarg, elemsig: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_msgarg_getkey = unsafe extern "system" fn(arg: alljoyn_msgarg) -> alljoyn_msgarg;
pub type alljoyn_msgarg_getmember = unsafe extern "system" fn(arg: alljoyn_msgarg, index: usize) -> alljoyn_msgarg;
pub type alljoyn_msgarg_getnummembers = unsafe extern "system" fn(arg: alljoyn_msgarg) -> usize;
pub type alljoyn_msgarg_gettype = unsafe extern "system" fn(arg: alljoyn_msgarg) -> alljoyn_typeid;
pub type alljoyn_msgarg_getvalue = unsafe extern "system" fn(arg: alljoyn_msgarg) -> alljoyn_msgarg;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_hassignature = unsafe extern "system" fn(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_set = unsafe extern "system" fn(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_set_and_stabilize = unsafe extern "system" fn(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_msgarg_set_bool = unsafe extern "system" fn(arg: alljoyn_msgarg, b: i32) -> QStatus;
pub type alljoyn_msgarg_set_bool_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, ab: *mut i32) -> QStatus;
pub type alljoyn_msgarg_set_double = unsafe extern "system" fn(arg: alljoyn_msgarg, d: f64) -> QStatus;
pub type alljoyn_msgarg_set_double_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, ad: *mut f64) -> QStatus;
pub type alljoyn_msgarg_set_int16 = unsafe extern "system" fn(arg: alljoyn_msgarg, n: i16) -> QStatus;
pub type alljoyn_msgarg_set_int16_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, an: *mut i16) -> QStatus;
pub type alljoyn_msgarg_set_int32 = unsafe extern "system" fn(arg: alljoyn_msgarg, i: i32) -> QStatus;
pub type alljoyn_msgarg_set_int32_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, ai: *mut i32) -> QStatus;
pub type alljoyn_msgarg_set_int64 = unsafe extern "system" fn(arg: alljoyn_msgarg, x: i64) -> QStatus;
pub type alljoyn_msgarg_set_int64_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, ax: *mut i64) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_set_objectpath = unsafe extern "system" fn(arg: alljoyn_msgarg, o: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_msgarg_set_objectpath_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, ao: *const *const i8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_set_signature = unsafe extern "system" fn(arg: alljoyn_msgarg, g: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_msgarg_set_signature_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, ag: *const *const i8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_set_string = unsafe extern "system" fn(arg: alljoyn_msgarg, s: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_msgarg_set_string_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, r#as: *const *const i8) -> QStatus;
pub type alljoyn_msgarg_set_uint16 = unsafe extern "system" fn(arg: alljoyn_msgarg, q: u16) -> QStatus;
pub type alljoyn_msgarg_set_uint16_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, aq: *mut u16) -> QStatus;
pub type alljoyn_msgarg_set_uint32 = unsafe extern "system" fn(arg: alljoyn_msgarg, u: u32) -> QStatus;
pub type alljoyn_msgarg_set_uint32_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, au: *mut u32) -> QStatus;
pub type alljoyn_msgarg_set_uint64 = unsafe extern "system" fn(arg: alljoyn_msgarg, t: u64) -> QStatus;
pub type alljoyn_msgarg_set_uint64_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, at: *mut u64) -> QStatus;
pub type alljoyn_msgarg_set_uint8 = unsafe extern "system" fn(arg: alljoyn_msgarg, y: u8) -> QStatus;
pub type alljoyn_msgarg_set_uint8_array = unsafe extern "system" fn(arg: alljoyn_msgarg, length: usize, ay: *mut u8) -> QStatus;
pub type alljoyn_msgarg_setdictentry = unsafe extern "system" fn(arg: alljoyn_msgarg, key: alljoyn_msgarg, value: alljoyn_msgarg) -> QStatus;
pub type alljoyn_msgarg_setstruct = unsafe extern "system" fn(arg: alljoyn_msgarg, struct_members: alljoyn_msgarg, num_members: usize) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_signature = unsafe extern "system" fn(arg: alljoyn_msgarg, str: super::super::Foundation::PSTR, buf: usize) -> usize;
pub type alljoyn_msgarg_stabilize = unsafe extern "system" fn(arg: alljoyn_msgarg);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_msgarg_tostring = unsafe extern "system" fn(arg: alljoyn_msgarg, str: super::super::Foundation::PSTR, buf: usize, indent: usize) -> usize;
pub type alljoyn_observer_create = unsafe extern "system" fn(bus: alljoyn_busattachment, mandatoryinterfaces: *const *const i8, nummandatoryinterfaces: usize) -> alljoyn_observer;
pub type alljoyn_observer_destroy = unsafe extern "system" fn(observer: alljoyn_observer);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_observer_get = unsafe extern "system" fn(observer: alljoyn_observer, uniquebusname: super::super::Foundation::PSTR, objectpath: super::super::Foundation::PSTR) -> alljoyn_proxybusobject_ref;
pub type alljoyn_observer_getfirst = unsafe extern "system" fn(observer: alljoyn_observer) -> alljoyn_proxybusobject_ref;
pub type alljoyn_observer_getnext = unsafe extern "system" fn(observer: alljoyn_observer, proxyref: alljoyn_proxybusobject_ref) -> alljoyn_proxybusobject_ref;
pub type alljoyn_observer_registerlistener = unsafe extern "system" fn(observer: alljoyn_observer, listener: alljoyn_observerlistener, triggeronexisting: i32);
pub type alljoyn_observer_unregisteralllisteners = unsafe extern "system" fn(observer: alljoyn_observer);
pub type alljoyn_observer_unregisterlistener = unsafe extern "system" fn(observer: alljoyn_observer, listener: alljoyn_observerlistener);
pub type alljoyn_observerlistener_create = unsafe extern "system" fn(callback: *const alljoyn_observerlistener_callback, context: *const ::core::ffi::c_void) -> alljoyn_observerlistener;
pub type alljoyn_observerlistener_destroy = unsafe extern "system" fn(listener: alljoyn_observerlistener);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_passwordmanager_setcredentials = unsafe extern "system" fn(authmechanism: super::super::Foundation::PSTR, password: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_permissionconfigurationlistener_create = unsafe extern "system" fn(callbacks: *const alljoyn_permissionconfigurationlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_permissionconfigurationlistener;
pub type alljoyn_permissionconfigurationlistener_destroy = unsafe extern "system" fn(listener: alljoyn_permissionconfigurationlistener);
pub type alljoyn_permissionconfigurator_certificatechain_destroy = unsafe extern "system" fn(certificatechain: *mut i8);
pub type alljoyn_permissionconfigurator_certificateid_cleanup = unsafe extern "system" fn(certificateid: *mut alljoyn_certificateid);
pub type alljoyn_permissionconfigurator_certificateidarray_cleanup = unsafe extern "system" fn(certificateidarray: *mut alljoyn_certificateidarray);
pub type alljoyn_permissionconfigurator_claim = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, cakey: *mut i8, identitycertificatechain: *mut i8, groupid: *const u8, groupsize: usize, groupauthority: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus;
pub type alljoyn_permissionconfigurator_endmanagement = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator) -> QStatus;
pub type alljoyn_permissionconfigurator_getapplicationstate = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, state: *mut alljoyn_applicationstate) -> QStatus;
pub type alljoyn_permissionconfigurator_getclaimcapabilities = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, claimcapabilities: *mut u16) -> QStatus;
pub type alljoyn_permissionconfigurator_getclaimcapabilitiesadditionalinfo = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, additionalinfo: *mut u16) -> QStatus;
pub type alljoyn_permissionconfigurator_getdefaultclaimcapabilities = unsafe extern "system" fn() -> u16;
pub type alljoyn_permissionconfigurator_getdefaultpolicy = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, policyxml: *mut *mut i8) -> QStatus;
pub type alljoyn_permissionconfigurator_getidentity = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, identitycertificatechain: *mut *mut i8) -> QStatus;
pub type alljoyn_permissionconfigurator_getidentitycertificateid = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, certificateid: *mut alljoyn_certificateid) -> QStatus;
pub type alljoyn_permissionconfigurator_getmanifests = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, manifestarray: *mut alljoyn_manifestarray) -> QStatus;
pub type alljoyn_permissionconfigurator_getmanifesttemplate = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, manifesttemplatexml: *mut *mut i8) -> QStatus;
pub type alljoyn_permissionconfigurator_getmembershipsummaries = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, certificateids: *mut alljoyn_certificateidarray) -> QStatus;
pub type alljoyn_permissionconfigurator_getpolicy = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, policyxml: *mut *mut i8) -> QStatus;
pub type alljoyn_permissionconfigurator_getpublickey = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, publickey: *mut *mut i8) -> QStatus;
pub type alljoyn_permissionconfigurator_installmanifests = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, manifestsxmls: *mut *mut i8, manifestscount: usize, append: i32) -> QStatus;
pub type alljoyn_permissionconfigurator_installmembership = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, membershipcertificatechain: *mut i8) -> QStatus;
pub type alljoyn_permissionconfigurator_manifestarray_cleanup = unsafe extern "system" fn(manifestarray: *mut alljoyn_manifestarray);
pub type alljoyn_permissionconfigurator_manifesttemplate_destroy = unsafe extern "system" fn(manifesttemplatexml: *mut i8);
pub type alljoyn_permissionconfigurator_policy_destroy = unsafe extern "system" fn(policyxml: *mut i8);
pub type alljoyn_permissionconfigurator_publickey_destroy = unsafe extern "system" fn(publickey: *mut i8);
pub type alljoyn_permissionconfigurator_removemembership = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, serial: *const u8, seriallen: usize, issuerpublickey: *mut i8, issueraki: *const u8, issuerakilen: usize) -> QStatus;
pub type alljoyn_permissionconfigurator_reset = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator) -> QStatus;
pub type alljoyn_permissionconfigurator_resetpolicy = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator) -> QStatus;
pub type alljoyn_permissionconfigurator_setapplicationstate = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, state: alljoyn_applicationstate) -> QStatus;
pub type alljoyn_permissionconfigurator_setclaimcapabilities = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, claimcapabilities: u16) -> QStatus;
pub type alljoyn_permissionconfigurator_setclaimcapabilitiesadditionalinfo = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, additionalinfo: u16) -> QStatus;
pub type alljoyn_permissionconfigurator_setmanifesttemplatefromxml = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, manifesttemplatexml: *mut i8) -> QStatus;
pub type alljoyn_permissionconfigurator_startmanagement = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator) -> QStatus;
pub type alljoyn_permissionconfigurator_updateidentity = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, identitycertificatechain: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus;
pub type alljoyn_permissionconfigurator_updatepolicy = unsafe extern "system" fn(configurator: alljoyn_permissionconfigurator, policyxml: *mut i8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_pinglistener_create = unsafe extern "system" fn(callback: *const alljoyn_pinglistener_callback, context: *const ::core::ffi::c_void) -> alljoyn_pinglistener;
pub type alljoyn_pinglistener_destroy = unsafe extern "system" fn(listener: alljoyn_pinglistener);
pub type alljoyn_proxybusobject_addchild = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, child: alljoyn_proxybusobject) -> QStatus;
pub type alljoyn_proxybusobject_addinterface = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: alljoyn_interfacedescription) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_addinterface_by_name = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, name: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_proxybusobject_copy = unsafe extern "system" fn(source: alljoyn_proxybusobject) -> alljoyn_proxybusobject;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_create = unsafe extern "system" fn(bus: alljoyn_busattachment, service: super::super::Foundation::PSTR, path: super::super::Foundation::PSTR, sessionid: u32) -> alljoyn_proxybusobject;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_create_secure = unsafe extern "system" fn(bus: alljoyn_busattachment, service: super::super::Foundation::PSTR, path: super::super::Foundation::PSTR, sessionid: u32) -> alljoyn_proxybusobject;
pub type alljoyn_proxybusobject_destroy = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject);
pub type alljoyn_proxybusobject_enablepropertycaching = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_getallproperties = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, values: alljoyn_msgarg) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_getallpropertiesasync = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, callback: alljoyn_proxybusobject_listener_getallpropertiescb_ptr, timeout: u32, context: *mut ::core::ffi::c_void) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_getchild = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, path: super::super::Foundation::PSTR) -> alljoyn_proxybusobject;
pub type alljoyn_proxybusobject_getchildren = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, children: *mut alljoyn_proxybusobject, numchildren: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_getinterface = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR) -> alljoyn_interfacedescription;
pub type alljoyn_proxybusobject_getinterfaces = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, ifaces: *const alljoyn_interfacedescription, numifaces: usize) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_getpath = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_getproperty = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, property: super::super::Foundation::PSTR, value: alljoyn_msgarg) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_getpropertyasync = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, property: super::super::Foundation::PSTR, callback: alljoyn_proxybusobject_listener_getpropertycb_ptr, timeout: u32, context: *mut ::core::ffi::c_void) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_getservicename = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject) -> super::super::Foundation::PSTR;
pub type alljoyn_proxybusobject_getsessionid = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_getuniquename = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject) -> super::super::Foundation::PSTR;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_implementsinterface = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR) -> i32;
pub type alljoyn_proxybusobject_introspectremoteobject = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject) -> QStatus;
pub type alljoyn_proxybusobject_introspectremoteobjectasync = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, callback: alljoyn_proxybusobject_listener_introspectcb_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
pub type alljoyn_proxybusobject_issecure = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject) -> i32;
pub type alljoyn_proxybusobject_isvalid = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_methodcall = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, ifacename: super::super::Foundation::PSTR, methodname: super::super::Foundation::PSTR, args: alljoyn_msgarg, numargs: usize, replymsg: alljoyn_message, timeout: u32, flags: u8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_methodcall_member = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, method: alljoyn_interfacedescription_member, args: alljoyn_msgarg, numargs: usize, replymsg: alljoyn_message, timeout: u32, flags: u8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_methodcall_member_noreply = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, method: alljoyn_interfacedescription_member, args: alljoyn_msgarg, numargs: usize, flags: u8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_methodcall_noreply = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, ifacename: super::super::Foundation::PSTR, methodname: super::super::Foundation::PSTR, args: alljoyn_msgarg, numargs: usize, flags: u8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_methodcallasync = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, ifacename: super::super::Foundation::PSTR, methodname: super::super::Foundation::PSTR, replyfunc: alljoyn_messagereceiver_replyhandler_ptr, args: alljoyn_msgarg, numargs: usize, context: *mut ::core::ffi::c_void, timeout: u32, flags: u8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_methodcallasync_member = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, method: alljoyn_interfacedescription_member, replyfunc: alljoyn_messagereceiver_replyhandler_ptr, args: alljoyn_msgarg, numargs: usize, context: *mut ::core::ffi::c_void, timeout: u32, flags: u8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_parsexml = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, xml: super::super::Foundation::PSTR, identifier: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_proxybusobject_ref_create = unsafe extern "system" fn(proxy: alljoyn_proxybusobject) -> alljoyn_proxybusobject_ref;
pub type alljoyn_proxybusobject_ref_decref = unsafe extern "system" fn(r#ref: alljoyn_proxybusobject_ref);
pub type alljoyn_proxybusobject_ref_get = unsafe extern "system" fn(r#ref: alljoyn_proxybusobject_ref) -> alljoyn_proxybusobject;
pub type alljoyn_proxybusobject_ref_incref = unsafe extern "system" fn(r#ref: alljoyn_proxybusobject_ref);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_registerpropertieschangedlistener = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, properties: *const *const i8, numproperties: usize, callback: alljoyn_proxybusobject_listener_propertieschanged_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_removechild = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, path: super::super::Foundation::PSTR) -> QStatus;
pub type alljoyn_proxybusobject_secureconnection = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, forceauth: i32) -> QStatus;
pub type alljoyn_proxybusobject_secureconnectionasync = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, forceauth: i32) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_setproperty = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, property: super::super::Foundation::PSTR, value: alljoyn_msgarg) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_setpropertyasync = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, property: super::super::Foundation::PSTR, value: alljoyn_msgarg, callback: alljoyn_proxybusobject_listener_setpropertycb_ptr, timeout: u32, context: *mut ::core::ffi::c_void) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_unregisterpropertieschangedlistener = unsafe extern "system" fn(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, callback: alljoyn_proxybusobject_listener_propertieschanged_ptr) -> QStatus;
pub type alljoyn_routerinit = unsafe extern "system" fn() -> QStatus;
pub type alljoyn_routerinitwithconfig = unsafe extern "system" fn(configxml: *mut i8) -> QStatus;
pub type alljoyn_routershutdown = unsafe extern "system" fn() -> QStatus;
pub type alljoyn_securityapplicationproxy_claim = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, cakey: *mut i8, identitycertificatechain: *mut i8, groupid: *const u8, groupsize: usize, groupauthority: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus;
pub type alljoyn_securityapplicationproxy_computemanifestdigest = unsafe extern "system" fn(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, digest: *mut *mut u8, digestsize: *mut usize) -> QStatus;
pub type alljoyn_securityapplicationproxy_create = unsafe extern "system" fn(bus: alljoyn_busattachment, appbusname: *mut i8, sessionid: u32) -> alljoyn_securityapplicationproxy;
pub type alljoyn_securityapplicationproxy_destroy = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy);
pub type alljoyn_securityapplicationproxy_digest_destroy = unsafe extern "system" fn(digest: *mut u8);
pub type alljoyn_securityapplicationproxy_eccpublickey_destroy = unsafe extern "system" fn(eccpublickey: *mut i8);
pub type alljoyn_securityapplicationproxy_endmanagement = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy) -> QStatus;
pub type alljoyn_securityapplicationproxy_getapplicationstate = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, applicationstate: *mut alljoyn_applicationstate) -> QStatus;
pub type alljoyn_securityapplicationproxy_getclaimcapabilities = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, capabilities: *mut u16) -> QStatus;
pub type alljoyn_securityapplicationproxy_getclaimcapabilitiesadditionalinfo = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, additionalinfo: *mut u16) -> QStatus;
pub type alljoyn_securityapplicationproxy_getdefaultpolicy = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, policyxml: *mut *mut i8) -> QStatus;
pub type alljoyn_securityapplicationproxy_geteccpublickey = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, eccpublickey: *mut *mut i8) -> QStatus;
pub type alljoyn_securityapplicationproxy_getmanifesttemplate = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, manifesttemplatexml: *mut *mut i8) -> QStatus;
pub type alljoyn_securityapplicationproxy_getpermissionmanagementsessionport = unsafe extern "system" fn() -> u16;
pub type alljoyn_securityapplicationproxy_getpolicy = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, policyxml: *mut *mut i8) -> QStatus;
pub type alljoyn_securityapplicationproxy_installmembership = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, membershipcertificatechain: *mut i8) -> QStatus;
pub type alljoyn_securityapplicationproxy_manifest_destroy = unsafe extern "system" fn(signedmanifestxml: *mut i8);
pub type alljoyn_securityapplicationproxy_manifesttemplate_destroy = unsafe extern "system" fn(manifesttemplatexml: *mut i8);
pub type alljoyn_securityapplicationproxy_policy_destroy = unsafe extern "system" fn(policyxml: *mut i8);
pub type alljoyn_securityapplicationproxy_reset = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy) -> QStatus;
pub type alljoyn_securityapplicationproxy_resetpolicy = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy) -> QStatus;
pub type alljoyn_securityapplicationproxy_setmanifestsignature = unsafe extern "system" fn(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, signature: *const u8, signaturesize: usize, signedmanifestxml: *mut *mut i8) -> QStatus;
pub type alljoyn_securityapplicationproxy_signmanifest = unsafe extern "system" fn(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, signingprivatekeypem: *mut i8, signedmanifestxml: *mut *mut i8) -> QStatus;
pub type alljoyn_securityapplicationproxy_startmanagement = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy) -> QStatus;
pub type alljoyn_securityapplicationproxy_updateidentity = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, identitycertificatechain: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus;
pub type alljoyn_securityapplicationproxy_updatepolicy = unsafe extern "system" fn(proxy: alljoyn_securityapplicationproxy, policyxml: *mut i8) -> QStatus;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_sessionlistener_create = unsafe extern "system" fn(callbacks: *const alljoyn_sessionlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_sessionlistener;
pub type alljoyn_sessionlistener_destroy = unsafe extern "system" fn(listener: alljoyn_sessionlistener);
pub type alljoyn_sessionopts_cmp = unsafe extern "system" fn(one: alljoyn_sessionopts, other: alljoyn_sessionopts) -> i32;
pub type alljoyn_sessionopts_create = unsafe extern "system" fn(traffic: u8, ismultipoint: i32, proximity: u8, transports: u16) -> alljoyn_sessionopts;
pub type alljoyn_sessionopts_destroy = unsafe extern "system" fn(opts: alljoyn_sessionopts);
pub type alljoyn_sessionopts_get_multipoint = unsafe extern "system" fn(opts: alljoyn_sessionopts) -> i32;
pub type alljoyn_sessionopts_get_proximity = unsafe extern "system" fn(opts: alljoyn_sessionopts) -> u8;
pub type alljoyn_sessionopts_get_traffic = unsafe extern "system" fn(opts: alljoyn_sessionopts) -> u8;
pub type alljoyn_sessionopts_get_transports = unsafe extern "system" fn(opts: alljoyn_sessionopts) -> u16;
pub type alljoyn_sessionopts_iscompatible = unsafe extern "system" fn(one: alljoyn_sessionopts, other: alljoyn_sessionopts) -> i32;
pub type alljoyn_sessionopts_set_multipoint = unsafe extern "system" fn(opts: alljoyn_sessionopts, ismultipoint: i32);
pub type alljoyn_sessionopts_set_proximity = unsafe extern "system" fn(opts: alljoyn_sessionopts, proximity: u8);
pub type alljoyn_sessionopts_set_traffic = unsafe extern "system" fn(opts: alljoyn_sessionopts, traffic: u8);
pub type alljoyn_sessionopts_set_transports = unsafe extern "system" fn(opts: alljoyn_sessionopts, transports: u16);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_sessionportlistener_create = unsafe extern "system" fn(callbacks: *const alljoyn_sessionportlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_sessionportlistener;
pub type alljoyn_sessionportlistener_destroy = unsafe extern "system" fn(listener: alljoyn_sessionportlistener);
pub type alljoyn_shutdown = unsafe extern "system" fn() -> QStatus;
pub type alljoyn_unity_deferred_callbacks_process = unsafe extern "system" fn() -> i32;
pub type alljoyn_unity_set_deferred_callback_mainthread_only = unsafe extern "system" fn(mainthread_only: i32);
pub const ALLJOYN_BIG_ENDIAN: u8 = 66u8;
pub const ALLJOYN_CRED_CERT_CHAIN: u16 = 4u16;
pub const ALLJOYN_CRED_EXPIRATION: u16 = 32u16;
pub const ALLJOYN_CRED_LOGON_ENTRY: u16 = 16u16;
pub const ALLJOYN_CRED_NEW_PASSWORD: u16 = 4097u16;
pub const ALLJOYN_CRED_ONE_TIME_PWD: u16 = 8193u16;
pub const ALLJOYN_CRED_PASSWORD: u16 = 1u16;
pub const ALLJOYN_CRED_PRIVATE_KEY: u16 = 8u16;
pub const ALLJOYN_CRED_USER_NAME: u16 = 2u16;
pub const ALLJOYN_DISCONNECTED: u32 = 4u32;
pub const ALLJOYN_LITTLE_ENDIAN: u8 = 108u8;
pub const ALLJOYN_MEMBER_ANNOTATE_DEPRECATED: u8 = 2u8;
pub const ALLJOYN_MEMBER_ANNOTATE_GLOBAL_BROADCAST: u8 = 32u8;
pub const ALLJOYN_MEMBER_ANNOTATE_NO_REPLY: u8 = 1u8;
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONCAST: u8 = 4u8;
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONLESS: u8 = 8u8;
pub const ALLJOYN_MEMBER_ANNOTATE_UNICAST: u8 = 16u8;
pub const ALLJOYN_MESSAGE_DEFAULT_TIMEOUT: u32 = 25000u32;
pub const ALLJOYN_MESSAGE_FLAG_ALLOW_REMOTE_MSG: u32 = 4u32;
pub const ALLJOYN_MESSAGE_FLAG_AUTO_START: u32 = 2u32;
pub const ALLJOYN_MESSAGE_FLAG_ENCRYPTED: u32 = 128u32;
pub const ALLJOYN_MESSAGE_FLAG_GLOBAL_BROADCAST: u32 = 32u32;
pub const ALLJOYN_MESSAGE_FLAG_NO_REPLY_EXPECTED: u32 = 1u32;
pub const ALLJOYN_MESSAGE_FLAG_SESSIONLESS: u32 = 16u32;
pub const ALLJOYN_PROP_ACCESS_READ: u8 = 1u8;
pub const ALLJOYN_PROP_ACCESS_RW: u8 = 3u8;
pub const ALLJOYN_PROP_ACCESS_WRITE: u8 = 2u8;
pub const ALLJOYN_PROXIMITY_ANY: u32 = 255u32;
pub const ALLJOYN_PROXIMITY_NETWORK: u32 = 2u32;
pub const ALLJOYN_PROXIMITY_PHYSICAL: u32 = 1u32;
pub const ALLJOYN_READ_READY: u32 = 1u32;
pub const ALLJOYN_TRAFFIC_TYPE_MESSAGES: u32 = 1u32;
pub const ALLJOYN_TRAFFIC_TYPE_RAW_RELIABLE: u32 = 4u32;
pub const ALLJOYN_TRAFFIC_TYPE_RAW_UNRELIABLE: u32 = 2u32;
pub const ALLJOYN_WRITE_READY: u32 = 2u32;
pub const QCC_FALSE: u32 = 0u32;
pub const QCC_TRUE: u32 = 1u32;
pub type QStatus = i32;
pub const ER_OK: QStatus = 0i32;
pub const ER_FAIL: QStatus = 1i32;
pub const ER_UTF_CONVERSION_FAILED: QStatus = 2i32;
pub const ER_BUFFER_TOO_SMALL: QStatus = 3i32;
pub const ER_OS_ERROR: QStatus = 4i32;
pub const ER_OUT_OF_MEMORY: QStatus = 5i32;
pub const ER_SOCKET_BIND_ERROR: QStatus = 6i32;
pub const ER_INIT_FAILED: QStatus = 7i32;
pub const ER_WOULDBLOCK: QStatus = 8i32;
pub const ER_NOT_IMPLEMENTED: QStatus = 9i32;
pub const ER_TIMEOUT: QStatus = 10i32;
pub const ER_SOCK_OTHER_END_CLOSED: QStatus = 11i32;
pub const ER_BAD_ARG_1: QStatus = 12i32;
pub const ER_BAD_ARG_2: QStatus = 13i32;
pub const ER_BAD_ARG_3: QStatus = 14i32;
pub const ER_BAD_ARG_4: QStatus = 15i32;
pub const ER_BAD_ARG_5: QStatus = 16i32;
pub const ER_BAD_ARG_6: QStatus = 17i32;
pub const ER_BAD_ARG_7: QStatus = 18i32;
pub const ER_BAD_ARG_8: QStatus = 19i32;
pub const ER_INVALID_ADDRESS: QStatus = 20i32;
pub const ER_INVALID_DATA: QStatus = 21i32;
pub const ER_READ_ERROR: QStatus = 22i32;
pub const ER_WRITE_ERROR: QStatus = 23i32;
pub const ER_OPEN_FAILED: QStatus = 24i32;
pub const ER_PARSE_ERROR: QStatus = 25i32;
pub const ER_END_OF_DATA: QStatus = 26i32;
pub const ER_CONN_REFUSED: QStatus = 27i32;
pub const ER_BAD_ARG_COUNT: QStatus = 28i32;
pub const ER_WARNING: QStatus = 29i32;
pub const ER_EOF: QStatus = 30i32;
pub const ER_DEADLOCK: QStatus = 31i32;
pub const ER_COMMON_ERRORS: QStatus = 4096i32;
pub const ER_STOPPING_THREAD: QStatus = 4097i32;
pub const ER_ALERTED_THREAD: QStatus = 4098i32;
pub const ER_XML_MALFORMED: QStatus = 4099i32;
pub const ER_AUTH_FAIL: QStatus = 4100i32;
pub const ER_AUTH_USER_REJECT: QStatus = 4101i32;
pub const ER_NO_SUCH_ALARM: QStatus = 4102i32;
pub const ER_TIMER_FALLBEHIND: QStatus = 4103i32;
pub const ER_SSL_ERRORS: QStatus = 4104i32;
pub const ER_SSL_INIT: QStatus = 4105i32;
pub const ER_SSL_CONNECT: QStatus = 4106i32;
pub const ER_SSL_VERIFY: QStatus = 4107i32;
pub const ER_EXTERNAL_THREAD: QStatus = 4108i32;
pub const ER_CRYPTO_ERROR: QStatus = 4109i32;
pub const ER_CRYPTO_TRUNCATED: QStatus = 4110i32;
pub const ER_CRYPTO_KEY_UNAVAILABLE: QStatus = 4111i32;
pub const ER_BAD_HOSTNAME: QStatus = 4112i32;
pub const ER_CRYPTO_KEY_UNUSABLE: QStatus = 4113i32;
pub const ER_EMPTY_KEY_BLOB: QStatus = 4114i32;
pub const ER_CORRUPT_KEYBLOB: QStatus = 4115i32;
pub const ER_INVALID_KEY_ENCODING: QStatus = 4116i32;
pub const ER_DEAD_THREAD: QStatus = 4117i32;
pub const ER_THREAD_RUNNING: QStatus = 4118i32;
pub const ER_THREAD_STOPPING: QStatus = 4119i32;
pub const ER_BAD_STRING_ENCODING: QStatus = 4120i32;
pub const ER_CRYPTO_INSUFFICIENT_SECURITY: QStatus = 4121i32;
pub const ER_CRYPTO_ILLEGAL_PARAMETERS: QStatus = 4122i32;
pub const ER_CRYPTO_HASH_UNINITIALIZED: QStatus = 4123i32;
pub const ER_THREAD_NO_WAIT: QStatus = 4124i32;
pub const ER_TIMER_EXITING: QStatus = 4125i32;
pub const ER_INVALID_GUID: QStatus = 4126i32;
pub const ER_THREADPOOL_EXHAUSTED: QStatus = 4127i32;
pub const ER_THREADPOOL_STOPPING: QStatus = 4128i32;
pub const ER_INVALID_STREAM: QStatus = 4129i32;
pub const ER_TIMER_FULL: QStatus = 4130i32;
pub const ER_IODISPATCH_STOPPING: QStatus = 4131i32;
pub const ER_SLAP_INVALID_PACKET_LEN: QStatus = 4132i32;
pub const ER_SLAP_HDR_CHECKSUM_ERROR: QStatus = 4133i32;
pub const ER_SLAP_INVALID_PACKET_TYPE: QStatus = 4134i32;
pub const ER_SLAP_LEN_MISMATCH: QStatus = 4135i32;
pub const ER_SLAP_PACKET_TYPE_MISMATCH: QStatus = 4136i32;
pub const ER_SLAP_CRC_ERROR: QStatus = 4137i32;
pub const ER_SLAP_ERROR: QStatus = 4138i32;
pub const ER_SLAP_OTHER_END_CLOSED: QStatus = 4139i32;
pub const ER_TIMER_NOT_ALLOWED: QStatus = 4140i32;
pub const ER_NOT_CONN: QStatus = 4141i32;
pub const ER_XML_CONVERTER_ERROR: QStatus = 8192i32;
pub const ER_XML_INVALID_RULES_COUNT: QStatus = 8193i32;
pub const ER_XML_INTERFACE_MEMBERS_MISSING: QStatus = 8194i32;
pub const ER_XML_INVALID_MEMBER_TYPE: QStatus = 8195i32;
pub const ER_XML_INVALID_MEMBER_ACTION: QStatus = 8196i32;
pub const ER_XML_MEMBER_DENY_ACTION_WITH_OTHER: QStatus = 8197i32;
pub const ER_XML_INVALID_ANNOTATIONS_COUNT: QStatus = 8198i32;
pub const ER_XML_INVALID_ELEMENT_NAME: QStatus = 8199i32;
pub const ER_XML_INVALID_ATTRIBUTE_VALUE: QStatus = 8200i32;
pub const ER_XML_INVALID_SECURITY_LEVEL_ANNOTATION_VALUE: QStatus = 8201i32;
pub const ER_XML_INVALID_ELEMENT_CHILDREN_COUNT: QStatus = 8202i32;
pub const ER_XML_INVALID_POLICY_VERSION: QStatus = 8203i32;
pub const ER_XML_INVALID_POLICY_SERIAL_NUMBER: QStatus = 8204i32;
pub const ER_XML_INVALID_ACL_PEER_TYPE: QStatus = 8205i32;
pub const ER_XML_INVALID_ACL_PEER_CHILDREN_COUNT: QStatus = 8206i32;
pub const ER_XML_ACL_ALL_TYPE_PEER_WITH_OTHERS: QStatus = 8207i32;
pub const ER_XML_INVALID_ACL_PEER_PUBLIC_KEY: QStatus = 8208i32;
pub const ER_XML_ACL_PEER_NOT_UNIQUE: QStatus = 8209i32;
pub const ER_XML_ACL_PEER_PUBLIC_KEY_SET: QStatus = 8210i32;
pub const ER_XML_ACLS_MISSING: QStatus = 8211i32;
pub const ER_XML_ACL_PEERS_MISSING: QStatus = 8212i32;
pub const ER_XML_INVALID_OBJECT_PATH: QStatus = 8213i32;
pub const ER_XML_INVALID_INTERFACE_NAME: QStatus = 8214i32;
pub const ER_XML_INVALID_MEMBER_NAME: QStatus = 8215i32;
pub const ER_XML_INVALID_MANIFEST_VERSION: QStatus = 8216i32;
pub const ER_XML_INVALID_OID: QStatus = 8217i32;
pub const ER_XML_INVALID_BASE64: QStatus = 8218i32;
pub const ER_XML_INTERFACE_NAME_NOT_UNIQUE: QStatus = 8219i32;
pub const ER_XML_MEMBER_NAME_NOT_UNIQUE: QStatus = 8220i32;
pub const ER_XML_OBJECT_PATH_NOT_UNIQUE: QStatus = 8221i32;
pub const ER_XML_ANNOTATION_NOT_UNIQUE: QStatus = 8222i32;
pub const ER_NONE: QStatus = 65535i32;
pub const ER_BUS_ERRORS: QStatus = 36864i32;
pub const ER_BUS_READ_ERROR: QStatus = 36865i32;
pub const ER_BUS_WRITE_ERROR: QStatus = 36866i32;
pub const ER_BUS_BAD_VALUE_TYPE: QStatus = 36867i32;
pub const ER_BUS_BAD_HEADER_FIELD: QStatus = 36868i32;
pub const ER_BUS_BAD_SIGNATURE: QStatus = 36869i32;
pub const ER_BUS_BAD_OBJ_PATH: QStatus = 36870i32;
pub const ER_BUS_BAD_MEMBER_NAME: QStatus = 36871i32;
pub const ER_BUS_BAD_INTERFACE_NAME: QStatus = 36872i32;
pub const ER_BUS_BAD_ERROR_NAME: QStatus = 36873i32;
pub const ER_BUS_BAD_BUS_NAME: QStatus = 36874i32;
pub const ER_BUS_NAME_TOO_LONG: QStatus = 36875i32;
pub const ER_BUS_BAD_LENGTH: QStatus = 36876i32;
pub const ER_BUS_BAD_VALUE: QStatus = 36877i32;
pub const ER_BUS_BAD_HDR_FLAGS: QStatus = 36878i32;
pub const ER_BUS_BAD_BODY_LEN: QStatus = 36879i32;
pub const ER_BUS_BAD_HEADER_LEN: QStatus = 36880i32;
pub const ER_BUS_UNKNOWN_SERIAL: QStatus = 36881i32;
pub const ER_BUS_UNKNOWN_PATH: QStatus = 36882i32;
pub const ER_BUS_UNKNOWN_INTERFACE: QStatus = 36883i32;
pub const ER_BUS_ESTABLISH_FAILED: QStatus = 36884i32;
pub const ER_BUS_UNEXPECTED_SIGNATURE: QStatus = 36885i32;
pub const ER_BUS_INTERFACE_MISSING: QStatus = 36886i32;
pub const ER_BUS_PATH_MISSING: QStatus = 36887i32;
pub const ER_BUS_MEMBER_MISSING: QStatus = 36888i32;
pub const ER_BUS_REPLY_SERIAL_MISSING: QStatus = 36889i32;
pub const ER_BUS_ERROR_NAME_MISSING: QStatus = 36890i32;
pub const ER_BUS_INTERFACE_NO_SUCH_MEMBER: QStatus = 36891i32;
pub const ER_BUS_NO_SUCH_OBJECT: QStatus = 36892i32;
pub const ER_BUS_OBJECT_NO_SUCH_MEMBER: QStatus = 36893i32;
pub const ER_BUS_OBJECT_NO_SUCH_INTERFACE: QStatus = 36894i32;
pub const ER_BUS_NO_SUCH_INTERFACE: QStatus = 36895i32;
pub const ER_BUS_MEMBER_NO_SUCH_SIGNATURE: QStatus = 36896i32;
pub const ER_BUS_NOT_NUL_TERMINATED: QStatus = 36897i32;
pub const ER_BUS_NO_SUCH_PROPERTY: QStatus = 36898i32;
pub const ER_BUS_SET_WRONG_SIGNATURE: QStatus = 36899i32;
pub const ER_BUS_PROPERTY_VALUE_NOT_SET: QStatus = 36900i32;
pub const ER_BUS_PROPERTY_ACCESS_DENIED: QStatus = 36901i32;
pub const ER_BUS_NO_TRANSPORTS: QStatus = 36902i32;
pub const ER_BUS_BAD_TRANSPORT_ARGS: QStatus = 36903i32;
pub const ER_BUS_NO_ROUTE: QStatus = 36904i32;
pub const ER_BUS_NO_ENDPOINT: QStatus = 36905i32;
pub const ER_BUS_BAD_SEND_PARAMETER: QStatus = 36906i32;
pub const ER_BUS_UNMATCHED_REPLY_SERIAL: QStatus = 36907i32;
pub const ER_BUS_BAD_SENDER_ID: QStatus = 36908i32;
pub const ER_BUS_TRANSPORT_NOT_STARTED: QStatus = 36909i32;
pub const ER_BUS_EMPTY_MESSAGE: QStatus = 36910i32;
pub const ER_BUS_NOT_OWNER: QStatus = 36911i32;
pub const ER_BUS_SET_PROPERTY_REJECTED: QStatus = 36912i32;
pub const ER_BUS_CONNECT_FAILED: QStatus = 36913i32;
pub const ER_BUS_REPLY_IS_ERROR_MESSAGE: QStatus = 36914i32;
pub const ER_BUS_NOT_AUTHENTICATING: QStatus = 36915i32;
pub const ER_BUS_NO_LISTENER: QStatus = 36916i32;
pub const ER_BUS_NOT_ALLOWED: QStatus = 36918i32;
pub const ER_BUS_WRITE_QUEUE_FULL: QStatus = 36919i32;
pub const ER_BUS_ENDPOINT_CLOSING: QStatus = 36920i32;
pub const ER_BUS_INTERFACE_MISMATCH: QStatus = 36921i32;
pub const ER_BUS_MEMBER_ALREADY_EXISTS: QStatus = 36922i32;
pub const ER_BUS_PROPERTY_ALREADY_EXISTS: QStatus = 36923i32;
pub const ER_BUS_IFACE_ALREADY_EXISTS: QStatus = 36924i32;
pub const ER_BUS_ERROR_RESPONSE: QStatus = 36925i32;
pub const ER_BUS_BAD_XML: QStatus = 36926i32;
pub const ER_BUS_BAD_CHILD_PATH: QStatus = 36927i32;
pub const ER_BUS_OBJ_ALREADY_EXISTS: QStatus = 36928i32;
pub const ER_BUS_OBJ_NOT_FOUND: QStatus = 36929i32;
pub const ER_BUS_CANNOT_EXPAND_MESSAGE: QStatus = 36930i32;
pub const ER_BUS_NOT_COMPRESSED: QStatus = 36931i32;
pub const ER_BUS_ALREADY_CONNECTED: QStatus = 36932i32;
pub const ER_BUS_NOT_CONNECTED: QStatus = 36933i32;
pub const ER_BUS_ALREADY_LISTENING: QStatus = 36934i32;
pub const ER_BUS_KEY_UNAVAILABLE: QStatus = 36935i32;
pub const ER_BUS_TRUNCATED: QStatus = 36936i32;
pub const ER_BUS_KEY_STORE_NOT_LOADED: QStatus = 36937i32;
pub const ER_BUS_NO_AUTHENTICATION_MECHANISM: QStatus = 36938i32;
pub const ER_BUS_BUS_ALREADY_STARTED: QStatus = 36939i32;
pub const ER_BUS_BUS_NOT_STARTED: QStatus = 36940i32;
pub const ER_BUS_KEYBLOB_OP_INVALID: QStatus = 36941i32;
pub const ER_BUS_INVALID_HEADER_CHECKSUM: QStatus = 36942i32;
pub const ER_BUS_MESSAGE_NOT_ENCRYPTED: QStatus = 36943i32;
pub const ER_BUS_INVALID_HEADER_SERIAL: QStatus = 36944i32;
pub const ER_BUS_TIME_TO_LIVE_EXPIRED: QStatus = 36945i32;
pub const ER_BUS_HDR_EXPANSION_INVALID: QStatus = 36946i32;
pub const ER_BUS_MISSING_COMPRESSION_TOKEN: QStatus = 36947i32;
pub const ER_BUS_NO_PEER_GUID: QStatus = 36948i32;
pub const ER_BUS_MESSAGE_DECRYPTION_FAILED: QStatus = 36949i32;
pub const ER_BUS_SECURITY_FATAL: QStatus = 36950i32;
pub const ER_BUS_KEY_EXPIRED: QStatus = 36951i32;
pub const ER_BUS_CORRUPT_KEYSTORE: QStatus = 36952i32;
pub const ER_BUS_NO_CALL_FOR_REPLY: QStatus = 36953i32;
pub const ER_BUS_NOT_A_COMPLETE_TYPE: QStatus = 36954i32;
pub const ER_BUS_POLICY_VIOLATION: QStatus = 36955i32;
pub const ER_BUS_NO_SUCH_SERVICE: QStatus = 36956i32;
pub const ER_BUS_TRANSPORT_NOT_AVAILABLE: QStatus = 36957i32;
pub const ER_BUS_INVALID_AUTH_MECHANISM: QStatus = 36958i32;
pub const ER_BUS_KEYSTORE_VERSION_MISMATCH: QStatus = 36959i32;
pub const ER_BUS_BLOCKING_CALL_NOT_ALLOWED: QStatus = 36960i32;
pub const ER_BUS_SIGNATURE_MISMATCH: QStatus = 36961i32;
pub const ER_BUS_STOPPING: QStatus = 36962i32;
pub const ER_BUS_METHOD_CALL_ABORTED: QStatus = 36963i32;
pub const ER_BUS_CANNOT_ADD_INTERFACE: QStatus = 36964i32;
pub const ER_BUS_CANNOT_ADD_HANDLER: QStatus = 36965i32;
pub const ER_BUS_KEYSTORE_NOT_LOADED: QStatus = 36966i32;
pub const ER_BUS_NO_SUCH_HANDLE: QStatus = 36971i32;
pub const ER_BUS_HANDLES_NOT_ENABLED: QStatus = 36972i32;
pub const ER_BUS_HANDLES_MISMATCH: QStatus = 36973i32;
pub const ER_BUS_NO_SESSION: QStatus = 36975i32;
pub const ER_BUS_ELEMENT_NOT_FOUND: QStatus = 36976i32;
pub const ER_BUS_NOT_A_DICTIONARY: QStatus = 36977i32;
pub const ER_BUS_WAIT_FAILED: QStatus = 36978i32;
pub const ER_BUS_BAD_SESSION_OPTS: QStatus = 36980i32;
pub const ER_BUS_CONNECTION_REJECTED: QStatus = 36981i32;
pub const ER_DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER: QStatus = 36982i32;
pub const ER_DBUS_REQUEST_NAME_REPLY_IN_QUEUE: QStatus = 36983i32;
pub const ER_DBUS_REQUEST_NAME_REPLY_EXISTS: QStatus = 36984i32;
pub const ER_DBUS_REQUEST_NAME_REPLY_ALREADY_OWNER: QStatus = 36985i32;
pub const ER_DBUS_RELEASE_NAME_REPLY_RELEASED: QStatus = 36986i32;
pub const ER_DBUS_RELEASE_NAME_REPLY_NON_EXISTENT: QStatus = 36987i32;
pub const ER_DBUS_RELEASE_NAME_REPLY_NOT_OWNER: QStatus = 36988i32;
pub const ER_DBUS_START_REPLY_ALREADY_RUNNING: QStatus = 36990i32;
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_ALREADY_EXISTS: QStatus = 36992i32;
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_FAILED: QStatus = 36993i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_NO_SESSION: QStatus = 36995i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_UNREACHABLE: QStatus = 36996i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_CONNECT_FAILED: QStatus = 36997i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_REJECTED: QStatus = 36998i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_BAD_SESSION_OPTS: QStatus = 36999i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_FAILED: QStatus = 37000i32;
pub const ER_ALLJOYN_LEAVESESSION_REPLY_NO_SESSION: QStatus = 37002i32;
pub const ER_ALLJOYN_LEAVESESSION_REPLY_FAILED: QStatus = 37003i32;
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = 37004i32;
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_ALREADY_ADVERTISING: QStatus = 37005i32;
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_FAILED: QStatus = 37006i32;
pub const ER_ALLJOYN_CANCELADVERTISENAME_REPLY_FAILED: QStatus = 37008i32;
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = 37009i32;
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_ALREADY_DISCOVERING: QStatus = 37010i32;
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_FAILED: QStatus = 37011i32;
pub const ER_ALLJOYN_CANCELFINDADVERTISEDNAME_REPLY_FAILED: QStatus = 37013i32;
pub const ER_BUS_UNEXPECTED_DISPOSITION: QStatus = 37014i32;
pub const ER_BUS_INTERFACE_ACTIVATED: QStatus = 37015i32;
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_BAD_PORT: QStatus = 37016i32;
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_FAILED: QStatus = 37017i32;
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_INVALID_OPTS: QStatus = 37018i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_ALREADY_JOINED: QStatus = 37019i32;
pub const ER_BUS_SELF_CONNECT: QStatus = 37020i32;
pub const ER_BUS_SECURITY_NOT_ENABLED: QStatus = 37021i32;
pub const ER_BUS_LISTENER_ALREADY_SET: QStatus = 37022i32;
pub const ER_BUS_PEER_AUTH_VERSION_MISMATCH: QStatus = 37023i32;
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NOT_SUPPORTED: QStatus = 37024i32;
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NO_DEST_SUPPORT: QStatus = 37025i32;
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_FAILED: QStatus = 37026i32;
pub const ER_ALLJOYN_ACCESS_PERMISSION_WARNING: QStatus = 37027i32;
pub const ER_ALLJOYN_ACCESS_PERMISSION_ERROR: QStatus = 37028i32;
pub const ER_BUS_DESTINATION_NOT_AUTHENTICATED: QStatus = 37029i32;
pub const ER_BUS_ENDPOINT_REDIRECTED: QStatus = 37030i32;
pub const ER_BUS_AUTHENTICATION_PENDING: QStatus = 37031i32;
pub const ER_BUS_NOT_AUTHORIZED: QStatus = 37032i32;
pub const ER_PACKET_BUS_NO_SUCH_CHANNEL: QStatus = 37033i32;
pub const ER_PACKET_BAD_FORMAT: QStatus = 37034i32;
pub const ER_PACKET_CONNECT_TIMEOUT: QStatus = 37035i32;
pub const ER_PACKET_CHANNEL_FAIL: QStatus = 37036i32;
pub const ER_PACKET_TOO_LARGE: QStatus = 37037i32;
pub const ER_PACKET_BAD_PARAMETER: QStatus = 37038i32;
pub const ER_PACKET_BAD_CRC: QStatus = 37039i32;
pub const ER_RENDEZVOUS_SERVER_DEACTIVATED_USER: QStatus = 37067i32;
pub const ER_RENDEZVOUS_SERVER_UNKNOWN_USER: QStatus = 37068i32;
pub const ER_UNABLE_TO_CONNECT_TO_RENDEZVOUS_SERVER: QStatus = 37069i32;
pub const ER_NOT_CONNECTED_TO_RENDEZVOUS_SERVER: QStatus = 37070i32;
pub const ER_UNABLE_TO_SEND_MESSAGE_TO_RENDEZVOUS_SERVER: QStatus = 37071i32;
pub const ER_INVALID_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = 37072i32;
pub const ER_INVALID_PERSISTENT_CONNECTION_MESSAGE_RESPONSE: QStatus = 37073i32;
pub const ER_INVALID_ON_DEMAND_CONNECTION_MESSAGE_RESPONSE: QStatus = 37074i32;
pub const ER_INVALID_HTTP_METHOD_USED_FOR_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = 37075i32;
pub const ER_RENDEZVOUS_SERVER_ERR500_INTERNAL_ERROR: QStatus = 37076i32;
pub const ER_RENDEZVOUS_SERVER_ERR503_STATUS_UNAVAILABLE: QStatus = 37077i32;
pub const ER_RENDEZVOUS_SERVER_ERR401_UNAUTHORIZED_REQUEST: QStatus = 37078i32;
pub const ER_RENDEZVOUS_SERVER_UNRECOVERABLE_ERROR: QStatus = 37079i32;
pub const ER_RENDEZVOUS_SERVER_ROOT_CERTIFICATE_UNINITIALIZED: QStatus = 37080i32;
pub const ER_BUS_NO_SUCH_ANNOTATION: QStatus = 37081i32;
pub const ER_BUS_ANNOTATION_ALREADY_EXISTS: QStatus = 37082i32;
pub const ER_SOCK_CLOSING: QStatus = 37083i32;
pub const ER_NO_SUCH_DEVICE: QStatus = 37084i32;
pub const ER_P2P: QStatus = 37085i32;
pub const ER_P2P_TIMEOUT: QStatus = 37086i32;
pub const ER_P2P_NOT_CONNECTED: QStatus = 37087i32;
pub const ER_BAD_TRANSPORT_MASK: QStatus = 37088i32;
pub const ER_PROXIMITY_CONNECTION_ESTABLISH_FAIL: QStatus = 37089i32;
pub const ER_PROXIMITY_NO_PEERS_FOUND: QStatus = 37090i32;
pub const ER_BUS_OBJECT_NOT_REGISTERED: QStatus = 37091i32;
pub const ER_P2P_DISABLED: QStatus = 37092i32;
pub const ER_P2P_BUSY: QStatus = 37093i32;
pub const ER_BUS_INCOMPATIBLE_DAEMON: QStatus = 37094i32;
pub const ER_P2P_NO_GO: QStatus = 37095i32;
pub const ER_P2P_NO_STA: QStatus = 37096i32;
pub const ER_P2P_FORBIDDEN: QStatus = 37097i32;
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_FAILED: QStatus = 37098i32;
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_UNSUPPORTED: QStatus = 37099i32;
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_FAILED: QStatus = 37100i32;
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_UNSUPPORTED: QStatus = 37101i32;
pub const ER_BUS_NO_SUCH_MESSAGE: QStatus = 37102i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_NO_SESSION: QStatus = 37103i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_BINDER: QStatus = 37104i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_MULTIPOINT: QStatus = 37105i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_FOUND: QStatus = 37106i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_INCOMPATIBLE_REMOTE_DAEMON: QStatus = 37107i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_FAILED: QStatus = 37108i32;
pub const ER_BUS_REMOVED_BY_BINDER: QStatus = 37109i32;
pub const ER_BUS_MATCH_RULE_NOT_FOUND: QStatus = 37110i32;
pub const ER_ALLJOYN_PING_FAILED: QStatus = 37111i32;
pub const ER_ALLJOYN_PING_REPLY_UNREACHABLE: QStatus = 37112i32;
pub const ER_UDP_MSG_TOO_LONG: QStatus = 37113i32;
pub const ER_UDP_DEMUX_NO_ENDPOINT: QStatus = 37114i32;
pub const ER_UDP_NO_NETWORK: QStatus = 37115i32;
pub const ER_UDP_UNEXPECTED_LENGTH: QStatus = 37116i32;
pub const ER_UDP_UNEXPECTED_FLOW: QStatus = 37117i32;
pub const ER_UDP_DISCONNECT: QStatus = 37118i32;
pub const ER_UDP_NOT_IMPLEMENTED: QStatus = 37119i32;
pub const ER_UDP_NO_LISTENER: QStatus = 37120i32;
pub const ER_UDP_STOPPING: QStatus = 37121i32;
pub const ER_ARDP_BACKPRESSURE: QStatus = 37122i32;
pub const ER_UDP_BACKPRESSURE: QStatus = 37123i32;
pub const ER_ARDP_INVALID_STATE: QStatus = 37124i32;
pub const ER_ARDP_TTL_EXPIRED: QStatus = 37125i32;
pub const ER_ARDP_PERSIST_TIMEOUT: QStatus = 37126i32;
pub const ER_ARDP_PROBE_TIMEOUT: QStatus = 37127i32;
pub const ER_ARDP_REMOTE_CONNECTION_RESET: QStatus = 37128i32;
pub const ER_UDP_BUSHELLO: QStatus = 37129i32;
pub const ER_UDP_MESSAGE: QStatus = 37130i32;
pub const ER_UDP_INVALID: QStatus = 37131i32;
pub const ER_UDP_UNSUPPORTED: QStatus = 37132i32;
pub const ER_UDP_ENDPOINT_STALLED: QStatus = 37133i32;
pub const ER_ARDP_INVALID_RESPONSE: QStatus = 37134i32;
pub const ER_ARDP_INVALID_CONNECTION: QStatus = 37135i32;
pub const ER_UDP_LOCAL_DISCONNECT: QStatus = 37136i32;
pub const ER_UDP_EARLY_EXIT: QStatus = 37137i32;
pub const ER_UDP_LOCAL_DISCONNECT_FAIL: QStatus = 37138i32;
pub const ER_ARDP_DISCONNECTING: QStatus = 37139i32;
pub const ER_ALLJOYN_PING_REPLY_INCOMPATIBLE_REMOTE_ROUTING_NODE: QStatus = 37140i32;
pub const ER_ALLJOYN_PING_REPLY_TIMEOUT: QStatus = 37141i32;
pub const ER_ALLJOYN_PING_REPLY_UNKNOWN_NAME: QStatus = 37142i32;
pub const ER_ALLJOYN_PING_REPLY_FAILED: QStatus = 37143i32;
pub const ER_TCP_MAX_UNTRUSTED: QStatus = 37144i32;
pub const ER_ALLJOYN_PING_REPLY_IN_PROGRESS: QStatus = 37145i32;
pub const ER_LANGUAGE_NOT_SUPPORTED: QStatus = 37146i32;
pub const ER_ABOUT_FIELD_ALREADY_SPECIFIED: QStatus = 37147i32;
pub const ER_UDP_NOT_DISCONNECTED: QStatus = 37148i32;
pub const ER_UDP_ENDPOINT_NOT_STARTED: QStatus = 37149i32;
pub const ER_UDP_ENDPOINT_REMOVED: QStatus = 37150i32;
pub const ER_ARDP_VERSION_NOT_SUPPORTED: QStatus = 37151i32;
pub const ER_CONNECTION_LIMIT_EXCEEDED: QStatus = 37152i32;
pub const ER_ARDP_WRITE_BLOCKED: QStatus = 37153i32;
pub const ER_PERMISSION_DENIED: QStatus = 37154i32;
pub const ER_ABOUT_DEFAULT_LANGUAGE_NOT_SPECIFIED: QStatus = 37155i32;
pub const ER_ABOUT_SESSIONPORT_NOT_BOUND: QStatus = 37156i32;
pub const ER_ABOUT_ABOUTDATA_MISSING_REQUIRED_FIELD: QStatus = 37157i32;
pub const ER_ABOUT_INVALID_ABOUTDATA_LISTENER: QStatus = 37158i32;
pub const ER_BUS_PING_GROUP_NOT_FOUND: QStatus = 37159i32;
pub const ER_BUS_REMOVED_BY_BINDER_SELF: QStatus = 37160i32;
pub const ER_INVALID_CONFIG: QStatus = 37161i32;
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_VALUE: QStatus = 37162i32;
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_APPID_SIZE: QStatus = 37163i32;
pub const ER_BUS_TRANSPORT_ACCESS_DENIED: QStatus = 37164i32;
pub const ER_INVALID_CERTIFICATE: QStatus = 37165i32;
pub const ER_CERTIFICATE_NOT_FOUND: QStatus = 37166i32;
pub const ER_DUPLICATE_CERTIFICATE: QStatus = 37167i32;
pub const ER_UNKNOWN_CERTIFICATE: QStatus = 37168i32;
pub const ER_MISSING_DIGEST_IN_CERTIFICATE: QStatus = 37169i32;
pub const ER_DIGEST_MISMATCH: QStatus = 37170i32;
pub const ER_DUPLICATE_KEY: QStatus = 37171i32;
pub const ER_NO_COMMON_TRUST: QStatus = 37172i32;
pub const ER_MANIFEST_NOT_FOUND: QStatus = 37173i32;
pub const ER_INVALID_CERT_CHAIN: QStatus = 37174i32;
pub const ER_NO_TRUST_ANCHOR: QStatus = 37175i32;
pub const ER_INVALID_APPLICATION_STATE: QStatus = 37176i32;
pub const ER_FEATURE_NOT_AVAILABLE: QStatus = 37177i32;
pub const ER_KEY_STORE_ALREADY_INITIALIZED: QStatus = 37178i32;
pub const ER_KEY_STORE_ID_NOT_YET_SET: QStatus = 37179i32;
pub const ER_POLICY_NOT_NEWER: QStatus = 37180i32;
pub const ER_MANIFEST_REJECTED: QStatus = 37181i32;
pub const ER_INVALID_CERTIFICATE_USAGE: QStatus = 37182i32;
pub const ER_INVALID_SIGNAL_EMISSION_TYPE: QStatus = 37183i32;
pub const ER_APPLICATION_STATE_LISTENER_ALREADY_EXISTS: QStatus = 37184i32;
pub const ER_APPLICATION_STATE_LISTENER_NO_SUCH_LISTENER: QStatus = 37185i32;
pub const ER_MANAGEMENT_ALREADY_STARTED: QStatus = 37186i32;
pub const ER_MANAGEMENT_NOT_STARTED: QStatus = 37187i32;
pub const ER_BUS_DESCRIPTION_ALREADY_EXISTS: QStatus = 37188i32;
#[repr(C)]
pub struct _alljoyn_abouticon_handle(pub u8);
#[repr(C)]
pub struct _alljoyn_abouticonobj_handle(pub u8);
#[repr(C)]
pub struct _alljoyn_abouticonproxy_handle(pub u8);
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_about_announced_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, busname: super::super::Foundation::PSTR, version: u16, port: u16, objectdescriptionarg: alljoyn_msgarg, aboutdataarg: alljoyn_msgarg)>;
pub type alljoyn_about_announceflag = i32;
pub const UNANNOUNCED: alljoyn_about_announceflag = 0i32;
pub const ANNOUNCED: alljoyn_about_announceflag = 1i32;
pub type alljoyn_aboutdata = isize;
pub type alljoyn_aboutdatalistener = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_aboutdatalistener_callbacks {
    pub about_datalistener_getaboutdata: alljoyn_aboutdatalistener_getaboutdata_ptr,
    pub about_datalistener_getannouncedaboutdata: alljoyn_aboutdatalistener_getannouncedaboutdata_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_aboutdatalistener_callbacks {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_aboutdatalistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_aboutdatalistener_getaboutdata_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, msgarg: alljoyn_msgarg, language: super::super::Foundation::PSTR) -> QStatus>;
pub type alljoyn_aboutdatalistener_getannouncedaboutdata_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, msgarg: alljoyn_msgarg) -> QStatus>;
pub type alljoyn_aboutlistener = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_aboutlistener_callback {
    pub about_listener_announced: alljoyn_about_announced_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_aboutlistener_callback {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_aboutlistener_callback {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_aboutobj = isize;
pub type alljoyn_aboutobjectdescription = isize;
pub type alljoyn_aboutproxy = isize;
pub type alljoyn_applicationstate = i32;
pub const NOT_CLAIMABLE: alljoyn_applicationstate = 0i32;
pub const CLAIMABLE: alljoyn_applicationstate = 1i32;
pub const CLAIMED: alljoyn_applicationstate = 2i32;
pub const NEED_UPDATE: alljoyn_applicationstate = 3i32;
pub type alljoyn_applicationstatelistener = isize;
#[repr(C)]
pub struct alljoyn_applicationstatelistener_callbacks {
    pub state: alljoyn_applicationstatelistener_state_ptr,
}
impl ::core::marker::Copy for alljoyn_applicationstatelistener_callbacks {}
impl ::core::clone::Clone for alljoyn_applicationstatelistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_applicationstatelistener_state_ptr = ::core::option::Option<unsafe extern "system" fn(busname: *mut i8, publickey: *mut i8, applicationstate: alljoyn_applicationstate, context: *mut ::core::ffi::c_void)>;
pub type alljoyn_authlistener = isize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_authlistener_authenticationcomplete_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, authmechanism: super::super::Foundation::PSTR, peername: super::super::Foundation::PSTR, success: i32)>;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_authlistener_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentials_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentials_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_authlistener_callbacks {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_authlistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_authlistener_requestcredentials_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, authmechanism: super::super::Foundation::PSTR, peername: super::super::Foundation::PSTR, authcount: u16, username: super::super::Foundation::PSTR, credmask: u16, credentials: alljoyn_credentials) -> i32>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_authlistener_requestcredentialsasync_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: super::super::Foundation::PSTR, peername: super::super::Foundation::PSTR, authcount: u16, username: super::super::Foundation::PSTR, credmask: u16, authcontext: *mut ::core::ffi::c_void) -> QStatus>;
pub type alljoyn_authlistener_securityviolation_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, status: QStatus, msg: alljoyn_message)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_authlistener_verifycredentials_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, authmechanism: super::super::Foundation::PSTR, peername: super::super::Foundation::PSTR, credentials: alljoyn_credentials) -> i32>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_authlistener_verifycredentialsasync_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: super::super::Foundation::PSTR, peername: super::super::Foundation::PSTR, credentials: alljoyn_credentials, authcontext: *mut ::core::ffi::c_void) -> QStatus>;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_authlistenerasync_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentialsasync_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentialsasync_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_authlistenerasync_callbacks {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_authlistenerasync_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_autopinger = isize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_autopinger_destination_found_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, group: super::super::Foundation::PSTR, destination: super::super::Foundation::PSTR)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_autopinger_destination_lost_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, group: super::super::Foundation::PSTR, destination: super::super::Foundation::PSTR)>;
pub type alljoyn_busattachment = isize;
pub type alljoyn_busattachment_joinsessioncb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, sessionid: u32, opts: alljoyn_sessionopts, context: *mut ::core::ffi::c_void)>;
pub type alljoyn_busattachment_setlinktimeoutcb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, timeout: u32, context: *mut ::core::ffi::c_void)>;
pub type alljoyn_buslistener = isize;
pub type alljoyn_buslistener_bus_disconnected_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_buslistener_bus_prop_changed_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, prop_name: super::super::Foundation::PSTR, prop_value: alljoyn_msgarg)>;
pub type alljoyn_buslistener_bus_stopping_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_buslistener_callbacks {
    pub listener_registered: alljoyn_buslistener_listener_registered_ptr,
    pub listener_unregistered: alljoyn_buslistener_listener_unregistered_ptr,
    pub found_advertised_name: alljoyn_buslistener_found_advertised_name_ptr,
    pub lost_advertised_name: alljoyn_buslistener_lost_advertised_name_ptr,
    pub name_owner_changed: alljoyn_buslistener_name_owner_changed_ptr,
    pub bus_stopping: alljoyn_buslistener_bus_stopping_ptr,
    pub bus_disconnected: alljoyn_buslistener_bus_disconnected_ptr,
    pub property_changed: alljoyn_buslistener_bus_prop_changed_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_buslistener_callbacks {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_buslistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_buslistener_found_advertised_name_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, name: super::super::Foundation::PSTR, transport: u16, nameprefix: super::super::Foundation::PSTR)>;
pub type alljoyn_buslistener_listener_registered_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, bus: alljoyn_busattachment)>;
pub type alljoyn_buslistener_listener_unregistered_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_buslistener_lost_advertised_name_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, name: super::super::Foundation::PSTR, transport: u16, nameprefix: super::super::Foundation::PSTR)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_buslistener_name_owner_changed_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, busname: super::super::Foundation::PSTR, previousowner: super::super::Foundation::PSTR, newowner: super::super::Foundation::PSTR)>;
pub type alljoyn_busobject = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_busobject_callbacks {
    pub property_get: alljoyn_busobject_prop_get_ptr,
    pub property_set: alljoyn_busobject_prop_set_ptr,
    pub object_registered: alljoyn_busobject_object_registration_ptr,
    pub object_unregistered: alljoyn_busobject_object_registration_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_busobject_callbacks {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_busobject_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_busobject_methodentry {
    pub member: *mut alljoyn_interfacedescription_member,
    pub method_handler: alljoyn_messagereceiver_methodhandler_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_busobject_methodentry {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_busobject_methodentry {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_busobject_object_registration_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_prop_get_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, ifcname: super::super::Foundation::PSTR, propname: super::super::Foundation::PSTR, val: alljoyn_msgarg) -> QStatus>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_busobject_prop_set_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, ifcname: super::super::Foundation::PSTR, propname: super::super::Foundation::PSTR, val: alljoyn_msgarg) -> QStatus>;
#[repr(C)]
pub struct alljoyn_certificateid {
    pub serial: *mut u8,
    pub serialLen: usize,
    pub issuerPublicKey: *mut i8,
    pub issuerAki: *mut u8,
    pub issuerAkiLen: usize,
}
impl ::core::marker::Copy for alljoyn_certificateid {}
impl ::core::clone::Clone for alljoyn_certificateid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct alljoyn_certificateidarray {
    pub count: usize,
    pub ids: *mut alljoyn_certificateid,
}
impl ::core::marker::Copy for alljoyn_certificateidarray {}
impl ::core::clone::Clone for alljoyn_certificateidarray {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_claimcapability_masks = i32;
pub const CAPABLE_ECDHE_NULL: alljoyn_claimcapability_masks = 1i32;
pub const CAPABLE_ECDHE_ECDSA: alljoyn_claimcapability_masks = 4i32;
pub const CAPABLE_ECDHE_SPEKE: alljoyn_claimcapability_masks = 8i32;
pub type alljoyn_claimcapabilityadditionalinfo_masks = i32;
pub const PASSWORD_GENERATED_BY_SECURITY_MANAGER: alljoyn_claimcapabilityadditionalinfo_masks = 1i32;
pub const PASSWORD_GENERATED_BY_APPLICATION: alljoyn_claimcapabilityadditionalinfo_masks = 2i32;
pub type alljoyn_credentials = isize;
pub type alljoyn_interfacedescription = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_interfacedescription_member {
    pub iface: alljoyn_interfacedescription,
    pub memberType: alljoyn_messagetype,
    pub name: super::super::Foundation::PSTR,
    pub signature: super::super::Foundation::PSTR,
    pub returnSignature: super::super::Foundation::PSTR,
    pub argNames: super::super::Foundation::PSTR,
    pub internal_member: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_interfacedescription_member {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_interfacedescription_member {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_interfacedescription_property {
    pub name: super::super::Foundation::PSTR,
    pub signature: super::super::Foundation::PSTR,
    pub access: u8,
    pub internal_property: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_interfacedescription_property {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_interfacedescription_property {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_interfacedescription_securitypolicy = i32;
pub const AJ_IFC_SECURITY_INHERIT: alljoyn_interfacedescription_securitypolicy = 0i32;
pub const AJ_IFC_SECURITY_REQUIRED: alljoyn_interfacedescription_securitypolicy = 1i32;
pub const AJ_IFC_SECURITY_OFF: alljoyn_interfacedescription_securitypolicy = 2i32;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_interfacedescription_translation_callback_ptr = ::core::option::Option<unsafe extern "system" fn(sourcelanguage: super::super::Foundation::PSTR, targetlanguage: super::super::Foundation::PSTR, sourcetext: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR>;
pub type alljoyn_keystore = isize;
pub type alljoyn_keystorelistener = isize;
pub type alljoyn_keystorelistener_acquireexclusivelock_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_keystorelistener) -> QStatus>;
#[repr(C)]
pub struct alljoyn_keystorelistener_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
}
impl ::core::marker::Copy for alljoyn_keystorelistener_callbacks {}
impl ::core::clone::Clone for alljoyn_keystorelistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_keystorelistener_loadrequest_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
pub type alljoyn_keystorelistener_releaseexclusivelock_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_keystorelistener)>;
pub type alljoyn_keystorelistener_storerequest_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
#[repr(C)]
pub struct alljoyn_keystorelistener_with_synchronization_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
    pub acquire_exclusive_lock: alljoyn_keystorelistener_acquireexclusivelock_ptr,
    pub release_exclusive_lock: alljoyn_keystorelistener_releaseexclusivelock_ptr,
}
impl ::core::marker::Copy for alljoyn_keystorelistener_with_synchronization_callbacks {}
impl ::core::clone::Clone for alljoyn_keystorelistener_with_synchronization_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct alljoyn_manifestarray {
    pub count: usize,
    pub xmls: *mut *mut i8,
}
impl ::core::marker::Copy for alljoyn_manifestarray {}
impl ::core::clone::Clone for alljoyn_manifestarray {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_message = isize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_messagereceiver_methodhandler_ptr = ::core::option::Option<unsafe extern "system" fn(bus: alljoyn_busobject, member: *const alljoyn_interfacedescription_member, message: alljoyn_message)>;
pub type alljoyn_messagereceiver_replyhandler_ptr = ::core::option::Option<unsafe extern "system" fn(message: alljoyn_message, context: *mut ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_messagereceiver_signalhandler_ptr = ::core::option::Option<unsafe extern "system" fn(member: *const alljoyn_interfacedescription_member, srcpath: super::super::Foundation::PSTR, message: alljoyn_message)>;
pub type alljoyn_messagetype = i32;
pub const ALLJOYN_MESSAGE_INVALID: alljoyn_messagetype = 0i32;
pub const ALLJOYN_MESSAGE_METHOD_CALL: alljoyn_messagetype = 1i32;
pub const ALLJOYN_MESSAGE_METHOD_RET: alljoyn_messagetype = 2i32;
pub const ALLJOYN_MESSAGE_ERROR: alljoyn_messagetype = 3i32;
pub const ALLJOYN_MESSAGE_SIGNAL: alljoyn_messagetype = 4i32;
pub type alljoyn_msgarg = isize;
pub type alljoyn_observer = isize;
pub type alljoyn_observer_object_discovered_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref)>;
pub type alljoyn_observer_object_lost_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref)>;
pub type alljoyn_observerlistener = isize;
#[repr(C)]
pub struct alljoyn_observerlistener_callback {
    pub object_discovered: alljoyn_observer_object_discovered_ptr,
    pub object_lost: alljoyn_observer_object_lost_ptr,
}
impl ::core::marker::Copy for alljoyn_observerlistener_callback {}
impl ::core::clone::Clone for alljoyn_observerlistener_callback {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_permissionconfigurationlistener = isize;
#[repr(C)]
pub struct alljoyn_permissionconfigurationlistener_callbacks {
    pub factory_reset: alljoyn_permissionconfigurationlistener_factoryreset_ptr,
    pub policy_changed: alljoyn_permissionconfigurationlistener_policychanged_ptr,
    pub start_management: alljoyn_permissionconfigurationlistener_startmanagement_ptr,
    pub end_management: alljoyn_permissionconfigurationlistener_endmanagement_ptr,
}
impl ::core::marker::Copy for alljoyn_permissionconfigurationlistener_callbacks {}
impl ::core::clone::Clone for alljoyn_permissionconfigurationlistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_permissionconfigurationlistener_endmanagement_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
pub type alljoyn_permissionconfigurationlistener_factoryreset_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> QStatus>;
pub type alljoyn_permissionconfigurationlistener_policychanged_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
pub type alljoyn_permissionconfigurationlistener_startmanagement_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
pub type alljoyn_permissionconfigurator = isize;
pub type alljoyn_pinglistener = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_pinglistener_callback {
    pub destination_found: alljoyn_autopinger_destination_found_ptr,
    pub destination_lost: alljoyn_autopinger_destination_lost_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_pinglistener_callback {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_pinglistener_callback {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_proxybusobject = isize;
pub type alljoyn_proxybusobject_listener_getallpropertiescb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, values: alljoyn_msgarg, context: *mut ::core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_getpropertycb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, value: alljoyn_msgarg, context: *mut ::core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_introspectcb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_proxybusobject_listener_propertieschanged_ptr = ::core::option::Option<unsafe extern "system" fn(obj: alljoyn_proxybusobject, ifacename: super::super::Foundation::PSTR, changed: alljoyn_msgarg, invalidated: alljoyn_msgarg, context: *mut ::core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_setpropertycb_ptr = ::core::option::Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut ::core::ffi::c_void)>;
pub type alljoyn_proxybusobject_ref = isize;
pub type alljoyn_securityapplicationproxy = isize;
pub type alljoyn_sessionlistener = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_sessionlistener_callbacks {
    pub session_lost: alljoyn_sessionlistener_sessionlost_ptr,
    pub session_member_added: alljoyn_sessionlistener_sessionmemberadded_ptr,
    pub session_member_removed: alljoyn_sessionlistener_sessionmemberremoved_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_sessionlistener_callbacks {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_sessionlistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
pub type alljoyn_sessionlistener_sessionlost_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionid: u32, reason: alljoyn_sessionlostreason)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_sessionlistener_sessionmemberadded_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionid: u32, uniquename: super::super::Foundation::PSTR)>;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_sessionlistener_sessionmemberremoved_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionid: u32, uniquename: super::super::Foundation::PSTR)>;
pub type alljoyn_sessionlostreason = i32;
pub const ALLJOYN_SESSIONLOST_INVALID: alljoyn_sessionlostreason = 0i32;
pub const ALLJOYN_SESSIONLOST_REMOTE_END_LEFT_SESSION: alljoyn_sessionlostreason = 1i32;
pub const ALLJOYN_SESSIONLOST_REMOTE_END_CLOSED_ABRUPTLY: alljoyn_sessionlostreason = 2i32;
pub const ALLJOYN_SESSIONLOST_REMOVED_BY_BINDER: alljoyn_sessionlostreason = 3i32;
pub const ALLJOYN_SESSIONLOST_LINK_TIMEOUT: alljoyn_sessionlostreason = 4i32;
pub const ALLJOYN_SESSIONLOST_REASON_OTHER: alljoyn_sessionlostreason = 5i32;
pub type alljoyn_sessionopts = isize;
pub type alljoyn_sessionportlistener = isize;
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_sessionportlistener_acceptsessionjoiner_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionport: u16, joiner: super::super::Foundation::PSTR, opts: alljoyn_sessionopts) -> i32>;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_sessionportlistener_callbacks {
    pub accept_session_joiner: alljoyn_sessionportlistener_acceptsessionjoiner_ptr,
    pub session_joined: alljoyn_sessionportlistener_sessionjoined_ptr,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for alljoyn_sessionportlistener_callbacks {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for alljoyn_sessionportlistener_callbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type alljoyn_sessionportlistener_sessionjoined_ptr = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionport: u16, id: u32, joiner: super::super::Foundation::PSTR)>;
pub type alljoyn_typeid = i32;
pub const ALLJOYN_INVALID: alljoyn_typeid = 0i32;
pub const ALLJOYN_ARRAY: alljoyn_typeid = 97i32;
pub const ALLJOYN_BOOLEAN: alljoyn_typeid = 98i32;
pub const ALLJOYN_DOUBLE: alljoyn_typeid = 100i32;
pub const ALLJOYN_DICT_ENTRY: alljoyn_typeid = 101i32;
pub const ALLJOYN_SIGNATURE: alljoyn_typeid = 103i32;
pub const ALLJOYN_HANDLE: alljoyn_typeid = 104i32;
pub const ALLJOYN_INT32: alljoyn_typeid = 105i32;
pub const ALLJOYN_INT16: alljoyn_typeid = 110i32;
pub const ALLJOYN_OBJECT_PATH: alljoyn_typeid = 111i32;
pub const ALLJOYN_UINT16: alljoyn_typeid = 113i32;
pub const ALLJOYN_STRUCT: alljoyn_typeid = 114i32;
pub const ALLJOYN_STRING: alljoyn_typeid = 115i32;
pub const ALLJOYN_UINT64: alljoyn_typeid = 116i32;
pub const ALLJOYN_UINT32: alljoyn_typeid = 117i32;
pub const ALLJOYN_VARIANT: alljoyn_typeid = 118i32;
pub const ALLJOYN_INT64: alljoyn_typeid = 120i32;
pub const ALLJOYN_BYTE: alljoyn_typeid = 121i32;
pub const ALLJOYN_STRUCT_OPEN: alljoyn_typeid = 40i32;
pub const ALLJOYN_STRUCT_CLOSE: alljoyn_typeid = 41i32;
pub const ALLJOYN_DICT_ENTRY_OPEN: alljoyn_typeid = 123i32;
pub const ALLJOYN_DICT_ENTRY_CLOSE: alljoyn_typeid = 125i32;
pub const ALLJOYN_BOOLEAN_ARRAY: alljoyn_typeid = 25185i32;
pub const ALLJOYN_DOUBLE_ARRAY: alljoyn_typeid = 25697i32;
pub const ALLJOYN_INT32_ARRAY: alljoyn_typeid = 26977i32;
pub const ALLJOYN_INT16_ARRAY: alljoyn_typeid = 28257i32;
pub const ALLJOYN_UINT16_ARRAY: alljoyn_typeid = 29025i32;
pub const ALLJOYN_UINT64_ARRAY: alljoyn_typeid = 29793i32;
pub const ALLJOYN_UINT32_ARRAY: alljoyn_typeid = 30049i32;
pub const ALLJOYN_INT64_ARRAY: alljoyn_typeid = 30817i32;
pub const ALLJOYN_BYTE_ARRAY: alljoyn_typeid = 31073i32;
pub const ALLJOYN_WILDCARD: alljoyn_typeid = 42i32;
