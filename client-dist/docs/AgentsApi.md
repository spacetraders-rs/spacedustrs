# \AgentsApi

All URIs are relative to *https://api.spacetraders.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_agent**](AgentsApi.md#get_agent) | **GET** /agents/{agentSymbol} | Get Public Agent
[**get_agents**](AgentsApi.md#get_agents) | **GET** /agents | List Agents
[**get_my_agent**](AgentsApi.md#get_my_agent) | **GET** /my/agent | Get Agent



## get_agent

> models::GetMyAgent200Response get_agent(agent_symbol)
Get Public Agent

Fetch agent details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_symbol** | **String** | The agent symbol | [required] |[default to FEBA66]

### Return type

[**models::GetMyAgent200Response**](get_my_agent_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agents

> models::GetAgents200Response get_agents(page, limit)
List Agents

Fetch agents details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | What entry offset to request |  |[default to 1]
**limit** | Option<**i32**> | How many entries to return per page |  |[default to 10]

### Return type

[**models::GetAgents200Response**](get_agents_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_agent

> models::GetMyAgent200Response get_my_agent()
Get Agent

Fetch your agent's details.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetMyAgent200Response**](get_my_agent_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

