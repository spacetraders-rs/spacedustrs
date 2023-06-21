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
[**get_mounts**](FleetApi.md#get_mounts) | **GET** /my/ships/{shipSymbol}/mounts | Get Mounts
[**get_my_ship**](FleetApi.md#get_my_ship) | **GET** /my/ships/{shipSymbol} | Get Ship
[**get_my_ship_cargo**](FleetApi.md#get_my_ship_cargo) | **GET** /my/ships/{shipSymbol}/cargo | Get Ship Cargo
[**get_my_ships**](FleetApi.md#get_my_ships) | **GET** /my/ships | List Ships
[**get_ship_cooldown**](FleetApi.md#get_ship_cooldown) | **GET** /my/ships/{shipSymbol}/cooldown | Get Ship Cooldown
[**get_ship_nav**](FleetApi.md#get_ship_nav) | **GET** /my/ships/{shipSymbol}/nav | Get Ship Nav
[**install_mount**](FleetApi.md#install_mount) | **POST** /my/ships/{shipSymbol}/mounts/install | Install Mount
[**jettison**](FleetApi.md#jettison) | **POST** /my/ships/{shipSymbol}/jettison | Jettison Cargo
[**jump_ship**](FleetApi.md#jump_ship) | **POST** /my/ships/{shipSymbol}/jump | Jump Ship
[**navigate_ship**](FleetApi.md#navigate_ship) | **POST** /my/ships/{shipSymbol}/navigate | Navigate Ship
[**negotiate_contract**](FleetApi.md#negotiate_contract) | **POST** /my/ships/{shipSymbol}/negotiate/contract | Negotiate Contract
[**orbit_ship**](FleetApi.md#orbit_ship) | **POST** /my/ships/{shipSymbol}/orbit | Orbit Ship
[**patch_ship_nav**](FleetApi.md#patch_ship_nav) | **PATCH** /my/ships/{shipSymbol}/nav | Patch Ship Nav
[**purchase_cargo**](FleetApi.md#purchase_cargo) | **POST** /my/ships/{shipSymbol}/purchase | Purchase Cargo
[**purchase_ship**](FleetApi.md#purchase_ship) | **POST** /my/ships | Purchase Ship
[**refuel_ship**](FleetApi.md#refuel_ship) | **POST** /my/ships/{shipSymbol}/refuel | Refuel Ship
[**remove_mount**](FleetApi.md#remove_mount) | **POST** /my/ships/{shipSymbol}/mounts/remove | Remove Mount
[**sell_cargo**](FleetApi.md#sell_cargo) | **POST** /my/ships/{shipSymbol}/sell | Sell Cargo
[**ship_refine**](FleetApi.md#ship_refine) | **POST** /my/ships/{shipSymbol}/refine | Ship Refine
[**transfer_cargo**](FleetApi.md#transfer_cargo) | **POST** /my/ships/{shipSymbol}/transfer | Transfer Cargo
[**warp_ship**](FleetApi.md#warp_ship) | **POST** /my/ships/{shipSymbol}/warp | Warp Ship



## create_chart

> crate::models::CreateChart201Response create_chart(ship_symbol)
Create Chart

Command a ship to chart the waypoint at its current location.  Most waypoints in the universe are uncharted by default. These waypoints have their traits hidden until they have been charted by a ship.  Charting a waypoint will record your agent as the one who created the chart, and all other agents would also be able to see the waypoint's traits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship. | [required] |

### Return type

[**crate::models::CreateChart201Response**](create_chart_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ship_ship_scan

> crate::models::CreateShipShipScan201Response create_ship_ship_scan(ship_symbol)
Scan Ships

Scan for nearby ships, retrieving information for all ships in range.  Requires a ship to have the `Sensor Array` mount installed to use.  The ship will enter a cooldown after using this function, during which it cannot execute certain actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |

### Return type

[**crate::models::CreateShipShipScan201Response**](create_ship_ship_scan_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ship_system_scan

> crate::models::CreateShipSystemScan201Response create_ship_system_scan(ship_symbol)
Scan Systems

Scan for nearby systems, retrieving information on the systems' distance from the ship and their waypoints. Requires a ship to have the `Sensor Array` mount installed to use.  The ship will enter a cooldown after using this function, during which it cannot execute certain actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |

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

Scan for nearby waypoints, retrieving detailed information on each waypoint in range. Scanning uncharted waypoints will allow you to ignore their uncharted state and will list the waypoints' traits.  Requires a ship to have the `Sensor Array` mount installed to use.  The ship will enter a cooldown after using this function, during which it cannot execute certain actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |

### Return type

[**crate::models::CreateShipWaypointScan201Response**](create_ship_waypoint_scan_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_survey

> crate::models::CreateSurvey201Response create_survey(ship_symbol)
Create Survey

Create surveys on a waypoint that can be extracted such as asteroid fields. A survey focuses on specific types of deposits from the extracted location. When ships extract using this survey, they are guaranteed to procure a high amount of one of the goods in the survey.  In order to use a survey, send the entire survey details in the body of the extract request.  Each survey may have multiple deposits, and if a symbol shows up more than once, that indicates a higher chance of extracting that resource.  Your ship will enter a cooldown after surveying in which it is unable to perform certain actions. Surveys will eventually expire after a period of time or will be exhausted after being extracted several times based on the survey's size. Multiple ships can use the same survey for extraction.  A ship must have the `Surveyor` mount installed in order to use this function.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship. | [required] |

### Return type

[**crate::models::CreateSurvey201Response**](create_survey_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dock_ship

> crate::models::DockShip200Response dock_ship(ship_symbol)
Dock Ship

Attempt to dock your ship at its current location. Docking will only succeed if your ship is capable of docking at the time of the request.  Docked ships can access elements in their current location, such as the market or a shipyard, but cannot do actions that require the ship to be above surface such as navigating or extracting.  The endpoint is idempotent - successive calls will succeed even if the ship is already docked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship. | [required] |

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

Extract resources from a waypoint that can be extracted, such as asteroid fields, into your ship. Send an optional survey as the payload to target specific yields.  The ship must be in orbit to be able to extract and must have mining equipments installed that can extract goods, such as the `Gas Siphon` mount for gas-based goods or `Mining Laser` mount for ore-based goods.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |
**extract_resources_request** | Option<[**ExtractResourcesRequest**](ExtractResourcesRequest.md)> |  |  |

### Return type

[**crate::models::ExtractResources201Response**](extract_resources_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mounts

> crate::models::GetMounts200Response get_mounts(ship_symbol)
Get Mounts

Get the mounts installed on a ship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship's symbol. | [required] |

### Return type

[**crate::models::GetMounts200Response**](Get_Mounts_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_ship

> crate::models::GetMyShip200Response get_my_ship(ship_symbol)
Get Ship

Retrieve the details of a ship under your agent's ownership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship. | [required] |

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

Retrieve the cargo of a ship under your agent's ownership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship. | [required] |

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

Return a paginated list of all of ships under your agent's ownership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | What entry offset to request |  |[default to 1]
**limit** | Option<**i32**> | How many entries to return per page |  |[default to 10]

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
**ship_symbol** | **String** | The symbol of the ship. | [required] |

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
**ship_symbol** | **String** | The ship symbol. | [required] |

### Return type

[**crate::models::GetShipNav200Response**](get_ship_nav_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## install_mount

> crate::models::InstallMount201Response install_mount(ship_symbol, install_mount_request)
Install Mount

Install a mount on a ship.  In order to install a mount, the ship must be docked and located in a waypoint that has a `Shipyard` trait. The ship also must have the mount to install in its cargo hold.  An installation fee will be deduced by the Shipyard for installing the mount on the ship. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship's symbol. | [required] |
**install_mount_request** | Option<[**InstallMountRequest**](InstallMountRequest.md)> |  |  |

### Return type

[**crate::models::InstallMount201Response**](Install_Mount_201_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jettison

> crate::models::Jettison200Response jettison(ship_symbol, jettison_request)
Jettison Cargo

Jettison cargo from your ship's cargo hold.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |
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

Jump your ship instantly to a target system. The ship must be in orbit to use this function. When used while in orbit of a Jump Gate waypoint, any ship can use this command, jumping to the target system's Jump Gate waypoint.  When used elsewhere, jumping requires the ship to have a `Jump Drive` module installed and consumes a unit of antimatter from the ship's cargo. The command will fail if there is no antimatter to consume. When jumping via the `Jump Drive` module, the ship ends up at its largest source of energy in the system, such as a gas planet or a jump gate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |
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

Navigate to a target destination. The ship must be in orbit to use this function. The destination waypoint must be within the same system as the ship's current location. Navigating will consume the necessary fuel from the ship's manifest based on the distance to the target waypoint.  The returned response will detail the route information including the expected time of arrival. Most ship actions are unavailable until the ship has arrived at it's destination.  To travel between systems, see the ship's Warp or Jump actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |
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

> crate::models::NegotiateContract200Response negotiate_contract(ship_symbol)
Negotiate Contract

Negotiate a new contract with the HQ.  In order to negotiate a new contract, an agent must not have ongoing or offered contracts over the allowed maximum amount. Currently the maximum contracts an agent can have at a time is 1.  Once a contract is negotiated, it is added to the list of contracts offered to the agent, which the agent can then accept.   The ship must be present at a faction's HQ waypoint to negotiate a contract with that faction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship's symbol. | [required] |

### Return type

[**crate::models::NegotiateContract200Response**](Negotiate_Contract_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orbit_ship

> crate::models::OrbitShip200Response orbit_ship(ship_symbol)
Orbit Ship

Attempt to move your ship into orbit at its current location. The request will only succeed if your ship is capable of moving into orbit at the time of the request.  Orbiting ships are able to do actions that require the ship to be above surface such as navigating or extracting, but cannot access elements in their current waypoint, such as the market or a shipyard.  The endpoint is idempotent - successive calls will succeed even if the ship is already in orbit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship. | [required] |

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

Update the nav configuration of a ship.  Currently only supports configuring the Flight Mode of the ship, which affects its speed and fuel consumption.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |
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

Purchase cargo from a market.  The ship must be docked in a waypoint that has `Marketplace` trait, and the market must be selling a good to be able to purchase it.  The maximum amount of units of a good that can be purchased in each transaction are denoted by the `tradeVolume` value of the good, which can be viewed by using the Get Market action.  Purchased goods are added to the ship's cargo hold.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship's symbol. | [required] |
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

Purchase a ship from a Shipyard. In order to use this function, a ship under your agent's ownership must be in a waypoint that has the `Shipyard` trait, and the Shipyard must sell the type of the desired ship.  Shipyards typically offer ship types, which are predefined templates of ships that have dedicated roles. A template comes with a preset of an engine, a reactor, and a frame. It may also include a few modules and mounts.

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

> crate::models::RefuelShip200Response refuel_ship(ship_symbol, refuel_ship_request)
Refuel Ship

Refuel your ship by buying fuel from the local market.  Requires the ship to be docked in a waypoint that has the `Marketplace` trait, and the market must be selling fuel in order to refuel.  Each fuel bought from the market replenishes 100 units in your ship's fuel.  Ships will always be refuel to their frame's maximum fuel capacity when using this action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |
**refuel_ship_request** | Option<[**RefuelShipRequest**](RefuelShipRequest.md)> |  |  |

### Return type

[**crate::models::RefuelShip200Response**](refuel_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_mount

> crate::models::RemoveMount201Response remove_mount(ship_symbol, remove_mount_request)
Remove Mount

Remove a mount from a ship.  The ship must be docked in a waypoint that has the `Shipyard` trait, and must have the desired mount that it wish to remove installed.  A removal fee will be deduced from the agent by the Shipyard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship's symbol. | [required] |
**remove_mount_request** | Option<[**RemoveMountRequest**](RemoveMountRequest.md)> |  |  |

### Return type

[**crate::models::RemoveMount201Response**](Remove_Mount_201_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sell_cargo

> crate::models::SellCargo201Response sell_cargo(ship_symbol, sell_cargo_request)
Sell Cargo

Sell cargo in your ship to a market that trades this cargo. The ship must be docked in a waypoint that has the `Marketplace` trait in order to use this function.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | Symbol of a ship. | [required] |
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

> crate::models::ShipRefine201Response ship_refine(ship_symbol, ship_refine_request)
Ship Refine

Attempt to refine the raw materials on your ship. The request will only succeed if your ship is capable of refining at the time of the request. In order to be able to refine, a ship must have goods that can be refined and have installed a `Refinery` module that can refine it.  When refining, 30 basic goods will be converted into 10 processed goods.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The symbol of the ship. | [required] |
**ship_refine_request** | Option<[**ShipRefineRequest**](ShipRefineRequest.md)> |  |  |

### Return type

[**crate::models::ShipRefine201Response**](Ship_Refine_201_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_cargo

> crate::models::TransferCargo200Response transfer_cargo(ship_symbol, transfer_cargo_request)
Transfer Cargo

Transfer cargo between ships.  The receiving ship must be in the same waypoint as the transferring ship, and it must able to hold the additional cargo after the transfer is complete. Both ships also must be in the same state, either both are docked or both are orbiting.  The response body's cargo shows the cargo of the transferring ship after the transfer is complete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The transferring ship's symbol. | [required] |
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

Warp your ship to a target destination in another system. The ship must be in orbit to use this function and must have the `Warp Drive` module installed. Warping will consume the necessary fuel from the ship's manifest.  The returned response will detail the route information including the expected time of arrival. Most ship actions are unavailable until the ship has arrived at its destination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ship_symbol** | **String** | The ship symbol. | [required] |
**navigate_ship_request** | Option<[**NavigateShipRequest**](NavigateShipRequest.md)> |  |  |

### Return type

[**crate::models::NavigateShip200Response**](navigate_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

