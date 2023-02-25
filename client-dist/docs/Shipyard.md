# Shipyard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | The symbol of the shipyard. The symbol is the same as the waypoint where the shipyard is located. | 
**ship_types** | [**Vec<crate::models::ShipyardShipTypesInner>**](Shipyard_shipTypes_inner.md) | The list of ship types available for purchase at this shipyard. | 
**transactions** | Option<[**Vec<crate::models::ShipyardTransaction>**](ShipyardTransaction.md)> | The list of recent transactions at this shipyard. | [optional]
**ships** | Option<[**Vec<crate::models::ShipyardShip>**](ShipyardShip.md)> | The ships that are currently available for purchase at the shipyard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


