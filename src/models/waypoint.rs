/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Waypoint : A waypoint is a location that ships can travel to such as a Planet, Moon or Space Station.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Waypoint {
    /// The symbol of the waypoint.
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "type")]
    pub r#type: models::WaypointType,
    /// The symbol of the system.
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    /// Relative position of the waypoint on the system's x axis. This is not an absolute position in the universe.
    #[serde(rename = "x")]
    pub x: i32,
    /// Relative position of the waypoint on the system's y axis. This is not an absolute position in the universe.
    #[serde(rename = "y")]
    pub y: i32,
    /// Waypoints that orbit this waypoint.
    #[serde(rename = "orbitals")]
    pub orbitals: Vec<models::WaypointOrbital>,
    /// The symbol of the parent waypoint, if this waypoint is in orbit around another waypoint. Otherwise this value is undefined.
    #[serde(rename = "orbits", skip_serializing_if = "Option::is_none")]
    pub orbits: Option<String>,
    #[serde(rename = "faction", skip_serializing_if = "Option::is_none")]
    pub faction: Option<Box<models::WaypointFaction>>,
    /// The traits of the waypoint.
    #[serde(rename = "traits")]
    pub traits: Vec<models::WaypointTrait>,
    /// The modifiers of the waypoint.
    #[serde(rename = "modifiers", skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<models::WaypointModifier>>,
    #[serde(rename = "chart", skip_serializing_if = "Option::is_none")]
    pub chart: Option<Box<models::Chart>>,
    /// True if the waypoint is under construction.
    #[serde(rename = "isUnderConstruction")]
    pub is_under_construction: bool,
}

impl Waypoint {
    /// A waypoint is a location that ships can travel to such as a Planet, Moon or Space Station.
    pub fn new(symbol: String, r#type: models::WaypointType, system_symbol: String, x: i32, y: i32, orbitals: Vec<models::WaypointOrbital>, traits: Vec<models::WaypointTrait>, is_under_construction: bool) -> Waypoint {
        Waypoint {
            symbol,
            r#type,
            system_symbol,
            x,
            y,
            orbitals,
            orbits: None,
            faction: None,
            traits,
            modifiers: None,
            chart: None,
            is_under_construction,
        }
    }
}

