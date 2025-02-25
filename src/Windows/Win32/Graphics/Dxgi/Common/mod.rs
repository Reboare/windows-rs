#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXGI_ALPHA_MODE(pub u32);
pub const DXGI_ALPHA_MODE_UNSPECIFIED: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(0u32);
pub const DXGI_ALPHA_MODE_PREMULTIPLIED: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(1u32);
pub const DXGI_ALPHA_MODE_STRAIGHT: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(2u32);
pub const DXGI_ALPHA_MODE_IGNORE: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(3u32);
pub const DXGI_ALPHA_MODE_FORCE_DWORD: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(4294967295u32);
impl ::core::convert::From<u32> for DXGI_ALPHA_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXGI_ALPHA_MODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for DXGI_ALPHA_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DXGI_ALPHA_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_ALPHA_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_ALPHA_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DXGI_ALPHA_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_CENTER_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967294u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXGI_COLOR_SPACE_TYPE(pub i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(0i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(1i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(2i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(3i32);
pub const DXGI_COLOR_SPACE_RESERVED: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(4i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(5i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(6i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(7i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(8i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(9i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(10i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(11i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(12i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(13i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(14i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(15i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(16i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(17i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(18i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(19i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(20i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(21i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(22i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(23i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(24i32);
pub const DXGI_COLOR_SPACE_CUSTOM: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(-1i32);
impl ::core::convert::From<i32> for DXGI_COLOR_SPACE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXGI_COLOR_SPACE_TYPE {
    type Abi = Self;
}
pub const DXGI_CPU_ACCESS_DYNAMIC: u32 = 1u32;
pub const DXGI_CPU_ACCESS_FIELD: u32 = 15u32;
pub const DXGI_CPU_ACCESS_NONE: u32 = 0u32;
pub const DXGI_CPU_ACCESS_READ_WRITE: u32 = 2u32;
pub const DXGI_CPU_ACCESS_SCRATCH: u32 = 3u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXGI_FORMAT(pub u32);
pub const DXGI_FORMAT_UNKNOWN: DXGI_FORMAT = DXGI_FORMAT(0u32);
pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(1u32);
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(2u32);
pub const DXGI_FORMAT_R32G32B32A32_UINT: DXGI_FORMAT = DXGI_FORMAT(3u32);
pub const DXGI_FORMAT_R32G32B32A32_SINT: DXGI_FORMAT = DXGI_FORMAT(4u32);
pub const DXGI_FORMAT_R32G32B32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(5u32);
pub const DXGI_FORMAT_R32G32B32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(6u32);
pub const DXGI_FORMAT_R32G32B32_UINT: DXGI_FORMAT = DXGI_FORMAT(7u32);
pub const DXGI_FORMAT_R32G32B32_SINT: DXGI_FORMAT = DXGI_FORMAT(8u32);
pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(9u32);
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: DXGI_FORMAT = DXGI_FORMAT(10u32);
pub const DXGI_FORMAT_R16G16B16A16_UNORM: DXGI_FORMAT = DXGI_FORMAT(11u32);
pub const DXGI_FORMAT_R16G16B16A16_UINT: DXGI_FORMAT = DXGI_FORMAT(12u32);
pub const DXGI_FORMAT_R16G16B16A16_SNORM: DXGI_FORMAT = DXGI_FORMAT(13u32);
pub const DXGI_FORMAT_R16G16B16A16_SINT: DXGI_FORMAT = DXGI_FORMAT(14u32);
pub const DXGI_FORMAT_R32G32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(15u32);
pub const DXGI_FORMAT_R32G32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(16u32);
pub const DXGI_FORMAT_R32G32_UINT: DXGI_FORMAT = DXGI_FORMAT(17u32);
pub const DXGI_FORMAT_R32G32_SINT: DXGI_FORMAT = DXGI_FORMAT(18u32);
pub const DXGI_FORMAT_R32G8X24_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(19u32);
pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: DXGI_FORMAT = DXGI_FORMAT(20u32);
pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(21u32);
pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: DXGI_FORMAT = DXGI_FORMAT(22u32);
pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(23u32);
pub const DXGI_FORMAT_R10G10B10A2_UNORM: DXGI_FORMAT = DXGI_FORMAT(24u32);
pub const DXGI_FORMAT_R10G10B10A2_UINT: DXGI_FORMAT = DXGI_FORMAT(25u32);
pub const DXGI_FORMAT_R11G11B10_FLOAT: DXGI_FORMAT = DXGI_FORMAT(26u32);
pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(27u32);
pub const DXGI_FORMAT_R8G8B8A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(28u32);
pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(29u32);
pub const DXGI_FORMAT_R8G8B8A8_UINT: DXGI_FORMAT = DXGI_FORMAT(30u32);
pub const DXGI_FORMAT_R8G8B8A8_SNORM: DXGI_FORMAT = DXGI_FORMAT(31u32);
pub const DXGI_FORMAT_R8G8B8A8_SINT: DXGI_FORMAT = DXGI_FORMAT(32u32);
pub const DXGI_FORMAT_R16G16_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(33u32);
pub const DXGI_FORMAT_R16G16_FLOAT: DXGI_FORMAT = DXGI_FORMAT(34u32);
pub const DXGI_FORMAT_R16G16_UNORM: DXGI_FORMAT = DXGI_FORMAT(35u32);
pub const DXGI_FORMAT_R16G16_UINT: DXGI_FORMAT = DXGI_FORMAT(36u32);
pub const DXGI_FORMAT_R16G16_SNORM: DXGI_FORMAT = DXGI_FORMAT(37u32);
pub const DXGI_FORMAT_R16G16_SINT: DXGI_FORMAT = DXGI_FORMAT(38u32);
pub const DXGI_FORMAT_R32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(39u32);
pub const DXGI_FORMAT_D32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(40u32);
pub const DXGI_FORMAT_R32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(41u32);
pub const DXGI_FORMAT_R32_UINT: DXGI_FORMAT = DXGI_FORMAT(42u32);
pub const DXGI_FORMAT_R32_SINT: DXGI_FORMAT = DXGI_FORMAT(43u32);
pub const DXGI_FORMAT_R24G8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(44u32);
pub const DXGI_FORMAT_D24_UNORM_S8_UINT: DXGI_FORMAT = DXGI_FORMAT(45u32);
pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(46u32);
pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: DXGI_FORMAT = DXGI_FORMAT(47u32);
pub const DXGI_FORMAT_R8G8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(48u32);
pub const DXGI_FORMAT_R8G8_UNORM: DXGI_FORMAT = DXGI_FORMAT(49u32);
pub const DXGI_FORMAT_R8G8_UINT: DXGI_FORMAT = DXGI_FORMAT(50u32);
pub const DXGI_FORMAT_R8G8_SNORM: DXGI_FORMAT = DXGI_FORMAT(51u32);
pub const DXGI_FORMAT_R8G8_SINT: DXGI_FORMAT = DXGI_FORMAT(52u32);
pub const DXGI_FORMAT_R16_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(53u32);
pub const DXGI_FORMAT_R16_FLOAT: DXGI_FORMAT = DXGI_FORMAT(54u32);
pub const DXGI_FORMAT_D16_UNORM: DXGI_FORMAT = DXGI_FORMAT(55u32);
pub const DXGI_FORMAT_R16_UNORM: DXGI_FORMAT = DXGI_FORMAT(56u32);
pub const DXGI_FORMAT_R16_UINT: DXGI_FORMAT = DXGI_FORMAT(57u32);
pub const DXGI_FORMAT_R16_SNORM: DXGI_FORMAT = DXGI_FORMAT(58u32);
pub const DXGI_FORMAT_R16_SINT: DXGI_FORMAT = DXGI_FORMAT(59u32);
pub const DXGI_FORMAT_R8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(60u32);
pub const DXGI_FORMAT_R8_UNORM: DXGI_FORMAT = DXGI_FORMAT(61u32);
pub const DXGI_FORMAT_R8_UINT: DXGI_FORMAT = DXGI_FORMAT(62u32);
pub const DXGI_FORMAT_R8_SNORM: DXGI_FORMAT = DXGI_FORMAT(63u32);
pub const DXGI_FORMAT_R8_SINT: DXGI_FORMAT = DXGI_FORMAT(64u32);
pub const DXGI_FORMAT_A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(65u32);
pub const DXGI_FORMAT_R1_UNORM: DXGI_FORMAT = DXGI_FORMAT(66u32);
pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: DXGI_FORMAT = DXGI_FORMAT(67u32);
pub const DXGI_FORMAT_R8G8_B8G8_UNORM: DXGI_FORMAT = DXGI_FORMAT(68u32);
pub const DXGI_FORMAT_G8R8_G8B8_UNORM: DXGI_FORMAT = DXGI_FORMAT(69u32);
pub const DXGI_FORMAT_BC1_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(70u32);
pub const DXGI_FORMAT_BC1_UNORM: DXGI_FORMAT = DXGI_FORMAT(71u32);
pub const DXGI_FORMAT_BC1_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(72u32);
pub const DXGI_FORMAT_BC2_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(73u32);
pub const DXGI_FORMAT_BC2_UNORM: DXGI_FORMAT = DXGI_FORMAT(74u32);
pub const DXGI_FORMAT_BC2_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(75u32);
pub const DXGI_FORMAT_BC3_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(76u32);
pub const DXGI_FORMAT_BC3_UNORM: DXGI_FORMAT = DXGI_FORMAT(77u32);
pub const DXGI_FORMAT_BC3_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(78u32);
pub const DXGI_FORMAT_BC4_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(79u32);
pub const DXGI_FORMAT_BC4_UNORM: DXGI_FORMAT = DXGI_FORMAT(80u32);
pub const DXGI_FORMAT_BC4_SNORM: DXGI_FORMAT = DXGI_FORMAT(81u32);
pub const DXGI_FORMAT_BC5_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(82u32);
pub const DXGI_FORMAT_BC5_UNORM: DXGI_FORMAT = DXGI_FORMAT(83u32);
pub const DXGI_FORMAT_BC5_SNORM: DXGI_FORMAT = DXGI_FORMAT(84u32);
pub const DXGI_FORMAT_B5G6R5_UNORM: DXGI_FORMAT = DXGI_FORMAT(85u32);
pub const DXGI_FORMAT_B5G5R5A1_UNORM: DXGI_FORMAT = DXGI_FORMAT(86u32);
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(87u32);
pub const DXGI_FORMAT_B8G8R8X8_UNORM: DXGI_FORMAT = DXGI_FORMAT(88u32);
pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: DXGI_FORMAT = DXGI_FORMAT(89u32);
pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(90u32);
pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(91u32);
pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(92u32);
pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(93u32);
pub const DXGI_FORMAT_BC6H_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(94u32);
pub const DXGI_FORMAT_BC6H_UF16: DXGI_FORMAT = DXGI_FORMAT(95u32);
pub const DXGI_FORMAT_BC6H_SF16: DXGI_FORMAT = DXGI_FORMAT(96u32);
pub const DXGI_FORMAT_BC7_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(97u32);
pub const DXGI_FORMAT_BC7_UNORM: DXGI_FORMAT = DXGI_FORMAT(98u32);
pub const DXGI_FORMAT_BC7_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(99u32);
pub const DXGI_FORMAT_AYUV: DXGI_FORMAT = DXGI_FORMAT(100u32);
pub const DXGI_FORMAT_Y410: DXGI_FORMAT = DXGI_FORMAT(101u32);
pub const DXGI_FORMAT_Y416: DXGI_FORMAT = DXGI_FORMAT(102u32);
pub const DXGI_FORMAT_NV12: DXGI_FORMAT = DXGI_FORMAT(103u32);
pub const DXGI_FORMAT_P010: DXGI_FORMAT = DXGI_FORMAT(104u32);
pub const DXGI_FORMAT_P016: DXGI_FORMAT = DXGI_FORMAT(105u32);
pub const DXGI_FORMAT_420_OPAQUE: DXGI_FORMAT = DXGI_FORMAT(106u32);
pub const DXGI_FORMAT_YUY2: DXGI_FORMAT = DXGI_FORMAT(107u32);
pub const DXGI_FORMAT_Y210: DXGI_FORMAT = DXGI_FORMAT(108u32);
pub const DXGI_FORMAT_Y216: DXGI_FORMAT = DXGI_FORMAT(109u32);
pub const DXGI_FORMAT_NV11: DXGI_FORMAT = DXGI_FORMAT(110u32);
pub const DXGI_FORMAT_AI44: DXGI_FORMAT = DXGI_FORMAT(111u32);
pub const DXGI_FORMAT_IA44: DXGI_FORMAT = DXGI_FORMAT(112u32);
pub const DXGI_FORMAT_P8: DXGI_FORMAT = DXGI_FORMAT(113u32);
pub const DXGI_FORMAT_A8P8: DXGI_FORMAT = DXGI_FORMAT(114u32);
pub const DXGI_FORMAT_B4G4R4A4_UNORM: DXGI_FORMAT = DXGI_FORMAT(115u32);
pub const DXGI_FORMAT_P208: DXGI_FORMAT = DXGI_FORMAT(130u32);
pub const DXGI_FORMAT_V208: DXGI_FORMAT = DXGI_FORMAT(131u32);
pub const DXGI_FORMAT_V408: DXGI_FORMAT = DXGI_FORMAT(132u32);
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE: DXGI_FORMAT = DXGI_FORMAT(189u32);
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE: DXGI_FORMAT = DXGI_FORMAT(190u32);
pub const DXGI_FORMAT_FORCE_UINT: DXGI_FORMAT = DXGI_FORMAT(4294967295u32);
impl ::core::convert::From<u32> for DXGI_FORMAT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXGI_FORMAT {
    type Abi = Self;
}
impl ::core::ops::BitOr for DXGI_FORMAT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DXGI_FORMAT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_FORMAT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_FORMAT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DXGI_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_FORMAT_DEFINED: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_GAMMA_CONTROL {
    pub Scale: DXGI_RGB,
    pub Offset: DXGI_RGB,
    pub GammaCurve: [DXGI_RGB; 1025],
}
impl DXGI_GAMMA_CONTROL {}
impl ::core::default::Default for DXGI_GAMMA_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXGI_GAMMA_CONTROL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGI_GAMMA_CONTROL").field("Scale", &self.Scale).field("Offset", &self.Offset).field("GammaCurve", &self.GammaCurve).finish()
    }
}
impl ::core::cmp::PartialEq for DXGI_GAMMA_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale && self.Offset == other.Offset && self.GammaCurve == other.GammaCurve
    }
}
impl ::core::cmp::Eq for DXGI_GAMMA_CONTROL {}
unsafe impl ::windows::core::Abi for DXGI_GAMMA_CONTROL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: super::super::super::Foundation::BOOL,
    pub MaxConvertedValue: f32,
    pub MinConvertedValue: f32,
    pub NumGammaControlPoints: u32,
    pub ControlPointPositions: [f32; 1025],
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_GAMMA_CONTROL_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGI_GAMMA_CONTROL_CAPABILITIES")
            .field("ScaleAndOffsetSupported", &self.ScaleAndOffsetSupported)
            .field("MaxConvertedValue", &self.MaxConvertedValue)
            .field("MinConvertedValue", &self.MinConvertedValue)
            .field("NumGammaControlPoints", &self.NumGammaControlPoints)
            .field("ControlPointPositions", &self.ControlPointPositions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.ScaleAndOffsetSupported == other.ScaleAndOffsetSupported && self.MaxConvertedValue == other.MaxConvertedValue && self.MinConvertedValue == other.MinConvertedValue && self.NumGammaControlPoints == other.NumGammaControlPoints && self.ControlPointPositions == other.ControlPointPositions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_GAMMA_CONTROL_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_GAMMA_CONTROL_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 16],
    pub CodeValues: [u8; 162],
}
impl DXGI_JPEG_AC_HUFFMAN_TABLE {}
impl ::core::default::Default for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_AC_HUFFMAN_TABLE").field("CodeCounts", &self.CodeCounts).field("CodeValues", &self.CodeValues).finish()
    }
}
impl ::core::cmp::PartialEq for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.CodeCounts == other.CodeCounts && self.CodeValues == other.CodeValues
    }
}
impl ::core::cmp::Eq for DXGI_JPEG_AC_HUFFMAN_TABLE {}
unsafe impl ::windows::core::Abi for DXGI_JPEG_AC_HUFFMAN_TABLE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 12],
    pub CodeValues: [u8; 12],
}
impl DXGI_JPEG_DC_HUFFMAN_TABLE {}
impl ::core::default::Default for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_DC_HUFFMAN_TABLE").field("CodeCounts", &self.CodeCounts).field("CodeValues", &self.CodeValues).finish()
    }
}
impl ::core::cmp::PartialEq for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.CodeCounts == other.CodeCounts && self.CodeValues == other.CodeValues
    }
}
impl ::core::cmp::Eq for DXGI_JPEG_DC_HUFFMAN_TABLE {}
unsafe impl ::windows::core::Abi for DXGI_JPEG_DC_HUFFMAN_TABLE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub Elements: [u8; 64],
}
impl DXGI_JPEG_QUANTIZATION_TABLE {}
impl ::core::default::Default for DXGI_JPEG_QUANTIZATION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXGI_JPEG_QUANTIZATION_TABLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_QUANTIZATION_TABLE").field("Elements", &self.Elements).finish()
    }
}
impl ::core::cmp::PartialEq for DXGI_JPEG_QUANTIZATION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DXGI_JPEG_QUANTIZATION_TABLE {}
unsafe impl ::windows::core::Abi for DXGI_JPEG_QUANTIZATION_TABLE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
impl DXGI_MODE_DESC {}
impl ::core::default::Default for DXGI_MODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXGI_MODE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGI_MODE_DESC").field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).finish()
    }
}
impl ::core::cmp::PartialEq for DXGI_MODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.RefreshRate == other.RefreshRate && self.Format == other.Format && self.ScanlineOrdering == other.ScanlineOrdering && self.Scaling == other.Scaling
    }
}
impl ::core::cmp::Eq for DXGI_MODE_DESC {}
unsafe impl ::windows::core::Abi for DXGI_MODE_DESC {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXGI_MODE_ROTATION(pub i32);
pub const DXGI_MODE_ROTATION_UNSPECIFIED: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(0i32);
pub const DXGI_MODE_ROTATION_IDENTITY: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(1i32);
pub const DXGI_MODE_ROTATION_ROTATE90: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(2i32);
pub const DXGI_MODE_ROTATION_ROTATE180: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(3i32);
pub const DXGI_MODE_ROTATION_ROTATE270: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(4i32);
impl ::core::convert::From<i32> for DXGI_MODE_ROTATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXGI_MODE_ROTATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXGI_MODE_SCALING(pub i32);
pub const DXGI_MODE_SCALING_UNSPECIFIED: DXGI_MODE_SCALING = DXGI_MODE_SCALING(0i32);
pub const DXGI_MODE_SCALING_CENTERED: DXGI_MODE_SCALING = DXGI_MODE_SCALING(1i32);
pub const DXGI_MODE_SCALING_STRETCHED: DXGI_MODE_SCALING = DXGI_MODE_SCALING(2i32);
impl ::core::convert::From<i32> for DXGI_MODE_SCALING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXGI_MODE_SCALING {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXGI_MODE_SCANLINE_ORDER(pub i32);
pub const DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED: DXGI_MODE_SCANLINE_ORDER = DXGI_MODE_SCANLINE_ORDER(0i32);
pub const DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE: DXGI_MODE_SCANLINE_ORDER = DXGI_MODE_SCANLINE_ORDER(1i32);
pub const DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER = DXGI_MODE_SCANLINE_ORDER(2i32);
pub const DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER = DXGI_MODE_SCANLINE_ORDER(3i32);
impl ::core::convert::From<i32> for DXGI_MODE_SCANLINE_ORDER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXGI_MODE_SCANLINE_ORDER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl DXGI_RATIONAL {}
impl ::core::default::Default for DXGI_RATIONAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXGI_RATIONAL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGI_RATIONAL").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::cmp::PartialEq for DXGI_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for DXGI_RATIONAL {}
unsafe impl ::windows::core::Abi for DXGI_RATIONAL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
impl DXGI_RGB {}
impl ::core::default::Default for DXGI_RGB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXGI_RGB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGI_RGB").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::core::cmp::PartialEq for DXGI_RGB {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for DXGI_RGB {}
unsafe impl ::windows::core::Abi for DXGI_RGB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
impl DXGI_SAMPLE_DESC {}
impl ::core::default::Default for DXGI_SAMPLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXGI_SAMPLE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGI_SAMPLE_DESC").field("Count", &self.Count).field("Quality", &self.Quality).finish()
    }
}
impl ::core::cmp::PartialEq for DXGI_SAMPLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Quality == other.Quality
    }
}
impl ::core::cmp::Eq for DXGI_SAMPLE_DESC {}
unsafe impl ::windows::core::Abi for DXGI_SAMPLE_DESC {
    type Abi = Self;
}
pub const DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967295u32;
pub const _FACDXGI: u32 = 2170u32;
