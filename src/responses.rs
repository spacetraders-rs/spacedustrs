//! All responses that come back from the API are in this module
use crate::shared::*;
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The representation of a claim agent response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ClaimAgent {
    /// The data of the response
    pub data: ClaimAgentData,
}

/// The representation of the response data from claiming an agent
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ClaimAgentData {
    /// The token of the claimed agent
    pub token: String,
    /// The agent information for the claimed agent
    pub agent: AgentInformation,
    /// The agent's starting faction info
    pub faction: FactionInformation,
    /// The agent's starting contract info
    pub contract: Contract,
    /// The agent's starting ship info
    pub ship: Ship,
}

/// The representation of a my agent response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct AgentDetails {
    /// The data of the response
    pub data: AgentInformation,
}

/// The representation of a my contracts response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ContractsResponse {
    /// The data of the response
    pub data: Vec<Contract>,
    /// Meta information about the response
    pub meta: Meta,
}

/// The representation of a my contract response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ContractResponse {
    /// The data of the response
    pub data: Contract,
}

/// The representation of an accept contract response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct AcceptedContractResponse {
    /// The data of the response
    pub data: AcceptedContractResponseResult,
}

/// The representation of the data from an accept contract response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct AcceptedContractResponseResult {
    /// New contract status
    pub accepted: bool,
}

/// The representation of a my ships response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipsResponse {
    /// The data of the response
    pub data: Vec<Ship>,
    /// Meta information about the response
    pub meta: Meta,
}

/// The representation of a my ship response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipResponse {
    /// The data of the response
    pub data: Ship,
}

/// The representation of a navigate response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct NavigateResponse {
    /// The data of the response
    pub data: NavigationInformation,
}

/// The representation of a survey cooldown response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SurveyCooldownResponse {
    /// The data of the response
    pub data: CooldownData,
}

/// The representation of a survey response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SurveyResponse {
    /// The data of the response
    pub data: SurveyData,
}

/// The representation of a system information response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemInformationResponse {
    /// The data of the response
    pub data: SystemInformation,
}

/// The representation of a system information response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsListResponse {
    /// The data of the response
    pub data: Vec<SystemInformation>,
    /// Meta information about the response
    pub meta: Meta,
}

/// The representation of a extract cooldown response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ExtractCooldownResponse {
    /// The data of the response
    pub data: CooldownData,
}

/// The representation of a extract response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ExtractResourcesResponse {
    /// The data of the response
    pub data: ExtractData,
}

/// The representation of a status response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct StatusResponse {
    /// The data of the response
    pub data: StatusData,
}

/// The representation of a delivery response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct DeliveryResponse {
    /// The data of the response
    pub data: DeliveryData,
}

/// The representation of a delivery response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct RefuelResponse {
    /// The data of the response
    pub data: RefuelData,
}

/// The representation of a scan response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ScanResponse {
    /// The data of the response
    pub data: ScanData,
}

/// The representation of a system waypoints response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsWaypointsResponse {
    /// The data of the response
    pub data: Vec<WaypointInformation>,
    /// Meta information about the response
    pub meta: Meta,
}

/// The representation of a system waypoint response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsWaypointResponse {
    /// The data of the response
    pub data: WaypointInformation,
}

/// The representation of a system shipyards response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsShipyardsResponse {
    /// The data of the response
    pub data: Vec<ShipyardInformation>,
    /// Meta information about the response
    pub meta: Meta,
}

/// The representation of a system shipyard response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsShipyardResponse {
    /// The data of the response
    pub data: ShipyardInformation,
}

/// The representation of a shipyard ships response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipyardShipsResponse {
    /// The data of the response
    pub data: Vec<ShipListing>,
    /// Meta information about the response
    pub meta: Meta,
}

/// The representation of a system waypoints response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsMarketsResponse {
    /// The data of the response
    pub data: Vec<MarketSummary>,
    /// Meta information about the response
    pub meta: Meta,
}

/// The representation of a system market response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsMarketResponse {
    /// The data of the response
    pub data: MarketInformation,
}

/// The representation of a jettison cargo response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct JettisonResponse {
    /// The data of the response
    pub data: JettisonData,
}

/// The representation of a buy/sell cargo response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct TransactionResponse {
    /// The data of the response
    pub data: TransactionData,
}

// /// The representation of a jump response
// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
// pub struct JumpResponse {
//     /// The data of the response
//     pub data: JumpData,
// }
