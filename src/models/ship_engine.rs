/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// ShipEngine : The engine determines how quickly a ship travels between waypoints.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipEngine {
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<i32>,
    #[serde(rename = "speed")]
    pub speed: f32,
    #[serde(rename = "requirements")]
    pub requirements: Box<crate::models::ShipRequirements>,
}

impl ShipEngine {
    /// The engine determines how quickly a ship travels between waypoints.
    pub fn new(symbol: Symbol, name: String, description: String, speed: f32, requirements: crate::models::ShipRequirements) -> ShipEngine {
        ShipEngine {
            symbol,
            name,
            description,
            condition: None,
            speed,
            requirements: Box::new(requirements),
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Symbol {
    #[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
    ImpulseDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_I")]
    IonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    IonDriveIi,
    #[serde(rename = "ENGINE_HYPER_DRIVE_I")]
    HyperDriveI,
}

impl Default for Symbol {
    fn default() -> Symbol {
        Self::ImpulseDriveI
    }
}

