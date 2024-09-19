use egui::Color32;

mod constructor;
mod merger;
mod miner;
mod smelter;
mod splitter;

pub use self::constructor::Constructor;
pub use self::merger::Merger;
pub use self::miner::{Miner, MinerLevel, ResourcePurity};
pub use self::smelter::{Smelter, SmelterRecipie};
pub use self::splitter::Splitter;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum Building {
    Miner(Miner),
    Smelter(Smelter),
    Splitter(Splitter),
    Merger(Merger),
    Constructor(Constructor),
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ResourceType {
    Bauxite,
    CateriumOre,
    CopperOre,
    IronOre,
    Limestone,
    RawQuartz,
    SamOre,
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
            Self::SamOre => "SAM Ore",
            Self::Sulfur => "Sulfur",
            Self::Uranium => "Uranium",
        }
    }

    pub fn output_material(&self) -> Material {
        match self {
            Self::Bauxite => Material::Bauxite,
            Self::CateriumOre => Material::CateriumOre,
            Self::CopperOre => Material::CopperOre,
            Self::IronOre => Material::IronOre,
            Self::Limestone => Material::Limestone,
            Self::RawQuartz => Material::RawQuartz,
            Self::SamOre => Material::SamOre,
            Self::Sulfur => Material::Sulfur,
            Self::Uranium => Material::Uranium,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Material {
    CopperOre,
    IronOre,
    CateriumOre,
    CopperIngot,
    IronIngot,
    CateriumIngot,
    AluminiumScrap,
    AluminiumIngot,
    Bauxite,
    Limestone,
    RawQuartz,
    SamOre,
    Sulfur,
    Uranium,
    AlienProtein,
    AlienDnaCapsule,
    Leaves,
    Mycelia,
    Wood,
    Wire,
    Plastic,
    FicsiteIngot,
    HatcherRemains,
    HogRemains,
    BluePowerSlug,
    YellowPowerSlug,
    PurplePowerSlug,
    Sam,
    IronRod,
    Biomass,
    SpitterRemains,
    SteelIngot,
    StingerRemains,
    SteelBeam,
    AluminiumCasing,
    Cable,
    Concrete,
    CopperPowder,
    CopperSheet,
    EmptyCanister,
    EmptyFluidTank,
    FicsiteTrigon,
    IronPlate,
    IronRebar,
    PowerShard,
    QuartzCrystal,
    ReanimatedSam,
    Screw,
    Silica,
    SolidBiofuel,
    SteelPipe,
    Coal,
    Quickwire,
}

impl Material {
    pub fn color(&self) -> Color32 {
        // Colors based on https://www.reddit.com/r/SatisfactoryGame/comments/154vft6/vencams_colour_list_25/
        match self {
            Self::CopperOre => Color32::from_hex("#BD4C39").unwrap(),
            Self::IronOre => Color32::from_hex("#8E5C5C").unwrap(),
            Self::CateriumOre => Color32::from_hex("#E2B148").unwrap(),
            Self::CopperIngot => Color32::from_hex("#A56355").unwrap(),
            Self::IronIngot => Color32::from_hex("#989A9D").unwrap(),
            Self::CateriumIngot => Color32::from_hex("#CCA566").unwrap(),
            Self::AluminiumScrap => Color32::from_hex("#BCC0C9").unwrap(),
            Self::AluminiumIngot => Color32::from_hex("#D2D3D4").unwrap(),
            Self::Bauxite => Color32::from_hex("#CD7660").unwrap(),
            Self::Limestone => Color32::from_hex("#C8BFA7").unwrap(),
            Self::RawQuartz => Color32::from_hex("#F177B5").unwrap(),
            Self::SamOre => Color32::from_hex("#AE1CD7").unwrap(), // TODO: better color
            Self::Sulfur => Color32::from_hex("#FCDC48").unwrap(),
            Self::Uranium => Color32::from_hex("#88D288").unwrap(),
            _ => Color32::from_hex("#697082").unwrap(),
        }
    }
}

impl Building {
    pub fn header_image(&self) -> &'static str {
        match self {
            Self::Miner(m) => m.header_image(),
            Self::Smelter(s) => s.header_image(),
            Self::Splitter(s) => s.header_image(),
            Self::Merger(s) => s.header_image(),
            Self::Constructor(s) => s.header_image(),
        }
    }

    pub fn input_material(&self) -> Option<Material> {
        match self {
            Self::Miner(m) => m.input_material(),
            Self::Smelter(s) => s.input_material(),
            Self::Splitter(s) => s.input_material(),
            Self::Merger(s) => s.input_material(),
            Self::Constructor(s) => s.input_material(),
        }
    }

    pub fn outputs(&self) -> usize {
        match self {
            Self::Miner(m) => m.num_outputs(),
            Self::Smelter(s) => s.num_outputs(),
            Self::Splitter(s) => s.num_outputs(),
            Self::Merger(s) => s.num_outputs(),
            Self::Constructor(s) => s.num_outputs(),
        }
    }

    pub fn inputs(&self) -> usize {
        match self {
            Self::Miner(m) => m.num_inputs(),
            Self::Smelter(s) => s.num_inputs(),
            Self::Splitter(s) => s.num_inputs(),
            Self::Merger(s) => s.num_inputs(),
            Self::Constructor(s) => s.num_inputs(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Miner(m) => m.name(),
            Self::Smelter(s) => s.name(),
            Self::Splitter(s) => s.name(),
            Self::Merger(s) => s.name(),
            Self::Constructor(s) => s.name(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::Miner(m) => m.description(),
            Self::Smelter(s) => s.description(),
            Self::Splitter(s) => s.description(),
            Self::Merger(s) => s.description(),
            Self::Constructor(s) => s.description(),
        }
    }
}

fn calc_output(input_size: Option<usize>, duration: f32, _output_size: f32, input_base: f32) -> f32 {
    let a = match input_size {
        Some(input_size) => {
            let input_size = (input_size as f32 / 60.) * duration;

            // 45/60 * 4secs = 3
            if input_size < input_base {
                input_size / input_base
            } else {
                1.
            }
        }
        None => 1.
    };

    // 60/4 * 1 = 15
    let b = (60. / duration) * a;
    b.round()
}
