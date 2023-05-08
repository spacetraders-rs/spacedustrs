/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// ShipRegistration : The public registration information of the ship



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipRegistration {
    /// The agent's registered name of the ship
    #[serde(rename = "name")]
    pub name: String,
    /// The symbol of the faction the ship is registered with
    #[serde(rename = "factionSymbol", skip_serializing_if = "Option::is_none")]
    pub faction_symbol: Option<String>,
    #[serde(rename = "role")]
    pub role: crate::models::ShipRole,
}

impl ShipRegistration {
    /// The public registration information of the ship
    pub fn new(name: String, role: crate::models::ShipRole) -> ShipRegistration {
        ShipRegistration {
            name,
            faction_symbol: None,
            role,
        }
    }
}


