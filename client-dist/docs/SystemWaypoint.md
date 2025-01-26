# SystemWaypoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | The symbol of the waypoint. | 
**r#type** | [**models::WaypointType**](WaypointType.md) |  | 
**x** | **i32** | Relative position of the waypoint on the system's x axis. This is not an absolute position in the universe. | 
**y** | **i32** | Relative position of the waypoint on the system's y axis. This is not an absolute position in the universe. | 
**orbitals** | [**Vec<models::WaypointOrbital>**](WaypointOrbital.md) | Waypoints that orbit this waypoint. | 
**orbits** | Option<**String**> | The symbol of the parent waypoint, if this waypoint is in orbit around another waypoint. Otherwise this value is undefined. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


