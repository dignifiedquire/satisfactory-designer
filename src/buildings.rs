use egui::Color32;

mod assembler;
mod blender;
mod constructor;
mod converter;
mod encoder;
mod foundry;
mod manufacturer;
mod merger;
mod miner;
mod oil_extractor;
mod packager;
mod particle_accelerator;
mod pipeline_junction;
mod refinery;
mod sink;
mod smelter;
mod splitter;
mod storage_container;
mod water_extractor;

use crate::node::{Input, Output};
use crate::util::load_img;

pub use self::assembler::Assembler;
pub use self::blender::Blender;
pub use self::constructor::Constructor;
pub use self::converter::Converter;
pub use self::encoder::QuantumEncoder;
pub use self::foundry::Foundry;
pub use self::manufacturer::Manufacturer;
pub use self::merger::Merger;
pub use self::miner::{Miner, MinerLevel, ResourcePurity};
pub use self::oil_extractor::OilExtractor;
pub use self::packager::Packager;
pub use self::particle_accelerator::ParticleAccelerator;
pub use self::pipeline_junction::PipelineJunction;
pub use self::refinery::Refinery;
pub use self::sink::AwesomeSink;
pub use self::smelter::Smelter;
pub use self::splitter::Splitter;
pub use self::storage_container::StorageContainer;
pub use self::water_extractor::WaterExtractor;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub enum Building {
    Miner(Miner),
    Smelter(Smelter),
    Splitter(Splitter),
    Merger(Merger),
    Constructor(Constructor),
    StorageContainer(StorageContainer),
    WaterExtractor(WaterExtractor),
    OilExtractor(OilExtractor),
    Packager(Packager),
    Refinery(Refinery),
    Foundry(Foundry),
    Assembler(Assembler),
    PipelineJunction(PipelineJunction),
    Manufacturer(Manufacturer),
    AwesomeSink(AwesomeSink),
    Blender(Blender),
    ParticleAccelerator(ParticleAccelerator),
    QuantumEncoder(QuantumEncoder),
    Converter(Converter),
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    strum::VariantArray,
    strum::Display,
)]
pub enum ResourceType {
    #[strum(to_string = "Bauxite")]
    Bauxite,
    #[strum(to_string = "Caterium Ore")]
    CateriumOre,
    #[strum(to_string = "Copper Ore")]
    CopperOre,
    #[strum(to_string = "Iron Ore")]
    IronOre,
    #[strum(to_string = "Limestone")]
    Limestone,
    #[strum(to_string = "Raw Quartz")]
    RawQuartz,
    #[strum(to_string = "SAM")]
    Sam,
    #[strum(to_string = "Sulfur")]
    Sulfur,
    #[strum(to_string = "Uranium")]
    Uranium,
    #[strum(to_string = "Coal")]
    Coal,
}

pub trait Selectable: strum::VariantArray + PartialEq + Clone {
    const NAME: &'static str;

    fn name(&self) -> String;
    fn image(&self) -> String;
}

impl Selectable for ResourceType {
    const NAME: &'static str = "Resource";

    fn name(&self) -> String {
        self.to_string()
    }

    fn image(&self) -> String {
        self.output_material().image()
    }
}

