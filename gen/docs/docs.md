# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [demeter/ops/v1alpha/ops.proto](#demeter_ops_v1alpha_ops-proto)
    - [CreateResourceRequest](#demeter-ops-v1alpha-CreateResourceRequest)
    - [CreateResourceResponse](#demeter-ops-v1alpha-CreateResourceResponse)
    - [DeleteResourceRequest](#demeter-ops-v1alpha-DeleteResourceRequest)
    - [DeleteResourceResponse](#demeter-ops-v1alpha-DeleteResourceResponse)
    - [ListResourcesRequest](#demeter-ops-v1alpha-ListResourcesRequest)
    - [ListResourcesResponse](#demeter-ops-v1alpha-ListResourcesResponse)
    - [PatchResourceRequest](#demeter-ops-v1alpha-PatchResourceRequest)
    - [PatchResourceResponse](#demeter-ops-v1alpha-PatchResourceResponse)
    - [ReadResourceRequest](#demeter-ops-v1alpha-ReadResourceRequest)
    - [ReadResourceResponse](#demeter-ops-v1alpha-ReadResourceResponse)
    - [Resource](#demeter-ops-v1alpha-Resource)
    - [ResourceMetadata](#demeter-ops-v1alpha-ResourceMetadata)
  
    - [OpsService](#demeter-ops-v1alpha-OpsService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="demeter_ops_v1alpha_ops-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## demeter/ops/v1alpha/ops.proto



<a name="demeter-ops-v1alpha-CreateResourceRequest"></a>

### CreateResourceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| metadata | [ResourceMetadata](#demeter-ops-v1alpha-ResourceMetadata) |  |  |
| spec | [google.protobuf.Any](#google-protobuf-Any) |  |  |






<a name="demeter-ops-v1alpha-CreateResourceResponse"></a>

### CreateResourceResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| event_receipt | [bytes](#bytes) |  |  |
| resource_uuid | [bytes](#bytes) |  |  |






<a name="demeter-ops-v1alpha-DeleteResourceRequest"></a>

### DeleteResourceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| metadata | [ResourceMetadata](#demeter-ops-v1alpha-ResourceMetadata) |  |  |






<a name="demeter-ops-v1alpha-DeleteResourceResponse"></a>

### DeleteResourceResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [google.protobuf.Any](#google-protobuf-Any) |  |  |






<a name="demeter-ops-v1alpha-ListResourcesRequest"></a>

### ListResourcesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| namespace | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-ListResourcesResponse"></a>

### ListResourcesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| items | [Resource](#demeter-ops-v1alpha-Resource) | repeated |  |






<a name="demeter-ops-v1alpha-PatchResourceRequest"></a>

### PatchResourceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| metadata | [ResourceMetadata](#demeter-ops-v1alpha-ResourceMetadata) |  |  |
| spec | [google.protobuf.Any](#google-protobuf-Any) |  |  |






<a name="demeter-ops-v1alpha-PatchResourceResponse"></a>

### PatchResourceResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [google.protobuf.Any](#google-protobuf-Any) |  |  |






<a name="demeter-ops-v1alpha-ReadResourceRequest"></a>

### ReadResourceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| metadata | [ResourceMetadata](#demeter-ops-v1alpha-ResourceMetadata) |  |  |






<a name="demeter-ops-v1alpha-ReadResourceResponse"></a>

### ReadResourceResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| metadata | [ResourceMetadata](#demeter-ops-v1alpha-ResourceMetadata) |  |  |
| spec | [google.protobuf.Any](#google-protobuf-Any) |  |  |
| status | [google.protobuf.Any](#google-protobuf-Any) |  |  |






<a name="demeter-ops-v1alpha-Resource"></a>

### Resource



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| metadata | [ResourceMetadata](#demeter-ops-v1alpha-ResourceMetadata) |  |  |
| spec | [google.protobuf.Any](#google-protobuf-Any) |  |  |
| status | [google.protobuf.Any](#google-protobuf-Any) |  |  |






<a name="demeter-ops-v1alpha-ResourceMetadata"></a>

### ResourceMetadata



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| namespace | [string](#string) |  |  |
| name | [string](#string) |  |  |





 

 

 


<a name="demeter-ops-v1alpha-OpsService"></a>

### OpsService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateResource | [CreateResourceRequest](#demeter-ops-v1alpha-CreateResourceRequest) | [CreateResourceResponse](#demeter-ops-v1alpha-CreateResourceResponse) |  |
| ListResources | [ListResourcesRequest](#demeter-ops-v1alpha-ListResourcesRequest) | [ListResourcesResponse](#demeter-ops-v1alpha-ListResourcesResponse) |  |
| ReadResource | [ReadResourceRequest](#demeter-ops-v1alpha-ReadResourceRequest) | [ReadResourceResponse](#demeter-ops-v1alpha-ReadResourceResponse) |  |
| PatchResource | [PatchResourceRequest](#demeter-ops-v1alpha-PatchResourceRequest) | [PatchResourceResponse](#demeter-ops-v1alpha-PatchResourceResponse) |  |
| DeleteResource | [DeleteResourceRequest](#demeter-ops-v1alpha-DeleteResourceRequest) | [DeleteResourceResponse](#demeter-ops-v1alpha-DeleteResourceResponse) |  |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

