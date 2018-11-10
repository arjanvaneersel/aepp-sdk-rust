# \InternalApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**call_contract**](InternalApi.md#call_contract) | **Post** /debug/contracts/code/call | 
[**compile_contract**](InternalApi.md#compile_contract) | **Post** /debug/contracts/code/compile | 
[**decode_data**](InternalApi.md#decode_data) | **Post** /debug/contracts/code/decode-data | 
[**encode_calldata**](InternalApi.md#encode_calldata) | **Post** /debug/contracts/code/encode-calldata | 
[**get_commitment_id**](InternalApi.md#get_commitment_id) | **Get** /debug/names/commitment-id | 
[**get_node_beneficiary**](InternalApi.md#get_node_beneficiary) | **Get** /debug/accounts/beneficiary | 
[**get_node_pubkey**](InternalApi.md#get_node_pubkey) | **Get** /debug/accounts/node | 
[**get_peers**](InternalApi.md#get_peers) | **Get** /debug/peers | 
[**get_pending_transactions**](InternalApi.md#get_pending_transactions) | **Get** /debug/transactions/pending | 
[**post_channel_close_mutual**](InternalApi.md#post_channel_close_mutual) | **Post** /debug/channels/close/mutual | 
[**post_channel_close_solo**](InternalApi.md#post_channel_close_solo) | **Post** /debug/channels/close/solo | 
[**post_channel_create**](InternalApi.md#post_channel_create) | **Post** /debug/channels/create | 
[**post_channel_deposit**](InternalApi.md#post_channel_deposit) | **Post** /debug/channels/deposit | 
[**post_channel_settle**](InternalApi.md#post_channel_settle) | **Post** /debug/channels/settle | 
[**post_channel_slash**](InternalApi.md#post_channel_slash) | **Post** /debug/channels/slash | 
[**post_channel_snapshot_solo**](InternalApi.md#post_channel_snapshot_solo) | **Post** /debug/channels/snapshot/solo | 
[**post_channel_withdraw**](InternalApi.md#post_channel_withdraw) | **Post** /debug/channels/withdraw | 
[**post_contract_call**](InternalApi.md#post_contract_call) | **Post** /debug/contracts/call | 
[**post_contract_call_compute**](InternalApi.md#post_contract_call_compute) | **Post** /debug/contracts/call/compute | 
[**post_contract_create**](InternalApi.md#post_contract_create) | **Post** /debug/contracts/create | 
[**post_contract_create_compute**](InternalApi.md#post_contract_create_compute) | **Post** /debug/contracts/create/compute | 
[**post_key_block**](InternalApi.md#post_key_block) | **Post** /key-blocks | 
[**post_name_claim**](InternalApi.md#post_name_claim) | **Post** /debug/names/claim | 
[**post_name_preclaim**](InternalApi.md#post_name_preclaim) | **Post** /debug/names/preclaim | 
[**post_name_revoke**](InternalApi.md#post_name_revoke) | **Post** /debug/names/revoke | 
[**post_name_transfer**](InternalApi.md#post_name_transfer) | **Post** /debug/names/transfer | 
[**post_name_update**](InternalApi.md#post_name_update) | **Post** /debug/names/update | 
[**post_oracle_extend**](InternalApi.md#post_oracle_extend) | **Post** /debug/oracles/extend | 
[**post_oracle_query**](InternalApi.md#post_oracle_query) | **Post** /debug/oracles/query | 
[**post_oracle_register**](InternalApi.md#post_oracle_register) | **Post** /debug/oracles/register | 
[**post_oracle_respond**](InternalApi.md#post_oracle_respond) | **Post** /debug/oracles/respond | 
[**post_spend**](InternalApi.md#post_spend) | **Post** /debug/transactions/spend | 


# **call_contract**
> ::models::CallResult call_contract(body)


Call a sophia function with a given name and argument in the given bytecode off chain.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ContractCallInput**](ContractCallInput.md)| contract call | 

### Return type

[**::models::CallResult**](CallResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **compile_contract**
> ::models::ByteCode compile_contract(body)


Compile a sophia contract from source and return byte code

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Contract**](Contract.md)| contract code | 

### Return type

[**::models::ByteCode**](ByteCode.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **decode_data**
> ::models::SophiaJsonData decode_data(body)


Decode Sophia return data encoded according to Sophia ABI.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**SophiaBinaryData**](SophiaBinaryData.md)| Binary data in Sophia ABI format | 

### Return type

[**::models::SophiaJsonData**](SophiaJsonData.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **encode_calldata**
> ::models::Calldata encode_calldata(body)


Encode sophia data and function name according to sophia ABI.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ContractCallInput**](ContractCallInput.md)| Arguments in sophia | 

### Return type

[**::models::Calldata**](Calldata.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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

# **post_channel_close_mutual**
> ::models::UnsignedTx post_channel_close_mutual(body)


Get a channel_close_mutual transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ChannelCloseMutualTx**](ChannelCloseMutualTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_channel_close_solo**
> ::models::UnsignedTx post_channel_close_solo(body)


Get a channel_close_solo transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ChannelCloseSoloTx**](ChannelCloseSoloTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_channel_create**
> ::models::UnsignedTx post_channel_create(body)


Get a channel_create transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ChannelCreateTx**](ChannelCreateTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_channel_deposit**
> ::models::UnsignedTx post_channel_deposit(body)


Get a channel_deposit transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ChannelDepositTx**](ChannelDepositTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_channel_settle**
> ::models::UnsignedTx post_channel_settle(body)


Get a channel_settle transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ChannelSettleTx**](ChannelSettleTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_channel_slash**
> ::models::UnsignedTx post_channel_slash(body)


Get a channel_slash transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ChannelSlashTx**](ChannelSlashTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_channel_snapshot_solo**
> ::models::UnsignedTx post_channel_snapshot_solo(body)


Get a channel_snapshot_solo transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ChannelSnapshotSoloTx**](ChannelSnapshotSoloTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_channel_withdraw**
> ::models::UnsignedTx post_channel_withdraw(body)


Get a channel_withdrawal transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ChannelWithdrawTx**](ChannelWithdrawTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_contract_call**
> ::models::UnsignedTx post_contract_call(body)


Get a contract_call transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ContractCallTx**](ContractCallTx.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_contract_call_compute**
> ::models::UnsignedTx post_contract_call_compute(body)


Compute the call_data for SOPHIA and get contract_call transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ContractCallCompute**](ContractCallCompute.md)|  | 

### Return type

[**::models::UnsignedTx**](UnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_contract_create**
> ::models::CreateContractUnsignedTx post_contract_create(body)


Get a contract_create transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ContractCreateTx**](ContractCreateTx.md)|  | 

### Return type

[**::models::CreateContractUnsignedTx**](CreateContractUnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_contract_create_compute**
> ::models::CreateContractUnsignedTx post_contract_create_compute(body)


Compute the call_data for SOPHIA and get a contract_create transaction object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ContractCreateCompute**](ContractCreateCompute.md)|  | 

### Return type

[**::models::CreateContractUnsignedTx**](CreateContractUnsignedTx.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
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

