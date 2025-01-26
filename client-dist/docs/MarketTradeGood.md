# MarketTradeGood

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | [**models::TradeSymbol**](TradeSymbol.md) |  | 
**r#type** | **String** | The type of trade good (export, import, or exchange). | 
**trade_volume** | **i32** | This is the maximum number of units that can be purchased or sold at this market in a single trade for this good. Trade volume also gives an indication of price volatility. A market with a low trade volume will have large price swings, while high trade volume will be more resilient to price changes. | 
**supply** | [**models::SupplyLevel**](SupplyLevel.md) |  | 
**activity** | Option<[**models::ActivityLevel**](ActivityLevel.md)> |  | [optional]
**purchase_price** | **i32** | The price at which this good can be purchased from the market. | 
**sell_price** | **i32** | The price at which this good can be sold to the market. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