impl ResourceType {
    pub fn output_material(&self) -> Material {
        match self {
            Self::Bauxite => Material::Bauxite,
            Self::CateriumOre => Material::CateriumOre,
            Self::CopperOre => Material::CopperOre,
            Self::IronOre => Material::IronOre,
            Self::Limestone => Material::Limestone,
            Self::RawQuartz => Material::RawQuartz,
            Self::Sam => Material::Sam,
            Self::Sulfur => Material::Sulfur,
            Self::Uranium => Material::Uranium,
            Self::Coal => Material::Coal,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    strum::Display,
    strum::VariantArray,
)]
pub enum Material {
    #[strum(to_string = "Copper Ore")]
    CopperOre,
    #[strum(to_string = "Iron Ore")]
    IronOre,
    #[strum(to_string = "Caterium Ore")]
    CateriumOre,
    #[strum(to_string = "Copper Ingot")]
    CopperIngot,
    #[strum(to_string = "Iron Ingot")]
    IronIngot,
    #[strum(to_string = "Caterium Ingot")]
    CateriumIngot,
    #[strum(to_string = "Aluminum Scrap")]
    AluminumScrap,
    #[strum(to_string = "Aluminum Ingot")]
    AluminumIngot,
    #[strum(to_string = "Bauxite")]
    Bauxite,
    #[strum(to_string = "Limestone")]
    Limestone,
    #[strum(to_string = "Raw Quartz")]
    RawQuartz,
    #[strum(to_string = "Sulfur")]
    Sulfur,
    #[strum(to_string = "Uranium")]
    Uranium,
    #[strum(to_string = "Alien Protein")]
    AlienProtein,
    #[strum(to_string = "Alien DNA Capsule")]
    AlienDnaCapsule,
    #[strum(to_string = "Leaves")]
    Leaves,
    #[strum(to_string = "Mycelia")]
    Mycelia,
    #[strum(to_string = "Wood")]
    Wood,
    #[strum(to_string = "Wire")]
    Wire,
    #[strum(to_string = "Plastic")]
    Plastic,
    #[strum(to_string = "Ficsite Ingot")]
    FicsiteIngot,
    #[strum(to_string = "Hatcher Remains")]
    HatcherRemains,
    #[strum(to_string = "Hog Remains")]
    HogRemains,
    #[strum(to_string = "Blue Power Slug")]
    BluePowerSlug,
    #[strum(to_string = "Yellow Power Slug")]
    YellowPowerSlug,
    #[strum(to_string = "Purple Power Slug")]
    PurplePowerSlug,
    #[strum(to_string = "SAM")]
    Sam,
    #[strum(to_string = "Iron Rod")]
    IronRod,
    #[strum(to_string = "Biomass")]
    Biomass,
    #[strum(to_string = "Spitter Remains")]
    SpitterRemains,
    #[strum(to_string = "Steel Ingot")]
    SteelIngot,
    #[strum(to_string = "Stinger Remains")]
    StingerRemains,
    #[strum(to_string = "Steel Beam")]
    SteelBeam,
    #[strum(to_string = "Aluminum Casing")]
    AluminumCasing,
    #[strum(to_string = "Cable")]
    Cable,
    #[strum(to_string = "Concrete")]
    Concrete,
    #[strum(to_string = "Copper Powder")]
    CopperPowder,
    #[strum(to_string = "Copper Sheet")]
    CopperSheet,
    #[strum(to_string = "Empty Canister")]
    EmptyCanister,
    #[strum(to_string = "Empty Fluid Tank")]
    EmptyFluidTank,
    #[strum(to_string = "Ficsite Trigon")]
    FicsiteTrigon,
    #[strum(to_string = "Iron Plate")]
    IronPlate,
    #[strum(to_string = "Iron Rebar")]
    IronRebar,
    #[strum(to_string = "Power Shard")]
    PowerShard,
    #[strum(to_string = "Quartz Crystal")]
    QuartzCrystal,
    #[strum(to_string = "Reanimated SAM")]
    ReanimatedSAM,
    #[strum(to_string = "Screw")]
    Screw,
    #[strum(to_string = "Silica")]
    Silica,
    #[strum(to_string = "Solid Biofuel")]
    SolidBiofuel,
    #[strum(to_string = "Steel Pipe")]
    SteelPipe,
    #[strum(to_string = "Coal")]
    Coal,
    #[strum(to_string = "Quickwire")]
    Quickwire,
    #[strum(to_string = "Packaged Alumnia Solution")]
    PackagedAluminaSolution,
    #[strum(to_string = "Packaged Fuel")]
    PackagedFuel,
    #[strum(to_string = "Packaged HeavyOil Residue")]
    PackagedHeavyOilResidue,
    #[strum(to_string = "Packaged Ionized Fuel")]
    PackagedIonizedFuel,
    #[strum(to_string = "Packaged Liquid Biofuel")]
    PackagedLiquidBiofuel,
    #[strum(to_string = "Packaged Nitric Acid")]
    PackagedNitricAcid,
    #[strum(to_string = "Packaged Nitrogen Gas")]
    PackagedNitrogenGas,
    #[strum(to_string = "Packaged Oil")]
    PackagedOil,
    #[strum(to_string = "Packaged Rocket Fuel")]
    PackagedRocketFuel,
    #[strum(to_string = "Packaged Sulfuric Acid")]
    PackagedSulfuricAcid,
    #[strum(to_string = "Packaged Turbofuel")]
    PackagedTurbofuel,
    #[strum(to_string = "Packaged Water")]
    PackagedWater,
    #[strum(to_string = "Polymer Resin")]
    PolymerResin,
    #[strum(to_string = "Black Powder")]
    BlackPowder,
    #[strum(to_string = "Petroleum Coke")]
    PetroleumCoke,
    #[strum(to_string = "Rubber")]
    Rubber,
    #[strum(to_string = "Compacted Coal")]
    CompactedCoal,
    #[strum(to_string = "Smokeless Powder")]
    SmokelessPowder,
    #[strum(to_string = "Fabric")]
    Fabric,
    #[strum(to_string = "AI Limiter")]
    AILimiter,
    #[strum(to_string = "Adaptive Control Unit")]
    AdaptiveControlUnit,
    #[strum(to_string = "Alclad Aluminum Sheet")]
    AlcladAluminumSheet,
    #[strum(to_string = "Supercomputer")]
    Supercomputer,
    #[strum(to_string = "Assembly Director System")]
    AssemblyDirectorSystem,
    #[strum(to_string = "Stator")]
    Stator,
    #[strum(to_string = "Automated Wiring")]
    AutomatedWiring,
    #[strum(to_string = "Circuit Board")]
    CircuitBoard,
    #[strum(to_string = "Cluster Nobelisk")]
    ClusterNobelisk,
    #[strum(to_string = "Nobelisk")]
    Nobelisk,
    #[strum(to_string = "Electromagnetic Control Rod")]
    ElectromagneticControlRod,
    #[strum(to_string = "EncasedIndustrial Beam")]
    EncasedIndustrialBeam,
    #[strum(to_string = "Plutonium Pellet")]
    PlutoniumPellet,
    #[strum(to_string = "Gas Nobelisk")]
    GasNobelisk,
    #[strum(to_string = "Encased Plutonium Cell")]
    EncasedPlutoniumCell,
    #[strum(to_string = "Rifle Ammo")]
    RifleAmmo,
    #[strum(to_string = "Heat Sink")]
    HeatSink,
    #[strum(to_string = "Homing Rifle Ammo")]
    HomingRifleAmmo,
    #[strum(to_string = "High-Speed Connector")]
    HighSpeedConnector,
    #[strum(to_string = "Versatile Framework")]
    VersatileFramework,
    #[strum(to_string = "Magnetic Field Generator")]
    MagneticFieldGenerator,
    #[strum(to_string = "Rotor")]
    Rotor,
    #[strum(to_string = "Crystal Oscillator")]
    CrystalOscillator,
    #[strum(to_string = "Computer")]
    Computer,
    #[strum(to_string = "Motor")]
    Motor,
    #[strum(to_string = "Cooling System")]
    CoolingSystem,
    #[strum(to_string = "Pressure Conversion Cube")]
    PressureConversionCube,
    #[strum(to_string = "Radio Control Unit")]
    RadioControlUnit,
    #[strum(to_string = "Plutonium Fuel Rod")]
    PlutoniumFuelRod,
    #[strum(to_string = "Modular Frame")]
    ModularFrame,
    #[strum(to_string = "Reinforced Iron Plate")]
    ReinforcedIronPlate,
    #[strum(to_string = "Pulse Nobelisk")]
    PulseNobelisk,
    #[strum(to_string = "Shatter Rebar")]
    ShatterRebar,
    #[strum(to_string = "Smart Plating")]
    SmartPlating,
    #[strum(to_string = "Stun Rebar")]
    StunRebar,
    #[strum(to_string = "Portable Miner")]
    PortableMiner,
    #[strum(to_string = "Heavy Modular Frame")]
    HeavyModularFrame,
    #[strum(to_string = "Thermal Propulsion Rocket")]
    ThermalPropulsionRocket,
    #[strum(to_string = "Singularity Cell")]
    SingularityCell,
    #[strum(to_string = "Dark Matter Crystal")]
    DarkMatterCrystal,
    #[strum(to_string = "Ballistic Warp Drive")]
    BallisticWarpDrive,
    #[strum(to_string = "Superposition Oscillator")]
    SuperpositionOscillator,
    #[strum(to_string = "Gas Filter")]
    GasFilter,
    #[strum(to_string = "Iodine Infused Filter")]
    IodineInfusedFilter,
    #[strum(to_string = "Modular Engine")]
    ModularEngine,
    #[strum(to_string = "Encased UraniumCell")]
    EncasedUraniumCell,
    #[strum(to_string = "Nuke Nobelisk")]
    NukeNobelisk,
    #[strum(to_string = "SAM Fluctuator")]
    SAMFluctuator,
    #[strum(to_string = "Nuclear Pasta")]
    NuclearPasta,
    #[strum(to_string = "Turbo Motor")]
    TurboMotor,
    #[strum(to_string = "Fused Modular Frame")]
    FusedModularFrame,
    #[strum(to_string = "Turbo Rifle Ammo")]
    TurboRifleAmmo,
    #[strum(to_string = "Uranium FuelRod")]
    UraniumFuelRod,
    #[strum(to_string = "Battery")]
    Battery,
    #[strum(to_string = "Explosive Rebar")]
    ExplosiveRebar,
    #[strum(to_string = "Biochemical Sculptor")]
    BiochemicalSculptor,
    #[strum(to_string = "Non-Fissile Uranium")]
    NonFissileUranium,
    #[strum(to_string = "Uranium Waste")]
    UraniumWaste,
    #[strum(to_string = "Diamonds")]
    Diamonds,
    #[strum(to_string = "Ficsonium")]
    Ficsonium,
    #[strum(to_string = "Plutonium Waste")]
    PlutoniumWaste,
    #[strum(to_string = "Time Crystal")]
    TimeCrystal,
    #[strum(to_string = "Neural-Quantum Processor")]
    NeuralQuantumProcessor,
    #[strum(to_string = "AI Expansion Server")]
    AIExpansionServer,
    #[strum(to_string = "Alien Power Matrix")]
    AlienPowerMatrix,
    #[strum(to_string = "Ficsonium Fuel Rod")]
    FicsoniumFuelRod,
}

impl Selectable for Material {
    const NAME: &'static str = "Material";

    fn name(&self) -> String {
        self.to_string()
    }

