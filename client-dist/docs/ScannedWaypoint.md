# ScannedWaypoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | The symbol of the waypoint. | 
**r#type** | [**models::WaypointType**](WaypointType.md) |  | 
**system_symbol** | **String** | The symbol of the system. | 
**x** | **i32** | Position in the universe in the x axis. | 
**y** | **i32** | Position in the universe in the y axis. | 
**orbitals** | [**Vec<models::WaypointOrbital>**](WaypointOrbital.md) | List of waypoints that orbit this waypoint. | 
**faction** | Option<[**models::WaypointFaction**](WaypointFaction.md)> |  | [optional]
**traits** | [**Vec<models::WaypointTrait>**](WaypointTrait.md) | The traits of the waypoint. | 
**chart** | Option<[**models::Chart**](Chart.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


