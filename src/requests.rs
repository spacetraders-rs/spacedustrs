//! All requests being sent to the api are in this module
// use crate::shared;
use crate::shared;
use serde::Serialize;

/// The representation of a claim agent request
#[derive(Serialize, Debug)]
pub struct ClaimAgentRequest {
    /// The requested agent symbol/name
    pub symbol: String,
    /// The faction to join
    pub faction: String,
}

/// The representation of a navigate request
#[derive(Serialize, Debug)]
pub struct NavigateRequest {
    /// The requested destination
    pub destination: String,
}

/// The representation of a extract request
#[derive(Serialize, Debug)]
pub struct ExtractRequest {
    /// The requested destination
    pub survey: shared::Survey,
}

/// The representation of a contract delivery request
#[derive(Serialize, Debug)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ContractDeliveryRequest {
    /// The target contract id
    #[serde(rename = "contractId")]
    pub contract_id: String,
    /// The trade symbol for the good to be delivered
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    /// The amount of said good to deliver
    pub units: u64,
}

/// The representation of a scan request
#[derive(Serialize, Debug)]
pub struct ScanShipsRequest {
    /// The requested destination
    pub mode: shared::ScanMode,
}

/// The representation of a transaction request
#[derive(Serialize, Debug)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct TransactionRequest {
    /// The symbol of the good
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    /// The number of units
    pub units: u64,
}

// /// The representation of a jump request
// #[derive(Serialize, Debug)]
// #[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
// pub struct JumpRequest {
//     /// The destination symbol
//     pub destination: String,
// }