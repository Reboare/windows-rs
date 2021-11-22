mod implement;
mod implement_macro;

use gen::*;
use implement_macro::*;
use quote::*;
use reader::*;
use syn::parse_macro_input;

/// Rust structs can use the [`macro@implement`] attribute macro to implement entire WinRT or COM
/// classes or any combination of existing COM and WinRT interfaces.
///
/// If the attribute [`proc_macro::TokenStream`] contains the name of a WinRT class then all
/// of its interfaces are implemented. Otherwise, whatever interfaces are contained within
/// the attribute TokenStream are implemented.
#[proc_macro_attribute]
pub fn implement(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    implement::gen(attribute, input)
}
