# \FleetApi

All URIs are relative to *https://api.spacetraders.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_chart**](FleetApi.md#create_chart) | **POST** /my/ships/{shipSymbol}/chart | Create Chart
[**create_ship_ship_scan**](FleetApi.md#create_ship_ship_scan) | **POST** /my/ships/{shipSymbol}/scan/ships | Scan Ships
[**create_ship_system_scan**](FleetApi.md#create_ship_system_scan) | **POST** /my/ships/{shipSymbol}/scan/systems | Scan Systems
[**create_ship_waypoint_scan**](FleetApi.md#create_ship_waypoint_scan) | **POST** /my/ships/{shipSymbol}/scan/waypoints | Scan Waypoints
[**create_survey**](FleetApi.md#create_survey) | **POST** /my/ships/{shipSymbol}/survey | Create Survey
[**dock_ship**](FleetApi.md#dock_ship) | **POST** /my/ships/{shipSymbol}/dock | Dock Ship
[**extract_resources**](FleetApi.md#extract_resources) | **POST** /my/ships/{shipSymbol}/extract | Extract Resources
[**get_my_ship**](FleetApi.md#get_my_ship) | **GET** /my/ships/{shipSymbol} | Get Ship
[**get_my_ship_cargo**](FleetApi.md#get_my_ship_cargo) | **GET** /my/ships/{shipSymbol}/cargo | Get Ship Cargo
[**get_my_ships**](FleetApi.md#get_my_ships) | **GET** /my/ships | List Ships
[**get_ship_cooldown**](FleetApi.md#get_ship_cooldown) | **GET** /my/ships/{shipSymbol}/cooldown | Get Ship Cooldown
[**get_ship_nav**](FleetApi.md#get_ship_nav) | **GET** /my/ships/{shipSymbol}/nav | Get Ship Nav
[**jettison**](FleetApi.md#jettison) | **POST** /my/ships/{shipSymbol}/jettison | Jettison Cargo
[**jump_ship**](FleetApi.md#jump_ship) | **POST** /my/ships/{shipSymbol}/jump | Jump Ship
[**navigate_ship**](FleetApi.md#navigate_ship) | **POST** /my/ships/{shipSymbol}/navigate | Navigate Ship
[**negotiate_contract**](FleetApi.md#negotiate_contract) | **POST** /my/ships/{shipSymbol}/negotiate/contract | Negotiate Contract
[**orbit_ship**](FleetApi.md#orbit_ship) | **POST** /my/ships/{shipSymbol}/orbit | Orbit Ship
[**patch_ship_nav**](FleetApi.md#patch_ship_nav) | **PATCH** /my/ships/{shipSymbol}/nav | Patch Ship Nav
[**purchase_cargo**](FleetApi.md#purchase_cargo) | **POST** /my/ships/{shipSymbol}/purchase | Purchase Cargo
[**purchase_ship**](FleetApi.md#purchase_ship) | **POST** /my/ships | Purchase Ship
[**refuel_ship**](FleetApi.md#refuel_ship) | **POST** /my/ships/{shipSymbol}/refuel | Refuel Ship
[**sell_cargo**](FleetApi.md#sell_cargo) | **POST** /my/ships/{shipSymbol}/sell | Sell Cargo
[**ship_refine**](FleetApi.md#ship_refine) | **POST** /my/ships/{shipSymbol}/refine | Ship Refine
[**transfer_cargo**](FleetApi.md#transfer_cargo) | **POST** /my/ships/{shipSymbol}/transfer | Transfer Cargo
[**warp_ship**](FleetApi.md#warp_ship) | **POST** /my/ships/{shipSymbol}/warp | Warp Ship



## create_chart

> crate::models::CreateChart201Response create_chart(ship_symbol, content_length)
Create Chart

Command a ship to chart the current waypoint.  Waypoints in the universe are uncharted by default. These locations will not show up in the API until they have been charted by a ship.  Charting a location will record your agent as the one who created the chart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship | [required] |
**content_length** | **i32** |  | [required] |[default to 0]

### Return type

[**crate::models::CreateChart201Response**](create_chart_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ship_ship_scan

> crate::models::CreateShipShipScan201Response create_ship_ship_scan(ship_symbol, content_length)
Scan Ships

Activate your ship's sensor arrays to scan for ship information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**content_length** | **f32** |  | [required] |[default to 0]

### Return type

[**crate::models::CreateShipShipScan201Response**](create_ship_ship_scan_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ship_system_scan

> crate::models::CreateShipSystemScan201Response create_ship_system_scan(ship_symbol, content_length)
Scan Systems

Activate your ship's sensor arrays to scan for system information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**content_length** | **i32** |  | [required] |[default to 0]

### Return type

[**crate::models::CreateShipSystemScan201Response**](create_ship_system_scan_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ship_waypoint_scan

> crate::models::CreateShipWaypointScan201Response create_ship_waypoint_scan(ship_symbol)
Scan Waypoints

Activate your ship's sensor arrays to scan for waypoint information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |

### Return type

[**crate::models::CreateShipWaypointScan201Response**](create_ship_waypoint_scan_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_survey

> crate::models::CreateSurvey201Response create_survey(ship_symbol, content_length)
Create Survey

If you want to target specific yields for an extraction, you can survey a waypoint, such as an asteroid field, and send the survey in the body of the extract request. Each survey may have multiple deposits, and if a symbol shows up more than once, that indicates a higher chance of extracting that resource.  Your ship will enter a cooldown between consecutive survey requests. Surveys will eventually expire after a period of time. Multiple ships can use the same survey for extraction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship | [required] |
**content_length** | **i32** |  | [required] |[default to 0]

### Return type

[**crate::models::CreateSurvey201Response**](create_survey_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dock_ship

> crate::models::DockShip200Response dock_ship(ship_symbol, content_length)
Dock Ship

Attempt to dock your ship at it's current location. Docking will only succeed if the waypoint is a dockable location, and your ship is capable of docking at the time of the request.  The endpoint is idempotent - successive calls will succeed even if the ship is already docked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship | [required] |
**content_length** | **f32** |  | [required] |[default to 0]

### Return type

[**crate::models::DockShip200Response**](Dock_Ship_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extract_resources

> crate::models::ExtractResources201Response extract_resources(ship_symbol, extract_resources_request)
Extract Resources

Extract resources from the waypoint into your ship. Send an optional survey as the payload to target specific yields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol | [required] |
**extract_resources_request** | Option<[**ExtractResourcesRequest**](ExtractResourcesRequest.md)> |  |  |

### Return type

[**crate::models::ExtractResources201Response**](extract_resources_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_ship

> crate::models::GetMyShip200Response get_my_ship(ship_symbol)
Get Ship

Retrieve the details of your ship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |

### Return type

[**crate::models::GetMyShip200Response**](get_my_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_ship_cargo

> crate::models::GetMyShipCargo200Response get_my_ship_cargo(ship_symbol)
Get Ship Cargo

Retrieve the cargo of your ship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship | [required] |

### Return type

[**crate::models::GetMyShipCargo200Response**](get_my_ship_cargo_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_ships

> crate::models::GetMyShips200Response get_my_ships(page, limit)
List Ships

Retrieve all of your ships.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | What entry offset to request |  |
**limit** | Option<**i32**> | How many entries to return per page |  |

### Return type

[**crate::models::GetMyShips200Response**](get_my_ships_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ship_cooldown

> crate::models::GetShipCooldown200Response get_ship_cooldown(ship_symbol)
Get Ship Cooldown

Retrieve the details of your ship's reactor cooldown. Some actions such as activating your jump drive, scanning, or extracting resources taxes your reactor and results in a cooldown.  Your ship cannot perform additional actions until your cooldown has expired. The duration of your cooldown is relative to the power consumption of the related modules or mounts for the action taken.  Response returns a 204 status code (no-content) when the ship has no cooldown.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |

### Return type

[**crate::models::GetShipCooldown200Response**](get_ship_cooldown_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ship_nav

> crate::models::GetShipNav200Response get_ship_nav(ship_symbol)
Get Ship Nav

Get the current nav status of a ship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol | [required] |

### Return type

[**crate::models::GetShipNav200Response**](get_ship_nav_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jettison

> crate::models::Jettison200Response jettison(ship_symbol, jettison_request)
Jettison Cargo

Jettison cargo from your ship's cargo hold.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**jettison_request** | Option<[**JettisonRequest**](JettisonRequest.md)> |  |  |

### Return type

[**crate::models::Jettison200Response**](jettison_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jump_ship

> crate::models::JumpShip200Response jump_ship(ship_symbol, jump_ship_request)
Jump Ship

Jump your ship instantly to a target system. Unlike other forms of navigation, jumping requires a unit of antimatter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**jump_ship_request** | Option<[**JumpShipRequest**](JumpShipRequest.md)> |  |  |

### Return type

[**crate::models::JumpShip200Response**](jump_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## navigate_ship

> crate::models::NavigateShip200Response navigate_ship(ship_symbol, navigate_ship_request)
Navigate Ship

Navigate to a target destination. The destination must be located within the same system as the ship. Navigating will consume the necessary fuel and supplies from the ship's manifest, and will pay out crew wages from the agent's account.  The returned response will detail the route information including the expected time of arrival. Most ship actions are unavailable until the ship has arrived at it's destination.  To travel between systems, see the ship's warp or jump actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol | [required] |
**navigate_ship_request** | Option<[**NavigateShipRequest**](NavigateShipRequest.md)> |  |  |

### Return type

[**crate::models::NavigateShip200Response**](navigate_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## negotiate_contract

> crate::models::NegotiateContract200Response negotiate_contract(ship_symbol, body)
Negotiate Contract



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::NegotiateContract200Response**](Negotiate_Contract_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orbit_ship

> crate::models::OrbitShip200Response orbit_ship(ship_symbol, content_length)
Orbit Ship

Attempt to move your ship into orbit at it's current location. The request will only succeed if your ship is capable of moving into orbit at the time of the request.  The endpoint is idempotent - successive calls will succeed even if the ship is already in orbit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship | [required] |
**content_length** | **i32** |  | [required] |[default to 0]

### Return type

[**crate::models::OrbitShip200Response**](Orbit_Ship_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_ship_nav

> crate::models::GetShipNav200Response patch_ship_nav(ship_symbol, patch_ship_nav_request)
Patch Ship Nav

Update the nav data of a ship, such as the flight mode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol | [required] |
**patch_ship_nav_request** | Option<[**PatchShipNavRequest**](PatchShipNavRequest.md)> |  |  |

### Return type

[**crate::models::GetShipNav200Response**](get_ship_nav_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_cargo

> crate::models::PurchaseCargo201Response purchase_cargo(ship_symbol, purchase_cargo_request)
Purchase Cargo

Purchase cargo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**purchase_cargo_request** | Option<[**PurchaseCargoRequest**](PurchaseCargoRequest.md)> |  |  |

### Return type

[**crate::models::PurchaseCargo201Response**](Purchase_Cargo_201_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_ship

> crate::models::PurchaseShip201Response purchase_ship(purchase_ship_request)
Purchase Ship

Purchase a ship

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_ship_request** | Option<[**PurchaseShipRequest**](PurchaseShipRequest.md)> |  |  |

### Return type

[**crate::models::PurchaseShip201Response**](purchase_ship_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refuel_ship

> crate::models::RefuelShip200Response refuel_ship(ship_symbol, content_length)
Refuel Ship

Refuel your ship from the local market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**content_length** | **i32** |  | [required] |[default to 0]

### Return type

[**crate::models::RefuelShip200Response**](refuel_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sell_cargo

> crate::models::SellCargo201Response sell_cargo(ship_symbol, sell_cargo_request)
Sell Cargo

Sell cargo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**sell_cargo_request** | Option<[**SellCargoRequest**](SellCargoRequest.md)> |  |  |

### Return type

[**crate::models::SellCargo201Response**](Sell_Cargo_201_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ship_refine

> crate::models::ShipRefine200Response ship_refine(ship_symbol, ship_refine_request)
Ship Refine

Attempt to refine the raw materials on your ship. The request will only succeed if your ship is capable of refining at the time of the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship | [required] |
**ship_refine_request** | Option<[**ShipRefineRequest**](ShipRefineRequest.md)> |  |  |

### Return type

[**crate::models::ShipRefine200Response**](Ship_Refine_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_cargo

> crate::models::TransferCargo200Response transfer_cargo(ship_symbol, transfer_cargo_request)
Transfer Cargo

Transfer cargo between ships.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**transfer_cargo_request** | Option<[**TransferCargoRequest**](TransferCargoRequest.md)> |  |  |

### Return type

[**crate::models::TransferCargo200Response**](Transfer_Cargo_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warp_ship

> crate::models::NavigateShip200Response warp_ship(ship_symbol, navigate_ship_request)
Warp Ship

Warp your ship to a target destination in another system. Warping will consume the necessary fuel and supplies from the ship's manifest, and will pay out crew wages from the agent's account.  The returned response will detail the route information including the expected time of arrival. Most ship actions are unavailable until the ship has arrived at it's destination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** |  | [required] |
**navigate_ship_request** | Option<[**NavigateShipRequest**](NavigateShipRequest.md)> |  |  |

### Return type

[**crate::models::NavigateShip200Response**](navigate_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

