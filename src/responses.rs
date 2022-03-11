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
