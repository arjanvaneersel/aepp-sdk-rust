# \ChannelApi

All URIs are relative to *http://localhost/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_channel_by_pubkey**](ChannelApi.md#get_channel_by_pubkey) | **Get** /channels/{pubkey} | 
[**post_channel_close_mutual**](ChannelApi.md#post_channel_close_mutual) | **Post** /debug/channels/close/mutual | 
[**post_channel_close_solo**](ChannelApi.md#post_channel_close_solo) | **Post** /debug/channels/close/solo | 
[**post_channel_create**](ChannelApi.md#post_channel_create) | **Post** /debug/channels/create | 
[**post_channel_deposit**](ChannelApi.md#post_channel_deposit) | **Post** /debug/channels/deposit | 
[**post_channel_settle**](ChannelApi.md#post_channel_settle) | **Post** /debug/channels/settle | 
[**post_channel_slash**](ChannelApi.md#post_channel_slash) | **Post** /debug/channels/slash | 
[**post_channel_snapshot_solo**](ChannelApi.md#post_channel_snapshot_solo) | **Post** /debug/channels/snapshot/solo | 
[**post_channel_withdraw**](ChannelApi.md#post_channel_withdraw) | **Post** /debug/channels/withdraw | 


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

