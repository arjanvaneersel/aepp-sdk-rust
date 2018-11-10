# \NameServiceApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_commitment_id**](NameServiceApi.md#get_commitment_id) | **Get** /debug/names/commitment-id | 
[**get_name_entry_by_name**](NameServiceApi.md#get_name_entry_by_name) | **Get** /names/{name} | 
[**post_name_claim**](NameServiceApi.md#post_name_claim) | **Post** /debug/names/claim | 
[**post_name_preclaim**](NameServiceApi.md#post_name_preclaim) | **Post** /debug/names/preclaim | 
[**post_name_revoke**](NameServiceApi.md#post_name_revoke) | **Post** /debug/names/revoke | 
[**post_name_transfer**](NameServiceApi.md#post_name_transfer) | **Post** /debug/names/transfer | 
[**post_name_update**](NameServiceApi.md#post_name_update) | **Post** /debug/names/update | 


# **get_commitment_id**
> ::models::CommitmentId get_commitment_id(name, salt)


Compute commitment ID for a given salt and name

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| Name | 
  **salt** | **i32**| Salt | 

### Return type

[**::models::CommitmentId**](CommitmentId.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_name_entry_by_name**
> ::models::NameEntry get_name_entry_by_name(name)


Get name entry from naming system

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The name key of the name entry | 

### Return type

[**::models::NameEntry**](NameEntry.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_name_claim**
> ::models::UnsignedTx post_name_claim(body)


Get a name_claim transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**NameClaimTx**](NameClaimTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_name_preclaim**
> ::models::UnsignedTx post_name_preclaim(body)


Get a name_preclaim transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**NamePreclaimTx**](NamePreclaimTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_name_revoke**
> ::models::UnsignedTx post_name_revoke(body)


Get a name_revoke transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**NameRevokeTx**](NameRevokeTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_name_transfer**
> ::models::UnsignedTx post_name_transfer(body)


Get a name_transfer transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**NameTransferTx**](NameTransferTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_name_update**
> ::models::UnsignedTx post_name_update(body)


Get a name_update transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**NameUpdateTx**](NameUpdateTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

