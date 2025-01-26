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

/// ShipConditionEvent : An event that represents damage or wear to a ship's reactor, frame, or engine, reducing the condition of the ship.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipConditionEvent {
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    #[serde(rename = "component")]
    pub component: Component,
    /// The name of the event.
    #[serde(rename = "name")]
    pub name: String,
    /// A description of the event.
    #[serde(rename = "description")]
    pub description: String,
}

impl ShipConditionEvent {
    /// An event that represents damage or wear to a ship's reactor, frame, or engine, reducing the condition of the ship.
    pub fn new(symbol: Symbol, component: Component, name: String, description: String) -> ShipConditionEvent {
        ShipConditionEvent {
            symbol,
            component,
            name,
            description,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Symbol {
    #[serde(rename = "REACTOR_OVERLOAD")]
    ReactorOverload,
    #[serde(rename = "ENERGY_SPIKE_FROM_MINERAL")]
    EnergySpikeFromMineral,
    #[serde(rename = "SOLAR_FLARE_INTERFERENCE")]
    SolarFlareInterference,
    #[serde(rename = "COOLANT_LEAK")]
    CoolantLeak,
    #[serde(rename = "POWER_DISTRIBUTION_FLUCTUATION")]
    PowerDistributionFluctuation,
    #[serde(rename = "MAGNETIC_FIELD_DISRUPTION")]
    MagneticFieldDisruption,
    #[serde(rename = "HULL_MICROMETEORITE_STRIKES")]
    HullMicrometeoriteStrikes,
    #[serde(rename = "STRUCTURAL_STRESS_FRACTURES")]
    StructuralStressFractures,
    #[serde(rename = "CORROSIVE_MINERAL_CONTAMINATION")]
    CorrosiveMineralContamination,
    #[serde(rename = "THERMAL_EXPANSION_MISMATCH")]
    ThermalExpansionMismatch,
    #[serde(rename = "VIBRATION_DAMAGE_FROM_DRILLING")]
    VibrationDamageFromDrilling,
    #[serde(rename = "ELECTROMAGNETIC_FIELD_INTERFERENCE")]
    ElectromagneticFieldInterference,
    #[serde(rename = "IMPACT_WITH_EXTRACTED_DEBRIS")]
    ImpactWithExtractedDebris,
    #[serde(rename = "FUEL_EFFICIENCY_DEGRADATION")]
    FuelEfficiencyDegradation,
    #[serde(rename = "COOLANT_SYSTEM_AGEING")]
    CoolantSystemAgeing,
    #[serde(rename = "DUST_MICROABRASIONS")]
    DustMicroabrasions,
    #[serde(rename = "THRUSTER_NOZZLE_WEAR")]
    ThrusterNozzleWear,
    #[serde(rename = "EXHAUST_PORT_CLOGGING")]
    ExhaustPortClogging,
    #[serde(rename = "BEARING_LUBRICATION_FADE")]
    BearingLubricationFade,
    #[serde(rename = "SENSOR_CALIBRATION_DRIFT")]
    SensorCalibrationDrift,
    #[serde(rename = "HULL_MICROMETEORITE_DAMAGE")]
    HullMicrometeoriteDamage,
    #[serde(rename = "SPACE_DEBRIS_COLLISION")]
    SpaceDebrisCollision,
    #[serde(rename = "THERMAL_STRESS")]
    ThermalStress,
    #[serde(rename = "VIBRATION_OVERLOAD")]
    VibrationOverload,
    #[serde(rename = "PRESSURE_DIFFERENTIAL_STRESS")]
    PressureDifferentialStress,
    #[serde(rename = "ELECTROMAGNETIC_SURGE_EFFECTS")]
    ElectromagneticSurgeEffects,
    #[serde(rename = "ATMOSPHERIC_ENTRY_HEAT")]
    AtmosphericEntryHeat,
}

impl Default for Symbol {
    fn default() -> Symbol {
        Self::ReactorOverload
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Component {
    #[serde(rename = "FRAME")]
    Frame,
    #[serde(rename = "REACTOR")]
    Reactor,
    #[serde(rename = "ENGINE")]
    Engine,
}

impl Default for Component {
    fn default() -> Component {
        Self::Frame
    }
}

