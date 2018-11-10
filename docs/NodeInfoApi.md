# \NodeInfoApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_node_beneficiary**](NodeInfoApi.md#get_node_beneficiary) | **Get** /debug/accounts/beneficiary | 
[**get_node_pubkey**](NodeInfoApi.md#get_node_pubkey) | **Get** /debug/accounts/node | 
[**get_peer_pubkey**](NodeInfoApi.md#get_peer_pubkey) | **Get** /peers/pubkey | 
[**get_peers**](NodeInfoApi.md#get_peers) | **Get** /debug/peers | 
[**get_status**](NodeInfoApi.md#get_status) | **Get** /status | 


# **get_node_beneficiary**
> ::models::PubKey get_node_beneficiary()


Get node's beneficiary public key

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::PubKey**](PubKey.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_pubkey**
> ::models::PubKey get_node_pubkey()


Get node's public key

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::PubKey**](PubKey.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_peer_pubkey**
> ::models::InlineResponse2003 get_peer_pubkey()


Get peer public key

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_peers**
> ::models::Peers get_peers()


Get node Peers

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::Peers**](Peers.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_status**
> ::models::Status get_status()


Get the status of a node

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::Status**](Status.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

