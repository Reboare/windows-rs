#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInstalledVoicesStatic {
    type Vtable = IInstalledVoicesStatic_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d526ecc_7533_4c3f_85be_888c2baeebdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInstalledVoicesStatic2 {
    type Vtable = IInstalledVoicesStatic2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64255f2e_358d_4058_be9a_fd3fcb423530);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, voice: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechSynthesisStream(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpeechSynthesisStream {
    type Vtable = ISpeechSynthesisStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83e46e93_244c_4622_ba0b_6229c4d0d65d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesisStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechSynthesizer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpeechSynthesizer {
    type Vtable = ISpeechSynthesizer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce9f7c76_97f4_4ced_ad68_d51c458e45c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ssml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechSynthesizer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpeechSynthesizer2 {
    type Vtable = ISpeechSynthesizer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7c5ecb2_4339_4d6a_bbf8_c7a4f1544c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpeechSynthesizerOptions {
    type Vtable = ISpeechSynthesizerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e23871_cc3d_43c9_91b1_ee185324d83d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpeechSynthesizerOptions2 {
    type Vtable = ISpeechSynthesizerOptions2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cbef60e_119c_4bed_b118_d250c3a25793);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpeechSynthesizerOptions3 {
    type Vtable = ISpeechSynthesizerOptions3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x401ed877_902c_4814_a582_a5d0c0769fa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SpeechAppendedSilence) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: SpeechAppendedSilence) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SpeechPunctuationSilence) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: SpeechPunctuationSilence) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceInformation {
    type Vtable = IVoiceInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb127d6a4_1291_4604_aa9c_83134083352c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VoiceGender) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechAppendedSilence(pub i32);
impl SpeechAppendedSilence {
    pub const Default: SpeechAppendedSilence = SpeechAppendedSilence(0i32);
    pub const Min: SpeechAppendedSilence = SpeechAppendedSilence(1i32);
}
impl ::core::convert::From<i32> for SpeechAppendedSilence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpeechAppendedSilence {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SpeechAppendedSilence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.SpeechAppendedSilence;i4)");
}
impl ::windows::core::DefaultType for SpeechAppendedSilence {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechPunctuationSilence(pub i32);
impl SpeechPunctuationSilence {
    pub const Default: SpeechPunctuationSilence = SpeechPunctuationSilence(0i32);
    pub const Min: SpeechPunctuationSilence = SpeechPunctuationSilence(1i32);
}
impl ::core::convert::From<i32> for SpeechPunctuationSilence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpeechPunctuationSilence {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SpeechPunctuationSilence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.SpeechPunctuationSilence;i4)");
}
impl ::windows::core::DefaultType for SpeechPunctuationSilence {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpeechSynthesisStream(pub ::windows::core::IInspectable);
impl SpeechSynthesisStream {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Markers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::IMediaMarker>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::IMediaMarker>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn TimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> {
        let this = &::windows::core::Interface::cast::<super::Core::ITimedMetadataTrackProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechSynthesisStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.SpeechSynthesisStream;{83e46e93-244c-4622-ba0b-6229c4d0d65d})");
}
unsafe impl ::windows::core::Interface for SpeechSynthesisStream {
    type Vtable = ISpeechSynthesisStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83e46e93_244c_4622_ba0b_6229c4d0d65d);
}
impl ::windows::core::RuntimeName for SpeechSynthesisStream {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesisStream";
}
impl ::core::convert::From<SpeechSynthesisStream> for ::windows::core::IUnknown {
    fn from(value: SpeechSynthesisStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpeechSynthesisStream> for ::windows::core::IUnknown {
    fn from(value: &SpeechSynthesisStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpeechSynthesisStream> for ::windows::core::IInspectable {
    fn from(value: SpeechSynthesisStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpeechSynthesisStream> for ::windows::core::IInspectable {
    fn from(value: &SpeechSynthesisStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IContentTypeProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Media_Core")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for super::Core::ITimedMetadataTrackProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Core")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for super::Core::ITimedMetadataTrackProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Core")]
impl<'a> ::windows::core::IntoParam<'a, super::Core::ITimedMetadataTrackProvider> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::Core::ITimedMetadataTrackProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Core")]
impl<'a> ::windows::core::IntoParam<'a, super::Core::ITimedMetadataTrackProvider> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::Core::ITimedMetadataTrackProvider> {
        ::core::convert::TryInto::<super::Core::ITimedMetadataTrackProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechSynthesisStream {}
unsafe impl ::core::marker::Sync for SpeechSynthesisStream {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpeechSynthesizer(pub ::windows::core::IInspectable);
impl SpeechSynthesizer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechSynthesizer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn SynthesizeTextToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SynthesizeSsmlToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, ssml: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ssml.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>(result__)
        }
    }
    pub fn SetVoice<'a, Param0: ::windows::core::IntoParam<'a, VoiceInformation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Voice(&self) -> ::windows::core::Result<VoiceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllVoices() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VoiceInformation>> {
        Self::IInstalledVoicesStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<VoiceInformation>>(result__)
        })
    }
    pub fn DefaultVoice() -> ::windows::core::Result<VoiceInformation> {
        Self::IInstalledVoicesStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceInformation>(result__)
        })
    }
    pub fn Options(&self) -> ::windows::core::Result<SpeechSynthesizerOptions> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizer2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechSynthesizerOptions>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrySetDefaultVoiceAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceInformation>>(voice: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IInstalledVoicesStatic2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), voice.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IInstalledVoicesStatic<R, F: FnOnce(&IInstalledVoicesStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechSynthesizer, IInstalledVoicesStatic> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInstalledVoicesStatic2<R, F: FnOnce(&IInstalledVoicesStatic2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechSynthesizer, IInstalledVoicesStatic2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechSynthesizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.SpeechSynthesizer;{ce9f7c76-97f4-4ced-ad68-d51c458e45c6})");
}
unsafe impl ::windows::core::Interface for SpeechSynthesizer {
    type Vtable = ISpeechSynthesizer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce9f7c76_97f4_4ced_ad68_d51c458e45c6);
}
impl ::windows::core::RuntimeName for SpeechSynthesizer {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesizer";
}
impl ::core::convert::From<SpeechSynthesizer> for ::windows::core::IUnknown {
    fn from(value: SpeechSynthesizer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpeechSynthesizer> for ::windows::core::IUnknown {
    fn from(value: &SpeechSynthesizer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpeechSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpeechSynthesizer> for ::windows::core::IInspectable {
    fn from(value: SpeechSynthesizer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpeechSynthesizer> for ::windows::core::IInspectable {
    fn from(value: &SpeechSynthesizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpeechSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SpeechSynthesizer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechSynthesizer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SpeechSynthesizer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechSynthesizer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for SpeechSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &SpeechSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechSynthesizer {}
unsafe impl ::core::marker::Sync for SpeechSynthesizer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpeechSynthesizerOptions(pub ::windows::core::IInspectable);
impl SpeechSynthesizerOptions {
    pub fn IncludeWordBoundaryMetadata(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeWordBoundaryMetadata(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IncludeSentenceBoundaryMetadata(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeSentenceBoundaryMetadata(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AudioVolume(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetAudioVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SpeakingRate(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetSpeakingRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AudioPitch(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetAudioPitch(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AppendedSilence(&self) -> ::windows::core::Result<SpeechAppendedSilence> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe {
            let mut result__: SpeechAppendedSilence = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechAppendedSilence>(result__)
        }
    }
    pub fn SetAppendedSilence(&self, value: SpeechAppendedSilence) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PunctuationSilence(&self) -> ::windows::core::Result<SpeechPunctuationSilence> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe {
            let mut result__: SpeechPunctuationSilence = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechPunctuationSilence>(result__)
        }
    }
    pub fn SetPunctuationSilence(&self, value: SpeechPunctuationSilence) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechSynthesizerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions;{a0e23871-cc3d-43c9-91b1-ee185324d83d})");
}
unsafe impl ::windows::core::Interface for SpeechSynthesizerOptions {
    type Vtable = ISpeechSynthesizerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e23871_cc3d_43c9_91b1_ee185324d83d);
}
impl ::windows::core::RuntimeName for SpeechSynthesizerOptions {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions";
}
impl ::core::convert::From<SpeechSynthesizerOptions> for ::windows::core::IUnknown {
    fn from(value: SpeechSynthesizerOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpeechSynthesizerOptions> for ::windows::core::IUnknown {
    fn from(value: &SpeechSynthesizerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechSynthesizerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpeechSynthesizerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpeechSynthesizerOptions> for ::windows::core::IInspectable {
    fn from(value: SpeechSynthesizerOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpeechSynthesizerOptions> for ::windows::core::IInspectable {
    fn from(value: &SpeechSynthesizerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechSynthesizerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpeechSynthesizerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpeechSynthesizerOptions {}
unsafe impl ::core::marker::Sync for SpeechSynthesizerOptions {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoiceGender(pub i32);
impl VoiceGender {
    pub const Male: VoiceGender = VoiceGender(0i32);
    pub const Female: VoiceGender = VoiceGender(1i32);
}
impl ::core::convert::From<i32> for VoiceGender {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VoiceGender {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VoiceGender {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.VoiceGender;i4)");
}
impl ::windows::core::DefaultType for VoiceGender {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceInformation(pub ::windows::core::IInspectable);
impl VoiceInformation {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Gender(&self) -> ::windows::core::Result<VoiceGender> {
        let this = self;
        unsafe {
            let mut result__: VoiceGender = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceGender>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.VoiceInformation;{b127d6a4-1291-4604-aa9c-83134083352c})");
}
unsafe impl ::windows::core::Interface for VoiceInformation {
    type Vtable = IVoiceInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb127d6a4_1291_4604_aa9c_83134083352c);
}
impl ::windows::core::RuntimeName for VoiceInformation {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.VoiceInformation";
}
impl ::core::convert::From<VoiceInformation> for ::windows::core::IUnknown {
    fn from(value: VoiceInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceInformation> for ::windows::core::IUnknown {
    fn from(value: &VoiceInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceInformation> for ::windows::core::IInspectable {
    fn from(value: VoiceInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceInformation> for ::windows::core::IInspectable {
    fn from(value: &VoiceInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceInformation {}
unsafe impl ::core::marker::Sync for VoiceInformation {}
