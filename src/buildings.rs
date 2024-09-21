use egui::Color32;

mod constructor;
mod merger;
mod miner;
mod oil_extractor;
mod packager;
mod refinery;
mod smelter;
mod splitter;
mod storage_container;
mod water_extractor;

use crate::util::load_img;

pub use self::constructor::Constructor;
pub use self::merger::Merger;
pub use self::miner::Miner;
pub use self::oil_extractor::OilExtractor;
pub use self::packager::Packager;
pub use self::refinery::Refinery;
pub use self::smelter::Smelter;
pub use self::splitter::Splitter;
pub use self::storage_container::StorageContainer;
pub use self::water_extractor::WaterExtractor;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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
}

#[derive(
    Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, strum::VariantArray,
)]
pub enum ResourceType {
    Bauxite,
    CateriumOre,
    CopperOre,
    IronOre,
    Limestone,
    RawQuartz,
    Sam,
    Sulfur,
    Uranium,
}

impl ResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bauxite => "Bauxite",
            Self::CateriumOre => "Caterium Ore",
            Self::CopperOre => "Copper Ore",
            Self::IronOre => "Iron Ore",
            Self::Limestone => "Limestone",
            Self::RawQuartz => "Raw Quartz",
            Self::Sam => "SAM",
            Self::Sulfur => "Sulfur",
            Self::Uranium => "Uranium",
        }
    }

    pub fn image(&self) -> String {
        self.output_material().image()
    }

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
    ReanimatedSam,
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
}

impl Material {
    pub fn name(&self) -> String {
        self.to_string()
    }

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

    pub fn image(&self) -> String {
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
            Self::ReanimatedSam => "40px-Reanimated_SAM.png",
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
        };
        load_img(name)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
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
        };
        load_img(name)
    }
}

impl Building {
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
            let b = if input_size_a < input_base_b {
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

impl Belt {
    pub fn name(&self) -> String {
        self.to_string()
    }

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
