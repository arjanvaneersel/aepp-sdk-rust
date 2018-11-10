# Rust API client for swagger

This is an SDK for the [Aeternity](https://www.aeternity.com/) Epoch API for the Rust programming language.

## Documentation for API Endpoints

### API version: 0.25.0

All URIs are relative to *http://localhost/v2*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountApi* | [**get_account_by_pubkey**](docs/AccountApi.md#get_account_by_pubkey) | **Get** /accounts/{pubkey} | 
*AccountApi* | [**get_pending_account_transactions_by_pubkey**](docs/AccountApi.md#get_pending_account_transactions_by_pubkey) | **Get** /accounts/{pubkey}/transactions/pending | 
*ChainApi* | [**get_current_generation**](docs/ChainApi.md#get_current_generation) | **Get** /generations/current | 
*ChainApi* | [**get_current_key_block**](docs/ChainApi.md#get_current_key_block) | **Get** /key-blocks/current | 
*ChainApi* | [**get_current_key_block_hash**](docs/ChainApi.md#get_current_key_block_hash) | **Get** /key-blocks/current/hash | 
*ChainApi* | [**get_current_key_block_height**](docs/ChainApi.md#get_current_key_block_height) | **Get** /key-blocks/current/height | 
*ChainApi* | [**get_generation_by_hash**](docs/ChainApi.md#get_generation_by_hash) | **Get** /generations/hash/{hash} | 
*ChainApi* | [**get_generation_by_height**](docs/ChainApi.md#get_generation_by_height) | **Get** /generations/height/{height} | 
*ChainApi* | [**get_key_block_by_hash**](docs/ChainApi.md#get_key_block_by_hash) | **Get** /key-blocks/hash/{hash} | 
*ChainApi* | [**get_key_block_by_height**](docs/ChainApi.md#get_key_block_by_height) | **Get** /key-blocks/height/{height} | 
*ChainApi* | [**get_micro_block_header_by_hash**](docs/ChainApi.md#get_micro_block_header_by_hash) | **Get** /micro-blocks/hash/{hash}/header | 
*ChainApi* | [**get_micro_block_transaction_by_hash_and_index**](docs/ChainApi.md#get_micro_block_transaction_by_hash_and_index) | **Get** /micro-blocks/hash/{hash}/transactions/index/{index} | 
*ChainApi* | [**get_micro_block_transactions_by_hash**](docs/ChainApi.md#get_micro_block_transactions_by_hash) | **Get** /micro-blocks/hash/{hash}/transactions | 
*ChainApi* | [**get_micro_block_transactions_count_by_hash**](docs/ChainApi.md#get_micro_block_transactions_count_by_hash) | **Get** /micro-blocks/hash/{hash}/transactions/count | 
*ChainApi* | [**get_pending_key_block**](docs/ChainApi.md#get_pending_key_block) | **Get** /key-blocks/pending | 
*ChainApi* | [**get_top_block**](docs/ChainApi.md#get_top_block) | **Get** /blocks/top | 
*ChainApi* | [**post_key_block**](docs/ChainApi.md#post_key_block) | **Post** /key-blocks | 
*ChannelApi* | [**get_channel_by_pubkey**](docs/ChannelApi.md#get_channel_by_pubkey) | **Get** /channels/{pubkey} | 
*ChannelApi* | [**post_channel_close_mutual**](docs/ChannelApi.md#post_channel_close_mutual) | **Post** /debug/channels/close/mutual | 
*ChannelApi* | [**post_channel_close_solo**](docs/ChannelApi.md#post_channel_close_solo) | **Post** /debug/channels/close/solo | 
*ChannelApi* | [**post_channel_create**](docs/ChannelApi.md#post_channel_create) | **Post** /debug/channels/create | 
*ChannelApi* | [**post_channel_deposit**](docs/ChannelApi.md#post_channel_deposit) | **Post** /debug/channels/deposit | 
*ChannelApi* | [**post_channel_settle**](docs/ChannelApi.md#post_channel_settle) | **Post** /debug/channels/settle | 
*ChannelApi* | [**post_channel_slash**](docs/ChannelApi.md#post_channel_slash) | **Post** /debug/channels/slash | 
*ChannelApi* | [**post_channel_snapshot_solo**](docs/ChannelApi.md#post_channel_snapshot_solo) | **Post** /debug/channels/snapshot/solo | 
*ChannelApi* | [**post_channel_withdraw**](docs/ChannelApi.md#post_channel_withdraw) | **Post** /debug/channels/withdraw | 
*ContractApi* | [**call_contract**](docs/ContractApi.md#call_contract) | **Post** /debug/contracts/code/call | 
*ContractApi* | [**compile_contract**](docs/ContractApi.md#compile_contract) | **Post** /debug/contracts/code/compile | 
*ContractApi* | [**decode_data**](docs/ContractApi.md#decode_data) | **Post** /debug/contracts/code/decode-data | 
*ContractApi* | [**encode_calldata**](docs/ContractApi.md#encode_calldata) | **Post** /debug/contracts/code/encode-calldata | 
*ContractApi* | [**get_contract**](docs/ContractApi.md#get_contract) | **Get** /contracts/{pubkey} | 
*ContractApi* | [**get_contract_code**](docs/ContractApi.md#get_contract_code) | **Get** /contracts/{pubkey}/code | 
*ContractApi* | [**get_contract_po_i**](docs/ContractApi.md#get_contract_po_i) | **Get** /contracts/{pubkey}/poi | 
*ContractApi* | [**get_contract_store**](docs/ContractApi.md#get_contract_store) | **Get** /contracts/{pubkey}/store | 
*ContractApi* | [**post_contract_call**](docs/ContractApi.md#post_contract_call) | **Post** /debug/contracts/call | 
*ContractApi* | [**post_contract_call_compute**](docs/ContractApi.md#post_contract_call_compute) | **Post** /debug/contracts/call/compute | 
*ContractApi* | [**post_contract_create**](docs/ContractApi.md#post_contract_create) | **Post** /debug/contracts/create | 
*ContractApi* | [**post_contract_create_compute**](docs/ContractApi.md#post_contract_create_compute) | **Post** /debug/contracts/create/compute | 
*DebugApi* | [**call_contract**](docs/DebugApi.md#call_contract) | **Post** /debug/contracts/code/call | 
*DebugApi* | [**compile_contract**](docs/DebugApi.md#compile_contract) | **Post** /debug/contracts/code/compile | 
*DebugApi* | [**decode_data**](docs/DebugApi.md#decode_data) | **Post** /debug/contracts/code/decode-data | 
*DebugApi* | [**encode_calldata**](docs/DebugApi.md#encode_calldata) | **Post** /debug/contracts/code/encode-calldata | 
*DebugApi* | [**get_commitment_id**](docs/DebugApi.md#get_commitment_id) | **Get** /debug/names/commitment-id | 
*DebugApi* | [**get_node_beneficiary**](docs/DebugApi.md#get_node_beneficiary) | **Get** /debug/accounts/beneficiary | 
*DebugApi* | [**get_node_pubkey**](docs/DebugApi.md#get_node_pubkey) | **Get** /debug/accounts/node | 
*DebugApi* | [**get_peers**](docs/DebugApi.md#get_peers) | **Get** /debug/peers | 
*DebugApi* | [**get_pending_transactions**](docs/DebugApi.md#get_pending_transactions) | **Get** /debug/transactions/pending | 
*DebugApi* | [**post_channel_close_mutual**](docs/DebugApi.md#post_channel_close_mutual) | **Post** /debug/channels/close/mutual | 
*DebugApi* | [**post_channel_close_solo**](docs/DebugApi.md#post_channel_close_solo) | **Post** /debug/channels/close/solo | 
*DebugApi* | [**post_channel_create**](docs/DebugApi.md#post_channel_create) | **Post** /debug/channels/create | 
*DebugApi* | [**post_channel_deposit**](docs/DebugApi.md#post_channel_deposit) | **Post** /debug/channels/deposit | 
*DebugApi* | [**post_channel_settle**](docs/DebugApi.md#post_channel_settle) | **Post** /debug/channels/settle | 
*DebugApi* | [**post_channel_slash**](docs/DebugApi.md#post_channel_slash) | **Post** /debug/channels/slash | 
*DebugApi* | [**post_channel_snapshot_solo**](docs/DebugApi.md#post_channel_snapshot_solo) | **Post** /debug/channels/snapshot/solo | 
*DebugApi* | [**post_channel_withdraw**](docs/DebugApi.md#post_channel_withdraw) | **Post** /debug/channels/withdraw | 
*DebugApi* | [**post_contract_call**](docs/DebugApi.md#post_contract_call) | **Post** /debug/contracts/call | 
*DebugApi* | [**post_contract_call_compute**](docs/DebugApi.md#post_contract_call_compute) | **Post** /debug/contracts/call/compute | 
*DebugApi* | [**post_contract_create**](docs/DebugApi.md#post_contract_create) | **Post** /debug/contracts/create | 
*DebugApi* | [**post_contract_create_compute**](docs/DebugApi.md#post_contract_create_compute) | **Post** /debug/contracts/create/compute | 
*DebugApi* | [**post_name_claim**](docs/DebugApi.md#post_name_claim) | **Post** /debug/names/claim | 
*DebugApi* | [**post_name_preclaim**](docs/DebugApi.md#post_name_preclaim) | **Post** /debug/names/preclaim | 
*DebugApi* | [**post_name_revoke**](docs/DebugApi.md#post_name_revoke) | **Post** /debug/names/revoke | 
*DebugApi* | [**post_name_transfer**](docs/DebugApi.md#post_name_transfer) | **Post** /debug/names/transfer | 
*DebugApi* | [**post_name_update**](docs/DebugApi.md#post_name_update) | **Post** /debug/names/update | 
*DebugApi* | [**post_oracle_extend**](docs/DebugApi.md#post_oracle_extend) | **Post** /debug/oracles/extend | 
*DebugApi* | [**post_oracle_query**](docs/DebugApi.md#post_oracle_query) | **Post** /debug/oracles/query | 
*DebugApi* | [**post_oracle_register**](docs/DebugApi.md#post_oracle_register) | **Post** /debug/oracles/register | 
*DebugApi* | [**post_oracle_respond**](docs/DebugApi.md#post_oracle_respond) | **Post** /debug/oracles/respond | 
*DebugApi* | [**post_spend**](docs/DebugApi.md#post_spend) | **Post** /debug/transactions/spend | 
*ExternalApi* | [**get_account_by_pubkey**](docs/ExternalApi.md#get_account_by_pubkey) | **Get** /accounts/{pubkey} | 
*ExternalApi* | [**get_channel_by_pubkey**](docs/ExternalApi.md#get_channel_by_pubkey) | **Get** /channels/{pubkey} | 
*ExternalApi* | [**get_contract**](docs/ExternalApi.md#get_contract) | **Get** /contracts/{pubkey} | 
*ExternalApi* | [**get_contract_code**](docs/ExternalApi.md#get_contract_code) | **Get** /contracts/{pubkey}/code | 
*ExternalApi* | [**get_contract_po_i**](docs/ExternalApi.md#get_contract_po_i) | **Get** /contracts/{pubkey}/poi | 
*ExternalApi* | [**get_contract_store**](docs/ExternalApi.md#get_contract_store) | **Get** /contracts/{pubkey}/store | 
*ExternalApi* | [**get_current_generation**](docs/ExternalApi.md#get_current_generation) | **Get** /generations/current | 
*ExternalApi* | [**get_current_key_block**](docs/ExternalApi.md#get_current_key_block) | **Get** /key-blocks/current | 
*ExternalApi* | [**get_current_key_block_hash**](docs/ExternalApi.md#get_current_key_block_hash) | **Get** /key-blocks/current/hash | 
*ExternalApi* | [**get_current_key_block_height**](docs/ExternalApi.md#get_current_key_block_height) | **Get** /key-blocks/current/height | 
*ExternalApi* | [**get_generation_by_hash**](docs/ExternalApi.md#get_generation_by_hash) | **Get** /generations/hash/{hash} | 
*ExternalApi* | [**get_generation_by_height**](docs/ExternalApi.md#get_generation_by_height) | **Get** /generations/height/{height} | 
*ExternalApi* | [**get_key_block_by_hash**](docs/ExternalApi.md#get_key_block_by_hash) | **Get** /key-blocks/hash/{hash} | 
*ExternalApi* | [**get_key_block_by_height**](docs/ExternalApi.md#get_key_block_by_height) | **Get** /key-blocks/height/{height} | 
*ExternalApi* | [**get_micro_block_header_by_hash**](docs/ExternalApi.md#get_micro_block_header_by_hash) | **Get** /micro-blocks/hash/{hash}/header | 
*ExternalApi* | [**get_micro_block_transaction_by_hash_and_index**](docs/ExternalApi.md#get_micro_block_transaction_by_hash_and_index) | **Get** /micro-blocks/hash/{hash}/transactions/index/{index} | 
*ExternalApi* | [**get_micro_block_transactions_by_hash**](docs/ExternalApi.md#get_micro_block_transactions_by_hash) | **Get** /micro-blocks/hash/{hash}/transactions | 
*ExternalApi* | [**get_micro_block_transactions_count_by_hash**](docs/ExternalApi.md#get_micro_block_transactions_count_by_hash) | **Get** /micro-blocks/hash/{hash}/transactions/count | 
*ExternalApi* | [**get_name_entry_by_name**](docs/ExternalApi.md#get_name_entry_by_name) | **Get** /names/{name} | 
*ExternalApi* | [**get_oracle_by_pubkey**](docs/ExternalApi.md#get_oracle_by_pubkey) | **Get** /oracles/{pubkey} | 
*ExternalApi* | [**get_oracle_queries_by_pubkey**](docs/ExternalApi.md#get_oracle_queries_by_pubkey) | **Get** /oracles/{pubkey}/queries | 
*ExternalApi* | [**get_oracle_query_by_pubkey_and_query_id**](docs/ExternalApi.md#get_oracle_query_by_pubkey_and_query_id) | **Get** /oracles/{pubkey}/queries/{query-id} | 
*ExternalApi* | [**get_peer_pubkey**](docs/ExternalApi.md#get_peer_pubkey) | **Get** /peers/pubkey | 
*ExternalApi* | [**get_pending_account_transactions_by_pubkey**](docs/ExternalApi.md#get_pending_account_transactions_by_pubkey) | **Get** /accounts/{pubkey}/transactions/pending | 
*ExternalApi* | [**get_pending_key_block**](docs/ExternalApi.md#get_pending_key_block) | **Get** /key-blocks/pending | 
*ExternalApi* | [**get_status**](docs/ExternalApi.md#get_status) | **Get** /status | 
*ExternalApi* | [**get_top_block**](docs/ExternalApi.md#get_top_block) | **Get** /blocks/top | 
*ExternalApi* | [**get_transaction_by_hash**](docs/ExternalApi.md#get_transaction_by_hash) | **Get** /transactions/{hash} | 
*ExternalApi* | [**get_transaction_info_by_hash**](docs/ExternalApi.md#get_transaction_info_by_hash) | **Get** /transactions/{hash}/info | 
*ExternalApi* | [**post_transaction**](docs/ExternalApi.md#post_transaction) | **Post** /transactions | 
*InternalApi* | [**call_contract**](docs/InternalApi.md#call_contract) | **Post** /debug/contracts/code/call | 
*InternalApi* | [**compile_contract**](docs/InternalApi.md#compile_contract) | **Post** /debug/contracts/code/compile | 
*InternalApi* | [**decode_data**](docs/InternalApi.md#decode_data) | **Post** /debug/contracts/code/decode-data | 
*InternalApi* | [**encode_calldata**](docs/InternalApi.md#encode_calldata) | **Post** /debug/contracts/code/encode-calldata | 
*InternalApi* | [**get_commitment_id**](docs/InternalApi.md#get_commitment_id) | **Get** /debug/names/commitment-id | 
*InternalApi* | [**get_node_beneficiary**](docs/InternalApi.md#get_node_beneficiary) | **Get** /debug/accounts/beneficiary | 
*InternalApi* | [**get_node_pubkey**](docs/InternalApi.md#get_node_pubkey) | **Get** /debug/accounts/node | 
*InternalApi* | [**get_peers**](docs/InternalApi.md#get_peers) | **Get** /debug/peers | 
*InternalApi* | [**get_pending_transactions**](docs/InternalApi.md#get_pending_transactions) | **Get** /debug/transactions/pending | 
*InternalApi* | [**post_channel_close_mutual**](docs/InternalApi.md#post_channel_close_mutual) | **Post** /debug/channels/close/mutual | 
*InternalApi* | [**post_channel_close_solo**](docs/InternalApi.md#post_channel_close_solo) | **Post** /debug/channels/close/solo | 
*InternalApi* | [**post_channel_create**](docs/InternalApi.md#post_channel_create) | **Post** /debug/channels/create | 
*InternalApi* | [**post_channel_deposit**](docs/InternalApi.md#post_channel_deposit) | **Post** /debug/channels/deposit | 
*InternalApi* | [**post_channel_settle**](docs/InternalApi.md#post_channel_settle) | **Post** /debug/channels/settle | 
*InternalApi* | [**post_channel_slash**](docs/InternalApi.md#post_channel_slash) | **Post** /debug/channels/slash | 
*InternalApi* | [**post_channel_snapshot_solo**](docs/InternalApi.md#post_channel_snapshot_solo) | **Post** /debug/channels/snapshot/solo | 
*InternalApi* | [**post_channel_withdraw**](docs/InternalApi.md#post_channel_withdraw) | **Post** /debug/channels/withdraw | 
*InternalApi* | [**post_contract_call**](docs/InternalApi.md#post_contract_call) | **Post** /debug/contracts/call | 
*InternalApi* | [**post_contract_call_compute**](docs/InternalApi.md#post_contract_call_compute) | **Post** /debug/contracts/call/compute | 
*InternalApi* | [**post_contract_create**](docs/InternalApi.md#post_contract_create) | **Post** /debug/contracts/create | 
*InternalApi* | [**post_contract_create_compute**](docs/InternalApi.md#post_contract_create_compute) | **Post** /debug/contracts/create/compute | 
*InternalApi* | [**post_key_block**](docs/InternalApi.md#post_key_block) | **Post** /key-blocks | 
*InternalApi* | [**post_name_claim**](docs/InternalApi.md#post_name_claim) | **Post** /debug/names/claim | 
*InternalApi* | [**post_name_preclaim**](docs/InternalApi.md#post_name_preclaim) | **Post** /debug/names/preclaim | 
*InternalApi* | [**post_name_revoke**](docs/InternalApi.md#post_name_revoke) | **Post** /debug/names/revoke | 
*InternalApi* | [**post_name_transfer**](docs/InternalApi.md#post_name_transfer) | **Post** /debug/names/transfer | 
*InternalApi* | [**post_name_update**](docs/InternalApi.md#post_name_update) | **Post** /debug/names/update | 
*InternalApi* | [**post_oracle_extend**](docs/InternalApi.md#post_oracle_extend) | **Post** /debug/oracles/extend | 
*InternalApi* | [**post_oracle_query**](docs/InternalApi.md#post_oracle_query) | **Post** /debug/oracles/query | 
*InternalApi* | [**post_oracle_register**](docs/InternalApi.md#post_oracle_register) | **Post** /debug/oracles/register | 
*InternalApi* | [**post_oracle_respond**](docs/InternalApi.md#post_oracle_respond) | **Post** /debug/oracles/respond | 
*InternalApi* | [**post_spend**](docs/InternalApi.md#post_spend) | **Post** /debug/transactions/spend | 
*NameServiceApi* | [**get_commitment_id**](docs/NameServiceApi.md#get_commitment_id) | **Get** /debug/names/commitment-id | 
*NameServiceApi* | [**get_name_entry_by_name**](docs/NameServiceApi.md#get_name_entry_by_name) | **Get** /names/{name} | 
*NameServiceApi* | [**post_name_claim**](docs/NameServiceApi.md#post_name_claim) | **Post** /debug/names/claim | 
*NameServiceApi* | [**post_name_preclaim**](docs/NameServiceApi.md#post_name_preclaim) | **Post** /debug/names/preclaim | 
*NameServiceApi* | [**post_name_revoke**](docs/NameServiceApi.md#post_name_revoke) | **Post** /debug/names/revoke | 
*NameServiceApi* | [**post_name_transfer**](docs/NameServiceApi.md#post_name_transfer) | **Post** /debug/names/transfer | 
*NameServiceApi* | [**post_name_update**](docs/NameServiceApi.md#post_name_update) | **Post** /debug/names/update | 
*NodeInfoApi* | [**get_node_beneficiary**](docs/NodeInfoApi.md#get_node_beneficiary) | **Get** /debug/accounts/beneficiary | 
*NodeInfoApi* | [**get_node_pubkey**](docs/NodeInfoApi.md#get_node_pubkey) | **Get** /debug/accounts/node | 
*NodeInfoApi* | [**get_peer_pubkey**](docs/NodeInfoApi.md#get_peer_pubkey) | **Get** /peers/pubkey | 
*NodeInfoApi* | [**get_peers**](docs/NodeInfoApi.md#get_peers) | **Get** /debug/peers | 
*NodeInfoApi* | [**get_status**](docs/NodeInfoApi.md#get_status) | **Get** /status | 
*OracleApi* | [**get_oracle_by_pubkey**](docs/OracleApi.md#get_oracle_by_pubkey) | **Get** /oracles/{pubkey} | 
*OracleApi* | [**get_oracle_queries_by_pubkey**](docs/OracleApi.md#get_oracle_queries_by_pubkey) | **Get** /oracles/{pubkey}/queries | 
*OracleApi* | [**get_oracle_query_by_pubkey_and_query_id**](docs/OracleApi.md#get_oracle_query_by_pubkey_and_query_id) | **Get** /oracles/{pubkey}/queries/{query-id} | 
*OracleApi* | [**post_oracle_extend**](docs/OracleApi.md#post_oracle_extend) | **Post** /debug/oracles/extend | 
*OracleApi* | [**post_oracle_query**](docs/OracleApi.md#post_oracle_query) | **Post** /debug/oracles/query | 
*OracleApi* | [**post_oracle_register**](docs/OracleApi.md#post_oracle_register) | **Post** /debug/oracles/register | 
*OracleApi* | [**post_oracle_respond**](docs/OracleApi.md#post_oracle_respond) | **Post** /debug/oracles/respond | 
*TransactionApi* | [**get_pending_transactions**](docs/TransactionApi.md#get_pending_transactions) | **Get** /debug/transactions/pending | 
*TransactionApi* | [**get_transaction_by_hash**](docs/TransactionApi.md#get_transaction_by_hash) | **Get** /transactions/{hash} | 
*TransactionApi* | [**get_transaction_info_by_hash**](docs/TransactionApi.md#get_transaction_info_by_hash) | **Get** /transactions/{hash}/info | 
*TransactionApi* | [**post_spend**](docs/TransactionApi.md#post_spend) | **Post** /debug/transactions/spend | 
*TransactionApi* | [**post_transaction**](docs/TransactionApi.md#post_transaction) | **Post** /transactions | 


## Documentation For Models

 - [Account](docs/Account.md)
 - [ByteCode](docs/ByteCode.md)
 - [CallResult](docs/CallResult.md)
 - [Calldata](docs/Calldata.md)
 - [Channel](docs/Channel.md)
 - [ChannelCloseMutualTx](docs/ChannelCloseMutualTx.md)
 - [ChannelCloseSoloTx](docs/ChannelCloseSoloTx.md)
 - [ChannelCreateTx](docs/ChannelCreateTx.md)
 - [ChannelDepositTx](docs/ChannelDepositTx.md)
 - [ChannelForceProgressTx](docs/ChannelForceProgressTx.md)
 - [ChannelSettleTx](docs/ChannelSettleTx.md)
 - [ChannelSlashTx](docs/ChannelSlashTx.md)
 - [ChannelSnapshotSoloTx](docs/ChannelSnapshotSoloTx.md)
 - [ChannelWithdrawTx](docs/ChannelWithdrawTx.md)
 - [CommitmentId](docs/CommitmentId.md)
 - [Contract](docs/Contract.md)
 - [ContractCallCompute](docs/ContractCallCompute.md)
 - [ContractCallInput](docs/ContractCallInput.md)
 - [ContractCallObject](docs/ContractCallObject.md)
 - [ContractCallTx](docs/ContractCallTx.md)
 - [ContractCreateCompute](docs/ContractCreateCompute.md)
 - [ContractCreateTx](docs/ContractCreateTx.md)
 - [ContractObject](docs/ContractObject.md)
 - [ContractStore](docs/ContractStore.md)
 - [ContractStoreStore](docs/ContractStoreStore.md)
 - [EncodedByteArray](docs/EncodedByteArray.md)
 - [EncodedHash](docs/EncodedHash.md)
 - [Error](docs/Error.md)
 - [Generation](docs/Generation.md)
 - [GenericSignedTx](docs/GenericSignedTx.md)
 - [GenericTx](docs/GenericTx.md)
 - [GenericTxs](docs/GenericTxs.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponse2001](docs/InlineResponse2001.md)
 - [InlineResponse2002](docs/InlineResponse2002.md)
 - [InlineResponse2003](docs/InlineResponse2003.md)
 - [KeyBlock](docs/KeyBlock.md)
 - [KeyBlockOrMicroBlockHeader](docs/KeyBlockOrMicroBlockHeader.md)
 - [MicroBlockHeader](docs/MicroBlockHeader.md)
 - [NameClaimTx](docs/NameClaimTx.md)
 - [NameEntry](docs/NameEntry.md)
 - [NameHash](docs/NameHash.md)
 - [NamePointer](docs/NamePointer.md)
 - [NamePreclaimTx](docs/NamePreclaimTx.md)
 - [NameRevokeTx](docs/NameRevokeTx.md)
 - [NameTransferTx](docs/NameTransferTx.md)
 - [NameUpdateTx](docs/NameUpdateTx.md)
 - [OffChainUpdate](docs/OffChainUpdate.md)
 - [OracleExtendTx](docs/OracleExtendTx.md)
 - [OracleQueries](docs/OracleQueries.md)
 - [OracleQuery](docs/OracleQuery.md)
 - [OracleQueryTx](docs/OracleQueryTx.md)
 - [OracleRegisterTx](docs/OracleRegisterTx.md)
 - [OracleRespondTx](docs/OracleRespondTx.md)
 - [Peer](docs/Peer.md)
 - [Peers](docs/Peers.md)
 - [PoI](docs/PoI.md)
 - [PostTxResponse](docs/PostTxResponse.md)
 - [Pow](docs/Pow.md)
 - [Protocol](docs/Protocol.md)
 - [PubKey](docs/PubKey.md)
 - [RegisteredOracle](docs/RegisteredOracle.md)
 - [RelativeTtl](docs/RelativeTtl.md)
 - [SophiaBinaryData](docs/SophiaBinaryData.md)
 - [SophiaJsonData](docs/SophiaJsonData.md)
 - [SpendTx](docs/SpendTx.md)
 - [Status](docs/Status.md)
 - [Ttl](docs/Ttl.md)
 - [Tx](docs/Tx.md)
 - [UnsignedTx](docs/UnsignedTx.md)
 - [Uri](docs/Uri.md)
 - [ChannelCloseMutualTxJson](docs/ChannelCloseMutualTxJson.md)
 - [ChannelCloseSoloTxJson](docs/ChannelCloseSoloTxJson.md)
 - [ChannelCreateTxJson](docs/ChannelCreateTxJson.md)
 - [ChannelDepositTxJson](docs/ChannelDepositTxJson.md)
 - [ChannelForceProgressTxJson](docs/ChannelForceProgressTxJson.md)
 - [ChannelSettleTxJson](docs/ChannelSettleTxJson.md)
 - [ChannelSlashTxJson](docs/ChannelSlashTxJson.md)
 - [ChannelSnapshotSoloTxJson](docs/ChannelSnapshotSoloTxJson.md)
 - [ChannelWithdrawalTxJson](docs/ChannelWithdrawalTxJson.md)
 - [ContractCallTxJson](docs/ContractCallTxJson.md)
 - [ContractCreateTxJson](docs/ContractCreateTxJson.md)
 - [CreateContractUnsignedTx](docs/CreateContractUnsignedTx.md)
 - [NameClaimTxJson](docs/NameClaimTxJson.md)
 - [NamePreclaimTxJson](docs/NamePreclaimTxJson.md)
 - [NameRevokeTxJson](docs/NameRevokeTxJson.md)
 - [NameTransferTxJson](docs/NameTransferTxJson.md)
 - [NameUpdateTxJson](docs/NameUpdateTxJson.md)
 - [OffChainCallContract](docs/OffChainCallContract.md)
 - [OffChainDeposit](docs/OffChainDeposit.md)
 - [OffChainNewContract](docs/OffChainNewContract.md)
 - [OffChainTransfer](docs/OffChainTransfer.md)
 - [OffChainWithdrawal](docs/OffChainWithdrawal.md)
 - [OracleExtendTxJson](docs/OracleExtendTxJson.md)
 - [OracleQueryTxJson](docs/OracleQueryTxJson.md)
 - [OracleRegisterTxJson](docs/OracleRegisterTxJson.md)
 - [OracleResponseTxJson](docs/OracleResponseTxJson.md)
 - [SpendTxJson](docs/SpendTxJson.md)


## Documentation For Authorization
 Endpoints do not require authorization.


## Author

arjan@vaneersel.me
