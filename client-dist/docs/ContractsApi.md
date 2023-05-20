# \ContractsApi

All URIs are relative to *https://api.spacetraders.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_contract**](ContractsApi.md#accept_contract) | **POST** /my/contracts/{contractId}/accept | Accept Contract
[**deliver_contract**](ContractsApi.md#deliver_contract) | **POST** /my/contracts/{contractId}/deliver | Deliver Contract
[**fulfill_contract**](ContractsApi.md#fulfill_contract) | **POST** /my/contracts/{contractId}/fulfill | Fulfill Contract
[**get_contract**](ContractsApi.md#get_contract) | **GET** /my/contracts/{contractId} | Get Contract
[**get_contracts**](ContractsApi.md#get_contracts) | **GET** /my/contracts | List Contracts



## accept_contract

> crate::models::AcceptContract200Response accept_contract(contract_id)
Accept Contract

Accept a contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |

### Return type

[**crate::models::AcceptContract200Response**](accept_contract_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deliver_contract

> crate::models::DeliverContract200Response deliver_contract(contract_id, deliver_contract_request)
Deliver Contract

Deliver cargo on a given contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | The ID of the contract | [required] |
**deliver_contract_request** | Option<[**DeliverContractRequest**](DeliverContractRequest.md)> |  |  |

### Return type

[**crate::models::DeliverContract200Response**](deliver_contract_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fulfill_contract

> crate::models::FulfillContract200Response fulfill_contract(contract_id)
Fulfill Contract

Fulfill a contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | The ID of the contract | [required] |

### Return type

[**crate::models::FulfillContract200Response**](fulfill_contract_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract

> crate::models::GetContract200Response get_contract(contract_id)
Get Contract

Get the details of a contract by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | The contract ID | [required] |

### Return type

[**crate::models::GetContract200Response**](get_contract_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts

> crate::models::GetContracts200Response get_contracts(page, limit)
List Contracts

List all of your contracts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | What entry offset to request |  |
**limit** | Option<**i32**> | How many entries to return per page |  |

### Return type

[**crate::models::GetContracts200Response**](get_contracts_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

