// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountResponse {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePortRequest {
    #[prost(string, tag="1")]
    pub project: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePortResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub kind: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectResponse {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub namespace: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `demeter.ops.v1alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xaa, 0x06, 0x0a, 0x27, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x64, 0x65, 0x6d, 0x65, 0x74,
    0x65, 0x72, 0x2f, 0x6f, 0x70, 0x73, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2f, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x64, 0x65,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x6f, 0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x1a, 0x19, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2a, 0x0a, 0x14,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x2b, 0x0a, 0x15, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x32, 0x78, 0x0a, 0x0e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x66, 0x0a, 0x0d, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x29, 0x2e, 0x64, 0x65, 0x6d, 0x65, 0x74,
    0x65, 0x72, 0x2e, 0x6f, 0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x43,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x1a, 0x2a, 0x2e, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x6f, 0x70,
    0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42,
    0xdc, 0x01, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e,
    0x6f, 0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x42, 0x0c, 0x41, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x45, 0x67, 0x69, 0x74,
    0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x62, 0x75, 0x66, 0x62, 0x75, 0x69, 0x6c, 0x64,
    0x2f, 0x62, 0x75, 0x66, 0x2d, 0x74, 0x6f, 0x75, 0x72, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2f, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2f, 0x6f, 0x70, 0x73, 0x2f,
    0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x3b, 0x6f, 0x70, 0x73, 0x76, 0x31, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0xa2, 0x02, 0x03, 0x44, 0x4f, 0x58, 0xaa, 0x02, 0x13, 0x44, 0x65, 0x6d, 0x65, 0x74,
    0x65, 0x72, 0x2e, 0x4f, 0x70, 0x73, 0x2e, 0x56, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0xca, 0x02,
    0x13, 0x44, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x5c, 0x4f, 0x70, 0x73, 0x5c, 0x56, 0x31, 0x61,
    0x6c, 0x70, 0x68, 0x61, 0xe2, 0x02, 0x1f, 0x44, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x5c, 0x4f,
    0x70, 0x73, 0x5c, 0x56, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x15, 0x44, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72,
    0x3a, 0x3a, 0x4f, 0x70, 0x73, 0x3a, 0x3a, 0x56, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x4a, 0x94,
    0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x10, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x06, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x02, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x07, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a,
    0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0b, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0e, 0x00,
    0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x02, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x06, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x0f, 0x14, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0f, 0x33, 0x48, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xf5, 0x07,
    0x0a, 0x24, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2f,
    0x6f, 0x70, 0x73, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2f, 0x70, 0x6f, 0x72, 0x74,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e,
    0x6f, 0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a, 0x19, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x55, 0x0a, 0x11, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x50, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x70,
    0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x72,
    0x6f, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0x38, 0x0a,
    0x12, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x32, 0x6c, 0x0a, 0x0b, 0x50, 0x6f, 0x72, 0x74, 0x53,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x5d, 0x0a, 0x0a, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x50, 0x6f, 0x72, 0x74, 0x12, 0x26, 0x2e, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x6f,
    0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x50, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x27, 0x2e, 0x64,
    0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x6f, 0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0xd9, 0x01, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x64, 0x65,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x6f, 0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x42, 0x09, 0x50, 0x6f, 0x72, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x45,
    0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x62, 0x75, 0x66, 0x62, 0x75,
    0x69, 0x6c, 0x64, 0x2f, 0x62, 0x75, 0x66, 0x2d, 0x74, 0x6f, 0x75, 0x72, 0x2f, 0x67, 0x65, 0x6e,
    0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2f, 0x6f,
    0x70, 0x73, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x3b, 0x6f, 0x70, 0x73, 0x76, 0x31,
    0x61, 0x6c, 0x70, 0x68, 0x61, 0xa2, 0x02, 0x03, 0x44, 0x4f, 0x58, 0xaa, 0x02, 0x13, 0x44, 0x65,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x4f, 0x70, 0x73, 0x2e, 0x56, 0x31, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0xca, 0x02, 0x13, 0x44, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x5c, 0x4f, 0x70, 0x73, 0x5c,
    0x56, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0xe2, 0x02, 0x1f, 0x44, 0x65, 0x6d, 0x65, 0x74, 0x65,
    0x72, 0x5c, 0x4f, 0x70, 0x73, 0x5c, 0x56, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x5c, 0x47, 0x50,
    0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x15, 0x44, 0x65, 0x6d, 0x65,
    0x74, 0x65, 0x72, 0x3a, 0x3a, 0x4f, 0x70, 0x73, 0x3a, 0x3a, 0x56, 0x31, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x4a, 0xb9, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x13, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1c,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x06, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x06, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x02, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x08, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x08, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x08, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x10,
    0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x02, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x09, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x0c, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0d, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x0e, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x09, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x10, 0x11, 0x0a, 0x0a,
    0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x11, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x12, 0x02, 0x41, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x06,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x12, 0x11, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x2d, 0x3f, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xff, 0x06, 0x0a, 0x27, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f,
    0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2f, 0x6f, 0x70, 0x73, 0x2f, 0x76, 0x31, 0x61, 0x6c,
    0x70, 0x68, 0x61, 0x2f, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x13, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x6f, 0x70, 0x73, 0x2e, 0x76,
    0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a, 0x19, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x2a, 0x0a, 0x14, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x6a, 0x65,
    0x63, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x49, 0x0a,
    0x15, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6e, 0x61,
    0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6e,
    0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x32, 0x78, 0x0a, 0x0e, 0x50, 0x72, 0x6f, 0x6a,
    0x65, 0x63, 0x74, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x66, 0x0a, 0x0d, 0x43, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x29, 0x2e, 0x64, 0x65,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x6f, 0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2a, 0x2e, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72,
    0x2e, 0x6f, 0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x43, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x42, 0xdc, 0x01, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x64, 0x65, 0x6d, 0x65, 0x74,
    0x65, 0x72, 0x2e, 0x6f, 0x70, 0x73, 0x2e, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x42, 0x0c,
    0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x45,
    0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x62, 0x75, 0x66, 0x62, 0x75,
    0x69, 0x6c, 0x64, 0x2f, 0x62, 0x75, 0x66, 0x2d, 0x74, 0x6f, 0x75, 0x72, 0x2f, 0x67, 0x65, 0x6e,
    0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x64, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2f, 0x6f,
    0x70, 0x73, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x3b, 0x6f, 0x70, 0x73, 0x76, 0x31,
    0x61, 0x6c, 0x70, 0x68, 0x61, 0xa2, 0x02, 0x03, 0x44, 0x4f, 0x58, 0xaa, 0x02, 0x13, 0x44, 0x65,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x4f, 0x70, 0x73, 0x2e, 0x56, 0x31, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0xca, 0x02, 0x13, 0x44, 0x65, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x5c, 0x4f, 0x70, 0x73, 0x5c,
    0x56, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0xe2, 0x02, 0x1f, 0x44, 0x65, 0x6d, 0x65, 0x74, 0x65,
    0x72, 0x5c, 0x4f, 0x70, 0x73, 0x5c, 0x56, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x5c, 0x47, 0x50,
    0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x15, 0x44, 0x65, 0x6d, 0x65,
    0x74, 0x65, 0x72, 0x3a, 0x3a, 0x4f, 0x70, 0x73, 0x3a, 0x3a, 0x56, 0x31, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x4a, 0xcb, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x11, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1c,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x06, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x06, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x02, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x0a, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x01, 0x12, 0x03, 0x0c, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c,
    0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x15, 0x16,
    0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0f, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x06, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x10, 0x02, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x10, 0x06, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x10, 0x14,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x33, 0x48, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("demeter.ops.v1alpha.serde.rs");
include!("demeter.ops.v1alpha.tonic.rs");
// @@protoc_insertion_point(module)