#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_IITCmdInt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa2_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITDatabase: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66673452_8c23_11d0_a84e_00aa006c7d01);
pub const CLSID_IITDatabaseLocal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa9_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITGroupUpdate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa4_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITIndexBuild: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5aa_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_IITPropList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daae_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITResultSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa7_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITSvMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa3_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITWWFilterBuild: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5ab_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_IITWordWheel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd73725c2_8c12_11d0_a84e_00aa006c7d01);
pub const CLSID_IITWordWheelLocal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa8_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITWordWheelUpdate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa5_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_ITEngStemmer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a8_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_ITStdBreaker: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daaf_d393_11d0_9a56_00c04fb68bf7);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct COLUMNSTATUS {
    pub cPropCount: i32,
    pub cPropsLoaded: i32,
}
impl COLUMNSTATUS {}
impl ::core::default::Default for COLUMNSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COLUMNSTATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COLUMNSTATUS").field("cPropCount", &self.cPropCount).field("cPropsLoaded", &self.cPropsLoaded).finish()
    }
}
impl ::core::cmp::PartialEq for COLUMNSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cPropCount == other.cPropCount && self.cPropsLoaded == other.cPropsLoaded
    }
}
impl ::core::cmp::Eq for COLUMNSTATUS {}
unsafe impl ::windows::core::Abi for COLUMNSTATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CProperty {
    pub dwPropID: u32,
    pub cbData: u32,
    pub dwType: u32,
    pub Anonymous: CProperty_0,
    pub fPersist: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl CProperty {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CProperty {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CProperty {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CProperty {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CProperty {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CProperty_0 {
    pub lpszwData: super::super::Foundation::PWSTR,
    pub lpvData: *mut ::core::ffi::c_void,
    pub dwValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CProperty_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CProperty_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CProperty_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CProperty_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CProperty_0 {
    type Abi = Self;
}
pub const E_ALL_WILD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479467i32 as _);
pub const E_ALREADYINIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479421i32 as _);
pub const E_ALREADYOPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479533i32 as _);
pub const E_ASSERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479546i32 as _);
pub const E_BADBREAKER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479469i32 as _);
pub const E_BADFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479549i32 as _);
pub const E_BADFILTERSIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479528i32 as _);
pub const E_BADFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479548i32 as _);
pub const E_BADINDEXFLAGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479456i32 as _);
pub const E_BADPARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479535i32 as _);
pub const E_BADRANGEOP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479459i32 as _);
pub const E_BADVALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479468i32 as _);
pub const E_BADVERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479550i32 as _);
pub const E_CANTFINDDLL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479538i32 as _);
pub const E_DISKFULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479496i32 as _);
pub const E_DUPLICATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479551i32 as _);
pub const E_EXPECTEDTERM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479465i32 as _);
pub const E_FILECLOSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479503i32 as _);
pub const E_FILECREATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479504i32 as _);
pub const E_FILEDELETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479499i32 as _);
pub const E_FILEINVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479498i32 as _);
pub const E_FILENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479497i32 as _);
pub const E_FILEREAD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479502i32 as _);
pub const E_FILESEEK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479501i32 as _);
pub const E_FILEWRITE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479500i32 as _);
pub const E_GETLASTERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479536i32 as _);
pub const E_GROUPIDTOOBIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479542i32 as _);
pub const E_INTERRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479545i32 as _);
pub const E_INVALIDSTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479534i32 as _);
pub const E_MISSINGPROP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479424i32 as _);
pub const E_MISSLPAREN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479464i32 as _);
pub const E_MISSQUOTE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479462i32 as _);
pub const E_MISSRPAREN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479463i32 as _);
pub const E_NAMETOOLONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479520i32 as _);
pub const E_NOHANDLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479537i32 as _);
pub const E_NOKEYPROP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479417i32 as _);
pub const E_NOMERGEDDATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479540i32 as _);
pub const E_NOPERMISSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479547i32 as _);
pub const E_NOSTEMMER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479454i32 as _);
pub const E_NOTEXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479552i32 as _);
pub const E_NOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479539i32 as _);
pub const E_NOTINIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479420i32 as _);
pub const E_NOTOPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479533i32 as _);
pub const E_NOTSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479544i32 as _);
pub const E_NULLQUERY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479461i32 as _);
pub const E_OUTOFRANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479543i32 as _);
pub const E_PROPLISTEMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479422i32 as _);
pub const E_PROPLISTNOTEMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479423i32 as _);
pub const E_RESULTSETEMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479419i32 as _);
pub const E_STOPWORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479460i32 as _);
pub const E_TOODEEP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479466i32 as _);
pub const E_TOOMANYCOLUMNS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479418i32 as _);
pub const E_TOOMANYDUPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479471i32 as _);
pub const E_TOOMANYOBJECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479527i32 as _);
pub const E_TOOMANYTITLES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479541i32 as _);
pub const E_TOOMANYTOPICS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479472i32 as _);
pub const E_TREETOOBIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479470i32 as _);
pub const E_UNKNOWN_TRANSPORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479530i32 as _);
pub const E_UNMATCHEDTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479458i32 as _);
pub const E_UNSUPPORTED_TRANSPORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479529i32 as _);
pub const E_WILD_IN_DTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479455i32 as _);
pub const E_WORDTOOLONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479457i32 as _);
pub const HHACT_BACK: i32 = 7i32;
pub const HHACT_CONTRACT: i32 = 6i32;
pub const HHACT_CUSTOMIZE: i32 = 16i32;
pub const HHACT_EXPAND: i32 = 5i32;
pub const HHACT_FORWARD: i32 = 8i32;
pub const HHACT_HIGHLIGHT: i32 = 15i32;
pub const HHACT_HOME: i32 = 11i32;
pub const HHACT_JUMP1: i32 = 17i32;
pub const HHACT_JUMP2: i32 = 18i32;
pub const HHACT_LAST_ENUM: i32 = 23i32;
pub const HHACT_NOTES: i32 = 22i32;
pub const HHACT_OPTIONS: i32 = 13i32;
pub const HHACT_PRINT: i32 = 14i32;
pub const HHACT_REFRESH: i32 = 10i32;
pub const HHACT_STOP: i32 = 9i32;
pub const HHACT_SYNC: i32 = 12i32;
pub const HHACT_TAB_CONTENTS: i32 = 0i32;
pub const HHACT_TAB_FAVORITES: i32 = 4i32;
pub const HHACT_TAB_HISTORY: i32 = 3i32;
pub const HHACT_TAB_INDEX: i32 = 1i32;
pub const HHACT_TAB_SEARCH: i32 = 2i32;
pub const HHACT_TOC_NEXT: i32 = 20i32;
pub const HHACT_TOC_PREV: i32 = 21i32;
pub const HHACT_ZOOM: i32 = 19i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct HHNTRACK {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszCurUrl: super::super::Foundation::PSTR,
    pub idAction: i32,
    pub phhWinType: *mut HH_WINTYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl HHNTRACK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for HHNTRACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::fmt::Debug for HHNTRACK {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HHNTRACK").field("hdr", &self.hdr).field("pszCurUrl", &self.pszCurUrl).field("idAction", &self.idAction).field("phhWinType", &self.phhWinType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for HHNTRACK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszCurUrl == other.pszCurUrl && self.idAction == other.idAction && self.phhWinType == other.phhWinType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for HHNTRACK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::core::Abi for HHNTRACK {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct HHN_NOTIFY {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszUrl: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl HHN_NOTIFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for HHN_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::fmt::Debug for HHN_NOTIFY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HHN_NOTIFY").field("hdr", &self.hdr).field("pszUrl", &self.pszUrl).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for HHN_NOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszUrl == other.pszUrl
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for HHN_NOTIFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::core::Abi for HHN_NOTIFY {
    type Abi = Self;
}
pub const HHWIN_BUTTON_BACK: u32 = 4u32;
pub const HHWIN_BUTTON_BROWSE_BCK: u32 = 256u32;
pub const HHWIN_BUTTON_BROWSE_FWD: u32 = 128u32;
pub const HHWIN_BUTTON_CONTENTS: u32 = 1024u32;
pub const HHWIN_BUTTON_EXPAND: u32 = 2u32;
pub const HHWIN_BUTTON_FAVORITES: u32 = 131072u32;
pub const HHWIN_BUTTON_FORWARD: u32 = 8u32;
pub const HHWIN_BUTTON_HISTORY: u32 = 65536u32;
pub const HHWIN_BUTTON_HOME: u32 = 64u32;
pub const HHWIN_BUTTON_INDEX: u32 = 16384u32;
pub const HHWIN_BUTTON_JUMP1: u32 = 262144u32;
pub const HHWIN_BUTTON_JUMP2: u32 = 524288u32;
pub const HHWIN_BUTTON_NOTES: u32 = 512u32;
pub const HHWIN_BUTTON_OPTIONS: u32 = 4096u32;
pub const HHWIN_BUTTON_PRINT: u32 = 8192u32;
pub const HHWIN_BUTTON_REFRESH: u32 = 32u32;
pub const HHWIN_BUTTON_SEARCH: u32 = 32768u32;
pub const HHWIN_BUTTON_STOP: u32 = 16u32;
pub const HHWIN_BUTTON_SYNC: u32 = 2048u32;
pub const HHWIN_BUTTON_TOC_NEXT: u32 = 2097152u32;
pub const HHWIN_BUTTON_TOC_PREV: u32 = 4194304u32;
pub const HHWIN_BUTTON_ZOOM: u32 = 1048576u32;
pub const HHWIN_NAVTAB_BOTTOM: i32 = 2i32;
pub const HHWIN_NAVTAB_LEFT: i32 = 1i32;
pub const HHWIN_NAVTAB_TOP: i32 = 0i32;
pub const HHWIN_NAVTYPE_AUTHOR: i32 = 5i32;
pub const HHWIN_NAVTYPE_CUSTOM_FIRST: i32 = 11i32;
pub const HHWIN_NAVTYPE_FAVORITES: i32 = 3i32;
pub const HHWIN_NAVTYPE_HISTORY: i32 = 4i32;
pub const HHWIN_NAVTYPE_INDEX: i32 = 1i32;
pub const HHWIN_NAVTYPE_SEARCH: i32 = 2i32;
pub const HHWIN_NAVTYPE_TOC: i32 = 0i32;
pub const HHWIN_PARAM_CUR_TAB: u32 = 8192u32;
pub const HHWIN_PARAM_EXPANSION: u32 = 512u32;
pub const HHWIN_PARAM_EXSTYLES: u32 = 8u32;
pub const HHWIN_PARAM_HISTORY_COUNT: u32 = 4096u32;
pub const HHWIN_PARAM_INFOTYPES: u32 = 128u32;
pub const HHWIN_PARAM_NAV_WIDTH: u32 = 32u32;
pub const HHWIN_PARAM_PROPERTIES: u32 = 2u32;
pub const HHWIN_PARAM_RECT: u32 = 16u32;
pub const HHWIN_PARAM_SHOWSTATE: u32 = 64u32;
pub const HHWIN_PARAM_STYLES: u32 = 4u32;
pub const HHWIN_PARAM_TABORDER: u32 = 2048u32;
pub const HHWIN_PARAM_TABPOS: u32 = 1024u32;
pub const HHWIN_PARAM_TB_FLAGS: u32 = 256u32;
pub const HHWIN_PROP_AUTO_SYNC: u32 = 256u32;
pub const HHWIN_PROP_CHANGE_TITLE: u32 = 8192u32;
pub const HHWIN_PROP_MENU: u32 = 65536u32;
pub const HHWIN_PROP_NAV_ONLY_WIN: u32 = 16384u32;
pub const HHWIN_PROP_NODEF_EXSTYLES: u32 = 16u32;
pub const HHWIN_PROP_NODEF_STYLES: u32 = 8u32;
pub const HHWIN_PROP_NOTB_TEXT: u32 = 64u32;
pub const HHWIN_PROP_NOTITLEBAR: u32 = 4u32;
pub const HHWIN_PROP_NO_TOOLBAR: u32 = 32768u32;
pub const HHWIN_PROP_ONTOP: u32 = 2u32;
pub const HHWIN_PROP_POST_QUIT: u32 = 128u32;
pub const HHWIN_PROP_TAB_ADVSEARCH: u32 = 131072u32;
pub const HHWIN_PROP_TAB_AUTOHIDESHOW: u32 = 1u32;
pub const HHWIN_PROP_TAB_CUSTOM1: u32 = 524288u32;
pub const HHWIN_PROP_TAB_CUSTOM2: u32 = 1048576u32;
pub const HHWIN_PROP_TAB_CUSTOM3: u32 = 2097152u32;
pub const HHWIN_PROP_TAB_CUSTOM4: u32 = 4194304u32;
pub const HHWIN_PROP_TAB_CUSTOM5: u32 = 8388608u32;
pub const HHWIN_PROP_TAB_CUSTOM6: u32 = 16777216u32;
pub const HHWIN_PROP_TAB_CUSTOM7: u32 = 33554432u32;
pub const HHWIN_PROP_TAB_CUSTOM8: u32 = 67108864u32;
pub const HHWIN_PROP_TAB_CUSTOM9: u32 = 134217728u32;
pub const HHWIN_PROP_TAB_FAVORITES: u32 = 4096u32;
pub const HHWIN_PROP_TAB_HISTORY: u32 = 2048u32;
pub const HHWIN_PROP_TAB_SEARCH: u32 = 1024u32;
pub const HHWIN_PROP_TRACKING: u32 = 512u32;
pub const HHWIN_PROP_TRI_PANE: u32 = 32u32;
pub const HHWIN_PROP_USER_POS: u32 = 262144u32;
pub const HHWIN_TB_MARGIN: u32 = 268435456u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_AKLINK {
    pub cbStruct: i32,
    pub fReserved: super::super::Foundation::BOOL,
    pub pszKeywords: *mut i8,
    pub pszUrl: *mut i8,
    pub pszMsgText: *mut i8,
    pub pszMsgTitle: *mut i8,
    pub pszWindow: *mut i8,
    pub fIndexOnFail: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_AKLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_AKLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_AKLINK {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HH_AKLINK")
            .field("cbStruct", &self.cbStruct)
            .field("fReserved", &self.fReserved)
            .field("pszKeywords", &self.pszKeywords)
            .field("pszUrl", &self.pszUrl)
            .field("pszMsgText", &self.pszMsgText)
            .field("pszMsgTitle", &self.pszMsgTitle)
            .field("pszWindow", &self.pszWindow)
            .field("fIndexOnFail", &self.fIndexOnFail)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_AKLINK {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.fReserved == other.fReserved && self.pszKeywords == other.pszKeywords && self.pszUrl == other.pszUrl && self.pszMsgText == other.pszMsgText && self.pszMsgTitle == other.pszMsgTitle && self.pszWindow == other.pszWindow && self.fIndexOnFail == other.fIndexOnFail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_AKLINK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_AKLINK {
    type Abi = Self;
}
pub const HH_ALINK_LOOKUP: u32 = 19u32;
pub const HH_CLOSE_ALL: u32 = 18u32;
pub const HH_DISPLAY_INDEX: u32 = 2u32;
pub const HH_DISPLAY_SEARCH: u32 = 3u32;
pub const HH_DISPLAY_TEXT_POPUP: u32 = 14u32;
pub const HH_DISPLAY_TOC: u32 = 1u32;
pub const HH_DISPLAY_TOPIC: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_ENUM_CAT {
    pub cbStruct: i32,
    pub pszCatName: super::super::Foundation::PSTR,
    pub pszCatDescription: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_ENUM_CAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_ENUM_CAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_ENUM_CAT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HH_ENUM_CAT").field("cbStruct", &self.cbStruct).field("pszCatName", &self.pszCatName).field("pszCatDescription", &self.pszCatDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_ENUM_CAT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszCatName == other.pszCatName && self.pszCatDescription == other.pszCatDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_ENUM_CAT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_ENUM_CAT {
    type Abi = Self;
}
pub const HH_ENUM_CATEGORY: u32 = 21u32;
pub const HH_ENUM_CATEGORY_IT: u32 = 22u32;
pub const HH_ENUM_INFO_TYPE: u32 = 7u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_ENUM_IT {
    pub cbStruct: i32,
    pub iType: i32,
    pub pszCatName: super::super::Foundation::PSTR,
    pub pszITName: super::super::Foundation::PSTR,
    pub pszITDescription: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_ENUM_IT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_ENUM_IT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_ENUM_IT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HH_ENUM_IT").field("cbStruct", &self.cbStruct).field("iType", &self.iType).field("pszCatName", &self.pszCatName).field("pszITName", &self.pszITName).field("pszITDescription", &self.pszITDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_ENUM_IT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.iType == other.iType && self.pszCatName == other.pszCatName && self.pszITName == other.pszITName && self.pszITDescription == other.pszITDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_ENUM_IT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_ENUM_IT {
    type Abi = Self;
}
pub const HH_FTS_DEFAULT_PROXIMITY: i32 = -1i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_FTS_QUERY {
    pub cbStruct: i32,
    pub fUniCodeStrings: super::super::Foundation::BOOL,
    pub pszSearchQuery: *mut i8,
    pub iProximity: i32,
    pub fStemmedSearch: super::super::Foundation::BOOL,
    pub fTitleOnly: super::super::Foundation::BOOL,
    pub fExecute: super::super::Foundation::BOOL,
    pub pszWindow: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_FTS_QUERY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_FTS_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_FTS_QUERY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HH_FTS_QUERY")
            .field("cbStruct", &self.cbStruct)
            .field("fUniCodeStrings", &self.fUniCodeStrings)
            .field("pszSearchQuery", &self.pszSearchQuery)
            .field("iProximity", &self.iProximity)
            .field("fStemmedSearch", &self.fStemmedSearch)
            .field("fTitleOnly", &self.fTitleOnly)
            .field("fExecute", &self.fExecute)
            .field("pszWindow", &self.pszWindow)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_FTS_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.fUniCodeStrings == other.fUniCodeStrings && self.pszSearchQuery == other.pszSearchQuery && self.iProximity == other.iProximity && self.fStemmedSearch == other.fStemmedSearch && self.fTitleOnly == other.fTitleOnly && self.fExecute == other.fExecute && self.pszWindow == other.pszWindow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_FTS_QUERY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_FTS_QUERY {
    type Abi = Self;
}
pub const HH_GET_LAST_ERROR: u32 = 20u32;
pub const HH_GET_WIN_HANDLE: u32 = 6u32;
pub const HH_GET_WIN_TYPE: u32 = 5u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for HH_GLOBAL_PROPERTY {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct HH_GLOBAL_PROPERTY {
    pub id: HH_GPROPID,
    pub var: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl HH_GLOBAL_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for HH_GLOBAL_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for HH_GLOBAL_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for HH_GLOBAL_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for HH_GLOBAL_PROPERTY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HH_GPROPID(pub i32);
pub const HH_GPROPID_SINGLETHREAD: HH_GPROPID = HH_GPROPID(1i32);
pub const HH_GPROPID_TOOLBAR_MARGIN: HH_GPROPID = HH_GPROPID(2i32);
pub const HH_GPROPID_UI_LANGUAGE: HH_GPROPID = HH_GPROPID(3i32);
pub const HH_GPROPID_CURRENT_SUBSET: HH_GPROPID = HH_GPROPID(4i32);
pub const HH_GPROPID_CONTENT_LANGUAGE: HH_GPROPID = HH_GPROPID(5i32);
impl ::core::convert::From<i32> for HH_GPROPID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HH_GPROPID {
    type Abi = Self;
}
pub const HH_HELP_CONTEXT: u32 = 15u32;
pub const HH_HELP_FINDER: u32 = 0u32;
pub const HH_INITIALIZE: u32 = 28u32;
pub const HH_KEYWORD_LOOKUP: u32 = 13u32;
pub const HH_MAX_TABS: u32 = 19u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_POPUP {
    pub cbStruct: i32,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub idString: u32,
    pub pszText: *mut i8,
    pub pt: super::super::Foundation::POINT,
    pub clrForeground: u32,
    pub clrBackground: u32,
    pub rcMargins: super::super::Foundation::RECT,
    pub pszFont: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_POPUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_POPUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_POPUP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HH_POPUP")
            .field("cbStruct", &self.cbStruct)
            .field("hinst", &self.hinst)
            .field("idString", &self.idString)
            .field("pszText", &self.pszText)
            .field("pt", &self.pt)
            .field("clrForeground", &self.clrForeground)
            .field("clrBackground", &self.clrBackground)
            .field("rcMargins", &self.rcMargins)
            .field("pszFont", &self.pszFont)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_POPUP {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.hinst == other.hinst && self.idString == other.idString && self.pszText == other.pszText && self.pt == other.pt && self.clrForeground == other.clrForeground && self.clrBackground == other.clrBackground && self.rcMargins == other.rcMargins && self.pszFont == other.pszFont
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_POPUP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_POPUP {
    type Abi = Self;
}
pub const HH_PRETRANSLATEMESSAGE: u32 = 253u32;
pub const HH_RESERVED1: u32 = 10u32;
pub const HH_RESERVED2: u32 = 11u32;
pub const HH_RESERVED3: u32 = 12u32;
pub const HH_RESET_IT_FILTER: u32 = 23u32;
pub const HH_SAFE_DISPLAY_TOPIC: u32 = 32u32;
pub const HH_SET_EXCLUSIVE_FILTER: u32 = 25u32;
pub const HH_SET_GLOBAL_PROPERTY: u32 = 252u32;
pub const HH_SET_INCLUSIVE_FILTER: u32 = 24u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_SET_INFOTYPE {
    pub cbStruct: i32,
    pub pszCatName: super::super::Foundation::PSTR,
    pub pszInfoTypeName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_SET_INFOTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_SET_INFOTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_SET_INFOTYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HH_SET_INFOTYPE").field("cbStruct", &self.cbStruct).field("pszCatName", &self.pszCatName).field("pszInfoTypeName", &self.pszInfoTypeName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_SET_INFOTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszCatName == other.pszCatName && self.pszInfoTypeName == other.pszInfoTypeName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_SET_INFOTYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_SET_INFOTYPE {
    type Abi = Self;
}
pub const HH_SET_INFO_TYPE: u32 = 8u32;
pub const HH_SET_QUERYSERVICE: u32 = 30u32;
pub const HH_SET_WIN_TYPE: u32 = 4u32;
pub const HH_SYNC: u32 = 9u32;
pub const HH_TAB_AUTHOR: i32 = 5i32;
pub const HH_TAB_CONTENTS: i32 = 0i32;
pub const HH_TAB_CUSTOM_FIRST: i32 = 11i32;
pub const HH_TAB_CUSTOM_LAST: i32 = 19i32;
pub const HH_TAB_FAVORITES: i32 = 3i32;
pub const HH_TAB_HISTORY: i32 = 4i32;
pub const HH_TAB_INDEX: i32 = 1i32;
pub const HH_TAB_SEARCH: i32 = 2i32;
pub const HH_TP_HELP_CONTEXTMENU: u32 = 16u32;
pub const HH_TP_HELP_WM_HELP: u32 = 17u32;
pub const HH_UNINITIALIZE: u32 = 29u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_WINTYPE {
    pub cbStruct: i32,
    pub fUniCodeStrings: super::super::Foundation::BOOL,
    pub pszType: *mut i8,
    pub fsValidMembers: u32,
    pub fsWinProperties: u32,
    pub pszCaption: *mut i8,
    pub dwStyles: u32,
    pub dwExStyles: u32,
    pub rcWindowPos: super::super::Foundation::RECT,
    pub nShowState: i32,
    pub hwndHelp: super::super::Foundation::HWND,
    pub hwndCaller: super::super::Foundation::HWND,
    pub paInfoTypes: *mut u32,
    pub hwndToolBar: super::super::Foundation::HWND,
    pub hwndNavigation: super::super::Foundation::HWND,
    pub hwndHTML: super::super::Foundation::HWND,
    pub iNavWidth: i32,
    pub rcHTML: super::super::Foundation::RECT,
    pub pszToc: *mut i8,
    pub pszIndex: *mut i8,
    pub pszFile: *mut i8,
    pub pszHome: *mut i8,
    pub fsToolBarFlags: u32,
    pub fNotExpanded: super::super::Foundation::BOOL,
    pub curNavType: i32,
    pub tabpos: i32,
    pub idNotify: i32,
    pub tabOrder: [u8; 20],
    pub cHistory: i32,
    pub pszJump1: *mut i8,
    pub pszJump2: *mut i8,
    pub pszUrlJump1: *mut i8,
    pub pszUrlJump2: *mut i8,
    pub rcMinSize: super::super::Foundation::RECT,
    pub cbInfoTypes: i32,
    pub pszCustomTabs: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_WINTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_WINTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_WINTYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HH_WINTYPE")
            .field("cbStruct", &self.cbStruct)
            .field("fUniCodeStrings", &self.fUniCodeStrings)
            .field("pszType", &self.pszType)
            .field("fsValidMembers", &self.fsValidMembers)
            .field("fsWinProperties", &self.fsWinProperties)
            .field("pszCaption", &self.pszCaption)
            .field("dwStyles", &self.dwStyles)
            .field("dwExStyles", &self.dwExStyles)
            .field("rcWindowPos", &self.rcWindowPos)
            .field("nShowState", &self.nShowState)
            .field("hwndHelp", &self.hwndHelp)
            .field("hwndCaller", &self.hwndCaller)
            .field("paInfoTypes", &self.paInfoTypes)
            .field("hwndToolBar", &self.hwndToolBar)
            .field("hwndNavigation", &self.hwndNavigation)
            .field("hwndHTML", &self.hwndHTML)
            .field("iNavWidth", &self.iNavWidth)
            .field("rcHTML", &self.rcHTML)
            .field("pszToc", &self.pszToc)
            .field("pszIndex", &self.pszIndex)
            .field("pszFile", &self.pszFile)
            .field("pszHome", &self.pszHome)
            .field("fsToolBarFlags", &self.fsToolBarFlags)
            .field("fNotExpanded", &self.fNotExpanded)
            .field("curNavType", &self.curNavType)
            .field("tabpos", &self.tabpos)
            .field("idNotify", &self.idNotify)
            .field("tabOrder", &self.tabOrder)
            .field("cHistory", &self.cHistory)
            .field("pszJump1", &self.pszJump1)
            .field("pszJump2", &self.pszJump2)
            .field("pszUrlJump1", &self.pszUrlJump1)
            .field("pszUrlJump2", &self.pszUrlJump2)
            .field("rcMinSize", &self.rcMinSize)
            .field("cbInfoTypes", &self.cbInfoTypes)
            .field("pszCustomTabs", &self.pszCustomTabs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_WINTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fUniCodeStrings == other.fUniCodeStrings
            && self.pszType == other.pszType
            && self.fsValidMembers == other.fsValidMembers
            && self.fsWinProperties == other.fsWinProperties
            && self.pszCaption == other.pszCaption
            && self.dwStyles == other.dwStyles
            && self.dwExStyles == other.dwExStyles
            && self.rcWindowPos == other.rcWindowPos
            && self.nShowState == other.nShowState
            && self.hwndHelp == other.hwndHelp
            && self.hwndCaller == other.hwndCaller
            && self.paInfoTypes == other.paInfoTypes
            && self.hwndToolBar == other.hwndToolBar
            && self.hwndNavigation == other.hwndNavigation
            && self.hwndHTML == other.hwndHTML
            && self.iNavWidth == other.iNavWidth
            && self.rcHTML == other.rcHTML
            && self.pszToc == other.pszToc
            && self.pszIndex == other.pszIndex
            && self.pszFile == other.pszFile
            && self.pszHome == other.pszHome
            && self.fsToolBarFlags == other.fsToolBarFlags
            && self.fNotExpanded == other.fNotExpanded
            && self.curNavType == other.curNavType
            && self.tabpos == other.tabpos
            && self.idNotify == other.idNotify
            && self.tabOrder == other.tabOrder
            && self.cHistory == other.cHistory
            && self.pszJump1 == other.pszJump1
            && self.pszJump2 == other.pszJump2
            && self.pszUrlJump1 == other.pszUrlJump1
            && self.pszUrlJump2 == other.pszUrlJump2
            && self.rcMinSize == other.rcMinSize
            && self.cbInfoTypes == other.cbInfoTypes
            && self.pszCustomTabs == other.pszCustomTabs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_WINTYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_WINTYPE {
    type Abi = Self;
}
pub const IDTB_BACK: u32 = 204u32;
pub const IDTB_BROWSE_BACK: u32 = 212u32;
pub const IDTB_BROWSE_FWD: u32 = 211u32;
pub const IDTB_CONTENTS: u32 = 213u32;
pub const IDTB_CONTRACT: u32 = 201u32;
pub const IDTB_CUSTOMIZE: u32 = 221u32;
pub const IDTB_EXPAND: u32 = 200u32;
pub const IDTB_FAVORITES: u32 = 217u32;
pub const IDTB_FORWARD: u32 = 209u32;
pub const IDTB_HISTORY: u32 = 216u32;
pub const IDTB_HOME: u32 = 205u32;
pub const IDTB_INDEX: u32 = 214u32;
pub const IDTB_JUMP1: u32 = 218u32;
pub const IDTB_JUMP2: u32 = 219u32;
pub const IDTB_NOTES: u32 = 210u32;
pub const IDTB_OPTIONS: u32 = 208u32;
pub const IDTB_PRINT: u32 = 207u32;
pub const IDTB_REFRESH: u32 = 203u32;
pub const IDTB_SEARCH: u32 = 215u32;
pub const IDTB_STOP: u32 = 202u32;
pub const IDTB_SYNC: u32 = 206u32;
pub const IDTB_TOC_NEXT: u32 = 223u32;
pub const IDTB_TOC_PREV: u32 = 224u32;
pub const IDTB_ZOOM: u32 = 222u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IITDatabase(pub ::windows::core::IUnknown);
impl IITDatabase {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpszhost: Param0, lpszmoniker: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpszhost.into_param().abi(), lpszmoniker.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CreateObject(&self, rclsid: *const ::windows::core::GUID, pdwobjinstance: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(pdwobjinstance)).ok()
    }
    pub unsafe fn GetObject(&self, dwobjinstance: u32, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwobjinstance), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectPersistence<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, lpwszobject: Param0, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), lpwszobject.into_param().abi(), ::core::mem::transmute(dwobjinstance), ::core::mem::transmute(ppvpersistence), fstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IITDatabase {
    type Vtable = IITDatabase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a2_dedf_11d0_9a61_00c04fb68bf7);
}
impl ::core::convert::From<IITDatabase> for ::windows::core::IUnknown {
    fn from(value: IITDatabase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IITDatabase> for ::windows::core::IUnknown {
    fn from(value: &IITDatabase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IITDatabase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IITDatabase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITDatabase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpszhost: super::super::Foundation::PWSTR, lpszmoniker: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwobjinstance: u32, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpwszobject: super::super::Foundation::PWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IITGroup(pub u8);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IITPropList(pub ::windows::core::IUnknown);
impl IITPropList {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Set<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, propid: u32, lpszwstring: Param1, dwoperation: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), lpszwstring.into_param().abi(), ::core::mem::transmute(dwoperation)).ok()
    }
    pub unsafe fn Set2(&self, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(lpvdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(dwoperation)).ok()
    }
    pub unsafe fn Set3(&self, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(dwdata), ::core::mem::transmute(dwoperation)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add(&self, prop: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(prop)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Get(&self, propid: u32, property: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(property)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersist<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fpersist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), fpersist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersist2<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, propid: u32, fpersist: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), fpersist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFirst(&self, property: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(property)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNext(&self, property: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(property)).ok()
    }
    pub unsafe fn GetPropCount(&self, cprop: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(cprop)).ok()
    }
    pub unsafe fn SaveHeader(&self, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvdata), ::core::mem::transmute(dwhdrsize)).ok()
    }
    pub unsafe fn SaveData(&self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvheader), ::core::mem::transmute(dwhdrsize), ::core::mem::transmute(lpvdata), ::core::mem::transmute(dwbufsize)).ok()
    }
    pub unsafe fn GetHeaderSize(&self, dwhdrsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwhdrsize)).ok()
    }
    pub unsafe fn GetDataSize(&self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvheader), ::core::mem::transmute(dwhdrsize), ::core::mem::transmute(dwdatasize)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveDataToStream<'a, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvheader), ::core::mem::transmute(dwhdrsize), pstream.into_param().abi()).ok()
    }
    pub unsafe fn LoadFromMem(&self, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvdata), ::core::mem::transmute(dwbufsize)).ok()
    }
    pub unsafe fn SaveToMem(&self, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvdata), ::core::mem::transmute(dwbufsize)).ok()
    }
}
unsafe impl ::windows::core::Interface for IITPropList {
    type Vtable = IITPropList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f403bb1_9997_11d0_a850_00aa006c7d01);
}
impl ::core::convert::From<IITPropList> for ::windows::core::IUnknown {
    fn from(value: IITPropList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IITPropList> for ::windows::core::IUnknown {
    fn from(value: &IITPropList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IITPropList> for super::super::System::Com::IPersistStreamInit {
    fn from(value: IITPropList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IITPropList> for super::super::System::Com::IPersistStreamInit {
    fn from(value: &IITPropList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersistStreamInit> for IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersistStreamInit> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersistStreamInit> for &IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersistStreamInit> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IITPropList> for super::super::System::Com::IPersist {
    fn from(value: IITPropList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IITPropList> for super::super::System::Com::IPersist {
    fn from(value: &IITPropList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersist> for IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersist> for &IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITPropList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbsize: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32, lpszwstring: super::super::Foundation::PWSTR, dwoperation: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prop: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32, property: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, property: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, property: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cprop: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwhdrsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IITQuery(pub u8);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IITResultSet(pub ::windows::core::IUnknown);
impl IITResultSet {
    pub unsafe fn SetColumnPriority(&self, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(columnpriority)).ok()
    }
    pub unsafe fn SetColumnHeap(&self, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: ::core::option::Option<PFNCOLHEAPFREE>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(lpvheap), ::core::mem::transmute(pfncolheapfree)).ok()
    }
    pub unsafe fn SetKeyProp(&self, propid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid)).ok()
    }
    pub unsafe fn Add(&self, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(dwdefaultdata), ::core::mem::transmute(priority)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add2<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, propid: u32, lpszwdefault: Param1, priority: PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), lpszwdefault.into_param().abi(), ::core::mem::transmute(priority)).ok()
    }
    pub unsafe fn Add3(&self, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(lpvdefaultdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(priority)).ok()
    }
    pub unsafe fn Add4(&self, lpvhdr: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvhdr)).ok()
    }
    pub unsafe fn Append(&self, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvhdr), ::core::mem::transmute(lpvdata)).ok()
    }
    pub unsafe fn Set(&self, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(lpvdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Set2<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lrowindex: i32, lcolumnindex: i32, lpwstr: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lcolumnindex), lpwstr.into_param().abi()).ok()
    }
    pub unsafe fn Set3(&self, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(dwdata)).ok()
    }
    pub unsafe fn Set4(&self, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lpvhdr), ::core::mem::transmute(lpvdata)).ok()
    }
    pub unsafe fn Copy<'a, Param0: ::windows::core::IntoParam<'a, IITResultSet>>(&self, prscopy: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), prscopy.into_param().abi()).ok()
    }
    pub unsafe fn AppendRows<'a, Param0: ::windows::core::IntoParam<'a, IITResultSet>>(&self, pressrc: Param0, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pressrc.into_param().abi(), ::core::mem::transmute(lrowsrcfirst), ::core::mem::transmute(csrcrows), ::core::mem::transmute(lrowfirstdest)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Get(&self, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(prop)).ok()
    }
    pub unsafe fn GetKeyProp(&self, keypropid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(keypropid)).ok()
    }
    pub unsafe fn GetColumnPriority(&self, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(columnpriority)).ok()
    }
    pub unsafe fn GetRowCount(&self, lnumberofrows: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnumberofrows)).ok()
    }
    pub unsafe fn GetColumnCount(&self, lnumberofcolumns: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnumberofcolumns)).ok()
    }
    pub unsafe fn GetColumn(&self, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(propid), ::core::mem::transmute(dwtype), ::core::mem::transmute(lpvdefaultvalue), ::core::mem::transmute(cbsize), ::core::mem::transmute(columnpriority)).ok()
    }
    pub unsafe fn GetColumn2(&self, lcolumnindex: i32, propid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(propid)).ok()
    }
    pub unsafe fn GetColumnFromPropID(&self, propid: u32, lcolumnindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(lcolumnindex)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ClearRows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Free(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Pause<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fpause: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), fpause.into_param().abi()).ok()
    }
    pub unsafe fn GetRowStatus(&self, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowfirst), ::core::mem::transmute(crows), ::core::mem::transmute(lprowstatus)).ok()
    }
    pub unsafe fn GetColumnStatus(&self, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpcolstatus)).ok()
    }
}
unsafe impl ::windows::core::Interface for IITResultSet {
    type Vtable = IITResultSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bb91d41_998b_11d0_a850_00aa006c7d01);
}
impl ::core::convert::From<IITResultSet> for ::windows::core::IUnknown {
    fn from(value: IITResultSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IITResultSet> for ::windows::core::IUnknown {
    fn from(value: &IITResultSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IITResultSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IITResultSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITResultSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32, lpszwdefault: super::super::Foundation::PWSTR, priority: PRIORITY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpvhdr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lrowindex: i32, lcolumnindex: i32, lpwstr: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prscopy: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pressrc: ::windows::core::RawPtr, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, keypropid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lnumberofrows: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lnumberofcolumns: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcolumnindex: i32, propid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: u32, lcolumnindex: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fpause: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IITStopWordList(pub u8);
pub const IITWBC_BREAK_ACCEPT_WILDCARDS: u32 = 1u32;
pub const IITWBC_BREAK_AND_STEM: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IITWordWheel(pub ::windows::core::IUnknown);
impl IITWordWheel {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, IITDatabase>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpitdb: Param0, lpszmoniker: Param1, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpitdb.into_param().abi(), lpszmoniker.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcodepageid), ::core::mem::transmute(plcid)).ok()
    }
    pub unsafe fn GetSorterInstance(&self, pdwobjinstance: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwobjinstance)).ok()
    }
    pub unsafe fn Count(&self, pcentries: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcentries)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lookup<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, lpcvprefix: *const ::core::ffi::c_void, fexactmatch: Param1, plentry: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpcvprefix), fexactmatch.into_param().abi(), ::core::mem::transmute(plentry)).ok()
    }
    pub unsafe fn Lookup2<'a, Param1: ::windows::core::IntoParam<'a, IITResultSet>>(&self, lentry: i32, lpitresult: Param1, centries: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lentry), lpitresult.into_param().abi(), ::core::mem::transmute(centries)).ok()
    }
    pub unsafe fn Lookup3(&self, lentry: i32, lpvkeybuf: *mut ::core::ffi::c_void, cbkeybuf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lentry), ::core::mem::transmute(lpvkeybuf), ::core::mem::transmute(cbkeybuf)).ok()
    }
    pub unsafe fn SetGroup(&self, piitgroup: *mut IITGroup) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(piitgroup)).ok()
    }
    pub unsafe fn GetGroup(&self, ppiitgroup: *mut *mut IITGroup) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppiitgroup)).ok()
    }
    pub unsafe fn GetDataCount(&self, lentry: i32, pdwcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(lentry), ::core::mem::transmute(pdwcount)).ok()
    }
    pub unsafe fn GetData<'a, Param1: ::windows::core::IntoParam<'a, IITResultSet>>(&self, lentry: i32, lpitresult: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(lentry), lpitresult.into_param().abi()).ok()
    }
    pub unsafe fn GetDataColumns<'a, Param0: ::windows::core::IntoParam<'a, IITResultSet>>(&self, prs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), prs.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IITWordWheel {
    type Vtable = IITWordWheel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a4_dedf_11d0_9a61_00c04fb68bf7);
}
impl ::core::convert::From<IITWordWheel> for ::windows::core::IUnknown {
    fn from(value: IITWordWheel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IITWordWheel> for ::windows::core::IUnknown {
    fn from(value: &IITWordWheel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IITWordWheel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IITWordWheel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITWordWheel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpitdb: ::windows::core::RawPtr, lpszmoniker: super::super::Foundation::PWSTR, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcentries: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpcvprefix: *const ::core::ffi::c_void, fexactmatch: super::super::Foundation::BOOL, plentry: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lentry: i32, lpitresult: ::windows::core::RawPtr, centries: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lentry: i32, lpvkeybuf: *mut ::core::ffi::c_void, cbkeybuf: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piitgroup: *mut IITGroup) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiitgroup: *mut *mut IITGroup) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lentry: i32, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lentry: i32, lpitresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prs: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStemSink(pub ::windows::core::IUnknown);
impl IStemSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PutAltWord<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwcinbuf: Param0, cwc: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwcinbuf.into_param().abi(), ::core::mem::transmute(cwc)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PutWord<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwcinbuf: Param0, cwc: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwcinbuf.into_param().abi(), ::core::mem::transmute(cwc)).ok()
    }
}
unsafe impl ::windows::core::Interface for IStemSink {
    type Vtable = IStemSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe77c330_7f42_11ce_be57_00aa0051fe20);
}
impl ::core::convert::From<IStemSink> for ::windows::core::IUnknown {
    fn from(value: IStemSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStemSink> for ::windows::core::IUnknown {
    fn from(value: &IStemSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStemSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStemmerConfig(pub ::windows::core::IUnknown);
impl IStemmerConfig {
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcodepageid), ::core::mem::transmute(lcid)).ok()
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcodepageid), ::core::mem::transmute(plcid)).ok()
    }
    pub unsafe fn SetControlInfo(&self, grfstemflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfstemflags), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetControlInfo(&self, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgrfstemflags), ::core::mem::transmute(pdwreserved)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalStemmerData<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0, dwextdatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ::core::mem::transmute(dwextdatatype)).ok()
    }
}
unsafe impl ::windows::core::Interface for IStemmerConfig {
    type Vtable = IStemmerConfig_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a7_dedf_11d0_9a61_00c04fb68bf7);
}
impl ::core::convert::From<IStemmerConfig> for ::windows::core::IUnknown {
    fn from(value: IStemmerConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStemmerConfig> for ::windows::core::IUnknown {
    fn from(value: &IStemmerConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStemmerConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStemmerConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStemmerConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfstemflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstream: ::windows::core::RawPtr, dwextdatatype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
pub const ITWW_CBKEY_MAX: u32 = 1024u32;
pub const ITWW_OPEN_NOCONNECT: u32 = 1u32;
pub const IT_EXCLUSIVE: i32 = 1i32;
pub const IT_HIDDEN: i32 = 2i32;
pub const IT_INCLUSIVE: i32 = 0i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWordBreakerConfig(pub ::windows::core::IUnknown);
impl IWordBreakerConfig {
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcodepageid), ::core::mem::transmute(lcid)).ok()
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcodepageid), ::core::mem::transmute(plcid)).ok()
    }
    pub unsafe fn SetBreakWordType(&self, dwbreakwordtype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbreakwordtype)).ok()
    }
    pub unsafe fn GetBreakWordType(&self, pdwbreakwordtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbreakwordtype)).ok()
    }
    pub unsafe fn SetControlInfo(&self, grfbreakflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbreakflags), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetControlInfo(&self, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgrfbreakflags), ::core::mem::transmute(pdwreserved)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalBreakerData<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0, dwextdatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ::core::mem::transmute(dwextdatatype)).ok()
    }
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn SetWordStemmer<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Search::IStemmer>>(&self, rclsid: *const ::windows::core::GUID, pstemmer: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), pstemmer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn GetWordStemmer(&self) -> ::windows::core::Result<super::super::System::Search::IStemmer> {
        let mut result__: <super::super::System::Search::IStemmer as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Search::IStemmer>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWordBreakerConfig {
    type Vtable = IWordBreakerConfig_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a6_dedf_11d0_9a61_00c04fb68bf7);
}
impl ::core::convert::From<IWordBreakerConfig> for ::windows::core::IUnknown {
    fn from(value: IWordBreakerConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWordBreakerConfig> for ::windows::core::IUnknown {
    fn from(value: &IWordBreakerConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWordBreakerConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWordBreakerConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordBreakerConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwbreakwordtype: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwbreakwordtype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbreakflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstream: ::windows::core::RawPtr, dwextdatatype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Search")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, pstemmer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))] usize,
    #[cfg(feature = "Win32_System_Search")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppstemmer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))] usize,
);
pub const MAX_COLUMNS: u32 = 256u32;
pub type PFNCOLHEAPFREE = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PRIORITY(pub i32);
pub const PRIORITY_LOW: PRIORITY = PRIORITY(0i32);
pub const PRIORITY_NORMAL: PRIORITY = PRIORITY(1i32);
pub const PRIORITY_HIGH: PRIORITY = PRIORITY(2i32);
impl ::core::convert::From<i32> for PRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PRIORITY {
    type Abi = Self;
}
pub const PROP_ADD: u32 = 0u32;
pub const PROP_DELETE: u32 = 1u32;
pub const PROP_UPDATE: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ROWSTATUS {
    pub lRowFirst: i32,
    pub cRows: i32,
    pub cProperties: i32,
    pub cRowsTotal: i32,
}
impl ROWSTATUS {}
impl ::core::default::Default for ROWSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ROWSTATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ROWSTATUS").field("lRowFirst", &self.lRowFirst).field("cRows", &self.cRows).field("cProperties", &self.cProperties).field("cRowsTotal", &self.cRowsTotal).finish()
    }
}
impl ::core::cmp::PartialEq for ROWSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.lRowFirst == other.lRowFirst && self.cRows == other.cRows && self.cProperties == other.cProperties && self.cRowsTotal == other.cRowsTotal
    }
}
impl ::core::cmp::Eq for ROWSTATUS {}
unsafe impl ::windows::core::Abi for ROWSTATUS {
    type Abi = Self;
}
pub const STDPROP_DISPLAYKEY: u32 = 101u32;
pub const STDPROP_INDEX_BREAK: u32 = 204u32;
pub const STDPROP_INDEX_DTYPE: u32 = 202u32;
pub const STDPROP_INDEX_LENGTH: u32 = 203u32;
pub const STDPROP_INDEX_TERM: u32 = 210u32;
pub const STDPROP_INDEX_TERM_RAW_LENGTH: u32 = 211u32;
pub const STDPROP_INDEX_TEXT: u32 = 200u32;
pub const STDPROP_INDEX_VFLD: u32 = 201u32;
pub const STDPROP_KEY: u32 = 4u32;
pub const STDPROP_SORTKEY: u32 = 100u32;
pub const STDPROP_SORTORDINAL: u32 = 102u32;
pub const STDPROP_TITLE: u32 = 2u32;
pub const STDPROP_UID: u32 = 1u32;
pub const STDPROP_USERDATA: u32 = 3u32;
pub const STDPROP_USERPROP_BASE: u32 = 65536u32;
pub const STDPROP_USERPROP_MAX: u32 = 2147483647u32;
pub const TYPE_POINTER: u32 = 1u32;
pub const TYPE_STRING: u32 = 2u32;
pub const TYPE_VALUE: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WORD_WHEEL_OPEN_FLAGS(pub u32);
pub const ITWW_OPEN_CONNECT: WORD_WHEEL_OPEN_FLAGS = WORD_WHEEL_OPEN_FLAGS(0u32);
impl ::core::convert::From<u32> for WORD_WHEEL_OPEN_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WORD_WHEEL_OPEN_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
