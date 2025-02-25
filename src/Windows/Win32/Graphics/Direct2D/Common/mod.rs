#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_ALPHA_MODE(pub u32);
pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(0u32);
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(1u32);
pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(2u32);
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(3u32);
pub const D2D1_ALPHA_MODE_FORCE_DWORD: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(4294967295u32);
impl ::core::convert::From<u32> for D2D1_ALPHA_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_ALPHA_MODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_ALPHA_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_ALPHA_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_ALPHA_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_ALPHA_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_ALPHA_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: D2D_POINT_2F,
    pub point2: D2D_POINT_2F,
    pub point3: D2D_POINT_2F,
}
impl D2D1_BEZIER_SEGMENT {}
impl ::core::default::Default for D2D1_BEZIER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D1_BEZIER_SEGMENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D1_BEZIER_SEGMENT").field("point1", &self.point1).field("point2", &self.point2).field("point3", &self.point3).finish()
    }
}
impl ::core::cmp::PartialEq for D2D1_BEZIER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.point1 == other.point1 && self.point2 == other.point2 && self.point3 == other.point3
    }
}
impl ::core::cmp::Eq for D2D1_BEZIER_SEGMENT {}
unsafe impl ::windows::core::Abi for D2D1_BEZIER_SEGMENT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_BLEND_MODE(pub u32);
pub const D2D1_BLEND_MODE_MULTIPLY: D2D1_BLEND_MODE = D2D1_BLEND_MODE(0u32);
pub const D2D1_BLEND_MODE_SCREEN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(1u32);
pub const D2D1_BLEND_MODE_DARKEN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(2u32);
pub const D2D1_BLEND_MODE_LIGHTEN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(3u32);
pub const D2D1_BLEND_MODE_DISSOLVE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(4u32);
pub const D2D1_BLEND_MODE_COLOR_BURN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(5u32);
pub const D2D1_BLEND_MODE_LINEAR_BURN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(6u32);
pub const D2D1_BLEND_MODE_DARKER_COLOR: D2D1_BLEND_MODE = D2D1_BLEND_MODE(7u32);
pub const D2D1_BLEND_MODE_LIGHTER_COLOR: D2D1_BLEND_MODE = D2D1_BLEND_MODE(8u32);
pub const D2D1_BLEND_MODE_COLOR_DODGE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(9u32);
pub const D2D1_BLEND_MODE_LINEAR_DODGE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(10u32);
pub const D2D1_BLEND_MODE_OVERLAY: D2D1_BLEND_MODE = D2D1_BLEND_MODE(11u32);
pub const D2D1_BLEND_MODE_SOFT_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(12u32);
pub const D2D1_BLEND_MODE_HARD_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(13u32);
pub const D2D1_BLEND_MODE_VIVID_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(14u32);
pub const D2D1_BLEND_MODE_LINEAR_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(15u32);
pub const D2D1_BLEND_MODE_PIN_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(16u32);
pub const D2D1_BLEND_MODE_HARD_MIX: D2D1_BLEND_MODE = D2D1_BLEND_MODE(17u32);
pub const D2D1_BLEND_MODE_DIFFERENCE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(18u32);
pub const D2D1_BLEND_MODE_EXCLUSION: D2D1_BLEND_MODE = D2D1_BLEND_MODE(19u32);
pub const D2D1_BLEND_MODE_HUE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(20u32);
pub const D2D1_BLEND_MODE_SATURATION: D2D1_BLEND_MODE = D2D1_BLEND_MODE(21u32);
pub const D2D1_BLEND_MODE_COLOR: D2D1_BLEND_MODE = D2D1_BLEND_MODE(22u32);
pub const D2D1_BLEND_MODE_LUMINOSITY: D2D1_BLEND_MODE = D2D1_BLEND_MODE(23u32);
pub const D2D1_BLEND_MODE_SUBTRACT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(24u32);
pub const D2D1_BLEND_MODE_DIVISION: D2D1_BLEND_MODE = D2D1_BLEND_MODE(25u32);
pub const D2D1_BLEND_MODE_FORCE_DWORD: D2D1_BLEND_MODE = D2D1_BLEND_MODE(4294967295u32);
impl ::core::convert::From<u32> for D2D1_BLEND_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_BLEND_MODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_BLEND_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_BLEND_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_BLEND_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_BLEND_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_BLEND_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_BORDER_MODE(pub u32);
pub const D2D1_BORDER_MODE_SOFT: D2D1_BORDER_MODE = D2D1_BORDER_MODE(0u32);
pub const D2D1_BORDER_MODE_HARD: D2D1_BORDER_MODE = D2D1_BORDER_MODE(1u32);
pub const D2D1_BORDER_MODE_FORCE_DWORD: D2D1_BORDER_MODE = D2D1_BORDER_MODE(4294967295u32);
impl ::core::convert::From<u32> for D2D1_BORDER_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_BORDER_MODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_BORDER_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_BORDER_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_BORDER_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_BORDER_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_BORDER_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_COLORMATRIX_ALPHA_MODE(pub u32);
pub const D2D1_COLORMATRIX_ALPHA_MODE_PREMULTIPLIED: D2D1_COLORMATRIX_ALPHA_MODE = D2D1_COLORMATRIX_ALPHA_MODE(1u32);
pub const D2D1_COLORMATRIX_ALPHA_MODE_STRAIGHT: D2D1_COLORMATRIX_ALPHA_MODE = D2D1_COLORMATRIX_ALPHA_MODE(2u32);
pub const D2D1_COLORMATRIX_ALPHA_MODE_FORCE_DWORD: D2D1_COLORMATRIX_ALPHA_MODE = D2D1_COLORMATRIX_ALPHA_MODE(4294967295u32);
impl ::core::convert::From<u32> for D2D1_COLORMATRIX_ALPHA_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLORMATRIX_ALPHA_MODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_COLORMATRIX_ALPHA_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_COLORMATRIX_ALPHA_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_COLORMATRIX_ALPHA_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_COLORMATRIX_ALPHA_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_COLORMATRIX_ALPHA_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D1_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl D2D1_COLOR_F {}
impl ::core::default::Default for D2D1_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D1_COLOR_F {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D1_COLOR_F").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::core::cmp::PartialEq for D2D1_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for D2D1_COLOR_F {}
unsafe impl ::windows::core::Abi for D2D1_COLOR_F {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_COMPOSITE_MODE(pub u32);
pub const D2D1_COMPOSITE_MODE_SOURCE_OVER: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(0u32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_OVER: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(1u32);
pub const D2D1_COMPOSITE_MODE_SOURCE_IN: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(2u32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_IN: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(3u32);
pub const D2D1_COMPOSITE_MODE_SOURCE_OUT: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(4u32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_OUT: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(5u32);
pub const D2D1_COMPOSITE_MODE_SOURCE_ATOP: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(6u32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_ATOP: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(7u32);
pub const D2D1_COMPOSITE_MODE_XOR: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(8u32);
pub const D2D1_COMPOSITE_MODE_PLUS: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(9u32);
pub const D2D1_COMPOSITE_MODE_SOURCE_COPY: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(10u32);
pub const D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(11u32);
pub const D2D1_COMPOSITE_MODE_MASK_INVERT: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(12u32);
pub const D2D1_COMPOSITE_MODE_FORCE_DWORD: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(4294967295u32);
impl ::core::convert::From<u32> for D2D1_COMPOSITE_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_COMPOSITE_MODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_COMPOSITE_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_COMPOSITE_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_COMPOSITE_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_COMPOSITE_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_COMPOSITE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_FIGURE_BEGIN(pub u32);
pub const D2D1_FIGURE_BEGIN_FILLED: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(0u32);
pub const D2D1_FIGURE_BEGIN_HOLLOW: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(1u32);
pub const D2D1_FIGURE_BEGIN_FORCE_DWORD: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(4294967295u32);
impl ::core::convert::From<u32> for D2D1_FIGURE_BEGIN {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_FIGURE_BEGIN {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_FIGURE_BEGIN {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_FIGURE_BEGIN {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_FIGURE_BEGIN {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_FIGURE_BEGIN {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_FIGURE_BEGIN {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_FIGURE_END(pub u32);
pub const D2D1_FIGURE_END_OPEN: D2D1_FIGURE_END = D2D1_FIGURE_END(0u32);
pub const D2D1_FIGURE_END_CLOSED: D2D1_FIGURE_END = D2D1_FIGURE_END(1u32);
pub const D2D1_FIGURE_END_FORCE_DWORD: D2D1_FIGURE_END = D2D1_FIGURE_END(4294967295u32);
impl ::core::convert::From<u32> for D2D1_FIGURE_END {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_FIGURE_END {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_FIGURE_END {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_FIGURE_END {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_FIGURE_END {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_FIGURE_END {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_FIGURE_END {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_FILL_MODE(pub u32);
pub const D2D1_FILL_MODE_ALTERNATE: D2D1_FILL_MODE = D2D1_FILL_MODE(0u32);
pub const D2D1_FILL_MODE_WINDING: D2D1_FILL_MODE = D2D1_FILL_MODE(1u32);
pub const D2D1_FILL_MODE_FORCE_DWORD: D2D1_FILL_MODE = D2D1_FILL_MODE(4294967295u32);
impl ::core::convert::From<u32> for D2D1_FILL_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_FILL_MODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_FILL_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_FILL_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_FILL_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_FILL_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_FILL_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_PATH_SEGMENT(pub u32);
pub const D2D1_PATH_SEGMENT_NONE: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(0u32);
pub const D2D1_PATH_SEGMENT_FORCE_UNSTROKED: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(1u32);
pub const D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(2u32);
pub const D2D1_PATH_SEGMENT_FORCE_DWORD: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(4294967295u32);
impl ::core::convert::From<u32> for D2D1_PATH_SEGMENT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_PATH_SEGMENT {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_PATH_SEGMENT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_PATH_SEGMENT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_PIXEL_FORMAT {
    pub format: super::super::Dxgi::Common::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl D2D1_PIXEL_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D2D1_PIXEL_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D2D1_PIXEL_FORMAT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D1_PIXEL_FORMAT").field("format", &self.format).field("alphaMode", &self.alphaMode).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D2D1_PIXEL_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format && self.alphaMode == other.alphaMode
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D2D1_PIXEL_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D2D1_PIXEL_FORMAT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_TURBULENCE_NOISE(pub u32);
pub const D2D1_TURBULENCE_NOISE_FRACTAL_SUM: D2D1_TURBULENCE_NOISE = D2D1_TURBULENCE_NOISE(0u32);
pub const D2D1_TURBULENCE_NOISE_TURBULENCE: D2D1_TURBULENCE_NOISE = D2D1_TURBULENCE_NOISE(1u32);
pub const D2D1_TURBULENCE_NOISE_FORCE_DWORD: D2D1_TURBULENCE_NOISE = D2D1_TURBULENCE_NOISE(4294967295u32);
impl ::core::convert::From<u32> for D2D1_TURBULENCE_NOISE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_TURBULENCE_NOISE {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_TURBULENCE_NOISE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_TURBULENCE_NOISE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_TURBULENCE_NOISE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_TURBULENCE_NOISE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_TURBULENCE_NOISE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl D2D_COLOR_F {}
impl ::core::default::Default for D2D_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_COLOR_F {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_COLOR_F").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for D2D_COLOR_F {}
unsafe impl ::windows::core::Abi for D2D_COLOR_F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_MATRIX_3X2_F {
    pub Anonymous: D2D_MATRIX_3X2_F_0,
}
impl D2D_MATRIX_3X2_F {}
impl ::core::default::Default for D2D_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_3X2_F {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_3X2_F {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_3X2_F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union D2D_MATRIX_3X2_F_0 {
    pub Anonymous1: D2D_MATRIX_3X2_F_0_0,
    pub Anonymous2: D2D_MATRIX_3X2_F_0_1,
    pub m: [f32; 6],
}
impl D2D_MATRIX_3X2_F_0 {}
impl ::core::default::Default for D2D_MATRIX_3X2_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_3X2_F_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_3X2_F_0 {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_3X2_F_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_MATRIX_3X2_F_0_0 {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
impl D2D_MATRIX_3X2_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_3X2_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_MATRIX_3X2_F_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("m11", &self.m11).field("m12", &self.m12).field("m21", &self.m21).field("m22", &self.m22).field("dx", &self.dx).field("dy", &self.dy).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_3X2_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.m11 == other.m11 && self.m12 == other.m12 && self.m21 == other.m21 && self.m22 == other.m22 && self.dx == other.dx && self.dy == other.dy
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_3X2_F_0_0 {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_3X2_F_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_MATRIX_3X2_F_0_1 {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl D2D_MATRIX_3X2_F_0_1 {}
impl ::core::default::Default for D2D_MATRIX_3X2_F_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_MATRIX_3X2_F_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("_11", &self._11).field("_12", &self._12).field("_21", &self._21).field("_22", &self._22).field("_31", &self._31).field("_32", &self._32).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_3X2_F_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._21 == other._21 && self._22 == other._22 && self._31 == other._31 && self._32 == other._32
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_3X2_F_0_1 {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_3X2_F_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_MATRIX_4X3_F {
    pub Anonymous: D2D_MATRIX_4X3_F_0,
}
impl D2D_MATRIX_4X3_F {}
impl ::core::default::Default for D2D_MATRIX_4X3_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X3_F {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X3_F {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X3_F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union D2D_MATRIX_4X3_F_0 {
    pub Anonymous: D2D_MATRIX_4X3_F_0_0,
    pub m: [f32; 12],
}
impl D2D_MATRIX_4X3_F_0 {}
impl ::core::default::Default for D2D_MATRIX_4X3_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X3_F_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X3_F_0 {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X3_F_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_MATRIX_4X3_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
}
impl D2D_MATRIX_4X3_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_4X3_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_MATRIX_4X3_F_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_11", &self._11)
            .field("_12", &self._12)
            .field("_13", &self._13)
            .field("_21", &self._21)
            .field("_22", &self._22)
            .field("_23", &self._23)
            .field("_31", &self._31)
            .field("_32", &self._32)
            .field("_33", &self._33)
            .field("_41", &self._41)
            .field("_42", &self._42)
            .field("_43", &self._43)
            .finish()
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X3_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._41 == other._41 && self._42 == other._42 && self._43 == other._43
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X3_F_0_0 {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X3_F_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_MATRIX_4X4_F {
    pub Anonymous: D2D_MATRIX_4X4_F_0,
}
impl D2D_MATRIX_4X4_F {}
impl ::core::default::Default for D2D_MATRIX_4X4_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X4_F {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X4_F {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X4_F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union D2D_MATRIX_4X4_F_0 {
    pub Anonymous: D2D_MATRIX_4X4_F_0_0,
    pub m: [f32; 16],
}
impl D2D_MATRIX_4X4_F_0 {}
impl ::core::default::Default for D2D_MATRIX_4X4_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X4_F_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X4_F_0 {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X4_F_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_MATRIX_4X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
impl D2D_MATRIX_4X4_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_4X4_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_MATRIX_4X4_F_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_11", &self._11)
            .field("_12", &self._12)
            .field("_13", &self._13)
            .field("_14", &self._14)
            .field("_21", &self._21)
            .field("_22", &self._22)
            .field("_23", &self._23)
            .field("_24", &self._24)
            .field("_31", &self._31)
            .field("_32", &self._32)
            .field("_33", &self._33)
            .field("_34", &self._34)
            .field("_41", &self._41)
            .field("_42", &self._42)
            .field("_43", &self._43)
            .field("_44", &self._44)
            .finish()
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X4_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._14 == other._14 && self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._24 == other._24 && self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._34 == other._34 && self._41 == other._41 && self._42 == other._42 && self._43 == other._43 && self._44 == other._44
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X4_F_0_0 {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X4_F_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_MATRIX_5X4_F {
    pub Anonymous: D2D_MATRIX_5X4_F_0,
}
impl D2D_MATRIX_5X4_F {}
impl ::core::default::Default for D2D_MATRIX_5X4_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_5X4_F {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_5X4_F {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_5X4_F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union D2D_MATRIX_5X4_F_0 {
    pub Anonymous: D2D_MATRIX_5X4_F_0_0,
    pub m: [f32; 20],
}
impl D2D_MATRIX_5X4_F_0 {}
impl ::core::default::Default for D2D_MATRIX_5X4_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_5X4_F_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_5X4_F_0 {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_5X4_F_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_MATRIX_5X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
    pub _51: f32,
    pub _52: f32,
    pub _53: f32,
    pub _54: f32,
}
impl D2D_MATRIX_5X4_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_5X4_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_MATRIX_5X4_F_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_11", &self._11)
            .field("_12", &self._12)
            .field("_13", &self._13)
            .field("_14", &self._14)
            .field("_21", &self._21)
            .field("_22", &self._22)
            .field("_23", &self._23)
            .field("_24", &self._24)
            .field("_31", &self._31)
            .field("_32", &self._32)
            .field("_33", &self._33)
            .field("_34", &self._34)
            .field("_41", &self._41)
            .field("_42", &self._42)
            .field("_43", &self._43)
            .field("_44", &self._44)
            .field("_51", &self._51)
            .field("_52", &self._52)
            .field("_53", &self._53)
            .field("_54", &self._54)
            .finish()
    }
}
impl ::core::cmp::PartialEq for D2D_MATRIX_5X4_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11
            && self._12 == other._12
            && self._13 == other._13
            && self._14 == other._14
            && self._21 == other._21
            && self._22 == other._22
            && self._23 == other._23
            && self._24 == other._24
            && self._31 == other._31
            && self._32 == other._32
            && self._33 == other._33
            && self._34 == other._34
            && self._41 == other._41
            && self._42 == other._42
            && self._43 == other._43
            && self._44 == other._44
            && self._51 == other._51
            && self._52 == other._52
            && self._53 == other._53
            && self._54 == other._54
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_5X4_F_0_0 {}
unsafe impl ::windows::core::Abi for D2D_MATRIX_5X4_F_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_POINT_2F {
    pub x: f32,
    pub y: f32,
}
impl D2D_POINT_2F {}
impl ::core::default::Default for D2D_POINT_2F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_POINT_2F {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_POINT_2F").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_POINT_2F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for D2D_POINT_2F {}
unsafe impl ::windows::core::Abi for D2D_POINT_2F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_POINT_2U {
    pub x: u32,
    pub y: u32,
}
impl D2D_POINT_2U {}
impl ::core::default::Default for D2D_POINT_2U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_POINT_2U {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_POINT_2U").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_POINT_2U {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for D2D_POINT_2U {}
unsafe impl ::windows::core::Abi for D2D_POINT_2U {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl D2D_RECT_F {}
impl ::core::default::Default for D2D_RECT_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_RECT_F {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_RECT_F").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_RECT_F {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for D2D_RECT_F {}
unsafe impl ::windows::core::Abi for D2D_RECT_F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}
impl D2D_RECT_U {}
impl ::core::default::Default for D2D_RECT_U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_RECT_U {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_RECT_U").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_RECT_U {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for D2D_RECT_U {}
unsafe impl ::windows::core::Abi for D2D_RECT_U {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
impl D2D_SIZE_F {}
impl ::core::default::Default for D2D_SIZE_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_SIZE_F {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_SIZE_F").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_SIZE_F {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for D2D_SIZE_F {}
unsafe impl ::windows::core::Abi for D2D_SIZE_F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
impl D2D_SIZE_U {}
impl ::core::default::Default for D2D_SIZE_U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_SIZE_U {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_SIZE_U").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_SIZE_U {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for D2D_SIZE_U {}
unsafe impl ::windows::core::Abi for D2D_SIZE_U {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_VECTOR_2F {
    pub x: f32,
    pub y: f32,
}
impl D2D_VECTOR_2F {}
impl ::core::default::Default for D2D_VECTOR_2F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_VECTOR_2F {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_VECTOR_2F").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_VECTOR_2F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_2F {}
unsafe impl ::windows::core::Abi for D2D_VECTOR_2F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_VECTOR_3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl D2D_VECTOR_3F {}
impl ::core::default::Default for D2D_VECTOR_3F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_VECTOR_3F {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_VECTOR_3F").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_VECTOR_3F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_3F {}
unsafe impl ::windows::core::Abi for D2D_VECTOR_3F {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct D2D_VECTOR_4F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl D2D_VECTOR_4F {}
impl ::core::default::Default for D2D_VECTOR_4F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D2D_VECTOR_4F {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D2D_VECTOR_4F").field("x", &self.x).field("y", &self.y).field("z", &self.z).field("w", &self.w).finish()
    }
}
impl ::core::cmp::PartialEq for D2D_VECTOR_4F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_4F {}
unsafe impl ::windows::core::Abi for D2D_VECTOR_4F {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ID2D1SimplifiedGeometrySink(pub ::windows::core::IUnknown);
impl ID2D1SimplifiedGeometrySink {
    pub unsafe fn SetFillMode(&self, fillmode: D2D1_FILL_MODE) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(fillmode)))
    }
    pub unsafe fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(vertexflags)))
    }
    pub unsafe fn BeginFigure<'a, Param0: ::windows::core::IntoParam<'a, D2D_POINT_2F>>(&self, startpoint: Param0, figurebegin: D2D1_FIGURE_BEGIN) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), startpoint.into_param().abi(), ::core::mem::transmute(figurebegin)))
    }
    pub unsafe fn AddLines(&self, points: *const D2D_POINT_2F, pointscount: u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(points), ::core::mem::transmute(pointscount)))
    }
    pub unsafe fn AddBeziers(&self, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(beziers), ::core::mem::transmute(bezierscount)))
    }
    pub unsafe fn EndFigure(&self, figureend: D2D1_FIGURE_END) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(figureend)))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ID2D1SimplifiedGeometrySink {
    type Vtable = ID2D1SimplifiedGeometrySink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd9069e_12e2_11dc_9fed_001143a055f9);
}
impl ::core::convert::From<ID2D1SimplifiedGeometrySink> for ::windows::core::IUnknown {
    fn from(value: ID2D1SimplifiedGeometrySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ID2D1SimplifiedGeometrySink> for ::windows::core::IUnknown {
    fn from(value: &ID2D1SimplifiedGeometrySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID2D1SimplifiedGeometrySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ID2D1SimplifiedGeometrySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SimplifiedGeometrySink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fillmode: D2D1_FILL_MODE),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vertexflags: D2D1_PATH_SEGMENT),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, points: *const D2D_POINT_2F, pointscount: u32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, figureend: D2D1_FIGURE_END),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
