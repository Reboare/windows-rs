#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type CMP_WaitNoPendingInstallEvents = unsafe extern "system" fn(dwtimeout: u32) -> u32;
#[cfg(feature = "Win32_Data_HtmlHelp")]
pub type CM_Add_Empty_Log_Conf = unsafe extern "system" fn(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Data_HtmlHelp")]
pub type CM_Add_Empty_Log_Conf_Ex = unsafe extern "system" fn(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Add_IDA = unsafe extern "system" fn(dndevinst: u32, pszid: super::super::Foundation::PSTR, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Add_IDW = unsafe extern "system" fn(dndevinst: u32, pszid: super::super::Foundation::PWSTR, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Add_ID_ExA = unsafe extern "system" fn(dndevinst: u32, pszid: super::super::Foundation::PSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Add_ID_ExW = unsafe extern "system" fn(dndevinst: u32, pszid: super::super::Foundation::PWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Add_Range = unsafe extern "system" fn(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Add_Res_Des = unsafe extern "system" fn(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Add_Res_Des_Ex = unsafe extern "system" fn(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Connect_MachineA = unsafe extern "system" fn(uncservername: super::super::Foundation::PSTR, phmachine: *mut isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Connect_MachineW = unsafe extern "system" fn(uncservername: super::super::Foundation::PWSTR, phmachine: *mut isize) -> CONFIGRET;
pub type CM_Create_DevNodeA = unsafe extern "system" fn(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Create_DevNodeW = unsafe extern "system" fn(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Create_DevNode_ExA = unsafe extern "system" fn(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Create_DevNode_ExW = unsafe extern "system" fn(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Create_Range_List = unsafe extern "system" fn(prlh: *mut usize, ulflags: u32) -> CONFIGRET;
pub type CM_Delete_Class_Key = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, ulflags: u32) -> CONFIGRET;
pub type CM_Delete_Class_Key_Ex = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Delete_DevNode_Key = unsafe extern "system" fn(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Delete_DevNode_Key_Ex = unsafe extern "system" fn(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Delete_Device_Interface_KeyA = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PSTR, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Delete_Device_Interface_KeyW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Delete_Device_Interface_Key_ExA = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Delete_Device_Interface_Key_ExW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Delete_Range = unsafe extern "system" fn(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Detect_Resource_Conflict = unsafe extern "system" fn(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Detect_Resource_Conflict_Ex = unsafe extern "system" fn(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Disable_DevNode = unsafe extern "system" fn(dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Disable_DevNode_Ex = unsafe extern "system" fn(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Disconnect_Machine = unsafe extern "system" fn(hmachine: isize) -> CONFIGRET;
pub type CM_Dup_Range_List = unsafe extern "system" fn(rlhold: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Enable_DevNode = unsafe extern "system" fn(dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Enable_DevNode_Ex = unsafe extern "system" fn(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Enumerate_Classes = unsafe extern "system" fn(ulclassindex: u32, classguid: *mut ::windows_sys::core::GUID, ulflags: u32) -> CONFIGRET;
pub type CM_Enumerate_Classes_Ex = unsafe extern "system" fn(ulclassindex: u32, classguid: *mut ::windows_sys::core::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Enumerate_EnumeratorsA = unsafe extern "system" fn(ulenumindex: u32, buffer: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Enumerate_EnumeratorsW = unsafe extern "system" fn(ulenumindex: u32, buffer: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Enumerate_Enumerators_ExA = unsafe extern "system" fn(ulenumindex: u32, buffer: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Enumerate_Enumerators_ExW = unsafe extern "system" fn(ulenumindex: u32, buffer: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Find_Range = unsafe extern "system" fn(pullstart: *mut u64, ullstart: u64, ullength: u32, ullalignment: u64, ullend: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
pub type CM_First_Range = unsafe extern "system" fn(rlh: usize, pullstart: *mut u64, pullend: *mut u64, preelement: *mut usize, ulflags: u32) -> CONFIGRET;
pub type CM_Free_Log_Conf = unsafe extern "system" fn(lclogconftobefreed: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Free_Log_Conf_Ex = unsafe extern "system" fn(lclogconftobefreed: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Free_Log_Conf_Handle = unsafe extern "system" fn(lclogconf: usize) -> CONFIGRET;
pub type CM_Free_Range_List = unsafe extern "system" fn(rlh: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Free_Res_Des = unsafe extern "system" fn(prdresdes: *mut usize, rdresdes: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Free_Res_Des_Ex = unsafe extern "system" fn(prdresdes: *mut usize, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Free_Res_Des_Handle = unsafe extern "system" fn(rdresdes: usize) -> CONFIGRET;
pub type CM_Free_Resource_Conflict_Handle = unsafe extern "system" fn(clconflictlist: usize) -> CONFIGRET;
pub type CM_Get_Child = unsafe extern "system" fn(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Child_Ex = unsafe extern "system" fn(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Class_Key_NameA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, pszkeyname: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Class_Key_NameW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, pszkeyname: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Class_Key_Name_ExA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, pszkeyname: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Class_Key_Name_ExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, pszkeyname: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Class_NameA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, buffer: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Class_NameW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, buffer: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Class_Name_ExA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, buffer: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Class_Name_ExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, buffer: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Get_Class_PropertyW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Get_Class_Property_ExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Get_Class_Property_Keys = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Get_Class_Property_Keys_Ex = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Class_Registry_PropertyA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Class_Registry_PropertyW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Depth = unsafe extern "system" fn(puldepth: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Depth_Ex = unsafe extern "system" fn(puldepth: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_DevNode_Custom_PropertyA = unsafe extern "system" fn(dndevinst: u32, pszcustompropertyname: super::super::Foundation::PSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_DevNode_Custom_PropertyW = unsafe extern "system" fn(dndevinst: u32, pszcustompropertyname: super::super::Foundation::PWSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_DevNode_Custom_Property_ExA = unsafe extern "system" fn(dndevinst: u32, pszcustompropertyname: super::super::Foundation::PSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_DevNode_Custom_Property_ExW = unsafe extern "system" fn(dndevinst: u32, pszcustompropertyname: super::super::Foundation::PWSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Get_DevNode_PropertyW = unsafe extern "system" fn(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Get_DevNode_Property_ExW = unsafe extern "system" fn(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Get_DevNode_Property_Keys = unsafe extern "system" fn(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Get_DevNode_Property_Keys_Ex = unsafe extern "system" fn(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_DevNode_Registry_PropertyA = unsafe extern "system" fn(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_DevNode_Registry_PropertyW = unsafe extern "system" fn(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_DevNode_Registry_Property_ExA = unsafe extern "system" fn(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_DevNode_Registry_Property_ExW = unsafe extern "system" fn(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_DevNode_Status = unsafe extern "system" fn(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_DevNode_Status_Ex = unsafe extern "system" fn(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_IDA = unsafe extern "system" fn(dndevinst: u32, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_IDW = unsafe extern "system" fn(dndevinst: u32, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_ExA = unsafe extern "system" fn(dndevinst: u32, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_ExW = unsafe extern "system" fn(dndevinst: u32, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_ListA = unsafe extern "system" fn(pszfilter: super::super::Foundation::PSTR, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_ListW = unsafe extern "system" fn(pszfilter: super::super::Foundation::PWSTR, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_List_ExA = unsafe extern "system" fn(pszfilter: super::super::Foundation::PSTR, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_List_ExW = unsafe extern "system" fn(pszfilter: super::super::Foundation::PWSTR, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_List_SizeA = unsafe extern "system" fn(pullen: *mut u32, pszfilter: super::super::Foundation::PSTR, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_List_SizeW = unsafe extern "system" fn(pullen: *mut u32, pszfilter: super::super::Foundation::PWSTR, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_List_Size_ExA = unsafe extern "system" fn(pullen: *mut u32, pszfilter: super::super::Foundation::PSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_ID_List_Size_ExW = unsafe extern "system" fn(pullen: *mut u32, pszfilter: super::super::Foundation::PWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Device_ID_Size = unsafe extern "system" fn(pullen: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Device_ID_Size_Ex = unsafe extern "system" fn(pullen: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_Interface_AliasA = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PSTR, aliasinterfaceguid: *const ::windows_sys::core::GUID, pszaliasdeviceinterface: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_Interface_AliasW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, aliasinterfaceguid: *const ::windows_sys::core::GUID, pszaliasdeviceinterface: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_Interface_Alias_ExA = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PSTR, aliasinterfaceguid: *const ::windows_sys::core::GUID, pszaliasdeviceinterface: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_Interface_Alias_ExW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, aliasinterfaceguid: *const ::windows_sys::core::GUID, pszaliasdeviceinterface: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_Interface_ListA = unsafe extern "system" fn(interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const i8, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_Interface_ListW = unsafe extern "system" fn(interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const u16, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_Interface_List_ExA = unsafe extern "system" fn(interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const i8, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Device_Interface_List_ExW = unsafe extern "system" fn(interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const u16, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Device_Interface_List_SizeA = unsafe extern "system" fn(pullen: *mut u32, interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Device_Interface_List_SizeW = unsafe extern "system" fn(pullen: *mut u32, interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Device_Interface_List_Size_ExA = unsafe extern "system" fn(pullen: *mut u32, interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Device_Interface_List_Size_ExW = unsafe extern "system" fn(pullen: *mut u32, interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type CM_Get_Device_Interface_PropertyW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type CM_Get_Device_Interface_Property_ExW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type CM_Get_Device_Interface_Property_KeysW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type CM_Get_Device_Interface_Property_Keys_ExW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_First_Log_Conf = unsafe extern "system" fn(plclogconf: *mut usize, dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_First_Log_Conf_Ex = unsafe extern "system" fn(plclogconf: *mut usize, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Global_State = unsafe extern "system" fn(pulstate: *mut u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Global_State_Ex = unsafe extern "system" fn(pulstate: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_HW_Prof_FlagsA = unsafe extern "system" fn(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_HW_Prof_FlagsW = unsafe extern "system" fn(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_HW_Prof_Flags_ExA = unsafe extern "system" fn(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_HW_Prof_Flags_ExW = unsafe extern "system" fn(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Hardware_Profile_InfoA = unsafe extern "system" fn(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Hardware_Profile_InfoW = unsafe extern "system" fn(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Hardware_Profile_Info_ExA = unsafe extern "system" fn(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Hardware_Profile_Info_ExW = unsafe extern "system" fn(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Log_Conf_Priority = unsafe extern "system" fn(lclogconf: usize, ppriority: *mut u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Log_Conf_Priority_Ex = unsafe extern "system" fn(lclogconf: usize, ppriority: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Next_Log_Conf = unsafe extern "system" fn(plclogconf: *mut usize, lclogconf: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Next_Log_Conf_Ex = unsafe extern "system" fn(plclogconf: *mut usize, lclogconf: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Next_Res_Des = unsafe extern "system" fn(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Next_Res_Des_Ex = unsafe extern "system" fn(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Parent = unsafe extern "system" fn(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Parent_Ex = unsafe extern "system" fn(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Res_Des_Data = unsafe extern "system" fn(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Res_Des_Data_Ex = unsafe extern "system" fn(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Res_Des_Data_Size = unsafe extern "system" fn(pulsize: *mut u32, rdresdes: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Res_Des_Data_Size_Ex = unsafe extern "system" fn(pulsize: *mut u32, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Resource_Conflict_Count = unsafe extern "system" fn(clconflictlist: usize, pulcount: *mut u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Get_Resource_Conflict_DetailsA = unsafe extern "system" fn(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_A) -> CONFIGRET;
pub type CM_Get_Resource_Conflict_DetailsW = unsafe extern "system" fn(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_W) -> CONFIGRET;
pub type CM_Get_Sibling = unsafe extern "system" fn(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Get_Sibling_Ex = unsafe extern "system" fn(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Get_Version = unsafe extern "system" fn() -> u16;
pub type CM_Get_Version_Ex = unsafe extern "system" fn(hmachine: isize) -> u16;
pub type CM_Intersect_Range_List = unsafe extern "system" fn(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Invert_Range_List = unsafe extern "system" fn(rlhold: usize, rlhnew: usize, ullmaxvalue: u64, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Is_Dock_Station_Present = unsafe extern "system" fn(pbpresent: *mut super::super::Foundation::BOOL) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Is_Dock_Station_Present_Ex = unsafe extern "system" fn(pbpresent: *mut super::super::Foundation::BOOL, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Is_Version_Available = unsafe extern "system" fn(wversion: u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Is_Version_Available_Ex = unsafe extern "system" fn(wversion: u16, hmachine: isize) -> super::super::Foundation::BOOL;
pub type CM_Locate_DevNodeA = unsafe extern "system" fn(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET;
pub type CM_Locate_DevNodeW = unsafe extern "system" fn(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET;
pub type CM_Locate_DevNode_ExA = unsafe extern "system" fn(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Locate_DevNode_ExW = unsafe extern "system" fn(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_MapCrToWin32Err = unsafe extern "system" fn(cmreturncode: CONFIGRET, defaulterr: u32) -> u32;
pub type CM_Merge_Range_List = unsafe extern "system" fn(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Modify_Res_Des = unsafe extern "system" fn(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Modify_Res_Des_Ex = unsafe extern "system" fn(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Move_DevNode = unsafe extern "system" fn(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Move_DevNode_Ex = unsafe extern "system" fn(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Next_Range = unsafe extern "system" fn(preelement: *mut usize, pullstart: *mut u64, pullend: *mut u64, ulflags: u32) -> CONFIGRET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type CM_Open_Class_KeyA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, pszclassname: super::super::Foundation::PSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type CM_Open_Class_KeyW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, pszclassname: super::super::Foundation::PWSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type CM_Open_Class_Key_ExA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, pszclassname: super::super::Foundation::PSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type CM_Open_Class_Key_ExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, pszclassname: super::super::Foundation::PWSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_System_Registry")]
pub type CM_Open_DevNode_Key = unsafe extern "system" fn(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_System_Registry")]
pub type CM_Open_DevNode_Key_Ex = unsafe extern "system" fn(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type CM_Open_Device_Interface_KeyA = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type CM_Open_Device_Interface_KeyW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type CM_Open_Device_Interface_Key_ExA = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type CM_Open_Device_Interface_Key_ExW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Query_And_Remove_SubTreeA = unsafe extern "system" fn(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Query_And_Remove_SubTreeW = unsafe extern "system" fn(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PWSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Query_And_Remove_SubTree_ExA = unsafe extern "system" fn(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Query_And_Remove_SubTree_ExW = unsafe extern "system" fn(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PWSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Query_Arbitrator_Free_Data = unsafe extern "system" fn(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Query_Arbitrator_Free_Data_Ex = unsafe extern "system" fn(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Query_Arbitrator_Free_Size = unsafe extern "system" fn(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Query_Arbitrator_Free_Size_Ex = unsafe extern "system" fn(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Query_Remove_SubTree = unsafe extern "system" fn(dnancestor: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Query_Remove_SubTree_Ex = unsafe extern "system" fn(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Query_Resource_Conflict_List = unsafe extern "system" fn(pclconflictlist: *mut usize, dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Reenumerate_DevNode = unsafe extern "system" fn(dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Reenumerate_DevNode_Ex = unsafe extern "system" fn(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Register_Device_Driver = unsafe extern "system" fn(dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Register_Device_Driver_Ex = unsafe extern "system" fn(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Register_Device_InterfaceA = unsafe extern "system" fn(dndevinst: u32, interfaceclassguid: *const ::windows_sys::core::GUID, pszreference: super::super::Foundation::PSTR, pszdeviceinterface: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Register_Device_InterfaceW = unsafe extern "system" fn(dndevinst: u32, interfaceclassguid: *const ::windows_sys::core::GUID, pszreference: super::super::Foundation::PWSTR, pszdeviceinterface: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Register_Device_Interface_ExA = unsafe extern "system" fn(dndevinst: u32, interfaceclassguid: *const ::windows_sys::core::GUID, pszreference: super::super::Foundation::PSTR, pszdeviceinterface: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Register_Device_Interface_ExW = unsafe extern "system" fn(dndevinst: u32, interfaceclassguid: *const ::windows_sys::core::GUID, pszreference: super::super::Foundation::PWSTR, pszdeviceinterface: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Register_Notification = unsafe extern "system" fn(pfilter: *const CM_NOTIFY_FILTER, pcontext: *const ::core::ffi::c_void, pcallback: PCM_NOTIFY_CALLBACK, pnotifycontext: *mut isize) -> CONFIGRET;
pub type CM_Remove_SubTree = unsafe extern "system" fn(dnancestor: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Remove_SubTree_Ex = unsafe extern "system" fn(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Request_Device_EjectA = unsafe extern "system" fn(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Request_Device_EjectW = unsafe extern "system" fn(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PWSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Request_Device_Eject_ExA = unsafe extern "system" fn(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Request_Device_Eject_ExW = unsafe extern "system" fn(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PWSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Request_Eject_PC = unsafe extern "system" fn() -> CONFIGRET;
pub type CM_Request_Eject_PC_Ex = unsafe extern "system" fn(hmachine: isize) -> CONFIGRET;
pub type CM_Run_Detection = unsafe extern "system" fn(ulflags: u32) -> CONFIGRET;
pub type CM_Run_Detection_Ex = unsafe extern "system" fn(ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Set_Class_PropertyW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Set_Class_Property_ExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Set_Class_Registry_PropertyA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Set_Class_Registry_PropertyW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Set_DevNode_Problem = unsafe extern "system" fn(dndevinst: u32, ulproblem: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Set_DevNode_Problem_Ex = unsafe extern "system" fn(dndevinst: u32, ulproblem: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Set_DevNode_PropertyW = unsafe extern "system" fn(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Devices_Properties")]
pub type CM_Set_DevNode_Property_ExW = unsafe extern "system" fn(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Set_DevNode_Registry_PropertyA = unsafe extern "system" fn(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Set_DevNode_Registry_PropertyW = unsafe extern "system" fn(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Set_DevNode_Registry_Property_ExA = unsafe extern "system" fn(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Set_DevNode_Registry_Property_ExW = unsafe extern "system" fn(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type CM_Set_Device_Interface_PropertyW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type CM_Set_Device_Interface_Property_ExW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Set_HW_Prof = unsafe extern "system" fn(ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Set_HW_Prof_Ex = unsafe extern "system" fn(ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Set_HW_Prof_FlagsA = unsafe extern "system" fn(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Set_HW_Prof_FlagsW = unsafe extern "system" fn(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Set_HW_Prof_Flags_ExA = unsafe extern "system" fn(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Set_HW_Prof_Flags_ExW = unsafe extern "system" fn(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Setup_DevNode = unsafe extern "system" fn(dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Setup_DevNode_Ex = unsafe extern "system" fn(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Test_Range_Available = unsafe extern "system" fn(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
pub type CM_Uninstall_DevNode = unsafe extern "system" fn(dndevinst: u32, ulflags: u32) -> CONFIGRET;
pub type CM_Uninstall_DevNode_Ex = unsafe extern "system" fn(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Unregister_Device_InterfaceA = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PSTR, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Unregister_Device_InterfaceW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, ulflags: u32) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Unregister_Device_Interface_ExA = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type CM_Unregister_Device_Interface_ExW = unsafe extern "system" fn(pszdeviceinterface: super::super::Foundation::PWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
pub type CM_Unregister_Notification = unsafe extern "system" fn(notifycontext: HCMNOTIFICATION) -> CONFIGRET;
#[cfg(feature = "Win32_Foundation")]
pub type DiInstallDevice = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DiInstallDriverA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, infpath: super::super::Foundation::PSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DiInstallDriverW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, infpath: super::super::Foundation::PWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DiRollbackDriver = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, hwndparent: super::super::Foundation::HWND, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DiShowUpdateDevice = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DiShowUpdateDriver = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, filepath: super::super::Foundation::PWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DiUninstallDevice = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DiUninstallDriverA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, infpath: super::super::Foundation::PSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DiUninstallDriverW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, infpath: super::super::Foundation::PWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type InstallHinfSectionA = unsafe extern "system" fn(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: super::super::Foundation::PSTR, showcommand: i32);
#[cfg(feature = "Win32_Foundation")]
pub type InstallHinfSectionW = unsafe extern "system" fn(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: super::super::Foundation::PWSTR, showcommand: i32);
#[cfg(feature = "Win32_Foundation")]
pub type SetupAddInstallSectionToDiskSpaceListA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupAddInstallSectionToDiskSpaceListW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupAddSectionToDiskSpaceListA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupAddSectionToDiskSpaceListW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupAddToDiskSpaceListA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, targetfilespec: super::super::Foundation::PSTR, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupAddToDiskSpaceListW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, targetfilespec: super::super::Foundation::PWSTR, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupAddToSourceListA = unsafe extern "system" fn(flags: u32, source: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupAddToSourceListW = unsafe extern "system" fn(flags: u32, source: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupAdjustDiskSpaceListA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, driveroot: super::super::Foundation::PSTR, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupAdjustDiskSpaceListW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, driveroot: super::super::Foundation::PWSTR, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupBackupErrorA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, sourcefile: super::super::Foundation::PSTR, targetfile: super::super::Foundation::PSTR, win32errorcode: u32, style: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupBackupErrorW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, sourcefile: super::super::Foundation::PWSTR, targetfile: super::super::Foundation::PWSTR, win32errorcode: u32, style: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupCancelTemporarySourceList = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupCloseFileQueue = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type SetupCloseInfFile = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void);
pub type SetupCloseLog = unsafe extern "system" fn();
#[cfg(feature = "Win32_Foundation")]
pub type SetupCommitFileQueueA = unsafe extern "system" fn(owner: super::super::Foundation::HWND, queuehandle: *const ::core::ffi::c_void, msghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupCommitFileQueueW = unsafe extern "system" fn(owner: super::super::Foundation::HWND, queuehandle: *const ::core::ffi::c_void, msghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupConfigureWmiFromInfSectionA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupConfigureWmiFromInfSectionW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupCopyErrorA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, diskname: super::super::Foundation::PSTR, pathtosource: super::super::Foundation::PSTR, sourcefile: super::super::Foundation::PSTR, targetpathfile: super::super::Foundation::PSTR, win32errorcode: u32, style: u32, pathbuffer: super::super::Foundation::PSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupCopyErrorW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, diskname: super::super::Foundation::PWSTR, pathtosource: super::super::Foundation::PWSTR, sourcefile: super::super::Foundation::PWSTR, targetpathfile: super::super::Foundation::PWSTR, win32errorcode: u32, style: u32, pathbuffer: super::super::Foundation::PWSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupCopyOEMInfA = unsafe extern "system" fn(sourceinffilename: super::super::Foundation::PSTR, oemsourcemedialocation: super::super::Foundation::PSTR, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: super::super::Foundation::PSTR, destinationinffilenamesize: u32, requiredsize: *mut u32, destinationinffilenamecomponent: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupCopyOEMInfW = unsafe extern "system" fn(sourceinffilename: super::super::Foundation::PWSTR, oemsourcemedialocation: super::super::Foundation::PWSTR, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: super::super::Foundation::PWSTR, destinationinffilenamesize: u32, requiredsize: *mut u32, destinationinffilenamecomponent: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
pub type SetupCreateDiskSpaceListA = unsafe extern "system" fn(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
pub type SetupCreateDiskSpaceListW = unsafe extern "system" fn(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDecompressOrCopyFileA = unsafe extern "system" fn(sourcefilename: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, compressiontype: *const u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDecompressOrCopyFileW = unsafe extern "system" fn(sourcefilename: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, compressiontype: *const u32) -> u32;
pub type SetupDefaultQueueCallbackA = unsafe extern "system" fn(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32;
pub type SetupDefaultQueueCallbackW = unsafe extern "system" fn(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDeleteErrorA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, file: super::super::Foundation::PSTR, win32errorcode: u32, style: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDeleteErrorW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, file: super::super::Foundation::PWSTR, win32errorcode: u32, style: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDestroyDiskSpaceList = unsafe extern "system" fn(diskspace: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiAskForOEMDisk = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiBuildClassInfoList = unsafe extern "system" fn(flags: u32, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiBuildClassInfoListExA = unsafe extern "system" fn(flags: u32, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiBuildClassInfoListExW = unsafe extern "system" fn(flags: u32, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiBuildDriverInfoList = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, drivertype: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiCallClassInstaller = unsafe extern "system" fn(installfunction: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiCancelDriverInfoSearch = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiChangeState = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiClassGuidsFromNameA = unsafe extern "system" fn(classname: super::super::Foundation::PSTR, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiClassGuidsFromNameExA = unsafe extern "system" fn(classname: super::super::Foundation::PSTR, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiClassGuidsFromNameExW = unsafe extern "system" fn(classname: super::super::Foundation::PWSTR, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiClassGuidsFromNameW = unsafe extern "system" fn(classname: super::super::Foundation::PWSTR, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiClassNameFromGuidA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, classname: super::super::Foundation::PSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiClassNameFromGuidExA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, classname: super::super::Foundation::PSTR, classnamesize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiClassNameFromGuidExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, classname: super::super::Foundation::PWSTR, classnamesize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiClassNameFromGuidW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, classname: super::super::Foundation::PWSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type SetupDiCreateDevRegKeyA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PSTR) -> super::super::System::Registry::HKEY;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type SetupDiCreateDevRegKeyW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PWSTR) -> super::super::System::Registry::HKEY;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiCreateDeviceInfoA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, devicename: super::super::Foundation::PSTR, classguid: *const ::windows_sys::core::GUID, devicedescription: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiCreateDeviceInfoList = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, hwndparent: super::super::Foundation::HWND) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiCreateDeviceInfoListExA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, hwndparent: super::super::Foundation::HWND, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiCreateDeviceInfoListExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, hwndparent: super::super::Foundation::HWND, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiCreateDeviceInfoW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, devicename: super::super::Foundation::PWSTR, classguid: *const ::windows_sys::core::GUID, devicedescription: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiCreateDeviceInterfaceA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows_sys::core::GUID, referencestring: super::super::Foundation::PSTR, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type SetupDiCreateDeviceInterfaceRegKeyA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PSTR) -> super::super::System::Registry::HKEY;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type SetupDiCreateDeviceInterfaceRegKeyW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PWSTR) -> super::super::System::Registry::HKEY;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiCreateDeviceInterfaceW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows_sys::core::GUID, referencestring: super::super::Foundation::PWSTR, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiDeleteDevRegKey = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiDeleteDeviceInfo = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiDeleteDeviceInterfaceData = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiDeleteDeviceInterfaceRegKey = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type SetupDiDestroyClassImageList = unsafe extern "system" fn(classimagelistdata: *const SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiDestroyDeviceInfoList = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiDestroyDriverInfoList = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type SetupDiDrawMiniIcon = unsafe extern "system" fn(hdc: super::super::Graphics::Gdi::HDC, rc: super::super::Foundation::RECT, miniiconindex: i32, flags: u32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiEnumDeviceInfo = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, memberindex: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiEnumDeviceInterfaces = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows_sys::core::GUID, memberindex: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiEnumDriverInfoA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiEnumDriverInfoW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupDiGetActualModelsSectionA = unsafe extern "system" fn(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: super::super::Foundation::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupDiGetActualModelsSectionW = unsafe extern "system" fn(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: super::super::Foundation::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetActualSectionToInstallA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PSTR, infsectionwithext: super::super::Foundation::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupDiGetActualSectionToInstallExA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: super::super::Foundation::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupDiGetActualSectionToInstallExW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PWSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: super::super::Foundation::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetActualSectionToInstallW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PWSTR, infsectionwithext: super::super::Foundation::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassBitmapIndex = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, miniiconindex: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassDescriptionA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, classdescription: super::super::Foundation::PSTR, classdescriptionsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassDescriptionExA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, classdescription: super::super::Foundation::PSTR, classdescriptionsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassDescriptionExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, classdescription: super::super::Foundation::PWSTR, classdescriptionsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassDescriptionW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, classdescription: super::super::Foundation::PWSTR, classdescriptionsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub type SetupDiGetClassDevPropertySheetsA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERA_V2, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub type SetupDiGetClassDevPropertySheetsW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERW_V2, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassDevsA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, enumerator: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, flags: u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassDevsExA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, enumerator: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, flags: u32, deviceinfoset: *const ::core::ffi::c_void, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassDevsExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, enumerator: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, flags: u32, deviceinfoset: *const ::core::ffi::c_void, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassDevsW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, enumerator: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, flags: u32) -> *mut ::core::ffi::c_void;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type SetupDiGetClassImageIndex = unsafe extern "system" fn(classimagelistdata: *const SP_CLASSIMAGELIST_DATA, classguid: *const ::windows_sys::core::GUID, imageindex: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type SetupDiGetClassImageList = unsafe extern "system" fn(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type SetupDiGetClassImageListExA = unsafe extern "system" fn(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type SetupDiGetClassImageListExW = unsafe extern "system" fn(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassInstallParamsA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassInstallParamsW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiGetClassPropertyExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiGetClassPropertyKeys = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiGetClassPropertyKeysExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiGetClassPropertyW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassRegistryPropertyA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetClassRegistryPropertyW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetCustomDevicePropertyA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: super::super::Foundation::PSTR, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetCustomDevicePropertyW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: super::super::Foundation::PWSTR, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInfoListClass = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, classguid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInfoListDetailA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInfoListDetailW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInstallParamsA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_A) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInstallParamsW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_W) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInstanceIdA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: super::super::Foundation::PSTR, deviceinstanceidsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInstanceIdW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: super::super::Foundation::PWSTR, deviceinstanceidsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInterfaceAlias = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid: *const ::windows_sys::core::GUID, aliasdeviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInterfaceDetailA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceInterfaceDetailW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiGetDeviceInterfacePropertyKeys = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiGetDeviceInterfacePropertyW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiGetDevicePropertyKeys = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiGetDevicePropertyW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceRegistryPropertyA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDeviceRegistryPropertyW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDriverInfoDetailA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_A, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDriverInfoDetailW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_W, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDriverInstallParamsA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetDriverInstallParamsW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetHwProfileFriendlyNameA = unsafe extern "system" fn(hwprofile: u32, friendlyname: super::super::Foundation::PSTR, friendlynamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetHwProfileFriendlyNameExA = unsafe extern "system" fn(hwprofile: u32, friendlyname: super::super::Foundation::PSTR, friendlynamesize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetHwProfileFriendlyNameExW = unsafe extern "system" fn(hwprofile: u32, friendlyname: super::super::Foundation::PWSTR, friendlynamesize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetHwProfileFriendlyNameW = unsafe extern "system" fn(hwprofile: u32, friendlyname: super::super::Foundation::PWSTR, friendlynamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetHwProfileList = unsafe extern "system" fn(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetHwProfileListExA = unsafe extern "system" fn(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetHwProfileListExW = unsafe extern "system" fn(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetINFClassA = unsafe extern "system" fn(infname: super::super::Foundation::PSTR, classguid: *mut ::windows_sys::core::GUID, classname: super::super::Foundation::PSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetINFClassW = unsafe extern "system" fn(infname: super::super::Foundation::PWSTR, classguid: *mut ::windows_sys::core::GUID, classname: super::super::Foundation::PWSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetSelectedDevice = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetSelectedDriverA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiGetSelectedDriverW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type SetupDiGetWizardPage = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, installwizarddata: *const SP_INSTALLWIZARD_DATA, pagetype: u32, flags: u32) -> super::super::UI::Controls::HPROPSHEETPAGE;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiInstallClassA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, inffilename: super::super::Foundation::PSTR, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiInstallClassExA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, inffilename: super::super::Foundation::PSTR, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows_sys::core::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiInstallClassExW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, inffilename: super::super::Foundation::PWSTR, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows_sys::core::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiInstallClassW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, inffilename: super::super::Foundation::PWSTR, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiInstallDevice = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiInstallDeviceInterfaces = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiInstallDriverFiles = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type SetupDiLoadClassIcon = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, largeicon: *mut super::super::UI::WindowsAndMessaging::HICON, miniiconindex: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type SetupDiLoadDeviceIcon = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, cxicon: u32, cyicon: u32, flags: u32, hicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_System_Registry")]
pub type SetupDiOpenClassRegKey = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, samdesired: u32) -> super::super::System::Registry::HKEY;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type SetupDiOpenClassRegKeyExA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, samdesired: u32, flags: u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type SetupDiOpenClassRegKeyExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, samdesired: u32, flags: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY;
#[cfg(feature = "Win32_System_Registry")]
pub type SetupDiOpenDevRegKey = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, samdesired: u32) -> super::super::System::Registry::HKEY;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiOpenDeviceInfoA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinstanceid: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiOpenDeviceInfoW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinstanceid: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiOpenDeviceInterfaceA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, devicepath: super::super::Foundation::PSTR, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_System_Registry")]
pub type SetupDiOpenDeviceInterfaceRegKey = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32) -> super::super::System::Registry::HKEY;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiOpenDeviceInterfaceW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, devicepath: super::super::Foundation::PWSTR, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiRegisterCoDeviceInstallers = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiRegisterDeviceInfo = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, flags: u32, compareproc: PSP_DETSIG_CMPPROC, comparecontext: *const ::core::ffi::c_void, dupdeviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiRemoveDevice = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiRemoveDeviceInterface = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiRestartDevices = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSelectBestCompatDrv = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSelectDevice = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSelectOEMDrv = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetClassInstallParamsA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetClassInstallParamsW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiSetClassPropertyExW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiSetClassPropertyW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetClassRegistryPropertyA = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetClassRegistryPropertyW = unsafe extern "system" fn(classguid: *const ::windows_sys::core::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetDeviceInstallParamsA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_A) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetDeviceInstallParamsW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_W) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetDeviceInterfaceDefault = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiSetDeviceInterfacePropertyW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type SetupDiSetDevicePropertyW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetDeviceRegistryPropertyA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetDeviceRegistryPropertyW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetDriverInstallParamsA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetDriverInstallParamsW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetSelectedDevice = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetSelectedDriverA = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiSetSelectedDriverW = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupDiUnremoveDevice = unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
pub type SetupDuplicateDiskSpaceListA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
pub type SetupDuplicateDiskSpaceListW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupEnumInfSectionsA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, index: u32, buffer: super::super::Foundation::PSTR, size: u32, sizeneeded: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupEnumInfSectionsW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, index: u32, buffer: super::super::Foundation::PWSTR, size: u32, sizeneeded: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupFindFirstLineA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR, key: super::super::Foundation::PSTR, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupFindFirstLineW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR, key: super::super::Foundation::PWSTR, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupFindNextLine = unsafe extern "system" fn(contextin: *const INFCONTEXT, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupFindNextMatchLineA = unsafe extern "system" fn(contextin: *const INFCONTEXT, key: super::super::Foundation::PSTR, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupFindNextMatchLineW = unsafe extern "system" fn(contextin: *const INFCONTEXT, key: super::super::Foundation::PWSTR, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupFreeSourceListA = unsafe extern "system" fn(list: *mut *mut super::super::Foundation::PSTR, count: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupFreeSourceListW = unsafe extern "system" fn(list: *mut *mut super::super::Foundation::PWSTR, count: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetBackupInformationA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_A) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetBackupInformationW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_W) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetBinaryField = unsafe extern "system" fn(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: *mut u8, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
pub type SetupGetFieldCount = unsafe extern "system" fn(context: *const INFCONTEXT) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetFileCompressionInfoA = unsafe extern "system" fn(sourcefilename: super::super::Foundation::PSTR, actualsourcefilename: *mut super::super::Foundation::PSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetFileCompressionInfoExA = unsafe extern "system" fn(sourcefilename: super::super::Foundation::PSTR, actualsourcefilenamebuffer: super::super::Foundation::PSTR, actualsourcefilenamebufferlen: u32, requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetFileCompressionInfoExW = unsafe extern "system" fn(sourcefilename: super::super::Foundation::PWSTR, actualsourcefilenamebuffer: super::super::Foundation::PWSTR, actualsourcefilenamebufferlen: u32, requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetFileCompressionInfoW = unsafe extern "system" fn(sourcefilename: super::super::Foundation::PWSTR, actualsourcefilename: *mut super::super::Foundation::PWSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetFileQueueCount = unsafe extern "system" fn(filequeue: *const ::core::ffi::c_void, subqueuefileop: u32, numoperations: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetFileQueueFlags = unsafe extern "system" fn(filequeue: *const ::core::ffi::c_void, flags: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupGetInfDriverStoreLocationA = unsafe extern "system" fn(filename: super::super::Foundation::PSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupGetInfDriverStoreLocationW = unsafe extern "system" fn(filename: super::super::Foundation::PWSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetInfFileListA = unsafe extern "system" fn(directorypath: super::super::Foundation::PSTR, infstyle: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetInfFileListW = unsafe extern "system" fn(directorypath: super::super::Foundation::PWSTR, infstyle: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetInfInformationA = unsafe extern "system" fn(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetInfInformationW = unsafe extern "system" fn(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetInfPublishedNameA = unsafe extern "system" fn(driverstorelocation: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetInfPublishedNameW = unsafe extern "system" fn(driverstorelocation: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetIntField = unsafe extern "system" fn(context: *const INFCONTEXT, fieldindex: u32, integervalue: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetLineByIndexA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetLineByIndexW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetLineCountA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetLineCountW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetLineTextA = unsafe extern "system" fn(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR, key: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetLineTextW = unsafe extern "system" fn(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR, key: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetMultiSzFieldA = unsafe extern "system" fn(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetMultiSzFieldW = unsafe extern "system" fn(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetNonInteractiveMode = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetSourceFileLocationA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: super::super::Foundation::PSTR, sourceid: *mut u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetSourceFileLocationW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: super::super::Foundation::PWSTR, sourceid: *mut u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetSourceFileSizeA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: super::super::Foundation::PSTR, section: super::super::Foundation::PSTR, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetSourceFileSizeW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: super::super::Foundation::PWSTR, section: super::super::Foundation::PWSTR, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetSourceInfoA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetSourceInfoW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetStringFieldA = unsafe extern "system" fn(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetStringFieldW = unsafe extern "system" fn(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetTargetPathA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupGetTargetPathW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
pub type SetupGetThreadLogToken = unsafe extern "system" fn() -> u64;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInitDefaultQueueCallback = unsafe extern "system" fn(ownerwindow: super::super::Foundation::HWND) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInitDefaultQueueCallbackEx = unsafe extern "system" fn(ownerwindow: super::super::Foundation::HWND, alternateprogresswindow: super::super::Foundation::HWND, progressmessage: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInitializeFileLogA = unsafe extern "system" fn(logfilename: super::super::Foundation::PSTR, flags: u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInitializeFileLogW = unsafe extern "system" fn(logfilename: super::super::Foundation::PWSTR, flags: u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallFileA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: super::super::Foundation::PSTR, sourcepathroot: super::super::Foundation::PSTR, destinationname: super::super::Foundation::PSTR, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallFileExA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: super::super::Foundation::PSTR, sourcepathroot: super::super::Foundation::PSTR, destinationname: super::super::Foundation::PSTR, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallFileExW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: super::super::Foundation::PWSTR, sourcepathroot: super::super::Foundation::PWSTR, destinationname: super::super::Foundation::PWSTR, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallFileW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: super::super::Foundation::PWSTR, sourcepathroot: super::super::Foundation::PWSTR, destinationname: super::super::Foundation::PWSTR, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallFilesFromInfSectionA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, sourcerootpath: super::super::Foundation::PSTR, copyflags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallFilesFromInfSectionW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, sourcerootpath: super::super::Foundation::PWSTR, copyflags: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type SetupInstallFromInfSectionA =
    unsafe extern "system" fn(owner: super::super::Foundation::HWND, infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, flags: u32, relativekeyroot: super::super::System::Registry::HKEY, sourcerootpath: super::super::Foundation::PSTR, copyflags: u32, msghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type SetupInstallFromInfSectionW =
    unsafe extern "system" fn(owner: super::super::Foundation::HWND, infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, flags: u32, relativekeyroot: super::super::System::Registry::HKEY, sourcerootpath: super::super::Foundation::PWSTR, copyflags: u32, msghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallServicesFromInfSectionA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallServicesFromInfSectionExA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, flags: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallServicesFromInfSectionExW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, flags: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupInstallServicesFromInfSectionW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupIterateCabinetA = unsafe extern "system" fn(cabinetfile: super::super::Foundation::PSTR, reserved: u32, msghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupIterateCabinetW = unsafe extern "system" fn(cabinetfile: super::super::Foundation::PWSTR, reserved: u32, msghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupLogErrorA = unsafe extern "system" fn(messagestring: super::super::Foundation::PSTR, severity: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupLogErrorW = unsafe extern "system" fn(messagestring: super::super::Foundation::PWSTR, severity: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupLogFileA = unsafe extern "system" fn(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PSTR, sourcefilename: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, checksum: u32, disktagfile: super::super::Foundation::PSTR, diskdescription: super::super::Foundation::PSTR, otherinfo: super::super::Foundation::PSTR, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupLogFileW = unsafe extern "system" fn(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PWSTR, sourcefilename: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, checksum: u32, disktagfile: super::super::Foundation::PWSTR, diskdescription: super::super::Foundation::PWSTR, otherinfo: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupOpenAppendInfFileA = unsafe extern "system" fn(filename: super::super::Foundation::PSTR, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupOpenAppendInfFileW = unsafe extern "system" fn(filename: super::super::Foundation::PWSTR, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL;
pub type SetupOpenFileQueue = unsafe extern "system" fn() -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupOpenInfFileA = unsafe extern "system" fn(filename: super::super::Foundation::PSTR, infclass: super::super::Foundation::PSTR, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupOpenInfFileW = unsafe extern "system" fn(filename: super::super::Foundation::PWSTR, infclass: super::super::Foundation::PWSTR, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupOpenLog = unsafe extern "system" fn(erase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
pub type SetupOpenMasterInf = unsafe extern "system" fn() -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type SetupPrepareQueueForRestoreA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, backuppath: super::super::Foundation::PSTR, restoreflags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupPrepareQueueForRestoreW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, backuppath: super::super::Foundation::PWSTR, restoreflags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupPromptForDiskA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, diskname: super::super::Foundation::PSTR, pathtosource: super::super::Foundation::PSTR, filesought: super::super::Foundation::PSTR, tagfile: super::super::Foundation::PSTR, diskpromptstyle: u32, pathbuffer: super::super::Foundation::PSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupPromptForDiskW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, diskname: super::super::Foundation::PWSTR, pathtosource: super::super::Foundation::PWSTR, filesought: super::super::Foundation::PWSTR, tagfile: super::super::Foundation::PWSTR, diskpromptstyle: u32, pathbuffer: super::super::Foundation::PWSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupPromptReboot = unsafe extern "system" fn(filequeue: *const ::core::ffi::c_void, owner: super::super::Foundation::HWND, scanonly: super::super::Foundation::BOOL) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueryDrivesInDiskSpaceListA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueryDrivesInDiskSpaceListW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueryFileLogA = unsafe extern "system" fn(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, desiredinfo: SetupFileLogInfo, dataout: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueryFileLogW = unsafe extern "system" fn(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, desiredinfo: SetupFileLogInfo, dataout: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueryInfFileInformationA = unsafe extern "system" fn(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueryInfFileInformationW = unsafe extern "system" fn(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupQueryInfOriginalFileInformationA = unsafe extern "system" fn(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_A) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupQueryInfOriginalFileInformationW = unsafe extern "system" fn(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_W) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueryInfVersionInformationA = unsafe extern "system" fn(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueryInfVersionInformationW = unsafe extern "system" fn(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQuerySourceListA = unsafe extern "system" fn(flags: u32, list: *mut *mut super::super::Foundation::PSTR, count: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQuerySourceListW = unsafe extern "system" fn(flags: u32, list: *mut *mut super::super::Foundation::PWSTR, count: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQuerySpaceRequiredOnDriveA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, drivespec: super::super::Foundation::PSTR, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQuerySpaceRequiredOnDriveW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, drivespec: super::super::Foundation::PWSTR, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueCopyA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PSTR, sourcepath: super::super::Foundation::PSTR, sourcefilename: super::super::Foundation::PSTR, sourcedescription: super::super::Foundation::PSTR, sourcetagfile: super::super::Foundation::PSTR, targetdirectory: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, copystyle: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueCopyIndirectA = unsafe extern "system" fn(copyparams: *const SP_FILE_COPY_PARAMS_A) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueCopyIndirectW = unsafe extern "system" fn(copyparams: *const SP_FILE_COPY_PARAMS_W) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueCopySectionA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PSTR, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR, copystyle: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueCopySectionW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PWSTR, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueCopyW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PWSTR, sourcepath: super::super::Foundation::PWSTR, sourcefilename: super::super::Foundation::PWSTR, sourcedescription: super::super::Foundation::PWSTR, sourcetagfile: super::super::Foundation::PWSTR, targetdirectory: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueDefaultCopyA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PSTR, sourcefilename: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, copystyle: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueDefaultCopyW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PWSTR, sourcefilename: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueDeleteA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, pathpart1: super::super::Foundation::PSTR, pathpart2: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueDeleteSectionA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueDeleteSectionW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueDeleteW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, pathpart1: super::super::Foundation::PWSTR, pathpart2: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueRenameA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, sourcepath: super::super::Foundation::PSTR, sourcefilename: super::super::Foundation::PSTR, targetpath: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueRenameSectionA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueRenameSectionW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupQueueRenameW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, sourcepath: super::super::Foundation::PWSTR, sourcefilename: super::super::Foundation::PWSTR, targetpath: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveFileLogEntryA = unsafe extern "system" fn(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveFileLogEntryW = unsafe extern "system" fn(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveFromDiskSpaceListA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, targetfilespec: super::super::Foundation::PSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveFromDiskSpaceListW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, targetfilespec: super::super::Foundation::PWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveFromSourceListA = unsafe extern "system" fn(flags: u32, source: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveFromSourceListW = unsafe extern "system" fn(flags: u32, source: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveInstallSectionFromDiskSpaceListA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveInstallSectionFromDiskSpaceListW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveSectionFromDiskSpaceListA = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRemoveSectionFromDiskSpaceListW = unsafe extern "system" fn(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRenameErrorA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, sourcefile: super::super::Foundation::PSTR, targetfile: super::super::Foundation::PSTR, win32errorcode: u32, style: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupRenameErrorW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, sourcefile: super::super::Foundation::PWSTR, targetfile: super::super::Foundation::PWSTR, win32errorcode: u32, style: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type SetupScanFileQueueA = unsafe extern "system" fn(filequeue: *const ::core::ffi::c_void, flags: u32, window: super::super::Foundation::HWND, callbackroutine: PSP_FILE_CALLBACK_A, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupScanFileQueueW = unsafe extern "system" fn(filequeue: *const ::core::ffi::c_void, flags: u32, window: super::super::Foundation::HWND, callbackroutine: PSP_FILE_CALLBACK_W, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetDirectoryIdA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, id: u32, directory: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetDirectoryIdExA = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, id: u32, directory: super::super::Foundation::PSTR, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetDirectoryIdExW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, id: u32, directory: super::super::Foundation::PWSTR, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetDirectoryIdW = unsafe extern "system" fn(infhandle: *const ::core::ffi::c_void, id: u32, directory: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupSetFileQueueAlternatePlatformA = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupSetFileQueueAlternatePlatformW = unsafe extern "system" fn(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetFileQueueFlags = unsafe extern "system" fn(filequeue: *const ::core::ffi::c_void, flagmask: u32, flags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetNonInteractiveMode = unsafe extern "system" fn(noninteractiveflag: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetPlatformPathOverrideA = unsafe extern "system" fn(r#override: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetPlatformPathOverrideW = unsafe extern "system" fn(r#override: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetSourceListA = unsafe extern "system" fn(flags: u32, sourcelist: *const super::super::Foundation::PSTR, sourcecount: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupSetSourceListW = unsafe extern "system" fn(flags: u32, sourcelist: *const super::super::Foundation::PWSTR, sourcecount: u32) -> super::super::Foundation::BOOL;
pub type SetupSetThreadLogToken = unsafe extern "system" fn(logtoken: u64);
pub type SetupTermDefaultQueueCallback = unsafe extern "system" fn(context: *const ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type SetupTerminateFileLog = unsafe extern "system" fn(fileloghandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupUninstallNewlyCopiedInfs = unsafe extern "system" fn(filequeue: *const ::core::ffi::c_void, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupUninstallOEMInfA = unsafe extern "system" fn(inffilename: super::super::Foundation::PSTR, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupUninstallOEMInfW = unsafe extern "system" fn(inffilename: super::super::Foundation::PWSTR, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupVerifyInfFileA = unsafe extern "system" fn(infname: super::super::Foundation::PSTR, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_A) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type SetupVerifyInfFileW = unsafe extern "system" fn(infname: super::super::Foundation::PWSTR, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_W) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SetupWriteTextLog = unsafe extern "system" fn(logtoken: u64, category: u32, flags: u32, messagestr: super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type SetupWriteTextLogError = unsafe extern "system" fn(logtoken: u64, category: u32, logflags: u32, error: u32, messagestr: super::super::Foundation::PSTR);
pub type SetupWriteTextLogInfLine = unsafe extern "system" fn(logtoken: u64, flags: u32, infhandle: *const ::core::ffi::c_void, context: *const INFCONTEXT);
#[cfg(feature = "Win32_Foundation")]
pub type UpdateDriverForPlugAndPlayDevicesA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, hardwareid: super::super::Foundation::PSTR, fullinfpath: super::super::Foundation::PSTR, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type UpdateDriverForPlugAndPlayDevicesW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, hardwareid: super::super::Foundation::PWSTR, fullinfpath: super::super::Foundation::PWSTR, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
pub const ALLOC_LOG_CONF: u32 = 2u32;
pub const BASIC_LOG_CONF: u32 = 0u32;
pub const BOOT_LOG_CONF: u32 = 3u32;
#[repr(C, packed(1))]
pub struct BUSNUMBER_DES {
    pub BUSD_Count: u32,
    pub BUSD_Type: u32,
    pub BUSD_Flags: u32,
    pub BUSD_Alloc_Base: u32,
    pub BUSD_Alloc_End: u32,
}
impl ::core::marker::Copy for BUSNUMBER_DES {}
impl ::core::clone::Clone for BUSNUMBER_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct BUSNUMBER_RANGE {
    pub BUSR_Min: u32,
    pub BUSR_Max: u32,
    pub BUSR_nBusNumbers: u32,
    pub BUSR_Flags: u32,
}
impl ::core::marker::Copy for BUSNUMBER_RANGE {}
impl ::core::clone::Clone for BUSNUMBER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct BUSNUMBER_RESOURCE {
    pub BusNumber_Header: BUSNUMBER_DES,
    pub BusNumber_Data: [BUSNUMBER_RANGE; 1],
}
impl ::core::marker::Copy for BUSNUMBER_RESOURCE {}
impl ::core::clone::Clone for BUSNUMBER_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct CABINET_INFO_A {
    pub CabinetPath: super::super::Foundation::PSTR,
    pub CabinetFile: super::super::Foundation::PSTR,
    pub DiskName: super::super::Foundation::PSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABINET_INFO_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct CABINET_INFO_A {
    pub CabinetPath: super::super::Foundation::PSTR,
    pub CabinetFile: super::super::Foundation::PSTR,
    pub DiskName: super::super::Foundation::PSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABINET_INFO_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct CABINET_INFO_W {
    pub CabinetPath: super::super::Foundation::PWSTR,
    pub CabinetFile: super::super::Foundation::PWSTR,
    pub DiskName: super::super::Foundation::PWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABINET_INFO_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct CABINET_INFO_W {
    pub CabinetPath: super::super::Foundation::PWSTR,
    pub CabinetFile: super::super::Foundation::PWSTR,
    pub DiskName: super::super::Foundation::PWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABINET_INFO_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CM_ADD_ID_BITS: u32 = 1u32;
pub const CM_ADD_ID_COMPATIBLE: u32 = 1u32;
pub const CM_ADD_ID_HARDWARE: u32 = 0u32;
pub const CM_ADD_RANGE_ADDIFCONFLICT: u32 = 0u32;
pub const CM_ADD_RANGE_BITS: u32 = 1u32;
pub const CM_ADD_RANGE_DONOTADDIFCONFLICT: u32 = 1u32;
pub const CM_CDFLAGS_DRIVER: u32 = 1u32;
pub const CM_CDFLAGS_RESERVED: u32 = 4u32;
pub const CM_CDFLAGS_ROOT_OWNED: u32 = 2u32;
pub const CM_CDMASK_DESCRIPTION: u32 = 8u32;
pub const CM_CDMASK_DEVINST: u32 = 1u32;
pub const CM_CDMASK_FLAGS: u32 = 4u32;
pub const CM_CDMASK_RESDES: u32 = 2u32;
pub const CM_CDMASK_VALID: u32 = 15u32;
pub const CM_CLASS_PROPERTY_BITS: u32 = 1u32;
pub const CM_CLASS_PROPERTY_INSTALLER: u32 = 0u32;
pub const CM_CLASS_PROPERTY_INTERFACE: u32 = 1u32;
pub const CM_CREATE_DEVINST_BITS: u32 = 15u32;
pub const CM_CREATE_DEVINST_DO_NOT_INSTALL: u32 = 8u32;
pub const CM_CREATE_DEVINST_GENERATE_ID: u32 = 4u32;
pub const CM_CREATE_DEVINST_NORMAL: u32 = 0u32;
pub const CM_CREATE_DEVINST_NO_WAIT_INSTALL: u32 = 1u32;
pub const CM_CREATE_DEVINST_PHANTOM: u32 = 2u32;
pub const CM_CREATE_DEVNODE_BITS: u32 = 15u32;
pub const CM_CREATE_DEVNODE_DO_NOT_INSTALL: u32 = 8u32;
pub const CM_CREATE_DEVNODE_GENERATE_ID: u32 = 4u32;
pub const CM_CREATE_DEVNODE_NORMAL: u32 = 0u32;
pub const CM_CREATE_DEVNODE_NO_WAIT_INSTALL: u32 = 1u32;
pub const CM_CREATE_DEVNODE_PHANTOM: u32 = 2u32;
pub const CM_CRP_CHARACTERISTICS: u32 = 28u32;
pub const CM_CRP_DEVTYPE: u32 = 26u32;
pub const CM_CRP_EXCLUSIVE: u32 = 27u32;
pub const CM_CRP_LOWERFILTERS: u32 = 19u32;
pub const CM_CRP_MAX: u32 = 37u32;
pub const CM_CRP_MIN: u32 = 1u32;
pub const CM_CRP_SECURITY: u32 = 24u32;
pub const CM_CRP_SECURITY_SDS: u32 = 25u32;
pub const CM_CRP_UPPERFILTERS: u32 = 18u32;
pub const CM_CUSTOMDEVPROP_BITS: u32 = 1u32;
pub const CM_CUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1u32;
pub const CM_DELETE_CLASS_BITS: u32 = 3u32;
pub const CM_DELETE_CLASS_INTERFACE: u32 = 2u32;
pub const CM_DELETE_CLASS_ONLY: u32 = 0u32;
pub const CM_DELETE_CLASS_SUBKEYS: u32 = 1u32;
pub const CM_DETECT_BITS: u32 = 2147483655u32;
pub const CM_DETECT_CRASHED: u32 = 2u32;
pub const CM_DETECT_HWPROF_FIRST_BOOT: u32 = 4u32;
pub const CM_DETECT_NEW_PROFILE: u32 = 1u32;
pub const CM_DETECT_RUN: u32 = 2147483648u32;
pub const CM_DEVCAP_DOCKDEVICE: u32 = 8u32;
pub const CM_DEVCAP_EJECTSUPPORTED: u32 = 2u32;
pub const CM_DEVCAP_HARDWAREDISABLED: u32 = 256u32;
pub const CM_DEVCAP_LOCKSUPPORTED: u32 = 1u32;
pub const CM_DEVCAP_NONDYNAMIC: u32 = 512u32;
pub const CM_DEVCAP_RAWDEVICEOK: u32 = 64u32;
pub const CM_DEVCAP_REMOVABLE: u32 = 4u32;
pub const CM_DEVCAP_SECUREDEVICE: u32 = 1024u32;
pub const CM_DEVCAP_SILENTINSTALL: u32 = 32u32;
pub const CM_DEVCAP_SURPRISEREMOVALOK: u32 = 128u32;
pub const CM_DEVCAP_UNIQUEID: u32 = 16u32;
pub const CM_DEVICE_PANEL_EDGE_BOTTOM: u32 = 2u32;
pub const CM_DEVICE_PANEL_EDGE_LEFT: u32 = 3u32;
pub const CM_DEVICE_PANEL_EDGE_RIGHT: u32 = 4u32;
pub const CM_DEVICE_PANEL_EDGE_TOP: u32 = 1u32;
pub const CM_DEVICE_PANEL_EDGE_UNKNOWN: u32 = 0u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_HINGE: u32 = 2u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_PIVOT: u32 = 3u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_PLANAR: u32 = 1u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_SWIVEL: u32 = 4u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_UNKNOWN: u32 = 0u32;
pub const CM_DEVICE_PANEL_ORIENTATION_HORIZONTAL: u32 = 0u32;
pub const CM_DEVICE_PANEL_ORIENTATION_VERTICAL: u32 = 1u32;
pub const CM_DEVICE_PANEL_SHAPE_OVAL: u32 = 2u32;
pub const CM_DEVICE_PANEL_SHAPE_RECTANGLE: u32 = 1u32;
pub const CM_DEVICE_PANEL_SHAPE_UNKNOWN: u32 = 0u32;
pub const CM_DEVICE_PANEL_SIDE_BACK: u32 = 6u32;
pub const CM_DEVICE_PANEL_SIDE_BOTTOM: u32 = 2u32;
pub const CM_DEVICE_PANEL_SIDE_FRONT: u32 = 5u32;
pub const CM_DEVICE_PANEL_SIDE_LEFT: u32 = 3u32;
pub const CM_DEVICE_PANEL_SIDE_RIGHT: u32 = 4u32;
pub const CM_DEVICE_PANEL_SIDE_TOP: u32 = 1u32;
pub const CM_DEVICE_PANEL_SIDE_UNKNOWN: u32 = 0u32;
pub const CM_DISABLE_ABSOLUTE: u32 = 1u32;
pub const CM_DISABLE_BITS: u32 = 15u32;
pub const CM_DISABLE_HARDWARE: u32 = 2u32;
pub const CM_DISABLE_PERSIST: u32 = 8u32;
pub const CM_DISABLE_POLITE: u32 = 0u32;
pub const CM_DISABLE_UI_NOT_OK: u32 = 4u32;
pub const CM_DRP_ADDRESS: u32 = 29u32;
pub const CM_DRP_BASE_CONTAINERID: u32 = 37u32;
pub const CM_DRP_BUSNUMBER: u32 = 22u32;
pub const CM_DRP_BUSTYPEGUID: u32 = 20u32;
pub const CM_DRP_CAPABILITIES: u32 = 16u32;
pub const CM_DRP_CHARACTERISTICS: u32 = 28u32;
pub const CM_DRP_CLASS: u32 = 8u32;
pub const CM_DRP_CLASSGUID: u32 = 9u32;
pub const CM_DRP_COMPATIBLEIDS: u32 = 3u32;
pub const CM_DRP_CONFIGFLAGS: u32 = 11u32;
pub const CM_DRP_DEVICEDESC: u32 = 1u32;
pub const CM_DRP_DEVICE_POWER_DATA: u32 = 31u32;
pub const CM_DRP_DEVTYPE: u32 = 26u32;
pub const CM_DRP_DRIVER: u32 = 10u32;
pub const CM_DRP_ENUMERATOR_NAME: u32 = 23u32;
pub const CM_DRP_EXCLUSIVE: u32 = 27u32;
pub const CM_DRP_FRIENDLYNAME: u32 = 13u32;
pub const CM_DRP_HARDWAREID: u32 = 2u32;
pub const CM_DRP_INSTALL_STATE: u32 = 35u32;
pub const CM_DRP_LEGACYBUSTYPE: u32 = 21u32;
pub const CM_DRP_LOCATION_INFORMATION: u32 = 14u32;
pub const CM_DRP_LOCATION_PATHS: u32 = 36u32;
pub const CM_DRP_LOWERFILTERS: u32 = 19u32;
pub const CM_DRP_MAX: u32 = 37u32;
pub const CM_DRP_MFG: u32 = 12u32;
pub const CM_DRP_MIN: u32 = 1u32;
pub const CM_DRP_PHYSICAL_DEVICE_OBJECT_NAME: u32 = 15u32;
pub const CM_DRP_REMOVAL_POLICY: u32 = 32u32;
pub const CM_DRP_REMOVAL_POLICY_HW_DEFAULT: u32 = 33u32;
pub const CM_DRP_REMOVAL_POLICY_OVERRIDE: u32 = 34u32;
pub const CM_DRP_SECURITY: u32 = 24u32;
pub const CM_DRP_SECURITY_SDS: u32 = 25u32;
pub const CM_DRP_SERVICE: u32 = 5u32;
pub const CM_DRP_UI_NUMBER: u32 = 17u32;
pub const CM_DRP_UI_NUMBER_DESC_FORMAT: u32 = 30u32;
pub const CM_DRP_UNUSED0: u32 = 4u32;
pub const CM_DRP_UNUSED1: u32 = 6u32;
pub const CM_DRP_UNUSED2: u32 = 7u32;
pub const CM_DRP_UPPERFILTERS: u32 = 18u32;
pub const CM_ENUMERATE_CLASSES_BITS: u32 = 1u32;
pub const CM_ENUMERATE_CLASSES_INSTALLER: u32 = 0u32;
pub const CM_ENUMERATE_CLASSES_INTERFACE: u32 = 1u32;
pub const CM_GETIDLIST_DONOTGENERATE: u32 = 268435520u32;
pub const CM_GETIDLIST_FILTER_BITS: u32 = 268435583u32;
pub const CM_GETIDLIST_FILTER_BUSRELATIONS: u32 = 32u32;
pub const CM_GETIDLIST_FILTER_CLASS: u32 = 512u32;
pub const CM_GETIDLIST_FILTER_EJECTRELATIONS: u32 = 4u32;
pub const CM_GETIDLIST_FILTER_ENUMERATOR: u32 = 1u32;
pub const CM_GETIDLIST_FILTER_NONE: u32 = 0u32;
pub const CM_GETIDLIST_FILTER_POWERRELATIONS: u32 = 16u32;
pub const CM_GETIDLIST_FILTER_PRESENT: u32 = 256u32;
pub const CM_GETIDLIST_FILTER_REMOVALRELATIONS: u32 = 8u32;
pub const CM_GETIDLIST_FILTER_SERVICE: u32 = 2u32;
pub const CM_GETIDLIST_FILTER_TRANSPORTRELATIONS: u32 = 128u32;
pub const CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES: u32 = 1u32;
pub const CM_GET_DEVICE_INTERFACE_LIST_BITS: u32 = 1u32;
pub const CM_GET_DEVICE_INTERFACE_LIST_PRESENT: u32 = 0u32;
pub const CM_GLOBAL_STATE_CAN_DO_UI: u32 = 1u32;
pub const CM_GLOBAL_STATE_DETECTION_PENDING: u32 = 16u32;
pub const CM_GLOBAL_STATE_ON_BIG_STACK: u32 = 2u32;
pub const CM_GLOBAL_STATE_REBOOT_REQUIRED: u32 = 32u32;
pub const CM_GLOBAL_STATE_SERVICES_AVAILABLE: u32 = 4u32;
pub const CM_GLOBAL_STATE_SHUTTING_DOWN: u32 = 8u32;
pub const CM_HWPI_DOCKED: u32 = 2u32;
pub const CM_HWPI_NOT_DOCKABLE: u32 = 0u32;
pub const CM_HWPI_UNDOCKED: u32 = 1u32;
pub const CM_INSTALL_STATE_FAILED_INSTALL: u32 = 2u32;
pub const CM_INSTALL_STATE_FINISH_INSTALL: u32 = 3u32;
pub const CM_INSTALL_STATE_INSTALLED: u32 = 0u32;
pub const CM_INSTALL_STATE_NEEDS_REINSTALL: u32 = 1u32;
pub const CM_LOCATE_DEVINST_BITS: u32 = 7u32;
pub const CM_LOCATE_DEVINST_CANCELREMOVE: u32 = 2u32;
pub const CM_LOCATE_DEVINST_NORMAL: u32 = 0u32;
pub const CM_LOCATE_DEVINST_NOVALIDATION: u32 = 4u32;
pub const CM_LOCATE_DEVINST_PHANTOM: u32 = 1u32;
pub const CM_LOCATE_DEVNODE_BITS: u32 = 7u32;
pub const CM_LOCATE_DEVNODE_CANCELREMOVE: u32 = 2u32;
pub const CM_LOCATE_DEVNODE_NORMAL: u32 = 0u32;
pub const CM_LOCATE_DEVNODE_NOVALIDATION: u32 = 4u32;
pub const CM_LOCATE_DEVNODE_PHANTOM: u32 = 1u32;
pub const CM_NAME_ATTRIBUTE_NAME_RETRIEVED_FROM_DEVICE: u32 = 1u32;
pub const CM_NAME_ATTRIBUTE_USER_ASSIGNED_NAME: u32 = 2u32;
pub type CM_NOTIFY_ACTION = i32;
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEARRIVAL: CM_NOTIFY_ACTION = 0i32;
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEREMOVAL: CM_NOTIFY_ACTION = 1i32;
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVE: CM_NOTIFY_ACTION = 2i32;
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVEFAILED: CM_NOTIFY_ACTION = 3i32;
pub const CM_NOTIFY_ACTION_DEVICEREMOVEPENDING: CM_NOTIFY_ACTION = 4i32;
pub const CM_NOTIFY_ACTION_DEVICEREMOVECOMPLETE: CM_NOTIFY_ACTION = 5i32;
pub const CM_NOTIFY_ACTION_DEVICECUSTOMEVENT: CM_NOTIFY_ACTION = 6i32;
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEENUMERATED: CM_NOTIFY_ACTION = 7i32;
pub const CM_NOTIFY_ACTION_DEVICEINSTANCESTARTED: CM_NOTIFY_ACTION = 8i32;
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEREMOVED: CM_NOTIFY_ACTION = 9i32;
pub const CM_NOTIFY_ACTION_MAX: CM_NOTIFY_ACTION = 10i32;
#[repr(C)]
pub struct CM_NOTIFY_EVENT_DATA {
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_EVENT_DATA_0,
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union CM_NOTIFY_EVENT_DATA_0 {
    pub DeviceInterface: CM_NOTIFY_EVENT_DATA_0_2,
    pub DeviceHandle: CM_NOTIFY_EVENT_DATA_0_0,
    pub DeviceInstance: CM_NOTIFY_EVENT_DATA_0_1,
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CM_NOTIFY_EVENT_DATA_0_0 {
    pub EventGuid: ::windows_sys::core::GUID,
    pub NameOffset: i32,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0_0 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CM_NOTIFY_EVENT_DATA_0_1 {
    pub InstanceId: [u16; 1],
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0_1 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CM_NOTIFY_EVENT_DATA_0_2 {
    pub ClassGuid: ::windows_sys::core::GUID,
    pub SymbolicLink: [u16; 1],
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0_2 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER {
    pub cbSize: u32,
    pub Flags: u32,
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_FILTER_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CM_NOTIFY_FILTER_0 {
    pub DeviceInterface: CM_NOTIFY_FILTER_0_2,
    pub DeviceHandle: CM_NOTIFY_FILTER_0_0,
    pub DeviceInstance: CM_NOTIFY_FILTER_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER_0_0 {
    pub hTarget: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER_0_1 {
    pub InstanceId: [u16; 200],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER_0_2 {
    pub ClassGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CM_NOTIFY_FILTER_FLAG_ALL_DEVICE_INSTANCES: u32 = 2u32;
pub const CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES: u32 = 1u32;
pub type CM_NOTIFY_FILTER_TYPE = i32;
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINTERFACE: CM_NOTIFY_FILTER_TYPE = 0i32;
pub const CM_NOTIFY_FILTER_TYPE_DEVICEHANDLE: CM_NOTIFY_FILTER_TYPE = 1i32;
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINSTANCE: CM_NOTIFY_FILTER_TYPE = 2i32;
pub const CM_NOTIFY_FILTER_TYPE_MAX: CM_NOTIFY_FILTER_TYPE = 3i32;
pub const CM_OPEN_CLASS_KEY_BITS: u32 = 1u32;
pub const CM_OPEN_CLASS_KEY_INSTALLER: u32 = 0u32;
pub const CM_OPEN_CLASS_KEY_INTERFACE: u32 = 1u32;
pub const CM_PROB_BIOS_TABLE: u32 = 35u32;
pub const CM_PROB_BOOT_CONFIG_CONFLICT: u32 = 6u32;
pub const CM_PROB_CANT_SHARE_IRQ: u32 = 30u32;
pub const CM_PROB_CONSOLE_LOCKED: u32 = 55u32;
pub const CM_PROB_DEVICE_NOT_THERE: u32 = 24u32;
pub const CM_PROB_DEVICE_RESET: u32 = 54u32;
pub const CM_PROB_DEVLOADER_FAILED: u32 = 2u32;
pub const CM_PROB_DEVLOADER_NOT_FOUND: u32 = 8u32;
pub const CM_PROB_DEVLOADER_NOT_READY: u32 = 23u32;
pub const CM_PROB_DISABLED: u32 = 22u32;
pub const CM_PROB_DISABLED_SERVICE: u32 = 32u32;
pub const CM_PROB_DRIVER_BLOCKED: u32 = 48u32;
pub const CM_PROB_DRIVER_FAILED_LOAD: u32 = 39u32;
pub const CM_PROB_DRIVER_FAILED_PRIOR_UNLOAD: u32 = 38u32;
pub const CM_PROB_DRIVER_SERVICE_KEY_INVALID: u32 = 40u32;
pub const CM_PROB_DUPLICATE_DEVICE: u32 = 42u32;
pub const CM_PROB_ENTRY_IS_WRONG_TYPE: u32 = 4u32;
pub const CM_PROB_FAILED_ADD: u32 = 31u32;
pub const CM_PROB_FAILED_DRIVER_ENTRY: u32 = 37u32;
pub const CM_PROB_FAILED_FILTER: u32 = 7u32;
pub const CM_PROB_FAILED_INSTALL: u32 = 28u32;
pub const CM_PROB_FAILED_POST_START: u32 = 43u32;
pub const CM_PROB_FAILED_START: u32 = 10u32;
pub const CM_PROB_GUEST_ASSIGNMENT_FAILED: u32 = 57u32;
pub const CM_PROB_HALTED: u32 = 44u32;
pub const CM_PROB_HARDWARE_DISABLED: u32 = 29u32;
pub const CM_PROB_HELD_FOR_EJECT: u32 = 47u32;
pub const CM_PROB_INVALID_DATA: u32 = 9u32;
pub const CM_PROB_IRQ_TRANSLATION_FAILED: u32 = 36u32;
pub const CM_PROB_LACKED_ARBITRATOR: u32 = 5u32;
pub const CM_PROB_LEGACY_SERVICE_NO_DEVICES: u32 = 41u32;
pub const CM_PROB_LIAR: u32 = 11u32;
pub const CM_PROB_MOVED: u32 = 25u32;
pub const CM_PROB_NEED_CLASS_CONFIG: u32 = 56u32;
pub const CM_PROB_NEED_RESTART: u32 = 14u32;
pub const CM_PROB_NORMAL_CONFLICT: u32 = 12u32;
pub const CM_PROB_NOT_CONFIGURED: u32 = 1u32;
pub const CM_PROB_NOT_VERIFIED: u32 = 13u32;
pub const CM_PROB_NO_SOFTCONFIG: u32 = 34u32;
pub const CM_PROB_NO_VALID_LOG_CONF: u32 = 27u32;
pub const CM_PROB_OUT_OF_MEMORY: u32 = 3u32;
pub const CM_PROB_PARTIAL_LOG_CONF: u32 = 16u32;
pub const CM_PROB_PHANTOM: u32 = 45u32;
pub const CM_PROB_REENUMERATION: u32 = 15u32;
pub const CM_PROB_REGISTRY: u32 = 19u32;
pub const CM_PROB_REGISTRY_TOO_LARGE: u32 = 49u32;
pub const CM_PROB_REINSTALL: u32 = 18u32;
pub const CM_PROB_SETPROPERTIES_FAILED: u32 = 50u32;
pub const CM_PROB_SYSTEM_SHUTDOWN: u32 = 46u32;
pub const CM_PROB_TOO_EARLY: u32 = 26u32;
pub const CM_PROB_TRANSLATION_FAILED: u32 = 33u32;
pub const CM_PROB_UNKNOWN_RESOURCE: u32 = 17u32;
pub const CM_PROB_UNSIGNED_DRIVER: u32 = 52u32;
pub const CM_PROB_USED_BY_DEBUGGER: u32 = 53u32;
pub const CM_PROB_VXDLDR: u32 = 20u32;
pub const CM_PROB_WAITING_ON_DEPENDENCY: u32 = 51u32;
pub const CM_PROB_WILL_BE_REMOVED: u32 = 21u32;
pub const CM_QUERY_ARBITRATOR_BITS: u32 = 1u32;
pub const CM_QUERY_ARBITRATOR_RAW: u32 = 0u32;
pub const CM_QUERY_ARBITRATOR_TRANSLATED: u32 = 1u32;
pub const CM_QUERY_REMOVE_UI_NOT_OK: u32 = 1u32;
pub const CM_QUERY_REMOVE_UI_OK: u32 = 0u32;
pub const CM_REENUMERATE_ASYNCHRONOUS: u32 = 4u32;
pub const CM_REENUMERATE_BITS: u32 = 7u32;
pub const CM_REENUMERATE_NORMAL: u32 = 0u32;
pub const CM_REENUMERATE_RETRY_INSTALLATION: u32 = 2u32;
pub const CM_REENUMERATE_SYNCHRONOUS: u32 = 1u32;
pub const CM_REGISTER_DEVICE_DRIVER_BITS: u32 = 3u32;
pub const CM_REGISTER_DEVICE_DRIVER_DISABLEABLE: u32 = 1u32;
pub const CM_REGISTER_DEVICE_DRIVER_REMOVABLE: u32 = 2u32;
pub const CM_REGISTER_DEVICE_DRIVER_STATIC: u32 = 0u32;
pub const CM_REGISTRY_BITS: u32 = 769u32;
pub const CM_REGISTRY_CONFIG: u32 = 512u32;
pub const CM_REGISTRY_HARDWARE: u32 = 0u32;
pub const CM_REGISTRY_SOFTWARE: u32 = 1u32;
pub const CM_REGISTRY_USER: u32 = 256u32;
pub const CM_REMOVAL_POLICY_EXPECT_NO_REMOVAL: u32 = 1u32;
pub const CM_REMOVAL_POLICY_EXPECT_ORDERLY_REMOVAL: u32 = 2u32;
pub const CM_REMOVAL_POLICY_EXPECT_SURPRISE_REMOVAL: u32 = 3u32;
pub const CM_REMOVE_BITS: u32 = 7u32;
pub const CM_REMOVE_DISABLE: u32 = 4u32;
pub const CM_REMOVE_NO_RESTART: u32 = 2u32;
pub const CM_REMOVE_UI_NOT_OK: u32 = 1u32;
pub const CM_REMOVE_UI_OK: u32 = 0u32;
pub const CM_RESDES_WIDTH_32: u32 = 1u32;
pub const CM_RESDES_WIDTH_64: u32 = 2u32;
pub const CM_RESDES_WIDTH_BITS: u32 = 3u32;
pub const CM_RESDES_WIDTH_DEFAULT: u32 = 0u32;
pub const CM_SETUP_BITS: u32 = 15u32;
pub const CM_SETUP_DEVINST_CONFIG: u32 = 5u32;
pub const CM_SETUP_DEVINST_CONFIG_CLASS: u32 = 6u32;
pub const CM_SETUP_DEVINST_CONFIG_EXTENSIONS: u32 = 7u32;
pub const CM_SETUP_DEVINST_CONFIG_RESET: u32 = 8u32;
pub const CM_SETUP_DEVINST_READY: u32 = 0u32;
pub const CM_SETUP_DEVINST_RESET: u32 = 4u32;
pub const CM_SETUP_DEVNODE_CONFIG: u32 = 5u32;
pub const CM_SETUP_DEVNODE_CONFIG_CLASS: u32 = 6u32;
pub const CM_SETUP_DEVNODE_CONFIG_EXTENSIONS: u32 = 7u32;
pub const CM_SETUP_DEVNODE_CONFIG_RESET: u32 = 8u32;
pub const CM_SETUP_DEVNODE_READY: u32 = 0u32;
pub const CM_SETUP_DEVNODE_RESET: u32 = 4u32;
pub const CM_SETUP_DOWNLOAD: u32 = 1u32;
pub const CM_SETUP_PROP_CHANGE: u32 = 3u32;
pub const CM_SETUP_WRITE_LOG_CONFS: u32 = 2u32;
pub const CM_SET_DEVINST_PROBLEM_BITS: u32 = 1u32;
pub const CM_SET_DEVINST_PROBLEM_NORMAL: u32 = 0u32;
pub const CM_SET_DEVINST_PROBLEM_OVERRIDE: u32 = 1u32;
pub const CM_SET_DEVNODE_PROBLEM_BITS: u32 = 1u32;
pub const CM_SET_DEVNODE_PROBLEM_NORMAL: u32 = 0u32;
pub const CM_SET_DEVNODE_PROBLEM_OVERRIDE: u32 = 1u32;
pub const CM_SET_HW_PROF_FLAGS_BITS: u32 = 1u32;
pub const CM_SET_HW_PROF_FLAGS_UI_NOT_OK: u32 = 1u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: super::super::Foundation::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COINSTALLER_CONTEXT_DATA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COINSTALLER_CONTEXT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: super::super::Foundation::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COINSTALLER_CONTEXT_DATA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COINSTALLER_CONTEXT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CONFIGMG_VERSION: u32 = 1024u32;
pub type CONFIGRET = u32;
pub const CR_SUCCESS: CONFIGRET = 0u32;
pub const CR_DEFAULT: CONFIGRET = 1u32;
pub const CR_OUT_OF_MEMORY: CONFIGRET = 2u32;
pub const CR_INVALID_POINTER: CONFIGRET = 3u32;
pub const CR_INVALID_FLAG: CONFIGRET = 4u32;
pub const CR_INVALID_DEVNODE: CONFIGRET = 5u32;
pub const CR_INVALID_DEVINST: CONFIGRET = 5u32;
pub const CR_INVALID_RES_DES: CONFIGRET = 6u32;
pub const CR_INVALID_LOG_CONF: CONFIGRET = 7u32;
pub const CR_INVALID_ARBITRATOR: CONFIGRET = 8u32;
pub const CR_INVALID_NODELIST: CONFIGRET = 9u32;
pub const CR_DEVNODE_HAS_REQS: CONFIGRET = 10u32;
pub const CR_DEVINST_HAS_REQS: CONFIGRET = 10u32;
pub const CR_INVALID_RESOURCEID: CONFIGRET = 11u32;
pub const CR_DLVXD_NOT_FOUND: CONFIGRET = 12u32;
pub const CR_NO_SUCH_DEVNODE: CONFIGRET = 13u32;
pub const CR_NO_SUCH_DEVINST: CONFIGRET = 13u32;
pub const CR_NO_MORE_LOG_CONF: CONFIGRET = 14u32;
pub const CR_NO_MORE_RES_DES: CONFIGRET = 15u32;
pub const CR_ALREADY_SUCH_DEVNODE: CONFIGRET = 16u32;
pub const CR_ALREADY_SUCH_DEVINST: CONFIGRET = 16u32;
pub const CR_INVALID_RANGE_LIST: CONFIGRET = 17u32;
pub const CR_INVALID_RANGE: CONFIGRET = 18u32;
pub const CR_FAILURE: CONFIGRET = 19u32;
pub const CR_NO_SUCH_LOGICAL_DEV: CONFIGRET = 20u32;
pub const CR_CREATE_BLOCKED: CONFIGRET = 21u32;
pub const CR_NOT_SYSTEM_VM: CONFIGRET = 22u32;
pub const CR_REMOVE_VETOED: CONFIGRET = 23u32;
pub const CR_APM_VETOED: CONFIGRET = 24u32;
pub const CR_INVALID_LOAD_TYPE: CONFIGRET = 25u32;
pub const CR_BUFFER_SMALL: CONFIGRET = 26u32;
pub const CR_NO_ARBITRATOR: CONFIGRET = 27u32;
pub const CR_NO_REGISTRY_HANDLE: CONFIGRET = 28u32;
pub const CR_REGISTRY_ERROR: CONFIGRET = 29u32;
pub const CR_INVALID_DEVICE_ID: CONFIGRET = 30u32;
pub const CR_INVALID_DATA: CONFIGRET = 31u32;
pub const CR_INVALID_API: CONFIGRET = 32u32;
pub const CR_DEVLOADER_NOT_READY: CONFIGRET = 33u32;
pub const CR_NEED_RESTART: CONFIGRET = 34u32;
pub const CR_NO_MORE_HW_PROFILES: CONFIGRET = 35u32;
pub const CR_DEVICE_NOT_THERE: CONFIGRET = 36u32;
pub const CR_NO_SUCH_VALUE: CONFIGRET = 37u32;
pub const CR_WRONG_TYPE: CONFIGRET = 38u32;
pub const CR_INVALID_PRIORITY: CONFIGRET = 39u32;
pub const CR_NOT_DISABLEABLE: CONFIGRET = 40u32;
pub const CR_FREE_RESOURCES: CONFIGRET = 41u32;
pub const CR_QUERY_VETOED: CONFIGRET = 42u32;
pub const CR_CANT_SHARE_IRQ: CONFIGRET = 43u32;
pub const CR_NO_DEPENDENT: CONFIGRET = 44u32;
pub const CR_SAME_RESOURCES: CONFIGRET = 45u32;
pub const CR_NO_SUCH_REGISTRY_KEY: CONFIGRET = 46u32;
pub const CR_INVALID_MACHINENAME: CONFIGRET = 47u32;
pub const CR_REMOTE_COMM_FAILURE: CONFIGRET = 48u32;
pub const CR_MACHINE_UNAVAILABLE: CONFIGRET = 49u32;
pub const CR_NO_CM_SERVICES: CONFIGRET = 50u32;
pub const CR_ACCESS_DENIED: CONFIGRET = 51u32;
pub const CR_CALL_NOT_IMPLEMENTED: CONFIGRET = 52u32;
pub const CR_INVALID_PROPERTY: CONFIGRET = 53u32;
pub const CR_DEVICE_INTERFACE_ACTIVE: CONFIGRET = 54u32;
pub const CR_NO_SUCH_DEVICE_INTERFACE: CONFIGRET = 55u32;
pub const CR_INVALID_REFERENCE_STRING: CONFIGRET = 56u32;
pub const CR_INVALID_CONFLICT_LIST: CONFIGRET = 57u32;
pub const CR_INVALID_INDEX: CONFIGRET = 58u32;
pub const CR_INVALID_STRUCTURE_SIZE: CONFIGRET = 59u32;
pub const NUM_CR_RESULTS: CONFIGRET = 60u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONFLICT_DETAILS_A {
    pub CD_ulSize: u32,
    pub CD_ulMask: u32,
    pub CD_dnDevInst: u32,
    pub CD_rdResDes: usize,
    pub CD_ulFlags: u32,
    pub CD_szDescription: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONFLICT_DETAILS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONFLICT_DETAILS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CONFLICT_DETAILS_W {
    pub CD_ulSize: u32,
    pub CD_ulMask: u32,
    pub CD_dnDevInst: u32,
    pub CD_rdResDes: usize,
    pub CD_ulFlags: u32,
    pub CD_szDescription: [u16; 260],
}
impl ::core::marker::Copy for CONFLICT_DETAILS_W {}
impl ::core::clone::Clone for CONFLICT_DETAILS_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const COPYFLG_FORCE_FILE_IN_USE: u32 = 8u32;
pub const COPYFLG_IN_USE_TRY_RENAME: u32 = 16384u32;
pub const COPYFLG_NODECOMP: u32 = 2048u32;
pub const COPYFLG_NOPRUNE: u32 = 8192u32;
pub const COPYFLG_NOSKIP: u32 = 2u32;
pub const COPYFLG_NOVERSIONCHECK: u32 = 4u32;
pub const COPYFLG_NO_OVERWRITE: u32 = 16u32;
pub const COPYFLG_NO_VERSION_DIALOG: u32 = 32u32;
pub const COPYFLG_OVERWRITE_OLDER_ONLY: u32 = 64u32;
pub const COPYFLG_PROTECTED_WINDOWS_DRIVER_FILE: u32 = 256u32;
pub const COPYFLG_REPLACEONLY: u32 = 1024u32;
pub const COPYFLG_REPLACE_BOOT_FILE: u32 = 4096u32;
pub const COPYFLG_WARN_IF_SKIP: u32 = 1u32;
#[repr(C, packed(1))]
pub struct CS_DES {
    pub CSD_SignatureLength: u32,
    pub CSD_LegacyDataOffset: u32,
    pub CSD_LegacyDataSize: u32,
    pub CSD_Flags: u32,
    pub CSD_ClassGuid: ::windows_sys::core::GUID,
    pub CSD_Signature: [u8; 1],
}
impl ::core::marker::Copy for CS_DES {}
impl ::core::clone::Clone for CS_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CS_RESOURCE {
    pub CS_Header: CS_DES,
}
impl ::core::marker::Copy for CS_RESOURCE {}
impl ::core::clone::Clone for CS_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct Connection_Des_s {
    pub COND_Type: u32,
    pub COND_Flags: u32,
    pub COND_Class: u8,
    pub COND_ClassType: u8,
    pub COND_Reserved1: u8,
    pub COND_Reserved2: u8,
    pub COND_Id: i64,
}
impl ::core::marker::Copy for Connection_Des_s {}
impl ::core::clone::Clone for Connection_Des_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct Connection_Resource_s {
    pub Connection_Header: Connection_Des_s,
}
impl ::core::marker::Copy for Connection_Resource_s {}
impl ::core::clone::Clone for Connection_Resource_s {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DELFLG_IN_USE: u32 = 1u32;
pub const DELFLG_IN_USE1: u32 = 65536u32;
pub const DIBCI_NODISPLAYCLASS: u32 = 2u32;
pub const DIBCI_NOINSTALLCLASS: u32 = 1u32;
pub const DICD_GENERATE_ID: u32 = 1u32;
pub const DICD_INHERIT_CLASSDRVS: u32 = 2u32;
pub const DICLASSPROP_INSTALLER: u32 = 1u32;
pub const DICLASSPROP_INTERFACE: u32 = 2u32;
pub const DICS_DISABLE: u32 = 2u32;
pub const DICS_ENABLE: u32 = 1u32;
pub const DICS_FLAG_CONFIGGENERAL: u32 = 4u32;
pub const DICS_FLAG_CONFIGSPECIFIC: u32 = 2u32;
pub const DICS_FLAG_GLOBAL: u32 = 1u32;
pub const DICS_PROPCHANGE: u32 = 3u32;
pub const DICS_START: u32 = 4u32;
pub const DICS_STOP: u32 = 5u32;
pub const DICUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1u32;
pub const DIF_ADDPROPERTYPAGE_ADVANCED: u32 = 35u32;
pub const DIF_ADDPROPERTYPAGE_BASIC: u32 = 36u32;
pub const DIF_ADDREMOTEPROPERTYPAGE_ADVANCED: u32 = 40u32;
pub const DIF_ALLOW_INSTALL: u32 = 24u32;
pub const DIF_ASSIGNRESOURCES: u32 = 3u32;
pub const DIF_CALCDISKSPACE: u32 = 11u32;
pub const DIF_DESTROYPRIVATEDATA: u32 = 12u32;
pub const DIF_DESTROYWIZARDDATA: u32 = 17u32;
pub const DIF_DETECT: u32 = 15u32;
pub const DIF_DETECTCANCEL: u32 = 33u32;
pub const DIF_DETECTVERIFY: u32 = 20u32;
pub const DIF_ENABLECLASS: u32 = 19u32;
pub const DIF_FINISHINSTALL_ACTION: u32 = 42u32;
pub const DIF_FIRSTTIMESETUP: u32 = 6u32;
pub const DIF_FOUNDDEVICE: u32 = 7u32;
pub const DIF_INSTALLCLASSDRIVERS: u32 = 10u32;
pub const DIF_INSTALLDEVICE: u32 = 2u32;
pub const DIF_INSTALLDEVICEFILES: u32 = 21u32;
pub const DIF_INSTALLINTERFACES: u32 = 32u32;
pub const DIF_INSTALLWIZARD: u32 = 16u32;
pub const DIF_MOVEDEVICE: u32 = 14u32;
pub const DIF_NEWDEVICEWIZARD_FINISHINSTALL: u32 = 30u32;
pub const DIF_NEWDEVICEWIZARD_POSTANALYZE: u32 = 29u32;
pub const DIF_NEWDEVICEWIZARD_PREANALYZE: u32 = 28u32;
pub const DIF_NEWDEVICEWIZARD_PRESELECT: u32 = 26u32;
pub const DIF_NEWDEVICEWIZARD_SELECT: u32 = 27u32;
pub const DIF_POWERMESSAGEWAKE: u32 = 39u32;
pub const DIF_PROPERTIES: u32 = 4u32;
pub const DIF_PROPERTYCHANGE: u32 = 18u32;
pub const DIF_REGISTERDEVICE: u32 = 25u32;
pub const DIF_REGISTER_COINSTALLERS: u32 = 34u32;
pub const DIF_REMOVE: u32 = 5u32;
pub const DIF_RESERVED1: u32 = 37u32;
pub const DIF_RESERVED2: u32 = 48u32;
pub const DIF_SELECTBESTCOMPATDRV: u32 = 23u32;
pub const DIF_SELECTCLASSDRIVERS: u32 = 8u32;
pub const DIF_SELECTDEVICE: u32 = 1u32;
pub const DIF_TROUBLESHOOTER: u32 = 38u32;
pub const DIF_UNREMOVE: u32 = 22u32;
pub const DIF_UNUSED1: u32 = 31u32;
pub const DIF_UPDATEDRIVER_UI: u32 = 41u32;
pub const DIF_VALIDATECLASSDRIVERS: u32 = 9u32;
pub const DIF_VALIDATEDRIVER: u32 = 13u32;
pub const DIGCDP_FLAG_ADVANCED: u32 = 2u32;
pub const DIGCDP_FLAG_BASIC: u32 = 1u32;
pub const DIGCDP_FLAG_REMOTE_ADVANCED: u32 = 4u32;
pub const DIGCDP_FLAG_REMOTE_BASIC: u32 = 3u32;
pub const DIGCF_ALLCLASSES: u32 = 4u32;
pub const DIGCF_DEFAULT: u32 = 1u32;
pub const DIGCF_DEVICEINTERFACE: u32 = 16u32;
pub const DIGCF_INTERFACEDEVICE: u32 = 16u32;
pub const DIGCF_PRESENT: u32 = 2u32;
pub const DIGCF_PROFILE: u32 = 8u32;
pub const DIIDFLAG_BITS: u32 = 15u32;
pub const DIIDFLAG_INSTALLCOPYINFDRIVERS: u32 = 8u32;
pub const DIIDFLAG_INSTALLNULLDRIVER: u32 = 4u32;
pub const DIIDFLAG_NOFINISHINSTALLUI: u32 = 2u32;
pub const DIIDFLAG_SHOWSEARCHUI: u32 = 1u32;
pub const DIIRFLAG_FORCE_INF: u32 = 2u32;
pub const DIIRFLAG_HOTPATCH: u32 = 8u32;
pub const DIIRFLAG_HW_USING_THE_INF: u32 = 4u32;
pub const DIIRFLAG_INF_ALREADY_COPIED: u32 = 1u32;
pub const DIIRFLAG_INSTALL_AS_SET: u32 = 64u32;
pub const DIIRFLAG_NOBACKUP: u32 = 16u32;
pub const DIIRFLAG_PRE_CONFIGURE_INF: u32 = 32u32;
pub const DIOCR_INSTALLER: u32 = 1u32;
pub const DIOCR_INTERFACE: u32 = 2u32;
pub const DIODI_NO_ADD: u32 = 1u32;
pub const DIOD_CANCEL_REMOVE: u32 = 4u32;
pub const DIOD_INHERIT_CLASSDRVS: u32 = 2u32;
pub const DIREG_BOTH: u32 = 4u32;
pub const DIREG_DEV: u32 = 1u32;
pub const DIREG_DRV: u32 = 2u32;
pub const DIRID_ABSOLUTE: i32 = -1i32;
pub const DIRID_ABSOLUTE_16BIT: u32 = 65535u32;
pub const DIRID_APPS: u32 = 24u32;
pub const DIRID_BOOT: u32 = 30u32;
pub const DIRID_COLOR: u32 = 23u32;
pub const DIRID_COMMON_APPDATA: u32 = 16419u32;
pub const DIRID_COMMON_DESKTOPDIRECTORY: u32 = 16409u32;
pub const DIRID_COMMON_DOCUMENTS: u32 = 16430u32;
pub const DIRID_COMMON_FAVORITES: u32 = 16415u32;
pub const DIRID_COMMON_PROGRAMS: u32 = 16407u32;
pub const DIRID_COMMON_STARTMENU: u32 = 16406u32;
pub const DIRID_COMMON_STARTUP: u32 = 16408u32;
pub const DIRID_COMMON_TEMPLATES: u32 = 16429u32;
pub const DIRID_DEFAULT: u32 = 11u32;
pub const DIRID_DRIVERS: u32 = 12u32;
pub const DIRID_DRIVER_STORE: u32 = 13u32;
pub const DIRID_FONTS: u32 = 20u32;
pub const DIRID_HELP: u32 = 18u32;
pub const DIRID_INF: u32 = 17u32;
pub const DIRID_IOSUBSYS: u32 = 12u32;
pub const DIRID_LOADER: u32 = 54u32;
pub const DIRID_NULL: u32 = 0u32;
pub const DIRID_PRINTPROCESSOR: u32 = 55u32;
pub const DIRID_PROGRAM_FILES: u32 = 16422u32;
pub const DIRID_PROGRAM_FILES_COMMON: u32 = 16427u32;
pub const DIRID_PROGRAM_FILES_COMMONX86: u32 = 16428u32;
pub const DIRID_PROGRAM_FILES_X86: u32 = 16426u32;
pub const DIRID_SHARED: u32 = 25u32;
pub const DIRID_SPOOL: u32 = 51u32;
pub const DIRID_SPOOLDRIVERS: u32 = 52u32;
pub const DIRID_SRCPATH: u32 = 1u32;
pub const DIRID_SYSTEM: u32 = 11u32;
pub const DIRID_SYSTEM16: u32 = 50u32;
pub const DIRID_SYSTEM_X86: u32 = 16425u32;
pub const DIRID_USER: u32 = 32768u32;
pub const DIRID_USERPROFILE: u32 = 53u32;
pub const DIRID_VIEWERS: u32 = 21u32;
pub const DIRID_WINDOWS: u32 = 10u32;
pub const DIURFLAG_NO_REMOVE_INF: u32 = 1u32;
pub const DIURFLAG_RESERVED: u32 = 2u32;
pub const DI_AUTOASSIGNRES: i32 = 64i32;
pub const DI_CLASSINSTALLPARAMS: i32 = 1048576i32;
pub const DI_COMPAT_FROM_CLASS: i32 = 524288i32;
pub const DI_DIDCLASS: i32 = 32i32;
pub const DI_DIDCOMPAT: i32 = 16i32;
pub const DI_DISABLED: i32 = 2048i32;
pub const DI_DONOTCALLCONFIGMG: i32 = 131072i32;
pub const DI_DRIVERPAGE_ADDED: i32 = 67108864i32;
pub const DI_ENUMSINGLEINF: i32 = 65536i32;
pub const DI_FLAGSEX_ALLOWEXCLUDEDDRVS: i32 = 2048i32;
pub const DI_FLAGSEX_ALTPLATFORM_DRVSEARCH: i32 = 268435456i32;
pub const DI_FLAGSEX_ALWAYSWRITEIDS: i32 = 512i32;
pub const DI_FLAGSEX_APPENDDRIVERLIST: i32 = 262144i32;
pub const DI_FLAGSEX_BACKUPONREPLACE: i32 = 1048576i32;
pub const DI_FLAGSEX_CI_FAILED: i32 = 4i32;
pub const DI_FLAGSEX_DEVICECHANGE: i32 = 256i32;
pub const DI_FLAGSEX_DIDCOMPATINFO: i32 = 32i32;
pub const DI_FLAGSEX_DIDINFOLIST: i32 = 16i32;
pub const DI_FLAGSEX_DRIVERLIST_FROM_URL: i32 = 2097152i32;
pub const DI_FLAGSEX_EXCLUDE_OLD_INET_DRIVERS: i32 = 8388608i32;
pub const DI_FLAGSEX_FILTERCLASSES: i32 = 64i32;
pub const DI_FLAGSEX_FILTERSIMILARDRIVERS: i32 = 33554432i32;
pub const DI_FLAGSEX_FINISHINSTALL_ACTION: i32 = 8i32;
pub const DI_FLAGSEX_INET_DRIVER: i32 = 131072i32;
pub const DI_FLAGSEX_INSTALLEDDRIVER: i32 = 67108864i32;
pub const DI_FLAGSEX_IN_SYSTEM_SETUP: i32 = 65536i32;
pub const DI_FLAGSEX_NOUIONQUERYREMOVE: i32 = 4096i32;
pub const DI_FLAGSEX_NO_CLASSLIST_NODE_MERGE: i32 = 134217728i32;
pub const DI_FLAGSEX_NO_DRVREG_MODIFY: i32 = 32768i32;
pub const DI_FLAGSEX_POWERPAGE_ADDED: i32 = 16777216i32;
pub const DI_FLAGSEX_PREINSTALLBACKUP: i32 = 524288i32;
pub const DI_FLAGSEX_PROPCHANGE_PENDING: i32 = 1024i32;
pub const DI_FLAGSEX_RECURSIVESEARCH: i32 = 1073741824i32;
pub const DI_FLAGSEX_RESERVED1: i32 = 4194304i32;
pub const DI_FLAGSEX_RESERVED2: i32 = 1i32;
pub const DI_FLAGSEX_RESERVED3: i32 = 2i32;
pub const DI_FLAGSEX_RESERVED4: i32 = 16384i32;
pub const DI_FLAGSEX_RESTART_DEVICE_ONLY: i32 = 536870912i32;
pub const DI_FLAGSEX_SEARCH_PUBLISHED_INFS: i32 = -2147483648i32;
pub const DI_FLAGSEX_SETFAILEDINSTALL: i32 = 128i32;
pub const DI_FLAGSEX_USECLASSFORCOMPAT: i32 = 8192i32;
pub const DI_FORCECOPY: i32 = 33554432i32;
pub const DI_GENERALPAGE_ADDED: i32 = 4096i32;
pub const DI_INF_IS_SORTED: i32 = 32768i32;
pub const DI_INSTALLDISABLED: i32 = 262144i32;
pub const DI_MULTMFGS: i32 = 1024i32;
pub const DI_NEEDREBOOT: i32 = 256i32;
pub const DI_NEEDRESTART: i32 = 128i32;
pub const DI_NOBROWSE: i32 = 512i32;
pub const DI_NODI_DEFAULTACTION: i32 = 2097152i32;
pub const DI_NOFILECOPY: i32 = 16777216i32;
pub const DI_NOSELECTICONS: i32 = 1073741824i32;
pub const DI_NOVCP: i32 = 8i32;
pub const DI_NOWRITE_IDS: i32 = -2147483648i32;
pub const DI_OVERRIDE_INFFLAGS: i32 = 268435456i32;
pub const DI_PROPERTIES_CHANGE: i32 = 16384i32;
pub const DI_PROPS_NOCHANGEUSAGE: i32 = 536870912i32;
pub const DI_QUIETINSTALL: i32 = 8388608i32;
pub const DI_REMOVEDEVICE_CONFIGSPECIFIC: u32 = 2u32;
pub const DI_REMOVEDEVICE_GLOBAL: u32 = 1u32;
pub const DI_RESOURCEPAGE_ADDED: i32 = 8192i32;
pub const DI_SHOWALL: i32 = 7i32;
pub const DI_SHOWCLASS: i32 = 4i32;
pub const DI_SHOWCOMPAT: i32 = 2i32;
pub const DI_SHOWOEM: i32 = 1i32;
pub const DI_UNREMOVEDEVICE_CONFIGSPECIFIC: u32 = 2u32;
pub const DI_USECI_SELECTSTRINGS: i32 = 134217728i32;
#[repr(C, packed(1))]
pub struct DMA_DES {
    pub DD_Count: u32,
    pub DD_Type: u32,
    pub DD_Flags: u32,
    pub DD_Alloc_Chan: u32,
}
impl ::core::marker::Copy for DMA_DES {}
impl ::core::clone::Clone for DMA_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct DMA_RANGE {
    pub DR_Min: u32,
    pub DR_Max: u32,
    pub DR_Flags: u32,
}
impl ::core::marker::Copy for DMA_RANGE {}
impl ::core::clone::Clone for DMA_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DMA_RESOURCE {
    pub DMA_Header: DMA_DES,
    pub DMA_Data: [DMA_RANGE; 1],
}
impl ::core::marker::Copy for DMA_RESOURCE {}
impl ::core::clone::Clone for DMA_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DMI_BKCOLOR: u32 = 2u32;
pub const DMI_MASK: u32 = 1u32;
pub const DMI_USERECT: u32 = 4u32;
pub const DNF_ALWAYSEXCLUDEFROMLIST: u32 = 524288u32;
pub const DNF_AUTHENTICODE_SIGNED: u32 = 131072u32;
pub const DNF_BAD_DRIVER: u32 = 2048u32;
pub const DNF_BASIC_DRIVER: u32 = 65536u32;
pub const DNF_CLASS_DRIVER: u32 = 32u32;
pub const DNF_COMPATIBLE_DRIVER: u32 = 64u32;
pub const DNF_DUPDESC: u32 = 1u32;
pub const DNF_DUPDRIVERVER: u32 = 32768u32;
pub const DNF_DUPPROVIDER: u32 = 4096u32;
pub const DNF_EXCLUDEFROMLIST: u32 = 4u32;
pub const DNF_INBOX_DRIVER: u32 = 1048576u32;
pub const DNF_INET_DRIVER: u32 = 128u32;
pub const DNF_INF_IS_SIGNED: u32 = 8192u32;
pub const DNF_INSTALLEDDRIVER: u32 = 262144u32;
pub const DNF_LEGACYINF: u32 = 16u32;
pub const DNF_NODRIVER: u32 = 8u32;
pub const DNF_OEM_F6_INF: u32 = 16384u32;
pub const DNF_OLDDRIVER: u32 = 2u32;
pub const DNF_OLD_INET_DRIVER: u32 = 1024u32;
pub const DNF_REQUESTADDITIONALSOFTWARE: u32 = 2097152u32;
pub const DNF_UNUSED1: u32 = 256u32;
pub const DNF_UNUSED2: u32 = 512u32;
pub const DNF_UNUSED_22: u32 = 4194304u32;
pub const DNF_UNUSED_23: u32 = 8388608u32;
pub const DNF_UNUSED_24: u32 = 16777216u32;
pub const DNF_UNUSED_25: u32 = 33554432u32;
pub const DNF_UNUSED_26: u32 = 67108864u32;
pub const DNF_UNUSED_27: u32 = 134217728u32;
pub const DNF_UNUSED_28: u32 = 268435456u32;
pub const DNF_UNUSED_29: u32 = 536870912u32;
pub const DNF_UNUSED_30: u32 = 1073741824u32;
pub const DNF_UNUSED_31: u32 = 2147483648u32;
pub const DN_APM_DRIVER: u32 = 268435456u32;
pub const DN_APM_ENUMERATOR: u32 = 134217728u32;
pub const DN_ARM_WAKEUP: u32 = 67108864u32;
pub const DN_BAD_PARTIAL: u32 = 4194304u32;
pub const DN_BOOT_LOG_PROB: u32 = 2147483648u32;
pub const DN_CHILD_WITH_INVALID_ID: u32 = 512u32;
pub const DN_DEVICE_DISCONNECTED: u32 = 33554432u32;
pub const DN_DISABLEABLE: u32 = 8192u32;
pub const DN_DRIVER_BLOCKED: u32 = 64u32;
pub const DN_DRIVER_LOADED: u32 = 2u32;
pub const DN_ENUM_LOADED: u32 = 4u32;
pub const DN_FILTERED: u32 = 2048u32;
pub const DN_HARDWARE_ENUM: u32 = 128u32;
pub const DN_HAS_MARK: u32 = 512u32;
pub const DN_HAS_PROBLEM: u32 = 1024u32;
pub const DN_LEGACY_DRIVER: u32 = 4096u32;
pub const DN_LIAR: u32 = 256u32;
pub const DN_MANUAL: u32 = 16u32;
pub const DN_MF_CHILD: u32 = 131072u32;
pub const DN_MF_PARENT: u32 = 65536u32;
pub const DN_MOVED: u32 = 4096u32;
pub const DN_NEEDS_LOCKING: u32 = 33554432u32;
pub const DN_NEED_RESTART: u32 = 256u32;
pub const DN_NEED_TO_ENUM: u32 = 32u32;
pub const DN_NOT_FIRST_TIME: u32 = 64u32;
pub const DN_NOT_FIRST_TIMEE: u32 = 524288u32;
pub const DN_NO_SHOW_IN_DM: u32 = 1073741824u32;
pub const DN_NT_DRIVER: u32 = 16777216u32;
pub const DN_NT_ENUMERATOR: u32 = 8388608u32;
pub const DN_PRIVATE_PROBLEM: u32 = 32768u32;
pub const DN_QUERY_REMOVE_ACTIVE: u32 = 131072u32;
pub const DN_QUERY_REMOVE_PENDING: u32 = 65536u32;
pub const DN_REBAL_CANDIDATE: u32 = 2097152u32;
pub const DN_REMOVABLE: u32 = 16384u32;
pub const DN_ROOT_ENUMERATED: u32 = 1u32;
pub const DN_SILENT_INSTALL: u32 = 536870912u32;
pub const DN_STARTED: u32 = 8u32;
pub const DN_STOP_FREE_RES: u32 = 1048576u32;
pub const DN_WILL_BE_REMOVED: u32 = 262144u32;
pub const DPROMPT_BUFFERTOOSMALL: u32 = 3u32;
pub const DPROMPT_CANCEL: u32 = 1u32;
pub const DPROMPT_OUTOFMEMORY: u32 = 4u32;
pub const DPROMPT_SKIPFILE: u32 = 2u32;
pub const DPROMPT_SUCCESS: u32 = 0u32;
pub const DRIVER_COMPATID_RANK: u32 = 16383u32;
pub const DRIVER_HARDWAREID_MASK: u32 = 2147487743u32;
pub const DRIVER_HARDWAREID_RANK: u32 = 4095u32;
pub const DRIVER_UNTRUSTED_COMPATID_RANK: u32 = 49151u32;
pub const DRIVER_UNTRUSTED_HARDWAREID_RANK: u32 = 36863u32;
pub const DRIVER_UNTRUSTED_RANK: u32 = 2147483648u32;
pub const DRIVER_W9X_SUSPECT_COMPATID_RANK: u32 = 65535u32;
pub const DRIVER_W9X_SUSPECT_HARDWAREID_RANK: u32 = 53247u32;
pub const DRIVER_W9X_SUSPECT_RANK: u32 = 3221225472u32;
pub const DWORD_MAX: u32 = 4294967295u32;
pub const DYNAWIZ_FLAG_ANALYZE_HANDLECONFLICT: u32 = 8u32;
pub const DYNAWIZ_FLAG_INSTALLDET_NEXT: u32 = 2u32;
pub const DYNAWIZ_FLAG_INSTALLDET_PREV: u32 = 4u32;
pub const DYNAWIZ_FLAG_PAGESADDED: u32 = 1u32;
#[repr(C, packed(1))]
pub struct DevPrivate_Des_s {
    pub PD_Count: u32,
    pub PD_Type: u32,
    pub PD_Data1: u32,
    pub PD_Data2: u32,
    pub PD_Data3: u32,
    pub PD_Flags: u32,
}
impl ::core::marker::Copy for DevPrivate_Des_s {}
impl ::core::clone::Clone for DevPrivate_Des_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct DevPrivate_Range_s {
    pub PR_Data1: u32,
    pub PR_Data2: u32,
    pub PR_Data3: u32,
}
impl ::core::marker::Copy for DevPrivate_Range_s {}
impl ::core::clone::Clone for DevPrivate_Range_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DevPrivate_Resource_s {
    pub PRV_Header: DevPrivate_Des_s,
    pub PRV_Data: [DevPrivate_Range_s; 1],
}
impl ::core::marker::Copy for DevPrivate_Resource_s {}
impl ::core::clone::Clone for DevPrivate_Resource_s {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ENABLECLASS_FAILURE: u32 = 2u32;
pub const ENABLECLASS_QUERY: u32 = 0u32;
pub const ENABLECLASS_SUCCESS: u32 = 1u32;
pub const FILEOP_ABORT: u32 = 0u32;
pub const FILEOP_BACKUP: u32 = 3u32;
pub const FILEOP_DOIT: u32 = 1u32;
pub const FILEOP_NEWPATH: u32 = 4u32;
pub const FILEOP_RENAME: u32 = 1u32;
pub const FILEOP_RETRY: u32 = 1u32;
pub const FILEOP_SKIP: u32 = 2u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILEPATHS_A {
    pub Target: super::super::Foundation::PSTR,
    pub Source: super::super::Foundation::PSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILEPATHS_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILEPATHS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILEPATHS_A {
    pub Target: super::super::Foundation::PSTR,
    pub Source: super::super::Foundation::PSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILEPATHS_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILEPATHS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: super::super::Foundation::PSTR,
    pub Source: super::super::Foundation::PSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: super::super::Foundation::PSTR,
    pub Version: super::super::Foundation::PSTR,
    pub CatalogFile: super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: super::super::Foundation::PSTR,
    pub Source: super::super::Foundation::PSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: super::super::Foundation::PSTR,
    pub Version: super::super::Foundation::PSTR,
    pub CatalogFile: super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: super::super::Foundation::PWSTR,
    pub Source: super::super::Foundation::PWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: super::super::Foundation::PWSTR,
    pub Version: super::super::Foundation::PWSTR,
    pub CatalogFile: super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: super::super::Foundation::PWSTR,
    pub Source: super::super::Foundation::PWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: super::super::Foundation::PWSTR,
    pub Version: super::super::Foundation::PWSTR,
    pub CatalogFile: super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILEPATHS_W {
    pub Target: super::super::Foundation::PWSTR,
    pub Source: super::super::Foundation::PWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILEPATHS_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILEPATHS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILEPATHS_W {
    pub Target: super::super::Foundation::PWSTR,
    pub Source: super::super::Foundation::PWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILEPATHS_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILEPATHS_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FILE_COMPRESSION_MSZIP: u32 = 2u32;
pub const FILE_COMPRESSION_NONE: u32 = 0u32;
pub const FILE_COMPRESSION_NTCAB: u32 = 3u32;
pub const FILE_COMPRESSION_WINLZA: u32 = 1u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: super::super::Foundation::PSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: super::super::Foundation::PSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: super::super::Foundation::PWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: super::super::Foundation::PWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FILTERED_LOG_CONF: u32 = 1u32;
pub const FLG_ADDPROPERTY_AND: u32 = 16u32;
pub const FLG_ADDPROPERTY_APPEND: u32 = 4u32;
pub const FLG_ADDPROPERTY_NOCLOBBER: u32 = 1u32;
pub const FLG_ADDPROPERTY_OR: u32 = 8u32;
pub const FLG_ADDPROPERTY_OVERWRITEONLY: u32 = 2u32;
pub const FLG_ADDREG_32BITKEY: u32 = 16384u32;
pub const FLG_ADDREG_64BITKEY: u32 = 4096u32;
pub const FLG_ADDREG_APPEND: u32 = 8u32;
pub const FLG_ADDREG_BINVALUETYPE: u32 = 1u32;
pub const FLG_ADDREG_DELREG_BIT: u32 = 32768u32;
pub const FLG_ADDREG_DELVAL: u32 = 4u32;
pub const FLG_ADDREG_KEYONLY: u32 = 16u32;
pub const FLG_ADDREG_KEYONLY_COMMON: u32 = 8192u32;
pub const FLG_ADDREG_NOCLOBBER: u32 = 2u32;
pub const FLG_ADDREG_OVERWRITEONLY: u32 = 32u32;
pub const FLG_ADDREG_TYPE_EXPAND_SZ: u32 = 131072u32;
pub const FLG_ADDREG_TYPE_MULTI_SZ: u32 = 65536u32;
pub const FLG_ADDREG_TYPE_SZ: u32 = 0u32;
pub const FLG_BITREG_32BITKEY: u32 = 16384u32;
pub const FLG_BITREG_64BITKEY: u32 = 4096u32;
pub const FLG_BITREG_CLEARBITS: u32 = 0u32;
pub const FLG_BITREG_SETBITS: u32 = 1u32;
pub const FLG_DELPROPERTY_MULTI_SZ_DELSTRING: u32 = 1u32;
pub const FLG_DELREG_32BITKEY: u32 = 16384u32;
pub const FLG_DELREG_64BITKEY: u32 = 4096u32;
pub const FLG_DELREG_KEYONLY_COMMON: u32 = 8192u32;
pub const FLG_DELREG_OPERATION_MASK: u32 = 254u32;
pub const FLG_DELREG_TYPE_EXPAND_SZ: u32 = 131072u32;
pub const FLG_DELREG_TYPE_MULTI_SZ: u32 = 65536u32;
pub const FLG_DELREG_TYPE_SZ: u32 = 0u32;
pub const FLG_DELREG_VALUE: u32 = 0u32;
pub const FLG_INI2REG_32BITKEY: u32 = 16384u32;
pub const FLG_INI2REG_64BITKEY: u32 = 4096u32;
pub const FLG_PROFITEM_CSIDL: u32 = 8u32;
pub const FLG_PROFITEM_CURRENTUSER: u32 = 1u32;
pub const FLG_PROFITEM_DELETE: u32 = 2u32;
pub const FLG_PROFITEM_GROUP: u32 = 4u32;
pub const FLG_REGSVR_DLLINSTALL: u32 = 2u32;
pub const FLG_REGSVR_DLLREGISTER: u32 = 1u32;
pub const FORCED_LOG_CONF: u32 = 4u32;
pub const GUID_ACPI_CMOS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 982320004, data2: 25861, data3: 16586, data4: [188, 57, 86, 193, 95, 140, 95, 237] };
pub const GUID_ACPI_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2962333834, data2: 47767, data3: 4560, data4: [189, 20, 0, 170, 0, 183, 179, 42] };
pub const GUID_ACPI_INTERFACE_STANDARD2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3899219811, data2: 6193, data3: 18544, data4: [168, 207, 156, 47, 3, 249, 220, 181] };
pub const GUID_ACPI_PORT_RANGES_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4048511131, data2: 52157, data3: 18775, data4: [166, 116, 188, 0, 33, 63, 28, 151] };
pub const GUID_ACPI_REGS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 101980518, data2: 29253, data3: 25449, data4: [70, 46, 78, 101, 108, 115, 111, 110] };
pub const GUID_AGP_TARGET_BUS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2975661288,
    data2: 1745,
    data3: 19767,
    data4: [157, 76, 190, 221, 224, 194, 166, 255],
};
pub const GUID_ARBITER_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3863277957, data2: 35854, data3: 4560, data4: [190, 207, 8, 0, 43, 226, 9, 47] };
pub const GUID_BUS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1231782528, data2: 28453, data3: 4560, data4: [190, 175, 8, 0, 43, 226, 9, 47] };
pub const GUID_BUS_RESOURCE_UPDATE_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 667947053,
    data2: 49074,
    data3: 16740,
    data4: [129, 221, 219, 184, 47, 150, 139, 72],
};
pub const GUID_BUS_TYPE_1394: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4149113835,
    data2: 39621,
    data3: 17899,
    data4: [190, 77, 119, 44, 199, 29, 223, 179],
};
pub const GUID_BUS_TYPE_ACPI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3618924693, data2: 26, data3: 18754, data4: [137, 31, 167, 212, 102, 16, 168, 67] };
pub const GUID_BUS_TYPE_AVC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3228562021,
    data2: 44553,
    data3: 18672,
    data4: [129, 44, 22, 117, 61, 124, 186, 131],
};
pub const GUID_BUS_TYPE_DOT4PRT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1142874113, data2: 17218, data3: 4565, data4: [161, 132, 0, 192, 79, 96, 82, 77] };
pub const GUID_BUS_TYPE_EISA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3720566025, data2: 62460, data3: 4560, data4: [165, 55, 0, 0, 248, 117, 62, 209] };
pub const GUID_BUS_TYPE_HID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4004460496, data2: 6499, data3: 18372, data4: [170, 72, 114, 71, 109, 183, 207, 73] };
pub const GUID_BUS_TYPE_INTERNAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 355527283, data2: 2155, data3: 4561, data4: [160, 159, 0, 192, 79, 195, 64, 177] };
pub const GUID_BUS_TYPE_IRDA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2061598145, data2: 51524, data3: 17622, data4: [136, 31, 76, 46, 97, 5, 59, 193] };
pub const GUID_BUS_TYPE_ISAPNP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3866556500, data2: 55421, data3: 4560, data4: [146, 178, 0, 160, 201, 5, 95, 197] };
pub const GUID_BUS_TYPE_LPTENUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3301576704, data2: 11740, data3: 4565, data4: [161, 122, 0, 192, 79, 96, 82, 77] };
pub const GUID_BUS_TYPE_MCA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 477469050, data2: 56371, data3: 4560, data4: [146, 178, 0, 160, 201, 5, 95, 197] };
pub const GUID_BUS_TYPE_PCI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3370901424, data2: 46352, data3: 4560, data4: [128, 229, 0, 160, 201, 37, 66, 227] };
pub const GUID_BUS_TYPE_PCMCIA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 154416688, data2: 44959, data3: 4560, data4: [146, 233, 0, 0, 248, 30, 27, 48] };
pub const GUID_BUS_TYPE_SCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 928667922, data2: 32844, data3: 17834, data4: [189, 194, 253, 210, 90, 29, 149, 18] };
pub const GUID_BUS_TYPE_SD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3875589124,
    data2: 16438,
    data3: 20105,
    data4: [149, 121, 137, 235, 244, 95, 0, 205],
};
pub const GUID_BUS_TYPE_SERENUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1997621895, data2: 35140, data3: 4561, data4: [189, 144, 0, 160, 201, 6, 190, 45] };
pub const GUID_BUS_TYPE_SW_DEVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 114361122, data2: 32224, data3: 19695, data4: [142, 37, 25, 125, 14, 116, 66, 226] };
pub const GUID_BUS_TYPE_USB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2642275260, data2: 51293, data3: 4561, data4: [158, 180, 0, 96, 8, 195, 161, 154] };
pub const GUID_BUS_TYPE_USBPRINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1142874112, data2: 17218, data3: 4565, data4: [161, 132, 0, 192, 79, 96, 82, 77] };
pub const GUID_D3COLD_AUX_POWER_AND_TIMING_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4511914, data2: 63076, data3: 17800, data4: [159, 252, 42, 254, 175, 89, 80, 185] };
pub const GUID_D3COLD_SUPPORT_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3011678437,
    data2: 15568,
    data3: 20381,
    data4: [153, 55, 245, 254, 43, 68, 212, 122],
};
pub const GUID_DEVCLASS_1394: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653697, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_1394DEBUG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1727156438, data2: 30721, data3: 19044, data4: [177, 57, 238, 168, 10, 69, 11, 36] };
pub const GUID_DEVCLASS_61883: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2126445504, data2: 12800, data3: 4562, data4: [180, 194, 0, 160, 201, 105, 125, 7] };
pub const GUID_DEVCLASS_ADAPTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444324, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_APMSUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3562740760, data2: 51450, data3: 4561, data4: [159, 119, 0, 0, 248, 5, 245, 48] };
pub const GUID_DEVCLASS_AVC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3228562021,
    data2: 44553,
    data3: 18672,
    data4: [129, 44, 22, 117, 61, 124, 186, 131],
};
pub const GUID_DEVCLASS_BATTERY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1919098452, data2: 30884, data3: 4560, data4: [188, 247, 0, 170, 0, 183, 179, 42] };
pub const GUID_DEVCLASS_BIOMETRIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1406312183,
    data2: 14204,
    data3: 19732,
    data4: [134, 75, 235, 58, 133, 118, 147, 89],
};
pub const GUID_DEVCLASS_BLUETOOTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3771461740,
    data2: 52619,
    data3: 17991,
    data4: [187, 138, 38, 59, 67, 240, 249, 116],
};
pub const GUID_DEVCLASS_CAMERA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3393092281, data2: 46275, data3: 19174, data4: [130, 81, 87, 158, 249, 51, 137, 15] };
pub const GUID_DEVCLASS_CDROM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444325, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_COMPUTEACCELERATOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4028276051, data2: 16374, data3: 18642, data4: [159, 151, 200, 167, 0, 75, 225, 12] };
pub const GUID_DEVCLASS_COMPUTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444326, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653698, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_DISKDRIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444327, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_DISPLAY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444328, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_DOT4: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1215437654, data2: 26517, data3: 4562, data4: [177, 168, 0, 128, 199, 46, 116, 162] };
pub const GUID_DEVCLASS_DOT4PRINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1238264520, data2: 28550, data3: 4562, data4: [177, 229, 0, 128, 199, 46, 116, 162] };
pub const GUID_DEVCLASS_EHSTORAGESILO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2644686863, data2: 63647, data3: 19017, data4: [165, 194, 81, 27, 8, 91, 158, 138] };
pub const GUID_DEVCLASS_ENUM1394: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3294224213, data2: 56072, data3: 4561, data4: [176, 9, 0, 160, 201, 8, 31, 246] };
pub const GUID_DEVCLASS_EXTENSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3807923431,
    data2: 36602,
    data3: 16668,
    data4: [170, 105, 151, 69, 76, 164, 203, 87],
};
pub const GUID_DEVCLASS_FDC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444329, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_FIRMWARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4075281778,
    data2: 25704,
    data3: 20022,
    data4: [182, 241, 100, 136, 244, 44, 27, 82],
};
pub const GUID_DEVCLASS_FLOPPYDISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444352, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_FSFILTER_ACTIVITYMONITOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3094216529,
    data2: 41758,
    data3: 19372,
    data4: [179, 207, 232, 207, 231, 92, 159, 194],
};
pub const GUID_DEVCLASS_FSFILTER_ANTIVIRUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2983305577,
    data2: 50511,
    data3: 17273,
    data4: [129, 219, 190, 231, 216, 141, 116, 84],
};
pub const GUID_DEVCLASS_FSFILTER_BOTTOM: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 930504352,
    data2: 22872,
    data3: 20425,
    data4: [176, 75, 47, 223, 239, 151, 229, 158],
};
pub const GUID_DEVCLASS_FSFILTER_CFSMETADATASERVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3452897593,
    data2: 46939,
    data3: 17968,
    data4: [191, 118, 128, 247, 186, 101, 88, 132],
};
pub const GUID_DEVCLASS_FSFILTER_COMPRESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4082658223, data2: 46506, data3: 18869, data4: [141, 108, 5, 105, 40, 76, 99, 159] };
pub const GUID_DEVCLASS_FSFILTER_CONTENTSCREENER: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1044317812,
    data2: 51260,
    data3: 17752,
    data4: [187, 38, 152, 32, 225, 235, 165, 197],
};
pub const GUID_DEVCLASS_FSFILTER_CONTINUOUSBACKUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1906971896,
    data2: 28589,
    data3: 17954,
    data4: [173, 119, 146, 187, 157, 126, 105, 71],
};
pub const GUID_DEVCLASS_FSFILTER_COPYPROTECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2306371569,
    data2: 39954,
    data3: 16431,
    data4: [156, 158, 23, 117, 60, 127, 67, 117],
};
pub const GUID_DEVCLASS_FSFILTER_ENCRYPTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2695299520, data2: 42257, data3: 17151, data4: [170, 108, 6, 220, 3, 149, 87, 111] };
pub const GUID_DEVCLASS_FSFILTER_HSM: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3578155018,
    data2: 10987,
    data3: 17910,
    data4: [148, 130, 244, 177, 121, 156, 49, 119],
};
pub const GUID_DEVCLASS_FSFILTER_INFRASTRUCTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3848251129, data2: 4748, data3: 19716, data4: [171, 171, 99, 12, 116, 177, 69, 58] };
pub const GUID_DEVCLASS_FSFILTER_OPENFILEBACKUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4176261030,
    data2: 26321,
    data3: 16805,
    data4: [137, 155, 102, 88, 93, 114, 22, 183],
};
pub const GUID_DEVCLASS_FSFILTER_PHYSICALQUOTAMANAGEMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1779076728, data2: 48038, data3: 20420, data4: [167, 9, 30, 51, 205, 9, 214, 126] };
pub const GUID_DEVCLASS_FSFILTER_QUOTAMANAGEMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2231617809,
    data2: 42695,
    data3: 18713,
    data4: [143, 121, 80, 40, 245, 134, 107, 12],
};
pub const GUID_DEVCLASS_FSFILTER_REPLICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1221848004,
    data2: 19704,
    data3: 18687,
    data4: [184, 105, 156, 104, 173, 66, 235, 159],
};
pub const GUID_DEVCLASS_FSFILTER_SECURITYENHANCER: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3492529114,
    data2: 3214,
    data3: 18757,
    data4: [155, 213, 241, 136, 60, 34, 108, 140],
};
pub const GUID_DEVCLASS_FSFILTER_SYSTEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1562090154, data2: 482, data3: 18095, data4: [132, 159, 39, 43, 63, 50, 76, 70] };
pub const GUID_DEVCLASS_FSFILTER_SYSTEMRECOVERY: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 766595956,
    data2: 28782,
    data3: 16689,
    data4: [160, 199, 215, 199, 142, 176, 40, 154],
};
pub const GUID_DEVCLASS_FSFILTER_TOP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3010050804,
    data2: 21864,
    data3: 20098,
    data4: [168, 126, 169, 62, 177, 107, 202, 135],
};
pub const GUID_DEVCLASS_FSFILTER_UNDELETE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4270790002,
    data2: 50810,
    data3: 18624,
    data4: [187, 172, 11, 92, 109, 102, 202, 251],
};
pub const GUID_DEVCLASS_FSFILTER_VIRTUALIZATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4149905088,
    data2: 4312,
    data3: 19514,
    data4: [178, 51, 237, 96, 228, 205, 250, 172],
};
pub const GUID_DEVCLASS_GPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653699, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_HDC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444330, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_HIDCLASS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1952061344, data2: 29907, data3: 4560, data4: [182, 254, 0, 160, 201, 15, 87, 218] };
pub const GUID_DEVCLASS_HOLOGRAPHIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3591525693,
    data2: 1713,
    data3: 18890,
    data4: [137, 56, 227, 158, 248, 14, 177, 111],
};
pub const GUID_DEVCLASS_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653702, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_INFINIBAND: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 820998450, data2: 55384, data3: 18956, data4: [172, 36, 185, 2, 138, 92, 202, 63] };
pub const GUID_DEVCLASS_INFRARED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653701, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_KEYBOARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444331, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_LEGACYDRIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2395735389, data2: 1151, data3: 4561, data4: [165, 55, 0, 0, 248, 117, 62, 209] };
pub const GUID_DEVCLASS_MEDIA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444332, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MEDIUM_CHANGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3461953966, data2: 60382, data3: 4560, data4: [177, 129, 0, 0, 248, 117, 62, 196] };
pub const GUID_DEVCLASS_MEMORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1352242250, data2: 63161, data3: 16471, data4: [160, 86, 140, 85, 2, 40, 84, 76] };
pub const GUID_DEVCLASS_MODEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444333, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MONITOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444334, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MOUSE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444335, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MTD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444336, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MULTIFUNCTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444337, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MULTIPORTSERIAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1351642296, data2: 47634, data3: 4561, data4: [191, 93, 0, 0, 248, 5, 245, 48] };
pub const GUID_DEVCLASS_NET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444338, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_NETCLIENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444339, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_NETDRIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2280626897,
    data2: 36720,
    data3: 18926,
    data4: [178, 21, 171, 31, 202, 220, 190, 60],
};
pub const GUID_DEVCLASS_NETSERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444340, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_NETTRANS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444341, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_NETUIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2022779841,
    data2: 52110,
    data3: 19240,
    data4: [163, 41, 243, 34, 235, 173, 190, 15],
};
pub const GUID_DEVCLASS_NODRIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444342, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PCMCIA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444343, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PNPPRINTERS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1180233342, data2: 61520, data3: 4561, data4: [182, 189, 0, 192, 79, 163, 114, 167] };
pub const GUID_DEVCLASS_PORTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444344, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PRINTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444345, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PRINTERUPGRADE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444346, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PRINTQUEUE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 517127161,
    data2: 4592,
    data3: 16516,
    data4: [178, 31, 173, 131, 168, 230, 220, 220],
};
pub const GUID_DEVCLASS_PROCESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1343389123,
    data2: 3894,
    data3: 16734,
    data4: [166, 204, 76, 179, 190, 145, 11, 101],
};
pub const GUID_DEVCLASS_SBP2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3565255102, data2: 60448, data3: 4561, data4: [182, 184, 0, 192, 79, 163, 114, 167] };
pub const GUID_DEVCLASS_SCMDISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1402367153, data2: 19782, data3: 16742, data4: [191, 35, 197, 34, 64, 60, 212, 149] };
pub const GUID_DEVCLASS_SCMVOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1405923657,
    data2: 58691,
    data3: 19588,
    data4: [182, 224, 188, 228, 246, 183, 232, 6],
};
pub const GUID_DEVCLASS_SCSIADAPTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444347, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_SECURITYACCELERATOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 646747553, data2: 60926, data3: 4563, data4: [149, 195, 0, 16, 220, 64, 80, 165] };
pub const GUID_DEVCLASS_SENSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1366676276,
    data2: 50033,
    data3: 18438,
    data4: [179, 186, 113, 253, 83, 201, 37, 141],
};
pub const GUID_DEVCLASS_SIDESHOW: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2574998925,
    data2: 50242,
    data3: 20270,
    data4: [186, 243, 156, 142, 103, 30, 158, 33],
};
pub const GUID_DEVCLASS_SMARTCARDREADER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1356681776, data2: 47754, data3: 4561, data4: [191, 93, 0, 0, 248, 5, 245, 48] };
pub const GUID_DEVCLASS_SMRDISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1397259299,
    data2: 26639,
    data3: 17797,
    data4: [172, 195, 31, 16, 214, 119, 126, 130],
};
pub const GUID_DEVCLASS_SMRVOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1404292867,
    data2: 36698,
    data3: 18312,
    data4: [145, 182, 209, 158, 217, 252, 204, 191],
};
pub const GUID_DEVCLASS_SOFTWARECOMPONENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1548497714,
    data2: 13389,
    data3: 18492,
    data4: [135, 57, 37, 158, 147, 76, 156, 200],
};
pub const GUID_DEVCLASS_SOUND: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444348, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_SYSTEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444349, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_TAPEDRIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1837136004, data2: 32033, data3: 4559, data4: [128, 28, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_UCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3874597404,
    data2: 32571,
    data3: 17523,
    data4: [178, 232, 201, 125, 138, 199, 29, 83],
};
pub const GUID_DEVCLASS_UNKNOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444350, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_USB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 922525280, data2: 50277, data3: 4559, data4: [128, 86, 68, 69, 83, 84, 0, 0] };
pub const GUID_DEVCLASS_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1906474205, data2: 33066, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_VOLUMESNAPSHOT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1396464516, data2: 60528, data3: 4562, data4: [149, 5, 0, 192, 79, 121, 222, 175] };
pub const GUID_DEVCLASS_WCEUSBS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 635162193, data2: 27791, data3: 19058, data4: [138, 109, 181, 76, 43, 79, 200, 53] };
pub const GUID_DEVCLASS_WPD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4005932440,
    data2: 32896,
    data3: 16991,
    data4: [146, 42, 218, 191, 61, 227, 246, 154],
};
pub const GUID_DEVICE_INTERFACE_ARRIVAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592324, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_DEVICE_INTERFACE_REMOVAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592325, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_DEVICE_RESET_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1688198950, data2: 15296, data3: 18451, data4: [173, 36, 126, 12, 30, 218, 63, 163] };
pub const GUID_DMA_CACHE_COHERENCY_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3038836730,
    data2: 35418,
    data3: 20032,
    data4: [163, 246, 107, 225, 225, 98, 217, 53],
};
pub const GUID_HWPROFILE_CHANGE_CANCELLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592322, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_HWPROFILE_CHANGE_COMPLETE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592323, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_HWPROFILE_QUERY_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592321, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_INT_ROUTE_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1888754676, data2: 115, data3: 4561, data4: [160, 158, 0, 192, 79, 195, 64, 177] };
pub const GUID_IOMMU_BUS_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 520020146, data2: 53880, data3: 19172, data4: [189, 220, 27, 52, 221, 100, 128, 67] };
pub const GUID_KERNEL_SOFT_RESTART_CANCEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 836188135, data2: 35851, data3: 18058, data4: [149, 110, 159, 67, 62, 195, 88, 251] };
pub const GUID_KERNEL_SOFT_RESTART_FINALIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 552147645, data2: 13578, data3: 19791, data4: [133, 119, 153, 200, 21, 7, 71, 58] };
pub const GUID_KERNEL_SOFT_RESTART_PREPARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3728162287,
    data2: 43100,
    data3: 20342,
    data4: [140, 191, 249, 107, 234, 139, 209, 15],
};
pub const GUID_LEGACY_DEVICE_DETECTION_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1358868702, data2: 22890, data3: 4562, data4: [165, 184, 0, 0, 248, 26, 70, 25] };
pub const GUID_MF_ENUMERATION_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2931332592, data2: 21894, data3: 4561, data4: [141, 132, 0, 160, 201, 6, 178, 68] };
pub const GUID_MSIX_TABLE_CONFIG_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 443172363, data2: 6479, data3: 17757, data4: [179, 75, 184, 76, 91, 5, 113, 43] };
pub const GUID_NPEM_CONTROL_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1301632829, data2: 46964, data3: 18570, data4: [177, 32, 79, 40, 74, 158, 255, 81] };
pub const GUID_PARTITION_UNIT_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1379286875,
    data2: 55441,
    data3: 17051,
    data4: [129, 149, 174, 197, 254, 246, 133, 60],
};
pub const GUID_PCC_INTERFACE_INTERNAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2093900494, data2: 49545, data3: 18452, data4: [166, 167, 18, 17, 32, 137, 233, 56] };
pub const GUID_PCC_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1055439459, data2: 3929, data3: 18980, data4: [138, 69, 53, 128, 139, 221, 18, 73] };
pub const GUID_PCI_ATS_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 17465320, data2: 38645, data3: 18755, data4: [190, 223, 149, 230, 81, 185, 52, 18] };
pub const GUID_PCI_BUS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1231782529, data2: 28453, data3: 4560, data4: [190, 175, 8, 0, 43, 226, 9, 47] };
pub const GUID_PCI_BUS_INTERFACE_STANDARD2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3734301030,
    data2: 65023,
    data3: 19612,
    data4: [153, 152, 103, 71, 177, 80, 231, 76],
};
pub const GUID_PCI_DEVICE_PRESENT_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3518508070,
    data2: 48969,
    data3: 17903,
    data4: [178, 22, 113, 203, 215, 136, 155, 87],
};
pub const GUID_PCI_EXPRESS_LINK_QUIESCENT_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 342676508, data2: 56035, data3: 17463, data4: [138, 255, 42, 243, 240, 56, 9, 155] };
pub const GUID_PCI_EXPRESS_ROOT_PORT_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2208789322, data2: 33991, data3: 16737, data4: [154, 152, 96, 0, 237, 12, 74, 51] };
pub const GUID_PCI_FPGA_CONTROL_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 770963368, data2: 47539, data3: 16483, data4: [146, 21, 181, 209, 74, 11, 38, 110] };
pub const GUID_PCI_PTM_CONTROL_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 881483451, data2: 47652, data3: 17591, data4: [153, 22, 40, 86, 135, 115, 81, 23] };
pub const GUID_PCI_SECURITY_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1853822033, data2: 6558, data3: 19148, data4: [186, 45, 118, 43, 78, 223, 70, 116] };
pub const GUID_PCI_VIRTUALIZATION_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1686731591,
    data2: 14922,
    data3: 19829,
    data4: [188, 116, 137, 221, 108, 7, 130, 147],
};
pub const GUID_PCMCIA_BUS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1981233904, data2: 50436, data3: 4561, data4: [148, 127, 0, 192, 79, 185, 96, 238] };
pub const GUID_PNP_CUSTOM_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2896641934, data2: 36131, data3: 4561, data4: [172, 125, 0, 0, 248, 117, 113, 208] };
pub const GUID_PNP_EXTENDED_ADDRESS_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3102315244,
    data2: 42903,
    data3: 19908,
    data4: [136, 70, 132, 208, 65, 112, 116, 70],
};
pub const GUID_PNP_LOCATION_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1881217806, data2: 2811, data3: 18395, data4: [175, 193, 65, 11, 248, 66, 73, 122] };
pub const GUID_PNP_POWER_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3268347488, data2: 60282, data3: 4561, data4: [189, 127, 0, 0, 248, 117, 113, 208] };
pub const GUID_PNP_POWER_SETTING_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 700881726,
    data2: 51098,
    data3: 17343,
    data4: [187, 222, 169, 50, 250, 27, 234, 126],
};
pub const GUID_POWER_DEVICE_ENABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2189167215, data2: 65200, data3: 4560, data4: [189, 38, 0, 170, 0, 183, 179, 42] };
pub const GUID_POWER_DEVICE_TIMEOUTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2757601077, data2: 65200, data3: 4560, data4: [189, 38, 0, 170, 0, 183, 179, 42] };
pub const GUID_POWER_DEVICE_WAKE_ENABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2840881794, data2: 65200, data3: 4560, data4: [189, 38, 0, 170, 0, 183, 179, 42] };
pub const GUID_PROCESSOR_PCC_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 934379162, data2: 49692, data3: 17046, data4: [151, 45, 17, 196, 179, 43, 40, 240] };
pub const GUID_QUERY_CRASHDUMP_FUNCTIONS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2630269183,
    data2: 13026,
    data3: 18484,
    data4: [177, 222, 179, 46, 248, 136, 10, 75],
};
pub const GUID_RECOVERY_NVMED_PREPARE_SHUTDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1268216042, data2: 48615, data3: 16395, data4: [169, 185, 79, 104, 79, 84, 204, 42] };
pub const GUID_RECOVERY_PCI_PREPARE_SHUTDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2430110174,
    data2: 34564,
    data3: 17615,
    data4: [129, 21, 237, 133, 40, 210, 178, 218],
};
pub const GUID_REENUMERATE_SELF_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 720044611, data2: 27246, data3: 18539, data4: [130, 252, 216, 21, 246, 185, 112, 6] };
pub const GUID_SCM_BUS_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 630474627, data2: 52857, data3: 16946, data4: [129, 94, 74, 48, 1, 78, 142, 180] };
pub const GUID_SCM_BUS_LD_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2609459325, data2: 55147, data3: 20296, data4: [177, 134, 84, 4, 26, 233, 46, 141] };
pub const GUID_SCM_BUS_NVD_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2380293375,
    data2: 46640,
    data3: 17124,
    data4: [136, 234, 111, 36, 200, 100, 17, 117],
};
pub const GUID_SCM_PHYSICAL_NVDIMM_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 7979547, data2: 37246, data3: 16478, data4: [169, 206, 7, 50, 181, 187, 206, 189] };
pub const GUID_SDEV_IDENTIFIER_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1238792952,
    data2: 37228,
    data3: 20200,
    data4: [157, 241, 136, 159, 23, 210, 30, 145],
};
pub const GUID_SECURE_DRIVER_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 923756513, data2: 20469, data3: 19092, data4: [154, 53, 6, 197, 217, 204, 48, 226] };
pub const GUID_TARGET_DEVICE_QUERY_REMOVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592326, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_TARGET_DEVICE_REMOVE_CANCELLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592327, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_TARGET_DEVICE_REMOVE_COMPLETE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592328, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_TARGET_DEVICE_TRANSPORT_RELATIONS_CHANGED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4243925238, data2: 43055, data3: 18353, data4: [173, 58, 128, 80, 89, 76, 173, 40] };
pub const GUID_THERMAL_COOLING_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3971893160,
    data2: 50328,
    data3: 19385,
    data4: [189, 112, 232, 103, 224, 148, 13, 34],
};
pub const GUID_TRANSLATOR_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1813334674, data2: 43727, data3: 4560, data4: [141, 42, 0, 160, 201, 6, 178, 68] };
pub const GUID_WUDF_DEVICE_HOST_PROBLEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3292341693,
    data2: 37702,
    data3: 16622,
    data4: [162, 210, 215, 12, 21, 248, 183, 91],
};
pub type HCMNOTIFICATION = isize;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct HWProfileInfo_sA {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [super::super::Foundation::CHAR; 80],
    pub HWPI_dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HWProfileInfo_sA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HWProfileInfo_sA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct HWProfileInfo_sW {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [u16; 80],
    pub HWPI_dwFlags: u32,
}
impl ::core::marker::Copy for HWProfileInfo_sW {}
impl ::core::clone::Clone for HWProfileInfo_sW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IDD_DYNAWIZ_ANALYZEDEV_PAGE: u32 = 10010u32;
pub const IDD_DYNAWIZ_ANALYZE_NEXTPAGE: u32 = 10004u32;
pub const IDD_DYNAWIZ_ANALYZE_PREVPAGE: u32 = 10003u32;
pub const IDD_DYNAWIZ_FIRSTPAGE: u32 = 10000u32;
pub const IDD_DYNAWIZ_INSTALLDETECTEDDEVS_PAGE: u32 = 10011u32;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NEXTPAGE: u32 = 10007u32;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NODEVS: u32 = 10008u32;
pub const IDD_DYNAWIZ_INSTALLDETECTED_PREVPAGE: u32 = 10006u32;
pub const IDD_DYNAWIZ_SELECTCLASS_PAGE: u32 = 10012u32;
pub const IDD_DYNAWIZ_SELECTDEV_PAGE: u32 = 10009u32;
pub const IDD_DYNAWIZ_SELECT_NEXTPAGE: u32 = 10002u32;
pub const IDD_DYNAWIZ_SELECT_PREVPAGE: u32 = 10001u32;
pub const IDF_CHECKFIRST: u32 = 256u32;
pub const IDF_NOBEEP: u32 = 512u32;
pub const IDF_NOBROWSE: u32 = 1u32;
pub const IDF_NOCOMPRESSED: u32 = 8u32;
pub const IDF_NODETAILS: u32 = 4u32;
pub const IDF_NOFOREGROUND: u32 = 1024u32;
pub const IDF_NOREMOVABLEMEDIAPROMPT: u32 = 4096u32;
pub const IDF_NOSKIP: u32 = 2u32;
pub const IDF_OEMDISK: u32 = 2147483648u32;
pub const IDF_USEDISKNAMEASPROMPT: u32 = 8192u32;
pub const IDF_WARNIFSKIP: u32 = 2048u32;
pub const IDI_CLASSICON_OVERLAYFIRST: u32 = 500u32;
pub const IDI_CLASSICON_OVERLAYLAST: u32 = 502u32;
pub const IDI_CONFLICT: u32 = 161u32;
pub const IDI_DISABLED_OVL: u32 = 501u32;
pub const IDI_FORCED_OVL: u32 = 502u32;
pub const IDI_PROBLEM_OVL: u32 = 500u32;
pub const IDI_RESOURCE: u32 = 159u32;
pub const IDI_RESOURCEFIRST: u32 = 159u32;
pub const IDI_RESOURCELAST: u32 = 161u32;
pub const IDI_RESOURCEOVERLAYFIRST: u32 = 161u32;
pub const IDI_RESOURCEOVERLAYLAST: u32 = 161u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct INFCONTEXT {
    pub Inf: *mut ::core::ffi::c_void,
    pub CurrentInf: *mut ::core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for INFCONTEXT {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for INFCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct INFCONTEXT {
    pub Inf: *mut ::core::ffi::c_void,
    pub CurrentInf: *mut ::core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for INFCONTEXT {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for INFCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const INFINFO_DEFAULT_SEARCH: u32 = 3u32;
pub const INFINFO_INF_NAME_IS_ABSOLUTE: u32 = 2u32;
pub const INFINFO_INF_PATH_LIST_SEARCH: u32 = 5u32;
pub const INFINFO_INF_SPEC_IS_HINF: u32 = 1u32;
pub const INFINFO_REVERSE_DEFAULT_SEARCH: u32 = 4u32;
pub const INF_STYLE_CACHE_DISABLE: u32 = 32u32;
pub const INF_STYLE_CACHE_ENABLE: u32 = 16u32;
pub const INF_STYLE_CACHE_IGNORE: u32 = 64u32;
pub const INSTALLFLAG_BITS: u32 = 7u32;
pub const INSTALLFLAG_FORCE: u32 = 1u32;
pub const INSTALLFLAG_NONINTERACTIVE: u32 = 4u32;
pub const INSTALLFLAG_READONLY: u32 = 2u32;
pub const IOA_Local: u32 = 255u32;
pub const IO_ALIAS_10_BIT_DECODE: u32 = 4u32;
pub const IO_ALIAS_12_BIT_DECODE: u32 = 16u32;
pub const IO_ALIAS_16_BIT_DECODE: u32 = 0u32;
pub const IO_ALIAS_POSITIVE_DECODE: u32 = 255u32;
#[repr(C, packed(1))]
pub struct IO_DES {
    pub IOD_Count: u32,
    pub IOD_Type: u32,
    pub IOD_Alloc_Base: u64,
    pub IOD_Alloc_End: u64,
    pub IOD_DesFlags: u32,
}
impl ::core::marker::Copy for IO_DES {}
impl ::core::clone::Clone for IO_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct IO_RANGE {
    pub IOR_Align: u64,
    pub IOR_nPorts: u32,
    pub IOR_Min: u64,
    pub IOR_Max: u64,
    pub IOR_RangeFlags: u32,
    pub IOR_Alias: u64,
}
impl ::core::marker::Copy for IO_RANGE {}
impl ::core::clone::Clone for IO_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IO_RESOURCE {
    pub IO_Header: IO_DES,
    pub IO_Data: [IO_RANGE; 1],
}
impl ::core::marker::Copy for IO_RESOURCE {}
impl ::core::clone::Clone for IO_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct IRQ_DES_32 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: u32,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u32,
}
impl ::core::marker::Copy for IRQ_DES_32 {}
impl ::core::clone::Clone for IRQ_DES_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct IRQ_DES_64 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: u32,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u64,
}
impl ::core::marker::Copy for IRQ_DES_64 {}
impl ::core::clone::Clone for IRQ_DES_64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct IRQ_RANGE {
    pub IRQR_Min: u32,
    pub IRQR_Max: u32,
    pub IRQR_Flags: u32,
}
impl ::core::marker::Copy for IRQ_RANGE {}
impl ::core::clone::Clone for IRQ_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IRQ_RESOURCE_32 {
    pub IRQ_Header: IRQ_DES_32,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl ::core::marker::Copy for IRQ_RESOURCE_32 {}
impl ::core::clone::Clone for IRQ_RESOURCE_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IRQ_RESOURCE_64 {
    pub IRQ_Header: IRQ_DES_64,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl ::core::marker::Copy for IRQ_RESOURCE_64 {}
impl ::core::clone::Clone for IRQ_RESOURCE_64 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LCPRI_BOOTCONFIG: u32 = 1u32;
pub const LCPRI_DESIRED: u32 = 8192u32;
pub const LCPRI_DISABLED: u32 = 65535u32;
pub const LCPRI_FORCECONFIG: u32 = 0u32;
pub const LCPRI_HARDRECONFIG: u32 = 49152u32;
pub const LCPRI_HARDWIRED: u32 = 57344u32;
pub const LCPRI_IMPOSSIBLE: u32 = 61440u32;
pub const LCPRI_LASTBESTCONFIG: u32 = 16383u32;
pub const LCPRI_LASTSOFTCONFIG: u32 = 32767u32;
pub const LCPRI_NORMAL: u32 = 12288u32;
pub const LCPRI_POWEROFF: u32 = 40960u32;
pub const LCPRI_REBOOT: u32 = 36864u32;
pub const LCPRI_RESTART: u32 = 32768u32;
pub const LCPRI_SUBOPTIMAL: u32 = 20480u32;
pub const LINE_LEN: u32 = 256u32;
pub const LOG_CONF_BITS: u32 = 7u32;
pub const LogSevError: u32 = 2u32;
pub const LogSevFatalError: u32 = 3u32;
pub const LogSevInformation: u32 = 0u32;
pub const LogSevMaximum: u32 = 4u32;
pub const LogSevWarning: u32 = 1u32;
pub const MAX_CLASS_NAME_LEN: u32 = 32u32;
pub const MAX_CONFIG_VALUE: u32 = 9999u32;
pub const MAX_DEVICE_ID_LEN: u32 = 200u32;
pub const MAX_DEVNODE_ID_LEN: u32 = 200u32;
pub const MAX_DMA_CHANNELS: u32 = 7u32;
pub const MAX_GUID_STRING_LEN: u32 = 39u32;
pub const MAX_IDD_DYNAWIZ_RESOURCE_ID: u32 = 11000u32;
pub const MAX_INFSTR_STRKEY_LEN: u32 = 32u32;
pub const MAX_INF_FLAG: u32 = 20u32;
pub const MAX_INF_SECTION_NAME_LENGTH: u32 = 255u32;
pub const MAX_INF_STRING_LENGTH: u32 = 4096u32;
pub const MAX_INSTALLWIZARD_DYNAPAGES: u32 = 20u32;
pub const MAX_INSTANCE_VALUE: u32 = 9999u32;
pub const MAX_INSTRUCTION_LEN: u32 = 256u32;
pub const MAX_IO_PORTS: u32 = 20u32;
pub const MAX_IRQS: u32 = 7u32;
pub const MAX_KEY_LEN: u32 = 100u32;
pub const MAX_LABEL_LEN: u32 = 30u32;
pub const MAX_LCPRI: u32 = 65535u32;
pub const MAX_MEM_REGISTERS: u32 = 9u32;
pub const MAX_PRIORITYSTR_LEN: u32 = 16u32;
pub const MAX_PROFILE_LEN: u32 = 80u32;
pub const MAX_SERVICE_NAME_LEN: u32 = 256u32;
pub const MAX_SUBTITLE_LEN: u32 = 256u32;
pub const MAX_TITLE_LEN: u32 = 60u32;
#[repr(C, packed(1))]
pub struct MEM_DES {
    pub MD_Count: u32,
    pub MD_Type: u32,
    pub MD_Alloc_Base: u64,
    pub MD_Alloc_End: u64,
    pub MD_Flags: u32,
    pub MD_Reserved: u32,
}
impl ::core::marker::Copy for MEM_DES {}
impl ::core::clone::Clone for MEM_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct MEM_RANGE {
    pub MR_Align: u64,
    pub MR_nBytes: u32,
    pub MR_Min: u64,
    pub MR_Max: u64,
    pub MR_Flags: u32,
    pub MR_Reserved: u32,
}
impl ::core::marker::Copy for MEM_RANGE {}
impl ::core::clone::Clone for MEM_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MEM_RESOURCE {
    pub MEM_Header: MEM_DES,
    pub MEM_Data: [MEM_RANGE; 1],
}
impl ::core::marker::Copy for MEM_RESOURCE {}
impl ::core::clone::Clone for MEM_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct MFCARD_DES {
    pub PMF_Count: u32,
    pub PMF_Type: u32,
    pub PMF_Flags: u32,
    pub PMF_ConfigOptions: u8,
    pub PMF_IoResourceIndex: u8,
    pub PMF_Reserved: [u8; 2],
    pub PMF_ConfigRegisterBase: u32,
}
impl ::core::marker::Copy for MFCARD_DES {}
impl ::core::clone::Clone for MFCARD_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MFCARD_RESOURCE {
    pub MfCard_Header: MFCARD_DES,
}
impl ::core::marker::Copy for MFCARD_RESOURCE {}
impl ::core::clone::Clone for MFCARD_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MIN_IDD_DYNAWIZ_RESOURCE_ID: u32 = 10000u32;
#[repr(C, packed(1))]
pub struct Mem_Large_Des_s {
    pub MLD_Count: u32,
    pub MLD_Type: u32,
    pub MLD_Alloc_Base: u64,
    pub MLD_Alloc_End: u64,
    pub MLD_Flags: u32,
    pub MLD_Reserved: u32,
}
impl ::core::marker::Copy for Mem_Large_Des_s {}
impl ::core::clone::Clone for Mem_Large_Des_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct Mem_Large_Range_s {
    pub MLR_Align: u64,
    pub MLR_nBytes: u64,
    pub MLR_Min: u64,
    pub MLR_Max: u64,
    pub MLR_Flags: u32,
    pub MLR_Reserved: u32,
}
impl ::core::marker::Copy for Mem_Large_Range_s {}
impl ::core::clone::Clone for Mem_Large_Range_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct Mem_Large_Resource_s {
    pub MEM_LARGE_Header: Mem_Large_Des_s,
    pub MEM_LARGE_Data: [Mem_Large_Range_s; 1],
}
impl ::core::marker::Copy for Mem_Large_Resource_s {}
impl ::core::clone::Clone for Mem_Large_Resource_s {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NDW_INSTALLFLAG_CI_PICKED_OEM: u32 = 32768u32;
pub const NDW_INSTALLFLAG_DIDFACTDEFS: u32 = 1u32;
pub const NDW_INSTALLFLAG_EXPRESSINTRO: u32 = 1024u32;
pub const NDW_INSTALLFLAG_HARDWAREALLREADYIN: u32 = 2u32;
pub const NDW_INSTALLFLAG_INSTALLSPECIFIC: u32 = 8192u32;
pub const NDW_INSTALLFLAG_KNOWNCLASS: u32 = 524288u32;
pub const NDW_INSTALLFLAG_NEEDREBOOT: i32 = 256i32;
pub const NDW_INSTALLFLAG_NEEDRESTART: i32 = 128i32;
pub const NDW_INSTALLFLAG_NEEDSHUTDOWN: u32 = 512u32;
pub const NDW_INSTALLFLAG_NODETECTEDDEVS: u32 = 4096u32;
pub const NDW_INSTALLFLAG_PCMCIADEVICE: u32 = 131072u32;
pub const NDW_INSTALLFLAG_PCMCIAMODE: u32 = 65536u32;
pub const NDW_INSTALLFLAG_SKIPCLASSLIST: u32 = 16384u32;
pub const NDW_INSTALLFLAG_SKIPISDEVINSTALLED: u32 = 2048u32;
pub const NDW_INSTALLFLAG_USERCANCEL: u32 = 262144u32;
pub const NUM_CM_PROB: u32 = 58u32;
pub const NUM_CM_PROB_V1: u32 = 37u32;
pub const NUM_CM_PROB_V2: u32 = 50u32;
pub const NUM_CM_PROB_V3: u32 = 51u32;
pub const NUM_CM_PROB_V4: u32 = 52u32;
pub const NUM_CM_PROB_V5: u32 = 53u32;
pub const NUM_CM_PROB_V6: u32 = 54u32;
pub const NUM_CM_PROB_V7: u32 = 55u32;
pub const NUM_CM_PROB_V8: u32 = 57u32;
pub const NUM_CM_PROB_V9: u32 = 58u32;
pub const NUM_LOG_CONF: u32 = 6u32;
pub type OEM_SOURCE_MEDIA_TYPE = u32;
pub const SPOST_NONE: OEM_SOURCE_MEDIA_TYPE = 0u32;
pub const SPOST_PATH: OEM_SOURCE_MEDIA_TYPE = 1u32;
pub const SPOST_URL: OEM_SOURCE_MEDIA_TYPE = 2u32;
pub const OVERRIDE_LOG_CONF: u32 = 5u32;
#[repr(C, packed(1))]
pub struct PCCARD_DES {
    pub PCD_Count: u32,
    pub PCD_Type: u32,
    pub PCD_Flags: u32,
    pub PCD_ConfigIndex: u8,
    pub PCD_Reserved: [u8; 3],
    pub PCD_MemoryCardBase1: u32,
    pub PCD_MemoryCardBase2: u32,
    pub PCD_MemoryCardBase: [u32; 2],
    pub PCD_MemoryFlags: [u16; 2],
    pub PCD_IoFlags: [u8; 2],
}
impl ::core::marker::Copy for PCCARD_DES {}
impl ::core::clone::Clone for PCCARD_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PCCARD_RESOURCE {
    pub PcCard_Header: PCCARD_DES,
}
impl ::core::marker::Copy for PCCARD_RESOURCE {}
impl ::core::clone::Clone for PCCARD_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PCD_MAX_IO: u32 = 2u32;
pub const PCD_MAX_MEMORY: u32 = 2u32;
pub type PCM_NOTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hnotify: HCMNOTIFICATION, context: *const ::core::ffi::c_void, action: CM_NOTIFY_ACTION, eventdata: *const CM_NOTIFY_EVENT_DATA, eventdatasize: u32) -> u32>;
#[cfg(feature = "Win32_Foundation")]
pub type PDETECT_PROGRESS_NOTIFY = ::core::option::Option<unsafe extern "system" fn(progressnotifyparam: *const ::core::ffi::c_void, detectcomplete: u32) -> super::super::Foundation::BOOL>;
pub type PNP_VETO_TYPE = i32;
pub const PNP_VetoTypeUnknown: PNP_VETO_TYPE = 0i32;
pub const PNP_VetoLegacyDevice: PNP_VETO_TYPE = 1i32;
pub const PNP_VetoPendingClose: PNP_VETO_TYPE = 2i32;
pub const PNP_VetoWindowsApp: PNP_VETO_TYPE = 3i32;
pub const PNP_VetoWindowsService: PNP_VETO_TYPE = 4i32;
pub const PNP_VetoOutstandingOpen: PNP_VETO_TYPE = 5i32;
pub const PNP_VetoDevice: PNP_VETO_TYPE = 6i32;
pub const PNP_VetoDriver: PNP_VETO_TYPE = 7i32;
pub const PNP_VetoIllegalDeviceRequest: PNP_VETO_TYPE = 8i32;
pub const PNP_VetoInsufficientPower: PNP_VETO_TYPE = 9i32;
pub const PNP_VetoNonDisableable: PNP_VETO_TYPE = 10i32;
pub const PNP_VetoLegacyDriver: PNP_VETO_TYPE = 11i32;
pub const PNP_VetoInsufficientRights: PNP_VETO_TYPE = 12i32;
pub const PNP_VetoAlreadyRemoved: PNP_VETO_TYPE = 13i32;
pub const PRIORITY_BIT: u32 = 8u32;
pub const PRIORITY_EQUAL_FIRST: u32 = 8u32;
pub const PRIORITY_EQUAL_LAST: u32 = 0u32;
pub type PSP_DETSIG_CMPPROC = ::core::option::Option<unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, newdevicedata: *const SP_DEVINFO_DATA, existingdevicedata: *const SP_DEVINFO_DATA, comparecontext: *const ::core::ffi::c_void) -> u32>;
pub type PSP_FILE_CALLBACK_A = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
pub type PSP_FILE_CALLBACK_W = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
pub const ROLLBACK_BITS: u32 = 1u32;
pub const ROLLBACK_FLAG_NO_UI: u32 = 1u32;
pub const RegDisposition_Bits: u32 = 1u32;
pub const RegDisposition_OpenAlways: u32 = 0u32;
pub const RegDisposition_OpenExisting: u32 = 1u32;
pub const ResType_All: u32 = 0u32;
pub const ResType_BusNumber: u32 = 6u32;
pub const ResType_ClassSpecific: u32 = 65535u32;
pub const ResType_Connection: u32 = 32772u32;
pub const ResType_DMA: u32 = 3u32;
pub const ResType_DevicePrivate: u32 = 32769u32;
pub const ResType_DoNotUse: u32 = 5u32;
pub const ResType_IO: u32 = 2u32;
pub const ResType_IRQ: u32 = 4u32;
pub const ResType_Ignored_Bit: u32 = 32768u32;
pub const ResType_MAX: u32 = 7u32;
pub const ResType_Mem: u32 = 1u32;
pub const ResType_MemLarge: u32 = 7u32;
pub const ResType_MfCardConfig: u32 = 32771u32;
pub const ResType_None: u32 = 0u32;
pub const ResType_PcCardConfig: u32 = 32770u32;
pub const ResType_Reserved: u32 = 32768u32;
pub const SCWMI_CLOBBER_SECURITY: u32 = 1u32;
pub const SETDIRID_NOT_FULL_PATH: u32 = 1u32;
pub type SETUP_DI_BUILD_DRIVER_DRIVER_TYPE = u32;
pub const SPDIT_CLASSDRIVER: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE = 1u32;
pub const SPDIT_COMPATDRIVER: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE = 2u32;
pub type SETUP_FILE_OPERATION = u32;
pub const FILEOP_DELETE: SETUP_FILE_OPERATION = 2u32;
pub const FILEOP_COPY: SETUP_FILE_OPERATION = 0u32;
pub const SIGNERSCORE_AUTHENTICODE: u32 = 251658240u32;
pub const SIGNERSCORE_INBOX: u32 = 218103811u32;
pub const SIGNERSCORE_LOGO_PREMIUM: u32 = 218103809u32;
pub const SIGNERSCORE_LOGO_STANDARD: u32 = 218103810u32;
pub const SIGNERSCORE_MASK: u32 = 4278190080u32;
pub const SIGNERSCORE_SIGNED_MASK: u32 = 4026531840u32;
pub const SIGNERSCORE_UNCLASSIFIED: u32 = 218103812u32;
pub const SIGNERSCORE_UNKNOWN: u32 = 4278190080u32;
pub const SIGNERSCORE_UNSIGNED: u32 = 2147483648u32;
pub const SIGNERSCORE_W9X_SUSPECT: u32 = 3221225472u32;
pub const SIGNERSCORE_WHQL: u32 = 218103813u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SOURCE_MEDIA_A {
    pub Reserved: super::super::Foundation::PSTR,
    pub Tagfile: super::super::Foundation::PSTR,
    pub Description: super::super::Foundation::PSTR,
    pub SourcePath: super::super::Foundation::PSTR,
    pub SourceFile: super::super::Foundation::PSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SOURCE_MEDIA_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SOURCE_MEDIA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SOURCE_MEDIA_A {
    pub Reserved: super::super::Foundation::PSTR,
    pub Tagfile: super::super::Foundation::PSTR,
    pub Description: super::super::Foundation::PSTR,
    pub SourcePath: super::super::Foundation::PSTR,
    pub SourceFile: super::super::Foundation::PSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SOURCE_MEDIA_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SOURCE_MEDIA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SOURCE_MEDIA_W {
    pub Reserved: super::super::Foundation::PWSTR,
    pub Tagfile: super::super::Foundation::PWSTR,
    pub Description: super::super::Foundation::PWSTR,
    pub SourcePath: super::super::Foundation::PWSTR,
    pub SourceFile: super::super::Foundation::PWSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SOURCE_MEDIA_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SOURCE_MEDIA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SOURCE_MEDIA_W {
    pub Reserved: super::super::Foundation::PWSTR,
    pub Tagfile: super::super::Foundation::PWSTR,
    pub Description: super::super::Foundation::PWSTR,
    pub SourcePath: super::super::Foundation::PWSTR,
    pub SourceFile: super::super::Foundation::PWSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SOURCE_MEDIA_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SOURCE_MEDIA_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPCRP_CHARACTERISTICS: u32 = 27u32;
pub const SPCRP_DEVTYPE: u32 = 25u32;
pub const SPCRP_EXCLUSIVE: u32 = 26u32;
pub const SPCRP_LOWERFILTERS: u32 = 18u32;
pub const SPCRP_MAXIMUM_PROPERTY: u32 = 28u32;
pub const SPCRP_SECURITY: u32 = 23u32;
pub const SPCRP_SECURITY_SDS: u32 = 24u32;
pub const SPCRP_UPPERFILTERS: u32 = 17u32;
pub const SPDIT_NODRIVER: u32 = 0u32;
pub const SPDRP_ADDRESS: u32 = 28u32;
pub const SPDRP_BASE_CONTAINERID: u32 = 36u32;
pub const SPDRP_BUSNUMBER: u32 = 21u32;
pub const SPDRP_BUSTYPEGUID: u32 = 19u32;
pub const SPDRP_CAPABILITIES: u32 = 15u32;
pub const SPDRP_CHARACTERISTICS: u32 = 27u32;
pub const SPDRP_CLASS: u32 = 7u32;
pub const SPDRP_CLASSGUID: u32 = 8u32;
pub const SPDRP_COMPATIBLEIDS: u32 = 2u32;
pub const SPDRP_CONFIGFLAGS: u32 = 10u32;
pub const SPDRP_DEVICEDESC: u32 = 0u32;
pub const SPDRP_DEVICE_POWER_DATA: u32 = 30u32;
pub const SPDRP_DEVTYPE: u32 = 25u32;
pub const SPDRP_DRIVER: u32 = 9u32;
pub const SPDRP_ENUMERATOR_NAME: u32 = 22u32;
pub const SPDRP_EXCLUSIVE: u32 = 26u32;
pub const SPDRP_FRIENDLYNAME: u32 = 12u32;
pub const SPDRP_HARDWAREID: u32 = 1u32;
pub const SPDRP_INSTALL_STATE: u32 = 34u32;
pub const SPDRP_LEGACYBUSTYPE: u32 = 20u32;
pub const SPDRP_LOCATION_INFORMATION: u32 = 13u32;
pub const SPDRP_LOCATION_PATHS: u32 = 35u32;
pub const SPDRP_LOWERFILTERS: u32 = 18u32;
pub const SPDRP_MAXIMUM_PROPERTY: u32 = 37u32;
pub const SPDRP_MFG: u32 = 11u32;
pub const SPDRP_PHYSICAL_DEVICE_OBJECT_NAME: u32 = 14u32;
pub const SPDRP_REMOVAL_POLICY: u32 = 31u32;
pub const SPDRP_REMOVAL_POLICY_HW_DEFAULT: u32 = 32u32;
pub const SPDRP_REMOVAL_POLICY_OVERRIDE: u32 = 33u32;
pub const SPDRP_SECURITY: u32 = 23u32;
pub const SPDRP_SECURITY_SDS: u32 = 24u32;
pub const SPDRP_SERVICE: u32 = 4u32;
pub const SPDRP_UI_NUMBER: u32 = 16u32;
pub const SPDRP_UI_NUMBER_DESC_FORMAT: u32 = 29u32;
pub const SPDRP_UNUSED0: u32 = 3u32;
pub const SPDRP_UNUSED1: u32 = 5u32;
pub const SPDRP_UNUSED2: u32 = 6u32;
pub const SPDRP_UPPERFILTERS: u32 = 17u32;
pub const SPDSL_DISALLOW_NEGATIVE_ADJUST: u32 = 2u32;
pub const SPDSL_IGNORE_DISK: u32 = 1u32;
pub const SPFILELOG_FORCENEW: u32 = 2u32;
pub const SPFILELOG_OEMFILE: u32 = 1u32;
pub const SPFILELOG_QUERYONLY: u32 = 4u32;
pub const SPFILELOG_SYSTEMLOG: u32 = 1u32;
pub const SPFILENOTIFY_BACKUPERROR: u32 = 22u32;
pub const SPFILENOTIFY_CABINETINFO: u32 = 16u32;
pub const SPFILENOTIFY_COPYERROR: u32 = 13u32;
pub const SPFILENOTIFY_DELETEERROR: u32 = 7u32;
pub const SPFILENOTIFY_ENDBACKUP: u32 = 23u32;
pub const SPFILENOTIFY_ENDCOPY: u32 = 12u32;
pub const SPFILENOTIFY_ENDDELETE: u32 = 6u32;
pub const SPFILENOTIFY_ENDQUEUE: u32 = 2u32;
pub const SPFILENOTIFY_ENDREGISTRATION: u32 = 32u32;
pub const SPFILENOTIFY_ENDRENAME: u32 = 9u32;
pub const SPFILENOTIFY_ENDSUBQUEUE: u32 = 4u32;
pub const SPFILENOTIFY_FILEEXTRACTED: u32 = 19u32;
pub const SPFILENOTIFY_FILEINCABINET: u32 = 17u32;
pub const SPFILENOTIFY_FILEOPDELAYED: u32 = 20u32;
pub const SPFILENOTIFY_LANGMISMATCH: u32 = 65536u32;
pub const SPFILENOTIFY_NEEDMEDIA: u32 = 14u32;
pub const SPFILENOTIFY_NEEDNEWCABINET: u32 = 18u32;
pub const SPFILENOTIFY_QUEUESCAN: u32 = 15u32;
pub const SPFILENOTIFY_QUEUESCAN_EX: u32 = 24u32;
pub const SPFILENOTIFY_QUEUESCAN_SIGNERINFO: u32 = 64u32;
pub const SPFILENOTIFY_RENAMEERROR: u32 = 10u32;
pub const SPFILENOTIFY_STARTBACKUP: u32 = 21u32;
pub const SPFILENOTIFY_STARTCOPY: u32 = 11u32;
pub const SPFILENOTIFY_STARTDELETE: u32 = 5u32;
pub const SPFILENOTIFY_STARTQUEUE: u32 = 1u32;
pub const SPFILENOTIFY_STARTREGISTRATION: u32 = 25u32;
pub const SPFILENOTIFY_STARTRENAME: u32 = 8u32;
pub const SPFILENOTIFY_STARTSUBQUEUE: u32 = 3u32;
pub const SPFILENOTIFY_TARGETEXISTS: u32 = 131072u32;
pub const SPFILENOTIFY_TARGETNEWER: u32 = 262144u32;
pub const SPFILEQ_FILE_IN_USE: u32 = 1u32;
pub const SPFILEQ_REBOOT_IN_PROGRESS: u32 = 4u32;
pub const SPFILEQ_REBOOT_RECOMMENDED: u32 = 2u32;
pub const SPID_ACTIVE: u32 = 1u32;
pub const SPID_DEFAULT: u32 = 2u32;
pub const SPID_REMOVED: u32 = 4u32;
pub const SPINST_ALL: u32 = 2047u32;
pub const SPINST_BITREG: u32 = 32u32;
pub const SPINST_COPYINF: u32 = 512u32;
pub const SPINST_DEVICEINSTALL: u32 = 1048576u32;
pub const SPINST_FILES: u32 = 16u32;
pub const SPINST_INI2REG: u32 = 8u32;
pub const SPINST_INIFILES: u32 = 2u32;
pub const SPINST_LOGCONFIG: u32 = 1u32;
pub const SPINST_LOGCONFIGS_ARE_OVERRIDES: u32 = 262144u32;
pub const SPINST_LOGCONFIG_IS_FORCED: u32 = 131072u32;
pub const SPINST_PROFILEITEMS: u32 = 256u32;
pub const SPINST_PROPERTIES: u32 = 1024u32;
pub const SPINST_REGISTERCALLBACKAWARE: u32 = 524288u32;
pub const SPINST_REGISTRY: u32 = 4u32;
pub const SPINST_REGSVR: u32 = 64u32;
pub const SPINST_SINGLESECTION: u32 = 65536u32;
pub const SPINST_UNREGSVR: u32 = 128u32;
pub const SPINT_ACTIVE: u32 = 1u32;
pub const SPINT_DEFAULT: u32 = 2u32;
pub const SPINT_REMOVED: u32 = 4u32;
pub const SPOST_MAX: u32 = 3u32;
pub const SPPSR_ENUM_ADV_DEVICE_PROPERTIES: u32 = 3u32;
pub const SPPSR_ENUM_BASIC_DEVICE_PROPERTIES: u32 = 2u32;
pub const SPPSR_SELECT_DEVICE_RESOURCES: u32 = 1u32;
pub const SPQ_DELAYED_COPY: u32 = 1u32;
pub const SPQ_FLAG_ABORT_IF_UNSIGNED: u32 = 2u32;
pub const SPQ_FLAG_BACKUP_AWARE: u32 = 1u32;
pub const SPQ_FLAG_DO_SHUFFLEMOVE: u32 = 8u32;
pub const SPQ_FLAG_FILES_MODIFIED: u32 = 4u32;
pub const SPQ_FLAG_VALID: u32 = 15u32;
pub const SPQ_SCAN_ACTIVATE_DRP: u32 = 1024u32;
pub const SPQ_SCAN_FILE_COMPARISON: u32 = 512u32;
pub const SPQ_SCAN_FILE_PRESENCE: u32 = 1u32;
pub const SPQ_SCAN_FILE_PRESENCE_WITHOUT_SOURCE: u32 = 256u32;
pub const SPQ_SCAN_FILE_VALIDITY: u32 = 2u32;
pub const SPQ_SCAN_INFORM_USER: u32 = 16u32;
pub const SPQ_SCAN_PRUNE_COPY_QUEUE: u32 = 32u32;
pub const SPQ_SCAN_PRUNE_DELREN: u32 = 128u32;
pub const SPQ_SCAN_USE_CALLBACK: u32 = 4u32;
pub const SPQ_SCAN_USE_CALLBACKEX: u32 = 8u32;
pub const SPQ_SCAN_USE_CALLBACK_SIGNERINFO: u32 = 64u32;
pub const SPRDI_FIND_DUPS: u32 = 1u32;
pub const SPREG_DLLINSTALL: u32 = 4u32;
pub const SPREG_GETPROCADDR: u32 = 2u32;
pub const SPREG_LOADLIBRARY: u32 = 1u32;
pub const SPREG_REGSVR: u32 = 3u32;
pub const SPREG_SUCCESS: u32 = 0u32;
pub const SPREG_TIMEOUT: u32 = 5u32;
pub const SPREG_UNKNOWN: u32 = 4294967295u32;
pub const SPSVCINST_ASSOCSERVICE: u32 = 2u32;
pub const SPSVCINST_CLOBBER_SECURITY: u32 = 1024u32;
pub const SPSVCINST_DELETEEVENTLOGENTRY: u32 = 4u32;
pub const SPSVCINST_NOCLOBBER_DELAYEDAUTOSTART: u32 = 32768u32;
pub const SPSVCINST_NOCLOBBER_DEPENDENCIES: u32 = 128u32;
pub const SPSVCINST_NOCLOBBER_DESCRIPTION: u32 = 256u32;
pub const SPSVCINST_NOCLOBBER_DISPLAYNAME: u32 = 8u32;
pub const SPSVCINST_NOCLOBBER_ERRORCONTROL: u32 = 32u32;
pub const SPSVCINST_NOCLOBBER_LOADORDERGROUP: u32 = 64u32;
pub const SPSVCINST_NOCLOBBER_REQUIREDPRIVILEGES: u32 = 4096u32;
pub const SPSVCINST_NOCLOBBER_SERVICESIDTYPE: u32 = 16384u32;
pub const SPSVCINST_NOCLOBBER_STARTTYPE: u32 = 16u32;
pub const SPSVCINST_NOCLOBBER_TRIGGERS: u32 = 8192u32;
pub const SPSVCINST_STARTSERVICE: u32 = 2048u32;
pub const SPSVCINST_STOPSERVICE: u32 = 512u32;
pub const SPSVCINST_TAGTOFRONT: u32 = 1u32;
pub const SPSVCINST_UNIQUE_NAME: u32 = 65536u32;
pub const SPWPT_SELECTDEVICE: u32 = 1u32;
pub const SPWP_USE_DEVINFO_DATA: u32 = 1u32;
pub const SP_ALTPLATFORM_FLAGS_SUITE_MASK: u32 = 2u32;
pub const SP_ALTPLATFORM_FLAGS_VERSION_RANGE: u32 = 1u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V1 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V1 {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2_0 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2 {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2_0 {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3_0 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3 {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3_0 {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SP_BACKUP_BACKUPPASS: u32 = 1u32;
pub const SP_BACKUP_BOOTFILE: u32 = 8u32;
pub const SP_BACKUP_DEMANDPASS: u32 = 2u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SP_BACKUP_SPECIAL: u32 = 4u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_UI_Controls")]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::super::UI::Controls::HIMAGELIST,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::marker::Copy for SP_CLASSIMAGELIST_DATA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::clone::Clone for SP_CLASSIMAGELIST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_UI_Controls")]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::super::UI::Controls::HIMAGELIST,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::marker::Copy for SP_CLASSIMAGELIST_DATA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::clone::Clone for SP_CLASSIMAGELIST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_CLASSINSTALL_HEADER {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_CLASSINSTALL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_CLASSINSTALL_HEADER {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_CLASSINSTALL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SP_COPY_STYLE = u32;
pub const SP_COPY_DELETESOURCE: SP_COPY_STYLE = 1u32;
pub const SP_COPY_REPLACEONLY: SP_COPY_STYLE = 2u32;
pub const SP_COPY_NEWER_OR_SAME: SP_COPY_STYLE = 4u32;
pub const SP_COPY_NEWER_ONLY: SP_COPY_STYLE = 65536u32;
pub const SP_COPY_NOOVERWRITE: SP_COPY_STYLE = 8u32;
pub const SP_COPY_NODECOMP: SP_COPY_STYLE = 16u32;
pub const SP_COPY_LANGUAGEAWARE: SP_COPY_STYLE = 32u32;
pub const SP_COPY_SOURCE_ABSOLUTE: SP_COPY_STYLE = 64u32;
pub const SP_COPY_SOURCEPATH_ABSOLUTE: SP_COPY_STYLE = 128u32;
pub const SP_COPY_FORCE_IN_USE: SP_COPY_STYLE = 512u32;
pub const SP_COPY_IN_USE_NEEDS_REBOOT: SP_COPY_STYLE = 256u32;
pub const SP_COPY_NOSKIP: SP_COPY_STYLE = 1024u32;
pub const SP_COPY_FORCE_NOOVERWRITE: SP_COPY_STYLE = 4096u32;
pub const SP_COPY_FORCE_NEWER: SP_COPY_STYLE = 8192u32;
pub const SP_COPY_WARNIFSKIP: SP_COPY_STYLE = 16384u32;
pub const SP_COPY_NOBROWSE: SP_COPY_STYLE = 32768u32;
pub const SP_COPY_NEWER: SP_COPY_STYLE = 4u32;
pub const SP_COPY_RESERVED: SP_COPY_STYLE = 131072u32;
pub const SP_COPY_OEMINF_CATALOG_ONLY: SP_COPY_STYLE = 262144u32;
pub const SP_COPY_REPLACE_BOOT_FILE: SP_COPY_STYLE = 524288u32;
pub const SP_COPY_NOPRUNE: SP_COPY_STYLE = 1048576u32;
pub const SP_COPY_OEM_F6_INF: SP_COPY_STYLE = 2097152u32;
pub const SP_COPY_ALREADYDECOMP: SP_COPY_STYLE = 4194304u32;
pub const SP_COPY_WINDOWS_SIGNED: SP_COPY_STYLE = 16777216u32;
pub const SP_COPY_PNPLOCKED: SP_COPY_STYLE = 33554432u32;
pub const SP_COPY_IN_USE_TRY_RENAME: SP_COPY_STYLE = 67108864u32;
pub const SP_COPY_INBOX_INF: SP_COPY_STYLE = 134217728u32;
pub const SP_COPY_HARDLINK: SP_COPY_STYLE = 268435456u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DETECTDEVICE_PARAMS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DETECTDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DETECTDEVICE_PARAMS {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DETECTDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: ::windows_sys::core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DATA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: ::windows_sys::core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DATA {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [super::super::Foundation::CHAR; 1],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [super::super::Foundation::CHAR; 1],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_DEVINFO_DATA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_DEVINFO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_DEVINFO_DATA {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_DEVINFO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [super::super::Foundation::CHAR; 263],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [super::super::Foundation::CHAR; 263],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [super::super::Foundation::CHAR; 256],
    pub InfFileName: [super::super::Foundation::CHAR; 260],
    pub DrvDescription: [super::super::Foundation::CHAR; 256],
    pub HardwareID: [super::super::Foundation::CHAR; 1],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [super::super::Foundation::CHAR; 256],
    pub InfFileName: [super::super::Foundation::CHAR; 260],
    pub DrvDescription: [super::super::Foundation::CHAR; 256],
    pub HardwareID: [super::super::Foundation::CHAR; 1],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: u32,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_DRVINSTALL_PARAMS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_DRVINSTALL_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: u32,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_DRVINSTALL_PARAMS {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_DRVINSTALL_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub EnableMessage: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_ENABLECLASS_PARAMS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_ENABLECLASS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub EnableMessage: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_ENABLECLASS_PARAMS {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_ENABLECLASS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: super::super::Foundation::PSTR,
    pub SourcePath: super::super::Foundation::PSTR,
    pub SourceFilename: super::super::Foundation::PSTR,
    pub SourceDescription: super::super::Foundation::PSTR,
    pub SourceTagfile: super::super::Foundation::PSTR,
    pub TargetDirectory: super::super::Foundation::PSTR,
    pub TargetFilename: super::super::Foundation::PSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: super::super::Foundation::PSTR,
    pub SourcePath: super::super::Foundation::PSTR,
    pub SourceFilename: super::super::Foundation::PSTR,
    pub SourceDescription: super::super::Foundation::PSTR,
    pub SourceTagfile: super::super::Foundation::PSTR,
    pub TargetDirectory: super::super::Foundation::PSTR,
    pub TargetFilename: super::super::Foundation::PSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: super::super::Foundation::PWSTR,
    pub SourcePath: super::super::Foundation::PWSTR,
    pub SourceFilename: super::super::Foundation::PWSTR,
    pub SourceDescription: super::super::Foundation::PWSTR,
    pub SourceTagfile: super::super::Foundation::PWSTR,
    pub TargetDirectory: super::super::Foundation::PWSTR,
    pub TargetFilename: super::super::Foundation::PWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: super::super::Foundation::PWSTR,
    pub SourcePath: super::super::Foundation::PWSTR,
    pub SourceFilename: super::super::Foundation::PWSTR,
    pub SourceDescription: super::super::Foundation::PWSTR,
    pub SourceTagfile: super::super::Foundation::PWSTR,
    pub TargetDirectory: super::super::Foundation::PWSTR,
    pub TargetFilename: super::super::Foundation::PWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SP_FLAG_CABINETCONTINUATION: u32 = 2048u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_INF_INFORMATION {
    pub InfStyle: SP_INF_STYLE,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_INF_INFORMATION {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_INF_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_INF_INFORMATION {
    pub InfStyle: SP_INF_STYLE,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_INF_INFORMATION {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_INF_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SP_INF_STYLE = u32;
pub const INF_STYLE_NONE: SP_INF_STYLE = 0u32;
pub const INF_STYLE_OLDNT: SP_INF_STYLE = 1u32;
pub const INF_STYLE_WIN4: SP_INF_STYLE = 2u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::super::Foundation::LPARAM,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_INSTALLWIZARD_DATA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_INSTALLWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::super::Foundation::LPARAM,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_INSTALLWIZARD_DATA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_INSTALLWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SP_MAX_MACHINENAME_LENGTH: u32 = 263u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_NEWDEVICEWIZARD_DATA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_NEWDEVICEWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_NEWDEVICEWIZARD_DATA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_NEWDEVICEWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [super::super::Foundation::CHAR; 260],
    pub OriginalCatalogName: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [super::super::Foundation::CHAR; 260],
    pub OriginalCatalogName: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_POWERMESSAGEWAKE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [super::super::Foundation::CHAR; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_POWERMESSAGEWAKE_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_POWERMESSAGEWAKE_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_POWERMESSAGEWAKE_PARAMS_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_POWERMESSAGEWAKE_PARAMS_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: u32,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_PROPCHANGE_PARAMS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_PROPCHANGE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: u32,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_PROPCHANGE_PARAMS {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_PROPCHANGE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: *mut ::core::ffi::c_void,
    pub DeviceInfoData: *mut SP_DEVINFO_DATA,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_PROPSHEETPAGE_REQUEST {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_PROPSHEETPAGE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: *mut ::core::ffi::c_void,
    pub DeviceInfoData: *mut SP_DEVINFO_DATA,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_PROPSHEETPAGE_REQUEST {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_PROPSHEETPAGE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: super::super::Foundation::PSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: super::super::Foundation::PSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: super::super::Foundation::PWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: super::super::Foundation::PWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_REMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_REMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_REMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_REMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_SELECTDEVICE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [super::super::Foundation::CHAR; 60],
    pub Instructions: [super::super::Foundation::CHAR; 256],
    pub ListLabel: [super::super::Foundation::CHAR; 30],
    pub SubTitle: [super::super::Foundation::CHAR; 256],
    pub Reserved: [u8; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_SELECTDEVICE_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_SELECTDEVICE_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_SELECTDEVICE_PARAMS_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_SELECTDEVICE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_SELECTDEVICE_PARAMS_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_SELECTDEVICE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_TROUBLESHOOTER_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [super::super::Foundation::CHAR; 260],
    pub HtmlTroubleShooter: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_TROUBLESHOOTER_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_TROUBLESHOOTER_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_TROUBLESHOOTER_PARAMS_W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_TROUBLESHOOTER_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_TROUBLESHOOTER_PARAMS_W {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_TROUBLESHOOTER_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for SP_UNREMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for SP_UNREMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SP_UNREMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SP_UNREMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SRCINFO_DESCRIPTION: u32 = 3u32;
pub const SRCINFO_FLAGS: u32 = 4u32;
pub const SRCINFO_PATH: u32 = 1u32;
pub const SRCINFO_TAGFILE: u32 = 2u32;
pub const SRCINFO_TAGFILE2: u32 = 5u32;
pub const SRCLIST_APPEND: u32 = 512u32;
pub const SRCLIST_NOBROWSE: u32 = 2u32;
pub const SRCLIST_NOSTRIPPLATFORM: u32 = 1024u32;
pub const SRCLIST_SUBDIRS: u32 = 256u32;
pub const SRCLIST_SYSIFADMIN: u32 = 64u32;
pub const SRCLIST_SYSTEM: u32 = 16u32;
pub const SRCLIST_TEMPORARY: u32 = 1u32;
pub const SRCLIST_USER: u32 = 32u32;
pub const SRC_FLAGS_CABFILE: u32 = 16u32;
pub const SUOI_FORCEDELETE: u32 = 1u32;
pub const SUOI_INTERNAL1: u32 = 2u32;
pub type SetupFileLogInfo = i32;
pub const SetupFileLogSourceFilename: SetupFileLogInfo = 0i32;
pub const SetupFileLogChecksum: SetupFileLogInfo = 1i32;
pub const SetupFileLogDiskTagfile: SetupFileLogInfo = 2i32;
pub const SetupFileLogDiskDescription: SetupFileLogInfo = 3i32;
pub const SetupFileLogOtherInfo: SetupFileLogInfo = 4i32;
pub const SetupFileLogMax: SetupFileLogInfo = 5i32;
