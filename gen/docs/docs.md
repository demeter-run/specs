# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [demeter/ops/v1alpha/metadata.proto](#demeter_ops_v1alpha_metadata-proto)
    - [FetchMetadataRequest](#demeter-ops-v1alpha-FetchMetadataRequest)
    - [FetchMetadataResponse](#demeter-ops-v1alpha-FetchMetadataResponse)
  
    - [MetadataService](#demeter-ops-v1alpha-MetadataService)
  
- [demeter/ops/v1alpha/project.proto](#demeter_ops_v1alpha_project-proto)
    - [CreateProjectRequest](#demeter-ops-v1alpha-CreateProjectRequest)
    - [CreateProjectResponse](#demeter-ops-v1alpha-CreateProjectResponse)
    - [CreateProjectSecretRequest](#demeter-ops-v1alpha-CreateProjectSecretRequest)
    - [CreateProjectSecretResponse](#demeter-ops-v1alpha-CreateProjectSecretResponse)
    - [FetchProjectsRequest](#demeter-ops-v1alpha-FetchProjectsRequest)
    - [FetchProjectsResponse](#demeter-ops-v1alpha-FetchProjectsResponse)
    - [Project](#demeter-ops-v1alpha-Project)
    - [UpdateProjectRequest](#demeter-ops-v1alpha-UpdateProjectRequest)
    - [UpdateProjectResponse](#demeter-ops-v1alpha-UpdateProjectResponse)
  
    - [ProjectService](#demeter-ops-v1alpha-ProjectService)
  
- [demeter/ops/v1alpha/resource.proto](#demeter_ops_v1alpha_resource-proto)
    - [CreateResourceRequest](#demeter-ops-v1alpha-CreateResourceRequest)
    - [CreateResourceResponse](#demeter-ops-v1alpha-CreateResourceResponse)
    - [DeleteResourceRequest](#demeter-ops-v1alpha-DeleteResourceRequest)
    - [DeleteResourceResponse](#demeter-ops-v1alpha-DeleteResourceResponse)
    - [FetchResourcesByIdRequest](#demeter-ops-v1alpha-FetchResourcesByIdRequest)
    - [FetchResourcesByIdResponse](#demeter-ops-v1alpha-FetchResourcesByIdResponse)
    - [FetchResourcesRequest](#demeter-ops-v1alpha-FetchResourcesRequest)
    - [FetchResourcesResponse](#demeter-ops-v1alpha-FetchResourcesResponse)
    - [Resource](#demeter-ops-v1alpha-Resource)
  
    - [ResourceService](#demeter-ops-v1alpha-ResourceService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="demeter_ops_v1alpha_metadata-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## demeter/ops/v1alpha/metadata.proto



<a name="demeter-ops-v1alpha-FetchMetadataRequest"></a>

### FetchMetadataRequest







<a name="demeter-ops-v1alpha-FetchMetadataResponse"></a>

### FetchMetadataResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [string](#string) | repeated |  |





 

 

 


<a name="demeter-ops-v1alpha-MetadataService"></a>

### MetadataService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| FetchMetadata | [FetchMetadataRequest](#demeter-ops-v1alpha-FetchMetadataRequest) | [FetchMetadataResponse](#demeter-ops-v1alpha-FetchMetadataResponse) |  |

 



<a name="demeter_ops_v1alpha_project-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## demeter/ops/v1alpha/project.proto



<a name="demeter-ops-v1alpha-CreateProjectRequest"></a>

### CreateProjectRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-CreateProjectResponse"></a>

### CreateProjectResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| namespace | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-CreateProjectSecretRequest"></a>

### CreateProjectSecretRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| project_id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-CreateProjectSecretResponse"></a>

### CreateProjectSecretResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| key | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-FetchProjectsRequest"></a>

### FetchProjectsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page | [uint32](#uint32) | optional |  |
| page_size | [uint32](#uint32) | optional |  |






<a name="demeter-ops-v1alpha-FetchProjectsResponse"></a>

### FetchProjectsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [Project](#demeter-ops-v1alpha-Project) | repeated |  |






<a name="demeter-ops-v1alpha-Project"></a>

### Project



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| namespace | [string](#string) |  |  |
| status | [string](#string) |  |  |
| created_at | [string](#string) |  |  |
| updated_at | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-UpdateProjectRequest"></a>

### UpdateProjectRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) | optional |  |
| status | [string](#string) | optional |  |






<a name="demeter-ops-v1alpha-UpdateProjectResponse"></a>

### UpdateProjectResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| updated | [Project](#demeter-ops-v1alpha-Project) |  |  |





 

 

 


<a name="demeter-ops-v1alpha-ProjectService"></a>

### ProjectService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| FetchProjects | [FetchProjectsRequest](#demeter-ops-v1alpha-FetchProjectsRequest) | [FetchProjectsResponse](#demeter-ops-v1alpha-FetchProjectsResponse) |  |
| CreateProject | [CreateProjectRequest](#demeter-ops-v1alpha-CreateProjectRequest) | [CreateProjectResponse](#demeter-ops-v1alpha-CreateProjectResponse) |  |
| UpdateProject | [UpdateProjectRequest](#demeter-ops-v1alpha-UpdateProjectRequest) | [UpdateProjectResponse](#demeter-ops-v1alpha-UpdateProjectResponse) |  |
| CreateProjectSecret | [CreateProjectSecretRequest](#demeter-ops-v1alpha-CreateProjectSecretRequest) | [CreateProjectSecretResponse](#demeter-ops-v1alpha-CreateProjectSecretResponse) |  |

 



<a name="demeter_ops_v1alpha_resource-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## demeter/ops/v1alpha/resource.proto



<a name="demeter-ops-v1alpha-CreateResourceRequest"></a>

### CreateResourceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| kind | [string](#string) |  |  |
| spec | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-CreateResourceResponse"></a>

### CreateResourceResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| kind | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-DeleteResourceRequest"></a>

### DeleteResourceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| resource_id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-DeleteResourceResponse"></a>

### DeleteResourceResponse







<a name="demeter-ops-v1alpha-FetchResourcesByIdRequest"></a>

### FetchResourcesByIdRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| resource_id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-FetchResourcesByIdResponse"></a>

### FetchResourcesByIdResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [Resource](#demeter-ops-v1alpha-Resource) | repeated |  |






<a name="demeter-ops-v1alpha-FetchResourcesRequest"></a>

### FetchResourcesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| page | [uint32](#uint32) | optional |  |
| page_size | [uint32](#uint32) | optional |  |






<a name="demeter-ops-v1alpha-FetchResourcesResponse"></a>

### FetchResourcesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [Resource](#demeter-ops-v1alpha-Resource) | repeated |  |






<a name="demeter-ops-v1alpha-Resource"></a>

### Resource



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| kind | [string](#string) |  |  |
| spec | [string](#string) |  |  |
| status | [string](#string) |  |  |
| created_at | [string](#string) |  |  |
| updated_at | [string](#string) |  |  |





 

 

 


<a name="demeter-ops-v1alpha-ResourceService"></a>

### ResourceService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| FetchResources | [FetchResourcesRequest](#demeter-ops-v1alpha-FetchResourcesRequest) | [FetchResourcesResponse](#demeter-ops-v1alpha-FetchResourcesResponse) |  |
| FetchResourcesById | [FetchResourcesByIdRequest](#demeter-ops-v1alpha-FetchResourcesByIdRequest) | [FetchResourcesByIdResponse](#demeter-ops-v1alpha-FetchResourcesByIdResponse) |  |
| CreateResource | [CreateResourceRequest](#demeter-ops-v1alpha-CreateResourceRequest) | [CreateResourceResponse](#demeter-ops-v1alpha-CreateResourceResponse) |  |
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

