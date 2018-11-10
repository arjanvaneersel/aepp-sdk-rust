# \TransactionApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pending_transactions**](TransactionApi.md#get_pending_transactions) | **Get** /debug/transactions/pending | 
[**get_transaction_by_hash**](TransactionApi.md#get_transaction_by_hash) | **Get** /transactions/{hash} | 
[**get_transaction_info_by_hash**](TransactionApi.md#get_transaction_info_by_hash) | **Get** /transactions/{hash}/info | 
[**post_spend**](TransactionApi.md#post_spend) | **Post** /debug/transactions/spend | 
[**post_transaction**](TransactionApi.md#post_transaction) | **Post** /transactions | 


# **get_pending_transactions**
> ::models::GenericTxs get_pending_transactions()


Get pending transactions

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GenericTxs**](GenericTxs.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction_by_hash**
> ::models::GenericSignedTx get_transaction_by_hash(hash)


Get a transaction by hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| The hash of the transaction | 

### Return type

[**::models::GenericSignedTx**](GenericSignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction_info_by_hash**
> ::models::ContractCallObject get_transaction_info_by_hash(hash)


Get a transaction info by hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| The hash of the transaction | 

### Return type

[**::models::ContractCallObject**](ContractCallObject.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_spend**
> ::models::UnsignedTx post_spend(body)


Get a spend transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**SpendTx**](SpendTx.md)| A spend transaction | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_transaction**
> ::models::PostTxResponse post_transaction(body)


Post a new transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Tx**](Tx.md)| The new transaction | 

### Return type

[**::models::PostTxResponse**](PostTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

