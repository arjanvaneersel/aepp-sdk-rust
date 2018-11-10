# \ContractApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**call_contract**](ContractApi.md#call_contract) | **Post** /debug/contracts/code/call | 
[**compile_contract**](ContractApi.md#compile_contract) | **Post** /debug/contracts/code/compile | 
[**decode_data**](ContractApi.md#decode_data) | **Post** /debug/contracts/code/decode-data | 
[**encode_calldata**](ContractApi.md#encode_calldata) | **Post** /debug/contracts/code/encode-calldata | 
[**get_contract**](ContractApi.md#get_contract) | **Get** /contracts/{pubkey} | 
[**get_contract_code**](ContractApi.md#get_contract_code) | **Get** /contracts/{pubkey}/code | 
[**get_contract_po_i**](ContractApi.md#get_contract_po_i) | **Get** /contracts/{pubkey}/poi | 
[**get_contract_store**](ContractApi.md#get_contract_store) | **Get** /contracts/{pubkey}/store | 
[**post_contract_call**](ContractApi.md#post_contract_call) | **Post** /debug/contracts/call | 
[**post_contract_call_compute**](ContractApi.md#post_contract_call_compute) | **Post** /debug/contracts/call/compute | 
[**post_contract_create**](ContractApi.md#post_contract_create) | **Post** /debug/contracts/create | 
[**post_contract_create_compute**](ContractApi.md#post_contract_create_compute) | **Post** /debug/contracts/create/compute | 


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

