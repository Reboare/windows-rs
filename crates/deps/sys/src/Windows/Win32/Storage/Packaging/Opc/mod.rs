#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type IOpcCertificateEnumerator = *mut ::core::ffi::c_void;
pub type IOpcCertificateSet = *mut ::core::ffi::c_void;
pub type IOpcDigitalSignature = *mut ::core::ffi::c_void;
pub type IOpcDigitalSignatureEnumerator = *mut ::core::ffi::c_void;
pub type IOpcDigitalSignatureManager = *mut ::core::ffi::c_void;
pub type IOpcFactory = *mut ::core::ffi::c_void;
pub type IOpcPackage = *mut ::core::ffi::c_void;
pub type IOpcPart = *mut ::core::ffi::c_void;
pub type IOpcPartEnumerator = *mut ::core::ffi::c_void;
pub type IOpcPartSet = *mut ::core::ffi::c_void;
pub type IOpcPartUri = *mut ::core::ffi::c_void;
pub type IOpcRelationship = *mut ::core::ffi::c_void;
pub type IOpcRelationshipEnumerator = *mut ::core::ffi::c_void;
pub type IOpcRelationshipSelector = *mut ::core::ffi::c_void;
pub type IOpcRelationshipSelectorEnumerator = *mut ::core::ffi::c_void;
pub type IOpcRelationshipSelectorSet = *mut ::core::ffi::c_void;
pub type IOpcRelationshipSet = *mut ::core::ffi::c_void;
pub type IOpcSignatureCustomObject = *mut ::core::ffi::c_void;
pub type IOpcSignatureCustomObjectEnumerator = *mut ::core::ffi::c_void;
pub type IOpcSignatureCustomObjectSet = *mut ::core::ffi::c_void;
pub type IOpcSignaturePartReference = *mut ::core::ffi::c_void;
pub type IOpcSignaturePartReferenceEnumerator = *mut ::core::ffi::c_void;
pub type IOpcSignaturePartReferenceSet = *mut ::core::ffi::c_void;
pub type IOpcSignatureReference = *mut ::core::ffi::c_void;
pub type IOpcSignatureReferenceEnumerator = *mut ::core::ffi::c_void;
pub type IOpcSignatureReferenceSet = *mut ::core::ffi::c_void;
pub type IOpcSignatureRelationshipReference = *mut ::core::ffi::c_void;
pub type IOpcSignatureRelationshipReferenceEnumerator = *mut ::core::ffi::c_void;
pub type IOpcSignatureRelationshipReferenceSet = *mut ::core::ffi::c_void;
pub type IOpcSigningOptions = *mut ::core::ffi::c_void;
pub type IOpcUri = *mut ::core::ffi::c_void;
pub type OPC_CANONICALIZATION_METHOD = i32;
pub const OPC_CANONICALIZATION_NONE: OPC_CANONICALIZATION_METHOD = 0i32;
pub const OPC_CANONICALIZATION_C14N: OPC_CANONICALIZATION_METHOD = 1i32;
pub const OPC_CANONICALIZATION_C14N_WITH_COMMENTS: OPC_CANONICALIZATION_METHOD = 2i32;
pub type OPC_CERTIFICATE_EMBEDDING_OPTION = i32;
pub const OPC_CERTIFICATE_IN_CERTIFICATE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = 0i32;
pub const OPC_CERTIFICATE_IN_SIGNATURE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = 1i32;
pub const OPC_CERTIFICATE_NOT_EMBEDDED: OPC_CERTIFICATE_EMBEDDING_OPTION = 2i32;
pub type OPC_COMPRESSION_OPTIONS = i32;
pub const OPC_COMPRESSION_NONE: OPC_COMPRESSION_OPTIONS = -1i32;
pub const OPC_COMPRESSION_NORMAL: OPC_COMPRESSION_OPTIONS = 0i32;
pub const OPC_COMPRESSION_MAXIMUM: OPC_COMPRESSION_OPTIONS = 1i32;
pub const OPC_COMPRESSION_FAST: OPC_COMPRESSION_OPTIONS = 2i32;
pub const OPC_COMPRESSION_SUPERFAST: OPC_COMPRESSION_OPTIONS = 3i32;
pub const OPC_E_CONFLICTING_SETTINGS: ::windows_sys::core::HRESULT = -2142175212i32;
pub const OPC_E_COULD_NOT_RECOVER: ::windows_sys::core::HRESULT = -2142175154i32;
pub const OPC_E_DS_DEFAULT_DIGEST_METHOD_NOT_SET: ::windows_sys::core::HRESULT = -2142175161i32;
pub const OPC_E_DS_DIGEST_VALUE_ERROR: ::windows_sys::core::HRESULT = -2142175206i32;
pub const OPC_E_DS_DUPLICATE_PACKAGE_OBJECT_REFERENCES: ::windows_sys::core::HRESULT = -2142175187i32;
pub const OPC_E_DS_DUPLICATE_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175205i32;
pub const OPC_E_DS_DUPLICATE_SIGNATURE_PROPERTY_ELEMENT: ::windows_sys::core::HRESULT = -2142175192i32;
pub const OPC_E_DS_EXTERNAL_SIGNATURE: ::windows_sys::core::HRESULT = -2142175202i32;
pub const OPC_E_DS_EXTERNAL_SIGNATURE_REFERENCE: ::windows_sys::core::HRESULT = -2142175185i32;
pub const OPC_E_DS_INVALID_CANONICALIZATION_METHOD: ::windows_sys::core::HRESULT = -2142175198i32;
pub const OPC_E_DS_INVALID_CERTIFICATE_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175203i32;
pub const OPC_E_DS_INVALID_OPC_SIGNATURE_TIME_FORMAT: ::windows_sys::core::HRESULT = -2142175196i32;
pub const OPC_E_DS_INVALID_RELATIONSHIPS_SIGNING_OPTION: ::windows_sys::core::HRESULT = -2142175197i32;
pub const OPC_E_DS_INVALID_RELATIONSHIP_TRANSFORM_XML: ::windows_sys::core::HRESULT = -2142175199i32;
pub const OPC_E_DS_INVALID_SIGNATURE_COUNT: ::windows_sys::core::HRESULT = -2142175189i32;
pub const OPC_E_DS_INVALID_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175204i32;
pub const OPC_E_DS_INVALID_SIGNATURE_XML: ::windows_sys::core::HRESULT = -2142175190i32;
pub const OPC_E_DS_MISSING_CANONICALIZATION_TRANSFORM: ::windows_sys::core::HRESULT = -2142175182i32;
pub const OPC_E_DS_MISSING_CERTIFICATE_PART: ::windows_sys::core::HRESULT = -2142175146i32;
pub const OPC_E_DS_MISSING_PACKAGE_OBJECT_REFERENCE: ::windows_sys::core::HRESULT = -2142175186i32;
pub const OPC_E_DS_MISSING_SIGNATURE_ALGORITHM: ::windows_sys::core::HRESULT = -2142175188i32;
pub const OPC_E_DS_MISSING_SIGNATURE_ORIGIN_PART: ::windows_sys::core::HRESULT = -2142175201i32;
pub const OPC_E_DS_MISSING_SIGNATURE_PART: ::windows_sys::core::HRESULT = -2142175200i32;
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTIES_ELEMENT: ::windows_sys::core::HRESULT = -2142175194i32;
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTY_ELEMENT: ::windows_sys::core::HRESULT = -2142175193i32;
pub const OPC_E_DS_MISSING_SIGNATURE_TIME_PROPERTY: ::windows_sys::core::HRESULT = -2142175191i32;
pub const OPC_E_DS_MULTIPLE_RELATIONSHIP_TRANSFORMS: ::windows_sys::core::HRESULT = -2142175183i32;
pub const OPC_E_DS_PACKAGE_REFERENCE_URI_RESERVED: ::windows_sys::core::HRESULT = -2142175195i32;
pub const OPC_E_DS_REFERENCE_MISSING_CONTENT_TYPE: ::windows_sys::core::HRESULT = -2142175184i32;
pub const OPC_E_DS_SIGNATURE_CORRUPT: ::windows_sys::core::HRESULT = -2142175207i32;
pub const OPC_E_DS_SIGNATURE_METHOD_NOT_SET: ::windows_sys::core::HRESULT = -2142175162i32;
pub const OPC_E_DS_SIGNATURE_ORIGIN_EXISTS: ::windows_sys::core::HRESULT = -2142175148i32;
pub const OPC_E_DS_SIGNATURE_PROPERTY_MISSING_TARGET: ::windows_sys::core::HRESULT = -2142175163i32;
pub const OPC_E_DS_SIGNATURE_REFERENCE_MISSING_URI: ::windows_sys::core::HRESULT = -2142175165i32;
pub const OPC_E_DS_UNSIGNED_PACKAGE: ::windows_sys::core::HRESULT = -2142175147i32;
pub const OPC_E_DUPLICATE_DEFAULT_EXTENSION: ::windows_sys::core::HRESULT = -2142175217i32;
pub const OPC_E_DUPLICATE_OVERRIDE_PART: ::windows_sys::core::HRESULT = -2142175219i32;
pub const OPC_E_DUPLICATE_PART: ::windows_sys::core::HRESULT = -2142175221i32;
pub const OPC_E_DUPLICATE_PIECE: ::windows_sys::core::HRESULT = -2142175211i32;
pub const OPC_E_DUPLICATE_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175213i32;
pub const OPC_E_ENUM_CANNOT_MOVE_NEXT: ::windows_sys::core::HRESULT = -2142175151i32;
pub const OPC_E_ENUM_CANNOT_MOVE_PREVIOUS: ::windows_sys::core::HRESULT = -2142175150i32;
pub const OPC_E_ENUM_COLLECTION_CHANGED: ::windows_sys::core::HRESULT = -2142175152i32;
pub const OPC_E_ENUM_INVALID_POSITION: ::windows_sys::core::HRESULT = -2142175149i32;
pub const OPC_E_INVALID_CONTENT_TYPE: ::windows_sys::core::HRESULT = -2142175164i32;
pub const OPC_E_INVALID_CONTENT_TYPE_XML: ::windows_sys::core::HRESULT = -2142175226i32;
pub const OPC_E_INVALID_DEFAULT_EXTENSION: ::windows_sys::core::HRESULT = -2142175218i32;
pub const OPC_E_INVALID_OVERRIDE_PART_NAME: ::windows_sys::core::HRESULT = -2142175220i32;
pub const OPC_E_INVALID_PIECE: ::windows_sys::core::HRESULT = -2142175210i32;
pub const OPC_E_INVALID_RELATIONSHIP_ID: ::windows_sys::core::HRESULT = -2142175216i32;
pub const OPC_E_INVALID_RELATIONSHIP_TARGET: ::windows_sys::core::HRESULT = -2142175214i32;
pub const OPC_E_INVALID_RELATIONSHIP_TARGET_MODE: ::windows_sys::core::HRESULT = -2142175155i32;
pub const OPC_E_INVALID_RELATIONSHIP_TYPE: ::windows_sys::core::HRESULT = -2142175215i32;
pub const OPC_E_INVALID_RELS_XML: ::windows_sys::core::HRESULT = -2142175222i32;
pub const OPC_E_INVALID_XML_ENCODING: ::windows_sys::core::HRESULT = -2142175166i32;
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ATTRIBUTES: ::windows_sys::core::HRESULT = -2142175157i32;
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ELEMENTS: ::windows_sys::core::HRESULT = -2142175156i32;
pub const OPC_E_MC_INCONSISTENT_PROCESS_CONTENT: ::windows_sys::core::HRESULT = -2142175158i32;
pub const OPC_E_MC_INVALID_ATTRIBUTES_ON_IGNORABLE_ELEMENT: ::windows_sys::core::HRESULT = -2142175168i32;
pub const OPC_E_MC_INVALID_ENUM_TYPE: ::windows_sys::core::HRESULT = -2142175172i32;
pub const OPC_E_MC_INVALID_PREFIX_LIST: ::windows_sys::core::HRESULT = -2142175177i32;
pub const OPC_E_MC_INVALID_QNAME_LIST: ::windows_sys::core::HRESULT = -2142175176i32;
pub const OPC_E_MC_INVALID_XMLNS_ATTRIBUTE: ::windows_sys::core::HRESULT = -2142175167i32;
pub const OPC_E_MC_MISSING_CHOICE: ::windows_sys::core::HRESULT = -2142175173i32;
pub const OPC_E_MC_MISSING_REQUIRES_ATTR: ::windows_sys::core::HRESULT = -2142175179i32;
pub const OPC_E_MC_MULTIPLE_FALLBACK_ELEMENTS: ::windows_sys::core::HRESULT = -2142175159i32;
pub const OPC_E_MC_NESTED_ALTERNATE_CONTENT: ::windows_sys::core::HRESULT = -2142175175i32;
pub const OPC_E_MC_UNEXPECTED_ATTR: ::windows_sys::core::HRESULT = -2142175178i32;
pub const OPC_E_MC_UNEXPECTED_CHOICE: ::windows_sys::core::HRESULT = -2142175174i32;
pub const OPC_E_MC_UNEXPECTED_ELEMENT: ::windows_sys::core::HRESULT = -2142175181i32;
pub const OPC_E_MC_UNEXPECTED_REQUIRES_ATTR: ::windows_sys::core::HRESULT = -2142175180i32;
pub const OPC_E_MC_UNKNOWN_NAMESPACE: ::windows_sys::core::HRESULT = -2142175170i32;
pub const OPC_E_MC_UNKNOWN_PREFIX: ::windows_sys::core::HRESULT = -2142175169i32;
pub const OPC_E_MISSING_CONTENT_TYPES: ::windows_sys::core::HRESULT = -2142175225i32;
pub const OPC_E_MISSING_PIECE: ::windows_sys::core::HRESULT = -2142175209i32;
pub const OPC_E_NONCONFORMING_CONTENT_TYPES_XML: ::windows_sys::core::HRESULT = -2142175224i32;
pub const OPC_E_NONCONFORMING_RELS_XML: ::windows_sys::core::HRESULT = -2142175223i32;
pub const OPC_E_NONCONFORMING_URI: ::windows_sys::core::HRESULT = -2142175231i32;
pub const OPC_E_NO_SUCH_PART: ::windows_sys::core::HRESULT = -2142175208i32;
pub const OPC_E_NO_SUCH_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175160i32;
pub const OPC_E_NO_SUCH_SETTINGS: ::windows_sys::core::HRESULT = -2142175145i32;
pub const OPC_E_PART_CANNOT_BE_DIRECTORY: ::windows_sys::core::HRESULT = -2142175228i32;
pub const OPC_E_RELATIONSHIP_URI_REQUIRED: ::windows_sys::core::HRESULT = -2142175229i32;
pub const OPC_E_RELATIVE_URI_REQUIRED: ::windows_sys::core::HRESULT = -2142175230i32;
pub const OPC_E_UNEXPECTED_CONTENT_TYPE: ::windows_sys::core::HRESULT = -2142175227i32;
pub const OPC_E_UNSUPPORTED_PACKAGE: ::windows_sys::core::HRESULT = -2142175153i32;
pub const OPC_E_ZIP_CENTRAL_DIRECTORY_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171127i32;
pub const OPC_E_ZIP_COMMENT_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171124i32;
pub const OPC_E_ZIP_COMPRESSION_FAILED: ::windows_sys::core::HRESULT = -2142171133i32;
pub const OPC_E_ZIP_CORRUPTED_ARCHIVE: ::windows_sys::core::HRESULT = -2142171134i32;
pub const OPC_E_ZIP_DECOMPRESSION_FAILED: ::windows_sys::core::HRESULT = -2142171132i32;
pub const OPC_E_ZIP_DUPLICATE_NAME: ::windows_sys::core::HRESULT = -2142171125i32;
pub const OPC_E_ZIP_EXTRA_FIELDS_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171123i32;
pub const OPC_E_ZIP_FILE_HEADER_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171122i32;
pub const OPC_E_ZIP_INCONSISTENT_DIRECTORY: ::windows_sys::core::HRESULT = -2142171130i32;
pub const OPC_E_ZIP_INCONSISTENT_FILEITEM: ::windows_sys::core::HRESULT = -2142171131i32;
pub const OPC_E_ZIP_INCORRECT_DATA_SIZE: ::windows_sys::core::HRESULT = -2142171135i32;
pub const OPC_E_ZIP_MISSING_DATA_DESCRIPTOR: ::windows_sys::core::HRESULT = -2142171129i32;
pub const OPC_E_ZIP_MISSING_END_OF_CENTRAL_DIRECTORY: ::windows_sys::core::HRESULT = -2142171121i32;
pub const OPC_E_ZIP_NAME_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171126i32;
pub const OPC_E_ZIP_REQUIRES_64_BIT: ::windows_sys::core::HRESULT = -2142171120i32;
pub const OPC_E_ZIP_UNSUPPORTEDARCHIVE: ::windows_sys::core::HRESULT = -2142171128i32;
pub type OPC_READ_FLAGS = u32;
pub const OPC_READ_DEFAULT: OPC_READ_FLAGS = 0u32;
pub const OPC_VALIDATE_ON_LOAD: OPC_READ_FLAGS = 1u32;
pub const OPC_CACHE_ON_ACCESS: OPC_READ_FLAGS = 2u32;
pub type OPC_RELATIONSHIPS_SIGNING_OPTION = i32;
pub const OPC_RELATIONSHIP_SIGN_USING_SELECTORS: OPC_RELATIONSHIPS_SIGNING_OPTION = 0i32;
pub const OPC_RELATIONSHIP_SIGN_PART: OPC_RELATIONSHIPS_SIGNING_OPTION = 1i32;
pub type OPC_RELATIONSHIP_SELECTOR = i32;
pub const OPC_RELATIONSHIP_SELECT_BY_ID: OPC_RELATIONSHIP_SELECTOR = 0i32;
pub const OPC_RELATIONSHIP_SELECT_BY_TYPE: OPC_RELATIONSHIP_SELECTOR = 1i32;
pub type OPC_SIGNATURE_TIME_FORMAT = i32;
pub const OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS: OPC_SIGNATURE_TIME_FORMAT = 0i32;
pub const OPC_SIGNATURE_TIME_FORMAT_SECONDS: OPC_SIGNATURE_TIME_FORMAT = 1i32;
pub const OPC_SIGNATURE_TIME_FORMAT_MINUTES: OPC_SIGNATURE_TIME_FORMAT = 2i32;
pub const OPC_SIGNATURE_TIME_FORMAT_DAYS: OPC_SIGNATURE_TIME_FORMAT = 3i32;
pub const OPC_SIGNATURE_TIME_FORMAT_MONTHS: OPC_SIGNATURE_TIME_FORMAT = 4i32;
pub const OPC_SIGNATURE_TIME_FORMAT_YEARS: OPC_SIGNATURE_TIME_FORMAT = 5i32;
pub type OPC_SIGNATURE_VALIDATION_RESULT = i32;
pub const OPC_SIGNATURE_VALID: OPC_SIGNATURE_VALIDATION_RESULT = 0i32;
pub const OPC_SIGNATURE_INVALID: OPC_SIGNATURE_VALIDATION_RESULT = -1i32;
pub type OPC_STREAM_IO_MODE = i32;
pub const OPC_STREAM_IO_READ: OPC_STREAM_IO_MODE = 1i32;
pub const OPC_STREAM_IO_WRITE: OPC_STREAM_IO_MODE = 2i32;
pub type OPC_URI_TARGET_MODE = i32;
pub const OPC_URI_TARGET_MODE_INTERNAL: OPC_URI_TARGET_MODE = 0i32;
pub const OPC_URI_TARGET_MODE_EXTERNAL: OPC_URI_TARGET_MODE = 1i32;
pub type OPC_WRITE_FLAGS = u32;
pub const OPC_WRITE_DEFAULT: OPC_WRITE_FLAGS = 0u32;
pub const OPC_WRITE_FORCE_ZIP32: OPC_WRITE_FLAGS = 1u32;
pub const OpcFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1798138784, data2: 40766, data3: 20263, data4: [146, 11, 49, 60, 196, 38, 163, 158] };
