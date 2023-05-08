/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterRequest {
    /// The faction you choose determines your headquarters.
    #[serde(rename = "faction")]
    pub faction: Faction,
    /// How other agents will see your ships and information.
    #[serde(rename = "symbol")]
    pub symbol: String,
}

impl RegisterRequest {
    pub fn new(faction: Faction, symbol: String) -> RegisterRequest {
        RegisterRequest {
            faction,
            symbol,
        }
    }
}

/// The faction you choose determines your headquarters.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Faction {
    #[serde(rename = "COSMIC")]
    Cosmic,
    #[serde(rename = "VOID")]
    Void,
    #[serde(rename = "GALACTIC")]
    Galactic,
    #[serde(rename = "QUANTUM")]
    Quantum,
    #[serde(rename = "DOMINION")]
    Dominion,
}

impl Default for Faction {
    fn default() -> Faction {
        Self::Cosmic
    }
}

