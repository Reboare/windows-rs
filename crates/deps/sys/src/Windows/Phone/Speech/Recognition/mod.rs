#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
pub struct SpeechRecognitionUIStatus(pub i32);
impl SpeechRecognitionUIStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Busy: Self = Self(1i32);
    pub const Cancelled: Self = Self(2i32);
    pub const Preempted: Self = Self(3i32);
    pub const PrivacyPolicyDeclined: Self = Self(4i32);
}
impl ::core::marker::Copy for SpeechRecognitionUIStatus {}
impl ::core::clone::Clone for SpeechRecognitionUIStatus {
    fn clone(&self) -> Self {
        *self
    }
}
