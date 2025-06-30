# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [proto/demeter/ops/v1alpha/metadata.proto](#proto_demeter_ops_v1alpha_metadata-proto)
    - [FetchMetadataRequest](#demeter-ops-v1alpha-FetchMetadataRequest)
    - [FetchMetadataResponse](#demeter-ops-v1alpha-FetchMetadataResponse)
    - [Metadata](#demeter-ops-v1alpha-Metadata)
  
    - [MetadataService](#demeter-ops-v1alpha-MetadataService)
  
- [proto/demeter/ops/v1alpha/project.proto](#proto_demeter_ops_v1alpha_project-proto)
    - [AcceptProjectUserInviteRequest](#demeter-ops-v1alpha-AcceptProjectUserInviteRequest)
    - [AcceptProjectUserInviteResponse](#demeter-ops-v1alpha-AcceptProjectUserInviteResponse)
    - [CreateProjectRequest](#demeter-ops-v1alpha-CreateProjectRequest)
    - [CreateProjectResponse](#demeter-ops-v1alpha-CreateProjectResponse)
    - [CreateProjectSecretRequest](#demeter-ops-v1alpha-CreateProjectSecretRequest)
    - [CreateProjectSecretResponse](#demeter-ops-v1alpha-CreateProjectSecretResponse)
    - [CreateProjectUserInviteRequest](#demeter-ops-v1alpha-CreateProjectUserInviteRequest)
    - [CreateProjectUserInviteResponse](#demeter-ops-v1alpha-CreateProjectUserInviteResponse)
    - [DeleteProjectRequest](#demeter-ops-v1alpha-DeleteProjectRequest)
    - [DeleteProjectResponse](#demeter-ops-v1alpha-DeleteProjectResponse)
    - [DeleteProjectSecretRequest](#demeter-ops-v1alpha-DeleteProjectSecretRequest)
    - [DeleteProjectSecretResponse](#demeter-ops-v1alpha-DeleteProjectSecretResponse)
    - [DeleteProjectUserInviteRequest](#demeter-ops-v1alpha-DeleteProjectUserInviteRequest)
    - [DeleteProjectUserInviteResponse](#demeter-ops-v1alpha-DeleteProjectUserInviteResponse)
    - [DeleteProjectUserRequest](#demeter-ops-v1alpha-DeleteProjectUserRequest)
    - [DeleteProjectUserResponse](#demeter-ops-v1alpha-DeleteProjectUserResponse)
    - [FetchMeProjectUserRequest](#demeter-ops-v1alpha-FetchMeProjectUserRequest)
    - [FetchMeProjectUserResponse](#demeter-ops-v1alpha-FetchMeProjectUserResponse)
    - [FetchProjectByIdRequest](#demeter-ops-v1alpha-FetchProjectByIdRequest)
    - [FetchProjectByIdResponse](#demeter-ops-v1alpha-FetchProjectByIdResponse)
    - [FetchProjectByNamespaceRequest](#demeter-ops-v1alpha-FetchProjectByNamespaceRequest)
    - [FetchProjectByNamespaceResponse](#demeter-ops-v1alpha-FetchProjectByNamespaceResponse)
    - [FetchProjectSecretsRequest](#demeter-ops-v1alpha-FetchProjectSecretsRequest)
    - [FetchProjectSecretsResponse](#demeter-ops-v1alpha-FetchProjectSecretsResponse)
    - [FetchProjectUserInvitesRequest](#demeter-ops-v1alpha-FetchProjectUserInvitesRequest)
    - [FetchProjectUserInvitesResponse](#demeter-ops-v1alpha-FetchProjectUserInvitesResponse)
    - [FetchProjectUsersRequest](#demeter-ops-v1alpha-FetchProjectUsersRequest)
    - [FetchProjectUsersResponse](#demeter-ops-v1alpha-FetchProjectUsersResponse)
    - [FetchProjectsRequest](#demeter-ops-v1alpha-FetchProjectsRequest)
    - [FetchProjectsResponse](#demeter-ops-v1alpha-FetchProjectsResponse)
    - [Project](#demeter-ops-v1alpha-Project)
    - [ProjectSecret](#demeter-ops-v1alpha-ProjectSecret)
    - [ProjectUser](#demeter-ops-v1alpha-ProjectUser)
    - [ProjectUserInvite](#demeter-ops-v1alpha-ProjectUserInvite)
    - [ResendProjectUserInviteRequest](#demeter-ops-v1alpha-ResendProjectUserInviteRequest)
    - [ResendProjectUserInviteResponse](#demeter-ops-v1alpha-ResendProjectUserInviteResponse)
    - [UpdateProjectRequest](#demeter-ops-v1alpha-UpdateProjectRequest)
    - [UpdateProjectResponse](#demeter-ops-v1alpha-UpdateProjectResponse)
  
    - [ProjectService](#demeter-ops-v1alpha-ProjectService)
  
- [proto/demeter/ops/v1alpha/resource.proto](#proto_demeter_ops_v1alpha_resource-proto)
    - [CreateResourceRequest](#demeter-ops-v1alpha-CreateResourceRequest)
    - [CreateResourceResponse](#demeter-ops-v1alpha-CreateResourceResponse)
    - [DeleteResourceRequest](#demeter-ops-v1alpha-DeleteResourceRequest)
    - [DeleteResourceResponse](#demeter-ops-v1alpha-DeleteResourceResponse)
    - [FetchResourcesByIdRequest](#demeter-ops-v1alpha-FetchResourcesByIdRequest)
    - [FetchResourcesByIdResponse](#demeter-ops-v1alpha-FetchResourcesByIdResponse)
    - [FetchResourcesRequest](#demeter-ops-v1alpha-FetchResourcesRequest)
    - [FetchResourcesResponse](#demeter-ops-v1alpha-FetchResourcesResponse)
    - [Resource](#demeter-ops-v1alpha-Resource)
    - [UpdateResourceRequest](#demeter-ops-v1alpha-UpdateResourceRequest)
    - [UpdateResourceResponse](#demeter-ops-v1alpha-UpdateResourceResponse)
  
    - [ResourceService](#demeter-ops-v1alpha-ResourceService)
  
- [proto/demeter/ops/v1alpha/usage.proto](#proto_demeter_ops_v1alpha_usage-proto)
    - [FetchUsageClusterRequest](#demeter-ops-v1alpha-FetchUsageClusterRequest)
    - [FetchUsageClusterResponse](#demeter-ops-v1alpha-FetchUsageClusterResponse)
    - [FetchUsageReportRequest](#demeter-ops-v1alpha-FetchUsageReportRequest)
    - [FetchUsageReportResponse](#demeter-ops-v1alpha-FetchUsageReportResponse)
    - [UsageReport](#demeter-ops-v1alpha-UsageReport)
  
    - [UsageService](#demeter-ops-v1alpha-UsageService)
  
- [proto/demeter/ops/v1alpha/worker.proto](#proto_demeter_ops_v1alpha_worker-proto)
    - [DeleteKeyValueRequest](#demeter-ops-v1alpha-DeleteKeyValueRequest)
    - [DeleteKeyValueResponse](#demeter-ops-v1alpha-DeleteKeyValueResponse)
    - [FetchKeyValueRequest](#demeter-ops-v1alpha-FetchKeyValueRequest)
    - [FetchKeyValueResponse](#demeter-ops-v1alpha-FetchKeyValueResponse)
    - [FetchLogsRequest](#demeter-ops-v1alpha-FetchLogsRequest)
    - [FetchLogsResponse](#demeter-ops-v1alpha-FetchLogsResponse)
    - [GetPublicKeySignerRequest](#demeter-ops-v1alpha-GetPublicKeySignerRequest)
    - [GetPublicKeySignerResponse](#demeter-ops-v1alpha-GetPublicKeySignerResponse)
    - [KeyValue](#demeter-ops-v1alpha-KeyValue)
    - [ListSignerRequest](#demeter-ops-v1alpha-ListSignerRequest)
    - [ListSignerResponse](#demeter-ops-v1alpha-ListSignerResponse)
    - [Log](#demeter-ops-v1alpha-Log)
    - [SignPayloadSignerRequest](#demeter-ops-v1alpha-SignPayloadSignerRequest)
    - [SignPayloadSignerResponse](#demeter-ops-v1alpha-SignPayloadSignerResponse)
    - [Signer](#demeter-ops-v1alpha-Signer)
    - [UpdateKeyValueRequest](#demeter-ops-v1alpha-UpdateKeyValueRequest)
    - [UpdateKeyValueResponse](#demeter-ops-v1alpha-UpdateKeyValueResponse)
  
    - [Direction](#demeter-ops-v1alpha-Direction)
  
    - [KeyValueService](#demeter-ops-v1alpha-KeyValueService)
    - [LogsService](#demeter-ops-v1alpha-LogsService)
    - [SignerService](#demeter-ops-v1alpha-SignerService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="proto_demeter_ops_v1alpha_metadata-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## proto/demeter/ops/v1alpha/metadata.proto



<a name="demeter-ops-v1alpha-FetchMetadataRequest"></a>

### FetchMetadataRequest







<a name="demeter-ops-v1alpha-FetchMetadataResponse"></a>

### FetchMetadataResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [Metadata](#demeter-ops-v1alpha-Metadata) | repeated |  |






<a name="demeter-ops-v1alpha-Metadata"></a>

### Metadata



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| options | [string](#string) |  |  |
| crd | [string](#string) |  |  |





 

 

 


<a name="demeter-ops-v1alpha-MetadataService"></a>

### MetadataService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| FetchMetadata | [FetchMetadataRequest](#demeter-ops-v1alpha-FetchMetadataRequest) | [FetchMetadataResponse](#demeter-ops-v1alpha-FetchMetadataResponse) |  |

 



<a name="proto_demeter_ops_v1alpha_project-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## proto/demeter/ops/v1alpha/project.proto



<a name="demeter-ops-v1alpha-AcceptProjectUserInviteRequest"></a>

### AcceptProjectUserInviteRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| code | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-AcceptProjectUserInviteResponse"></a>

### AcceptProjectUserInviteResponse







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






<a name="demeter-ops-v1alpha-CreateProjectUserInviteRequest"></a>

### CreateProjectUserInviteRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| email | [string](#string) |  |  |
| role | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-CreateProjectUserInviteResponse"></a>

### CreateProjectUserInviteResponse







<a name="demeter-ops-v1alpha-DeleteProjectRequest"></a>

### DeleteProjectRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-DeleteProjectResponse"></a>

### DeleteProjectResponse







<a name="demeter-ops-v1alpha-DeleteProjectSecretRequest"></a>

### DeleteProjectSecretRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-DeleteProjectSecretResponse"></a>

### DeleteProjectSecretResponse







<a name="demeter-ops-v1alpha-DeleteProjectUserInviteRequest"></a>

### DeleteProjectUserInviteRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-DeleteProjectUserInviteResponse"></a>

### DeleteProjectUserInviteResponse







<a name="demeter-ops-v1alpha-DeleteProjectUserRequest"></a>

### DeleteProjectUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-DeleteProjectUserResponse"></a>

### DeleteProjectUserResponse







<a name="demeter-ops-v1alpha-FetchMeProjectUserRequest"></a>

### FetchMeProjectUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-FetchMeProjectUserResponse"></a>

### FetchMeProjectUserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [ProjectUser](#demeter-ops-v1alpha-ProjectUser) | repeated |  |






<a name="demeter-ops-v1alpha-FetchProjectByIdRequest"></a>

### FetchProjectByIdRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-FetchProjectByIdResponse"></a>

### FetchProjectByIdResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [Project](#demeter-ops-v1alpha-Project) | repeated |  |






<a name="demeter-ops-v1alpha-FetchProjectByNamespaceRequest"></a>

### FetchProjectByNamespaceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| namespace | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-FetchProjectByNamespaceResponse"></a>

### FetchProjectByNamespaceResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [Project](#demeter-ops-v1alpha-Project) | repeated |  |






<a name="demeter-ops-v1alpha-FetchProjectSecretsRequest"></a>

### FetchProjectSecretsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-FetchProjectSecretsResponse"></a>

### FetchProjectSecretsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [ProjectSecret](#demeter-ops-v1alpha-ProjectSecret) | repeated |  |






<a name="demeter-ops-v1alpha-FetchProjectUserInvitesRequest"></a>

### FetchProjectUserInvitesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| page | [uint32](#uint32) | optional |  |
| page_size | [uint32](#uint32) | optional |  |






<a name="demeter-ops-v1alpha-FetchProjectUserInvitesResponse"></a>

### FetchProjectUserInvitesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [ProjectUserInvite](#demeter-ops-v1alpha-ProjectUserInvite) | repeated |  |






<a name="demeter-ops-v1alpha-FetchProjectUsersRequest"></a>

### FetchProjectUsersRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| page | [uint32](#uint32) | optional |  |
| page_size | [uint32](#uint32) | optional |  |






<a name="demeter-ops-v1alpha-FetchProjectUsersResponse"></a>

### FetchProjectUsersResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [ProjectUser](#demeter-ops-v1alpha-ProjectUser) | repeated |  |






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
| billing_provider | [string](#string) |  |  |
| billing_provider_id | [string](#string) |  |  |
| billing_subscription_id | [string](#string) | optional |  |
| created_at | [string](#string) |  |  |
| updated_at | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-ProjectSecret"></a>

### ProjectSecret



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| project_id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| created_at | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-ProjectUser"></a>

### ProjectUser



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| email | [string](#string) |  |  |
| role | [string](#string) |  |  |
| created_at | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-ProjectUserInvite"></a>

### ProjectUserInvite



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| project_id | [string](#string) |  |  |
| email | [string](#string) |  |  |
| role | [string](#string) |  |  |
| status | [string](#string) |  |  |
| expires_in | [string](#string) |  |  |
| created_at | [string](#string) |  |  |
| updated_at | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-ResendProjectUserInviteRequest"></a>

### ResendProjectUserInviteRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-ResendProjectUserInviteResponse"></a>

### ResendProjectUserInviteResponse







<a name="demeter-ops-v1alpha-UpdateProjectRequest"></a>

### UpdateProjectRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |






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
| FetchProjectById | [FetchProjectByIdRequest](#demeter-ops-v1alpha-FetchProjectByIdRequest) | [FetchProjectByIdResponse](#demeter-ops-v1alpha-FetchProjectByIdResponse) |  |
| FetchProjectByNamespace | [FetchProjectByNamespaceRequest](#demeter-ops-v1alpha-FetchProjectByNamespaceRequest) | [FetchProjectByNamespaceResponse](#demeter-ops-v1alpha-FetchProjectByNamespaceResponse) |  |
| CreateProject | [CreateProjectRequest](#demeter-ops-v1alpha-CreateProjectRequest) | [CreateProjectResponse](#demeter-ops-v1alpha-CreateProjectResponse) |  |
| UpdateProject | [UpdateProjectRequest](#demeter-ops-v1alpha-UpdateProjectRequest) | [UpdateProjectResponse](#demeter-ops-v1alpha-UpdateProjectResponse) |  |
| DeleteProject | [DeleteProjectRequest](#demeter-ops-v1alpha-DeleteProjectRequest) | [DeleteProjectResponse](#demeter-ops-v1alpha-DeleteProjectResponse) |  |
| FetchProjectSecrets | [FetchProjectSecretsRequest](#demeter-ops-v1alpha-FetchProjectSecretsRequest) | [FetchProjectSecretsResponse](#demeter-ops-v1alpha-FetchProjectSecretsResponse) |  |
| CreateProjectSecret | [CreateProjectSecretRequest](#demeter-ops-v1alpha-CreateProjectSecretRequest) | [CreateProjectSecretResponse](#demeter-ops-v1alpha-CreateProjectSecretResponse) |  |
| DeleteProjectSecret | [DeleteProjectSecretRequest](#demeter-ops-v1alpha-DeleteProjectSecretRequest) | [DeleteProjectSecretResponse](#demeter-ops-v1alpha-DeleteProjectSecretResponse) |  |
| FetchProjectUsers | [FetchProjectUsersRequest](#demeter-ops-v1alpha-FetchProjectUsersRequest) | [FetchProjectUsersResponse](#demeter-ops-v1alpha-FetchProjectUsersResponse) |  |
| FetchMeProjectUser | [FetchMeProjectUserRequest](#demeter-ops-v1alpha-FetchMeProjectUserRequest) | [FetchMeProjectUserResponse](#demeter-ops-v1alpha-FetchMeProjectUserResponse) |  |
| DeleteProjectUser | [DeleteProjectUserRequest](#demeter-ops-v1alpha-DeleteProjectUserRequest) | [DeleteProjectUserResponse](#demeter-ops-v1alpha-DeleteProjectUserResponse) |  |
| FetchProjectUserInvites | [FetchProjectUserInvitesRequest](#demeter-ops-v1alpha-FetchProjectUserInvitesRequest) | [FetchProjectUserInvitesResponse](#demeter-ops-v1alpha-FetchProjectUserInvitesResponse) |  |
| CreateProjectUserInvite | [CreateProjectUserInviteRequest](#demeter-ops-v1alpha-CreateProjectUserInviteRequest) | [CreateProjectUserInviteResponse](#demeter-ops-v1alpha-CreateProjectUserInviteResponse) |  |
| AcceptProjectUserInvite | [AcceptProjectUserInviteRequest](#demeter-ops-v1alpha-AcceptProjectUserInviteRequest) | [AcceptProjectUserInviteResponse](#demeter-ops-v1alpha-AcceptProjectUserInviteResponse) |  |
| ResendProjectUserInvite | [ResendProjectUserInviteRequest](#demeter-ops-v1alpha-ResendProjectUserInviteRequest) | [ResendProjectUserInviteResponse](#demeter-ops-v1alpha-ResendProjectUserInviteResponse) |  |
| DeleteProjectUserInvite | [DeleteProjectUserInviteRequest](#demeter-ops-v1alpha-DeleteProjectUserInviteRequest) | [DeleteProjectUserInviteResponse](#demeter-ops-v1alpha-DeleteProjectUserInviteResponse) |  |

 



<a name="proto_demeter_ops_v1alpha_resource-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## proto/demeter/ops/v1alpha/resource.proto



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
| name | [string](#string) |  |  |
| kind | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-DeleteResourceRequest"></a>

### DeleteResourceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-DeleteResourceResponse"></a>

### DeleteResourceResponse







<a name="demeter-ops-v1alpha-FetchResourcesByIdRequest"></a>

### FetchResourcesByIdRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






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
| category | [string](#string) | optional |  |






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
| name | [string](#string) |  |  |
| kind | [string](#string) |  |  |
| spec | [string](#string) |  |  |
| annotations | [string](#string) | optional |  |
| status | [string](#string) |  |  |
| created_at | [string](#string) |  |  |
| updated_at | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-UpdateResourceRequest"></a>

### UpdateResourceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| spec_patch | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-UpdateResourceResponse"></a>

### UpdateResourceResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| updated | [Resource](#demeter-ops-v1alpha-Resource) |  |  |





 

 

 


<a name="demeter-ops-v1alpha-ResourceService"></a>

### ResourceService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| FetchResources | [FetchResourcesRequest](#demeter-ops-v1alpha-FetchResourcesRequest) | [FetchResourcesResponse](#demeter-ops-v1alpha-FetchResourcesResponse) |  |
| FetchResourcesById | [FetchResourcesByIdRequest](#demeter-ops-v1alpha-FetchResourcesByIdRequest) | [FetchResourcesByIdResponse](#demeter-ops-v1alpha-FetchResourcesByIdResponse) |  |
| CreateResource | [CreateResourceRequest](#demeter-ops-v1alpha-CreateResourceRequest) | [CreateResourceResponse](#demeter-ops-v1alpha-CreateResourceResponse) |  |
| UpdateResource | [UpdateResourceRequest](#demeter-ops-v1alpha-UpdateResourceRequest) | [UpdateResourceResponse](#demeter-ops-v1alpha-UpdateResourceResponse) |  |
| DeleteResource | [DeleteResourceRequest](#demeter-ops-v1alpha-DeleteResourceRequest) | [DeleteResourceResponse](#demeter-ops-v1alpha-DeleteResourceResponse) |  |

 



<a name="proto_demeter_ops_v1alpha_usage-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## proto/demeter/ops/v1alpha/usage.proto



<a name="demeter-ops-v1alpha-FetchUsageClusterRequest"></a>

### FetchUsageClusterRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| page | [uint32](#uint32) | optional |  |
| page_size | [uint32](#uint32) | optional |  |






<a name="demeter-ops-v1alpha-FetchUsageClusterResponse"></a>

### FetchUsageClusterResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| clusters | [string](#string) | repeated |  |






<a name="demeter-ops-v1alpha-FetchUsageReportRequest"></a>

### FetchUsageReportRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| project_id | [string](#string) |  |  |
| page | [uint32](#uint32) | optional |  |
| page_size | [uint32](#uint32) | optional |  |
| cluster_id | [string](#string) | optional |  |






<a name="demeter-ops-v1alpha-FetchUsageReportResponse"></a>

### FetchUsageReportResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [UsageReport](#demeter-ops-v1alpha-UsageReport) | repeated |  |






<a name="demeter-ops-v1alpha-UsageReport"></a>

### UsageReport



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| resource_id | [string](#string) |  |  |
| resource_name | [string](#string) |  |  |
| resource_kind | [string](#string) |  |  |
| resource_spec | [string](#string) |  |  |
| units | [int64](#int64) |  |  |
| tier | [string](#string) |  |  |
| period | [string](#string) |  |  |
| units_cost | [double](#double) | optional |  |
| minimum_cost | [double](#double) | optional |  |





 

 

 


<a name="demeter-ops-v1alpha-UsageService"></a>

### UsageService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| FetchUsageReport | [FetchUsageReportRequest](#demeter-ops-v1alpha-FetchUsageReportRequest) | [FetchUsageReportResponse](#demeter-ops-v1alpha-FetchUsageReportResponse) |  |
| FetchUsageCluster | [FetchUsageClusterRequest](#demeter-ops-v1alpha-FetchUsageClusterRequest) | [FetchUsageClusterResponse](#demeter-ops-v1alpha-FetchUsageClusterResponse) |  |

 



<a name="proto_demeter_ops_v1alpha_worker-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## proto/demeter/ops/v1alpha/worker.proto



<a name="demeter-ops-v1alpha-DeleteKeyValueRequest"></a>

### DeleteKeyValueRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| worker_id | [string](#string) |  |  |
| key | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-DeleteKeyValueResponse"></a>

### DeleteKeyValueResponse







<a name="demeter-ops-v1alpha-FetchKeyValueRequest"></a>

### FetchKeyValueRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| worker_id | [string](#string) |  |  |
| key | [string](#string) | optional |  |
| page | [uint32](#uint32) | optional |  |
| page_size | [uint32](#uint32) | optional |  |






<a name="demeter-ops-v1alpha-FetchKeyValueResponse"></a>

### FetchKeyValueResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_records | [uint32](#uint32) |  |  |
| records | [KeyValue](#demeter-ops-v1alpha-KeyValue) | repeated |  |






<a name="demeter-ops-v1alpha-FetchLogsRequest"></a>

### FetchLogsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| worker_id | [string](#string) |  |  |
| cursor | [uint32](#uint32) |  |  |
| direction | [Direction](#demeter-ops-v1alpha-Direction) | optional |  |
| limit | [uint32](#uint32) | optional |  |






<a name="demeter-ops-v1alpha-FetchLogsResponse"></a>

### FetchLogsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| records | [Log](#demeter-ops-v1alpha-Log) | repeated |  |






<a name="demeter-ops-v1alpha-GetPublicKeySignerRequest"></a>

### GetPublicKeySignerRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| worker_id | [string](#string) |  |  |
| key_name | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-GetPublicKeySignerResponse"></a>

### GetPublicKeySignerResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| public_key | [bytes](#bytes) | optional |  |






<a name="demeter-ops-v1alpha-KeyValue"></a>

### KeyValue



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [bytes](#bytes) |  |  |






<a name="demeter-ops-v1alpha-ListSignerRequest"></a>

### ListSignerRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| worker_id | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-ListSignerResponse"></a>

### ListSignerResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keys | [KeyValue](#demeter-ops-v1alpha-KeyValue) | repeated |  |






<a name="demeter-ops-v1alpha-Log"></a>

### Log



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| timestamp | [uint32](#uint32) |  |  |
| level | [string](#string) |  |  |
| message | [string](#string) |  |  |
| context | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-SignPayloadSignerRequest"></a>

### SignPayloadSignerRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| worker_id | [string](#string) |  |  |
| key_name | [string](#string) |  |  |
| payload | [bytes](#bytes) |  |  |






<a name="demeter-ops-v1alpha-SignPayloadSignerResponse"></a>

### SignPayloadSignerResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signed_payload | [bytes](#bytes) |  |  |






<a name="demeter-ops-v1alpha-Signer"></a>

### Signer



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key_name | [string](#string) |  |  |






<a name="demeter-ops-v1alpha-UpdateKeyValueRequest"></a>

### UpdateKeyValueRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| worker_id | [string](#string) |  |  |
| key | [string](#string) |  |  |
| value | [bytes](#bytes) |  |  |






<a name="demeter-ops-v1alpha-UpdateKeyValueResponse"></a>

### UpdateKeyValueResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| updated | [KeyValue](#demeter-ops-v1alpha-KeyValue) |  |  |





 


<a name="demeter-ops-v1alpha-Direction"></a>

### Direction


| Name | Number | Description |
| ---- | ------ | ----------- |
| Prev | 0 |  |
| Next | 1 |  |


 

 


<a name="demeter-ops-v1alpha-KeyValueService"></a>

### KeyValueService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| FetchKeyValue | [FetchKeyValueRequest](#demeter-ops-v1alpha-FetchKeyValueRequest) | [FetchKeyValueResponse](#demeter-ops-v1alpha-FetchKeyValueResponse) |  |
| UpdateKeyValue | [UpdateKeyValueRequest](#demeter-ops-v1alpha-UpdateKeyValueRequest) | [UpdateKeyValueResponse](#demeter-ops-v1alpha-UpdateKeyValueResponse) |  |
| DeleteKeyValue | [DeleteKeyValueRequest](#demeter-ops-v1alpha-DeleteKeyValueRequest) | [DeleteKeyValueResponse](#demeter-ops-v1alpha-DeleteKeyValueResponse) |  |


<a name="demeter-ops-v1alpha-LogsService"></a>

### LogsService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| FetchWindow | [FetchLogsRequest](#demeter-ops-v1alpha-FetchLogsRequest) | [FetchLogsResponse](#demeter-ops-v1alpha-FetchLogsResponse) |  |


<a name="demeter-ops-v1alpha-SignerService"></a>

### SignerService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| List | [ListSignerRequest](#demeter-ops-v1alpha-ListSignerRequest) | [ListSignerResponse](#demeter-ops-v1alpha-ListSignerResponse) |  |
| GetPublicKey | [GetPublicKeySignerRequest](#demeter-ops-v1alpha-GetPublicKeySignerRequest) | [GetPublicKeySignerResponse](#demeter-ops-v1alpha-GetPublicKeySignerResponse) |  |
| SignPayload | [SignPayloadSignerRequest](#demeter-ops-v1alpha-SignPayloadSignerRequest) | [SignPayloadSignerResponse](#demeter-ops-v1alpha-SignPayloadSignerResponse) |  |

 



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

