# ScannedShip

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | The globally unique identifier of the ship. | 
**registration** | [**models::ShipRegistration**](ShipRegistration.md) |  | 
**nav** | [**models::ShipNav**](ShipNav.md) |  | 
**frame** | Option<[**models::ScannedShipFrame**](ScannedShip_frame.md)> |  | [optional]
**reactor** | Option<[**models::ScannedShipReactor**](ScannedShip_reactor.md)> |  | [optional]
**engine** | [**models::ScannedShipEngine**](ScannedShip_engine.md) |  | 
**mounts** | Option<[**Vec<models::ScannedShipMountsInner>**](ScannedShip_mounts_inner.md)> | List of mounts installed in the ship. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