    fn image(&self) -> String {
        let name = match self {
            Self::CopperOre => "40px-Copper_Ore.png",
            Self::IronOre => "40px-Iron_Ore.png",
            Self::CateriumOre => "40px-Caterium_Ore.png",
            Self::CopperIngot => "40px-Copper_Ingot.png",
            Self::IronIngot => "40px-Iron_Ingot.png",
            Self::CateriumIngot => "40px-Caterium_Ingot.png",
            Self::AluminumScrap => "40px-Aluminum_Scrap.png",
            Self::AluminumIngot => "40px-Aluminum_Ingot.png",
            Self::Bauxite => "40px-Bauxite.png",
            Self::Limestone => "40px-Limestone.png",
            Self::RawQuartz => "40px-Raw_Quartz.png",
            Self::Sam => "40px-SAM.png",
            Self::Sulfur => "40px-Sulfur.png",
            Self::Uranium => "40px-Uranium.png",
            Self::AlienProtein => "40px-Alien_Protein.png",
            Self::AlienDnaCapsule => "40px-Alien_DNA_Capsule.png",
            Self::Leaves => "40px-Leaves.png",
            Self::Mycelia => "40px-Mycelia.png",
            Self::Wood => "40px-Wood.png",
            Self::Wire => "40px-Wire.png",
            Self::Plastic => "40px-Plastic.png",
            Self::FicsiteIngot => "40px-Ficsite_Ingot.png",
            Self::HatcherRemains => "40px-Hatcher_Remains.png",
            Self::HogRemains => "40px-Hog_Remains.png",
            Self::BluePowerSlug => "40px-Blue_Power_Slug.png",
            Self::YellowPowerSlug => "40px-Yellow_Power_Slug.png",
            Self::PurplePowerSlug => "40px-Purple_Power_Slug.png",
            Self::IronRod => "40px-Iron_Rod.png",
            Self::Biomass => "40px-Biomass.png",
            Self::SpitterRemains => "40px-Spitter_Remains.png",
            Self::SteelIngot => "40px-Steel_Ingot.png",
            Self::StingerRemains => "40px-Stinger_Remains.png",
            Self::SteelBeam => "40px-Steel_Beam.png",
            Self::AluminumCasing => "40px-Aluminum_Casing.png",
            Self::Cable => "40px-Cable.png",
            Self::Concrete => "40px-Concrete.png",
            Self::CopperPowder => "40px-Copper_Powder.png",
            Self::CopperSheet => "40px-Copper_Sheet.png",
            Self::EmptyCanister => "40px-Empty_Canister.png",
            Self::EmptyFluidTank => "40px-Empty_Fluid_Tank.png",
            Self::FicsiteTrigon => "40px-Ficsite_Trigon.png",
            Self::IronPlate => "40px-Iron_Plate.png",
            Self::IronRebar => "40px-Iron_Rebar.png",
            Self::PowerShard => "40px-Power_Shard.png",
            Self::QuartzCrystal => "40px-Quartz_Crystal.png",
            Self::ReanimatedSAM => "40px-Reanimated_SAM.png",
            Self::Screw => "40px-Screw.png",
            Self::Silica => "40px-Silica.png",
            Self::SolidBiofuel => "40px-Solid_Biofuel.png",
            Self::SteelPipe => "40px-Steel_Pipe.png",
            Self::Coal => "40px-Coal.png",
            Self::Quickwire => "40px-Quickwire.png",
            Self::PackagedAluminaSolution => "40px-Packaged_Alumina_Solution.png",
            Self::PackagedFuel => "40px-Packaged_Fuel.png",
            Self::PackagedHeavyOilResidue => "40px-Packaged_Heavy_Oil_Residue.png",
            Self::PackagedIonizedFuel => "40px-Packaged_Ionized_Fuel.png",
            Self::PackagedLiquidBiofuel => "40px-Packaged_Liquid_Biofuel.png",
            Self::PackagedNitricAcid => "40px-Packaged_Nitric_Acid.png",
            Self::PackagedNitrogenGas => "40px-Packaged_Nitrogen_Gas.png",
            Self::PackagedOil => "40px-Packaged_Oil.png",
            Self::PackagedRocketFuel => "40px-Packaged_Rocket_Fuel.png",
            Self::PackagedSulfuricAcid => "40px-Packaged_Sulfuric_Acid.png",
            Self::PackagedTurbofuel => "40px-Packaged_Turbofuel.png",
            Self::PackagedWater => "40px-Packaged_Water.png",
            Self::PolymerResin => "40px-Polymer_Resin.png",
            Self::BlackPowder => "40px-Black_Powder.png",
            Self::PetroleumCoke => "40px-Petroleum_Coke.png",
            Self::Rubber => "40px-Rubber.png",
            Self::CompactedCoal => "40px-Compacted_Coal.png",
            Self::SmokelessPowder => "40px-Smokeless_Powder.png",
            Self::Fabric => "40px-Fabric.png",
            Self::AILimiter => "40px-AI_Limiter.png",
            Self::AdaptiveControlUnit => "40px-Adaptive_Control_Unit.png",
            Self::AlcladAluminumSheet => "40px-Alclad_Aluminum_Sheet.png",
            Self::Supercomputer => "40px-Supercomputer.png",
            Self::AssemblyDirectorSystem => "40px-Assembly_Director_System.png",
            Self::Stator => "40px-Stator.png",
            Self::AutomatedWiring => "40px-Automated_Wiring.png",
            Self::CircuitBoard => "40px-Circuit_Board.png",
            Self::ClusterNobelisk => "40px-Cluster_Nobelisk.png",
            Self::Nobelisk => "40px-Nobelisk.png",
            Self::ElectromagneticControlRod => "40px-Electromagnetic_Control_Rod.png",
            Self::EncasedIndustrialBeam => "40px-Encased_Industrial_Beam.png",
            Self::PlutoniumPellet => "40px-Plutonium_Pellet.png",
            Self::GasNobelisk => "40px-Gas_Nobelisk.png",
            Self::EncasedPlutoniumCell => "40px-Encased_Plutonium_Cell.png",
            Self::RifleAmmo => "40px-Rifle_Ammo.png",
            Self::HeatSink => "40px-Heat_Sink.png",
            Self::HomingRifleAmmo => "40px-Homing_Rifle_Ammo.png",
            Self::HighSpeedConnector => "40px-High-Speed_Connector.png",
            Self::VersatileFramework => "40px-Versatile_Framework.png",
            Self::MagneticFieldGenerator => "40px-Magnetic_Field_Generator.png",
            Self::Rotor => "40px-Rotor.png",
            Self::CrystalOscillator => "40px-Crystal_Oscillator.png",
            Self::Computer => "40px-Computer.png",
            Self::Motor => "40px-Motor.png",
            Self::CoolingSystem => "40px-Cooling_System.png",
            Self::PressureConversionCube => "40px-Pressure_Conversion_Cube.png",
            Self::RadioControlUnit => "40px-Radio_Control_Unit.png",
            Self::PlutoniumFuelRod => "40px-Plutonium_Fuel_Rod.png",
            Self::ModularFrame => "40px-Modular_Frame.png",
            Self::ReinforcedIronPlate => "40px-Reinforced_Iron_Plate.png",
            Self::PulseNobelisk => "40px-Pulse_Nobelisk.png",
            Self::ShatterRebar => "40px-Shatter_Rebar.png",
            Self::SmartPlating => "40px-Smart_Plating.png",
            Self::StunRebar => "40px-Stun_Rebar.png",
            Self::PortableMiner => "40px-Portable_Miner.png",
            Self::HeavyModularFrame => "40px-Heavy_Modular_Frame.png",
            Self::ThermalPropulsionRocket => "40px-Thermal_Propulsion_Rocket.png",
            Self::SingularityCell => "40px-Singularity_Cell.png",
            Self::DarkMatterCrystal => "40px-Dark_Matter_Crystal.png",
            Self::BallisticWarpDrive => "40px-Ballistic_Warp_Drive.png",
            Self::SuperpositionOscillator => "40px-Superposition_Oscillator.png",
            Self::GasFilter => "40px-Gas_Filter.png",
            Self::IodineInfusedFilter => "40px-Iodine-Infused_Filter.png",
            Self::ModularEngine => "40px-Modular_Engine.png",
            Self::EncasedUraniumCell => "40px-Encased_Uranium_Cell.png",
            Self::NukeNobelisk => "40px-Nuke_Nobelisk.png",
            Self::SAMFluctuator => "40px-SAM_Fluctuator.png",
            Self::NuclearPasta => "40px-Nuclear_Pasta.png",
            Self::TurboMotor => "40px-Turbo_Motor.png",
            Self::FusedModularFrame => "40px-Fused_Modular_Frame.png",
            Self::TurboRifleAmmo => "40px-Turbo_Rifle_Ammo.png",
            Self::UraniumFuelRod => "40px-Uranium_Fuel_Rod.png",
            Self::Battery => "40px-Battery.png",
            Self::ExplosiveRebar => "40px-Explosive_Rebar.png",
            Self::BiochemicalSculptor => "40px-Biochemical_Sculptor.png",
            Self::NonFissileUranium => "40px-Non-Fissile_Uranium.png",
            Self::UraniumWaste => "40px-Uranium_Waste.png",
            Self::Diamonds => "40px-Diamonds.png",
            Self::Ficsonium => "40px-Ficsonium.png",
            Self::PlutoniumWaste => "40px-Plutonium_Waste.png",
            Self::TimeCrystal => "40px-Time_Crystal.png",
            Self::NeuralQuantumProcessor => "40px-Neural-Quantum_Processor.png",
            Self::AIExpansionServer => "40px-AI_Expansion_Server.png",
            Self::AlienPowerMatrix => "40px-Alien_Power_Matrix.png",
            Self::FicsoniumFuelRod => "40px-Ficsonium_Fuel_Rod.png",
        };
        load_img(name)
    }
}

impl Material {
    pub fn color(&self) -> Color32 {
        // Colors based on https://www.reddit.com/r/SatisfactoryGame/comments/154vft6/vencams_colour_list_25/
        let color = match self {
            Self::CopperOre => "#BD4C39",
            Self::IronOre => "#8E5C5C",
            Self::CateriumOre => "#E2B148",
            Self::CopperIngot => "#A56355",
            Self::IronIngot => "#989A9D",
            Self::CateriumIngot => "#CCA566",
            Self::AluminumScrap => "#BCC0C9",
            Self::AluminumIngot => "#D2D3D4",
            Self::Bauxite => "#CD7660",
            Self::Limestone => "#C8BFA7",
            Self::RawQuartz => "#F177B5",
            Self::Sam => "#AE1CD7",
            Self::Sulfur => "#FCDC48",
            Self::Uranium => "#88D288",
            _ => "#697082",
        };
        Color32::from_hex(color).unwrap()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    strum::Display,
    strum::VariantArray,
)]
pub enum Fluid {
    #[strum(to_string = "Alumina Solution")]
    AluminaSolution,
    #[strum(to_string = "Fuel")]
    Fuel,
    #[strum(to_string = "Heavy Oil Residue")]
    HeavyOilResidue,
    #[strum(to_string = "Ionized Fuel")]
    IonizedFuel,
    #[strum(to_string = "Liquid Biofuel")]
    LiquidBiofuel,
    #[strum(to_string = "Nitric Acid")]
    NitricAcid,
    #[strum(to_string = "Nitrogen Gas")]
    NitrogenGas,
    #[strum(to_string = "Crude Oil")]
    CrudeOil,
    #[strum(to_string = "Rocket Fuel")]
    RocketFuel,
    #[strum(to_string = "Sulfuric Acid")]
    SulfuricAcid,
    #[strum(to_string = "Turbofuel")]
    Turbofuel,
    #[strum(to_string = "Water")]
    Water,
    #[strum(to_string = "Dissolved Silica")]
    DissolvedSilica,
    #[strum(to_string = "Dark Matter Residue")]
    DarkMatterResidue,
    #[strum(to_string = "Excited Photonic Matter")]
    ExcitedPhotonicMatter,
}

impl Fluid {
    pub fn name(&self) -> String {
        self.to_string()
    }

