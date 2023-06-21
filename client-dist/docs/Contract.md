# Contract

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the contract. | 
**faction_symbol** | **String** | The symbol of the faction that this contract is for. | 
**r#type** | **String** | Type of contract. | 
**terms** | [**crate::models::ContractTerms**](ContractTerms.md) |  | 
**accepted** | **bool** | Whether the contract has been accepted by the agent | [default to false]
**fulfilled** | **bool** | Whether the contract has been fulfilled | [default to false]
**expiration** | **String** | Deprecated in favor of deadlineToAccept | 
**deadline_to_accept** | Option<**String**> | The time at which the contract is no longer available to be accepted | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


