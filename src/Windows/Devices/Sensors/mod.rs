#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Sensors_Custom")]
pub mod Custom;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Accelerometer(pub ::windows::core::IInspectable);
impl Accelerometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccelerometerReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Shaken<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveShaken<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccelerometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::Interface::cast::<IAccelerometer2>(self)?;
        unsafe {
            let mut result__: super::super::Graphics::Display::DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccelerometerDeviceId>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Accelerometer> {
        Self::IAccelerometerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Accelerometer>(result__)
        })
    }
    pub fn ReadingType(&self) -> ::windows::core::Result<AccelerometerReadingType> {
        let this = &::windows::core::Interface::cast::<IAccelerometer4>(self)?;
        unsafe {
            let mut result__: AccelerometerReadingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccelerometerReadingType>(result__)
        }
    }
    pub fn GetDefaultWithAccelerometerReadingType(readingtype: AccelerometerReadingType) -> ::windows::core::Result<Accelerometer> {
        Self::IAccelerometerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), readingtype, &mut result__).from_abi::<Accelerometer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Accelerometer>> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Accelerometer>>(result__)
        })
    }
    pub fn GetDeviceSelector(readingtype: AccelerometerReadingType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), readingtype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<AccelerometerDataThreshold> {
        let this = &::windows::core::Interface::cast::<IAccelerometer5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccelerometerDataThreshold>(result__)
        }
    }
    pub fn IAccelerometerStatics<R, F: FnOnce(&IAccelerometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Accelerometer, IAccelerometerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccelerometerStatics2<R, F: FnOnce(&IAccelerometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Accelerometer, IAccelerometerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccelerometerStatics3<R, F: FnOnce(&IAccelerometerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Accelerometer, IAccelerometerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Accelerometer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Accelerometer;{df184548-2711-4da7-8098-4b82205d3c7d})");
}
unsafe impl ::windows::core::Interface for Accelerometer {
    type Vtable = IAccelerometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf184548_2711_4da7_8098_4b82205d3c7d);
}
impl ::windows::core::RuntimeName for Accelerometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Accelerometer";
}
impl ::core::convert::From<Accelerometer> for ::windows::core::IUnknown {
    fn from(value: Accelerometer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Accelerometer> for ::windows::core::IUnknown {
    fn from(value: &Accelerometer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Accelerometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Accelerometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Accelerometer> for ::windows::core::IInspectable {
    fn from(value: Accelerometer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Accelerometer> for ::windows::core::IInspectable {
    fn from(value: &Accelerometer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Accelerometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Accelerometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Accelerometer {}
unsafe impl ::core::marker::Sync for Accelerometer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccelerometerDataThreshold(pub ::windows::core::IInspectable);
impl AccelerometerDataThreshold {
    pub fn XAxisInGForce(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetXAxisInGForce(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn YAxisInGForce(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetYAxisInGForce(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ZAxisInGForce(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetZAxisInGForce(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AccelerometerDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerDataThreshold;{f92c1b68-6320-5577-879e-9942621c3dd9})");
}
unsafe impl ::windows::core::Interface for AccelerometerDataThreshold {
    type Vtable = IAccelerometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf92c1b68_6320_5577_879e_9942621c3dd9);
}
impl ::windows::core::RuntimeName for AccelerometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerDataThreshold";
}
impl ::core::convert::From<AccelerometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: AccelerometerDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccelerometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &AccelerometerDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccelerometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccelerometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccelerometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: AccelerometerDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccelerometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &AccelerometerDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccelerometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccelerometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccelerometerDataThreshold {}
unsafe impl ::core::marker::Sync for AccelerometerDataThreshold {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccelerometerReading(pub ::windows::core::IInspectable);
impl AccelerometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn AccelerationX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn AccelerationY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn AccelerationZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AccelerometerReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerReading;{b9fe7acb-d351-40af-8bb6-7aa9ae641fb7})");
}
unsafe impl ::windows::core::Interface for AccelerometerReading {
    type Vtable = IAccelerometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9fe7acb_d351_40af_8bb6_7aa9ae641fb7);
}
impl ::windows::core::RuntimeName for AccelerometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReading";
}
impl ::core::convert::From<AccelerometerReading> for ::windows::core::IUnknown {
    fn from(value: AccelerometerReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccelerometerReading> for ::windows::core::IUnknown {
    fn from(value: &AccelerometerReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccelerometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccelerometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccelerometerReading> for ::windows::core::IInspectable {
    fn from(value: AccelerometerReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccelerometerReading> for ::windows::core::IInspectable {
    fn from(value: &AccelerometerReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccelerometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccelerometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccelerometerReading {}
unsafe impl ::core::marker::Sync for AccelerometerReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccelerometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl AccelerometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccelerometerReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AccelerometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerReadingChangedEventArgs;{0095c65b-b6ac-475a-9f44-8b32d35a3f25})");
}
unsafe impl ::windows::core::Interface for AccelerometerReadingChangedEventArgs {
    type Vtable = IAccelerometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0095c65b_b6ac_475a_9f44_8b32d35a3f25);
}
impl ::windows::core::RuntimeName for AccelerometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReadingChangedEventArgs";
}
impl ::core::convert::From<AccelerometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccelerometerReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccelerometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccelerometerReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccelerometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccelerometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccelerometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccelerometerReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccelerometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccelerometerReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccelerometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccelerometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccelerometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for AccelerometerReadingChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AccelerometerReadingType(pub i32);
impl AccelerometerReadingType {
    pub const Standard: AccelerometerReadingType = AccelerometerReadingType(0i32);
    pub const Linear: AccelerometerReadingType = AccelerometerReadingType(1i32);
    pub const Gravity: AccelerometerReadingType = AccelerometerReadingType(2i32);
}
impl ::core::convert::From<i32> for AccelerometerReadingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AccelerometerReadingType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AccelerometerReadingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.AccelerometerReadingType;i4)");
}
impl ::windows::core::DefaultType for AccelerometerReadingType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccelerometerShakenEventArgs(pub ::windows::core::IInspectable);
impl AccelerometerShakenEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AccelerometerShakenEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerShakenEventArgs;{95ff01d1-4a28-4f35-98e8-8178aae4084a})");
}
unsafe impl ::windows::core::Interface for AccelerometerShakenEventArgs {
    type Vtable = IAccelerometerShakenEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95ff01d1_4a28_4f35_98e8_8178aae4084a);
}
impl ::windows::core::RuntimeName for AccelerometerShakenEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerShakenEventArgs";
}
impl ::core::convert::From<AccelerometerShakenEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccelerometerShakenEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccelerometerShakenEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccelerometerShakenEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccelerometerShakenEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccelerometerShakenEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccelerometerShakenEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccelerometerShakenEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccelerometerShakenEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccelerometerShakenEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccelerometerShakenEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccelerometerShakenEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccelerometerShakenEventArgs {}
unsafe impl ::core::marker::Sync for AccelerometerShakenEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ActivitySensor(pub ::windows::core::IInspectable);
impl ActivitySensor {
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentReadingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensorReading>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivitySensorReading>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SubscribedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ActivityType>>(result__)
        }
    }
    pub fn PowerInMilliwatts(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivityType>>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivitySensor>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivitySensor>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSystemHistoryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(fromtime: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), fromtime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSystemHistoryWithDurationAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(fromtime: Param0, duration: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), fromtime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>(result__)
        })
    }
    pub fn IActivitySensorStatics<R, F: FnOnce(&IActivitySensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ActivitySensor, IActivitySensorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ActivitySensor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensor;{cd7a630c-fb5f-48eb-b09b-a2708d1c61ef})");
}
unsafe impl ::windows::core::Interface for ActivitySensor {
    type Vtable = IActivitySensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd7a630c_fb5f_48eb_b09b_a2708d1c61ef);
}
impl ::windows::core::RuntimeName for ActivitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensor";
}
impl ::core::convert::From<ActivitySensor> for ::windows::core::IUnknown {
    fn from(value: ActivitySensor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ActivitySensor> for ::windows::core::IUnknown {
    fn from(value: &ActivitySensor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivitySensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivitySensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ActivitySensor> for ::windows::core::IInspectable {
    fn from(value: ActivitySensor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ActivitySensor> for ::windows::core::IInspectable {
    fn from(value: &ActivitySensor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivitySensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivitySensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ActivitySensor {}
unsafe impl ::core::marker::Sync for ActivitySensor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ActivitySensorReading(pub ::windows::core::IInspectable);
impl ActivitySensorReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Activity(&self) -> ::windows::core::Result<ActivityType> {
        let this = self;
        unsafe {
            let mut result__: ActivityType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivityType>(result__)
        }
    }
    pub fn Confidence(&self) -> ::windows::core::Result<ActivitySensorReadingConfidence> {
        let this = self;
        unsafe {
            let mut result__: ActivitySensorReadingConfidence = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivitySensorReadingConfidence>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ActivitySensorReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorReading;{85125a96-1472-40a2-b2ae-e1ef29226c78})");
}
unsafe impl ::windows::core::Interface for ActivitySensorReading {
    type Vtable = IActivitySensorReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85125a96_1472_40a2_b2ae_e1ef29226c78);
}
impl ::windows::core::RuntimeName for ActivitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReading";
}
impl ::core::convert::From<ActivitySensorReading> for ::windows::core::IUnknown {
    fn from(value: ActivitySensorReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ActivitySensorReading> for ::windows::core::IUnknown {
    fn from(value: &ActivitySensorReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivitySensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivitySensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ActivitySensorReading> for ::windows::core::IInspectable {
    fn from(value: ActivitySensorReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ActivitySensorReading> for ::windows::core::IInspectable {
    fn from(value: &ActivitySensorReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivitySensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivitySensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ActivitySensorReading {}
unsafe impl ::core::marker::Sync for ActivitySensorReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ActivitySensorReadingChangeReport(pub ::windows::core::IInspectable);
impl ActivitySensorReadingChangeReport {
    pub fn Reading(&self) -> ::windows::core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivitySensorReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ActivitySensorReadingChangeReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorReadingChangeReport;{4f3c2915-d93b-47bd-960a-f20fb2f322b9})");
}
unsafe impl ::windows::core::Interface for ActivitySensorReadingChangeReport {
    type Vtable = IActivitySensorReadingChangeReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f3c2915_d93b_47bd_960a_f20fb2f322b9);
}
impl ::windows::core::RuntimeName for ActivitySensorReadingChangeReport {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangeReport";
}
impl ::core::convert::From<ActivitySensorReadingChangeReport> for ::windows::core::IUnknown {
    fn from(value: ActivitySensorReadingChangeReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ActivitySensorReadingChangeReport> for ::windows::core::IUnknown {
    fn from(value: &ActivitySensorReadingChangeReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivitySensorReadingChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivitySensorReadingChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ActivitySensorReadingChangeReport> for ::windows::core::IInspectable {
    fn from(value: ActivitySensorReadingChangeReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ActivitySensorReadingChangeReport> for ::windows::core::IInspectable {
    fn from(value: &ActivitySensorReadingChangeReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivitySensorReadingChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivitySensorReadingChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ActivitySensorReadingChangeReport {}
unsafe impl ::core::marker::Sync for ActivitySensorReadingChangeReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ActivitySensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl ActivitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivitySensorReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ActivitySensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorReadingChangedEventArgs;{de386717-aeb6-4ec7-946a-d9cc19b951ec})");
}
unsafe impl ::windows::core::Interface for ActivitySensorReadingChangedEventArgs {
    type Vtable = IActivitySensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde386717_aeb6_4ec7_946a_d9cc19b951ec);
}
impl ::windows::core::RuntimeName for ActivitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangedEventArgs";
}
impl ::core::convert::From<ActivitySensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ActivitySensorReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ActivitySensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ActivitySensorReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ActivitySensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ActivitySensorReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ActivitySensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ActivitySensorReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ActivitySensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for ActivitySensorReadingChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivitySensorReadingConfidence(pub i32);
impl ActivitySensorReadingConfidence {
    pub const High: ActivitySensorReadingConfidence = ActivitySensorReadingConfidence(0i32);
    pub const Low: ActivitySensorReadingConfidence = ActivitySensorReadingConfidence(1i32);
}
impl ::core::convert::From<i32> for ActivitySensorReadingConfidence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivitySensorReadingConfidence {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivitySensorReadingConfidence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivitySensorReadingConfidence;i4)");
}
impl ::windows::core::DefaultType for ActivitySensorReadingConfidence {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ActivitySensorTriggerDetails(pub ::windows::core::IInspectable);
impl ActivitySensorTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ActivitySensorTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorTriggerDetails;{2c9e6612-b9ca-4677-b263-243297f79d3a})");
}
unsafe impl ::windows::core::Interface for ActivitySensorTriggerDetails {
    type Vtable = IActivitySensorTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c9e6612_b9ca_4677_b263_243297f79d3a);
}
impl ::windows::core::RuntimeName for ActivitySensorTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorTriggerDetails";
}
impl ::core::convert::From<ActivitySensorTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: ActivitySensorTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ActivitySensorTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &ActivitySensorTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivitySensorTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivitySensorTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ActivitySensorTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: ActivitySensorTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ActivitySensorTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &ActivitySensorTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivitySensorTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivitySensorTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ActivitySensorTriggerDetails {}
unsafe impl ::core::marker::Sync for ActivitySensorTriggerDetails {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivityType(pub i32);
impl ActivityType {
    pub const Unknown: ActivityType = ActivityType(0i32);
    pub const Idle: ActivityType = ActivityType(1i32);
    pub const Stationary: ActivityType = ActivityType(2i32);
    pub const Fidgeting: ActivityType = ActivityType(3i32);
    pub const Walking: ActivityType = ActivityType(4i32);
    pub const Running: ActivityType = ActivityType(5i32);
    pub const InVehicle: ActivityType = ActivityType(6i32);
    pub const Biking: ActivityType = ActivityType(7i32);
}
impl ::core::convert::From<i32> for ActivityType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivityType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivityType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivityType;i4)");
}
impl ::windows::core::DefaultType for ActivityType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Altimeter(pub ::windows::core::IInspectable);
impl Altimeter {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AltimeterReading>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<Altimeter> {
        Self::IAltimeterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Altimeter>(result__)
        })
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAltimeter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn IAltimeterStatics<R, F: FnOnce(&IAltimeterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Altimeter, IAltimeterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Altimeter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Altimeter;{72f057fd-8f04-49f1-b4a7-f4e363b701a2})");
}
unsafe impl ::windows::core::Interface for Altimeter {
    type Vtable = IAltimeter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72f057fd_8f04_49f1_b4a7_f4e363b701a2);
}
impl ::windows::core::RuntimeName for Altimeter {
    const NAME: &'static str = "Windows.Devices.Sensors.Altimeter";
}
impl ::core::convert::From<Altimeter> for ::windows::core::IUnknown {
    fn from(value: Altimeter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Altimeter> for ::windows::core::IUnknown {
    fn from(value: &Altimeter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Altimeter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Altimeter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Altimeter> for ::windows::core::IInspectable {
    fn from(value: Altimeter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Altimeter> for ::windows::core::IInspectable {
    fn from(value: &Altimeter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Altimeter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Altimeter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Altimeter {}
unsafe impl ::core::marker::Sync for Altimeter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AltimeterReading(pub ::windows::core::IInspectable);
impl AltimeterReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn AltitudeChangeInMeters(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AltimeterReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AltimeterReading;{fbe8ef73-7f5e-48c8-aa1a-f1f3befc1144})");
}
unsafe impl ::windows::core::Interface for AltimeterReading {
    type Vtable = IAltimeterReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbe8ef73_7f5e_48c8_aa1a_f1f3befc1144);
}
impl ::windows::core::RuntimeName for AltimeterReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReading";
}
impl ::core::convert::From<AltimeterReading> for ::windows::core::IUnknown {
    fn from(value: AltimeterReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AltimeterReading> for ::windows::core::IUnknown {
    fn from(value: &AltimeterReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AltimeterReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AltimeterReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AltimeterReading> for ::windows::core::IInspectable {
    fn from(value: AltimeterReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AltimeterReading> for ::windows::core::IInspectable {
    fn from(value: &AltimeterReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AltimeterReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AltimeterReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AltimeterReading {}
unsafe impl ::core::marker::Sync for AltimeterReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AltimeterReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl AltimeterReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AltimeterReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AltimeterReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AltimeterReadingChangedEventArgs;{7069d077-446d-47f7-998c-ebc23b45e4a2})");
}
unsafe impl ::windows::core::Interface for AltimeterReadingChangedEventArgs {
    type Vtable = IAltimeterReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7069d077_446d_47f7_998c_ebc23b45e4a2);
}
impl ::windows::core::RuntimeName for AltimeterReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReadingChangedEventArgs";
}
impl ::core::convert::From<AltimeterReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AltimeterReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AltimeterReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AltimeterReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AltimeterReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AltimeterReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AltimeterReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AltimeterReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AltimeterReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AltimeterReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AltimeterReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AltimeterReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AltimeterReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for AltimeterReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Barometer(pub ::windows::core::IInspectable);
impl Barometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarometerReading>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<Barometer> {
        Self::IBarometerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Barometer>(result__)
        })
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBarometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Barometer>> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Barometer>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<BarometerDataThreshold> {
        let this = &::windows::core::Interface::cast::<IBarometer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarometerDataThreshold>(result__)
        }
    }
    pub fn IBarometerStatics<R, F: FnOnce(&IBarometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Barometer, IBarometerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBarometerStatics2<R, F: FnOnce(&IBarometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Barometer, IBarometerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Barometer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Barometer;{934475a8-78bf-452f-b017-f0209ce6dab4})");
}
unsafe impl ::windows::core::Interface for Barometer {
    type Vtable = IBarometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x934475a8_78bf_452f_b017_f0209ce6dab4);
}
impl ::windows::core::RuntimeName for Barometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Barometer";
}
impl ::core::convert::From<Barometer> for ::windows::core::IUnknown {
    fn from(value: Barometer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Barometer> for ::windows::core::IUnknown {
    fn from(value: &Barometer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Barometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Barometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Barometer> for ::windows::core::IInspectable {
    fn from(value: Barometer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Barometer> for ::windows::core::IInspectable {
    fn from(value: &Barometer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Barometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Barometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Barometer {}
unsafe impl ::core::marker::Sync for Barometer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarometerDataThreshold(pub ::windows::core::IInspectable);
impl BarometerDataThreshold {
    pub fn Hectopascals(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetHectopascals(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for BarometerDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.BarometerDataThreshold;{076b952c-cb62-5a90-a0d1-f85e4a936394})");
}
unsafe impl ::windows::core::Interface for BarometerDataThreshold {
    type Vtable = IBarometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076b952c_cb62_5a90_a0d1_f85e4a936394);
}
impl ::windows::core::RuntimeName for BarometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerDataThreshold";
}
impl ::core::convert::From<BarometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: BarometerDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &BarometerDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: BarometerDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &BarometerDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarometerDataThreshold {}
unsafe impl ::core::marker::Sync for BarometerDataThreshold {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarometerReading(pub ::windows::core::IInspectable);
impl BarometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn StationPressureInHectopascals(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BarometerReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.BarometerReading;{f5b9d2e6-1df6-4a1a-a7ad-321d4f5db247})");
}
unsafe impl ::windows::core::Interface for BarometerReading {
    type Vtable = IBarometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b9d2e6_1df6_4a1a_a7ad_321d4f5db247);
}
impl ::windows::core::RuntimeName for BarometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReading";
}
impl ::core::convert::From<BarometerReading> for ::windows::core::IUnknown {
    fn from(value: BarometerReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarometerReading> for ::windows::core::IUnknown {
    fn from(value: &BarometerReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarometerReading> for ::windows::core::IInspectable {
    fn from(value: BarometerReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarometerReading> for ::windows::core::IInspectable {
    fn from(value: &BarometerReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarometerReading {}
unsafe impl ::core::marker::Sync for BarometerReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl BarometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarometerReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BarometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.BarometerReadingChangedEventArgs;{3d84945f-037b-404f-9bbb-6232d69543c3})");
}
unsafe impl ::windows::core::Interface for BarometerReadingChangedEventArgs {
    type Vtable = IBarometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d84945f_037b_404f_9bbb_6232d69543c3);
}
impl ::windows::core::RuntimeName for BarometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReadingChangedEventArgs";
}
impl ::core::convert::From<BarometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarometerReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarometerReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarometerReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarometerReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for BarometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Compass(pub ::windows::core::IInspectable);
impl Compass {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CompassReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICompass2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::Interface::cast::<ICompass2>(self)?;
        unsafe {
            let mut result__: super::super::Graphics::Display::DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICompassDeviceId>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Compass> {
        Self::ICompassStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Compass>(result__)
        })
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICompass3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Compass>> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Compass>>(result__)
        })
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<CompassDataThreshold> {
        let this = &::windows::core::Interface::cast::<ICompass4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CompassDataThreshold>(result__)
        }
    }
    pub fn ICompassStatics<R, F: FnOnce(&ICompassStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Compass, ICompassStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICompassStatics2<R, F: FnOnce(&ICompassStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Compass, ICompassStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Compass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Compass;{292ffa94-1b45-403c-ba06-b106dba69a64})");
}
unsafe impl ::windows::core::Interface for Compass {
    type Vtable = ICompass_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x292ffa94_1b45_403c_ba06_b106dba69a64);
}
impl ::windows::core::RuntimeName for Compass {
    const NAME: &'static str = "Windows.Devices.Sensors.Compass";
}
impl ::core::convert::From<Compass> for ::windows::core::IUnknown {
    fn from(value: Compass) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Compass> for ::windows::core::IUnknown {
    fn from(value: &Compass) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Compass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Compass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Compass> for ::windows::core::IInspectable {
    fn from(value: Compass) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Compass> for ::windows::core::IInspectable {
    fn from(value: &Compass) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Compass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Compass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Compass {}
unsafe impl ::core::marker::Sync for Compass {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompassDataThreshold(pub ::windows::core::IInspectable);
impl CompassDataThreshold {
    pub fn Degrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CompassDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.CompassDataThreshold;{d15b52b3-d39d-5ec8-b2e4-f193e6ab34ed})");
}
unsafe impl ::windows::core::Interface for CompassDataThreshold {
    type Vtable = ICompassDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd15b52b3_d39d_5ec8_b2e4_f193e6ab34ed);
}
impl ::windows::core::RuntimeName for CompassDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassDataThreshold";
}
impl ::core::convert::From<CompassDataThreshold> for ::windows::core::IUnknown {
    fn from(value: CompassDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompassDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &CompassDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompassDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompassDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompassDataThreshold> for ::windows::core::IInspectable {
    fn from(value: CompassDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompassDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &CompassDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompassDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompassDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CompassDataThreshold {}
unsafe impl ::core::marker::Sync for CompassDataThreshold {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompassReading(pub ::windows::core::IInspectable);
impl CompassReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn HeadingMagneticNorth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadingTrueNorth(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    pub fn HeadingAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy> {
        let this = &::windows::core::Interface::cast::<ICompassReadingHeadingAccuracy>(self)?;
        unsafe {
            let mut result__: MagnetometerAccuracy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagnetometerAccuracy>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CompassReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.CompassReading;{82911128-513d-4dc9-b781-5eedfbf02d0c})");
}
unsafe impl ::windows::core::Interface for CompassReading {
    type Vtable = ICompassReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82911128_513d_4dc9_b781_5eedfbf02d0c);
}
impl ::windows::core::RuntimeName for CompassReading {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReading";
}
impl ::core::convert::From<CompassReading> for ::windows::core::IUnknown {
    fn from(value: CompassReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompassReading> for ::windows::core::IUnknown {
    fn from(value: &CompassReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompassReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompassReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompassReading> for ::windows::core::IInspectable {
    fn from(value: CompassReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompassReading> for ::windows::core::IInspectable {
    fn from(value: &CompassReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompassReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompassReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CompassReading {}
unsafe impl ::core::marker::Sync for CompassReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompassReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl CompassReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CompassReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CompassReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.CompassReadingChangedEventArgs;{8f1549b0-e8bc-4c7e-b009-4e41df137072})");
}
unsafe impl ::windows::core::Interface for CompassReadingChangedEventArgs {
    type Vtable = ICompassReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1549b0_e8bc_4c7e_b009_4e41df137072);
}
impl ::windows::core::RuntimeName for CompassReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReadingChangedEventArgs";
}
impl ::core::convert::From<CompassReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CompassReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompassReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CompassReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompassReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompassReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompassReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CompassReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompassReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CompassReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompassReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompassReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CompassReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for CompassReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Gyrometer(pub ::windows::core::IInspectable);
impl Gyrometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GyrometerReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGyrometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::Interface::cast::<IGyrometer2>(self)?;
        unsafe {
            let mut result__: super::super::Graphics::Display::DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IGyrometerDeviceId>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Gyrometer> {
        Self::IGyrometerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Gyrometer>(result__)
        })
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGyrometer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Gyrometer>> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Gyrometer>>(result__)
        })
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<GyrometerDataThreshold> {
        let this = &::windows::core::Interface::cast::<IGyrometer4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GyrometerDataThreshold>(result__)
        }
    }
    pub fn IGyrometerStatics<R, F: FnOnce(&IGyrometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Gyrometer, IGyrometerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGyrometerStatics2<R, F: FnOnce(&IGyrometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Gyrometer, IGyrometerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Gyrometer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Gyrometer;{fdb9a9c4-84b1-4ca2-9763-9b589506c70c})");
}
unsafe impl ::windows::core::Interface for Gyrometer {
    type Vtable = IGyrometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdb9a9c4_84b1_4ca2_9763_9b589506c70c);
}
impl ::windows::core::RuntimeName for Gyrometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Gyrometer";
}
impl ::core::convert::From<Gyrometer> for ::windows::core::IUnknown {
    fn from(value: Gyrometer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Gyrometer> for ::windows::core::IUnknown {
    fn from(value: &Gyrometer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Gyrometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Gyrometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Gyrometer> for ::windows::core::IInspectable {
    fn from(value: Gyrometer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Gyrometer> for ::windows::core::IInspectable {
    fn from(value: &Gyrometer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Gyrometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Gyrometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Gyrometer {}
unsafe impl ::core::marker::Sync for Gyrometer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GyrometerDataThreshold(pub ::windows::core::IInspectable);
impl GyrometerDataThreshold {
    pub fn XAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetXAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn YAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetYAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ZAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetZAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for GyrometerDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.GyrometerDataThreshold;{8648b31e-6e52-5259-bbad-242a69dc38c8})");
}
unsafe impl ::windows::core::Interface for GyrometerDataThreshold {
    type Vtable = IGyrometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8648b31e_6e52_5259_bbad_242a69dc38c8);
}
impl ::windows::core::RuntimeName for GyrometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerDataThreshold";
}
impl ::core::convert::From<GyrometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: GyrometerDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GyrometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &GyrometerDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GyrometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GyrometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GyrometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: GyrometerDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GyrometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &GyrometerDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GyrometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GyrometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GyrometerDataThreshold {}
unsafe impl ::core::marker::Sync for GyrometerDataThreshold {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GyrometerReading(pub ::windows::core::IInspectable);
impl GyrometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn AngularVelocityX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn AngularVelocityY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn AngularVelocityZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GyrometerReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.GyrometerReading;{b3d6de5c-1ee4-456f-9de7-e2493b5c8e03})");
}
unsafe impl ::windows::core::Interface for GyrometerReading {
    type Vtable = IGyrometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3d6de5c_1ee4_456f_9de7_e2493b5c8e03);
}
impl ::windows::core::RuntimeName for GyrometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReading";
}
impl ::core::convert::From<GyrometerReading> for ::windows::core::IUnknown {
    fn from(value: GyrometerReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GyrometerReading> for ::windows::core::IUnknown {
    fn from(value: &GyrometerReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GyrometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GyrometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GyrometerReading> for ::windows::core::IInspectable {
    fn from(value: GyrometerReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GyrometerReading> for ::windows::core::IInspectable {
    fn from(value: &GyrometerReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GyrometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GyrometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GyrometerReading {}
unsafe impl ::core::marker::Sync for GyrometerReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GyrometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl GyrometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GyrometerReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GyrometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.GyrometerReadingChangedEventArgs;{0fdf1895-6f9e-42ce-8d58-388c0ab8356d})");
}
unsafe impl ::windows::core::Interface for GyrometerReadingChangedEventArgs {
    type Vtable = IGyrometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fdf1895_6f9e_42ce_8d58_388c0ab8356d);
}
impl ::windows::core::RuntimeName for GyrometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReadingChangedEventArgs";
}
impl ::core::convert::From<GyrometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GyrometerReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GyrometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GyrometerReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GyrometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GyrometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GyrometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GyrometerReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GyrometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GyrometerReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GyrometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GyrometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GyrometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for GyrometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HingeAngleReading(pub ::windows::core::IInspectable);
impl HingeAngleReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn AngleInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HingeAngleReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HingeAngleReading;{a3cd45b9-1bf1-4f65-a704-e2da04f182c0})");
}
unsafe impl ::windows::core::Interface for HingeAngleReading {
    type Vtable = IHingeAngleReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3cd45b9_1bf1_4f65_a704_e2da04f182c0);
}
impl ::windows::core::RuntimeName for HingeAngleReading {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleReading";
}
impl ::core::convert::From<HingeAngleReading> for ::windows::core::IUnknown {
    fn from(value: HingeAngleReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HingeAngleReading> for ::windows::core::IUnknown {
    fn from(value: &HingeAngleReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HingeAngleReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HingeAngleReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HingeAngleReading> for ::windows::core::IInspectable {
    fn from(value: HingeAngleReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HingeAngleReading> for ::windows::core::IInspectable {
    fn from(value: &HingeAngleReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HingeAngleReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HingeAngleReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HingeAngleReading {}
unsafe impl ::core::marker::Sync for HingeAngleReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HingeAngleSensor(pub ::windows::core::IInspectable);
impl HingeAngleSensor {
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentReadingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleReading>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HingeAngleReading>>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MinReportThresholdInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ReportThresholdInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetReportThresholdInDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRelatedToAdjacentPanelsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(firstpanelid: Param0, secondpanelid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), firstpanelid.into_param().abi(), secondpanelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>(result__)
        })
    }
    pub fn IHingeAngleSensorStatics<R, F: FnOnce(&IHingeAngleSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HingeAngleSensor, IHingeAngleSensorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for HingeAngleSensor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HingeAngleSensor;{e9d3be02-bfdf-437f-8c29-88c77393d309})");
}
unsafe impl ::windows::core::Interface for HingeAngleSensor {
    type Vtable = IHingeAngleSensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9d3be02_bfdf_437f_8c29_88c77393d309);
}
impl ::windows::core::RuntimeName for HingeAngleSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensor";
}
impl ::core::convert::From<HingeAngleSensor> for ::windows::core::IUnknown {
    fn from(value: HingeAngleSensor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HingeAngleSensor> for ::windows::core::IUnknown {
    fn from(value: &HingeAngleSensor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HingeAngleSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HingeAngleSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HingeAngleSensor> for ::windows::core::IInspectable {
    fn from(value: HingeAngleSensor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HingeAngleSensor> for ::windows::core::IInspectable {
    fn from(value: &HingeAngleSensor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HingeAngleSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HingeAngleSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HingeAngleSensor {}
unsafe impl ::core::marker::Sync for HingeAngleSensor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HingeAngleSensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl HingeAngleSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<HingeAngleReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HingeAngleReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HingeAngleSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HingeAngleSensorReadingChangedEventArgs;{24d9558b-fad0-42b8-a854-78923049a1ba})");
}
unsafe impl ::windows::core::Interface for HingeAngleSensorReadingChangedEventArgs {
    type Vtable = IHingeAngleSensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24d9558b_fad0_42b8_a854_78923049a1ba);
}
impl ::windows::core::RuntimeName for HingeAngleSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensorReadingChangedEventArgs";
}
impl ::core::convert::From<HingeAngleSensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HingeAngleSensorReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HingeAngleSensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HingeAngleSensorReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HingeAngleSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HingeAngleSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HingeAngleSensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HingeAngleSensorReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HingeAngleSensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HingeAngleSensorReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HingeAngleSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HingeAngleSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HingeAngleSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for HingeAngleSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometer {
    type Vtable = IAccelerometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf184548_2711_4da7_8098_4b82205d3c7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometer2 {
    type Vtable = IAccelerometer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8f092ee_4964_401a_b602_220d7153c60a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometer3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometer3 {
    type Vtable = IAccelerometer3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87e0022a_ed80_49eb_bf8a_a4ea31e5cd84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometer4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometer4 {
    type Vtable = IAccelerometer4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d373c4f_42d3_45b2_8144_ab7fb665eb59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AccelerometerReadingType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometer5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometer5 {
    type Vtable = IAccelerometer5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e7e7021_def4_53a6_af43_806fd538edf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer5_abi(
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
pub struct IAccelerometerDataThreshold(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometerDataThreshold {
    type Vtable = IAccelerometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf92c1b68_6320_5577_879e_9942621c3dd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerDataThreshold_abi(
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
pub struct IAccelerometerDeviceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometerDeviceId {
    type Vtable = IAccelerometerDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7eac64a9_97d5_446d_ab5a_917df9b96a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometerReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometerReading {
    type Vtable = IAccelerometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9fe7acb_d351_40af_8bb6_7aa9ae641fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometerReading2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometerReading2 {
    type Vtable = IAccelerometerReading2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a864aa2_15ae_4a40_be55_db58d7de7389);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReading2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometerReadingChangedEventArgs {
    type Vtable = IAccelerometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0095c65b_b6ac_475a_9f44_8b32d35a3f25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReadingChangedEventArgs_abi(
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
pub struct IAccelerometerShakenEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometerShakenEventArgs {
    type Vtable = IAccelerometerShakenEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95ff01d1_4a28_4f35_98e8_8178aae4084a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerShakenEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometerStatics {
    type Vtable = IAccelerometerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5e28b74_5a87_4a2d_becc_0f906ea061dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics_abi(
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
pub struct IAccelerometerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometerStatics2 {
    type Vtable = IAccelerometerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4c4842f_d86b_4685_b2d7_3396f798d57b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, readingtype: AccelerometerReadingType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccelerometerStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccelerometerStatics3 {
    type Vtable = IAccelerometerStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9de218cf_455d_4cf3_8200_70e1410340f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, readingtype: AccelerometerReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivitySensor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivitySensor {
    type Vtable = IActivitySensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd7a630c_fb5f_48eb_b09b_a2708d1c61ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivitySensorReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivitySensorReading {
    type Vtable = IActivitySensorReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85125a96_1472_40a2_b2ae_e1ef29226c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ActivityType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ActivitySensorReadingConfidence) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivitySensorReadingChangeReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivitySensorReadingChangeReport {
    type Vtable = IActivitySensorReadingChangeReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f3c2915_d93b_47bd_960a_f20fb2f322b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReadingChangeReport_abi(
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
pub struct IActivitySensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivitySensorReadingChangedEventArgs {
    type Vtable = IActivitySensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde386717_aeb6_4ec7_946a_d9cc19b951ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReadingChangedEventArgs_abi(
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
pub struct IActivitySensorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivitySensorStatics {
    type Vtable = IActivitySensorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa71e0e9d_ee8b_45d1_b25b_08cc0df92ab6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fromtime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivitySensorTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivitySensorTriggerDetails {
    type Vtable = IActivitySensorTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c9e6612_b9ca_4677_b263_243297f79d3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTriggerDetails_abi(
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
pub struct IAltimeter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAltimeter {
    type Vtable = IAltimeter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72f057fd_8f04_49f1_b4a7_f4e363b701a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAltimeter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAltimeter2 {
    type Vtable = IAltimeter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9471bf9_2add_48f5_9f08_3d0c7660d938);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAltimeterReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAltimeterReading {
    type Vtable = IAltimeterReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbe8ef73_7f5e_48c8_aa1a_f1f3befc1144);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAltimeterReading2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAltimeterReading2 {
    type Vtable = IAltimeterReading2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x543a1bd9_6d0b_42b2_bd69_bc8fae0f782c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReading2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAltimeterReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAltimeterReadingChangedEventArgs {
    type Vtable = IAltimeterReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7069d077_446d_47f7_998c_ebc23b45e4a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReadingChangedEventArgs_abi(
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
pub struct IAltimeterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAltimeterStatics {
    type Vtable = IAltimeterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9eb4d7c3_e5ac_47ce_8eef_d3718168c01f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterStatics_abi(
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
pub struct IBarometer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarometer {
    type Vtable = IBarometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x934475a8_78bf_452f_b017_f0209ce6dab4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarometer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarometer2 {
    type Vtable = IBarometer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32bcc418_3eeb_4d04_9574_7633a8781f9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarometer3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarometer3 {
    type Vtable = IBarometer3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e35f0ea_02b5_5a04_b03d_822084863a54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer3_abi(
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
pub struct IBarometerDataThreshold(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarometerDataThreshold {
    type Vtable = IBarometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076b952c_cb62_5a90_a0d1_f85e4a936394);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerDataThreshold_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarometerReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarometerReading {
    type Vtable = IBarometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b9d2e6_1df6_4a1a_a7ad_321d4f5db247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarometerReading2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarometerReading2 {
    type Vtable = IBarometerReading2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85a244eb_90c5_4875_891c_3865b4c357e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReading2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarometerReadingChangedEventArgs {
    type Vtable = IBarometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d84945f_037b_404f_9bbb_6232d69543c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReadingChangedEventArgs_abi(
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
pub struct IBarometerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarometerStatics {
    type Vtable = IBarometerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x286b270a_02e3_4f86_84fc_fdd892b5940f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerStatics_abi(
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
pub struct IBarometerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarometerStatics2 {
    type Vtable = IBarometerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fc6b1e7_95ff_44ac_878e_d65c8308c34c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompass(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompass {
    type Vtable = ICompass_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x292ffa94_1b45_403c_ba06_b106dba69a64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompass2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompass2 {
    type Vtable = ICompass2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36f26d09_c7d7_434f_b461_979ddfc2322f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompass3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompass3 {
    type Vtable = ICompass3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa424801b_c5ea_4d45_a0ec_4b791f041a89);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompass4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompass4 {
    type Vtable = ICompass4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x291e7f11_ec32_5dcc_bfcb_0bb39eba5774);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass4_abi(
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
pub struct ICompassDataThreshold(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompassDataThreshold {
    type Vtable = ICompassDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd15b52b3_d39d_5ec8_b2e4_f193e6ab34ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassDataThreshold_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompassDeviceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompassDeviceId {
    type Vtable = ICompassDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd181ca29_b085_4b1d_870a_4ff57ba74fd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompassReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompassReading {
    type Vtable = ICompassReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82911128_513d_4dc9_b781_5eedfbf02d0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompassReading2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompassReading2 {
    type Vtable = ICompassReading2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb13a661e_51bb_4a12_bedd_ad47ff87d2e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReading2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompassReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompassReadingChangedEventArgs {
    type Vtable = ICompassReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1549b0_e8bc_4c7e_b009_4e41df137072);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReadingChangedEventArgs_abi(
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
pub struct ICompassReadingHeadingAccuracy(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompassReadingHeadingAccuracy {
    type Vtable = ICompassReadingHeadingAccuracy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe761354e_8911_40f7_9e16_6ecc7daec5de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReadingHeadingAccuracy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompassStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompassStatics {
    type Vtable = ICompassStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9abc97df_56ec_4c25_b54d_40a68bb5b269);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassStatics_abi(
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
pub struct ICompassStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompassStatics2 {
    type Vtable = ICompassStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ace0ead_3baa_4990_9ce4_be0913754ed2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGyrometer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometer {
    type Vtable = IGyrometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdb9a9c4_84b1_4ca2_9763_9b589506c70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGyrometer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometer2 {
    type Vtable = IGyrometer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63df2443_8ce8_41c3_ac44_8698810b557f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGyrometer3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometer3 {
    type Vtable = IGyrometer3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d6f88d5_8fbc_4484_914b_528adfd947b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGyrometer4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometer4 {
    type Vtable = IGyrometer4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0628a60c_4c4b_5096_94e6_c356df68bef7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer4_abi(
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
pub struct IGyrometerDataThreshold(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometerDataThreshold {
    type Vtable = IGyrometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8648b31e_6e52_5259_bbad_242a69dc38c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerDataThreshold_abi(
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
pub struct IGyrometerDeviceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometerDeviceId {
    type Vtable = IGyrometerDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee5e978_89a2_4275_9e95_7126f4708760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGyrometerReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometerReading {
    type Vtable = IGyrometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3d6de5c_1ee4_456f_9de7_e2493b5c8e03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGyrometerReading2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometerReading2 {
    type Vtable = IGyrometerReading2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16afe13c_2b89_44bb_822b_d1e1556ff09b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReading2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGyrometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometerReadingChangedEventArgs {
    type Vtable = IGyrometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fdf1895_6f9e_42ce_8d58_388c0ab8356d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReadingChangedEventArgs_abi(
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
pub struct IGyrometerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometerStatics {
    type Vtable = IGyrometerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83b6e7c9_e49d_4b39_86e6_cd554be4c5c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerStatics_abi(
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
pub struct IGyrometerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGyrometerStatics2 {
    type Vtable = IGyrometerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef83f7a1_d700_4204_9613_79c6b161df4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHingeAngleReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHingeAngleReading {
    type Vtable = IHingeAngleReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3cd45b9_1bf1_4f65_a704_e2da04f182c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHingeAngleSensor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHingeAngleSensor {
    type Vtable = IHingeAngleSensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9d3be02_bfdf_437f_8c29_88c77393d309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHingeAngleSensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHingeAngleSensorReadingChangedEventArgs {
    type Vtable = IHingeAngleSensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24d9558b_fad0_42b8_a854_78923049a1ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensorReadingChangedEventArgs_abi(
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
pub struct IHingeAngleSensorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHingeAngleSensorStatics {
    type Vtable = IHingeAngleSensorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7b63910_fbb1_4123_89ce_4ea34eb0dfca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, firstpanelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, secondpanelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometer {
    type Vtable = IInclinometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2648ca6f_2286_406f_9161_f0c4bd806ebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometer2 {
    type Vtable = IInclinometer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x029f3393_28b2_45f8_bb16_61e86a7fae6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SensorReadingType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometer3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometer3 {
    type Vtable = IInclinometer3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a095004_d765_4384_a3d7_0283f3abe6ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometer4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometer4 {
    type Vtable = IInclinometer4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43852618_8fca_548e_bbf5_5c50412b6aa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer4_abi(
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
pub struct IInclinometerDataThreshold(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerDataThreshold {
    type Vtable = IInclinometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf80a4783_7bfe_545e_bb60_a0ebc47bd2fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerDataThreshold_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometerDeviceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerDeviceId {
    type Vtable = IInclinometerDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01e91982_41ff_4406_ae83_62210ff16fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometerReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerReading {
    type Vtable = IInclinometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f44f055_b6f6_497f_b127_1a775e501458);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometerReading2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerReading2 {
    type Vtable = IInclinometerReading2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f164781_e90b_4658_8915_0103e08a805a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReading2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerReadingChangedEventArgs {
    type Vtable = IInclinometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ae91dc1_e7eb_4938_8511_ae0d6b440438);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReadingChangedEventArgs_abi(
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
pub struct IInclinometerReadingYawAccuracy(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerReadingYawAccuracy {
    type Vtable = IInclinometerReadingYawAccuracy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb453e880_1fe3_4986_a257_e6ece2723949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReadingYawAccuracy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerStatics {
    type Vtable = IInclinometerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf22ec551_9c30_453a_8b49_3c3eeb33cb61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics_abi(
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
pub struct IInclinometerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerStatics2 {
    type Vtable = IInclinometerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x043f9775_6a1e_499c_86e0_638c1a864b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics2_abi(
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
pub struct IInclinometerStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerStatics3 {
    type Vtable = IInclinometerStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd9a4280_b91a_4829_9392_abc0b6bdf2b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sensorreadingtype: SensorReadingType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInclinometerStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInclinometerStatics4 {
    type Vtable = IInclinometerStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8ba96f9_6e85_4a83_aed0_d7cdcc9856c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, readingtype: SensorReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILightSensor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensor {
    type Vtable = ILightSensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf84c0718_0c54_47ae_922e_789f57fb03a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILightSensor2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensor2 {
    type Vtable = ILightSensor2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x486b24e8_a94c_4090_8f48_09f782a9f7d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILightSensor3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensor3 {
    type Vtable = ILightSensor3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4876d0ff_9f4c_5f72_adbd_a3471b063c00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor3_abi(
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
pub struct ILightSensorDataThreshold(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensorDataThreshold {
    type Vtable = ILightSensorDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb160afd1_878f_5492_9f2c_33dc3ae584a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorDataThreshold_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILightSensorDeviceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensorDeviceId {
    type Vtable = ILightSensorDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fee49f8_0afb_4f51_87f0_6c26375ce94f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILightSensorReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensorReading {
    type Vtable = ILightSensorReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffdf6300_227c_4d2b_b302_fc0142485c68);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILightSensorReading2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensorReading2 {
    type Vtable = ILightSensorReading2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7512185_44a3_44c9_8190_9ef6de0a8a74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReading2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILightSensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensorReadingChangedEventArgs {
    type Vtable = ILightSensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a2f4cf_258b_420c_b8ab_8edd601ecf50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReadingChangedEventArgs_abi(
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
pub struct ILightSensorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensorStatics {
    type Vtable = ILightSensorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45db8c84_c3a8_471e_9a53_6457fad87c0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorStatics_abi(
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
pub struct ILightSensorStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILightSensorStatics2 {
    type Vtable = ILightSensorStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ec0a650_ddc6_40ab_ace3_ec3359d42c51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagnetometer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometer {
    type Vtable = IMagnetometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x484f626e_d3c9_4111_b3f6_2cf1faa418d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagnetometer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometer2 {
    type Vtable = IMagnetometer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4656c85_26f6_444b_a9e2_a23f966cd368);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagnetometer3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometer3 {
    type Vtable = IMagnetometer3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe93db7c_a625_48ef_acf7_fac104832671);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagnetometer4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometer4 {
    type Vtable = IMagnetometer4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfb17901_3e0f_508f_b24b_f2bb75015f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer4_abi(
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
pub struct IMagnetometerDataThreshold(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometerDataThreshold {
    type Vtable = IMagnetometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd177cb01_9063_5fa5_b596_b445e9dc3401);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerDataThreshold_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagnetometerDeviceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometerDeviceId {
    type Vtable = IMagnetometerDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58b498c2_7e4b_404c_9fc5_5de8b40ebae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagnetometerReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometerReading {
    type Vtable = IMagnetometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c2cc40d_ebfd_4e5c_bb11_afc29b3cae61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagnetometerReading2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometerReading2 {
    type Vtable = IMagnetometerReading2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4c95c61_61d9_404b_a328_066f177a1409);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReading2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagnetometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometerReadingChangedEventArgs {
    type Vtable = IMagnetometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17eae872_2eb9_4ee7_8ad0_3127537d949b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReadingChangedEventArgs_abi(
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
pub struct IMagnetometerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometerStatics {
    type Vtable = IMagnetometerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x853c64cc_0698_4dda_a6df_9cb9cc4ab40a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerStatics_abi(
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
pub struct IMagnetometerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagnetometerStatics2 {
    type Vtable = IMagnetometerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c0819f0_ffc6_4f89_a06f_18fa10792933);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientationSensor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensor {
    type Vtable = IOrientationSensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e354635_cf6b_4c63_abd8_10252b0bf6ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientationSensor2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensor2 {
    type Vtable = IOrientationSensor2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d924cf9_2f1f_49c9_8042_4a1813d67760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SensorReadingType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientationSensor3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensor3 {
    type Vtable = IOrientationSensor3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cce578d_646b_48c5_b7ee_44fdc4c6aafd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientationSensorDeviceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensorDeviceId {
    type Vtable = IOrientationSensorDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a69b648_4c29_49ec_b28f_ea1d117b66f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientationSensorReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensorReading {
    type Vtable = IOrientationSensorReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4756c993_6595_4897_bcc6_d537ee757564);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientationSensorReading2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensorReading2 {
    type Vtable = IOrientationSensorReading2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00576e5f_49f8_4c05_9e07_24fac79408c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReading2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientationSensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensorReadingChangedEventArgs {
    type Vtable = IOrientationSensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x012c1186_c3ba_46bc_ae65_7a98996cbfb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReadingChangedEventArgs_abi(
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
pub struct IOrientationSensorReadingYawAccuracy(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensorReadingYawAccuracy {
    type Vtable = IOrientationSensorReadingYawAccuracy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1ac9824_3f5a_49a2_bc7b_1180bc38cd2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReadingYawAccuracy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientationSensorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensorStatics {
    type Vtable = IOrientationSensorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ef8712_fb4c_428a_898b_2765e409e669);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics_abi(
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
pub struct IOrientationSensorStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensorStatics2 {
    type Vtable = IOrientationSensorStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59da0d0b_d40a_4c71_9276_8a272a0a6619);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics2_abi(
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
pub struct IOrientationSensorStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensorStatics3 {
    type Vtable = IOrientationSensorStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd82ce920_2777_40ff_9f59_d654b085f12f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sensorreadingtype: SensorReadingType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOrientationSensorStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOrientationSensorStatics4 {
    type Vtable = IOrientationSensorStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa67feb55_2c85_4b28_a0fe_58c4b20495f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, readingtype: SensorReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPedometer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPedometer {
    type Vtable = IPedometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a1e013d_3d98_45f8_8920_8e4ecaca5f97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPedometer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPedometer2 {
    type Vtable = IPedometer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5a406df_2b81_4add_b2ff_77ab6c98ba19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometer2_abi(
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
pub struct IPedometerDataThresholdFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPedometerDataThresholdFactory {
    type Vtable = IPedometerDataThresholdFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbad8f50_7a54_466b_9010_77a162fca5d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerDataThresholdFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sensor: ::windows::core::RawPtr, stepgoal: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPedometerReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPedometerReading {
    type Vtable = IPedometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2245dcf4_a8e1_432f_896a_be0dd9b02d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PedometerStepKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPedometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPedometerReadingChangedEventArgs {
    type Vtable = IPedometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf855e47e_abbc_4456_86a8_25cf2b333742);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerReadingChangedEventArgs_abi(
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
pub struct IPedometerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPedometerStatics {
    type Vtable = IPedometerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82980a2f_4083_4dfb_b411_938ea0f4b946);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fromtime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPedometerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPedometerStatics2 {
    type Vtable = IPedometerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f5c6bb_ce0e_4133_b47e_8627ea72f677);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximitySensor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProximitySensor {
    type Vtable = IProximitySensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54c076b8_ecfb_4944_b928_74fc504d47ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximitySensorDataThresholdFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProximitySensorDataThresholdFactory {
    type Vtable = IProximitySensorDataThresholdFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905ac121_6d27_4ad3_9db5_6467f2a5ad9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorDataThresholdFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sensor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximitySensorReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProximitySensorReading {
    type Vtable = IProximitySensorReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71228d59_132d_4d5f_8ff9_2f0db8751ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximitySensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProximitySensorReadingChangedEventArgs {
    type Vtable = IProximitySensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfc2f366_c3e8_40fd_8cc3_67e289004938);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorReadingChangedEventArgs_abi(
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
pub struct IProximitySensorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProximitySensorStatics {
    type Vtable = IProximitySensorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29186649_6269_4e57_a5ad_82be80813392);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sensorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximitySensorStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProximitySensorStatics2 {
    type Vtable = IProximitySensorStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbf473ae_e9ca_422f_ad67_4c3d25df350c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISensorDataThreshold(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISensorDataThreshold {
    type Vtable = ISensorDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54daec61_fe4b_4e07_b260_3a4cdfbe396e);
}
impl ISensorDataThreshold {}
unsafe impl ::windows::core::RuntimeType for ISensorDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{54daec61-fe4b-4e07-b260-3a4cdfbe396e}");
}
impl ::core::convert::From<ISensorDataThreshold> for ::windows::core::IUnknown {
    fn from(value: ISensorDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISensorDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &ISensorDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISensorDataThreshold> for ::windows::core::IInspectable {
    fn from(value: ISensorDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISensorDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &ISensorDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThreshold_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISensorDataThresholdTriggerDetails {
    type Vtable = ISensorDataThresholdTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9106f1b7_e88d_48b1_bc90_619c7b349391);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SensorType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISensorQuaternion(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISensorQuaternion {
    type Vtable = ISensorQuaternion_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9c5c827_c71c_46e7_9da3_36a193b232bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorQuaternion_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISensorRotationMatrix(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISensorRotationMatrix {
    type Vtable = ISensorRotationMatrix_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a3d5a67_22f4_4392_9538_65d0bd064aa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorRotationMatrix_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensor {
    type Vtable = ISimpleOrientationSensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ff53856_214a_4dee_a3f9_616f1ab06ffd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SimpleOrientation) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensor2 {
    type Vtable = ISimpleOrientationSensor2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa277a798_8870_453e_8bd6_b8f5d8d7941b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
    #[cfg(feature = "Graphics_Display")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorDeviceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensorDeviceId {
    type Vtable = ISimpleOrientationSensorDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc00acb_3b76_41f6_8091_30efe646d3cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensorOrientationChangedEventArgs {
    type Vtable = ISimpleOrientationSensorOrientationChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcd5c660_23d4_4b4c_a22e_ba81ade0c601);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SimpleOrientation) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensorStatics {
    type Vtable = ISimpleOrientationSensorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72ed066f_70aa_40c6_9b1b_3433f7459b4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorStatics_abi(
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
pub struct ISimpleOrientationSensorStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensorStatics2 {
    type Vtable = ISimpleOrientationSensorStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x848f9c7f_b138_4e11_8910_a2a2a3b56d83);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Inclinometer(pub ::windows::core::IInspectable);
impl Inclinometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InclinometerReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInclinometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::Interface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__: super::super::Graphics::Display::DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn ReadingType(&self) -> ::windows::core::Result<SensorReadingType> {
        let this = &::windows::core::Interface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__: SensorReadingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SensorReadingType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IInclinometerDeviceId>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Inclinometer> {
        Self::IInclinometerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Inclinometer>(result__)
        })
    }
    pub fn GetDefaultForRelativeReadings() -> ::windows::core::Result<Inclinometer> {
        Self::IInclinometerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Inclinometer>(result__)
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> ::windows::core::Result<Inclinometer> {
        Self::IInclinometerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sensorreadingtype, &mut result__).from_abi::<Inclinometer>(result__)
        })
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInclinometer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), readingtype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Inclinometer>> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Inclinometer>>(result__)
        })
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<InclinometerDataThreshold> {
        let this = &::windows::core::Interface::cast::<IInclinometer4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InclinometerDataThreshold>(result__)
        }
    }
    pub fn IInclinometerStatics<R, F: FnOnce(&IInclinometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Inclinometer, IInclinometerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInclinometerStatics2<R, F: FnOnce(&IInclinometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Inclinometer, IInclinometerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInclinometerStatics3<R, F: FnOnce(&IInclinometerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Inclinometer, IInclinometerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInclinometerStatics4<R, F: FnOnce(&IInclinometerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Inclinometer, IInclinometerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Inclinometer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Inclinometer;{2648ca6f-2286-406f-9161-f0c4bd806ebf})");
}
unsafe impl ::windows::core::Interface for Inclinometer {
    type Vtable = IInclinometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2648ca6f_2286_406f_9161_f0c4bd806ebf);
}
impl ::windows::core::RuntimeName for Inclinometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Inclinometer";
}
impl ::core::convert::From<Inclinometer> for ::windows::core::IUnknown {
    fn from(value: Inclinometer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Inclinometer> for ::windows::core::IUnknown {
    fn from(value: &Inclinometer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Inclinometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Inclinometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Inclinometer> for ::windows::core::IInspectable {
    fn from(value: Inclinometer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Inclinometer> for ::windows::core::IInspectable {
    fn from(value: &Inclinometer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Inclinometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Inclinometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Inclinometer {}
unsafe impl ::core::marker::Sync for Inclinometer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InclinometerDataThreshold(pub ::windows::core::IInspectable);
impl InclinometerDataThreshold {
    pub fn PitchInDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetPitchInDegrees(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RollInDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetRollInDegrees(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn YawInDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetYawInDegrees(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InclinometerDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.InclinometerDataThreshold;{f80a4783-7bfe-545e-bb60-a0ebc47bd2fb})");
}
unsafe impl ::windows::core::Interface for InclinometerDataThreshold {
    type Vtable = IInclinometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf80a4783_7bfe_545e_bb60_a0ebc47bd2fb);
}
impl ::windows::core::RuntimeName for InclinometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerDataThreshold";
}
impl ::core::convert::From<InclinometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: InclinometerDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InclinometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &InclinometerDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InclinometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InclinometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InclinometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: InclinometerDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InclinometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &InclinometerDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InclinometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InclinometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InclinometerDataThreshold {}
unsafe impl ::core::marker::Sync for InclinometerDataThreshold {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InclinometerReading(pub ::windows::core::IInspectable);
impl InclinometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn PitchDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn RollDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn YawDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn YawAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy> {
        let this = &::windows::core::Interface::cast::<IInclinometerReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__: MagnetometerAccuracy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagnetometerAccuracy>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InclinometerReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.InclinometerReading;{9f44f055-b6f6-497f-b127-1a775e501458})");
}
unsafe impl ::windows::core::Interface for InclinometerReading {
    type Vtable = IInclinometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f44f055_b6f6_497f_b127_1a775e501458);
}
impl ::windows::core::RuntimeName for InclinometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReading";
}
impl ::core::convert::From<InclinometerReading> for ::windows::core::IUnknown {
    fn from(value: InclinometerReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InclinometerReading> for ::windows::core::IUnknown {
    fn from(value: &InclinometerReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InclinometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InclinometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InclinometerReading> for ::windows::core::IInspectable {
    fn from(value: InclinometerReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InclinometerReading> for ::windows::core::IInspectable {
    fn from(value: &InclinometerReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InclinometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InclinometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InclinometerReading {}
unsafe impl ::core::marker::Sync for InclinometerReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InclinometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl InclinometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InclinometerReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InclinometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.InclinometerReadingChangedEventArgs;{4ae91dc1-e7eb-4938-8511-ae0d6b440438})");
}
unsafe impl ::windows::core::Interface for InclinometerReadingChangedEventArgs {
    type Vtable = IInclinometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ae91dc1_e7eb_4938_8511_ae0d6b440438);
}
impl ::windows::core::RuntimeName for InclinometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReadingChangedEventArgs";
}
impl ::core::convert::From<InclinometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: InclinometerReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InclinometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InclinometerReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InclinometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InclinometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InclinometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: InclinometerReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InclinometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InclinometerReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InclinometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InclinometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InclinometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for InclinometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LightSensor(pub ::windows::core::IInspectable);
impl LightSensor {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LightSensorReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILightSensorDeviceId>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<LightSensor> {
        Self::ILightSensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LightSensor>(result__)
        })
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILightSensor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LightSensor>> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LightSensor>>(result__)
        })
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<LightSensorDataThreshold> {
        let this = &::windows::core::Interface::cast::<ILightSensor3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LightSensorDataThreshold>(result__)
        }
    }
    pub fn ILightSensorStatics<R, F: FnOnce(&ILightSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LightSensor, ILightSensorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILightSensorStatics2<R, F: FnOnce(&ILightSensorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LightSensor, ILightSensorStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LightSensor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensor;{f84c0718-0c54-47ae-922e-789f57fb03a0})");
}
unsafe impl ::windows::core::Interface for LightSensor {
    type Vtable = ILightSensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf84c0718_0c54_47ae_922e_789f57fb03a0);
}
impl ::windows::core::RuntimeName for LightSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensor";
}
impl ::core::convert::From<LightSensor> for ::windows::core::IUnknown {
    fn from(value: LightSensor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LightSensor> for ::windows::core::IUnknown {
    fn from(value: &LightSensor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LightSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LightSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LightSensor> for ::windows::core::IInspectable {
    fn from(value: LightSensor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LightSensor> for ::windows::core::IInspectable {
    fn from(value: &LightSensor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LightSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LightSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LightSensor {}
unsafe impl ::core::marker::Sync for LightSensor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LightSensorDataThreshold(pub ::windows::core::IInspectable);
impl LightSensorDataThreshold {
    pub fn LuxPercentage(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetLuxPercentage(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AbsoluteLux(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetAbsoluteLux(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LightSensorDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensorDataThreshold;{b160afd1-878f-5492-9f2c-33dc3ae584a3})");
}
unsafe impl ::windows::core::Interface for LightSensorDataThreshold {
    type Vtable = ILightSensorDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb160afd1_878f_5492_9f2c_33dc3ae584a3);
}
impl ::windows::core::RuntimeName for LightSensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorDataThreshold";
}
impl ::core::convert::From<LightSensorDataThreshold> for ::windows::core::IUnknown {
    fn from(value: LightSensorDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LightSensorDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &LightSensorDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LightSensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LightSensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LightSensorDataThreshold> for ::windows::core::IInspectable {
    fn from(value: LightSensorDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LightSensorDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &LightSensorDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LightSensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LightSensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LightSensorDataThreshold {}
unsafe impl ::core::marker::Sync for LightSensorDataThreshold {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LightSensorReading(pub ::windows::core::IInspectable);
impl LightSensorReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn IlluminanceInLux(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LightSensorReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensorReading;{ffdf6300-227c-4d2b-b302-fc0142485c68})");
}
unsafe impl ::windows::core::Interface for LightSensorReading {
    type Vtable = ILightSensorReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffdf6300_227c_4d2b_b302_fc0142485c68);
}
impl ::windows::core::RuntimeName for LightSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReading";
}
impl ::core::convert::From<LightSensorReading> for ::windows::core::IUnknown {
    fn from(value: LightSensorReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LightSensorReading> for ::windows::core::IUnknown {
    fn from(value: &LightSensorReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LightSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LightSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LightSensorReading> for ::windows::core::IInspectable {
    fn from(value: LightSensorReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LightSensorReading> for ::windows::core::IInspectable {
    fn from(value: &LightSensorReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LightSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LightSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LightSensorReading {}
unsafe impl ::core::marker::Sync for LightSensorReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LightSensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl LightSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LightSensorReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LightSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensorReadingChangedEventArgs;{a3a2f4cf-258b-420c-b8ab-8edd601ecf50})");
}
unsafe impl ::windows::core::Interface for LightSensorReadingChangedEventArgs {
    type Vtable = ILightSensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a2f4cf_258b_420c_b8ab_8edd601ecf50);
}
impl ::windows::core::RuntimeName for LightSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReadingChangedEventArgs";
}
impl ::core::convert::From<LightSensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LightSensorReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LightSensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LightSensorReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LightSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LightSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LightSensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LightSensorReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LightSensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LightSensorReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LightSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LightSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LightSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for LightSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Magnetometer(pub ::windows::core::IInspectable);
impl Magnetometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagnetometerReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMagnetometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::Interface::cast::<IMagnetometer2>(self)?;
        unsafe {
            let mut result__: super::super::Graphics::Display::DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMagnetometerDeviceId>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Magnetometer> {
        Self::IMagnetometerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Magnetometer>(result__)
        })
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Magnetometer>> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Magnetometer>>(result__)
        })
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<MagnetometerDataThreshold> {
        let this = &::windows::core::Interface::cast::<IMagnetometer4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagnetometerDataThreshold>(result__)
        }
    }
    pub fn IMagnetometerStatics<R, F: FnOnce(&IMagnetometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Magnetometer, IMagnetometerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMagnetometerStatics2<R, F: FnOnce(&IMagnetometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Magnetometer, IMagnetometerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Magnetometer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Magnetometer;{484f626e-d3c9-4111-b3f6-2cf1faa418d5})");
}
unsafe impl ::windows::core::Interface for Magnetometer {
    type Vtable = IMagnetometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x484f626e_d3c9_4111_b3f6_2cf1faa418d5);
}
impl ::windows::core::RuntimeName for Magnetometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Magnetometer";
}
impl ::core::convert::From<Magnetometer> for ::windows::core::IUnknown {
    fn from(value: Magnetometer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Magnetometer> for ::windows::core::IUnknown {
    fn from(value: &Magnetometer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Magnetometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Magnetometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Magnetometer> for ::windows::core::IInspectable {
    fn from(value: Magnetometer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Magnetometer> for ::windows::core::IInspectable {
    fn from(value: &Magnetometer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Magnetometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Magnetometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Magnetometer {}
unsafe impl ::core::marker::Sync for Magnetometer {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagnetometerAccuracy(pub i32);
impl MagnetometerAccuracy {
    pub const Unknown: MagnetometerAccuracy = MagnetometerAccuracy(0i32);
    pub const Unreliable: MagnetometerAccuracy = MagnetometerAccuracy(1i32);
    pub const Approximate: MagnetometerAccuracy = MagnetometerAccuracy(2i32);
    pub const High: MagnetometerAccuracy = MagnetometerAccuracy(3i32);
}
impl ::core::convert::From<i32> for MagnetometerAccuracy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MagnetometerAccuracy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MagnetometerAccuracy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.MagnetometerAccuracy;i4)");
}
impl ::windows::core::DefaultType for MagnetometerAccuracy {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagnetometerDataThreshold(pub ::windows::core::IInspectable);
impl MagnetometerDataThreshold {
    pub fn XAxisMicroteslas(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetXAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn YAxisMicroteslas(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetYAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ZAxisMicroteslas(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetZAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MagnetometerDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.MagnetometerDataThreshold;{d177cb01-9063-5fa5-b596-b445e9dc3401})");
}
unsafe impl ::windows::core::Interface for MagnetometerDataThreshold {
    type Vtable = IMagnetometerDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd177cb01_9063_5fa5_b596_b445e9dc3401);
}
impl ::windows::core::RuntimeName for MagnetometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerDataThreshold";
}
impl ::core::convert::From<MagnetometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: MagnetometerDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagnetometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &MagnetometerDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagnetometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagnetometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagnetometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: MagnetometerDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagnetometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &MagnetometerDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagnetometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagnetometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagnetometerDataThreshold {}
unsafe impl ::core::marker::Sync for MagnetometerDataThreshold {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagnetometerReading(pub ::windows::core::IInspectable);
impl MagnetometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn MagneticFieldX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn MagneticFieldY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn MagneticFieldZ(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn DirectionalAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy> {
        let this = self;
        unsafe {
            let mut result__: MagnetometerAccuracy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagnetometerAccuracy>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagnetometerReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.MagnetometerReading;{0c2cc40d-ebfd-4e5c-bb11-afc29b3cae61})");
}
unsafe impl ::windows::core::Interface for MagnetometerReading {
    type Vtable = IMagnetometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c2cc40d_ebfd_4e5c_bb11_afc29b3cae61);
}
impl ::windows::core::RuntimeName for MagnetometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReading";
}
impl ::core::convert::From<MagnetometerReading> for ::windows::core::IUnknown {
    fn from(value: MagnetometerReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagnetometerReading> for ::windows::core::IUnknown {
    fn from(value: &MagnetometerReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagnetometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagnetometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagnetometerReading> for ::windows::core::IInspectable {
    fn from(value: MagnetometerReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagnetometerReading> for ::windows::core::IInspectable {
    fn from(value: &MagnetometerReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagnetometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagnetometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagnetometerReading {}
unsafe impl ::core::marker::Sync for MagnetometerReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagnetometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl MagnetometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagnetometerReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagnetometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.MagnetometerReadingChangedEventArgs;{17eae872-2eb9-4ee7-8ad0-3127537d949b})");
}
unsafe impl ::windows::core::Interface for MagnetometerReadingChangedEventArgs {
    type Vtable = IMagnetometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17eae872_2eb9_4ee7_8ad0_3127537d949b);
}
impl ::windows::core::RuntimeName for MagnetometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReadingChangedEventArgs";
}
impl ::core::convert::From<MagnetometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagnetometerReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagnetometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagnetometerReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagnetometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagnetometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagnetometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagnetometerReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagnetometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagnetometerReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagnetometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagnetometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagnetometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for MagnetometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OrientationSensor(pub ::windows::core::IInspectable);
impl OrientationSensor {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OrientationSensorReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__: super::super::Graphics::Display::DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn ReadingType(&self) -> ::windows::core::Result<SensorReadingType> {
        let this = &::windows::core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__: SensorReadingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SensorReadingType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OrientationSensor>(result__)
        })
    }
    pub fn GetDefaultForRelativeReadings() -> ::windows::core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OrientationSensor>(result__)
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> ::windows::core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sensorreadingtype, &mut result__).from_abi::<OrientationSensor>(result__)
        })
    }
    pub fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal(sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sensorreadingtype, optimizationgoal, &mut result__).from_abi::<OrientationSensor>(result__)
        })
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), readingtype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal(readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), readingtype, optimizationgoal, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<OrientationSensor>> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<OrientationSensor>>(result__)
        })
    }
    pub fn IOrientationSensorStatics<R, F: FnOnce(&IOrientationSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OrientationSensor, IOrientationSensorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IOrientationSensorStatics2<R, F: FnOnce(&IOrientationSensorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OrientationSensor, IOrientationSensorStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IOrientationSensorStatics3<R, F: FnOnce(&IOrientationSensorStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OrientationSensor, IOrientationSensorStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IOrientationSensorStatics4<R, F: FnOnce(&IOrientationSensorStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OrientationSensor, IOrientationSensorStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for OrientationSensor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.OrientationSensor;{5e354635-cf6b-4c63-abd8-10252b0bf6ec})");
}
unsafe impl ::windows::core::Interface for OrientationSensor {
    type Vtable = IOrientationSensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e354635_cf6b_4c63_abd8_10252b0bf6ec);
}
impl ::windows::core::RuntimeName for OrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensor";
}
impl ::core::convert::From<OrientationSensor> for ::windows::core::IUnknown {
    fn from(value: OrientationSensor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OrientationSensor> for ::windows::core::IUnknown {
    fn from(value: &OrientationSensor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OrientationSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OrientationSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OrientationSensor> for ::windows::core::IInspectable {
    fn from(value: OrientationSensor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OrientationSensor> for ::windows::core::IInspectable {
    fn from(value: &OrientationSensor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OrientationSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OrientationSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OrientationSensor {}
unsafe impl ::core::marker::Sync for OrientationSensor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OrientationSensorReading(pub ::windows::core::IInspectable);
impl OrientationSensorReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn RotationMatrix(&self) -> ::windows::core::Result<SensorRotationMatrix> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SensorRotationMatrix>(result__)
        }
    }
    pub fn Quaternion(&self) -> ::windows::core::Result<SensorQuaternion> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SensorQuaternion>(result__)
        }
    }
    pub fn YawAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy> {
        let this = &::windows::core::Interface::cast::<IOrientationSensorReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__: MagnetometerAccuracy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagnetometerAccuracy>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OrientationSensorReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.OrientationSensorReading;{4756c993-6595-4897-bcc6-d537ee757564})");
}
unsafe impl ::windows::core::Interface for OrientationSensorReading {
    type Vtable = IOrientationSensorReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4756c993_6595_4897_bcc6_d537ee757564);
}
impl ::windows::core::RuntimeName for OrientationSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReading";
}
impl ::core::convert::From<OrientationSensorReading> for ::windows::core::IUnknown {
    fn from(value: OrientationSensorReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OrientationSensorReading> for ::windows::core::IUnknown {
    fn from(value: &OrientationSensorReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OrientationSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OrientationSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OrientationSensorReading> for ::windows::core::IInspectable {
    fn from(value: OrientationSensorReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OrientationSensorReading> for ::windows::core::IInspectable {
    fn from(value: &OrientationSensorReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OrientationSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OrientationSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OrientationSensorReading {}
unsafe impl ::core::marker::Sync for OrientationSensorReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OrientationSensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl OrientationSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OrientationSensorReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OrientationSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.OrientationSensorReadingChangedEventArgs;{012c1186-c3ba-46bc-ae65-7a98996cbfb8})");
}
unsafe impl ::windows::core::Interface for OrientationSensorReadingChangedEventArgs {
    type Vtable = IOrientationSensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x012c1186_c3ba_46bc_ae65_7a98996cbfb8);
}
impl ::windows::core::RuntimeName for OrientationSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReadingChangedEventArgs";
}
impl ::core::convert::From<OrientationSensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: OrientationSensorReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OrientationSensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &OrientationSensorReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OrientationSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OrientationSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OrientationSensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: OrientationSensorReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OrientationSensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &OrientationSensorReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OrientationSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OrientationSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OrientationSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for OrientationSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Pedometer(pub ::windows::core::IInspectable);
impl Pedometer {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PowerInMilliwatts(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Pedometer>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Pedometer>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSystemHistoryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(fromtime: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), fromtime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSystemHistoryWithDurationAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(fromtime: Param0, duration: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), fromtime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentReadings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>> {
        let this = &::windows::core::Interface::cast::<IPedometer2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetReadingsFromTriggerDetails<'a, Param0: ::windows::core::IntoParam<'a, SensorDataThresholdTriggerDetails>>(triggerdetails: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PedometerReading>> {
        Self::IPedometerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggerdetails.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PedometerReading>>(result__)
        })
    }
    pub fn IPedometerStatics<R, F: FnOnce(&IPedometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Pedometer, IPedometerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPedometerStatics2<R, F: FnOnce(&IPedometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Pedometer, IPedometerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Pedometer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Pedometer;{9a1e013d-3d98-45f8-8920-8e4ecaca5f97})");
}
unsafe impl ::windows::core::Interface for Pedometer {
    type Vtable = IPedometer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a1e013d_3d98_45f8_8920_8e4ecaca5f97);
}
impl ::windows::core::RuntimeName for Pedometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Pedometer";
}
impl ::core::convert::From<Pedometer> for ::windows::core::IUnknown {
    fn from(value: Pedometer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Pedometer> for ::windows::core::IUnknown {
    fn from(value: &Pedometer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Pedometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Pedometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Pedometer> for ::windows::core::IInspectable {
    fn from(value: Pedometer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Pedometer> for ::windows::core::IInspectable {
    fn from(value: &Pedometer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Pedometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Pedometer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Pedometer {}
unsafe impl ::core::marker::Sync for Pedometer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PedometerDataThreshold(pub ::windows::core::IInspectable);
impl PedometerDataThreshold {
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, Pedometer>>(sensor: Param0, stepgoal: i32) -> ::windows::core::Result<PedometerDataThreshold> {
        Self::IPedometerDataThresholdFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sensor.into_param().abi(), stepgoal, &mut result__).from_abi::<PedometerDataThreshold>(result__)
        })
    }
    pub fn IPedometerDataThresholdFactory<R, F: FnOnce(&IPedometerDataThresholdFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PedometerDataThreshold, IPedometerDataThresholdFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PedometerDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.PedometerDataThreshold;{54daec61-fe4b-4e07-b260-3a4cdfbe396e})");
}
unsafe impl ::windows::core::Interface for PedometerDataThreshold {
    type Vtable = ISensorDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54daec61_fe4b_4e07_b260_3a4cdfbe396e);
}
impl ::windows::core::RuntimeName for PedometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerDataThreshold";
}
impl ::core::convert::From<PedometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: PedometerDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PedometerDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &PedometerDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PedometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PedometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PedometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: PedometerDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PedometerDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &PedometerDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PedometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PedometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PedometerDataThreshold> for ISensorDataThreshold {
    fn from(value: PedometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PedometerDataThreshold> for ISensorDataThreshold {
    fn from(value: &PedometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISensorDataThreshold> for PedometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ISensorDataThreshold> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISensorDataThreshold> for &PedometerDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ISensorDataThreshold> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PedometerDataThreshold {}
unsafe impl ::core::marker::Sync for PedometerDataThreshold {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PedometerReading(pub ::windows::core::IInspectable);
impl PedometerReading {
    pub fn StepKind(&self) -> ::windows::core::Result<PedometerStepKind> {
        let this = self;
        unsafe {
            let mut result__: PedometerStepKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PedometerStepKind>(result__)
        }
    }
    pub fn CumulativeSteps(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CumulativeStepsDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PedometerReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.PedometerReading;{2245dcf4-a8e1-432f-896a-be0dd9b02d24})");
}
unsafe impl ::windows::core::Interface for PedometerReading {
    type Vtable = IPedometerReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2245dcf4_a8e1_432f_896a_be0dd9b02d24);
}
impl ::windows::core::RuntimeName for PedometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReading";
}
impl ::core::convert::From<PedometerReading> for ::windows::core::IUnknown {
    fn from(value: PedometerReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PedometerReading> for ::windows::core::IUnknown {
    fn from(value: &PedometerReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PedometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PedometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PedometerReading> for ::windows::core::IInspectable {
    fn from(value: PedometerReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PedometerReading> for ::windows::core::IInspectable {
    fn from(value: &PedometerReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PedometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PedometerReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PedometerReading {}
unsafe impl ::core::marker::Sync for PedometerReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PedometerReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl PedometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<PedometerReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PedometerReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PedometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.PedometerReadingChangedEventArgs;{f855e47e-abbc-4456-86a8-25cf2b333742})");
}
unsafe impl ::windows::core::Interface for PedometerReadingChangedEventArgs {
    type Vtable = IPedometerReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf855e47e_abbc_4456_86a8_25cf2b333742);
}
impl ::windows::core::RuntimeName for PedometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReadingChangedEventArgs";
}
impl ::core::convert::From<PedometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PedometerReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PedometerReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PedometerReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PedometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PedometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PedometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PedometerReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PedometerReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PedometerReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PedometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PedometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PedometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for PedometerReadingChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PedometerStepKind(pub i32);
impl PedometerStepKind {
    pub const Unknown: PedometerStepKind = PedometerStepKind(0i32);
    pub const Walking: PedometerStepKind = PedometerStepKind(1i32);
    pub const Running: PedometerStepKind = PedometerStepKind(2i32);
}
impl ::core::convert::From<i32> for PedometerStepKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PedometerStepKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PedometerStepKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.PedometerStepKind;i4)");
}
impl ::windows::core::DefaultType for PedometerStepKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProximitySensor(pub ::windows::core::IInspectable);
impl ProximitySensor {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MinDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProximitySensorReading>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateDisplayOnOffController(&self) -> ::windows::core::Result<ProximitySensorDisplayOnOffController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProximitySensorDisplayOnOffController>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sensorid: Param0) -> ::windows::core::Result<ProximitySensor> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sensorid.into_param().abi(), &mut result__).from_abi::<ProximitySensor>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetReadingsFromTriggerDetails<'a, Param0: ::windows::core::IntoParam<'a, SensorDataThresholdTriggerDetails>>(triggerdetails: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>> {
        Self::IProximitySensorStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggerdetails.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>>(result__)
        })
    }
    pub fn IProximitySensorStatics<R, F: FnOnce(&IProximitySensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProximitySensor, IProximitySensorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProximitySensorStatics2<R, F: FnOnce(&IProximitySensorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProximitySensor, IProximitySensorStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ProximitySensor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensor;{54c076b8-ecfb-4944-b928-74fc504d47ee})");
}
unsafe impl ::windows::core::Interface for ProximitySensor {
    type Vtable = IProximitySensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54c076b8_ecfb_4944_b928_74fc504d47ee);
}
impl ::windows::core::RuntimeName for ProximitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensor";
}
impl ::core::convert::From<ProximitySensor> for ::windows::core::IUnknown {
    fn from(value: ProximitySensor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProximitySensor> for ::windows::core::IUnknown {
    fn from(value: &ProximitySensor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProximitySensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProximitySensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProximitySensor> for ::windows::core::IInspectable {
    fn from(value: ProximitySensor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProximitySensor> for ::windows::core::IInspectable {
    fn from(value: &ProximitySensor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProximitySensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProximitySensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProximitySensor {}
unsafe impl ::core::marker::Sync for ProximitySensor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProximitySensorDataThreshold(pub ::windows::core::IInspectable);
impl ProximitySensorDataThreshold {
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ProximitySensor>>(sensor: Param0) -> ::windows::core::Result<ProximitySensorDataThreshold> {
        Self::IProximitySensorDataThresholdFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sensor.into_param().abi(), &mut result__).from_abi::<ProximitySensorDataThreshold>(result__)
        })
    }
    pub fn IProximitySensorDataThresholdFactory<R, F: FnOnce(&IProximitySensorDataThresholdFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProximitySensorDataThreshold, IProximitySensorDataThresholdFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ProximitySensorDataThreshold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorDataThreshold;{54daec61-fe4b-4e07-b260-3a4cdfbe396e})");
}
unsafe impl ::windows::core::Interface for ProximitySensorDataThreshold {
    type Vtable = ISensorDataThreshold_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54daec61_fe4b_4e07_b260_3a4cdfbe396e);
}
impl ::windows::core::RuntimeName for ProximitySensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDataThreshold";
}
impl ::core::convert::From<ProximitySensorDataThreshold> for ::windows::core::IUnknown {
    fn from(value: ProximitySensorDataThreshold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProximitySensorDataThreshold> for ::windows::core::IUnknown {
    fn from(value: &ProximitySensorDataThreshold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProximitySensorDataThreshold> for ::windows::core::IInspectable {
    fn from(value: ProximitySensorDataThreshold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProximitySensorDataThreshold> for ::windows::core::IInspectable {
    fn from(value: &ProximitySensorDataThreshold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ProximitySensorDataThreshold> for ISensorDataThreshold {
    fn from(value: ProximitySensorDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximitySensorDataThreshold> for ISensorDataThreshold {
    fn from(value: &ProximitySensorDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISensorDataThreshold> for ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ISensorDataThreshold> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISensorDataThreshold> for &ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows::core::Param<'a, ISensorDataThreshold> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProximitySensorDataThreshold {}
unsafe impl ::core::marker::Sync for ProximitySensorDataThreshold {}
#[cfg(feature = "Foundation")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProximitySensorDisplayOnOffController(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ProximitySensorDisplayOnOffController {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for ProximitySensorDisplayOnOffController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorDisplayOnOffController;{30d5a829-7fa4-4026-83bb-d75bae4ea99e})");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for ProximitySensorDisplayOnOffController {
    type Vtable = super::super::Foundation::IClosable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30d5a829_7fa4_4026_83bb_d75bae4ea99e);
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ProximitySensorDisplayOnOffController {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDisplayOnOffController";
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<ProximitySensorDisplayOnOffController> for ::windows::core::IUnknown {
    fn from(value: ProximitySensorDisplayOnOffController) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&ProximitySensorDisplayOnOffController> for ::windows::core::IUnknown {
    fn from(value: &ProximitySensorDisplayOnOffController) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<ProximitySensorDisplayOnOffController> for ::windows::core::IInspectable {
    fn from(value: ProximitySensorDisplayOnOffController) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&ProximitySensorDisplayOnOffController> for ::windows::core::IInspectable {
    fn from(value: &ProximitySensorDisplayOnOffController) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<ProximitySensorDisplayOnOffController> for super::super::Foundation::IClosable {
    fn from(value: ProximitySensorDisplayOnOffController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&ProximitySensorDisplayOnOffController> for super::super::Foundation::IClosable {
    fn from(value: &ProximitySensorDisplayOnOffController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for ProximitySensorDisplayOnOffController {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for ProximitySensorDisplayOnOffController {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProximitySensorReading(pub ::windows::core::IInspectable);
impl ProximitySensorReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn IsDetected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProximitySensorReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorReading;{71228d59-132d-4d5f-8ff9-2f0db8751ced})");
}
unsafe impl ::windows::core::Interface for ProximitySensorReading {
    type Vtable = IProximitySensorReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71228d59_132d_4d5f_8ff9_2f0db8751ced);
}
impl ::windows::core::RuntimeName for ProximitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReading";
}
impl ::core::convert::From<ProximitySensorReading> for ::windows::core::IUnknown {
    fn from(value: ProximitySensorReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProximitySensorReading> for ::windows::core::IUnknown {
    fn from(value: &ProximitySensorReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProximitySensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProximitySensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProximitySensorReading> for ::windows::core::IInspectable {
    fn from(value: ProximitySensorReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProximitySensorReading> for ::windows::core::IInspectable {
    fn from(value: &ProximitySensorReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProximitySensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProximitySensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProximitySensorReading {}
unsafe impl ::core::marker::Sync for ProximitySensorReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProximitySensorReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl ProximitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProximitySensorReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProximitySensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorReadingChangedEventArgs;{cfc2f366-c3e8-40fd-8cc3-67e289004938})");
}
unsafe impl ::windows::core::Interface for ProximitySensorReadingChangedEventArgs {
    type Vtable = IProximitySensorReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfc2f366_c3e8_40fd_8cc3_67e289004938);
}
impl ::windows::core::RuntimeName for ProximitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReadingChangedEventArgs";
}
impl ::core::convert::From<ProximitySensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProximitySensorReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProximitySensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProximitySensorReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProximitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProximitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProximitySensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProximitySensorReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProximitySensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProximitySensorReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProximitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProximitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProximitySensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for ProximitySensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SensorDataThresholdTriggerDetails(pub ::windows::core::IInspectable);
impl SensorDataThresholdTriggerDetails {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SensorType(&self) -> ::windows::core::Result<SensorType> {
        let this = self;
        unsafe {
            let mut result__: SensorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SensorType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SensorDataThresholdTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SensorDataThresholdTriggerDetails;{9106f1b7-e88d-48b1-bc90-619c7b349391})");
}
unsafe impl ::windows::core::Interface for SensorDataThresholdTriggerDetails {
    type Vtable = ISensorDataThresholdTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9106f1b7_e88d_48b1_bc90_619c7b349391);
}
impl ::windows::core::RuntimeName for SensorDataThresholdTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorDataThresholdTriggerDetails";
}
impl ::core::convert::From<SensorDataThresholdTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: SensorDataThresholdTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SensorDataThresholdTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &SensorDataThresholdTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SensorDataThresholdTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SensorDataThresholdTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SensorDataThresholdTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: SensorDataThresholdTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SensorDataThresholdTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &SensorDataThresholdTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SensorDataThresholdTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SensorDataThresholdTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SensorDataThresholdTriggerDetails {}
unsafe impl ::core::marker::Sync for SensorDataThresholdTriggerDetails {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SensorOptimizationGoal(pub i32);
impl SensorOptimizationGoal {
    pub const Precision: SensorOptimizationGoal = SensorOptimizationGoal(0i32);
    pub const PowerEfficiency: SensorOptimizationGoal = SensorOptimizationGoal(1i32);
}
impl ::core::convert::From<i32> for SensorOptimizationGoal {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SensorOptimizationGoal {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SensorOptimizationGoal {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorOptimizationGoal;i4)");
}
impl ::windows::core::DefaultType for SensorOptimizationGoal {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SensorQuaternion(pub ::windows::core::IInspectable);
impl SensorQuaternion {
    pub fn W(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn X(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Y(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Z(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SensorQuaternion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SensorQuaternion;{c9c5c827-c71c-46e7-9da3-36a193b232bc})");
}
unsafe impl ::windows::core::Interface for SensorQuaternion {
    type Vtable = ISensorQuaternion_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9c5c827_c71c_46e7_9da3_36a193b232bc);
}
impl ::windows::core::RuntimeName for SensorQuaternion {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorQuaternion";
}
impl ::core::convert::From<SensorQuaternion> for ::windows::core::IUnknown {
    fn from(value: SensorQuaternion) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SensorQuaternion> for ::windows::core::IUnknown {
    fn from(value: &SensorQuaternion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SensorQuaternion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SensorQuaternion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SensorQuaternion> for ::windows::core::IInspectable {
    fn from(value: SensorQuaternion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SensorQuaternion> for ::windows::core::IInspectable {
    fn from(value: &SensorQuaternion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SensorQuaternion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SensorQuaternion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SensorQuaternion {}
unsafe impl ::core::marker::Sync for SensorQuaternion {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SensorReadingType(pub i32);
impl SensorReadingType {
    pub const Absolute: SensorReadingType = SensorReadingType(0i32);
    pub const Relative: SensorReadingType = SensorReadingType(1i32);
}
impl ::core::convert::From<i32> for SensorReadingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SensorReadingType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SensorReadingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorReadingType;i4)");
}
impl ::windows::core::DefaultType for SensorReadingType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SensorRotationMatrix(pub ::windows::core::IInspectable);
impl SensorRotationMatrix {
    pub fn M11(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn M12(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn M13(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn M21(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn M22(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn M23(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn M31(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn M32(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn M33(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SensorRotationMatrix {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SensorRotationMatrix;{0a3d5a67-22f4-4392-9538-65d0bd064aa6})");
}
unsafe impl ::windows::core::Interface for SensorRotationMatrix {
    type Vtable = ISensorRotationMatrix_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a3d5a67_22f4_4392_9538_65d0bd064aa6);
}
impl ::windows::core::RuntimeName for SensorRotationMatrix {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorRotationMatrix";
}
impl ::core::convert::From<SensorRotationMatrix> for ::windows::core::IUnknown {
    fn from(value: SensorRotationMatrix) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SensorRotationMatrix> for ::windows::core::IUnknown {
    fn from(value: &SensorRotationMatrix) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SensorRotationMatrix {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SensorRotationMatrix {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SensorRotationMatrix> for ::windows::core::IInspectable {
    fn from(value: SensorRotationMatrix) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SensorRotationMatrix> for ::windows::core::IInspectable {
    fn from(value: &SensorRotationMatrix) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SensorRotationMatrix {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SensorRotationMatrix {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SensorRotationMatrix {}
unsafe impl ::core::marker::Sync for SensorRotationMatrix {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SensorType(pub i32);
impl SensorType {
    pub const Accelerometer: SensorType = SensorType(0i32);
    pub const ActivitySensor: SensorType = SensorType(1i32);
    pub const Barometer: SensorType = SensorType(2i32);
    pub const Compass: SensorType = SensorType(3i32);
    pub const CustomSensor: SensorType = SensorType(4i32);
    pub const Gyroscope: SensorType = SensorType(5i32);
    pub const ProximitySensor: SensorType = SensorType(6i32);
    pub const Inclinometer: SensorType = SensorType(7i32);
    pub const LightSensor: SensorType = SensorType(8i32);
    pub const OrientationSensor: SensorType = SensorType(9i32);
    pub const Pedometer: SensorType = SensorType(10i32);
    pub const RelativeInclinometer: SensorType = SensorType(11i32);
    pub const RelativeOrientationSensor: SensorType = SensorType(12i32);
    pub const SimpleOrientationSensor: SensorType = SensorType(13i32);
}
impl ::core::convert::From<i32> for SensorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SensorType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SensorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorType;i4)");
}
impl ::windows::core::DefaultType for SensorType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SimpleOrientation(pub i32);
impl SimpleOrientation {
    pub const NotRotated: SimpleOrientation = SimpleOrientation(0i32);
    pub const Rotated90DegreesCounterclockwise: SimpleOrientation = SimpleOrientation(1i32);
    pub const Rotated180DegreesCounterclockwise: SimpleOrientation = SimpleOrientation(2i32);
    pub const Rotated270DegreesCounterclockwise: SimpleOrientation = SimpleOrientation(3i32);
    pub const Faceup: SimpleOrientation = SimpleOrientation(4i32);
    pub const Facedown: SimpleOrientation = SimpleOrientation(5i32);
}
impl ::core::convert::From<i32> for SimpleOrientation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SimpleOrientation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SimpleOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SimpleOrientation;i4)");
}
impl ::windows::core::DefaultType for SimpleOrientation {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SimpleOrientationSensor(pub ::windows::core::IInspectable);
impl SimpleOrientationSensor {
    pub fn GetCurrentOrientation(&self) -> ::windows::core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__: SimpleOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SimpleOrientation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OrientationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveOrientationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::Interface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe {
            let mut result__: super::super::Graphics::Display::DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISimpleOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<SimpleOrientationSensor> {
        Self::ISimpleOrientationSensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SimpleOrientationSensor>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>>(result__)
        })
    }
    pub fn ISimpleOrientationSensorStatics<R, F: FnOnce(&ISimpleOrientationSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISimpleOrientationSensorStatics2<R, F: FnOnce(&ISimpleOrientationSensorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SimpleOrientationSensor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SimpleOrientationSensor;{5ff53856-214a-4dee-a3f9-616f1ab06ffd})");
}
unsafe impl ::windows::core::Interface for SimpleOrientationSensor {
    type Vtable = ISimpleOrientationSensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ff53856_214a_4dee_a3f9_616f1ab06ffd);
}
impl ::windows::core::RuntimeName for SimpleOrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensor";
}
impl ::core::convert::From<SimpleOrientationSensor> for ::windows::core::IUnknown {
    fn from(value: SimpleOrientationSensor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SimpleOrientationSensor> for ::windows::core::IUnknown {
    fn from(value: &SimpleOrientationSensor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SimpleOrientationSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SimpleOrientationSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SimpleOrientationSensor> for ::windows::core::IInspectable {
    fn from(value: SimpleOrientationSensor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SimpleOrientationSensor> for ::windows::core::IInspectable {
    fn from(value: &SimpleOrientationSensor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SimpleOrientationSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SimpleOrientationSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SimpleOrientationSensor {}
unsafe impl ::core::marker::Sync for SimpleOrientationSensor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SimpleOrientationSensorOrientationChangedEventArgs(pub ::windows::core::IInspectable);
impl SimpleOrientationSensorOrientationChangedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__: SimpleOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SimpleOrientation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SimpleOrientationSensorOrientationChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SimpleOrientationSensorOrientationChangedEventArgs;{bcd5c660-23d4-4b4c-a22e-ba81ade0c601})");
}
unsafe impl ::windows::core::Interface for SimpleOrientationSensorOrientationChangedEventArgs {
    type Vtable = ISimpleOrientationSensorOrientationChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcd5c660_23d4_4b4c_a22e_ba81ade0c601);
}
impl ::windows::core::RuntimeName for SimpleOrientationSensorOrientationChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensorOrientationChangedEventArgs";
}
impl ::core::convert::From<SimpleOrientationSensorOrientationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SimpleOrientationSensorOrientationChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SimpleOrientationSensorOrientationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SimpleOrientationSensorOrientationChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SimpleOrientationSensorOrientationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SimpleOrientationSensorOrientationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SimpleOrientationSensorOrientationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SimpleOrientationSensorOrientationChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SimpleOrientationSensorOrientationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SimpleOrientationSensorOrientationChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SimpleOrientationSensorOrientationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SimpleOrientationSensorOrientationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SimpleOrientationSensorOrientationChangedEventArgs {}
unsafe impl ::core::marker::Sync for SimpleOrientationSensorOrientationChangedEventArgs {}