    pub fn color(&self) -> Color32 {
        let code = match self {
            Self::AluminaSolution => "#DDDEDF",
            Self::Fuel => "#D47615",
            Self::HeavyOilResidue => "#AE1CD7",
            Self::IonizedFuel => "#fdf1cf",
            Self::LiquidBiofuel => "#70b846",
            Self::NitricAcid => "#F7FAD7",
            Self::NitrogenGas => "#898990",
            Self::CrudeOil => "#0A090F",
            Self::RocketFuel => "#ea3928",
            Self::SulfuricAcid => "#f5ff17",
            Self::Turbofuel => "#A10000",
            Self::Water => "#1662AD",
            Self::DissolvedSilica => "#eac9e3",
            Self::DarkMatterResidue => "#e8bce4",
            Self::ExcitedPhotonicMatter => "#ffffff",
        };
        Color32::from_hex(code).unwrap()
    }

    pub fn image(&self) -> String {
        let name = match self {
            Self::AluminaSolution => "40px-Alumina_Solution.png",
            Self::Fuel => "40px-Fuel.png",
            Self::HeavyOilResidue => "40px-Heavy_Oil_Residue.png",
            Self::IonizedFuel => "40px-Ionized_Fuel.png",
            Self::LiquidBiofuel => "40px-Liquid_Biofuel.png",
            Self::NitricAcid => "40px-Nitric_Acid.png",
            Self::NitrogenGas => "40px-Nitrogen_Gas.png",
            Self::CrudeOil => "40px-Crude_Oil.png",
            Self::RocketFuel => "40px-Rocket_Fuel.png",
            Self::SulfuricAcid => "40px-Sulfuric_Acid.png",
            Self::Turbofuel => "40px-Turbofuel.png",
            Self::Water => "40px-Water.png",
            Self::DissolvedSilica => "40px-Dissolved_Silica.png",
            Self::DarkMatterResidue => "40px-Dark_Matter_Residue.png",
            Self::ExcitedPhotonicMatter => "40px-Excited_Photonic_Matter.png",
        };
        load_img(name)
    }
}

impl Building {
    /// Clone, but with caches reset
    pub fn clear_clone(&self) -> Self {
        match self {
            Self::Miner(m) => Self::Miner(m.clear_clone()),
            Self::Smelter(s) => Self::Smelter(s.clear_clone()),
            Self::Splitter(s) => Self::Splitter(s.clear_clone()),
            Self::Merger(s) => Self::Merger(s.clear_clone()),
            Self::Constructor(s) => Self::Constructor(s.clear_clone()),
            Self::StorageContainer(s) => Self::StorageContainer(s.clear_clone()),
            Self::WaterExtractor(s) => Self::WaterExtractor(s.clear_clone()),
            Self::OilExtractor(s) => Self::OilExtractor(s.clear_clone()),
            Self::Packager(s) => Self::Packager(s.clear_clone()),
            Self::Refinery(s) => Self::Refinery(s.clear_clone()),
            Self::Foundry(s) => Self::Foundry(s.clear_clone()),
            Self::Assembler(s) => Self::Assembler(s.clear_clone()),
            Self::PipelineJunction(s) => Self::PipelineJunction(s.clear_clone()),
            Self::Manufacturer(s) => Self::Manufacturer(s.clear_clone()),
            Self::AwesomeSink(s) => Self::AwesomeSink(s.clear_clone()),
            Self::Blender(s) => Self::Blender(s.clear_clone()),
            Self::ParticleAccelerator(s) => Self::ParticleAccelerator(s.clear_clone()),
            Self::QuantumEncoder(s) => Self::QuantumEncoder(s.clear_clone()),
            Self::Converter(s) => Self::Converter(s.clear_clone()),
        }
    }

    pub fn header_image(&self) -> String {
        match self {
            Self::Miner(m) => m.header_image(),
            Self::Smelter(s) => s.header_image(),
            Self::Splitter(s) => s.header_image(),
            Self::Merger(s) => s.header_image(),
            Self::Constructor(s) => s.header_image(),
            Self::StorageContainer(s) => s.header_image(),
            Self::WaterExtractor(s) => s.header_image(),
            Self::OilExtractor(s) => s.header_image(),
            Self::Packager(s) => s.header_image(),
            Self::Refinery(s) => s.header_image(),
            Self::Foundry(s) => s.header_image(),
            Self::Assembler(s) => s.header_image(),
            Self::PipelineJunction(s) => s.header_image(),
            Self::Manufacturer(s) => s.header_image(),
            Self::AwesomeSink(s) => s.header_image(),
            Self::Blender(s) => s.header_image(),
            Self::ParticleAccelerator(s) => s.header_image(),
            Self::QuantumEncoder(s) => s.header_image(),
            Self::Converter(s) => s.header_image(),
        }
    }

