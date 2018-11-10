# ContractCreateCompute

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner_id** | **String** | Contract owner pub_key | [default to null]
**nonce** | **i32** | Owner&#39;s nonce | [optional] [default to null]
**code** | **String** | Contract&#39;s code | [default to null]
**vm_version** | **i32** | Virtual machine&#39;s version | [default to null]
**deposit** | **i32** | Deposit | [default to null]
**amount** | **i32** | Amount | [default to null]
**gas** | **i32** | Contract gas | [default to null]
**gas_price** | **i32** | Gas price | [default to null]
**fee** | **i32** | Transaction fee | [default to null]
**ttl** | **i32** | Transaction TTL | [optional] [default to null]
**arguments** | **String** | Contract call data init function arguments (deprecated, use call) | [optional] [default to null]
**call** | **String** | Source code for a contract with a function __call() &#x3D; init(args) | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


