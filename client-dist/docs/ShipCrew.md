# ShipCrew

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current** | **i32** | The current number of crew members on the ship. | 
**required** | **i32** | The minimum number of crew members required to maintain the ship. | 
**capacity** | **i32** | The maximum number of crew members the ship can support. | 
**rotation** | **String** | The rotation of crew shifts. A stricter shift improves the ship's performance. A more relaxed shift improves the crew's morale. | [default to Strict]
**morale** | **i32** | A rough measure of the crew's morale. A higher morale means the crew is happier and more productive. A lower morale means the ship is more prone to accidents. | 
**wages** | **i32** | The amount of credits per crew member paid per hour. Wages are paid when a ship docks at a civilized waypoint. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


