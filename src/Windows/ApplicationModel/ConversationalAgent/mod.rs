#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ActivationSignalDetectionConfiguration(pub ::windows::core::IInspectable);
impl ActivationSignalDetectionConfiguration {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetEnabledAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn AvailabilityInfo(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DetectionConfigurationAvailabilityInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AvailabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAvailabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetModelDataAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetModelDataType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetModelDataTypeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetModelData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>>(result__)
        }
    }
    pub fn ClearModelData(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ClearModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn TrainingStepsCompleted(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn TrainingStepsRemaining(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn TrainingDataFormat(&self) -> ::windows::core::Result<ActivationSignalDetectionTrainingDataFormat> {
        let this = self;
        unsafe {
            let mut result__: ActivationSignalDetectionTrainingDataFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectionTrainingDataFormat>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ApplyTrainingData<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: Param1) -> ::windows::core::Result<DetectionConfigurationTrainingStatus> {
        let this = self;
        unsafe {
            let mut result__: DetectionConfigurationTrainingStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), trainingdataformat, trainingdata.into_param().abi(), &mut result__).from_abi::<DetectionConfigurationTrainingStatus>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ApplyTrainingDataAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), trainingdataformat, trainingdata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>>(result__)
        }
    }
    pub fn ClearTrainingData(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ClearTrainingDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelDataWithResult<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::core::Result<ActivationSignalDetectionConfigurationSetModelDataResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationSetModelDataResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfigurationSetModelDataResult>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetModelDataWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetEnabledWithResultAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>>(result__)
        }
    }
    pub fn SetEnabledWithResult(&self, value: bool) -> ::windows::core::Result<ActivationSignalDetectionConfigurationStateChangeResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationStateChangeResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<ActivationSignalDetectionConfigurationStateChangeResult>(result__)
        }
    }
    pub fn TrainingStepCompletionMaxAllowedTime(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration;{40d8be16-5217-581c-9ab2-ce9b2f2e8e00})");
}
unsafe impl ::windows::core::Interface for ActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40d8be16_5217_581c_9ab2_ce9b2f2e8e00);
}
impl ::windows::core::RuntimeName for ActivationSignalDetectionConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration";
}
impl ::core::convert::From<ActivationSignalDetectionConfiguration> for ::windows::core::IUnknown {
    fn from(value: ActivationSignalDetectionConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfiguration> for ::windows::core::IUnknown {
    fn from(value: &ActivationSignalDetectionConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ActivationSignalDetectionConfiguration> for ::windows::core::IInspectable {
    fn from(value: ActivationSignalDetectionConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfiguration> for ::windows::core::IInspectable {
    fn from(value: &ActivationSignalDetectionConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ActivationSignalDetectionConfiguration> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ActivationSignalDetectionConfiguration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ActivationSignalDetectionConfiguration> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ActivationSignalDetectionConfiguration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetectionConfiguration {}
unsafe impl ::core::marker::Sync for ActivationSignalDetectionConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ActivationSignalDetectionConfigurationCreationResult(pub ::windows::core::IInspectable);
impl ActivationSignalDetectionConfigurationCreationResult {
    pub fn Status(&self) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectionConfigurationCreationStatus>(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows::core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectionConfiguration>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationCreationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult;{4c89bc1b-8d12-5e48-a71c-7f6bc1cd66e0})");
}
unsafe impl ::windows::core::Interface for ActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c89bc1b_8d12_5e48_a71c_7f6bc1cd66e0);
}
impl ::windows::core::RuntimeName for ActivationSignalDetectionConfigurationCreationResult {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult";
}
impl ::core::convert::From<ActivationSignalDetectionConfigurationCreationResult> for ::windows::core::IUnknown {
    fn from(value: ActivationSignalDetectionConfigurationCreationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfigurationCreationResult> for ::windows::core::IUnknown {
    fn from(value: &ActivationSignalDetectionConfigurationCreationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ActivationSignalDetectionConfigurationCreationResult> for ::windows::core::IInspectable {
    fn from(value: ActivationSignalDetectionConfigurationCreationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfigurationCreationResult> for ::windows::core::IInspectable {
    fn from(value: &ActivationSignalDetectionConfigurationCreationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetectionConfigurationCreationResult {}
unsafe impl ::core::marker::Sync for ActivationSignalDetectionConfigurationCreationResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationCreationStatus(pub i32);
impl ActivationSignalDetectionConfigurationCreationStatus {
    pub const Success: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(0i32);
    pub const SignalIdNotAvailable: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(1i32);
    pub const ModelIdNotSupported: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(2i32);
    pub const InvalidSignalId: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(3i32);
    pub const InvalidModelId: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(4i32);
    pub const InvalidDisplayName: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(5i32);
    pub const ConfigurationAlreadyExists: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(6i32);
    pub const CreationNotSupported: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(7i32);
}
impl ::core::convert::From<i32> for ActivationSignalDetectionConfigurationCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationCreationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationStatus;i4)");
}
impl ::windows::core::DefaultType for ActivationSignalDetectionConfigurationCreationStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationRemovalResult(pub i32);
impl ActivationSignalDetectionConfigurationRemovalResult {
    pub const Success: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(0i32);
    pub const NotFound: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(1i32);
    pub const CurrentlyEnabled: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(2i32);
    pub const RemovalNotSupported: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(3i32);
}
impl ::core::convert::From<i32> for ActivationSignalDetectionConfigurationRemovalResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationRemovalResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationRemovalResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult;i4)");
}
impl ::windows::core::DefaultType for ActivationSignalDetectionConfigurationRemovalResult {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationSetModelDataResult(pub i32);
impl ActivationSignalDetectionConfigurationSetModelDataResult {
    pub const Success: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(0i32);
    pub const EmptyModelData: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(1i32);
    pub const UnsupportedFormat: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(2i32);
    pub const ConfigurationCurrentlyEnabled: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(3i32);
    pub const InvalidData: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(4i32);
    pub const SetModelDataNotSupported: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(5i32);
    pub const ConfigurationNotFound: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(6i32);
    pub const UnknownError: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(7i32);
}
impl ::core::convert::From<i32> for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationSetModelDataResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationSetModelDataResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult;i4)");
}
impl ::windows::core::DefaultType for ActivationSignalDetectionConfigurationSetModelDataResult {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationStateChangeResult(pub i32);
impl ActivationSignalDetectionConfigurationStateChangeResult {
    pub const Success: ActivationSignalDetectionConfigurationStateChangeResult = ActivationSignalDetectionConfigurationStateChangeResult(0i32);
    pub const NoModelData: ActivationSignalDetectionConfigurationStateChangeResult = ActivationSignalDetectionConfigurationStateChangeResult(1i32);
    pub const ConfigurationNotFound: ActivationSignalDetectionConfigurationStateChangeResult = ActivationSignalDetectionConfigurationStateChangeResult(2i32);
}
impl ::core::convert::From<i32> for ActivationSignalDetectionConfigurationStateChangeResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationStateChangeResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationStateChangeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult;i4)");
}
impl ::windows::core::DefaultType for ActivationSignalDetectionConfigurationStateChangeResult {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionTrainingDataFormat(pub i32);
impl ActivationSignalDetectionTrainingDataFormat {
    pub const Voice8kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(0i32);
    pub const Voice8kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(1i32);
    pub const Voice16kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(2i32);
    pub const Voice16kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(3i32);
    pub const VoiceOEMDefined: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(4i32);
    pub const Audio44kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(5i32);
    pub const Audio44kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(6i32);
    pub const Audio48kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(7i32);
    pub const Audio48kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(8i32);
    pub const AudioOEMDefined: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(9i32);
    pub const OtherOEMDefined: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(10i32);
}
impl ::core::convert::From<i32> for ActivationSignalDetectionTrainingDataFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionTrainingDataFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionTrainingDataFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat;i4)");
}
impl ::windows::core::DefaultType for ActivationSignalDetectionTrainingDataFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ActivationSignalDetector(pub ::windows::core::IInspectable);
impl ActivationSignalDetector {
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind> {
        let this = self;
        unsafe {
            let mut result__: ActivationSignalDetectorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectorKind>(result__)
        }
    }
    pub fn CanCreateConfigurations(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModelDataTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTrainingDataFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPowerStates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedModelIdsForSignalId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSupportedModelIdsForSignalIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    pub fn CreateConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConfigurations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetConfigurationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>>(result__)
        }
    }
    pub fn GetConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfiguration>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>>(result__)
        }
    }
    pub fn RemoveConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetAvailableModelIdsForSignalIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAvailableModelIdsForSignalId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateConfigurationWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>>(result__)
        }
    }
    pub fn CreateConfigurationWithResult<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfigurationCreationResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>>(result__)
        }
    }
    pub fn RemoveConfigurationWithResult<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<ActivationSignalDetectionConfigurationRemovalResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationRemovalResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfigurationRemovalResult>(result__)
        }
    }
    pub fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector;{b5bf345f-a4d0-5b2b-8e65-b3c55ee756ff})");
}
unsafe impl ::windows::core::Interface for ActivationSignalDetector {
    type Vtable = IActivationSignalDetector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5bf345f_a4d0_5b2b_8e65_b3c55ee756ff);
}
impl ::windows::core::RuntimeName for ActivationSignalDetector {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector";
}
impl ::core::convert::From<ActivationSignalDetector> for ::windows::core::IUnknown {
    fn from(value: ActivationSignalDetector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ActivationSignalDetector> for ::windows::core::IUnknown {
    fn from(value: &ActivationSignalDetector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivationSignalDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivationSignalDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ActivationSignalDetector> for ::windows::core::IInspectable {
    fn from(value: ActivationSignalDetector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ActivationSignalDetector> for ::windows::core::IInspectable {
    fn from(value: &ActivationSignalDetector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivationSignalDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivationSignalDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetector {}
unsafe impl ::core::marker::Sync for ActivationSignalDetector {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectorKind(pub i32);
impl ActivationSignalDetectorKind {
    pub const AudioPattern: ActivationSignalDetectorKind = ActivationSignalDetectorKind(0i32);
    pub const AudioImpulse: ActivationSignalDetectorKind = ActivationSignalDetectorKind(1i32);
    pub const HardwareEvent: ActivationSignalDetectorKind = ActivationSignalDetectorKind(2i32);
}
impl ::core::convert::From<i32> for ActivationSignalDetectorKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectorKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorKind;i4)");
}
impl ::windows::core::DefaultType for ActivationSignalDetectorKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectorPowerState(pub i32);
impl ActivationSignalDetectorPowerState {
    pub const HighPower: ActivationSignalDetectorPowerState = ActivationSignalDetectorPowerState(0i32);
    pub const ConnectedLowPower: ActivationSignalDetectorPowerState = ActivationSignalDetectorPowerState(1i32);
    pub const DisconnectedLowPower: ActivationSignalDetectorPowerState = ActivationSignalDetectorPowerState(2i32);
}
impl ::core::convert::From<i32> for ActivationSignalDetectorPowerState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectorPowerState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectorPowerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState;i4)");
}
impl ::windows::core::DefaultType for ActivationSignalDetectorPowerState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentActivationKind(pub i32);
impl ConversationalAgentActivationKind {
    pub const VoiceActivationPreview: ConversationalAgentActivationKind = ConversationalAgentActivationKind(0i32);
    pub const Foreground: ConversationalAgentActivationKind = ConversationalAgentActivationKind(1i32);
}
impl ::core::convert::From<i32> for ConversationalAgentActivationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentActivationKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentActivationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationKind;i4)");
}
impl ::windows::core::DefaultType for ConversationalAgentActivationKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentActivationResult(pub i32);
impl ConversationalAgentActivationResult {
    pub const Success: ConversationalAgentActivationResult = ConversationalAgentActivationResult(0i32);
    pub const AgentInactive: ConversationalAgentActivationResult = ConversationalAgentActivationResult(1i32);
    pub const ScreenNotAvailable: ConversationalAgentActivationResult = ConversationalAgentActivationResult(2i32);
    pub const AgentInterrupted: ConversationalAgentActivationResult = ConversationalAgentActivationResult(3i32);
}
impl ::core::convert::From<i32> for ConversationalAgentActivationResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentActivationResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentActivationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult;i4)");
}
impl ::windows::core::DefaultType for ConversationalAgentActivationResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConversationalAgentDetectorManager(pub ::windows::core::IInspectable);
impl ConversationalAgentDetectorManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllActivationSignalDetectors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetAllActivationSignalDetectorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActivationSignalDetectors(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetActivationSignalDetectorsAsync(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>(result__)
        }
    }
    pub fn Default() -> ::windows::core::Result<ConversationalAgentDetectorManager> {
        Self::IConversationalAgentDetectorManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentDetectorManager>(result__)
        })
    }
    pub fn GetActivationSignalDetectorFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, detectorid: Param0) -> ::windows::core::Result<ActivationSignalDetector> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), detectorid.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetector>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetActivationSignalDetectorFromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, detectorid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), detectorid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>>(result__)
        }
    }
    pub fn IConversationalAgentDetectorManagerStatics<R, F: FnOnce(&IConversationalAgentDetectorManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConversationalAgentDetectorManager, IConversationalAgentDetectorManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentDetectorManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager;{de94fbb0-597a-5df8-8cfb-9dbb583ba3ff})");
}
unsafe impl ::windows::core::Interface for ConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde94fbb0_597a_5df8_8cfb_9dbb583ba3ff);
}
impl ::windows::core::RuntimeName for ConversationalAgentDetectorManager {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager";
}
impl ::core::convert::From<ConversationalAgentDetectorManager> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentDetectorManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConversationalAgentDetectorManager> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentDetectorManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConversationalAgentDetectorManager> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentDetectorManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConversationalAgentDetectorManager> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentDetectorManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentDetectorManager {}
unsafe impl ::core::marker::Sync for ConversationalAgentDetectorManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConversationalAgentSession(pub ::windows::core::IInspectable);
impl ConversationalAgentSession {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SessionInterrupted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionInterrupted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SignalDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSignalDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SystemStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn AgentState(&self) -> ::windows::core::Result<ConversationalAgentState> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentState>(result__)
        }
    }
    pub fn Signal(&self) -> ::windows::core::Result<ConversationalAgentSignal> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSignal>(result__)
        }
    }
    pub fn IsIndicatorLightAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsScreenAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsUserAuthenticated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsVoiceActivationAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsInterruptible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsInterrupted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestInterruptibleAsync(&self, interruptible: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), interruptible, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    pub fn RequestInterruptible(&self, interruptible: bool) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSessionUpdateResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), interruptible, &mut result__).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAgentStateChangeAsync(&self, state: ConversationalAgentState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), state, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    pub fn RequestAgentStateChange(&self, state: ConversationalAgentState) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSessionUpdateResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), state, &mut result__).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestForegroundActivationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    pub fn RequestForegroundActivation(&self) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSessionUpdateResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetAudioClientAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>(result__)
        }
    }
    pub fn GetAudioClient(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Audio"))]
    pub fn CreateAudioDeviceInputNodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Audio::AudioGraph>>(&self, graph: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), graph.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>>(result__)
        }
    }
    #[cfg(feature = "Media_Audio")]
    pub fn CreateAudioDeviceInputNode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Audio::AudioGraph>>(&self, graph: Param0) -> ::windows::core::Result<super::super::Media::Audio::AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), graph.into_param().abi(), &mut result__).from_abi::<super::super::Media::Audio::AudioDeviceInputNode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetAudioCaptureDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn GetAudioCaptureDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetAudioRenderDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn GetAudioRenderDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSignalModelIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn GetSignalModelId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSignalModelIdAsync(&self, signalmodelid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), signalmodelid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn SetSignalModelId(&self, signalmodelid: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), signalmodelid, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSupportedSignalModelIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSignalModelIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentSessionAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>>(result__)
        })
    }
    pub fn GetCurrentSessionSync() -> ::windows::core::Result<ConversationalAgentSession> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSession>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestActivationAsync(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activationkind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>>(result__)
        }
    }
    pub fn RequestActivation(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<ConversationalAgentActivationResult> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ConversationalAgentActivationResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activationkind, &mut result__).from_abi::<ConversationalAgentActivationResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSupportLockScreenActivationAsync(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), lockscreenactivationsupported, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetSupportLockScreenActivation(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), lockscreenactivationsupported).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMissingPrerequisites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetMissingPrerequisitesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>>(result__)
        }
    }
    pub fn IConversationalAgentSessionStatics<R, F: FnOnce(&IConversationalAgentSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConversationalAgentSession, IConversationalAgentSessionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession;{daaae09a-b7ba-57e5-ad13-df520f9b6fa7})");
}
unsafe impl ::windows::core::Interface for ConversationalAgentSession {
    type Vtable = IConversationalAgentSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaaae09a_b7ba_57e5_ad13_df520f9b6fa7);
}
impl ::windows::core::RuntimeName for ConversationalAgentSession {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession";
}
impl ::core::convert::From<ConversationalAgentSession> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConversationalAgentSession> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConversationalAgentSession> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConversationalAgentSession> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ConversationalAgentSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ConversationalAgentSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ConversationalAgentSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ConversationalAgentSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSession {}
unsafe impl ::core::marker::Sync for ConversationalAgentSession {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConversationalAgentSessionInterruptedEventArgs(pub ::windows::core::IInspectable);
impl ConversationalAgentSessionInterruptedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSessionInterruptedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs;{9766591f-f63d-5d3e-9bf2-bd0760552686})");
}
unsafe impl ::windows::core::Interface for ConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9766591f_f63d_5d3e_9bf2_bd0760552686);
}
impl ::windows::core::RuntimeName for ConversationalAgentSessionInterruptedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs";
}
impl ::core::convert::From<ConversationalAgentSessionInterruptedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSessionInterruptedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConversationalAgentSessionInterruptedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSessionInterruptedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConversationalAgentSessionInterruptedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSessionInterruptedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConversationalAgentSessionInterruptedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSessionInterruptedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSessionInterruptedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSessionInterruptedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentSessionUpdateResponse(pub i32);
impl ConversationalAgentSessionUpdateResponse {
    pub const Success: ConversationalAgentSessionUpdateResponse = ConversationalAgentSessionUpdateResponse(0i32);
    pub const Failed: ConversationalAgentSessionUpdateResponse = ConversationalAgentSessionUpdateResponse(1i32);
}
impl ::core::convert::From<i32> for ConversationalAgentSessionUpdateResponse {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentSessionUpdateResponse {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSessionUpdateResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse;i4)");
}
impl ::windows::core::DefaultType for ConversationalAgentSessionUpdateResponse {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConversationalAgentSignal(pub ::windows::core::IInspectable);
impl ConversationalAgentSignal {
    pub fn IsSignalVerificationRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSignalVerificationRequired(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSignalId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SignalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSignalName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SignalContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetSignalContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SignalStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSignalStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SignalEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSignalEnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DetectorKind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectorKind>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSignal {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal;{20ed25f7-b120-51f2-8603-265d6a47f232})");
}
unsafe impl ::windows::core::Interface for ConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20ed25f7_b120_51f2_8603_265d6a47f232);
}
impl ::windows::core::RuntimeName for ConversationalAgentSignal {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal";
}
impl ::core::convert::From<ConversationalAgentSignal> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSignal) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConversationalAgentSignal> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSignal) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConversationalAgentSignal> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSignal) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConversationalAgentSignal> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSignal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSignal {}
unsafe impl ::core::marker::Sync for ConversationalAgentSignal {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConversationalAgentSignalDetectedEventArgs(pub ::windows::core::IInspectable);
impl ConversationalAgentSignalDetectedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSignalDetectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs;{4d57eb8f-f88a-599b-91d3-d604876708bc})");
}
unsafe impl ::windows::core::Interface for ConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d57eb8f_f88a_599b_91d3_d604876708bc);
}
impl ::windows::core::RuntimeName for ConversationalAgentSignalDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs";
}
impl ::core::convert::From<ConversationalAgentSignalDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSignalDetectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConversationalAgentSignalDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSignalDetectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConversationalAgentSignalDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSignalDetectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConversationalAgentSignalDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSignalDetectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSignalDetectedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSignalDetectedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentState(pub i32);
impl ConversationalAgentState {
    pub const Inactive: ConversationalAgentState = ConversationalAgentState(0i32);
    pub const Detecting: ConversationalAgentState = ConversationalAgentState(1i32);
    pub const Listening: ConversationalAgentState = ConversationalAgentState(2i32);
    pub const Working: ConversationalAgentState = ConversationalAgentState(3i32);
    pub const Speaking: ConversationalAgentState = ConversationalAgentState(4i32);
    pub const ListeningAndSpeaking: ConversationalAgentState = ConversationalAgentState(5i32);
}
impl ::core::convert::From<i32> for ConversationalAgentState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentState;i4)");
}
impl ::windows::core::DefaultType for ConversationalAgentState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentSystemStateChangeType(pub i32);
impl ConversationalAgentSystemStateChangeType {
    pub const UserAuthentication: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(0i32);
    pub const ScreenAvailability: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(1i32);
    pub const IndicatorLightAvailability: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(2i32);
    pub const VoiceActivationAvailability: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(3i32);
}
impl ::core::convert::From<i32> for ConversationalAgentSystemStateChangeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentSystemStateChangeType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSystemStateChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangeType;i4)");
}
impl ::windows::core::DefaultType for ConversationalAgentSystemStateChangeType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConversationalAgentSystemStateChangedEventArgs(pub ::windows::core::IInspectable);
impl ConversationalAgentSystemStateChangedEventArgs {
    pub fn SystemStateChangeType(&self) -> ::windows::core::Result<ConversationalAgentSystemStateChangeType> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSystemStateChangeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSystemStateChangeType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSystemStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs;{1c2c6e3e-2785-59a7-8e71-38adeef79928})");
}
unsafe impl ::windows::core::Interface for ConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c2c6e3e_2785_59a7_8e71_38adeef79928);
}
impl ::windows::core::RuntimeName for ConversationalAgentSystemStateChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs";
}
impl ::core::convert::From<ConversationalAgentSystemStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSystemStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConversationalAgentSystemStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSystemStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConversationalAgentSystemStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSystemStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConversationalAgentSystemStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSystemStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSystemStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSystemStateChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentVoiceActivationPrerequisiteKind(pub i32);
impl ConversationalAgentVoiceActivationPrerequisiteKind {
    pub const MicrophonePermission: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(0i32);
    pub const KnownAgents: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(1i32);
    pub const AgentAllowed: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(2i32);
    pub const AppCapability: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(3i32);
    pub const BackgroundTaskRegistration: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(4i32);
    pub const PolicyPermission: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(5i32);
}
impl ::core::convert::From<i32> for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentVoiceActivationPrerequisiteKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentVoiceActivationPrerequisiteKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind;i4)");
}
impl ::windows::core::DefaultType for ConversationalAgentVoiceActivationPrerequisiteKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityChangeKind(pub i32);
impl DetectionConfigurationAvailabilityChangeKind {
    pub const SystemResourceAccess: DetectionConfigurationAvailabilityChangeKind = DetectionConfigurationAvailabilityChangeKind(0i32);
    pub const Permission: DetectionConfigurationAvailabilityChangeKind = DetectionConfigurationAvailabilityChangeKind(1i32);
    pub const LockScreenPermission: DetectionConfigurationAvailabilityChangeKind = DetectionConfigurationAvailabilityChangeKind(2i32);
}
impl ::core::convert::From<i32> for DetectionConfigurationAvailabilityChangeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DetectionConfigurationAvailabilityChangeKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationAvailabilityChangeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangeKind;i4)");
}
impl ::windows::core::DefaultType for DetectionConfigurationAvailabilityChangeKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DetectionConfigurationAvailabilityChangedEventArgs(pub ::windows::core::IInspectable);
impl DetectionConfigurationAvailabilityChangedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityChangeKind> {
        let this = self;
        unsafe {
            let mut result__: DetectionConfigurationAvailabilityChangeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DetectionConfigurationAvailabilityChangeKind>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs;{5129c9fb-4be8-5f14-af2b-88d62b1b4462})");
}
unsafe impl ::windows::core::Interface for DetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5129c9fb_4be8_5f14_af2b_88d62b1b4462);
}
impl ::windows::core::RuntimeName for DetectionConfigurationAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs";
}
impl ::core::convert::From<DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DetectionConfigurationAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for DetectionConfigurationAvailabilityChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DetectionConfigurationAvailabilityInfo(pub ::windows::core::IInspectable);
impl DetectionConfigurationAvailabilityInfo {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasSystemResourceAccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasPermission(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasLockScreenPermission(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnavailableSystemResources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>> {
        let this = &::windows::core::Interface::cast::<IDetectionConfigurationAvailabilityInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationAvailabilityInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo;{b5affeb0-40f0-5398-b838-91979c2c6208})");
}
unsafe impl ::windows::core::Interface for DetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5affeb0_40f0_5398_b838_91979c2c6208);
}
impl ::windows::core::RuntimeName for DetectionConfigurationAvailabilityInfo {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo";
}
impl ::core::convert::From<DetectionConfigurationAvailabilityInfo> for ::windows::core::IUnknown {
    fn from(value: DetectionConfigurationAvailabilityInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityInfo> for ::windows::core::IUnknown {
    fn from(value: &DetectionConfigurationAvailabilityInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DetectionConfigurationAvailabilityInfo> for ::windows::core::IInspectable {
    fn from(value: DetectionConfigurationAvailabilityInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityInfo> for ::windows::core::IInspectable {
    fn from(value: &DetectionConfigurationAvailabilityInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DetectionConfigurationAvailabilityInfo {}
unsafe impl ::core::marker::Sync for DetectionConfigurationAvailabilityInfo {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DetectionConfigurationTrainingStatus(pub i32);
impl DetectionConfigurationTrainingStatus {
    pub const Success: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(0i32);
    pub const FormatNotSupported: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(1i32);
    pub const VoiceTooQuiet: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(2i32);
    pub const VoiceTooLoud: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(3i32);
    pub const VoiceTooFast: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(4i32);
    pub const VoiceTooSlow: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(5i32);
    pub const VoiceQualityProblem: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(6i32);
    pub const TrainingSystemInternalError: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(7i32);
    pub const TrainingTimedOut: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(8i32);
    pub const ConfigurationNotFound: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(9i32);
}
impl ::core::convert::From<i32> for DetectionConfigurationTrainingStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DetectionConfigurationTrainingStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationTrainingStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus;i4)");
}
impl ::windows::core::DefaultType for DetectionConfigurationTrainingStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40d8be16_5217_581c_9ab2_ce9b2f2e8e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ActivationSignalDetectionTrainingDataFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::core::RawPtr, result__: *mut DetectionConfigurationTrainingStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivationSignalDetectionConfiguration2 {
    type Vtable = IActivationSignalDetectionConfiguration2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71d9b022_562c_57ce_a78b_8b4ff0145bab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ActivationSignalDetectionConfigurationSetModelDataResult) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool, result__: *mut ActivationSignalDetectionConfigurationStateChangeResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfigurationCreationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c89bc1b_8d12_5e48_a71c_7f6bc1cd66e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfigurationCreationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ActivationSignalDetectionConfigurationCreationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivationSignalDetector {
    type Vtable = IActivationSignalDetector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5bf345f_a4d0_5b2b_8e65_b3c55ee756ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetector2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivationSignalDetector2 {
    type Vtable = IActivationSignalDetector2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7e2490a_baa5_59d2_85d1_ba42f7cf78c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ActivationSignalDetectionConfigurationRemovalResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde94fbb0_597a_5df8_8cfb_9dbb583ba3ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kind: ActivationSignalDetectorKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kind: ActivationSignalDetectorKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentDetectorManager2 {
    type Vtable = IConversationalAgentDetectorManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84610f31_d7f3_52fe_9311_c9eb4e3eb30a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, detectorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, detectorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentDetectorManagerStatics {
    type Vtable = IConversationalAgentDetectorManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36a8d283_fa0e_5693_8489_0fb2f0ab40d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManagerStatics_abi(
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
pub struct IConversationalAgentSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentSession {
    type Vtable = IConversationalAgentSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaaae09a_b7ba_57e5_ad13_df520f9b6fa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ConversationalAgentState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interruptible: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interruptible: bool, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: ConversationalAgentState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: ConversationalAgentState, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Audio"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, graph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Audio")))] usize,
    #[cfg(feature = "Media_Audio")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, graph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalmodelid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalmodelid: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSession2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentSession2 {
    type Vtable = IConversationalAgentSession2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7a9fbf9_ac78_57ff_9596_acc7a1c9a607);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activationkind: ConversationalAgentActivationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activationkind: ConversationalAgentActivationKind, result__: *mut ConversationalAgentActivationResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lockscreenactivationsupported: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lockscreenactivationsupported: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSessionInterruptedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9766591f_f63d_5d3e_9bf2_bd0760552686);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionInterruptedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSessionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentSessionStatics {
    type Vtable = IConversationalAgentSessionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa005166e_e954_576e_be04_11b8ed10f37b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSignal(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20ed25f7_b120_51f2_8603_265d6a47f232);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSignal2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentSignal2 {
    type Vtable = IConversationalAgentSignal2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0cc7ba9_9a7b_5c34_880e_b6146c904ecb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSignalDetectedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d57eb8f_f88a_599b_91d3_d604876708bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignalDetectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSystemStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c2c6e3e_2785_59a7_8e71_38adeef79928);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSystemStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ConversationalAgentSystemStateChangeType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5129c9fb_4be8_5f14_af2b_88d62b1b4462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DetectionConfigurationAvailabilityChangeKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5affeb0_40f0_5398_b838_91979c2c6208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDetectionConfigurationAvailabilityInfo2 {
    type Vtable = IDetectionConfigurationAvailabilityInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30e06433_38b3_5c4b_84c3_62b6e685b2ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SignalDetectorResourceKind(pub i32);
impl SignalDetectorResourceKind {
    pub const ParallelModelSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(0i32);
    pub const ParallelModelSupportForAgent: SignalDetectorResourceKind = SignalDetectorResourceKind(1i32);
    pub const ParallelSignalSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(2i32);
    pub const ParallelSignalSupportForAgent: SignalDetectorResourceKind = SignalDetectorResourceKind(3i32);
    pub const DisplayOffSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(4i32);
    pub const PluggedInPower: SignalDetectorResourceKind = SignalDetectorResourceKind(5i32);
    pub const Detector: SignalDetectorResourceKind = SignalDetectorResourceKind(6i32);
    pub const SupportedSleepState: SignalDetectorResourceKind = SignalDetectorResourceKind(7i32);
    pub const SupportedBatterySaverState: SignalDetectorResourceKind = SignalDetectorResourceKind(8i32);
    pub const ScreenAvailability: SignalDetectorResourceKind = SignalDetectorResourceKind(9i32);
    pub const InputHardware: SignalDetectorResourceKind = SignalDetectorResourceKind(10i32);
    pub const AcousticEchoCancellation: SignalDetectorResourceKind = SignalDetectorResourceKind(11i32);
    pub const ModelIdSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(12i32);
    pub const DataChannel: SignalDetectorResourceKind = SignalDetectorResourceKind(13i32);
}
impl ::core::convert::From<i32> for SignalDetectorResourceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SignalDetectorResourceKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SignalDetectorResourceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind;i4)");
}
impl ::windows::core::DefaultType for SignalDetectorResourceKind {
    type DefaultType = Self;
}
