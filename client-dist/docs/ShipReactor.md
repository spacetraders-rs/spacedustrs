# ShipReactor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | Symbol of the reactor. | 
**name** | **String** | Name of the reactor. | 
**description** | **String** | Description of the reactor. | 
**condition** | Option<**i32**> | Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new. | [optional]
**power_output** | **i32** | The amount of power provided by this reactor. The more power a reactor provides to the ship, the lower the cooldown it gets when using a module or mount that taxes the ship's power. | 
**requirements** | [**crate::models::ShipRequirements**](ShipRequirements.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


