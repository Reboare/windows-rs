#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
pub struct TextPatternRangeEndpoint(pub i32);
impl TextPatternRangeEndpoint {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
}
impl ::core::marker::Copy for TextPatternRangeEndpoint {}
impl ::core::clone::Clone for TextPatternRangeEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextUnit(pub i32);
impl TextUnit {
    pub const Character: Self = Self(0i32);
    pub const Format: Self = Self(1i32);
    pub const Word: Self = Self(2i32);
    pub const Line: Self = Self(3i32);
    pub const Paragraph: Self = Self(4i32);
    pub const Page: Self = Self(5i32);
    pub const Document: Self = Self(6i32);
}
impl ::core::marker::Copy for TextUnit {}
impl ::core::clone::Clone for TextUnit {
    fn clone(&self) -> Self {
        *self
    }
}
