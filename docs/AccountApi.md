# \AccountApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_by_pubkey**](AccountApi.md#get_account_by_pubkey) | **Get** /accounts/{pubkey} | 
[**get_pending_account_transactions_by_pubkey**](AccountApi.md#get_pending_account_transactions_by_pubkey) | **Get** /accounts/{pubkey}/transactions/pending | 


# **get_account_by_pubkey**
> ::models::Account get_account_by_pubkey(pubkey)


Get an account by public key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| The public key of the account | 

### Return type

[**::models::Account**](Account.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_pending_account_transactions_by_pubkey**
> ::models::GenericTxs get_pending_account_transactions_by_pubkey(pubkey)


Get pending account transactions by public key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| The public key of the account | 

### Return type

[**::models::GenericTxs**](GenericTxs.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

