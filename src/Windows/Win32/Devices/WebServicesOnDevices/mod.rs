#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceDiscoveryMechanism(pub i32);
pub const MulticastDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(0i32);
pub const DirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(1i32);
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(2i32);
impl ::core::convert::From<i32> for DeviceDiscoveryMechanism {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DeviceDiscoveryMechanism {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDAddress(pub ::windows::core::IUnknown);
impl IWSDAddress {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszbuffer), ::core::mem::transmute(cchlength), fsafe.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deserialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszbuffer.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDAddress {
    type Vtable = IWSDAddress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9574c6c_12a6_4f74_93a1_3318ff605759);
}
impl ::core::convert::From<IWSDAddress> for ::windows::core::IUnknown {
    fn from(value: IWSDAddress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDAddress> for ::windows::core::IUnknown {
    fn from(value: &IWSDAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDAsyncCallback(pub ::windows::core::IUnknown);
impl IWSDAsyncCallback {
    pub unsafe fn AsyncOperationComplete<'a, Param0: ::windows::core::IntoParam<'a, IWSDAsyncResult>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pasyncresult: Param0, pasyncstate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pasyncresult.into_param().abi(), pasyncstate.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDAsyncCallback {
    type Vtable = IWSDAsyncCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa63e109d_ce72_49e2_ba98_e845f5ee1666);
}
impl ::core::convert::From<IWSDAsyncCallback> for ::windows::core::IUnknown {
    fn from(value: IWSDAsyncCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDAsyncCallback> for ::windows::core::IUnknown {
    fn from(value: &IWSDAsyncCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDAsyncCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDAsyncCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pasyncresult: ::windows::core::RawPtr, pasyncstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDAsyncResult(pub ::windows::core::IUnknown);
impl IWSDAsyncResult {
    pub unsafe fn SetCallback<'a, Param0: ::windows::core::IntoParam<'a, IWSDAsyncCallback>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcallback: Param0, pasyncstate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi(), pasyncstate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWaitHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hwaithandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwaithandle.into_param().abi()).ok()
    }
    pub unsafe fn HasCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetAsyncState(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEvent(&self) -> ::windows::core::Result<WSD_EVENT> {
        let mut result__: <WSD_EVENT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WSD_EVENT>(result__)
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__: <IWSDEndpointProxy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDEndpointProxy>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDAsyncResult {
    type Vtable = IWSDAsyncResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11a9852a_8dd8_423e_b537_9356db4fbfb8);
}
impl ::core::convert::From<IWSDAsyncResult> for ::windows::core::IUnknown {
    fn from(value: IWSDAsyncResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDAsyncResult> for ::windows::core::IUnknown {
    fn from(value: &IWSDAsyncResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDAsyncResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDAsyncResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pasyncstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwaithandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppasyncstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pevent: *mut ::core::mem::ManuallyDrop<WSD_EVENT>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppendpoint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDAttachment(pub ::windows::core::IUnknown);
impl IWSDAttachment {}
unsafe impl ::windows::core::Interface for IWSDAttachment {
    type Vtable = IWSDAttachment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d55a616_9df8_4b09_b156_9ba351a48b76);
}
impl ::core::convert::From<IWSDAttachment> for ::windows::core::IUnknown {
    fn from(value: IWSDAttachment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDAttachment> for ::windows::core::IUnknown {
    fn from(value: &IWSDAttachment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAttachment_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDDeviceHost(pub ::windows::core::IUnknown);
impl IWSDDeviceHost {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Init<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IWSDXMLContext>>(&self, pszlocalid: Param0, pcontext: Param1, pphostaddresses: *const ::core::option::Option<IWSDAddress>, dwhostaddresscount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(pphostaddresses), ::core::mem::transmute(dwhostaddresscount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<'a, Param2: ::windows::core::IntoParam<'a, IWSDDeviceHostNotify>>(&self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullinstanceid), ::core::mem::transmute(pscopelist), pnotificationsink.into_param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pporttype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pthismodelmetadata), ::core::mem::transmute(pthisdevicemetadata), ::core::mem::transmute(phostmetadata), ::core::mem::transmute(pcustommetadata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pszserviceid: Param0, pservice: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszserviceid.into_param().abi(), pservice.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RetireService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszserviceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDynamicService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pszserviceid: Param0, pszendpointaddress: Param1, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pszserviceid.into_param().abi(), pszendpointaddress.into_param().abi(), ::core::mem::transmute(pporttype), ::core::mem::transmute(pportname), ::core::mem::transmute(pany), pservice.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveDynamicService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pszserviceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceDiscoverable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszserviceid: Param0, fdiscoverable: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pszserviceid.into_param().abi(), fdiscoverable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SignalEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pszserviceid.into_param().abi(), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDDeviceHost {
    type Vtable = IWSDDeviceHost_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x917fe891_3d13_4138_9809_934c8abeb12c);
}
impl ::core::convert::From<IWSDDeviceHost> for ::windows::core::IUnknown {
    fn from(value: IWSDDeviceHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDDeviceHost> for ::windows::core::IUnknown {
    fn from(value: &IWSDDeviceHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDDeviceHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDDeviceHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHost_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, pphostaddresses: *const ::windows::core::RawPtr, dwhostaddresscount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszserviceid: super::super::Foundation::PWSTR, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszserviceid: super::super::Foundation::PWSTR, pszendpointaddress: super::super::Foundation::PWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszserviceid: super::super::Foundation::PWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszserviceid: super::super::Foundation::PWSTR, pbody: *const ::core::ffi::c_void, poperation: *const ::core::mem::ManuallyDrop<WSD_OPERATION>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDDeviceHostNotify(pub ::windows::core::IUnknown);
impl IWSDDeviceHostNotify {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszserviceid.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDDeviceHostNotify {
    type Vtable = IWSDDeviceHostNotify_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5bee9f9_eeda_41fe_96f7_f45e14990fb0);
}
impl ::core::convert::From<IWSDDeviceHostNotify> for ::windows::core::IUnknown {
    fn from(value: IWSDDeviceHostNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDDeviceHostNotify> for ::windows::core::IUnknown {
    fn from(value: &IWSDDeviceHostNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDDeviceHostNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDDeviceHostNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHostNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszserviceid: super::super::Foundation::PWSTR, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDDeviceProxy(pub ::windows::core::IUnknown);
impl IWSDDeviceProxy {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Init<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IWSDAddress>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IWSDXMLContext>, Param4: ::windows::core::IntoParam<'a, IWSDDeviceProxy>>(&self, pszdeviceid: Param0, pdeviceaddress: Param1, pszlocalid: Param2, pcontext: Param3, psponsor: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszdeviceid.into_param().abi(), pdeviceaddress.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), psponsor.into_param().abi()).ok()
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndGetMetadata<'a, Param0: ::windows::core::IntoParam<'a, IWSDAsyncResult>>(&self, presult: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), presult.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHostMetadata(&self) -> ::windows::core::Result<*mut WSD_HOST_METADATA> {
        let mut result__: <*mut WSD_HOST_METADATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_HOST_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetThisModelMetadata(&self) -> ::windows::core::Result<*mut WSD_THIS_MODEL_METADATA> {
        let mut result__: <*mut WSD_THIS_MODEL_METADATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_THIS_MODEL_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetThisDeviceMetadata(&self) -> ::windows::core::Result<*mut WSD_THIS_DEVICE_METADATA> {
        let mut result__: <*mut WSD_THIS_DEVICE_METADATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_THIS_DEVICE_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetServiceProxyById<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0) -> ::windows::core::Result<IWSDServiceProxy> {
        let mut result__: <IWSDServiceProxy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszserviceid.into_param().abi(), &mut result__).from_abi::<IWSDServiceProxy>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> ::windows::core::Result<IWSDServiceProxy> {
        let mut result__: <IWSDServiceProxy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), &mut result__).from_abi::<IWSDServiceProxy>(result__)
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__: <IWSDEndpointProxy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDEndpointProxy>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDDeviceProxy {
    type Vtable = IWSDDeviceProxy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeee0c031_c578_4c0e_9a3b_973c35f409db);
}
impl ::core::convert::From<IWSDDeviceProxy> for ::windows::core::IUnknown {
    fn from(value: IWSDDeviceProxy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDDeviceProxy> for ::windows::core::IUnknown {
    fn from(value: &IWSDDeviceProxy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDDeviceProxy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDDeviceProxy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceProxy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: ::windows::core::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, psponsor: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszserviceid: super::super::Foundation::PWSTR, ppserviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptype: *const WSDXML_NAME, ppserviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDEndpointProxy(pub ::windows::core::IUnknown);
impl IWSDEndpointProxy {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendOneWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendTwoWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation), ::core::mem::transmute(presponsecontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendTwoWayRequestAsync<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, IWSDAsyncCallback>>(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: Param2, pcallback: Param3) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation), pasyncstate.into_param().abi(), pcallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn AbortAsyncOperation<'a, Param0: ::windows::core::IntoParam<'a, IWSDAsyncResult>>(&self, pasyncresult: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pasyncresult.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfault)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetErrorInfo(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFaultInfo(&self) -> ::windows::core::Result<*mut WSD_SOAP_FAULT> {
        let mut result__: <*mut WSD_SOAP_FAULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDEndpointProxy {
    type Vtable = IWSDEndpointProxy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1860d430_b24c_4975_9f90_dbb39baa24ec);
}
impl ::core::convert::From<IWSDEndpointProxy> for ::windows::core::IUnknown {
    fn from(value: IWSDEndpointProxy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDEndpointProxy> for ::windows::core::IUnknown {
    fn from(value: &IWSDEndpointProxy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDEndpointProxy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDEndpointProxy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEndpointProxy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbody: *const ::core::ffi::c_void, poperation: *const ::core::mem::ManuallyDrop<WSD_OPERATION>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbody: *const ::core::ffi::c_void, poperation: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, presponsecontext: *const ::core::mem::ManuallyDrop<WSD_SYNCHRONOUS_RESPONSE_CONTEXT>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbody: *const ::core::ffi::c_void, poperation: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, pasyncstate: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, presult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pasyncresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszerrorinfo: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDEventingStatus(pub ::windows::core::IUnknown);
impl IWSDEventingStatus {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriptionRenewed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubscriptionaction: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszsubscriptionaction.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriptionRenewalFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubscriptionaction: Param0, hr: ::windows::core::HRESULT) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszsubscriptionaction.into_param().abi(), ::core::mem::transmute(hr)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriptionEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubscriptionaction: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszsubscriptionaction.into_param().abi()))
    }
}
unsafe impl ::windows::core::Interface for IWSDEventingStatus {
    type Vtable = IWSDEventingStatus_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49b17f52_637a_407a_ae99_fbe82a4d38c0);
}
impl ::core::convert::From<IWSDEventingStatus> for ::windows::core::IUnknown {
    fn from(value: IWSDEventingStatus) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDEventingStatus> for ::windows::core::IUnknown {
    fn from(value: &IWSDEventingStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDEventingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDEventingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEventingStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszsubscriptionaction: super::super::Foundation::PWSTR),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszsubscriptionaction: super::super::Foundation::PWSTR, hr: ::windows::core::HRESULT),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszsubscriptionaction: super::super::Foundation::PWSTR),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDHttpAddress(pub ::windows::core::IUnknown);
impl IWSDHttpAddress {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszbuffer), ::core::mem::transmute(cchlength), fsafe.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deserialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddress(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddressEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsafe: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fsafe.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransportAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszaddress.into_param().abi()).ok()
    }
    pub unsafe fn GetSecure(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecure<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsecure: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), fsecure.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDHttpAddress {
    type Vtable = IWSDHttpAddress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd09ac7bd_2a3e_4b85_8605_2737ff3e4ea0);
}
impl ::core::convert::From<IWSDHttpAddress> for ::windows::core::IUnknown {
    fn from(value: IWSDHttpAddress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDHttpAddress> for ::windows::core::IUnknown {
    fn from(value: &IWSDHttpAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDHttpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDHttpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWSDHttpAddress> for IWSDTransportAddress {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAddress> for IWSDTransportAddress {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDTransportAddress> for IWSDHttpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDTransportAddress> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDTransportAddress> for &IWSDHttpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDTransportAddress> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDHttpAddress> for IWSDAddress {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAddress> for IWSDAddress {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAddress> for IWSDHttpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAddress> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAddress> for &IWSDHttpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAddress> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwport: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wport: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fsecure: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDHttpAuthParameters(pub ::windows::core::IUnknown);
impl IWSDHttpAuthParameters {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientAccessToken(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    pub unsafe fn GetAuthType(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDHttpAuthParameters {
    type Vtable = IWSDHttpAuthParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b476df0_8dac_480d_b05c_99781a5884aa);
}
impl ::core::convert::From<IWSDHttpAuthParameters> for ::windows::core::IUnknown {
    fn from(value: IWSDHttpAuthParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDHttpAuthParameters> for ::windows::core::IUnknown {
    fn from(value: &IWSDHttpAuthParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDHttpAuthParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDHttpAuthParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAuthParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pauthtype: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDHttpMessageParameters(pub ::windows::core::IUnknown);
impl IWSDHttpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetLocalAddress<'a, Param0: ::windows::core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetRemoteAddress<'a, Param0: ::windows::core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows::core::Result<IWSDMessageParameters> {
        let mut result__: <IWSDMessageParameters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDMessageParameters>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInboundHttpHeaders<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszheaders: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pszheaders.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInboundHttpHeaders(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutboundHttpHeaders<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszheaders: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszheaders.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutboundHttpHeaders(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pszid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetID(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDHttpMessageParameters {
    type Vtable = IWSDHttpMessageParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x540bd122_5c83_4dec_b396_ea62a2697fdf);
}
impl ::core::convert::From<IWSDHttpMessageParameters> for ::windows::core::IUnknown {
    fn from(value: IWSDHttpMessageParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDHttpMessageParameters> for ::windows::core::IUnknown {
    fn from(value: &IWSDHttpMessageParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWSDHttpMessageParameters> for IWSDMessageParameters {
    fn from(value: IWSDHttpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpMessageParameters> for IWSDMessageParameters {
    fn from(value: &IWSDHttpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDMessageParameters> for IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDMessageParameters> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDMessageParameters> for &IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDMessageParameters> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpMessageParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptxparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDInboundAttachment(pub ::windows::core::IUnknown);
impl IWSDInboundAttachment {
    pub unsafe fn Read(&self, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwbytestoread), ::core::mem::transmute(pdwnumberofbytesread)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDInboundAttachment {
    type Vtable = IWSDInboundAttachment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bd6ca65_233c_4fb8_9f7a_2641619655c9);
}
impl ::core::convert::From<IWSDInboundAttachment> for ::windows::core::IUnknown {
    fn from(value: IWSDInboundAttachment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDInboundAttachment> for ::windows::core::IUnknown {
    fn from(value: &IWSDInboundAttachment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDInboundAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDInboundAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWSDInboundAttachment> for IWSDAttachment {
    fn from(value: IWSDInboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDInboundAttachment> for IWSDAttachment {
    fn from(value: &IWSDInboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAttachment> for IWSDInboundAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAttachment> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAttachment> for &IWSDInboundAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAttachment> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDInboundAttachment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDMessageParameters(pub ::windows::core::IUnknown);
impl IWSDMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetLocalAddress<'a, Param0: ::windows::core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetRemoteAddress<'a, Param0: ::windows::core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows::core::Result<IWSDMessageParameters> {
        let mut result__: <IWSDMessageParameters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDMessageParameters>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDMessageParameters {
    type Vtable = IWSDMessageParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fafe8a2_e6fc_4b80_b6cf_b7d45c416d7c);
}
impl ::core::convert::From<IWSDMessageParameters> for ::windows::core::IUnknown {
    fn from(value: IWSDMessageParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDMessageParameters> for ::windows::core::IUnknown {
    fn from(value: &IWSDMessageParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMessageParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptxparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDMetadataExchange(pub ::windows::core::IUnknown);
impl IWSDMetadataExchange {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDMetadataExchange {
    type Vtable = IWSDMetadataExchange_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06996d57_1d67_4928_9307_3d7833fdb846);
}
impl ::core::convert::From<IWSDMetadataExchange> for ::windows::core::IUnknown {
    fn from(value: IWSDMetadataExchange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDMetadataExchange> for ::windows::core::IUnknown {
    fn from(value: &IWSDMetadataExchange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDMetadataExchange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDMetadataExchange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMetadataExchange_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDOutboundAttachment(pub ::windows::core::IUnknown);
impl IWSDOutboundAttachment {
    pub unsafe fn Write(&self, pbuffer: *const u8, dwbytestowrite: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwbytestowrite), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDOutboundAttachment {
    type Vtable = IWSDOutboundAttachment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa302f8d_5a22_4ba5_b392_aa8486f4c15d);
}
impl ::core::convert::From<IWSDOutboundAttachment> for ::windows::core::IUnknown {
    fn from(value: IWSDOutboundAttachment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDOutboundAttachment> for ::windows::core::IUnknown {
    fn from(value: &IWSDOutboundAttachment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDOutboundAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDOutboundAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWSDOutboundAttachment> for IWSDAttachment {
    fn from(value: IWSDOutboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDOutboundAttachment> for IWSDAttachment {
    fn from(value: &IWSDOutboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAttachment> for IWSDOutboundAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAttachment> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAttachment> for &IWSDOutboundAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAttachment> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDOutboundAttachment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDSSLClientCertificate(pub ::windows::core::IUnknown);
impl IWSDSSLClientCertificate {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn GetClientCertificate(&self) -> ::windows::core::Result<*mut super::super::Security::Cryptography::CERT_CONTEXT> {
        let mut result__: <*mut super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::Security::Cryptography::CERT_CONTEXT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMappedAccessToken(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDSSLClientCertificate {
    type Vtable = IWSDSSLClientCertificate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde105e87_a0da_418e_98ad_27b9eed87bdc);
}
impl ::core::convert::From<IWSDSSLClientCertificate> for ::windows::core::IUnknown {
    fn from(value: IWSDSSLClientCertificate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDSSLClientCertificate> for ::windows::core::IUnknown {
    fn from(value: &IWSDSSLClientCertificate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDSSLClientCertificate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDSSLClientCertificate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSSLClientCertificate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDScopeMatchingRule(pub ::windows::core::IUnknown);
impl IWSDScopeMatchingRule {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScopeRule(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchScopes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszscope1: Param0, pszscope2: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszscope1.into_param().abi(), pszscope2.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDScopeMatchingRule {
    type Vtable = IWSDScopeMatchingRule_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcafe424_fef5_481a_bd9f_33ce0574256f);
}
impl ::core::convert::From<IWSDScopeMatchingRule> for ::windows::core::IUnknown {
    fn from(value: IWSDScopeMatchingRule) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDScopeMatchingRule> for ::windows::core::IUnknown {
    fn from(value: &IWSDScopeMatchingRule) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDScopeMatchingRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDScopeMatchingRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDScopeMatchingRule_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszscopematchingrule: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszscope1: super::super::Foundation::PWSTR, pszscope2: super::super::Foundation::PWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDServiceMessaging(pub ::windows::core::IUnknown);
impl IWSDServiceMessaging {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendResponse<'a, Param2: ::windows::core::IntoParam<'a, IWSDMessageParameters>>(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation), pmessageparameters.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FaultRequest<'a, Param1: ::windows::core::IntoParam<'a, IWSDMessageParameters>>(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: Param1, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequestheader), pmessageparameters.into_param().abi(), ::core::mem::transmute(pfault)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDServiceMessaging {
    type Vtable = IWSDServiceMessaging_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94974cf4_0cab_460d_a3f6_7a0ad623c0e6);
}
impl ::core::convert::From<IWSDServiceMessaging> for ::windows::core::IUnknown {
    fn from(value: IWSDServiceMessaging) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDServiceMessaging> for ::windows::core::IUnknown {
    fn from(value: &IWSDServiceMessaging) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDServiceMessaging {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDServiceMessaging {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceMessaging_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbody: *const ::core::ffi::c_void, poperation: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: ::windows::core::RawPtr, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDServiceProxy(pub ::windows::core::IUnknown);
impl IWSDServiceProxy {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndGetMetadata<'a, Param0: ::windows::core::IntoParam<'a, IWSDAsyncResult>>(&self, presult: Param0) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), presult.into_param().abi(), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetServiceMetadata(&self) -> ::windows::core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__: <*mut WSD_SERVICE_METADATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_SERVICE_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscribeToOperation<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, poperation: *const WSD_OPERATION, punknown: Param1, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<*mut WSDXML_ELEMENT> {
        let mut result__: <*mut WSDXML_ELEMENT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperation), punknown.into_param().abi(), ::core::mem::transmute(pany), &mut result__).from_abi::<*mut WSDXML_ELEMENT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperation)).ok()
    }
    pub unsafe fn SetEventingStatusCallback<'a, Param0: ::windows::core::IntoParam<'a, IWSDEventingStatus>>(&self, pstatus: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pstatus.into_param().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__: <IWSDEndpointProxy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDEndpointProxy>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDServiceProxy {
    type Vtable = IWSDServiceProxy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4c7fb9c_03ab_4175_9d67_094fafebf487);
}
impl ::core::convert::From<IWSDServiceProxy> for ::windows::core::IUnknown {
    fn from(value: IWSDServiceProxy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDServiceProxy> for ::windows::core::IUnknown {
    fn from(value: &IWSDServiceProxy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDServiceProxy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDServiceProxy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWSDServiceProxy> for IWSDMetadataExchange {
    fn from(value: IWSDServiceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxy> for IWSDMetadataExchange {
    fn from(value: &IWSDServiceProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDMetadataExchange> for IWSDServiceProxy {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDMetadataExchange> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDMetadataExchange> for &IWSDServiceProxy {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDMetadataExchange> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presult: ::windows::core::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperation: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, punknown: ::windows::core::RawPtr, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperation: *const ::core::mem::ManuallyDrop<WSD_OPERATION>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDServiceProxyEventing(pub ::windows::core::IUnknown);
impl IWSDServiceProxyEventing {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndGetMetadata<'a, Param0: ::windows::core::IntoParam<'a, IWSDAsyncResult>>(&self, presult: Param0) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), presult.into_param().abi(), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetServiceMetadata(&self) -> ::windows::core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__: <*mut WSD_SERVICE_METADATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_SERVICE_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscribeToOperation<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, poperation: *const WSD_OPERATION, punknown: Param1, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<*mut WSDXML_ELEMENT> {
        let mut result__: <*mut WSDXML_ELEMENT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperation), punknown.into_param().abi(), ::core::mem::transmute(pany), &mut result__).from_abi::<*mut WSDXML_ELEMENT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperation)).ok()
    }
    pub unsafe fn SetEventingStatusCallback<'a, Param0: ::windows::core::IntoParam<'a, IWSDEventingStatus>>(&self, pstatus: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pstatus.into_param().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__: <IWSDEndpointProxy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDEndpointProxy>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscribeToMultipleOperations<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: Param2, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), punknown.into_param().abi(), ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginSubscribeToMultipleOperations<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param5: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param6: ::windows::core::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: Param2, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: Param5, pasynccallback: Param6) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), punknown.into_param().abi(), ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndSubscribeToMultipleOperations<'a, Param2: ::windows::core::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Param2, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), presult.into_param().abi(), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), ::core::mem::transmute(pany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUnsubscribeToMultipleOperations<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: Param3, pasynccallback: Param4) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), ::core::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndUnsubscribeToMultipleOperations<'a, Param2: ::windows::core::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), presult.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginRenewMultipleOperations<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param5: ::windows::core::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: Param4, pasynccallback: Param5) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndRenewMultipleOperations<'a, Param2: ::windows::core::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Param2, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), presult.into_param().abi(), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), ::core::mem::transmute(pany), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginGetStatusForMultipleOperations<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: Param3, pasynccallback: Param4) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), ::core::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndGetStatusForMultipleOperations<'a, Param2: ::windows::core::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Param2, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(poperations), ::core::mem::transmute(dwoperationcount), presult.into_param().abi(), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDServiceProxyEventing {
    type Vtable = IWSDServiceProxyEventing_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9279d6d_1012_4a94_b8cc_fd35d2202bfe);
}
impl ::core::convert::From<IWSDServiceProxyEventing> for ::windows::core::IUnknown {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDServiceProxyEventing> for ::windows::core::IUnknown {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWSDServiceProxyEventing> for IWSDServiceProxy {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxyEventing> for IWSDServiceProxy {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDServiceProxy> for IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDServiceProxy> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDServiceProxy> for &IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDServiceProxy> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDServiceProxyEventing> for IWSDMetadataExchange {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxyEventing> for IWSDMetadataExchange {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDMetadataExchange> for IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDMetadataExchange> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDMetadataExchange> for &IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDMetadataExchange> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxyEventing_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presult: ::windows::core::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperation: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, punknown: ::windows::core::RawPtr, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperation: *const ::core::mem::ManuallyDrop<WSD_OPERATION>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, punknown: ::windows::core::RawPtr, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, punknown: ::windows::core::RawPtr, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: ::windows::core::RawPtr, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: ::windows::core::RawPtr, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: ::windows::core::RawPtr, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: ::windows::core::RawPtr, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poperations: *const ::core::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDSignatureProperty(pub ::windows::core::IUnknown);
impl IWSDSignatureProperty {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMessageSigned(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMessageSignatureTrusted(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetKeyInfo(&self, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbkeyinfo), ::core::mem::transmute(pdwkeyinfosize)).ok()
    }
    pub unsafe fn GetSignature(&self, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbsignature), ::core::mem::transmute(pdwsignaturesize)).ok()
    }
    pub unsafe fn GetSignedInfoHash(&self, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbsignedinfohash), ::core::mem::transmute(pdwhashsize)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDSignatureProperty {
    type Vtable = IWSDSignatureProperty_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03ce20aa_71c4_45e2_b32e_3766c61c790f);
}
impl ::core::convert::From<IWSDSignatureProperty> for ::windows::core::IUnknown {
    fn from(value: IWSDSignatureProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDSignatureProperty> for ::windows::core::IUnknown {
    fn from(value: &IWSDSignatureProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDSignatureProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDSignatureProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSignatureProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDTransportAddress(pub ::windows::core::IUnknown);
impl IWSDTransportAddress {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszbuffer), ::core::mem::transmute(cchlength), fsafe.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deserialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddress(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddressEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsafe: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fsafe.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransportAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszaddress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDTransportAddress {
    type Vtable = IWSDTransportAddress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70d23498_4ee6_4340_a3df_d845d2235467);
}
impl ::core::convert::From<IWSDTransportAddress> for ::windows::core::IUnknown {
    fn from(value: IWSDTransportAddress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDTransportAddress> for ::windows::core::IUnknown {
    fn from(value: &IWSDTransportAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDTransportAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDTransportAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWSDTransportAddress> for IWSDAddress {
    fn from(value: IWSDTransportAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDTransportAddress> for IWSDAddress {
    fn from(value: &IWSDTransportAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAddress> for IWSDTransportAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAddress> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAddress> for &IWSDTransportAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAddress> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDTransportAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwport: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wport: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDUdpAddress(pub ::windows::core::IUnknown);
impl IWSDUdpAddress {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszbuffer), ::core::mem::transmute(cchlength), fsafe.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deserialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddress(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddressEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsafe: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fsafe.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransportAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszaddress.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub unsafe fn SetSockaddr(&self, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(psockaddr)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub unsafe fn GetSockaddr(&self) -> ::windows::core::Result<super::super::Networking::WinSock::SOCKADDR_STORAGE> {
        let mut result__: <super::super::Networking::WinSock::SOCKADDR_STORAGE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Networking::WinSock::SOCKADDR_STORAGE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExclusive<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fexclusive: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), fexclusive.into_param().abi()).ok()
    }
    pub unsafe fn GetExclusive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagetype)).ok()
    }
    pub unsafe fn GetMessageType(&self) -> ::windows::core::Result<WSDUdpMessageType> {
        let mut result__: <WSDUdpMessageType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WSDUdpMessageType>(result__)
    }
    pub unsafe fn SetTTL(&self, dwttl: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwttl)).ok()
    }
    pub unsafe fn GetTTL(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAlias(&self, palias: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(palias)).ok()
    }
    pub unsafe fn GetAlias(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDUdpAddress {
    type Vtable = IWSDUdpAddress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d6124a_a441_4f78_a1eb_97a8d1996893);
}
impl ::core::convert::From<IWSDUdpAddress> for ::windows::core::IUnknown {
    fn from(value: IWSDUdpAddress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDUdpAddress> for ::windows::core::IUnknown {
    fn from(value: &IWSDUdpAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDUdpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDUdpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWSDUdpAddress> for IWSDTransportAddress {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpAddress> for IWSDTransportAddress {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDTransportAddress> for IWSDUdpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDTransportAddress> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDTransportAddress> for &IWSDUdpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDTransportAddress> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDUdpAddress> for IWSDAddress {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpAddress> for IWSDAddress {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAddress> for IWSDUdpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAddress> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDAddress> for &IWSDUdpAddress {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDAddress> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwport: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wport: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fexclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, messagetype: WSDUdpMessageType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessagetype: *mut WSDUdpMessageType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwttl: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwttl: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, palias: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, palias: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDUdpMessageParameters(pub ::windows::core::IUnknown);
impl IWSDUdpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetLocalAddress<'a, Param0: ::windows::core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetRemoteAddress<'a, Param0: ::windows::core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows::core::Result<IWSDMessageParameters> {
        let mut result__: <IWSDMessageParameters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDMessageParameters>(result__)
    }
    pub unsafe fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pparams)).ok()
    }
    pub unsafe fn GetRetransmitParams(&self) -> ::windows::core::Result<WSDUdpRetransmitParams> {
        let mut result__: <WSDUdpRetransmitParams as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WSDUdpRetransmitParams>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDUdpMessageParameters {
    type Vtable = IWSDUdpMessageParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9934149f_8f0c_447b_aa0b_73124b0ca7f0);
}
impl ::core::convert::From<IWSDUdpMessageParameters> for ::windows::core::IUnknown {
    fn from(value: IWSDUdpMessageParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDUdpMessageParameters> for ::windows::core::IUnknown {
    fn from(value: &IWSDUdpMessageParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWSDUdpMessageParameters> for IWSDMessageParameters {
    fn from(value: IWSDUdpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpMessageParameters> for IWSDMessageParameters {
    fn from(value: &IWSDUdpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDMessageParameters> for IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDMessageParameters> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWSDMessageParameters> for &IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows::core::Param<'a, IWSDMessageParameters> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpMessageParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptxparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pparams: *mut WSDUdpRetransmitParams) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDXMLContext(pub ::windows::core::IUnknown);
impl IWSDXMLContext {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddNamespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszuri: Param0, pszsuggestedprefix: Param1) -> ::windows::core::Result<*mut WSDXML_NAMESPACE> {
        let mut result__: <*mut WSDXML_NAMESPACE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszuri.into_param().abi(), pszsuggestedprefix.into_param().abi(), &mut result__).from_abi::<*mut WSDXML_NAMESPACE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddNameToNamespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszuri: Param0, pszname: Param1) -> ::windows::core::Result<*mut WSDXML_NAME> {
        let mut result__: <*mut WSDXML_NAME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszuri.into_param().abi(), pszname.into_param().abi(), &mut result__).from_abi::<*mut WSDXML_NAME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNamespaces(&self, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnamespaces), ::core::mem::transmute(wnamespacescount), ::core::mem::transmute(blayernumber)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTypes(&self, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptypes), ::core::mem::transmute(dwtypescount), ::core::mem::transmute(blayernumber)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDXMLContext {
    type Vtable = IWSDXMLContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d8f3ee_3e5a_43b4_a15a_bcf6887460c0);
}
impl ::core::convert::From<IWSDXMLContext> for ::windows::core::IUnknown {
    fn from(value: IWSDXMLContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDXMLContext> for ::windows::core::IUnknown {
    fn from(value: &IWSDXMLContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDXMLContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDXMLContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDXMLContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszuri: super::super::Foundation::PWSTR, pszsuggestedprefix: super::super::Foundation::PWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszuri: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDiscoveredService(pub ::windows::core::IUnknown);
impl IWSDiscoveredService {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEndpointReference(&self) -> ::windows::core::Result<*mut WSD_ENDPOINT_REFERENCE> {
        let mut result__: <*mut WSD_ENDPOINT_REFERENCE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_ENDPOINT_REFERENCE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTypes(&self) -> ::windows::core::Result<*mut WSD_NAME_LIST> {
        let mut result__: <*mut WSD_NAME_LIST as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_NAME_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScopes(&self) -> ::windows::core::Result<*mut WSD_URI_LIST> {
        let mut result__: <*mut WSD_URI_LIST as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_URI_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAddrs(&self) -> ::windows::core::Result<*mut WSD_URI_LIST> {
        let mut result__: <*mut WSD_URI_LIST as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_URI_LIST>(result__)
    }
    pub unsafe fn GetMetadataVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppheaderany), ::core::mem::transmute(ppbodyany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProbeResolveTag(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteTransportAddress(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalTransportAddress(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetLocalInterfaceGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveredService {
    type Vtable = IWSDiscoveredService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bad8a3b_b374_4420_9632_aac945b374aa);
}
impl ::core::convert::From<IWSDiscoveredService> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveredService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDiscoveredService> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveredService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDiscoveredService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDiscoveredService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveredService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pullmetadataversion: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsztag: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszremotetransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszlocaltransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pullinstanceid: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDiscoveryProvider(pub ::windows::core::IUnknown);
impl IWSDiscoveryProvider {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaddressfamily)).ok()
    }
    pub unsafe fn Attach<'a, Param0: ::windows::core::IntoParam<'a, IWSDiscoveryProviderNotify>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn Detach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchById<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszid: Param0, psztag: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszid.into_param().abi(), psztag.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchByAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszaddress: Param0, psztag: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszaddress.into_param().abi(), psztag.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchByType<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: Param2, psztag: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), pszmatchby.into_param().abi(), psztag.into_param().abi()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows::core::Result<IWSDXMLContext> {
        let mut result__: <IWSDXMLContext as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDXMLContext>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveryProvider {
    type Vtable = IWSDiscoveryProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ffc8e55_f0eb_480f_88b7_b435dd281d45);
}
impl ::core::convert::From<IWSDiscoveryProvider> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveryProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDiscoveryProvider> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveryProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDiscoveryProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDiscoveryProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaddressfamily: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszaddress: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDiscoveryProviderNotify(pub ::windows::core::IUnknown);
impl IWSDiscoveryProviderNotify {
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IWSDiscoveredService>>(&self, pservice: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pservice.into_param().abi()).ok()
    }
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, IWSDiscoveredService>>(&self, pservice: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pservice.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchFailed<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hr: ::windows::core::HRESULT, psztag: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr), psztag.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchComplete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psztag: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), psztag.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveryProviderNotify {
    type Vtable = IWSDiscoveryProviderNotify_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73ee3ced_b6e6_4329_a546_3e8ad46563d2);
}
impl ::core::convert::From<IWSDiscoveryProviderNotify> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveryProviderNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDiscoveryProviderNotify> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveryProviderNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDiscoveryProviderNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDiscoveryProviderNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProviderNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hr: ::windows::core::HRESULT, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDiscoveryPublisher(pub ::windows::core::IUnknown);
impl IWSDiscoveryPublisher {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaddressfamily)).ok()
    }
    pub unsafe fn RegisterNotificationSink<'a, Param0: ::windows::core::IntoParam<'a, IWSDiscoveryPublisherNotify>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn UnRegisterNotificationSink<'a, Param0: ::windows::core::IntoParam<'a, IWSDiscoveryPublisherNotify>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Publish<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszid: Param0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param4, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            pszid.into_param().abi(),
            ::core::mem::transmute(ullmetadataversion),
            ::core::mem::transmute(ullinstanceid),
            ::core::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnPublish<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszid: Param0, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param3, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszid.into_param().abi(), ::core::mem::transmute(ullinstanceid), ::core::mem::transmute(ullmessagenumber), pszsessionid.into_param().abi(), ::core::mem::transmute(pany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchProbe<'a, Param1: ::windows::core::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        pprobemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: Param1,
        pszid: Param2,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param6,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(pprobemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::core::mem::transmute(ullmetadataversion),
            ::core::mem::transmute(ullinstanceid),
            ::core::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchResolve<'a, Param1: ::windows::core::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        presolvemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: Param1,
        pszid: Param2,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param6,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(presolvemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::core::mem::transmute(ullmetadataversion),
            ::core::mem::transmute(ullinstanceid),
            ::core::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PublishEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        pszid: Param0,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param4,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            pszid.into_param().abi(),
            ::core::mem::transmute(ullmetadataversion),
            ::core::mem::transmute(ullinstanceid),
            ::core::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
            ::core::mem::transmute(pheaderany),
            ::core::mem::transmute(preferenceparameterany),
            ::core::mem::transmute(ppolicyany),
            ::core::mem::transmute(pendpointreferenceany),
            ::core::mem::transmute(pany),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchProbeEx<'a, Param1: ::windows::core::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        pprobemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: Param1,
        pszid: Param2,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param6,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(pprobemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::core::mem::transmute(ullmetadataversion),
            ::core::mem::transmute(ullinstanceid),
            ::core::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
            ::core::mem::transmute(pheaderany),
            ::core::mem::transmute(preferenceparameterany),
            ::core::mem::transmute(ppolicyany),
            ::core::mem::transmute(pendpointreferenceany),
            ::core::mem::transmute(pany),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchResolveEx<'a, Param1: ::windows::core::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        presolvemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: Param1,
        pszid: Param2,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param6,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(presolvemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::core::mem::transmute(ullmetadataversion),
            ::core::mem::transmute(ullinstanceid),
            ::core::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
            ::core::mem::transmute(pheaderany),
            ::core::mem::transmute(preferenceparameterany),
            ::core::mem::transmute(ppolicyany),
            ::core::mem::transmute(pendpointreferenceany),
            ::core::mem::transmute(pany),
        )
        .ok()
    }
    pub unsafe fn RegisterScopeMatchingRule<'a, Param0: ::windows::core::IntoParam<'a, IWSDScopeMatchingRule>>(&self, pscopematchingrule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pscopematchingrule.into_param().abi()).ok()
    }
    pub unsafe fn UnRegisterScopeMatchingRule<'a, Param0: ::windows::core::IntoParam<'a, IWSDScopeMatchingRule>>(&self, pscopematchingrule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pscopematchingrule.into_param().abi()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows::core::Result<IWSDXMLContext> {
        let mut result__: <IWSDXMLContext as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWSDXMLContext>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveryPublisher {
    type Vtable = IWSDiscoveryPublisher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae01e1a8_3ff9_4148_8116_057cc616fe13);
}
impl ::core::convert::From<IWSDiscoveryPublisher> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveryPublisher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDiscoveryPublisher> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveryPublisher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDiscoveryPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDiscoveryPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaddressfamily: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pszid: super::super::Foundation::PWSTR,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: super::super::Foundation::PWSTR,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pprobemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: ::windows::core::RawPtr,
        pszid: super::super::Foundation::PWSTR,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: super::super::Foundation::PWSTR,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        presolvemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: ::windows::core::RawPtr,
        pszid: super::super::Foundation::PWSTR,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: super::super::Foundation::PWSTR,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pscopematchingrule: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pscopematchingrule: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWSDiscoveryPublisherNotify(pub ::windows::core::IUnknown);
impl IWSDiscoveryPublisherNotify {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProbeHandler<'a, Param1: ::windows::core::IntoParam<'a, IWSDMessageParameters>>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(psoap), pmessageparameters.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResolveHandler<'a, Param1: ::windows::core::IntoParam<'a, IWSDMessageParameters>>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(psoap), pmessageparameters.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveryPublisherNotify {
    type Vtable = IWSDiscoveryPublisherNotify_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe67651b0_337a_4b3c_9758_733388568251);
}
impl ::core::convert::From<IWSDiscoveryPublisherNotify> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveryPublisherNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWSDiscoveryPublisherNotify> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveryPublisherNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSDiscoveryPublisherNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSDiscoveryPublisherNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisherNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[cfg(feature = "Win32_Foundation")]
pub type PWSD_SOAP_MESSAGE_HANDLER = unsafe extern "system" fn(thisunknown: ::windows::core::RawPtr, event: *mut ::core::mem::ManuallyDrop<WSD_EVENT>) -> ::windows::core::HRESULT;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl REQUESTBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQUESTBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REQUESTBODY_GetStatus {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("REQUESTBODY_GetStatus").field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQUESTBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQUESTBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REQUESTBODY_GetStatus {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl REQUESTBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQUESTBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REQUESTBODY_Renew {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("REQUESTBODY_Renew").field("Expires", &self.Expires).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQUESTBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        self.Expires == other.Expires && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQUESTBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REQUESTBODY_Renew {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl REQUESTBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQUESTBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REQUESTBODY_Subscribe {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("REQUESTBODY_Subscribe").field("EndTo", &self.EndTo).field("Delivery", &self.Delivery).field("Expires", &self.Expires).field("Filter", &self.Filter).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQUESTBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        self.EndTo == other.EndTo && self.Delivery == other.Delivery && self.Expires == other.Expires && self.Filter == other.Filter && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQUESTBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REQUESTBODY_Subscribe {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl REQUESTBODY_Unsubscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQUESTBODY_Unsubscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REQUESTBODY_Unsubscribe {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("REQUESTBODY_Unsubscribe").field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQUESTBODY_Unsubscribe {
    fn eq(&self, other: &Self) -> bool {
        self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQUESTBODY_Unsubscribe {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REQUESTBODY_Unsubscribe {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_GetMetadata {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_GetMetadata {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_GetMetadata {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_GetMetadata").field("Metadata", &self.Metadata).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_GetMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.Metadata == other.Metadata
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_GetMetadata {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_GetMetadata {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_GetStatus {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_GetStatus").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires && self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_GetStatus {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_Renew {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_Renew").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires && self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_Renew {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_Subscribe {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_Subscribe").field("SubscriptionManager", &self.SubscriptionManager).field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        self.SubscriptionManager == other.SubscriptionManager && self.expires == other.expires && self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_Subscribe {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: super::super::Foundation::PWSTR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_SubscriptionEnd {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_SubscriptionEnd {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_SubscriptionEnd {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_SubscriptionEnd").field("SubscriptionManager", &self.SubscriptionManager).field("Status", &self.Status).field("Reason", &self.Reason).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_SubscriptionEnd {
    fn eq(&self, other: &Self) -> bool {
        self.SubscriptionManager == other.SubscriptionManager && self.Status == other.Status && self.Reason == other.Reason && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_SubscriptionEnd {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_SubscriptionEnd {
    type Abi = Self;
}
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
#[inline]
pub unsafe fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(WSDAllocateLinkedMemory(::core::mem::transmute(pparent), ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(WSDAttachLinkedMemory(::core::mem::transmute(pparent), ::core::mem::transmute(pchild)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceHost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pszlocalid: Param0, pcontext: Param1) -> ::windows::core::Result<IWSDDeviceHost> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceHost(pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, ppdevicehost: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDDeviceHost as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDeviceHost(pszlocalid.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi::<IWSDDeviceHost>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceHost2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pszlocalid: Param0, pcontext: Param1, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32) -> ::windows::core::Result<IWSDDeviceHost> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceHost2(pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdevicehost: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDDeviceHost as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDeviceHost2(pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(pconfigparams), ::core::mem::transmute(dwconfigparamcount), &mut result__).from_abi::<IWSDDeviceHost>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceHostAdvanced<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pszlocalid: Param0, pcontext: Param1, pphostaddresses: *const ::core::option::Option<IWSDAddress>, dwhostaddresscount: u32) -> ::windows::core::Result<IWSDDeviceHost> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceHostAdvanced(pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, pphostaddresses: *const ::windows::core::RawPtr, dwhostaddresscount: u32, ppdevicehost: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDDeviceHost as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDeviceHostAdvanced(pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(pphostaddresses), ::core::mem::transmute(dwhostaddresscount), &mut result__).from_abi::<IWSDDeviceHost>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceProxy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pszdeviceid: Param0, pszlocalid: Param1, pcontext: Param2) -> ::windows::core::Result<IWSDDeviceProxy> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceProxy(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, ppdeviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDDeviceProxy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDeviceProxy(pszdeviceid.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi::<IWSDDeviceProxy>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceProxy2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pszdeviceid: Param0, pszlocalid: Param1, pcontext: Param2, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32) -> ::windows::core::Result<IWSDDeviceProxy> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceProxy2(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdeviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDDeviceProxy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDeviceProxy2(pszdeviceid.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(pconfigparams), ::core::mem::transmute(dwconfigparamcount), &mut result__).from_abi::<IWSDDeviceProxy>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceProxyAdvanced<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IWSDAddress>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pszdeviceid: Param0, pdeviceaddress: Param1, pszlocalid: Param2, pcontext: Param3) -> ::windows::core::Result<IWSDDeviceProxy> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceProxyAdvanced(pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: ::windows::core::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, ppdeviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDDeviceProxy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDeviceProxyAdvanced(pszdeviceid.into_param().abi(), pdeviceaddress.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi::<IWSDDeviceProxy>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider<'a, Param0: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0) -> ::windows::core::Result<IWSDiscoveryProvider> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryProvider(pcontext: ::windows::core::RawPtr, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDiscoveryProvider as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDiscoveryProvider(pcontext.into_param().abi(), &mut result__).from_abi::<IWSDiscoveryProvider>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider2<'a, Param0: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32) -> ::windows::core::Result<IWSDiscoveryProvider> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryProvider2(pcontext: ::windows::core::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDiscoveryProvider as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDiscoveryProvider2(pcontext.into_param().abi(), ::core::mem::transmute(pconfigparams), ::core::mem::transmute(dwconfigparamcount), &mut result__).from_abi::<IWSDiscoveryProvider>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher<'a, Param0: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0) -> ::windows::core::Result<IWSDiscoveryPublisher> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryPublisher(pcontext: ::windows::core::RawPtr, pppublisher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDiscoveryPublisher as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDiscoveryPublisher(pcontext.into_param().abi(), &mut result__).from_abi::<IWSDiscoveryPublisher>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher2<'a, Param0: ::windows::core::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32) -> ::windows::core::Result<IWSDiscoveryPublisher> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryPublisher2(pcontext: ::windows::core::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, pppublisher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDiscoveryPublisher as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateDiscoveryPublisher2(pcontext.into_param().abi(), ::core::mem::transmute(pconfigparams), ::core::mem::transmute(dwconfigparamcount), &mut result__).from_abi::<IWSDiscoveryPublisher>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateHttpAddress() -> ::windows::core::Result<IWSDHttpAddress> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateHttpAddress(ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDHttpAddress as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateHttpAddress(&mut result__).from_abi::<IWSDHttpAddress>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateHttpMessageParameters() -> ::windows::core::Result<IWSDHttpMessageParameters> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateHttpMessageParameters(pptxparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDHttpMessageParameters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateHttpMessageParameters(&mut result__).from_abi::<IWSDHttpMessageParameters>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateOutboundAttachment() -> ::windows::core::Result<IWSDOutboundAttachment> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateOutboundAttachment(ppattachment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDOutboundAttachment as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateOutboundAttachment(&mut result__).from_abi::<IWSDOutboundAttachment>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateUdpAddress() -> ::windows::core::Result<IWSDUdpAddress> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateUdpAddress(ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDUdpAddress as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateUdpAddress(&mut result__).from_abi::<IWSDUdpAddress>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateUdpMessageParameters() -> ::windows::core::Result<IWSDUdpMessageParameters> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateUdpMessageParameters(pptxparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDUdpMessageParameters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDCreateUdpMessageParameters(&mut result__).from_abi::<IWSDUdpMessageParameters>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(WSDDetachLinkedMemory(::core::mem::transmute(pvoid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSDEventType(pub i32);
pub const WSDET_NONE: WSDEventType = WSDEventType(0i32);
pub const WSDET_INCOMING_MESSAGE: WSDEventType = WSDEventType(1i32);
pub const WSDET_INCOMING_FAULT: WSDEventType = WSDEventType(2i32);
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = WSDEventType(3i32);
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = WSDEventType(4i32);
impl ::core::convert::From<i32> for WSDEventType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WSDEventType {
    type Abi = Self;
}
#[inline]
pub unsafe fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(WSDFreeLinkedMemory(::core::mem::transmute(pvoid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDGenerateFault<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, IWSDXMLContext>>(
    pszcode: Param0,
    pszsubcode: Param1,
    pszreason: Param2,
    pszdetail: Param3,
    pcontext: Param4,
) -> ::windows::core::Result<*mut WSD_SOAP_FAULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDGenerateFault(pszcode: super::super::Foundation::PWSTR, pszsubcode: super::super::Foundation::PWSTR, pszreason: super::super::Foundation::PWSTR, pszdetail: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut WSD_SOAP_FAULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDGenerateFault(pszcode.into_param().abi(), pszsubcode.into_param().abi(), pszreason.into_param().abi(), pszdetail.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDGenerateFaultEx<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: Param3) -> ::windows::core::Result<*mut WSD_SOAP_FAULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDGenerateFaultEx(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: super::super::Foundation::PWSTR, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut WSD_SOAP_FAULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDGenerateFaultEx(::core::mem::transmute(pcode), ::core::mem::transmute(psubcode), ::core::mem::transmute(preasons), pszdetail.into_param().abi(), &mut result__).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows::core::HRESULT;
        }
        WSDGetConfigurationOption(::core::mem::transmute(dwoption), ::core::mem::transmute(pvoid), ::core::mem::transmute(cboutbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows::core::HRESULT;
        }
        WSDSetConfigurationOption(::core::mem::transmute(dwoption), ::core::mem::transmute(pvoid), ::core::mem::transmute(cbinbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSDUdpMessageType(pub i32);
pub const ONE_WAY: WSDUdpMessageType = WSDUdpMessageType(0i32);
pub const TWO_WAY: WSDUdpMessageType = WSDUdpMessageType(1i32);
impl ::core::convert::From<i32> for WSDUdpMessageType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WSDUdpMessageType {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
impl WSDUdpRetransmitParams {}
impl ::core::default::Default for WSDUdpRetransmitParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WSDUdpRetransmitParams {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDUdpRetransmitParams").field("ulSendDelay", &self.ulSendDelay).field("ulRepeat", &self.ulRepeat).field("ulRepeatMinDelay", &self.ulRepeatMinDelay).field("ulRepeatMaxDelay", &self.ulRepeatMaxDelay).field("ulRepeatUpperDelay", &self.ulRepeatUpperDelay).finish()
    }
}
impl ::core::cmp::PartialEq for WSDUdpRetransmitParams {
    fn eq(&self, other: &Self) -> bool {
        self.ulSendDelay == other.ulSendDelay && self.ulRepeat == other.ulRepeat && self.ulRepeatMinDelay == other.ulRepeatMinDelay && self.ulRepeatMaxDelay == other.ulRepeatMaxDelay && self.ulRepeatUpperDelay == other.ulRepeatUpperDelay
    }
}
impl ::core::cmp::Eq for WSDUdpRetransmitParams {}
unsafe impl ::windows::core::Abi for WSDUdpRetransmitParams {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDUriDecode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(source: Param0, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDUriDecode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::core::HRESULT;
        }
        WSDUriDecode(source.into_param().abi(), ::core::mem::transmute(cchsource), ::core::mem::transmute(destout), ::core::mem::transmute(cchdestout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDUriEncode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(source: Param0, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDUriEncode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::core::HRESULT;
        }
        WSDUriEncode(source.into_param().abi(), ::core::mem::transmute(cchsource), ::core::mem::transmute(destout), ::core::mem::transmute(cchdestout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT;
        }
        WSDXMLAddChild(::core::mem::transmute(pparent), ::core::mem::transmute(pchild)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT;
        }
        WSDXMLAddSibling(::core::mem::transmute(pfirst), ::core::mem::transmute(psecond)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLBuildAnyForSingleElement<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pelementname: *mut WSDXML_NAME, psztext: Param1, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLBuildAnyForSingleElement(pelementname: *mut WSDXML_NAME, psztext: super::super::Foundation::PWSTR, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT;
        }
        WSDXMLBuildAnyForSingleElement(::core::mem::transmute(pelementname), psztext.into_param().abi(), ::core::mem::transmute(ppany)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT;
        }
        WSDXMLCleanupElement(::core::mem::transmute(pany)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDXMLCreateContext() -> ::windows::core::Result<IWSDXMLContext> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLCreateContext(ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWSDXMLContext as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDXMLCreateContext(&mut result__).from_abi::<IWSDXMLContext>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLGetNameFromBuiltinNamespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(psznamespace: Param0, pszname: Param1) -> ::windows::core::Result<*mut WSDXML_NAME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLGetNameFromBuiltinNamespace(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut WSDXML_NAME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WSDXMLGetNameFromBuiltinNamespace(psznamespace.into_param().abi(), pszname.into_param().abi(), &mut result__).from_abi::<*mut WSDXML_NAME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLGetValueFromAny<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(psznamespace: Param0, pszname: Param1, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLGetValueFromAny(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        WSDXMLGetValueFromAny(psznamespace.into_param().abi(), pszname.into_param().abi(), ::core::mem::transmute(pany), ::core::mem::transmute(ppszvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSDXML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSDXML_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDXML_ATTRIBUTE").field("Element", &self.Element).field("Next", &self.Next).field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSDXML_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.Element == other.Element && self.Next == other.Next && self.Name == other.Name && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSDXML_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSDXML_ATTRIBUTE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSDXML_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSDXML_ELEMENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDXML_ELEMENT").field("Node", &self.Node).field("Name", &self.Name).field("FirstAttribute", &self.FirstAttribute).field("FirstChild", &self.FirstChild).field("PrefixMappings", &self.PrefixMappings).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSDXML_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Node == other.Node && self.Name == other.Name && self.FirstAttribute == other.FirstAttribute && self.FirstChild == other.FirstChild && self.PrefixMappings == other.PrefixMappings
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSDXML_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSDXML_ELEMENT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_ELEMENT_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSDXML_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSDXML_ELEMENT_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDXML_ELEMENT_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSDXML_ELEMENT_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSDXML_ELEMENT_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSDXML_ELEMENT_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSDXML_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSDXML_NAME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDXML_NAME").field("Space", &self.Space).field("LocalName", &self.LocalName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSDXML_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Space == other.Space && self.LocalName == other.LocalName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSDXML_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSDXML_NAME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSDXML_NAMESPACE {
    pub Uri: super::super::Foundation::PWSTR,
    pub PreferredPrefix: super::super::Foundation::PWSTR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_NAMESPACE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSDXML_NAMESPACE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDXML_NAMESPACE").field("Uri", &self.Uri).field("PreferredPrefix", &self.PreferredPrefix).field("Names", &self.Names).field("NamesCount", &self.NamesCount).field("Encoding", &self.Encoding).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSDXML_NAMESPACE {
    fn eq(&self, other: &Self) -> bool {
        self.Uri == other.Uri && self.PreferredPrefix == other.PreferredPrefix && self.Names == other.Names && self.NamesCount == other.NamesCount && self.Encoding == other.Encoding
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSDXML_NAMESPACE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSDXML_NAMESPACE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_NODE {
    pub const ElementType: i32 = 0i32;
    pub const TextType: i32 = 1i32;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSDXML_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSDXML_NODE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDXML_NODE").field("Type", &self.Type).field("Parent", &self.Parent).field("Next", &self.Next).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSDXML_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Parent == other.Parent && self.Next == other.Next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSDXML_NODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSDXML_NODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSDXML_OP(pub i32);
pub const OpNone: WSDXML_OP = WSDXML_OP(0i32);
pub const OpEndOfTable: WSDXML_OP = WSDXML_OP(1i32);
pub const OpBeginElement_: WSDXML_OP = WSDXML_OP(2i32);
pub const OpBeginAnyElement: WSDXML_OP = WSDXML_OP(3i32);
pub const OpEndElement: WSDXML_OP = WSDXML_OP(4i32);
pub const OpElement_: WSDXML_OP = WSDXML_OP(5i32);
pub const OpAnyElement: WSDXML_OP = WSDXML_OP(6i32);
pub const OpAnyElements: WSDXML_OP = WSDXML_OP(7i32);
pub const OpAnyText: WSDXML_OP = WSDXML_OP(8i32);
pub const OpAttribute_: WSDXML_OP = WSDXML_OP(9i32);
pub const OpBeginChoice: WSDXML_OP = WSDXML_OP(10i32);
pub const OpEndChoice: WSDXML_OP = WSDXML_OP(11i32);
pub const OpBeginSequence: WSDXML_OP = WSDXML_OP(12i32);
pub const OpEndSequence: WSDXML_OP = WSDXML_OP(13i32);
pub const OpBeginAll: WSDXML_OP = WSDXML_OP(14i32);
pub const OpEndAll: WSDXML_OP = WSDXML_OP(15i32);
pub const OpAnything: WSDXML_OP = WSDXML_OP(16i32);
pub const OpAnyNumber: WSDXML_OP = WSDXML_OP(17i32);
pub const OpOneOrMore: WSDXML_OP = WSDXML_OP(18i32);
pub const OpOptional: WSDXML_OP = WSDXML_OP(19i32);
pub const OpFormatBool_: WSDXML_OP = WSDXML_OP(20i32);
pub const OpFormatInt8_: WSDXML_OP = WSDXML_OP(21i32);
pub const OpFormatInt16_: WSDXML_OP = WSDXML_OP(22i32);
pub const OpFormatInt32_: WSDXML_OP = WSDXML_OP(23i32);
pub const OpFormatInt64_: WSDXML_OP = WSDXML_OP(24i32);
pub const OpFormatUInt8_: WSDXML_OP = WSDXML_OP(25i32);
pub const OpFormatUInt16_: WSDXML_OP = WSDXML_OP(26i32);
pub const OpFormatUInt32_: WSDXML_OP = WSDXML_OP(27i32);
pub const OpFormatUInt64_: WSDXML_OP = WSDXML_OP(28i32);
pub const OpFormatUnicodeString_: WSDXML_OP = WSDXML_OP(29i32);
pub const OpFormatDom_: WSDXML_OP = WSDXML_OP(30i32);
pub const OpFormatStruct_: WSDXML_OP = WSDXML_OP(31i32);
pub const OpFormatUri_: WSDXML_OP = WSDXML_OP(32i32);
pub const OpFormatUuidUri_: WSDXML_OP = WSDXML_OP(33i32);
pub const OpFormatName_: WSDXML_OP = WSDXML_OP(34i32);
pub const OpFormatListInsertTail_: WSDXML_OP = WSDXML_OP(35i32);
pub const OpFormatType_: WSDXML_OP = WSDXML_OP(36i32);
pub const OpFormatDynamicType_: WSDXML_OP = WSDXML_OP(37i32);
pub const OpFormatLookupType_: WSDXML_OP = WSDXML_OP(38i32);
pub const OpFormatDuration_: WSDXML_OP = WSDXML_OP(39i32);
pub const OpFormatDateTime_: WSDXML_OP = WSDXML_OP(40i32);
pub const OpFormatFloat_: WSDXML_OP = WSDXML_OP(41i32);
pub const OpFormatDouble_: WSDXML_OP = WSDXML_OP(42i32);
pub const OpProcess_: WSDXML_OP = WSDXML_OP(43i32);
pub const OpQualifiedAttribute_: WSDXML_OP = WSDXML_OP(44i32);
pub const OpFormatXMLDeclaration_: WSDXML_OP = WSDXML_OP(45i32);
pub const OpFormatMax: WSDXML_OP = WSDXML_OP(46i32);
impl ::core::convert::From<i32> for WSDXML_OP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WSDXML_OP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_PREFIX_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSDXML_PREFIX_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSDXML_PREFIX_MAPPING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDXML_PREFIX_MAPPING").field("Refs", &self.Refs).field("Next", &self.Next).field("Space", &self.Space).field("Prefix", &self.Prefix).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSDXML_PREFIX_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.Refs == other.Refs && self.Next == other.Next && self.Space == other.Space && self.Prefix == other.Prefix
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSDXML_PREFIX_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSDXML_PREFIX_MAPPING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSDXML_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSDXML_TEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDXML_TEXT").field("Node", &self.Node).field("Text", &self.Text).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSDXML_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Node == other.Node && self.Text == other.Text
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSDXML_TEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSDXML_TEXT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSDXML_TYPE {
    pub Uri: super::super::Foundation::PWSTR,
    pub Table: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSDXML_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSDXML_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSDXML_TYPE").field("Uri", &self.Uri).field("Table", &self.Table).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSDXML_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Uri == other.Uri && self.Table == other.Table
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSDXML_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSDXML_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_APP_SEQUENCE {
    pub InstanceId: u64,
    pub SequenceId: super::super::Foundation::PWSTR,
    pub MessageNumber: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_APP_SEQUENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_APP_SEQUENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_APP_SEQUENCE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_APP_SEQUENCE").field("InstanceId", &self.InstanceId).field("SequenceId", &self.SequenceId).field("MessageNumber", &self.MessageNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_APP_SEQUENCE {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceId == other.InstanceId && self.SequenceId == other.SequenceId && self.MessageNumber == other.MessageNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_APP_SEQUENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_APP_SEQUENCE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_BYE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_BYE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_BYE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_BYE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_BYE {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_BYE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_BYE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut ::core::option::Option<IWSDAddress>,
    pub dwAddressCount: u32,
}
impl WSD_CONFIG_ADDRESSES {}
impl ::core::default::Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WSD_CONFIG_ADDRESSES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_CONFIG_ADDRESSES").field("addresses", &self.addresses).field("dwAddressCount", &self.dwAddressCount).finish()
    }
}
impl ::core::cmp::PartialEq for WSD_CONFIG_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        self.addresses == other.addresses && self.dwAddressCount == other.dwAddressCount
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_ADDRESSES {}
unsafe impl ::windows::core::Abi for WSD_CONFIG_ADDRESSES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut ::core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl WSD_CONFIG_PARAM {}
impl ::core::default::Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WSD_CONFIG_PARAM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_CONFIG_PARAM").field("configParamType", &self.configParamType).field("pConfigData", &self.pConfigData).field("dwConfigDataSize", &self.dwConfigDataSize).finish()
    }
}
impl ::core::cmp::PartialEq for WSD_CONFIG_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.configParamType == other.configParamType && self.pConfigData == other.pConfigData && self.dwConfigDataSize == other.dwConfigDataSize
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_PARAM {}
unsafe impl ::windows::core::Abi for WSD_CONFIG_PARAM {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSD_CONFIG_PARAM_TYPE(pub i32);
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(1i32);
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(2i32);
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(3i32);
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(4i32);
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(5i32);
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(6i32);
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(7i32);
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(8i32);
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(9i32);
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(10i32);
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(11i32);
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(12i32);
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(13i32);
impl ::core::convert::From<i32> for WSD_CONFIG_PARAM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WSD_CONFIG_PARAM_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_DATETIME {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub TZIsLocal: super::super::Foundation::BOOL,
    pub TZIsPositive: super::super::Foundation::BOOL,
    pub TZHour: u8,
    pub TZMinute: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_DATETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_DATETIME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_DATETIME")
            .field("isPositive", &self.isPositive)
            .field("year", &self.year)
            .field("month", &self.month)
            .field("day", &self.day)
            .field("hour", &self.hour)
            .field("minute", &self.minute)
            .field("second", &self.second)
            .field("millisecond", &self.millisecond)
            .field("TZIsLocal", &self.TZIsLocal)
            .field("TZIsPositive", &self.TZIsPositive)
            .field("TZHour", &self.TZHour)
            .field("TZMinute", &self.TZMinute)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.isPositive == other.isPositive && self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.millisecond == other.millisecond && self.TZIsLocal == other.TZIsLocal && self.TZIsPositive == other.TZIsPositive && self.TZHour == other.TZHour && self.TZMinute == other.TZMinute
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_DATETIME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_DATETIME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_DURATION {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_DURATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_DURATION").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_DURATION {
    fn eq(&self, other: &Self) -> bool {
        self.isPositive == other.isPositive && self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.millisecond == other.millisecond
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_DURATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_DURATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: super::super::Foundation::PWSTR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_ENDPOINT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_ENDPOINT_REFERENCE").field("Address", &self.Address).field("ReferenceProperties", &self.ReferenceProperties).field("ReferenceParameters", &self.ReferenceParameters).field("PortType", &self.PortType).field("ServiceName", &self.ServiceName).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.ReferenceProperties == other.ReferenceProperties && self.ReferenceParameters == other.ReferenceParameters && self.PortType == other.PortType && self.ServiceName == other.ServiceName && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_ENDPOINT_REFERENCE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_ENDPOINT_REFERENCE_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_ENDPOINT_REFERENCE_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_ENDPOINT_REFERENCE_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_EVENT {
    pub Hr: ::windows::core::HRESULT,
    pub EventType: u32,
    pub DispatchTag: super::super::Foundation::PWSTR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: ::core::option::Option<IWSDMessageParameters>,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_EVENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_EVENT").field("Hr", &self.Hr).field("EventType", &self.EventType).field("DispatchTag", &self.DispatchTag).field("HandlerContext", &self.HandlerContext).field("Soap", &self.Soap).field("Operation", &self.Operation).field("MessageParameters", &self.MessageParameters).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Hr == other.Hr && self.EventType == other.EventType && self.DispatchTag == other.DispatchTag && self.HandlerContext == other.HandlerContext && self.Soap == other.Soap && self.Operation == other.Operation && self.MessageParameters == other.MessageParameters
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_EVENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_EVENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: super::super::Foundation::PWSTR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_DELIVERY_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_DELIVERY_MODE").field("Mode", &self.Mode).field("Push", &self.Push).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Push == other.Push && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_EVENTING_DELIVERY_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_DELIVERY_MODE_PUSH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_DELIVERY_MODE_PUSH").field("NotifyTo", &self.NotifyTo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn eq(&self, other: &Self) -> bool {
        self.NotifyTo == other.NotifyTo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE_PUSH {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_EVENTING_DELIVERY_MODE_PUSH {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_EXPIRES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_EVENTING_EXPIRES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_EVENTING_EXPIRES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_EXPIRES").field("Duration", &self.Duration).field("DateTime", &self.DateTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_EVENTING_EXPIRES {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration && self.DateTime == other.DateTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_EVENTING_EXPIRES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_EVENTING_EXPIRES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_EVENTING_FILTER {
    pub Dialect: super::super::Foundation::PWSTR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_EVENTING_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_EVENTING_FILTER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_FILTER").field("Dialect", &self.Dialect).field("FilterAction", &self.FilterAction).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.Dialect == other.Dialect && self.FilterAction == other.FilterAction && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_EVENTING_FILTER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_EVENTING_FILTER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_FILTER_ACTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_EVENTING_FILTER_ACTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_EVENTING_FILTER_ACTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_FILTER_ACTION").field("Actions", &self.Actions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER_ACTION {
    fn eq(&self, other: &Self) -> bool {
        self.Actions == other.Actions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_EVENTING_FILTER_ACTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_EVENTING_FILTER_ACTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: ::core::option::Option<PWSD_SOAP_MESSAGE_HANDLER>,
    pub PVoid: *mut ::core::ffi::c_void,
    pub Unknown: ::core::option::Option<::windows::core::IUnknown>,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_HANDLER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_HANDLER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_HANDLER_CONTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_HANDLER_CONTEXT").field("PVoid", &self.PVoid).field("Unknown", &self.Unknown).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_HANDLER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Handler.map(|f| f as usize) == other.Handler.map(|f| f as usize) && self.PVoid == other.PVoid && self.Unknown == other.Unknown
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_HANDLER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_HANDLER_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_HEADER_RELATESTO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_HEADER_RELATESTO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_HEADER_RELATESTO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_HEADER_RELATESTO").field("RelationshipType", &self.RelationshipType).field("MessageID", &self.MessageID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_HEADER_RELATESTO {
    fn eq(&self, other: &Self) -> bool {
        self.RelationshipType == other.RelationshipType && self.MessageID == other.MessageID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_HEADER_RELATESTO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_HEADER_RELATESTO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_HELLO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_HELLO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_HELLO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_HELLO").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_HELLO {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_HELLO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_HELLO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_HOST_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_HOST_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_HOST_METADATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_HOST_METADATA").field("Host", &self.Host).field("Hosted", &self.Hosted).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_HOST_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Host == other.Host && self.Hosted == other.Hosted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_HOST_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_HOST_METADATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_LOCALIZED_STRING {
    pub lang: super::super::Foundation::PWSTR,
    pub String: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_LOCALIZED_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_LOCALIZED_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_LOCALIZED_STRING").field("lang", &self.lang).field("String", &self.String).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang && self.String == other.String
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_LOCALIZED_STRING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_LOCALIZED_STRING_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_LOCALIZED_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_LOCALIZED_STRING_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_LOCALIZED_STRING_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_METADATA_SECTION {
    pub Dialect: super::super::Foundation::PWSTR,
    pub Identifier: super::super::Foundation::PWSTR,
    pub Data: *mut ::core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: super::super::Foundation::PWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_METADATA_SECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_METADATA_SECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_METADATA_SECTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_METADATA_SECTION").field("Dialect", &self.Dialect).field("Identifier", &self.Identifier).field("Data", &self.Data).field("MetadataReference", &self.MetadataReference).field("Location", &self.Location).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION {
    fn eq(&self, other: &Self) -> bool {
        self.Dialect == other.Dialect && self.Identifier == other.Identifier && self.Data == other.Data && self.MetadataReference == other.MetadataReference && self.Location == other.Location && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_METADATA_SECTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_METADATA_SECTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_METADATA_SECTION_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_METADATA_SECTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_METADATA_SECTION_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_METADATA_SECTION_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_METADATA_SECTION_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_METADATA_SECTION_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_NAME_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_NAME_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_NAME_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_NAME_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_NAME_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_NAME_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_NAME_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: ::core::option::Option<WSD_STUB_FUNCTION>,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_OPERATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_OPERATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_OPERATION").field("RequestType", &self.RequestType).field("ResponseType", &self.ResponseType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        self.RequestType == other.RequestType && self.ResponseType == other.ResponseType && self.RequestStubFunction.map(|f| f as usize) == other.RequestStubFunction.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_OPERATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_OPERATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PORT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_PORT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_PORT_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_PORT_TYPE").field("EncodedName", &self.EncodedName).field("OperationCount", &self.OperationCount).field("Operations", &self.Operations).field("ProtocolType", &self.ProtocolType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_PORT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.EncodedName == other.EncodedName && self.OperationCount == other.OperationCount && self.Operations == other.Operations && self.ProtocolType == other.ProtocolType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_PORT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_PORT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PROBE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_PROBE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_PROBE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_PROBE").field("Types", &self.Types).field("Scopes", &self.Scopes).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_PROBE {
    fn eq(&self, other: &Self) -> bool {
        self.Types == other.Types && self.Scopes == other.Scopes && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_PROBE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_PROBE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PROBE_MATCH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_PROBE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_PROBE_MATCH {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_PROBE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_PROBE_MATCH {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_PROBE_MATCH {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PROBE_MATCHES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_PROBE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_PROBE_MATCHES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_PROBE_MATCHES").field("ProbeMatch", &self.ProbeMatch).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_PROBE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        self.ProbeMatch == other.ProbeMatch && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_PROBE_MATCHES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_PROBE_MATCHES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PROBE_MATCH_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_PROBE_MATCH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_PROBE_MATCH_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_PROBE_MATCH_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_PROBE_MATCH_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_PROBE_MATCH_LIST {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSD_PROTOCOL_TYPE(pub i32);
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(0i32);
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(1i32);
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(2i32);
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(4i32);
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(255i32);
impl ::core::convert::From<i32> for WSD_PROTOCOL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WSD_PROTOCOL_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_REFERENCE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_REFERENCE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_REFERENCE_PARAMETERS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_REFERENCE_PARAMETERS").field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_REFERENCE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_REFERENCE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_REFERENCE_PARAMETERS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_REFERENCE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_REFERENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_REFERENCE_PROPERTIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_REFERENCE_PROPERTIES").field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_REFERENCE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_REFERENCE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_REFERENCE_PROPERTIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: super::super::Foundation::PWSTR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_RELATIONSHIP_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_RELATIONSHIP_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_RELATIONSHIP_METADATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_RELATIONSHIP_METADATA").field("Type", &self.Type).field("Data", &self.Data).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_RELATIONSHIP_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Data == other.Data && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_RELATIONSHIP_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_RELATIONSHIP_METADATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_RESOLVE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_RESOLVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_RESOLVE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_RESOLVE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_RESOLVE {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_RESOLVE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_RESOLVE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_RESOLVE_MATCH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_RESOLVE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_RESOLVE_MATCH {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_RESOLVE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_RESOLVE_MATCH {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_RESOLVE_MATCH {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_RESOLVE_MATCHES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_RESOLVE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_RESOLVE_MATCHES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_RESOLVE_MATCHES").field("ResolveMatch", &self.ResolveMatch).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        self.ResolveMatch == other.ResolveMatch && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_RESOLVE_MATCHES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_RESOLVE_MATCHES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SCOPES {
    pub MatchBy: super::super::Foundation::PWSTR,
    pub Scopes: *mut WSD_URI_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SCOPES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SCOPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SCOPES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SCOPES").field("MatchBy", &self.MatchBy).field("Scopes", &self.Scopes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SCOPES {
    fn eq(&self, other: &Self) -> bool {
        self.MatchBy == other.MatchBy && self.Scopes == other.Scopes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SCOPES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SCOPES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: *mut ::core::ffi::c_void,
    pub hCertIssuerStore: *mut ::core::ffi::c_void,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: super::super::Foundation::PWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WSD_SECURITY_CERT_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SECURITY_CERT_VALIDATION")
            .field("certMatchArray", &self.certMatchArray)
            .field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount)
            .field("hCertMatchStore", &self.hCertMatchStore)
            .field("hCertIssuerStore", &self.hCertIssuerStore)
            .field("dwCertCheckOptions", &self.dwCertCheckOptions)
            .field("pszCNGHashAlgId", &self.pszCNGHashAlgId)
            .field("pbCertHash", &self.pbCertHash)
            .field("dwCertHashSize", &self.dwCertHashSize)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        self.certMatchArray == other.certMatchArray && self.dwCertMatchArrayCount == other.dwCertMatchArrayCount && self.hCertMatchStore == other.hCertMatchStore && self.hCertIssuerStore == other.hCertIssuerStore && self.dwCertCheckOptions == other.dwCertCheckOptions && self.pszCNGHashAlgId == other.pszCNGHashAlgId && self.pbCertHash == other.pbCertHash && self.dwCertHashSize == other.dwCertHashSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WSD_SECURITY_CERT_VALIDATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: *mut ::core::ffi::c_void,
    pub hCertIssuerStore: *mut ::core::ffi::c_void,
    pub dwCertCheckOptions: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SECURITY_CERT_VALIDATION_V1")
            .field("certMatchArray", &self.certMatchArray)
            .field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount)
            .field("hCertMatchStore", &self.hCertMatchStore)
            .field("hCertIssuerStore", &self.hCertIssuerStore)
            .field("dwCertCheckOptions", &self.dwCertCheckOptions)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.certMatchArray == other.certMatchArray && self.dwCertMatchArrayCount == other.dwCertMatchArrayCount && self.hCertMatchStore == other.hCertMatchStore && self.hCertIssuerStore == other.hCertIssuerStore && self.dwCertCheckOptions == other.dwCertCheckOptions
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WSD_SECURITY_CERT_VALIDATION_V1 {
    type Abi = Self;
}
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SECURITY_SIGNATURE_VALIDATION").field("signingCertArray", &self.signingCertArray).field("dwSigningCertArrayCount", &self.dwSigningCertArrayCount).field("hSigningCertStore", &self.hSigningCertStore).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        self.signingCertArray == other.signingCertArray && self.dwSigningCertArrayCount == other.dwSigningCertArrayCount && self.hSigningCertStore == other.hSigningCertStore && self.dwFlags == other.dwFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WSD_SECURITY_SIGNATURE_VALIDATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: super::super::Foundation::PWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SERVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SERVICE_METADATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SERVICE_METADATA").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("ServiceId", &self.ServiceId).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.ServiceId == other.ServiceId && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SERVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SERVICE_METADATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SERVICE_METADATA_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SERVICE_METADATA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SERVICE_METADATA_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SERVICE_METADATA_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SERVICE_METADATA_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SERVICE_METADATA_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: super::super::Foundation::PWSTR,
    pub Role: super::super::Foundation::PWSTR,
    pub Detail: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_FAULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SOAP_FAULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SOAP_FAULT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SOAP_FAULT").field("Code", &self.Code).field("Reason", &self.Reason).field("Node", &self.Node).field("Role", &self.Role).field("Detail", &self.Detail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT {
    fn eq(&self, other: &Self) -> bool {
        self.Code == other.Code && self.Reason == other.Reason && self.Node == other.Node && self.Role == other.Role && self.Detail == other.Detail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SOAP_FAULT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_FAULT_CODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SOAP_FAULT_CODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SOAP_FAULT_CODE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SOAP_FAULT_CODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_CODE {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Subcode == other.Subcode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SOAP_FAULT_CODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT_CODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_FAULT_REASON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SOAP_FAULT_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SOAP_FAULT_REASON {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SOAP_FAULT_REASON").field("Text", &self.Text).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_REASON {
    fn eq(&self, other: &Self) -> bool {
        self.Text == other.Text
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SOAP_FAULT_REASON {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT_REASON {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_FAULT_SUBCODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SOAP_FAULT_SUBCODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SOAP_FAULT_SUBCODE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SOAP_FAULT_SUBCODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_SUBCODE {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Subcode == other.Subcode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SOAP_FAULT_SUBCODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT_SUBCODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SOAP_HEADER {
    pub To: super::super::Foundation::PWSTR,
    pub Action: super::super::Foundation::PWSTR,
    pub MessageID: super::super::Foundation::PWSTR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SOAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SOAP_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SOAP_HEADER")
            .field("To", &self.To)
            .field("Action", &self.Action)
            .field("MessageID", &self.MessageID)
            .field("RelatesTo", &self.RelatesTo)
            .field("ReplyTo", &self.ReplyTo)
            .field("From", &self.From)
            .field("FaultTo", &self.FaultTo)
            .field("AppSequence", &self.AppSequence)
            .field("AnyHeaders", &self.AnyHeaders)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SOAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.To == other.To && self.Action == other.Action && self.MessageID == other.MessageID && self.RelatesTo == other.RelatesTo && self.ReplyTo == other.ReplyTo && self.From == other.From && self.FaultTo == other.FaultTo && self.AppSequence == other.AppSequence && self.AnyHeaders == other.AnyHeaders
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SOAP_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SOAP_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut ::core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SOAP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SOAP_MESSAGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SOAP_MESSAGE").field("Header", &self.Header).field("Body", &self.Body).field("BodyType", &self.BodyType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SOAP_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Body == other.Body && self.BodyType == other.BodyType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SOAP_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SOAP_MESSAGE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type WSD_STUB_FUNCTION = unsafe extern "system" fn(server: ::windows::core::RawPtr, session: ::windows::core::RawPtr, event: *mut ::core::mem::ManuallyDrop<WSD_EVENT>) -> ::windows::core::HRESULT;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: ::windows::core::HRESULT,
    pub eventHandle: super::super::Foundation::HANDLE,
    pub messageParameters: ::core::option::Option<IWSDMessageParameters>,
    pub results: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SYNCHRONOUS_RESPONSE_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_SYNCHRONOUS_RESPONSE_CONTEXT").field("hr", &self.hr).field("eventHandle", &self.eventHandle).field("messageParameters", &self.messageParameters).field("results", &self.results).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.eventHandle == other.eventHandle && self.messageParameters == other.messageParameters && self.results == other.results
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: super::super::Foundation::PWSTR,
    pub SerialNumber: super::super::Foundation::PWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_THIS_DEVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_THIS_DEVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_THIS_DEVICE_METADATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_THIS_DEVICE_METADATA").field("FriendlyName", &self.FriendlyName).field("FirmwareVersion", &self.FirmwareVersion).field("SerialNumber", &self.SerialNumber).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_THIS_DEVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.FriendlyName == other.FriendlyName && self.FirmwareVersion == other.FirmwareVersion && self.SerialNumber == other.SerialNumber && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_THIS_DEVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_THIS_DEVICE_METADATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: super::super::Foundation::PWSTR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: super::super::Foundation::PWSTR,
    pub ModelUrl: super::super::Foundation::PWSTR,
    pub PresentationUrl: super::super::Foundation::PWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_THIS_MODEL_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_THIS_MODEL_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_THIS_MODEL_METADATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_THIS_MODEL_METADATA")
            .field("Manufacturer", &self.Manufacturer)
            .field("ManufacturerUrl", &self.ManufacturerUrl)
            .field("ModelName", &self.ModelName)
            .field("ModelNumber", &self.ModelNumber)
            .field("ModelUrl", &self.ModelUrl)
            .field("PresentationUrl", &self.PresentationUrl)
            .field("Any", &self.Any)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_THIS_MODEL_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Manufacturer == other.Manufacturer && self.ManufacturerUrl == other.ManufacturerUrl && self.ModelName == other.ModelName && self.ModelNumber == other.ModelNumber && self.ModelUrl == other.ModelUrl && self.PresentationUrl == other.PresentationUrl && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_THIS_MODEL_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_THIS_MODEL_METADATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_UNKNOWN_LOOKUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_UNKNOWN_LOOKUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_UNKNOWN_LOOKUP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_UNKNOWN_LOOKUP").field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_UNKNOWN_LOOKUP {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_UNKNOWN_LOOKUP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_UNKNOWN_LOOKUP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_URI_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_URI_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_URI_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSD_URI_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_URI_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_URI_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_URI_LIST {
    type Abi = Self;
}
