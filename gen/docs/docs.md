# Protocol Documentation
<a name="top"></a>

## Table of Contents

<<<<<<< HEAD
- [proto/demeter/ops/v1alpha/port.proto](#proto_demeter_ops_v1alpha_port-proto)
    - [CreatePortRequest](#demeter-ops-v1alpha-CreatePortRequest)
    - [CreatePortResponse](#demeter-ops-v1alpha-CreatePortResponse)
  
    - [PortService](#demeter-ops-v1alpha-PortService)
=======
- [proto/demeter/ops/v1alpha/account.proto](#proto_demeter_ops_v1alpha_account-proto)
    - [CreateAccountRequest](#demeter-ops-v1alpha-CreateAccountRequest)
    - [CreateAccountResponse](#demeter-ops-v1alpha-CreateAccountResponse)
  
    - [AccountService](#demeter-ops-v1alpha-AccountService)
>>>>>>> cb08039162dc697886c1c8d7972ff9d1e52363ab
  
- [proto/demeter/ops/v1alpha/project.proto](#proto_demeter_ops_v1alpha_project-proto)
    - [CreateProjectRequest](#demeter-ops-v1alpha-CreateProjectRequest)
    - [CreateProjectResponse](#demeter-ops-v1alpha-CreateProjectResponse)
  
    - [ProjectService](#demeter-ops-v1alpha-ProjectService)
  
- [Scalar Value Types](#scalar-value-types)



<<<<<<< HEAD
<a name="proto_demeter_ops_v1alpha_port-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## proto/demeter/ops/v1alpha/port.proto



<a name="demeter-ops-v1alpha-CreatePortRequest"></a>

### CreatePortRequest
=======
<a name="proto_demeter_ops_v1alpha_account-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## proto/demeter/ops/v1alpha/account.proto



<a name="demeter-ops-v1alpha-CreateAccountRequest"></a>

### CreateAccountRequest
>>>>>>> cb08039162dc697886c1c8d7972ff9d1e52363ab



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
<<<<<<< HEAD
| project | [string](#string) |  |  |
| kind | [string](#string) |  |  |
| resource | [google.protobuf.Any](#google-protobuf-Any) |  |  |
=======
| name | [string](#string) |  |  |
>>>>>>> cb08039162dc697886c1c8d7972ff9d1e52363ab






<<<<<<< HEAD
<a name="demeter-ops-v1alpha-CreatePortResponse"></a>

### CreatePortResponse
=======
<a name="demeter-ops-v1alpha-CreateAccountResponse"></a>

### CreateAccountResponse
>>>>>>> cb08039162dc697886c1c8d7972ff9d1e52363ab



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
<<<<<<< HEAD
| kind | [string](#string) |  |  |
=======
| name | [string](#string) |  |  |
>>>>>>> cb08039162dc697886c1c8d7972ff9d1e52363ab





 

 

 


<<<<<<< HEAD
<a name="demeter-ops-v1alpha-PortService"></a>

### PortService
=======
<a name="demeter-ops-v1alpha-AccountService"></a>

### AccountService
>>>>>>> cb08039162dc697886c1c8d7972ff9d1e52363ab


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
<<<<<<< HEAD
| CreatePort | [CreatePortRequest](#demeter-ops-v1alpha-CreatePortRequest) | [CreatePortResponse](#demeter-ops-v1alpha-CreatePortResponse) |  |
=======
| CreateAccount | [CreateAccountRequest](#demeter-ops-v1alpha-CreateAccountRequest) | [CreateAccountResponse](#demeter-ops-v1alpha-CreateAccountResponse) |  |
>>>>>>> cb08039162dc697886c1c8d7972ff9d1e52363ab

 



<a name="proto_demeter_ops_v1alpha_project-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## proto/demeter/ops/v1alpha/project.proto



<a name="demeter-ops-v1alpha-CreateProjectRequest"></a>

### CreateProjectRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-CreateProjectResponse"></a>

### CreateProjectResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| namespace | [string](#string) |  |  |





 

 

 


<a name="demeter-ops-v1alpha-ProjectService"></a>

### ProjectService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateProject | [CreateProjectRequest](#demeter-ops-v1alpha-CreateProjectRequest) | [CreateProjectResponse](#demeter-ops-v1alpha-CreateProjectResponse) |  |

 



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

