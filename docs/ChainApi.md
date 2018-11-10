# \ChainApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_current_generation**](ChainApi.md#get_current_generation) | **Get** /generations/current | 
[**get_current_key_block**](ChainApi.md#get_current_key_block) | **Get** /key-blocks/current | 
[**get_current_key_block_hash**](ChainApi.md#get_current_key_block_hash) | **Get** /key-blocks/current/hash | 
[**get_current_key_block_height**](ChainApi.md#get_current_key_block_height) | **Get** /key-blocks/current/height | 
[**get_generation_by_hash**](ChainApi.md#get_generation_by_hash) | **Get** /generations/hash/{hash} | 
[**get_generation_by_height**](ChainApi.md#get_generation_by_height) | **Get** /generations/height/{height} | 
[**get_key_block_by_hash**](ChainApi.md#get_key_block_by_hash) | **Get** /key-blocks/hash/{hash} | 
[**get_key_block_by_height**](ChainApi.md#get_key_block_by_height) | **Get** /key-blocks/height/{height} | 
[**get_micro_block_header_by_hash**](ChainApi.md#get_micro_block_header_by_hash) | **Get** /micro-blocks/hash/{hash}/header | 
[**get_micro_block_transaction_by_hash_and_index**](ChainApi.md#get_micro_block_transaction_by_hash_and_index) | **Get** /micro-blocks/hash/{hash}/transactions/index/{index} | 
[**get_micro_block_transactions_by_hash**](ChainApi.md#get_micro_block_transactions_by_hash) | **Get** /micro-blocks/hash/{hash}/transactions | 
[**get_micro_block_transactions_count_by_hash**](ChainApi.md#get_micro_block_transactions_count_by_hash) | **Get** /micro-blocks/hash/{hash}/transactions/count | 
[**get_pending_key_block**](ChainApi.md#get_pending_key_block) | **Get** /key-blocks/pending | 
[**get_top_block**](ChainApi.md#get_top_block) | **Get** /blocks/top | 
[**post_key_block**](ChainApi.md#post_key_block) | **Post** /key-blocks | 


# **get_current_generation**
> ::models::Generation get_current_generation()


Get the current generation

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::Generation**](Generation.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_current_key_block**
> ::models::KeyBlock get_current_key_block()


Get the current key block

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::KeyBlock**](KeyBlock.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_current_key_block_hash**
> ::models::InlineResponse200 get_current_key_block_hash()


Get the hash of the current key block

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_current_key_block_height**
> ::models::InlineResponse2001 get_current_key_block_height()


Get the height of the current key block

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_generation_by_hash**
> ::models::Generation get_generation_by_hash(hash)


Get a generation by hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| The hash of the generation | 

### Return type

[**::models::Generation**](Generation.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_generation_by_height**
> ::models::Generation get_generation_by_height(height)


Get a generation by height

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **height** | **i32**| The height of the generation | 

### Return type

[**::models::Generation**](Generation.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_key_block_by_hash**
> ::models::KeyBlock get_key_block_by_hash(hash)


Get a key block by hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| The hash of the block | 

### Return type

[**::models::KeyBlock**](KeyBlock.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_key_block_by_height**
> ::models::KeyBlock get_key_block_by_height(height)


Get a key block by height

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **height** | **i32**| The height of the block | 

### Return type

[**::models::KeyBlock**](KeyBlock.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_micro_block_header_by_hash**
> ::models::MicroBlockHeader get_micro_block_header_by_hash(hash)


Get a micro block header by hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| The hash of the block | 

### Return type

[**::models::MicroBlockHeader**](MicroBlockHeader.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_micro_block_transaction_by_hash_and_index**
> ::models::GenericSignedTx get_micro_block_transaction_by_hash_and_index(hash, index)


Get a micro block transaction by hash and index

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| The hash of the block | 
  **index** | **i32**| The index of the transaction in a block | 

### Return type

[**::models::GenericSignedTx**](GenericSignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_micro_block_transactions_by_hash**
> ::models::GenericTxs get_micro_block_transactions_by_hash(hash)


Get micro block transactions by hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| The hash of the block | 

### Return type

[**::models::GenericTxs**](GenericTxs.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_micro_block_transactions_count_by_hash**
> ::models::InlineResponse2002 get_micro_block_transactions_count_by_hash(hash)


Get micro block transaction count by hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| The hash of the block | 

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_pending_key_block**
> ::models::KeyBlock get_pending_key_block()


Get the pending key block

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::KeyBlock**](KeyBlock.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_top_block**
> ::models::KeyBlockOrMicroBlockHeader get_top_block()


Get the top block (either key or micro block)

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::KeyBlockOrMicroBlockHeader**](KeyBlockOrMicroBlockHeader.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_key_block**
> post_key_block(body)


Post a mined key block

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**KeyBlock**](KeyBlock.md)| Mined key block | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

