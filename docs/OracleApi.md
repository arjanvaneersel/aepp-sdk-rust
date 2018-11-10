# \OracleApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_oracle_by_pubkey**](OracleApi.md#get_oracle_by_pubkey) | **Get** /oracles/{pubkey} | 
[**get_oracle_queries_by_pubkey**](OracleApi.md#get_oracle_queries_by_pubkey) | **Get** /oracles/{pubkey}/queries | 
[**get_oracle_query_by_pubkey_and_query_id**](OracleApi.md#get_oracle_query_by_pubkey_and_query_id) | **Get** /oracles/{pubkey}/queries/{query-id} | 
[**post_oracle_extend**](OracleApi.md#post_oracle_extend) | **Post** /debug/oracles/extend | 
[**post_oracle_query**](OracleApi.md#post_oracle_query) | **Post** /debug/oracles/query | 
[**post_oracle_register**](OracleApi.md#post_oracle_register) | **Post** /debug/oracles/register | 
[**post_oracle_respond**](OracleApi.md#post_oracle_respond) | **Post** /debug/oracles/respond | 


# **get_oracle_by_pubkey**
> ::models::RegisteredOracle get_oracle_by_pubkey(pubkey)


Get an oracle by public key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| The public key of the oracle | 

### Return type

[**::models::RegisteredOracle**](RegisteredOracle.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_oracle_queries_by_pubkey**
> ::models::OracleQueries get_oracle_queries_by_pubkey(pubkey, optional)


Get oracle queries by public key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| The public key of the oracle | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pubkey** | **String**| The public key of the oracle | 
 **from** | **String**| Last query id in previous page | 
 **limit** | **i32**| Max number of oracle queries | 
 **_type** | **String**| The type of a query: open, closed or all | 

### Return type

[**::models::OracleQueries**](OracleQueries.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_oracle_query_by_pubkey_and_query_id**
> ::models::OracleQuery get_oracle_query_by_pubkey_and_query_id(pubkey, query_id)


Get an oracle query by public key and query ID

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| The public key of the oracle | 
  **query_id** | **String**| The ID of the query | 

### Return type

[**::models::OracleQuery**](OracleQuery.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_oracle_extend**
> ::models::UnsignedTx post_oracle_extend(body)


Get an oracle_extend transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**OracleExtendTx**](OracleExtendTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_oracle_query**
> ::models::UnsignedTx post_oracle_query(body)


Get an oracle_query transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**OracleQueryTx**](OracleQueryTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_oracle_register**
> ::models::UnsignedTx post_oracle_register(body)


Get a oracle_register transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**OracleRegisterTx**](OracleRegisterTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_oracle_respond**
> ::models::UnsignedTx post_oracle_respond(body)


Get an oracle_response transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**OracleRespondTx**](OracleRespondTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

