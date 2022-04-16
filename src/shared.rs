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
    pub credits: i64,
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
    pub on_accepted: i64,
    /// The payment upon contract fulfillment
    #[serde(rename = "onFulfilled")]
    pub on_fulfilled: i64,
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
    pub units: i64,
    /// The number of already delivered units for this contract
    pub fulfilled: i64,
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
    pub fuel: i64,
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
    /// The ship's stats like fuelTank, jumpRange, and cargoLimit
    pub stats: ShipStats,
    /// The ship's registration information
    pub registration: ShipRegistration,
    /// The ship's integrity information
    pub integrity: ShipIntegrity,
    /// The ship's status/activity
    pub status: String,
    /// The ship's current location
    pub location: Option<String>,
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
    pub fee: i64,
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

/// The representation of ship stats
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipStats {
    /// The ship fuel tank size
    #[serde(rename = "fuelTank")]
    pub fuel_tank: u16,
    /// The ship cargo limit
    #[serde(rename = "cargoLimit")]
    pub cargo_limit: u16,
    /// The ship jump range
    #[serde(rename = "jumpRange")]
    pub jump_range: u16,
}

/// The representation of cargo
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Cargo {
    /// The good's trade symbol
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    /// The number of units
    pub units: i64,
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
    pub navigation: Navigation,
    /// The fuel cost of this flight
    #[serde(rename = "fuelCost")]
    pub fuel_cost: i64,
}

/// The representation of navigation summary
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct NavigationSummary {
    /// The symbol of the ship navigating
    pub navigation: Navigation,
}

/// The representation of a navigation object
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Navigation {
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

/// The representation of cooldown data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct CooldownData {
    /// The cooldown object
    pub cooldown: Cooldown,
}

/// The representation of a cooldown object
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Cooldown {
    /// The duration remaining
    pub duration: u64,
    /// The timestamp for the end of the cooldown
    pub expiration: String,
}

/// The representation of survey data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SurveyData {
    /// The cooldown data
    pub cooldown: Cooldown,
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

/// The representation of extract data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ExtractData {
    /// The extraction information
    pub extraction: Extraction,
    /// The cooldown till next extraction
    pub cooldown: Cooldown,
}

/// The representation of extraction information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Extraction {
    /// The symbol of the ship that completed extraction
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    /// The materials yielded from the extraction
    #[serde(rename = "yield")]
    pub extract_yield: Cargo,
}

/// The representation of status data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct StatusData {
    /// The cooldown remaining in seconds
    pub status: String,
}

/// The representation of delivery data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct DeliveryData {
    /// The trade symbol delivered
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    /// The destination delivered to
    pub destination: String,
    /// The number of units needed to fulfill the contract
    pub units: i64,
    /// The number of units fulfilled, after delivery
    pub fulfilled: i64,
}

/// The representation of refuel data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct RefuelData {
    /// The amount of credits spent on fuel
    pub credits: i64,
    /// The amount of fuel bought
    pub fuel: i64,
}

/// The various scan modes
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub enum ScanMode {
    /// Approaching ships scan mode
    #[serde(rename = "APPROACHING_SHIPS")]
    ApproachingShips,
}

/// The representation of scan data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ScanData {
    /// Cooldown till next scan available
    pub cooldown: Cooldown,
    /// List of ship scan data
    pub ships: Vec<ShipScan>,
}

/// The representation of a ship scan
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipScan {
    /// Scanned ship symbol
    pub symbol: String,
    /// Scanned ship registration
    pub registration: ShipScanRegistration,
    /// Scanned ship frame symbol
    #[serde(rename = "frameSymbol")]
    pub frame_symbol: String,
    /// Scanned ship reactor symbol
    #[serde(rename = "reactorSymbol")]
    pub reactor_symbol: String,
    /// Scanned ship engine symbol
    #[serde(rename = "engineSymbol")]
    pub engine_symbol: String,
    /// Scan expiration
    pub expiration: String,
}

/// The representation of a ship scan registration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipScanRegistration {
    /// Faction symbol of scanned ship
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    /// Role of scanned ship
    pub role: String,
}

/// The representation of waypoint information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct WaypointInformation {
    /// The symbol for the system
    pub system: String,
    /// The symbol for the waypoint
    pub symbol: String,
    /// The type of waypoint
    #[serde(rename = "type")]
    pub system_type: String,
    /// The waypoint x coordinate
    pub x: i64,
    /// The waypoint y coordinate
    pub y: i64,
    /// A list of celestial bodies orbiting the waypoint
    pub orbitals: Vec<String>,
    /// The faction that controls the waypoint
    pub faction: String,
    /// Waypoint features
    pub features: Vec<String>,
    /// Waypoint traits
    pub traits: Vec<String>,
    /// Whether the waypoint has been charted
    pub charted: bool,
    /// Who charted the system
    #[serde(rename = "chartedBy")]
    pub charted_by: Option<String>,
}

/// The representation of shipyard information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipyardInformation {
    /// The symbol for the shipyard
    pub symbol: String,
    /// The symbol for the system
    pub system: String,
    /// The faction that controls the shipyard
    pub faction: String,
    /// Ship types available
    pub ships: i64,
}

/// The representation of a ship listing
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipListing {
    /// The ship listing's id
    pub id: String,
    /// The waypoint the listing corresponds to
    pub waypoint: String,
    /// The price of the listing
    pub price: i64,
    /// The role of the ship
    pub role: String,
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
}

/// The representation of market summary
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MarketSummary {
    #[serde(rename = "waypointSymbol")]
    /// The waypoint symbol
    pub waypoint_symbol: String,
    /// The list of imported goods
    pub imports: Vec<String>,
    /// The list of exported goods
    pub exports: Vec<String>,
}

/// The representation of market information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MarketInformation {
    #[serde(rename = "waypointSymbol")]
    /// The waypoint symbol
    pub waypoint_symbol: String,
    /// The list of imported goods
    pub imports: Vec<MarketGood>,
    /// The list of exported goods
    pub exports: Vec<MarketGood>,
}

/// The representation of market goods
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MarketGood {
    #[serde(rename = "waypointSymbol")]
    /// The waypoint symbol
    pub waypoint_symbol: String,
    #[serde(rename = "tradeSymbol")]
    /// The trade symbol
    pub trade_symbol: String,
    /// The credit delta per unit
    pub price: i64,
    /// The tariff delta per unit
    pub tariff: i64,
}

/// The representation of jettison data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct JettisonData {
    #[serde(rename = "tradeSymbol")]
    /// The trade symbol of the good jettisoned
    pub trade_symbol: String,
    /// The number of units jettisoned
    pub units: i64,
}

/// The representation of transaction data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct TransactionData {
    #[serde(rename = "waypointSymbol")]
    /// The waypoint symbol of the market
    pub waypoint_symbol: String,
    #[serde(rename = "tradeSymbol")]
    /// The trade symbol of the good jettisoned
    pub trade_symbol: String,
    /// The delta of credits after TX
    pub credits: i64,
    /// The delta units in cargo after TX
    pub units: i64,
}

/// The representation of jump data with cooldown
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct JumpDataWithCooldown {
    /// The jump data
    pub jump: JumpData,
    /// The cooldown data
    pub cooldown: CooldownData,
}

/// The representation of jump data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct JumpData {
    #[serde(rename = "shipSymbol")]
    /// The ship symbol
    pub ship_symbol: String,
    /// The destination of the jump
    pub destination: String,
}

/// The representation of chart data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ChartData {
    /// The submitted symbols
    pub submitted: Option<Vec<String>>,
}
