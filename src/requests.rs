//! All requests being sent to the api are in this module
// use crate::shared;
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
