# ContractCallCompute

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caller_id** | [***::models::EncodedHash**](EncodedHash.md) | Contract caller pub_key | [default to null]
**nonce** | **i32** | Caller&#39;s nonce | [optional] [default to null]
**contract_id** | [***::models::EncodedHash**](EncodedHash.md) | Contract&#39;s pub_key | [default to null]
**vm_version** | **i32** | Virtual machine&#39;s version | [default to null]
**fee** | **i32** | Transaction fee | [default to null]
**ttl** | **i32** | Transaction TTL | [optional] [default to null]
**amount** | **i32** | Amount | [default to null]
**gas** | **i32** | Contract gas | [default to null]
**gas_price** | **i32** | Gas price | [default to null]
**function** | **String** | Contract call data function (deprecated, use call) | [optional] [default to null]
**arguments** | **String** | Contract call data function arguments (deprecated, use call) | [optional] [default to null]
**call** | **String** | Source code for a contract with a function __call() &#x3D; f(args), if calling a function f | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


