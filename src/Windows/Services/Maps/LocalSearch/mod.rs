#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ILocalCategoriesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILocalCategoriesStatics {
    type Vtable = ILocalCategoriesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf49399f5_8261_4321_9974_ef92d49a8dca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalCategoriesStatics_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILocalLocation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILocalLocation {
    type Vtable = ILocalLocation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb0fe9ab_4502_4f2c_94a9_0d60de0e2163);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILocalLocation2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILocalLocation2 {
    type Vtable = ILocalLocation2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e9e307c_ecb5_4ffc_bb8c_ba50ba8c2dc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocation2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILocalLocationFinderResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILocalLocationFinderResult {
    type Vtable = ILocalLocationFinderResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd09b6cc6_f338_4191_9fd8_5440b9a68f52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationFinderResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LocalLocationFinderStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILocalLocationFinderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILocalLocationFinderStatics {
    type Vtable = ILocalLocationFinderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ef7344_a0de_48ca_81a8_07c7dcfd37ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationFinderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, searchterm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, searcharea: ::windows::core::RawPtr, localcategory: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxresults: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILocalLocationHoursOfOperationItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILocalLocationHoursOfOperationItem {
    type Vtable = ILocalLocationHoursOfOperationItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23548c72_a1c7_43f1_a4f0_1091c39ec640);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationHoursOfOperationItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILocalLocationRatingInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILocalLocationRatingInfo {
    type Vtable = ILocalLocationRatingInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb1dab56_3354_4311_8bc0_a2d4d5eb806e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationRatingInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaceInfoHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlaceInfoHelperStatics {
    type Vtable = IPlaceInfoHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd1ca9a7_a9c6_491b_bc09_e80fcea48ee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
pub struct LocalCategories {}
impl LocalCategories {
    pub fn BankAndCreditUnions() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EatDrink() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Hospitals() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HotelsAndMotels() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn All() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Parking() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SeeDo() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Shop() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ILocalCategoriesStatics<R, F: FnOnce(&ILocalCategoriesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LocalCategories, ILocalCategoriesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for LocalCategories {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalCategories";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LocalLocation(pub ::windows::core::IInspectable);
impl LocalLocation {
    pub fn Address(&self) -> ::windows::core::Result<super::MapAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MapAddress>(result__)
        }
    }
    pub fn Identifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Point(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DataAttribution(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RatingInfo(&self) -> ::windows::core::Result<LocalLocationRatingInfo> {
        let this = &::windows::core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LocalLocationRatingInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HoursOfOperation(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>> {
        let this = &::windows::core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocation;{bb0fe9ab-4502-4f2c-94a9-0d60de0e2163})");
}
unsafe impl ::windows::core::Interface for LocalLocation {
    type Vtable = ILocalLocation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb0fe9ab_4502_4f2c_94a9_0d60de0e2163);
}
impl ::windows::core::RuntimeName for LocalLocation {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocation";
}
impl ::core::convert::From<LocalLocation> for ::windows::core::IUnknown {
    fn from(value: LocalLocation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LocalLocation> for ::windows::core::IUnknown {
    fn from(value: &LocalLocation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LocalLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LocalLocation> for ::windows::core::IInspectable {
    fn from(value: LocalLocation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LocalLocation> for ::windows::core::IInspectable {
    fn from(value: &LocalLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LocalLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LocalLocation {}
unsafe impl ::core::marker::Sync for LocalLocation {}
pub struct LocalLocationFinder {}
impl LocalLocationFinder {
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocalLocationsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::Geocircle>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(searchterm: Param0, searcharea: Param1, localcategory: Param2, maxresults: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>> {
        Self::ILocalLocationFinderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), searchterm.into_param().abi(), searcharea.into_param().abi(), localcategory.into_param().abi(), maxresults, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>>(result__)
        })
    }
    pub fn ILocalLocationFinderStatics<R, F: FnOnce(&ILocalLocationFinderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LocalLocationFinder, ILocalLocationFinderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for LocalLocationFinder {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinder";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LocalLocationFinderResult(pub ::windows::core::IInspectable);
impl LocalLocationFinderResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<LocalLocation>>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<LocalLocationFinderStatus> {
        let this = self;
        unsafe {
            let mut result__: LocalLocationFinderStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LocalLocationFinderStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationFinderResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationFinderResult;{d09b6cc6-f338-4191-9fd8-5440b9a68f52})");
}
unsafe impl ::windows::core::Interface for LocalLocationFinderResult {
    type Vtable = ILocalLocationFinderResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd09b6cc6_f338_4191_9fd8_5440b9a68f52);
}
impl ::windows::core::RuntimeName for LocalLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinderResult";
}
impl ::core::convert::From<LocalLocationFinderResult> for ::windows::core::IUnknown {
    fn from(value: LocalLocationFinderResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LocalLocationFinderResult> for ::windows::core::IUnknown {
    fn from(value: &LocalLocationFinderResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalLocationFinderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LocalLocationFinderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LocalLocationFinderResult> for ::windows::core::IInspectable {
    fn from(value: LocalLocationFinderResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LocalLocationFinderResult> for ::windows::core::IInspectable {
    fn from(value: &LocalLocationFinderResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalLocationFinderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LocalLocationFinderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LocalLocationFinderResult {}
unsafe impl ::core::marker::Sync for LocalLocationFinderResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LocalLocationFinderStatus(pub i32);
impl LocalLocationFinderStatus {
    pub const Success: LocalLocationFinderStatus = LocalLocationFinderStatus(0i32);
    pub const UnknownError: LocalLocationFinderStatus = LocalLocationFinderStatus(1i32);
    pub const InvalidCredentials: LocalLocationFinderStatus = LocalLocationFinderStatus(2i32);
    pub const InvalidCategory: LocalLocationFinderStatus = LocalLocationFinderStatus(3i32);
    pub const InvalidSearchTerm: LocalLocationFinderStatus = LocalLocationFinderStatus(4i32);
    pub const InvalidSearchArea: LocalLocationFinderStatus = LocalLocationFinderStatus(5i32);
    pub const NetworkFailure: LocalLocationFinderStatus = LocalLocationFinderStatus(6i32);
    pub const NotSupported: LocalLocationFinderStatus = LocalLocationFinderStatus(7i32);
}
impl ::core::convert::From<i32> for LocalLocationFinderStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LocalLocationFinderStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LocalLocationFinderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.LocalSearch.LocalLocationFinderStatus;i4)");
}
impl ::windows::core::DefaultType for LocalLocationFinderStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LocalLocationHoursOfOperationItem(pub ::windows::core::IInspectable);
impl LocalLocationHoursOfOperationItem {
    #[cfg(feature = "Globalization")]
    pub fn Day(&self) -> ::windows::core::Result<super::super::super::Globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Globalization::DayOfWeek = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Globalization::DayOfWeek>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Start(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Span(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationHoursOfOperationItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem;{23548c72-a1c7-43f1-a4f0-1091c39ec640})");
}
unsafe impl ::windows::core::Interface for LocalLocationHoursOfOperationItem {
    type Vtable = ILocalLocationHoursOfOperationItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23548c72_a1c7_43f1_a4f0_1091c39ec640);
}
impl ::windows::core::RuntimeName for LocalLocationHoursOfOperationItem {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem";
}
impl ::core::convert::From<LocalLocationHoursOfOperationItem> for ::windows::core::IUnknown {
    fn from(value: LocalLocationHoursOfOperationItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LocalLocationHoursOfOperationItem> for ::windows::core::IUnknown {
    fn from(value: &LocalLocationHoursOfOperationItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalLocationHoursOfOperationItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LocalLocationHoursOfOperationItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LocalLocationHoursOfOperationItem> for ::windows::core::IInspectable {
    fn from(value: LocalLocationHoursOfOperationItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LocalLocationHoursOfOperationItem> for ::windows::core::IInspectable {
    fn from(value: &LocalLocationHoursOfOperationItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalLocationHoursOfOperationItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LocalLocationHoursOfOperationItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LocalLocationHoursOfOperationItem {}
unsafe impl ::core::marker::Sync for LocalLocationHoursOfOperationItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LocalLocationRatingInfo(pub ::windows::core::IInspectable);
impl LocalLocationRatingInfo {
    #[cfg(feature = "Foundation")]
    pub fn AggregateRating(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RatingCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    pub fn ProviderIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationRatingInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo;{cb1dab56-3354-4311-8bc0-a2d4d5eb806e})");
}
unsafe impl ::windows::core::Interface for LocalLocationRatingInfo {
    type Vtable = ILocalLocationRatingInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb1dab56_3354_4311_8bc0_a2d4d5eb806e);
}
impl ::windows::core::RuntimeName for LocalLocationRatingInfo {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo";
}
impl ::core::convert::From<LocalLocationRatingInfo> for ::windows::core::IUnknown {
    fn from(value: LocalLocationRatingInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LocalLocationRatingInfo> for ::windows::core::IUnknown {
    fn from(value: &LocalLocationRatingInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalLocationRatingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LocalLocationRatingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LocalLocationRatingInfo> for ::windows::core::IInspectable {
    fn from(value: LocalLocationRatingInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LocalLocationRatingInfo> for ::windows::core::IInspectable {
    fn from(value: &LocalLocationRatingInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalLocationRatingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LocalLocationRatingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LocalLocationRatingInfo {}
unsafe impl ::core::marker::Sync for LocalLocationRatingInfo {}
pub struct PlaceInfoHelper {}
impl PlaceInfoHelper {
    pub fn CreateFromLocalLocation<'a, Param0: ::windows::core::IntoParam<'a, LocalLocation>>(location: Param0) -> ::windows::core::Result<super::PlaceInfo> {
        Self::IPlaceInfoHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), location.into_param().abi(), &mut result__).from_abi::<super::PlaceInfo>(result__)
        })
    }
    pub fn IPlaceInfoHelperStatics<R, F: FnOnce(&IPlaceInfoHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlaceInfoHelper, IPlaceInfoHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PlaceInfoHelper {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.PlaceInfoHelper";
}
