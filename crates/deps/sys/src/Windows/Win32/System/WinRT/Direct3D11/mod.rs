#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type CreateDirect3D11DeviceFromDXGIDevice = unsafe extern "system" fn(dxgidevice: super::super::super::Graphics::Dxgi::IDXGIDevice, graphicsdevice: *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type CreateDirect3D11SurfaceFromDXGISurface = unsafe extern "system" fn(dgxisurface: super::super::super::Graphics::Dxgi::IDXGISurface, graphicssurface: *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
pub type IDirect3DDxgiInterfaceAccess = *mut ::core::ffi::c_void;
