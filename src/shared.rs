//! The shared module contains all common structs and enums used in the API
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::collections::HashMap;

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
            "Error Code: {} Error Message: {} Error Data: {}",
            self.error.code, self.error.message
        )
    }
}

impl Error for ErrorMessage {}

/// A representation of an error message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ErrorMessageData<T> {
    /// The API sent error code
    pub code: i32,
    /// The message sent from the API about the error
    pub message: String,
    /// The data sent from the API server about the error in detail
    pub data: HashMap<String, Vec<String>>
}

/// The representation of agent information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct AgentInformation {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: uint64,
}

/// The representation of faction information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct FactionInformation {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<String>,
}

/// The representation of a contract
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Contract {
    pub id: String,
    pub faction: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub terms: ContractTerms,
    pub accepted: String,
    pub fulfilled: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

/// The representation of contract terms
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ContractTerms {
    pub deadline: String,
    pub payment: ContractPaymentTerms,
    pub deliver: Vec<ContractDeliveryTerms>
}

/// The representation of contract payment terms
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ContractPaymentTerms {
    #[serde(rename = "onAccepted")]
    pub on_accepted: uint64,
    #[serde(rename = "onFulfilled")]
    pub on_fulfilled: uint64,
}

/// The representation of contract delivery terms
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ContractDeliveryTerms {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    pub destination: String,
    pub units: uint64,
    pub fulfilled: uint64,
}

/// The representation of a ship
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Ship {
    pub symbol: String,
    pub crew: Option<String>, // Appears unimplemented 3/11/22
    pub officers: Option<String>, // Appears unimplemented 3/11/22
    pub fuel: uint64,
    pub frame: String,
    pub reactor: String,
    pub engine: String,
    pub modules: Vec<String>,
    pub mounts: Vec<String>,
    pub registration: ShipRegistration,
    pub integrity: ShipIntegrity,
    pub status: String,
    pub location: String,
    pub cargo: Vec<Cargo>,
}

/// The representation of cargo
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Cargo {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    pub units: uint64,
}