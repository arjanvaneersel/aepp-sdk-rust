# ChannelForceProgressTx

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_id** | [***::models::EncodedHash**](EncodedHash.md) |  | [default to null]
**from_id** | [***::models::EncodedHash**](EncodedHash.md) |  | [default to null]
**payload** | **String** |  | [default to null]
**round** | **i64** | Channel&#39;s next round | [default to null]
**update** | [***::models::OffChainUpdate**](OffChainUpdate.md) | Update to be applied. Must be a OffChainCallContract | [default to null]
**state_hash** | [***::models::EncodedHash**](EncodedHash.md) | Channel&#39;s next state_hash | [default to null]
**ttl** | **i64** |  | [optional] [default to null]
**fee** | **i64** |  | [default to null]
**nonce** | **i64** |  | [optional] [default to null]
**offchain_trees** | [***::models::EncodedHash**](EncodedHash.md) | The whole set of off-chain state trees | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


