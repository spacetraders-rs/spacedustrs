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

/// TradeSymbol : The good's symbol.
/// The good's symbol.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TradeSymbol {
    #[serde(rename = "PRECIOUS_STONES")]
    PreciousStones,
    #[serde(rename = "QUARTZ_SAND")]
    QuartzSand,
    #[serde(rename = "SILICON_CRYSTALS")]
    SiliconCrystals,
    #[serde(rename = "AMMONIA_ICE")]
    AmmoniaIce,
    #[serde(rename = "LIQUID_HYDROGEN")]
    LiquidHydrogen,
    #[serde(rename = "LIQUID_NITROGEN")]
    LiquidNitrogen,
    #[serde(rename = "ICE_WATER")]
    IceWater,
    #[serde(rename = "EXOTIC_MATTER")]
    ExoticMatter,
    #[serde(rename = "ADVANCED_CIRCUITRY")]
    AdvancedCircuitry,
    #[serde(rename = "GRAVITON_EMITTERS")]
    GravitonEmitters,
    #[serde(rename = "IRON")]
    Iron,
    #[serde(rename = "IRON_ORE")]
    IronOre,
    #[serde(rename = "COPPER")]
    Copper,
    #[serde(rename = "COPPER_ORE")]
    CopperOre,
    #[serde(rename = "ALUMINUM")]
    Aluminum,
    #[serde(rename = "ALUMINUM_ORE")]
    AluminumOre,
    #[serde(rename = "SILVER")]
    Silver,
    #[serde(rename = "SILVER_ORE")]
    SilverOre,
    #[serde(rename = "GOLD")]
    Gold,
    #[serde(rename = "GOLD_ORE")]
    GoldOre,
    #[serde(rename = "PLATINUM")]
    Platinum,
    #[serde(rename = "PLATINUM_ORE")]
    PlatinumOre,
    #[serde(rename = "DIAMONDS")]
    Diamonds,
    #[serde(rename = "URANITE")]
    Uranite,
    #[serde(rename = "URANITE_ORE")]
    UraniteOre,
    #[serde(rename = "MERITIUM")]
    Meritium,
    #[serde(rename = "MERITIUM_ORE")]
    MeritiumOre,
    #[serde(rename = "HYDROCARBON")]
    Hydrocarbon,
    #[serde(rename = "ANTIMATTER")]
    Antimatter,
    #[serde(rename = "FAB_MATS")]
    FabMats,
    #[serde(rename = "FERTILIZERS")]
    Fertilizers,
    #[serde(rename = "FABRICS")]
    Fabrics,
    #[serde(rename = "FOOD")]
    Food,
    #[serde(rename = "JEWELRY")]
    Jewelry,
    #[serde(rename = "MACHINERY")]
    Machinery,
    #[serde(rename = "FIREARMS")]
    Firearms,
    #[serde(rename = "ASSAULT_RIFLES")]
    AssaultRifles,
    #[serde(rename = "MILITARY_EQUIPMENT")]
    MilitaryEquipment,
    #[serde(rename = "EXPLOSIVES")]
    Explosives,
    #[serde(rename = "LAB_INSTRUMENTS")]
    LabInstruments,
    #[serde(rename = "AMMUNITION")]
    Ammunition,
    #[serde(rename = "ELECTRONICS")]
    Electronics,
    #[serde(rename = "SHIP_PLATING")]
    ShipPlating,
    #[serde(rename = "SHIP_PARTS")]
    ShipParts,
    #[serde(rename = "EQUIPMENT")]
    Equipment,
    #[serde(rename = "FUEL")]
    Fuel,
    #[serde(rename = "MEDICINE")]
    Medicine,
    #[serde(rename = "DRUGS")]
    Drugs,
    #[serde(rename = "CLOTHING")]
    Clothing,
    #[serde(rename = "MICROPROCESSORS")]
    Microprocessors,
    #[serde(rename = "PLASTICS")]
    Plastics,
    #[serde(rename = "POLYNUCLEOTIDES")]
    Polynucleotides,
    #[serde(rename = "BIOCOMPOSITES")]
    Biocomposites,
    #[serde(rename = "QUANTUM_STABILIZERS")]
    QuantumStabilizers,
    #[serde(rename = "NANOBOTS")]
    Nanobots,
    #[serde(rename = "AI_MAINFRAMES")]
    AiMainframes,
    #[serde(rename = "QUANTUM_DRIVES")]
    QuantumDrives,
    #[serde(rename = "ROBOTIC_DRONES")]
    RoboticDrones,
    #[serde(rename = "CYBER_IMPLANTS")]
    CyberImplants,
    #[serde(rename = "GENE_THERAPEUTICS")]
    GeneTherapeutics,
    #[serde(rename = "NEURAL_CHIPS")]
    NeuralChips,
    #[serde(rename = "MOOD_REGULATORS")]
    MoodRegulators,
    #[serde(rename = "VIRAL_AGENTS")]
    ViralAgents,
    #[serde(rename = "MICRO_FUSION_GENERATORS")]
    MicroFusionGenerators,
    #[serde(rename = "SUPERGRAINS")]
    Supergrains,
    #[serde(rename = "LASER_RIFLES")]
    LaserRifles,
    #[serde(rename = "HOLOGRAPHICS")]
    Holographics,
    #[serde(rename = "SHIP_SALVAGE")]
    ShipSalvage,
    #[serde(rename = "RELIC_TECH")]
    RelicTech,
    #[serde(rename = "NOVEL_LIFEFORMS")]
    NovelLifeforms,
    #[serde(rename = "BOTANICAL_SPECIMENS")]
    BotanicalSpecimens,
    #[serde(rename = "CULTURAL_ARTIFACTS")]
    CulturalArtifacts,
    #[serde(rename = "FRAME_PROBE")]
    FrameProbe,
    #[serde(rename = "FRAME_DRONE")]
    FrameDrone,
    #[serde(rename = "FRAME_INTERCEPTOR")]
    FrameInterceptor,
    #[serde(rename = "FRAME_RACER")]
    FrameRacer,
    #[serde(rename = "FRAME_FIGHTER")]
    FrameFighter,
    #[serde(rename = "FRAME_FRIGATE")]
    FrameFrigate,
    #[serde(rename = "FRAME_SHUTTLE")]
    FrameShuttle,
    #[serde(rename = "FRAME_EXPLORER")]
    FrameExplorer,
    #[serde(rename = "FRAME_MINER")]
    FrameMiner,
    #[serde(rename = "FRAME_LIGHT_FREIGHTER")]
    FrameLightFreighter,
    #[serde(rename = "FRAME_HEAVY_FREIGHTER")]
    FrameHeavyFreighter,
    #[serde(rename = "FRAME_TRANSPORT")]
    FrameTransport,
    #[serde(rename = "FRAME_DESTROYER")]
    FrameDestroyer,
    #[serde(rename = "FRAME_CRUISER")]
    FrameCruiser,
    #[serde(rename = "FRAME_CARRIER")]
    FrameCarrier,
    #[serde(rename = "REACTOR_SOLAR_I")]
    ReactorSolarI,
    #[serde(rename = "REACTOR_FUSION_I")]
    ReactorFusionI,
    #[serde(rename = "REACTOR_FISSION_I")]
    ReactorFissionI,
    #[serde(rename = "REACTOR_CHEMICAL_I")]
    ReactorChemicalI,
    #[serde(rename = "REACTOR_ANTIMATTER_I")]
    ReactorAntimatterI,
    #[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
    EngineImpulseDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_I")]
    EngineIonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    EngineIonDriveIi,
    #[serde(rename = "ENGINE_HYPER_DRIVE_I")]
    EngineHyperDriveI,
    #[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
    ModuleMineralProcessorI,
    #[serde(rename = "MODULE_GAS_PROCESSOR_I")]
    ModuleGasProcessorI,
    #[serde(rename = "MODULE_CARGO_HOLD_I")]
    ModuleCargoHoldI,
    #[serde(rename = "MODULE_CARGO_HOLD_II")]
    ModuleCargoHoldIi,
    #[serde(rename = "MODULE_CARGO_HOLD_III")]
    ModuleCargoHoldIii,
    #[serde(rename = "MODULE_CREW_QUARTERS_I")]
    ModuleCrewQuartersI,
    #[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
    ModuleEnvoyQuartersI,
    #[serde(rename = "MODULE_PASSENGER_CABIN_I")]
    ModulePassengerCabinI,
    #[serde(rename = "MODULE_MICRO_REFINERY_I")]
    ModuleMicroRefineryI,
    #[serde(rename = "MODULE_SCIENCE_LAB_I")]
    ModuleScienceLabI,
    #[serde(rename = "MODULE_JUMP_DRIVE_I")]
    ModuleJumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    ModuleJumpDriveIi,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    ModuleJumpDriveIii,
    #[serde(rename = "MODULE_WARP_DRIVE_I")]
    ModuleWarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    ModuleWarpDriveIi,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    ModuleWarpDriveIii,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
    ModuleShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ModuleShieldGeneratorIi,
    #[serde(rename = "MODULE_ORE_REFINERY_I")]
    ModuleOreRefineryI,
    #[serde(rename = "MODULE_FUEL_REFINERY_I")]
    ModuleFuelRefineryI,
    #[serde(rename = "MOUNT_GAS_SIPHON_I")]
    MountGasSiphonI,
    #[serde(rename = "MOUNT_GAS_SIPHON_II")]
    MountGasSiphonIi,
    #[serde(rename = "MOUNT_GAS_SIPHON_III")]
    MountGasSiphonIii,
    #[serde(rename = "MOUNT_SURVEYOR_I")]
    MountSurveyorI,
    #[serde(rename = "MOUNT_SURVEYOR_II")]
    MountSurveyorIi,
    #[serde(rename = "MOUNT_SURVEYOR_III")]
    MountSurveyorIii,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
    MountSensorArrayI,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
    MountSensorArrayIi,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
    MountSensorArrayIii,
    #[serde(rename = "MOUNT_MINING_LASER_I")]
    MountMiningLaserI,
    #[serde(rename = "MOUNT_MINING_LASER_II")]
    MountMiningLaserIi,
    #[serde(rename = "MOUNT_MINING_LASER_III")]
    MountMiningLaserIii,
    #[serde(rename = "MOUNT_LASER_CANNON_I")]
    MountLaserCannonI,
    #[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
    MountMissileLauncherI,
    #[serde(rename = "MOUNT_TURRET_I")]
    MountTurretI,
    #[serde(rename = "SHIP_PROBE")]
    ShipProbe,
    #[serde(rename = "SHIP_MINING_DRONE")]
    ShipMiningDrone,
    #[serde(rename = "SHIP_SIPHON_DRONE")]
    ShipSiphonDrone,
    #[serde(rename = "SHIP_INTERCEPTOR")]
    ShipInterceptor,
    #[serde(rename = "SHIP_LIGHT_HAULER")]
    ShipLightHauler,
    #[serde(rename = "SHIP_COMMAND_FRIGATE")]
    ShipCommandFrigate,
    #[serde(rename = "SHIP_EXPLORER")]
    ShipExplorer,
    #[serde(rename = "SHIP_HEAVY_FREIGHTER")]
    ShipHeavyFreighter,
    #[serde(rename = "SHIP_LIGHT_SHUTTLE")]
    ShipLightShuttle,
    #[serde(rename = "SHIP_ORE_HOUND")]
    ShipOreHound,
    #[serde(rename = "SHIP_REFINING_FREIGHTER")]
    ShipRefiningFreighter,
    #[serde(rename = "SHIP_SURVEYOR")]
    ShipSurveyor,

}

