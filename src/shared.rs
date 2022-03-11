//! The shared module contains all common structs and enums used in the API
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

/// An error response returned from the API
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ErrorMessage {
    /// The data about the error
    pub error: ErrorMessageData,
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error Code: {} Error Message: {} Error Data: {:#?}",
            self.error.code, self.error.message, self.error.data
        )
    }
}

impl Error for ErrorMessage {}

/// A representation of an error message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ErrorMessageData {
    /// The API sent error code
    pub code: i32,
    /// The message sent from the API about the error
    pub message: String,
    /// The data sent from the API server about the error in detail
    pub data: HashMap<String, Vec<String>>,
}

/// The representation of agent information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct AgentInformation {
    #[serde(rename = "accountId")]
    /// The unique account id of the agent
    pub account_id: String,
    /// The given agent symbol/name
    pub symbol: String,
    /// The agent's headquarters location
    pub headquarters: String,
    /// The agent's current credit balance
    pub credits: u64,
}

/// The representation of faction information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct FactionInformation {
    /// The faction's symbol
    pub symbol: String,
    /// The faction's name
    pub name: String,
    /// A short description of the faction
    pub description: String,
    /// The faction's headquarters location
    pub headquarters: String,
    /// A list of faction traits
    pub traits: Vec<String>,
}

/// The representation of a contract
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Contract {
    /// The unique contract id
    pub id: String,
    /// The faction giving the contract
    pub faction: String,
    /// The type of contract
    #[serde(rename = "type")]
    pub contract_type: String,
    /// The contract's terms
    pub terms: ContractTerms,
    /// Whether the contract has been accepted
    pub accepted: bool,
    /// Whether the contract has been fulfilled
    pub fulfilled: bool,
    /// The expiry timestamp for the contract (must accept before)
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

/// The representation of contract terms
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ContractTerms {
    /// The deadline for the contract (must fulfill before)
    pub deadline: String,
    /// The payment terms for the contract
    pub payment: ContractPaymentTerms,
    /// The delivery terms for the contract
    pub deliver: Vec<ContractDeliveryTerms>,
}

/// The representation of contract payment terms
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ContractPaymentTerms {
    /// The payment upon contract acceptance
    #[serde(rename = "onAccepted")]
    pub on_accepted: u64,
    /// The payment upon contract fulfillment
    #[serde(rename = "onFulfilled")]
    pub on_fulfilled: u64,
}

/// The representation of contract delivery terms
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ContractDeliveryTerms {
    /// The trade symbol of the good requested by the contract
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    /// The delivery destination for the contract
    pub destination: String,
    /// The number of units required by the contract
    pub units: u64,
    /// The number of already delivered units for this contract
    pub fulfilled: u64,
}

/// The representation of a ship
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Ship {
    /// The ship's unique symbol
    pub symbol: String,
    /// Appears to be unimplemented 3/11/22
    pub crew: Option<String>,
    /// Appears to be unimplemented 3/11/22
    pub officers: Option<String>,
    /// The fuel remaining
    pub fuel: u64,
    /// The installed frame
    pub frame: String,
    /// The installed reactor
    pub reactor: String,
    /// The installed engine
    pub engine: String,
    /// A list of the installed modules
    pub modules: Vec<String>,
    /// A list of the installed mounts
    pub mounts: Vec<String>,
    /// The ship's registration information
    pub registration: ShipRegistration,
    /// The ship's integrity information
    pub integrity: ShipIntegrity,
    /// The ship's status/activity
    pub status: String,
    /// The ship's current location
    pub location: String,
    /// The ship's stored cargo
    pub cargo: Vec<Cargo>,
}

/// The representation of ship registration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipRegistration {
    /// The faction symbol that issued the registration
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    /// The agent symbol whom the ship is registered to
    #[serde(rename = "agentSymbol")]
    pub agent_symbol: String,
    /// The registration fee
    pub fee: u64,
    /// The registered role
    pub role: String,
}

/// The representation of ship integrity
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipIntegrity {
    /// The ship frame integrity
    pub frame: u16,
    /// The ship reactor integrity
    pub reactor: u16,
    /// The ship engine integrity
    pub engine: u16,
}

/// The representation of cargo
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Cargo {
    /// The good's trade symbol
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    /// The number of units
    pub units: u64,
}

/// The representation of response meta info
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Meta {
    /// The total
    pub total: u16,
    /// The page
    pub page: u16,
    /// The limit
    pub limit: u16,
}

/// The representation of navigation information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct NavigationInformation {
    /// The symbol of the ship navigating
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    /// The departure location
    pub departure: String,
    /// The destination location
    pub destination: String,
    /// Duration remaining as of call
    #[serde(rename = "durationRemaining")]
    pub duration_remaining: Option<u64>,
    /// Timestamp of arrival
    #[serde(rename = "arrivedAt")]
    pub arrived_at: Option<String>,
}

/// The representation of survey cooldown data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SurveyCooldownData {
    /// The cooldown remaining in seconds
    pub cooldown: u64,
}

/// The representation of survey data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SurveyData {
    /// The cooldown data
    pub cooldown: u64,
    /// List of surveys (extraction locations) available
    pub surveys: Vec<Survey>,
}

/// The representation of a survey
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Survey {
    /// The signature of the survey
    pub signature: String,
    /// The list of deposits that the survey found
    pub deposits: Vec<String>,
    /// The expiration timestamp for the survey
    pub expiration: String,
}

/// The representation of system information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemInformation {
    /// The symbol for the system
    pub symbol: String,
    /// The sector that contains this system
    pub sector: String,
    /// The type of system
    #[serde(rename = "type")]
    pub system_type: String,
    /// The system x coordinate
    pub x: i64,
    /// The system y coordinate
    pub y: i64,
    /// A list of waypoints in the system
    pub waypoints: Vec<String>,
    /// A list of factions in the system
    pub factions: Vec<String>,
    /// Whether the system has been charted
    pub charted: bool,
    /// Who charted the system
    #[serde(rename = "chartedBy")]
    pub charted_by: Option<String>,
}
