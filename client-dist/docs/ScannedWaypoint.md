# ScannedWaypoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | Symbol of the waypoint. | 
**r#type** | [**crate::models::WaypointType**](WaypointType.md) |  | 
**system_symbol** | **String** | Symbol of the system. | 
**x** | **i32** | Position in the universe in the x axis. | 
**y** | **i32** | Position in the universe in the y axis. | 
**orbitals** | [**Vec<crate::models::WaypointOrbital>**](WaypointOrbital.md) | List of waypoints that orbit this waypoint. | 
**faction** | Option<[**crate::models::WaypointFaction**](WaypointFaction.md)> |  | [optional]
**traits** | [**Vec<crate::models::WaypointTrait>**](WaypointTrait.md) | The traits of the waypoint. | 
**chart** | Option<[**crate::models::Chart**](Chart.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