impl std::fmt::Display for TradeSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PreciousStones => write!(f, "PRECIOUS_STONES"),
            Self::QuartzSand => write!(f, "QUARTZ_SAND"),
            Self::SiliconCrystals => write!(f, "SILICON_CRYSTALS"),
            Self::AmmoniaIce => write!(f, "AMMONIA_ICE"),
            Self::LiquidHydrogen => write!(f, "LIQUID_HYDROGEN"),
            Self::LiquidNitrogen => write!(f, "LIQUID_NITROGEN"),
            Self::IceWater => write!(f, "ICE_WATER"),
            Self::ExoticMatter => write!(f, "EXOTIC_MATTER"),
            Self::AdvancedCircuitry => write!(f, "ADVANCED_CIRCUITRY"),
            Self::GravitonEmitters => write!(f, "GRAVITON_EMITTERS"),
            Self::Iron => write!(f, "IRON"),
            Self::IronOre => write!(f, "IRON_ORE"),
            Self::Copper => write!(f, "COPPER"),
            Self::CopperOre => write!(f, "COPPER_ORE"),
            Self::Aluminum => write!(f, "ALUMINUM"),
            Self::AluminumOre => write!(f, "ALUMINUM_ORE"),
            Self::Silver => write!(f, "SILVER"),
            Self::SilverOre => write!(f, "SILVER_ORE"),
            Self::Gold => write!(f, "GOLD"),
            Self::GoldOre => write!(f, "GOLD_ORE"),
            Self::Platinum => write!(f, "PLATINUM"),
            Self::PlatinumOre => write!(f, "PLATINUM_ORE"),
            Self::Diamonds => write!(f, "DIAMONDS"),
            Self::Uranite => write!(f, "URANITE"),
            Self::UraniteOre => write!(f, "URANITE_ORE"),
            Self::Meritium => write!(f, "MERITIUM"),
            Self::MeritiumOre => write!(f, "MERITIUM_ORE"),
            Self::Hydrocarbon => write!(f, "HYDROCARBON"),
            Self::Antimatter => write!(f, "ANTIMATTER"),
            Self::FabMats => write!(f, "FAB_MATS"),
            Self::Fertilizers => write!(f, "FERTILIZERS"),
            Self::Fabrics => write!(f, "FABRICS"),
            Self::Food => write!(f, "FOOD"),
            Self::Jewelry => write!(f, "JEWELRY"),
            Self::Machinery => write!(f, "MACHINERY"),
            Self::Firearms => write!(f, "FIREARMS"),
            Self::AssaultRifles => write!(f, "ASSAULT_RIFLES"),
            Self::MilitaryEquipment => write!(f, "MILITARY_EQUIPMENT"),
            Self::Explosives => write!(f, "EXPLOSIVES"),
            Self::LabInstruments => write!(f, "LAB_INSTRUMENTS"),
            Self::Ammunition => write!(f, "AMMUNITION"),
            Self::Electronics => write!(f, "ELECTRONICS"),
            Self::ShipPlating => write!(f, "SHIP_PLATING"),
            Self::ShipParts => write!(f, "SHIP_PARTS"),
            Self::Equipment => write!(f, "EQUIPMENT"),
            Self::Fuel => write!(f, "FUEL"),
            Self::Medicine => write!(f, "MEDICINE"),
            Self::Drugs => write!(f, "DRUGS"),
            Self::Clothing => write!(f, "CLOTHING"),
            Self::Microprocessors => write!(f, "MICROPROCESSORS"),
            Self::Plastics => write!(f, "PLASTICS"),
            Self::Polynucleotides => write!(f, "POLYNUCLEOTIDES"),
            Self::Biocomposites => write!(f, "BIOCOMPOSITES"),
            Self::QuantumStabilizers => write!(f, "QUANTUM_STABILIZERS"),
            Self::Nanobots => write!(f, "NANOBOTS"),
            Self::AiMainframes => write!(f, "AI_MAINFRAMES"),
            Self::QuantumDrives => write!(f, "QUANTUM_DRIVES"),
            Self::RoboticDrones => write!(f, "ROBOTIC_DRONES"),
            Self::CyberImplants => write!(f, "CYBER_IMPLANTS"),
            Self::GeneTherapeutics => write!(f, "GENE_THERAPEUTICS"),
            Self::NeuralChips => write!(f, "NEURAL_CHIPS"),
            Self::MoodRegulators => write!(f, "MOOD_REGULATORS"),
            Self::ViralAgents => write!(f, "VIRAL_AGENTS"),
            Self::MicroFusionGenerators => write!(f, "MICRO_FUSION_GENERATORS"),
            Self::Supergrains => write!(f, "SUPERGRAINS"),
            Self::LaserRifles => write!(f, "LASER_RIFLES"),
            Self::Holographics => write!(f, "HOLOGRAPHICS"),
            Self::ShipSalvage => write!(f, "SHIP_SALVAGE"),
            Self::RelicTech => write!(f, "RELIC_TECH"),
            Self::NovelLifeforms => write!(f, "NOVEL_LIFEFORMS"),
            Self::BotanicalSpecimens => write!(f, "BOTANICAL_SPECIMENS"),
            Self::CulturalArtifacts => write!(f, "CULTURAL_ARTIFACTS"),
            Self::FrameProbe => write!(f, "FRAME_PROBE"),
            Self::FrameDrone => write!(f, "FRAME_DRONE"),
            Self::FrameInterceptor => write!(f, "FRAME_INTERCEPTOR"),
            Self::FrameRacer => write!(f, "FRAME_RACER"),
            Self::FrameFighter => write!(f, "FRAME_FIGHTER"),
            Self::FrameFrigate => write!(f, "FRAME_FRIGATE"),
            Self::FrameShuttle => write!(f, "FRAME_SHUTTLE"),
            Self::FrameExplorer => write!(f, "FRAME_EXPLORER"),
            Self::FrameMiner => write!(f, "FRAME_MINER"),
            Self::FrameLightFreighter => write!(f, "FRAME_LIGHT_FREIGHTER"),
            Self::FrameHeavyFreighter => write!(f, "FRAME_HEAVY_FREIGHTER"),
            Self::FrameTransport => write!(f, "FRAME_TRANSPORT"),
            Self::FrameDestroyer => write!(f, "FRAME_DESTROYER"),
            Self::FrameCruiser => write!(f, "FRAME_CRUISER"),
            Self::FrameCarrier => write!(f, "FRAME_CARRIER"),
            Self::ReactorSolarI => write!(f, "REACTOR_SOLAR_I"),
            Self::ReactorFusionI => write!(f, "REACTOR_FUSION_I"),
            Self::ReactorFissionI => write!(f, "REACTOR_FISSION_I"),
            Self::ReactorChemicalI => write!(f, "REACTOR_CHEMICAL_I"),
            Self::ReactorAntimatterI => write!(f, "REACTOR_ANTIMATTER_I"),
            Self::EngineImpulseDriveI => write!(f, "ENGINE_IMPULSE_DRIVE_I"),
            Self::EngineIonDriveI => write!(f, "ENGINE_ION_DRIVE_I"),
            Self::EngineIonDriveIi => write!(f, "ENGINE_ION_DRIVE_II"),
            Self::EngineHyperDriveI => write!(f, "ENGINE_HYPER_DRIVE_I"),
            Self::ModuleMineralProcessorI => write!(f, "MODULE_MINERAL_PROCESSOR_I"),
            Self::ModuleGasProcessorI => write!(f, "MODULE_GAS_PROCESSOR_I"),
            Self::ModuleCargoHoldI => write!(f, "MODULE_CARGO_HOLD_I"),
            Self::ModuleCargoHoldIi => write!(f, "MODULE_CARGO_HOLD_II"),
            Self::ModuleCargoHoldIii => write!(f, "MODULE_CARGO_HOLD_III"),
            Self::ModuleCrewQuartersI => write!(f, "MODULE_CREW_QUARTERS_I"),
            Self::ModuleEnvoyQuartersI => write!(f, "MODULE_ENVOY_QUARTERS_I"),
            Self::ModulePassengerCabinI => write!(f, "MODULE_PASSENGER_CABIN_I"),
            Self::ModuleMicroRefineryI => write!(f, "MODULE_MICRO_REFINERY_I"),
            Self::ModuleScienceLabI => write!(f, "MODULE_SCIENCE_LAB_I"),
            Self::ModuleJumpDriveI => write!(f, "MODULE_JUMP_DRIVE_I"),
            Self::ModuleJumpDriveIi => write!(f, "MODULE_JUMP_DRIVE_II"),
            Self::ModuleJumpDriveIii => write!(f, "MODULE_JUMP_DRIVE_III"),
            Self::ModuleWarpDriveI => write!(f, "MODULE_WARP_DRIVE_I"),
            Self::ModuleWarpDriveIi => write!(f, "MODULE_WARP_DRIVE_II"),
            Self::ModuleWarpDriveIii => write!(f, "MODULE_WARP_DRIVE_III"),
            Self::ModuleShieldGeneratorI => write!(f, "MODULE_SHIELD_GENERATOR_I"),
            Self::ModuleShieldGeneratorIi => write!(f, "MODULE_SHIELD_GENERATOR_II"),
            Self::ModuleOreRefineryI => write!(f, "MODULE_ORE_REFINERY_I"),
            Self::ModuleFuelRefineryI => write!(f, "MODULE_FUEL_REFINERY_I"),
            Self::MountGasSiphonI => write!(f, "MOUNT_GAS_SIPHON_I"),
            Self::MountGasSiphonIi => write!(f, "MOUNT_GAS_SIPHON_II"),
            Self::MountGasSiphonIii => write!(f, "MOUNT_GAS_SIPHON_III"),
            Self::MountSurveyorI => write!(f, "MOUNT_SURVEYOR_I"),
            Self::MountSurveyorIi => write!(f, "MOUNT_SURVEYOR_II"),
            Self::MountSurveyorIii => write!(f, "MOUNT_SURVEYOR_III"),
            Self::MountSensorArrayI => write!(f, "MOUNT_SENSOR_ARRAY_I"),
            Self::MountSensorArrayIi => write!(f, "MOUNT_SENSOR_ARRAY_II"),
            Self::MountSensorArrayIii => write!(f, "MOUNT_SENSOR_ARRAY_III"),
            Self::MountMiningLaserI => write!(f, "MOUNT_MINING_LASER_I"),
            Self::MountMiningLaserIi => write!(f, "MOUNT_MINING_LASER_II"),
            Self::MountMiningLaserIii => write!(f, "MOUNT_MINING_LASER_III"),
            Self::MountLaserCannonI => write!(f, "MOUNT_LASER_CANNON_I"),
            Self::MountMissileLauncherI => write!(f, "MOUNT_MISSILE_LAUNCHER_I"),
            Self::MountTurretI => write!(f, "MOUNT_TURRET_I"),
            Self::ShipProbe => write!(f, "SHIP_PROBE"),
            Self::ShipMiningDrone => write!(f, "SHIP_MINING_DRONE"),
            Self::ShipSiphonDrone => write!(f, "SHIP_SIPHON_DRONE"),
            Self::ShipInterceptor => write!(f, "SHIP_INTERCEPTOR"),
            Self::ShipLightHauler => write!(f, "SHIP_LIGHT_HAULER"),
            Self::ShipCommandFrigate => write!(f, "SHIP_COMMAND_FRIGATE"),
            Self::ShipExplorer => write!(f, "SHIP_EXPLORER"),
            Self::ShipHeavyFreighter => write!(f, "SHIP_HEAVY_FREIGHTER"),
            Self::ShipLightShuttle => write!(f, "SHIP_LIGHT_SHUTTLE"),
            Self::ShipOreHound => write!(f, "SHIP_ORE_HOUND"),
            Self::ShipRefiningFreighter => write!(f, "SHIP_REFINING_FREIGHTER"),
            Self::ShipSurveyor => write!(f, "SHIP_SURVEYOR"),
        }
    }
}

impl Default for TradeSymbol {
    fn default() -> TradeSymbol {
        Self::PreciousStones
    }
}

