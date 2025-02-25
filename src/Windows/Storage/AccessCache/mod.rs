#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AccessCacheOptions(pub u32);
impl AccessCacheOptions {
    pub const None: AccessCacheOptions = AccessCacheOptions(0u32);
    pub const DisallowUserInput: AccessCacheOptions = AccessCacheOptions(1u32);
    pub const FastLocationsOnly: AccessCacheOptions = AccessCacheOptions(2u32);
    pub const UseReadOnlyCachedCopy: AccessCacheOptions = AccessCacheOptions(4u32);
    pub const SuppressAccessTimeUpdate: AccessCacheOptions = AccessCacheOptions(8u32);
}
impl ::core::convert::From<u32> for AccessCacheOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AccessCacheOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AccessCacheOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.AccessCacheOptions;u4)");
}
impl ::windows::core::DefaultType for AccessCacheOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for AccessCacheOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AccessCacheOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AccessCacheOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AccessCacheOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AccessCacheOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct AccessListEntry {
    pub Token: ::windows::core::HSTRING,
    pub Metadata: ::windows::core::HSTRING,
}
impl AccessListEntry {}
impl ::core::default::Default for AccessListEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AccessListEntry {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AccessListEntry").field("Token", &self.Token).field("Metadata", &self.Metadata).finish()
    }
}
impl ::core::cmp::PartialEq for AccessListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token && self.Metadata == other.Metadata
    }
}
impl ::core::cmp::Eq for AccessListEntry {}
unsafe impl ::windows::core::Abi for AccessListEntry {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for AccessListEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Storage.AccessCache.AccessListEntry;string;string)");
}
impl ::windows::core::DefaultType for AccessListEntry {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccessListEntryView(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl AccessListEntryView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<AccessListEntry> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<AccessListEntry> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<AccessListEntry>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, AccessListEntry>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<AccessListEntry as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<AccessListEntry>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<AccessListEntry>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<AccessListEntry>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for AccessListEntryView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.AccessListEntryView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};struct(Windows.Storage.AccessCache.AccessListEntry;string;string)))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for AccessListEntryView {
    type Vtable = super::super::Foundation::Collections::IVectorView_abi<AccessListEntry>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::Foundation::Collections::IVectorView<AccessListEntry> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for AccessListEntryView {
    const NAME: &'static str = "Windows.Storage.AccessCache.AccessListEntryView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<AccessListEntryView> for ::windows::core::IUnknown {
    fn from(value: AccessListEntryView) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&AccessListEntryView> for ::windows::core::IUnknown {
    fn from(value: &AccessListEntryView) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessListEntryView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessListEntryView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<AccessListEntryView> for ::windows::core::IInspectable {
    fn from(value: AccessListEntryView) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&AccessListEntryView> for ::windows::core::IInspectable {
    fn from(value: &AccessListEntryView) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessListEntryView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessListEntryView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<AccessListEntryView> for super::super::Foundation::Collections::IVectorView<AccessListEntry> {
    fn from(value: AccessListEntryView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&AccessListEntryView> for super::super::Foundation::Collections::IVectorView<AccessListEntry> {
    fn from(value: &AccessListEntryView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<AccessListEntry>> for AccessListEntryView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<AccessListEntry>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<AccessListEntry>> for &AccessListEntryView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<AccessListEntry>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<AccessListEntryView> for super::super::Foundation::Collections::IIterable<AccessListEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: AccessListEntryView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&AccessListEntryView> for super::super::Foundation::Collections::IIterable<AccessListEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AccessListEntryView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<AccessListEntry>> for AccessListEntryView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<AccessListEntry>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<AccessListEntry>> for &AccessListEntryView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<AccessListEntry>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<AccessListEntry>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IItemRemovedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IItemRemovedEventArgs {
    type Vtable = IItemRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59677e5c_55be_4c66_ba66_5eaea79d2631);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<AccessListEntry>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageApplicationPermissionsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageApplicationPermissionsStatics {
    type Vtable = IStorageApplicationPermissionsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4391dfaa_d033_48f9_8060_3ec847d2e3f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageApplicationPermissionsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageApplicationPermissionsStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageApplicationPermissionsStatics2 {
    type Vtable = IStorageApplicationPermissionsStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x072716ec_aa05_4294_9a11_1a3d04519ad0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageApplicationPermissionsStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStorageItemAccessList(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageItemAccessList {
    type Vtable = IStorageItemAccessList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2caff6ad_de90_47f5_b2c3_dd36c9fdd453);
}
impl IStorageItemAccessList {
    pub fn AddOverloadDefaultMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, file: Param0, metadata: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), metadata.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, token: Param0, file: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi(), file.into_param().abi()).ok() }
    }
    pub fn AddOrReplace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IStorageItem>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, file: Param1, metadata: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi(), file.into_param().abi(), metadata.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFileWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFolderWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ContainsItem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn CheckAccess<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> ::windows::core::Result<AccessListEntryView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessListEntryView>(result__)
        }
    }
    pub fn MaximumItemsAllowed(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemAccessList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2caff6ad-de90-47f5-b2c3-dd36c9fdd453}");
}
impl ::core::convert::From<IStorageItemAccessList> for ::windows::core::IUnknown {
    fn from(value: IStorageItemAccessList) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageItemAccessList> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemAccessList) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageItemAccessList> for ::windows::core::IInspectable {
    fn from(value: IStorageItemAccessList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageItemAccessList> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemAccessList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemAccessList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: ::windows::core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AccessCacheOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AccessCacheOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AccessCacheOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageItemMostRecentlyUsedList {
    type Vtable = IStorageItemMostRecentlyUsedList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x016239d5_510d_411e_8cf1_c3d1effa4c33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageItemMostRecentlyUsedList2 {
    type Vtable = IStorageItemMostRecentlyUsedList2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda481ea0_ed8d_4731_a1db_e44ee2204093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, visibility: RecentStorageItemVisibility, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: ::windows::core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, visibility: RecentStorageItemVisibility) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ItemRemovedEventArgs(pub ::windows::core::IInspectable);
impl ItemRemovedEventArgs {
    pub fn RemovedEntry(&self) -> ::windows::core::Result<AccessListEntry> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<AccessListEntry> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessListEntry>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ItemRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.ItemRemovedEventArgs;{59677e5c-55be-4c66-ba66-5eaea79d2631})");
}
unsafe impl ::windows::core::Interface for ItemRemovedEventArgs {
    type Vtable = IItemRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59677e5c_55be_4c66_ba66_5eaea79d2631);
}
impl ::windows::core::RuntimeName for ItemRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.AccessCache.ItemRemovedEventArgs";
}
impl ::core::convert::From<ItemRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ItemRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ItemRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ItemRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ItemRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ItemRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ItemRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ItemRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ItemRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ItemRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ItemRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ItemRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RecentStorageItemVisibility(pub i32);
impl RecentStorageItemVisibility {
    pub const AppOnly: RecentStorageItemVisibility = RecentStorageItemVisibility(0i32);
    pub const AppAndSystem: RecentStorageItemVisibility = RecentStorageItemVisibility(1i32);
}
impl ::core::convert::From<i32> for RecentStorageItemVisibility {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RecentStorageItemVisibility {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RecentStorageItemVisibility {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.RecentStorageItemVisibility;i4)");
}
impl ::windows::core::DefaultType for RecentStorageItemVisibility {
    type DefaultType = Self;
}
pub struct StorageApplicationPermissions {}
impl StorageApplicationPermissions {
    pub fn FutureAccessList() -> ::windows::core::Result<StorageItemAccessList> {
        Self::IStorageApplicationPermissionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageItemAccessList>(result__)
        })
    }
    pub fn MostRecentlyUsedList() -> ::windows::core::Result<StorageItemMostRecentlyUsedList> {
        Self::IStorageApplicationPermissionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageItemMostRecentlyUsedList>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetFutureAccessListForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<StorageItemAccessList> {
        Self::IStorageApplicationPermissionsStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<StorageItemAccessList>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetMostRecentlyUsedListForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<StorageItemMostRecentlyUsedList> {
        Self::IStorageApplicationPermissionsStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<StorageItemMostRecentlyUsedList>(result__)
        })
    }
    pub fn IStorageApplicationPermissionsStatics<R, F: FnOnce(&IStorageApplicationPermissionsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStorageApplicationPermissionsStatics2<R, F: FnOnce(&IStorageApplicationPermissionsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StorageApplicationPermissions {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageApplicationPermissions";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageItemAccessList(pub ::windows::core::IInspectable);
impl StorageItemAccessList {
    pub fn AddOverloadDefaultMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, file: Param0, metadata: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), metadata.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, token: Param0, file: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi(), file.into_param().abi()).ok() }
    }
    pub fn AddOrReplace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IStorageItem>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, file: Param1, metadata: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi(), file.into_param().abi(), metadata.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFileWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFolderWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ContainsItem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn CheckAccess<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> ::windows::core::Result<AccessListEntryView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessListEntryView>(result__)
        }
    }
    pub fn MaximumItemsAllowed(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageItemAccessList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.StorageItemAccessList;{2caff6ad-de90-47f5-b2c3-dd36c9fdd453})");
}
unsafe impl ::windows::core::Interface for StorageItemAccessList {
    type Vtable = IStorageItemAccessList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2caff6ad_de90_47f5_b2c3_dd36c9fdd453);
}
impl ::windows::core::RuntimeName for StorageItemAccessList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemAccessList";
}
impl ::core::convert::From<StorageItemAccessList> for ::windows::core::IUnknown {
    fn from(value: StorageItemAccessList) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageItemAccessList> for ::windows::core::IUnknown {
    fn from(value: &StorageItemAccessList) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageItemAccessList> for ::windows::core::IInspectable {
    fn from(value: StorageItemAccessList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageItemAccessList> for ::windows::core::IInspectable {
    fn from(value: &StorageItemAccessList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<StorageItemAccessList> for IStorageItemAccessList {
    fn from(value: StorageItemAccessList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemAccessList> for IStorageItemAccessList {
    fn from(value: &StorageItemAccessList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemAccessList> for StorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemAccessList> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemAccessList> for &StorageItemAccessList {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemAccessList> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageItemMostRecentlyUsedList(pub ::windows::core::IInspectable);
impl StorageItemMostRecentlyUsedList {
    #[cfg(feature = "Foundation")]
    pub fn ItemRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn AddOverloadDefaultMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, file: Param0, metadata: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), metadata.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, token: Param0, file: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi(), file.into_param().abi()).ok() }
    }
    pub fn AddOrReplace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IStorageItem>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, file: Param1, metadata: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi(), file.into_param().abi(), metadata.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFileWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFolderWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ContainsItem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn CheckAccess<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> ::windows::core::Result<AccessListEntryView> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessListEntryView>(result__)
        }
    }
    pub fn MaximumItemsAllowed(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AddWithMetadataAndVisibility<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, file: Param0, metadata: Param1, visibility: RecentStorageItemVisibility) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemMostRecentlyUsedList2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), metadata.into_param().abi(), visibility, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AddOrReplaceWithMetadataAndVisibility<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IStorageItem>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, token: Param0, file: Param1, metadata: Param2, visibility: RecentStorageItemVisibility) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageItemMostRecentlyUsedList2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi(), file.into_param().abi(), metadata.into_param().abi(), visibility).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageItemMostRecentlyUsedList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.StorageItemMostRecentlyUsedList;{016239d5-510d-411e-8cf1-c3d1effa4c33})");
}
unsafe impl ::windows::core::Interface for StorageItemMostRecentlyUsedList {
    type Vtable = IStorageItemMostRecentlyUsedList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x016239d5_510d_411e_8cf1_c3d1effa4c33);
}
impl ::windows::core::RuntimeName for StorageItemMostRecentlyUsedList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemMostRecentlyUsedList";
}
impl ::core::convert::From<StorageItemMostRecentlyUsedList> for ::windows::core::IUnknown {
    fn from(value: StorageItemMostRecentlyUsedList) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageItemMostRecentlyUsedList> for ::windows::core::IUnknown {
    fn from(value: &StorageItemMostRecentlyUsedList) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageItemMostRecentlyUsedList> for ::windows::core::IInspectable {
    fn from(value: StorageItemMostRecentlyUsedList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageItemMostRecentlyUsedList> for ::windows::core::IInspectable {
    fn from(value: &StorageItemMostRecentlyUsedList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<StorageItemMostRecentlyUsedList> for IStorageItemAccessList {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemMostRecentlyUsedList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageItemMostRecentlyUsedList> for IStorageItemAccessList {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemMostRecentlyUsedList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemAccessList> for StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemAccessList> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemAccessList> for &StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemAccessList> {
        ::core::convert::TryInto::<IStorageItemAccessList>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
