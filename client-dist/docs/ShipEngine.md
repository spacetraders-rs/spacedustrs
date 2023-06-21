# ShipEngine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | The symbol of the engine. | 
**name** | **String** | The name of the engine. | 
**description** | **String** | The description of the engine. | 
**condition** | Option<**i32**> | Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new. | [optional]
**speed** | **i32** | The speed stat of this engine. The higher the speed, the faster a ship can travel from one point to another. Reduces the time of arrival when navigating the ship. | 
**requirements** | [**crate::models::ShipRequirements**](ShipRequirements.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