    pub fn outputs(&self) -> usize {
        match self {
            Self::Miner(m) => m.num_outputs(),
            Self::Smelter(s) => s.num_outputs(),
            Self::Splitter(s) => s.num_outputs(),
            Self::Merger(s) => s.num_outputs(),
            Self::Constructor(s) => s.num_outputs(),
            Self::StorageContainer(s) => s.num_outputs(),
            Self::WaterExtractor(s) => s.num_outputs(),
            Self::OilExtractor(s) => s.num_outputs(),
            Self::Packager(s) => s.num_outputs(),
            Self::Refinery(s) => s.num_outputs(),
            Self::Foundry(s) => s.num_outputs(),
            Self::Assembler(s) => s.num_outputs(),
            Self::PipelineJunction(s) => s.num_outputs(),
            Self::Manufacturer(s) => s.num_outputs(),
            Self::AwesomeSink(s) => s.num_outputs(),
            Self::Blender(s) => s.num_outputs(),
            Self::ParticleAccelerator(s) => s.num_outputs(),
            Self::QuantumEncoder(s) => s.num_outputs(),
            Self::Converter(s) => s.num_outputs(),
        }
    }

    pub fn inputs(&self) -> usize {
        match self {
            Self::Miner(m) => m.num_inputs(),
            Self::Smelter(s) => s.num_inputs(),
            Self::Splitter(s) => s.num_inputs(),
            Self::Merger(s) => s.num_inputs(),
            Self::Constructor(s) => s.num_inputs(),
            Self::StorageContainer(s) => s.num_inputs(),
            Self::WaterExtractor(s) => s.num_inputs(),
            Self::OilExtractor(s) => s.num_inputs(),
            Self::Packager(s) => s.num_inputs(),
            Self::Refinery(s) => s.num_inputs(),
            Self::Foundry(s) => s.num_inputs(),
            Self::Assembler(s) => s.num_inputs(),
            Self::PipelineJunction(s) => s.num_inputs(),
            Self::Manufacturer(s) => s.num_inputs(),
            Self::AwesomeSink(s) => s.num_inputs(),
            Self::Blender(s) => s.num_inputs(),
            Self::ParticleAccelerator(s) => s.num_inputs(),
            Self::QuantumEncoder(s) => s.num_inputs(),
            Self::Converter(s) => s.num_inputs(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Miner(m) => m.name(),
            Self::Smelter(s) => s.name(),
            Self::Splitter(s) => s.name(),
            Self::Merger(s) => s.name(),
            Self::Constructor(s) => s.name(),
            Self::StorageContainer(s) => s.name(),
            Self::WaterExtractor(s) => s.name(),
            Self::OilExtractor(s) => s.name(),
            Self::Packager(s) => s.name(),
            Self::Refinery(s) => s.name(),
            Self::Foundry(s) => s.name(),
            Self::Assembler(s) => s.name(),
            Self::PipelineJunction(s) => s.name(),
            Self::Manufacturer(s) => s.name(),
            Self::AwesomeSink(s) => s.name(),
            Self::Blender(s) => s.name(),
            Self::ParticleAccelerator(s) => s.name(),
            Self::QuantumEncoder(s) => s.name(),
            Self::Converter(s) => s.name(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::Miner(m) => m.description(),
            Self::Smelter(s) => s.description(),
            Self::Splitter(s) => s.description(),
            Self::Merger(s) => s.description(),
            Self::Constructor(s) => s.description(),
            Self::StorageContainer(s) => s.description(),
            Self::WaterExtractor(s) => s.description(),
            Self::OilExtractor(s) => s.description(),
            Self::Packager(s) => s.description(),
            Self::Refinery(s) => s.description(),
            Self::Foundry(s) => s.description(),
            Self::Assembler(s) => s.description(),
            Self::PipelineJunction(s) => s.description(),
            Self::Manufacturer(s) => s.description(),
            Self::AwesomeSink(s) => s.description(),
            Self::Blender(s) => s.description(),
            Self::ParticleAccelerator(s) => s.description(),
            Self::QuantumEncoder(s) => s.description(),
            Self::Converter(s) => s.description(),
        }
    }

    pub fn input_resource(&self, input_id: usize) -> crate::node::ResourceType {
        match self {
            Self::Miner(m) => m.input_resource(input_id),
            Self::Smelter(s) => s.input_resource(input_id),
            Self::Splitter(s) => s.input_resource(input_id),
            Self::Merger(s) => s.input_resource(input_id),
            Self::Constructor(s) => s.input_resource(input_id),
            Self::StorageContainer(s) => s.input_resource(input_id),
            Self::WaterExtractor(s) => s.input_resource(input_id),
            Self::OilExtractor(s) => s.input_resource(input_id),
            Self::Packager(s) => s.input_resource(input_id),
            Self::Refinery(s) => s.input_resource(input_id),
            Self::Foundry(s) => s.input_resource(input_id),
            Self::Assembler(s) => s.input_resource(input_id),
            Self::PipelineJunction(s) => s.input_resource(input_id),
            Self::Manufacturer(s) => s.input_resource(input_id),
            Self::AwesomeSink(s) => s.input_resource(input_id),
            Self::Blender(s) => s.input_resource(input_id),
            Self::ParticleAccelerator(s) => s.input_resource(input_id),
            Self::QuantumEncoder(s) => s.input_resource(input_id),
            Self::Converter(s) => s.input_resource(input_id),
        }
    }

    pub fn output_resource(&self, output_id: usize) -> crate::node::ResourceType {
        match self {
            Self::Miner(m) => m.output_resource(output_id),
            Self::Smelter(s) => s.output_resource(output_id),
            Self::Splitter(s) => s.output_resource(output_id),
            Self::Merger(s) => s.output_resource(output_id),
            Self::Constructor(s) => s.output_resource(output_id),
            Self::StorageContainer(s) => s.output_resource(output_id),
            Self::WaterExtractor(s) => s.output_resource(output_id),
            Self::OilExtractor(s) => s.output_resource(output_id),
            Self::Packager(s) => s.output_resource(output_id),
            Self::Refinery(s) => s.output_resource(output_id),
            Self::Foundry(s) => s.output_resource(output_id),
            Self::Assembler(s) => s.output_resource(output_id),
            Self::PipelineJunction(s) => s.output_resource(output_id),
            Self::Manufacturer(s) => s.output_resource(output_id),
            Self::AwesomeSink(s) => s.output_resource(output_id),
            Self::Blender(s) => s.output_resource(output_id),
            Self::ParticleAccelerator(s) => s.output_resource(output_id),
            Self::QuantumEncoder(s) => s.output_resource(output_id),
            Self::Converter(s) => s.output_resource(output_id),
        }
    }

    pub fn current_output(&self, output_id: usize) -> Option<Output> {
        match self {
            Self::Miner(m) => {
                assert_eq!(output_id, 0, "1 output");
                m.current_output()
            }
            Self::Smelter(s) => {
                assert_eq!(output_id, 0, "1 output");
                s.current_output()
            }
            Self::Constructor(s) => {
                assert_eq!(output_id, 0, "1 output");
                s.current_output()
            }
            Self::StorageContainer(s) => {
                assert_eq!(output_id, 0, "1 output");
                s.current_output()
            }
            Self::WaterExtractor(w) => {
                assert_eq!(output_id, 0, "1 output");
                w.current_output()
            }
            Self::OilExtractor(o) => {
                assert_eq!(output_id, 0, "1 output");
                o.current_output()
            }
            Self::Packager(p) => match output_id {
                0 => p.current_output_fluid(),
                1 => p.current_output_material(),
                _ => unreachable!("only two output"),
            },
            Self::Refinery(r) => match output_id {
                0 => r.current_output_fluid(),
                1 => r.current_output_material(),
                _ => unreachable!("only two output"),
            },
            Self::Foundry(f) => {
                assert_eq!(output_id, 0, "1 output");
                f.current_output()
            }
            Self::Assembler(a) => {
                assert_eq!(output_id, 0, "1 output");
                a.current_output()
            }
            Self::Manufacturer(m) => {
                assert_eq!(output_id, 0, "1 output");
                m.current_output()
            }
            Self::Splitter(s) => match output_id {
                0 => s.current_output_0(),
                1 => s.current_output_1(),
                2 => s.current_output_2(),
                _ => unreachable!("3 outputs"),
            },
            Self::Merger(m) => {
                assert_eq!(output_id, 0, "1 output");
                m.current_output()
            }
            Self::PipelineJunction(s) => match output_id {
                0 => s.current_output_0(),
                1 => s.current_output_1(),
                2 => s.current_output_2(),
                3 => s.current_output_3(),
                _ => unreachable!("4 outputs"),
            },
            Self::AwesomeSink(_) => None,
            Self::Blender(b) => match output_id {
                0 => b.current_output_fluid(),
                1 => b.current_output_material(),
                _ => unreachable!("2 outputs"),
            },
            Self::ParticleAccelerator(b) => match output_id {
                0 => b.current_output_material(),
                _ => unreachable!("1 outputs"),
            },
            Self::QuantumEncoder(q) => match output_id {
                0 => q.current_output_fluid(),
                1 => q.current_output_material(),
                _ => unreachable!("2 outputs"),
            },
            Self::Converter(c) => match output_id {
                0 => c.current_output_fluid(),
                1 => c.current_output_material(),
                _ => unreachable!("2 outputs"),
            },
        }
    }

    pub fn current_input(&self, input_id: usize) -> Option<Input> {
        match self {
            Self::Miner(_) => {
                unreachable!("no inputs");
            }
            Self::Smelter(s) => {
                assert_eq!(input_id, 0, "1 input");
                s.current_input.clone()
            }
            Self::Constructor(c) => {
                assert_eq!(input_id, 0, "1 input");
                c.current_input.clone()
            }
            Self::StorageContainer(_) => {
                unreachable!("no inputs");
            }
            Self::WaterExtractor(_) => {
                unreachable!("no inputs");
            }
            Self::OilExtractor(_) => {
                unreachable!("no inputs");
            }
            Self::Packager(p) => match input_id {
                0 => p.current_input_fluid.clone(),
                1 => p.current_input_material.clone(),
                _ => unreachable!("2 inputs"),
            },
            Self::Refinery(r) => match input_id {
                0 => r.current_input_fluid.clone(),
                1 => r.current_input_material.clone(),
                _ => unreachable!("2 inputs"),
            },
            Self::Foundry(f) => match input_id {
                0 => f.current_input_material_0.clone(),
                1 => f.current_input_material_1.clone(),
                _ => unreachable!("2 inputs"),
            },
            Self::Assembler(a) => match input_id {
                0 => a.current_input_material_0.clone(),
                1 => a.current_input_material_1.clone(),
                _ => unreachable!("2 inputs"),
            },
            Self::Manufacturer(m) => match input_id {
                0 => m.current_input_material_0.clone(),
                1 => m.current_input_material_1.clone(),
                2 => m.current_input_material_2.clone(),
                3 => m.current_input_material_3.clone(),
                _ => unreachable!("4 inputs"),
            },
            Self::Splitter(s) => {
                assert_eq!(input_id, 0, "1 input");
                s.current_input.clone()
            }
            Self::Merger(m) => match input_id {
                0 => m.current_input_0.clone(),
                1 => m.current_input_1.clone(),
                2 => m.current_input_2.clone(),
                _ => unreachable!("3 inputs"),
            },
            Self::PipelineJunction(s) => match input_id {
                0 => s.current_input_0.clone(),
                1 => s.current_input_1.clone(),
                2 => s.current_input_2.clone(),
                3 => s.current_input_3.clone(),
                _ => unreachable!("3 inputs"),
            },
            Self::AwesomeSink(s) => {
                assert_eq!(input_id, 0, "1 input");
                s.current_input.clone()
            }
            Self::Blender(b) => match input_id {
                0 => b.current_input_fluid_0.clone(),
                1 => b.current_input_fluid_1.clone(),
                2 => b.current_input_material_0.clone(),
                3 => b.current_input_material_1.clone(),
                _ => unreachable!("4 inputs"),
            },
            Self::ParticleAccelerator(b) => match input_id {
                0 => b.current_input_fluid_0.clone(),
                1 => b.current_input_material_0.clone(),
                2 => b.current_input_material_1.clone(),
                _ => unreachable!("3 inputs"),
            },
            Self::QuantumEncoder(q) => match input_id {
                0 => q.current_input_fluid_0.clone(),
                1 => q.current_input_material_0.clone(),
                2 => q.current_input_material_1.clone(),
                3 => q.current_input_material_2.clone(),
                _ => unreachable!("4 inputs"),
            },
            Self::Converter(c) => match input_id {
                0 => c.current_input_material_0.clone(),
                1 => c.current_input_material_1.clone(),
                _ => unreachable!("2 inputs"),
            },
        }
    }

    pub fn set_current_input(&mut self, input: Output, input_id: usize) {
        match self {
            Self::Miner(_) => {
                unreachable!("no inputs");
            }
            Self::Smelter(s) => {
                assert_eq!(input_id, 0, "1 input");
                s.current_input.replace(input.into());
            }
            Self::Constructor(c) => {
                assert_eq!(input_id, 0, "1 input");
                c.current_input.replace(input.into());
            }
            Self::StorageContainer(_) => {
                unreachable!("no inputs");
            }
            Self::WaterExtractor(_) => {
                unreachable!("no inputs");
            }
            Self::OilExtractor(_) => {
                unreachable!("no inputs");
            }
            Self::Packager(p) => match input_id {
                0 => {
                    p.current_input_fluid.replace(input.into());
                }
                1 => {
                    p.current_input_material.replace(input.into());
                }
                _ => unreachable!("2 inputs"),
            },
            Self::Refinery(r) => match input_id {
                0 => {
                    r.current_input_fluid.replace(input.into());
                }
                1 => {
                    r.current_input_material.replace(input.into());
                }
                _ => unreachable!("2 inputs"),
            },
            Self::Foundry(f) => match input_id {
                0 => {
                    f.current_input_material_0.replace(input.into());
                }
                1 => {
                    f.current_input_material_1.replace(input.into());
                }
                _ => unreachable!("2 inputs"),
            },
            Self::Assembler(a) => match input_id {
                0 => {
                    a.current_input_material_0.replace(input.into());
                }
                1 => {
                    a.current_input_material_1.replace(input.into());
                }
                _ => unreachable!("2 inputs"),
            },
            Self::Manufacturer(m) => match input_id {
                0 => {
                    m.current_input_material_0.replace(input.into());
                }
                1 => {
                    m.current_input_material_1.replace(input.into());
                }
                2 => {
                    m.current_input_material_2.replace(input.into());
                }
                3 => {
                    m.current_input_material_3.replace(input.into());
                }
                _ => unreachable!("4 inputs"),
            },
            Self::Splitter(s) => {
                assert_eq!(input_id, 0, "1 input");
                s.current_input.replace(input.into());
            }
            Self::Merger(m) => match input_id {
                0 => {
                    m.current_input_0.replace(input.into());
                }
                1 => {
                    m.current_input_1.replace(input.into());
                }
                2 => {
                    m.current_input_2.replace(input.into());
                }
                _ => unreachable!("3 inputs"),
            },
            Self::PipelineJunction(s) => match input_id {
                0 => {
                    s.current_input_0.replace(input.into());
                }
                1 => {
                    s.current_input_1.replace(input.into());
                }
                2 => {
                    s.current_input_2.replace(input.into());
                }
                3 => {
                    s.current_input_3.replace(input.into());
                }
                _ => unreachable!("3 inputs"),
            },
            Self::AwesomeSink(s) => {
                assert_eq!(input_id, 0, "1 input");
                s.current_input.replace(input.into());
            }
            Self::Blender(b) => match input_id {
                0 => {
                    b.current_input_fluid_0.replace(input.into());
                }
                1 => {
                    b.current_input_fluid_1.replace(input.into());
                }
                2 => {
                    b.current_input_material_0.replace(input.into());
                }
                3 => {
                    b.current_input_material_1.replace(input.into());
                }
                _ => unreachable!("4 inputs"),
            },
            Self::ParticleAccelerator(b) => match input_id {
                0 => {
                    b.current_input_fluid_0.replace(input.into());
                }
                1 => {
                    b.current_input_material_0.replace(input.into());
                }
                2 => {
                    b.current_input_material_1.replace(input.into());
                }
                _ => unreachable!("3 inputs"),
            },
            Self::QuantumEncoder(q) => match input_id {
                0 => {
                    q.current_input_fluid_0.replace(input.into());
                }
                1 => {
                    q.current_input_material_0.replace(input.into());
                }
                2 => {
                    q.current_input_material_1.replace(input.into());
                }
                3 => {
                    q.current_input_material_2.replace(input.into());
                }
                _ => unreachable!("4 inputs"),
            },
            Self::Converter(c) => match input_id {
                0 => {
                    c.current_input_material_0.replace(input.into());
                }
                1 => {
                    c.current_input_material_1.replace(input.into());
                }
                _ => unreachable!("2 inputs"),
            },
        }
    }

    pub fn clear_current_input(&mut self, input_id: usize) {
        match self {
            Self::Miner(_) => {
                unreachable!("no inputs");
            }
            Self::Smelter(s) => {
                assert_eq!(input_id, 0, "1 input");
                s.current_input = None;
            }
            Self::Constructor(c) => {
                assert_eq!(input_id, 0, "1 input");
                c.current_input = None;
            }
            Self::StorageContainer(_) => {
                unreachable!("no inputs");
            }
            Self::WaterExtractor(_) => {
                unreachable!("no inputs");
            }
            Self::OilExtractor(_) => {
                unreachable!("no inputs");
            }
            Self::Packager(p) => match input_id {
                0 => {
                    p.current_input_fluid = None;
                }
                1 => {
                    p.current_input_material = None;
                }
                _ => unreachable!("2 inputs"),
            },
            Self::Refinery(r) => match input_id {
                0 => {
                    r.current_input_fluid = None;
                }
                1 => {
                    r.current_input_material = None;
                }
                _ => unreachable!("2 inputs"),
            },
            Self::Foundry(f) => match input_id {
                0 => {
                    f.current_input_material_0 = None;
                }
                1 => {
                    f.current_input_material_1 = None;
                }
                _ => unreachable!("2 inputs"),
            },
            Self::Assembler(a) => match input_id {
                0 => {
                    a.current_input_material_0 = None;
                }
                1 => {
                    a.current_input_material_1 = None;
                }
                _ => unreachable!("2 inputs"),
            },
            Self::Manufacturer(m) => match input_id {
                0 => {
                    m.current_input_material_0 = None;
                }
                1 => {
                    m.current_input_material_1 = None;
                }
                2 => {
                    m.current_input_material_2 = None;
                }
                3 => {
                    m.current_input_material_3 = None;
                }
                _ => unreachable!("4 inputs"),
            },
            Self::Splitter(s) => {
                assert_eq!(input_id, 0, "1 input");
                s.current_input = None;
            }
            Self::Merger(m) => match input_id {
                0 => {
                    m.current_input_0 = None;
                }
                1 => {
                    m.current_input_1 = None;
                }
                2 => {
                    m.current_input_2 = None;
                }
                _ => unreachable!("3 inputs"),
            },
            Self::PipelineJunction(s) => match input_id {
                0 => {
                    s.current_input_0 = None;
                }
                1 => {
                    s.current_input_1 = None;
                }
                2 => {
                    s.current_input_2 = None;
                }
                3 => {
                    s.current_input_3 = None;
                }
                _ => unreachable!("4 inputs"),
            },
            Self::AwesomeSink(s) => {
                assert_eq!(input_id, 0, "1 input");
                s.current_input = None;
            }
            Self::Blender(b) => match input_id {
                0 => {
                    b.current_input_fluid_0 = None;
                }
                1 => {
                    b.current_input_fluid_1 = None;
                }
                2 => {
                    b.current_input_material_0 = None;
                }
                3 => {
                    b.current_input_material_1 = None;
                }
                _ => unreachable!("4 inputs"),
            },
            Self::ParticleAccelerator(b) => match input_id {
                0 => {
                    b.current_input_fluid_0 = None;
                }
                1 => {
                    b.current_input_material_0 = None;
                }
                2 => {
                    b.current_input_material_1 = None;
                }
                _ => unreachable!("3 inputs"),
            },
            Self::QuantumEncoder(q) => match input_id {
                0 => {
                    q.current_input_fluid_0 = None;
                }
                1 => {
                    q.current_input_material_0 = None;
                }
                2 => {
                    q.current_input_material_1 = None;
                }
                3 => {
                    q.current_input_material_2 = None;
                }
                _ => unreachable!("4 inputs"),
            },
            Self::Converter(c) => match input_id {
                0 => {
                    c.current_input_material_0 = None;
                }
                1 => {
                    c.current_input_material_1 = None;
                }
                _ => unreachable!("2 inputs"),
            },
        }
    }

    pub fn set_current_output_connected(&mut self, output_id: usize) {
        match self {
            Building::Splitter(s) => match output_id {
                0 => {
                    s.output_0_connected = true;
                }
                1 => {
                    s.output_1_connected = true;
                }
                2 => {
                    s.output_2_connected = true;
                }
                _ => unreachable!("3 outputs"),
            },
            Building::PipelineJunction(s) => match output_id {
                0 => {
                    s.output_0_connected = true;
                }
                1 => {
                    s.output_1_connected = true;
                }
                2 => {
                    s.output_2_connected = true;
                }
                3 => {
                    s.output_3_connected = true;
                }
                _ => unreachable!("4 outputs"),
            },
            _ => {}
        }
    }

    pub fn set_current_output_disconnected(&mut self, output_id: usize) {
        match self {
            Building::Splitter(s) => match output_id {
                0 => {
                    s.output_0_connected = false;
                }
                1 => {
                    s.output_1_connected = false;
                }
                2 => {
                    s.output_2_connected = false;
                }
                _ => unreachable!("3 outputs"),
            },
            Building::PipelineJunction(s) => match output_id {
                0 => {
                    s.output_0_connected = false;
                }
                1 => {
                    s.output_1_connected = false;
                }
                2 => {
                    s.output_2_connected = false;
                }
                3 => {
                    s.output_2_connected = false;
                }
                _ => unreachable!("4 outputs"),
            },
            _ => {}
        }
    }
}

fn calc_output(input_size: Option<f32>, duration: f32, output_size: f32, input_base: f32) -> f32 {
    let a = match input_size {
        Some(input_size) => {
            let input_size = (input_size / 60.) * duration;

            // 45/60 * 4secs = 3
            if input_size < input_base {
                input_size / input_base
            } else {
                1.
            }
        }
        None => 1.,
    };

    // 60/4 * 1 = 15
    let b = (60. / duration) * a * output_size;
    round(b)
}

/// Round by satisfactory precision of 6 digits
fn round(x: f32) -> f32 {
    (x * 1_000_000.).round() / 1_000_000.
}

fn calc_output2(
    input_size: Option<(f32, f32)>,
    duration: f32,
    output_size: f32,
    input_base_a: f32,
    input_base_b: f32,
) -> f32 {
    let a = match input_size {
        Some((input_size_a, input_size_b)) => {
            let input_size_a = (input_size_a / 60.) * duration;
            let input_size_b = (input_size_b / 60.) * duration;

            // 45/60 * 4secs = 3
            let a = if input_size_a < input_base_a {
                input_size_a / input_base_a
            } else {
                1.
            };
            let b = if input_size_b < input_base_b {
                input_size_b / input_base_b
            } else {
                1.
            };
            // restrict to the minimum
            if a < b {
                a
            } else {
                b
            }
        }
        None => 1.,
    };

    // 60/4 * 1 = 15
    let b = (60. / duration) * a * output_size;
    round(b)
}

fn calc_output3(
    input_size: Option<(f32, f32, f32)>,
    duration: f32,
    output_size: f32,
    input_base_a: f32,
    input_base_b: f32,
    input_base_c: f32,
) -> f32 {
    let a = match input_size {
        Some((input_size_a, input_size_b, input_size_c)) => {
            let input_size_a = (input_size_a / 60.) * duration;
            let input_size_b = (input_size_b / 60.) * duration;
            let input_size_c = (input_size_c / 60.) * duration;

            // 45/60 * 4secs = 3
            let a = if input_size_a < input_base_a {
                input_size_a / input_base_a
            } else {
                1.
            };
            let b = if input_size_b < input_base_b {
                input_size_b / input_base_b
            } else {
                1.
            };
            let c = if input_size_c < input_base_c {
                input_size_c / input_base_c
            } else {
                1.
            };
            min3(a, b, c)
        }
        None => 1.,
    };

    // 60/4 * 1 = 15
    let b = (60. / duration) * a * output_size;
    round(b)
}

fn calc_output4(
    input_size: Option<(f32, f32, f32, f32)>,
    duration: f32,
    output_size: f32,
    input_base_a: f32,
    input_base_b: f32,
    input_base_c: f32,
    input_base_d: f32,
) -> f32 {
    let a = match input_size {
        Some((input_size_a, input_size_b, input_size_c, input_size_d)) => {
            let input_size_a = (input_size_a / 60.) * duration;
            let input_size_b = (input_size_b / 60.) * duration;
            let input_size_c = (input_size_c / 60.) * duration;
            let input_size_d = (input_size_d / 60.) * duration;

            // 45/60 * 4secs = 3
            let a = if input_size_a < input_base_a {
                input_size_a / input_base_a
            } else {
                1.
            };
            let b = if input_size_b < input_base_b {
                input_size_b / input_base_b
            } else {
                1.
            };
            let c = if input_size_c < input_base_c {
                input_size_c / input_base_c
            } else {
                1.
            };
            let d = if input_size_d < input_base_d {
                input_size_d / input_base_d
            } else {
                1.
            };

            // restrict to the minimum
            min4(a, b, c, d)
        }
        None => 1.,
    };

    // 60/4 * 1 = 15
    let b = (60. / duration) * a * output_size;
    round(b)
}

fn calc_output4_2(
    input_size: Option<(f32, f32, f32, f32)>,
    duration: f32,
    output_size_a: f32,
    output_size_b: f32,
    input_base_a: f32,
    input_base_b: f32,
    input_base_c: f32,
    input_base_d: f32,
) -> (f32, f32) {
    let a = match input_size {
        Some((input_size_a, input_size_b, input_size_c, input_size_d)) => {
            let input_size_a = (input_size_a / 60.) * duration;
            let input_size_b = (input_size_b / 60.) * duration;
            let input_size_c = (input_size_c / 60.) * duration;
            let input_size_d = (input_size_d / 60.) * duration;

            let a = if input_size_a < input_base_a {
                input_size_a / input_base_a
            } else {
                1.
            };
            let b = if input_size_b < input_base_b {
                input_size_b / input_base_b
            } else {
                1.
            };
            let c = if input_size_c < input_base_c {
                input_size_c / input_base_c
            } else {
                1.
            };
            let d = if input_size_d < input_base_d {
                input_size_d / input_base_d
            } else {
                1.
            };

            // restrict to the minimum
            min4(a, b, c, d)
        }
        None => 1.,
    };

    // 60/4 * 1 = 15
    let b_a = round((60. / duration) * a * output_size_a);
    let b_b = round((60. / duration) * a * output_size_b);
    (b_a, b_b)
}

fn calc_output2_2(
    input_size: Option<(f32, f32)>,
    duration: f32,
    output_size_a: f32,
    output_size_b: f32,
    input_base_a: f32,
    input_base_b: f32,
) -> (f32, f32) {
    let a = match input_size {
        Some((input_size_a, input_size_b)) => {
            let input_size_a = (input_size_a / 60.) * duration;
            let input_size_b = (input_size_b / 60.) * duration;

            let a = if input_size_a < input_base_a {
                input_size_a / input_base_a
            } else {
                1.
            };
            let b = if input_size_b < input_base_b {
                input_size_b / input_base_b
            } else {
                1.
            };
            // restrict to the minimum
            if a < b {
                a
            } else {
                b
            }
        }
        None => 1.,
    };

    // 60/4 * 1 = 15
    let b_a = round((60. / duration) * a * output_size_a);
    let b_b = round((60. / duration) * a * output_size_b);
    (b_a, b_b)
}

fn min3(a: f32, b: f32, c: f32) -> f32 {
    let mut sizes = [a, b, c];
    sizes.sort_by(|a, b| {
        if a < b {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sizes[0]
}

fn min4(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut sizes = [a, b, c, d];
    sizes.sort_by(|a, b| {
        if a < b {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sizes[0]
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    strum::VariantArray,
    strum::Display,
)]
pub enum Belt {
    #[strum(to_string = "Mk.1")]
    Mk1,
    #[strum(to_string = "Mk.2")]
    Mk2,
    #[strum(to_string = "Mk.3")]
    Mk3,
    #[strum(to_string = "Mk.4")]
    Mk4,
    #[strum(to_string = "Mk.5")]
    Mk5,
    #[strum(to_string = "Mk.6")]
    Mk6,
}

impl Selectable for Belt {
    const NAME: &'static str = "Belt";

    fn name(&self) -> String {
        self.to_string()
    }

    fn image(&self) -> String {
        let img = match self {
            Self::Mk1 => "Conveyor_Belt_Mk.1.png",
            Self::Mk2 => "Conveyor_Belt_Mk.2.png",
            Self::Mk3 => "Conveyor_Belt_Mk.3.png",
            Self::Mk4 => "Conveyor_Belt_Mk.4.png",
            Self::Mk5 => "Conveyor_Belt_Mk.5.png",
            Self::Mk6 => "Conveyor_Belt_Mk.6.png",
        };
        load_img(img)
    }
}

impl Belt {
    pub fn speed(&self) -> f32 {
        match self {
            Self::Mk1 => 60.,
            Self::Mk2 => 120.,
            Self::Mk3 => 270.,
            Self::Mk4 => 480.,
            Self::Mk5 => 780.,
            Self::Mk6 => 1200.,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    strum::VariantArray,
    strum::Display,
)]
pub enum Pipe {
    #[strum(to_string = "Mk.1")]
    Mk1,
    #[strum(to_string = "Mk.2")]
    Mk2,
}

impl Pipe {
    pub fn name(&self) -> String {
        self.to_string()
    }

    pub fn speed(&self) -> f32 {
        match self {
            Self::Mk1 => 300.,
            Self::Mk2 => 600.,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::Display,
)]
pub enum SomersloopSlot1 {
    #[strum(to_string = "No Somersloop")]
    Empty,
    #[strum(to_string = "1 Somersloop")]
    One,
}

impl SomersloopSlot1 {
    pub fn factor(&self) -> f32 {
        match self {
            Self::Empty => 1.,
            Self::One => 2.,
        }
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::Display,
)]
pub enum SomersloopSlot2 {
    #[strum(to_string = "No Somersloop")]
    Empty,
    #[strum(to_string = "1 Somersloop")]
    One,
    #[strum(to_string = "2 Somersloops")]
    Two,
}

impl SomersloopSlot2 {
    pub fn factor(&self) -> f32 {
        match self {
            Self::Empty => 1.,
            Self::One => 1.5,
            Self::Two => 2.,
        }
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::Display,
)]
pub enum SomersloopSlot4 {
    #[strum(to_string = "No Somersloop")]
    Empty,
    #[strum(to_string = "1 Somersloop")]
    One,
    #[strum(to_string = "2 Somersloops")]
    Two,
    #[strum(to_string = "3 Somersloops")]
    Three,
    #[strum(to_string = "4 Somersloops")]
    Four,
}

impl SomersloopSlot4 {
    pub fn factor(&self) -> f32 {
        match self {
            Self::Empty => 1.,
            Self::One => 1.25,
            Self::Two => 1.5,
            Self::Three => 1.75,
            Self::Four => 2.,
        }
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min4() {
        assert_eq!(min4(1., 2., 3., 4.), 1.);
        assert_eq!(min4(4., 2., 28., 0.03), 0.03);
    }

    #[test]
    fn test_min3() {
        assert_eq!(min3(1., 2., 3.), 1.);
        assert_eq!(min3(4., 2., 0.03), 0.03);
    }
}
