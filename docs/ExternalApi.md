# \ExternalApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_by_pubkey**](ExternalApi.md#get_account_by_pubkey) | **Get** /accounts/{pubkey} | 
[**get_channel_by_pubkey**](ExternalApi.md#get_channel_by_pubkey) | **Get** /channels/{pubkey} | 
[**get_contract**](ExternalApi.md#get_contract) | **Get** /contracts/{pubkey} | 
[**get_contract_code**](ExternalApi.md#get_contract_code) | **Get** /contracts/{pubkey}/code | 
[**get_contract_po_i**](ExternalApi.md#get_contract_po_i) | **Get** /contracts/{pubkey}/poi | 
[**get_contract_store**](ExternalApi.md#get_contract_store) | **Get** /contracts/{pubkey}/store | 
[**get_current_generation**](ExternalApi.md#get_current_generation) | **Get** /generations/current | 
[**get_current_key_block**](ExternalApi.md#get_current_key_block) | **Get** /key-blocks/current | 
[**get_current_key_block_hash**](ExternalApi.md#get_current_key_block_hash) | **Get** /key-blocks/current/hash | 
[**get_current_key_block_height**](ExternalApi.md#get_current_key_block_height) | **Get** /key-blocks/current/height | 
[**get_generation_by_hash**](ExternalApi.md#get_generation_by_hash) | **Get** /generations/hash/{hash} | 
[**get_generation_by_height**](ExternalApi.md#get_generation_by_height) | **Get** /generations/height/{height} | 
[**get_key_block_by_hash**](ExternalApi.md#get_key_block_by_hash) | **Get** /key-blocks/hash/{hash} | 
[**get_key_block_by_height**](ExternalApi.md#get_key_block_by_height) | **Get** /key-blocks/height/{height} | 
[**get_micro_block_header_by_hash**](ExternalApi.md#get_micro_block_header_by_hash) | **Get** /micro-blocks/hash/{hash}/header | 
[**get_micro_block_transaction_by_hash_and_index**](ExternalApi.md#get_micro_block_transaction_by_hash_and_index) | **Get** /micro-blocks/hash/{hash}/transactions/index/{index} | 
[**get_micro_block_transactions_by_hash**](ExternalApi.md#get_micro_block_transactions_by_hash) | **Get** /micro-blocks/hash/{hash}/transactions | 
[**get_micro_block_transactions_count_by_hash**](ExternalApi.md#get_micro_block_transactions_count_by_hash) | **Get** /micro-blocks/hash/{hash}/transactions/count | 
[**get_name_entry_by_name**](ExternalApi.md#get_name_entry_by_name) | **Get** /names/{name} | 
[**get_oracle_by_pubkey**](ExternalApi.md#get_oracle_by_pubkey) | **Get** /oracles/{pubkey} | 
[**get_oracle_queries_by_pubkey**](ExternalApi.md#get_oracle_queries_by_pubkey) | **Get** /oracles/{pubkey}/queries | 
[**get_oracle_query_by_pubkey_and_query_id**](ExternalApi.md#get_oracle_query_by_pubkey_and_query_id) | **Get** /oracles/{pubkey}/queries/{query-id} | 
[**get_peer_pubkey**](ExternalApi.md#get_peer_pubkey) | **Get** /peers/pubkey | 
[**get_pending_account_transactions_by_pubkey**](ExternalApi.md#get_pending_account_transactions_by_pubkey) | **Get** /accounts/{pubkey}/transactions/pending | 
[**get_pending_key_block**](ExternalApi.md#get_pending_key_block) | **Get** /key-blocks/pending | 
[**get_status**](ExternalApi.md#get_status) | **Get** /status | 
[**get_top_block**](ExternalApi.md#get_top_block) | **Get** /blocks/top | 
[**get_transaction_by_hash**](ExternalApi.md#get_transaction_by_hash) | **Get** /transactions/{hash} | 
[**get_transaction_info_by_hash**](ExternalApi.md#get_transaction_info_by_hash) | **Get** /transactions/{hash}/info | 
[**post_transaction**](ExternalApi.md#post_transaction) | **Post** /transactions | 


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

# **get_channel_by_pubkey**
> ::models::Channel get_channel_by_pubkey(pubkey)


Get channel by public key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| The pubkey of the channel | 

### Return type

[**::models::Channel**](Channel.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_contract**
> ::models::ContractObject get_contract(pubkey)


Get a contract by pubkey

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| The pubkey of the contract | 

### Return type

[**::models::ContractObject**](ContractObject.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_contract_code**
> ::models::ByteCode get_contract_code(pubkey)


Get contract code by pubkey

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| The pubkey of the contract | 

### Return type

[**::models::ByteCode**](ByteCode.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_contract_po_i**
> ::models::PoI get_contract_po_i(pubkey)


Get a proof of inclusion for a contract

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| Contract pubkey to get proof for | 

### Return type

[**::models::PoI**](PoI.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_contract_store**
> ::models::ContractStore get_contract_store(pubkey)


Get contract store by pubkey

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pubkey** | **String**| The pubkey of the contract | 

### Return type

[**::models::ContractStore**](ContractStore.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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

