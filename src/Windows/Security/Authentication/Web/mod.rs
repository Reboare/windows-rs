#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerStatics {
    type Vtable = IWebAuthenticationBrokerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f149f1a_e673_40b5_bc22_201a6864a37b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: WebAuthenticationOptions, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: WebAuthenticationOptions, requesturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerStatics2 {
    type Vtable = IWebAuthenticationBrokerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73cdfb9e_14e7_41da_a971_aaf4410b621e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requesturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr, continuationdata: ::windows::core::RawPtr, options: WebAuthenticationOptions) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requesturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requesturi: ::windows::core::RawPtr, options: WebAuthenticationOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWebAuthenticationResult {
    type Vtable = IWebAuthenticationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64002b4b_ede9_470a_a5cd_0323faf6e262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WebAuthenticationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TokenBindingKeyType(pub i32);
impl TokenBindingKeyType {
    pub const Rsa2048: TokenBindingKeyType = TokenBindingKeyType(0i32);
    pub const EcdsaP256: TokenBindingKeyType = TokenBindingKeyType(1i32);
    pub const AnyExisting: TokenBindingKeyType = TokenBindingKeyType(2i32);
}
impl ::core::convert::From<i32> for TokenBindingKeyType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TokenBindingKeyType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TokenBindingKeyType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.TokenBindingKeyType;i4)");
}
impl ::windows::core::DefaultType for TokenBindingKeyType {
    type DefaultType = Self;
}
pub struct WebAuthenticationBroker {}
impl WebAuthenticationBroker {
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateWithCallbackUriAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(options: WebAuthenticationOptions, requesturi: Param1, callbackuri: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), options, requesturi.into_param().abi(), callbackuri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateWithoutCallbackUriAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(options: WebAuthenticationOptions, requesturi: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), options, requesturi.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentApplicationCallbackUri() -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateAndContinue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0) -> ::windows::core::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), requesturi.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateWithCallbackUriAndContinue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0, callbackuri: Param1) -> ::windows::core::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), requesturi.into_param().abi(), callbackuri.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::ValueSet>>(requesturi: Param0, callbackuri: Param1, continuationdata: Param2, options: WebAuthenticationOptions) -> ::windows::core::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), requesturi.into_param().abi(), callbackuri.into_param().abi(), continuationdata.into_param().abi(), options).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateSilentlyAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), requesturi.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateSilentlyWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0, options: WebAuthenticationOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), requesturi.into_param().abi(), options, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    pub fn IWebAuthenticationBrokerStatics<R, F: FnOnce(&IWebAuthenticationBrokerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAuthenticationBroker, IWebAuthenticationBrokerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebAuthenticationBrokerStatics2<R, F: FnOnce(&IWebAuthenticationBrokerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAuthenticationBroker, IWebAuthenticationBrokerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WebAuthenticationBroker {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationBroker";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebAuthenticationOptions(pub u32);
impl WebAuthenticationOptions {
    pub const None: WebAuthenticationOptions = WebAuthenticationOptions(0u32);
    pub const SilentMode: WebAuthenticationOptions = WebAuthenticationOptions(1u32);
    pub const UseTitle: WebAuthenticationOptions = WebAuthenticationOptions(2u32);
    pub const UseHttpPost: WebAuthenticationOptions = WebAuthenticationOptions(4u32);
    pub const UseCorporateNetwork: WebAuthenticationOptions = WebAuthenticationOptions(8u32);
}
impl ::core::convert::From<u32> for WebAuthenticationOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WebAuthenticationOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WebAuthenticationOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationOptions;u4)");
}
impl ::windows::core::DefaultType for WebAuthenticationOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for WebAuthenticationOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for WebAuthenticationOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for WebAuthenticationOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for WebAuthenticationOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for WebAuthenticationOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WebAuthenticationResult(pub ::windows::core::IInspectable);
impl WebAuthenticationResult {
    pub fn ResponseData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ResponseStatus(&self) -> ::windows::core::Result<WebAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: WebAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAuthenticationStatus>(result__)
        }
    }
    pub fn ResponseErrorDetail(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WebAuthenticationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.WebAuthenticationResult;{64002b4b-ede9-470a-a5cd-0323faf6e262})");
}
unsafe impl ::windows::core::Interface for WebAuthenticationResult {
    type Vtable = IWebAuthenticationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64002b4b_ede9_470a_a5cd_0323faf6e262);
}
impl ::windows::core::RuntimeName for WebAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationResult";
}
impl ::core::convert::From<WebAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: WebAuthenticationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WebAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: &WebAuthenticationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WebAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: WebAuthenticationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WebAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: &WebAuthenticationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebAuthenticationStatus(pub i32);
impl WebAuthenticationStatus {
    pub const Success: WebAuthenticationStatus = WebAuthenticationStatus(0i32);
    pub const UserCancel: WebAuthenticationStatus = WebAuthenticationStatus(1i32);
    pub const ErrorHttp: WebAuthenticationStatus = WebAuthenticationStatus(2i32);
}
impl ::core::convert::From<i32> for WebAuthenticationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WebAuthenticationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WebAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationStatus;i4)");
}
impl ::windows::core::DefaultType for WebAuthenticationStatus {
    type DefaultType = Self;
}
