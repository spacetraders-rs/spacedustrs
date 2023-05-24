# \SystemsApi

All URIs are relative to *https://api.spacetraders.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_jump_gate**](SystemsApi.md#get_jump_gate) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/jump-gate | Get Jump Gate
[**get_market**](SystemsApi.md#get_market) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/market | Get Market
[**get_shipyard**](SystemsApi.md#get_shipyard) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/shipyard | Get Shipyard
[**get_system**](SystemsApi.md#get_system) | **GET** /systems/{systemSymbol} | Get System
[**get_system_waypoints**](SystemsApi.md#get_system_waypoints) | **GET** /systems/{systemSymbol}/waypoints | List Waypoints
[**get_systems**](SystemsApi.md#get_systems) | **GET** /systems | List Systems
[**get_systems_all**](SystemsApi.md#get_systems_all) | **GET** /systems.json | Get All Systems
[**get_waypoint**](SystemsApi.md#get_waypoint) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol} | Get Waypoint



## get_jump_gate

> crate::models::GetJumpGate200Response get_jump_gate(system_symbol, waypoint_symbol)
Get Jump Gate

Get jump gate details for a waypoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_symbol** | **String** | The system symbol | [required] |
**waypoint_symbol** | **String** | The waypoint symbol | [required] |

### Return type

[**crate::models::GetJumpGate200Response**](get_jump_gate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market

> crate::models::GetMarket200Response get_market(system_symbol, waypoint_symbol)
Get Market

Retrieve imports, exports and exchange data from a marketplace. Imports can be sold, exports can be purchased, and exchange goods can be purchased or sold. Send a ship to the waypoint to access trade good prices and recent transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_symbol** | **String** | The system symbol | [required] |
**waypoint_symbol** | **String** | The waypoint symbol | [required] |

### Return type

[**crate::models::GetMarket200Response**](get_market_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipyard

> crate::models::GetShipyard200Response get_shipyard(system_symbol, waypoint_symbol)
Get Shipyard

Get the shipyard for a waypoint. Send a ship to the waypoint to access ships that are currently available for purchase and recent transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_symbol** | **String** | The system symbol | [required] |
**waypoint_symbol** | **String** | The waypoint symbol | [required] |

### Return type

[**crate::models::GetShipyard200Response**](get_shipyard_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system

> crate::models::GetSystem200Response get_system(system_symbol)
Get System

Get the details of a system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_symbol** | **String** | The system symbol | [required] |[default to X1-OE]

### Return type

[**crate::models::GetSystem200Response**](get_system_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_waypoints

> crate::models::GetSystemWaypoints200Response get_system_waypoints(system_symbol, page, limit)
List Waypoints

Fetch all of the waypoints for a given system. System must be charted or a ship must be present to return waypoint details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_symbol** | **String** | The system symbol | [required] |
**page** | Option<**i32**> | What entry offset to request |  |
**limit** | Option<**i32**> | How many entries to return per page |  |

### Return type

[**crate::models::GetSystemWaypoints200Response**](get_system_waypoints_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_systems

> crate::models::GetSystems200Response get_systems(page, limit)
List Systems

Return a list of all systems.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | What entry offset to request |  |
**limit** | Option<**i32**> | How many entries to return per page |  |

### Return type

[**crate::models::GetSystems200Response**](get_systems_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_systems_all

> Vec<crate::models::System> get_systems_all()
Get All Systems

Return a json file containing all systems

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::System>**](System.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_waypoint

> crate::models::GetWaypoint200Response get_waypoint(system_symbol, waypoint_symbol)
Get Waypoint

View the details of a waypoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_symbol** | **String** | The system symbol | [required] |
**waypoint_symbol** | **String** | The waypoint symbol | [required] |

### Return type

[**crate::models::GetWaypoint200Response**](get_waypoint_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

