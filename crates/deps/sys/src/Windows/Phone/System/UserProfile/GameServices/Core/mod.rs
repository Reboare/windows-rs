#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
pub struct GameServiceGameOutcome(pub i32);
impl GameServiceGameOutcome {
    pub const None: Self = Self(0i32);
    pub const Win: Self = Self(1i32);
    pub const Loss: Self = Self(2i32);
    pub const Tie: Self = Self(3i32);
}
impl ::core::marker::Copy for GameServiceGameOutcome {}
impl ::core::clone::Clone for GameServiceGameOutcome {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GameServicePropertyCollection = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct GameServiceScoreKind(pub i32);
impl GameServiceScoreKind {
    pub const Number: Self = Self(0i32);
    pub const Time: Self = Self(1i32);
}
impl ::core::marker::Copy for GameServiceScoreKind {}
impl ::core::clone::Clone for GameServiceScoreKind {
    fn clone(&self) -> Self {
        *self
    }
}
