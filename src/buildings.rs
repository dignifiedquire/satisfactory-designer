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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, strum::Display)]
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
    #[strum(to_string = "Aluminium Scrap")]
    AluminiumScrap,
    #[strum(to_string = "Aluminium Ingot")]
    AluminiumIngot,
    #[strum(to_string = "Bauxite")]
    Bauxite,
    #[strum(to_string = "Limestone")]
    Limestone,
    #[strum(to_string = "Raw Quartz")]
    RawQuartz,
    #[strum(to_string = "Sam Ore")]
    SamOre,
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
    #[strum(to_string = "Aluminium Casing")]
    AluminiumCasing,
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
            Self::AluminiumScrap => "#BCC0C9",
            Self::AluminiumIngot => "#D2D3D4",
            Self::Bauxite => "#CD7660",
            Self::Limestone => "#C8BFA7",
            Self::RawQuartz => "#F177B5",
            Self::SamOre => "#AE1CD7",
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
            Self::AluminiumScrap => "40px-Aluminium_Scrap.png",
            Self::AluminiumIngot => "40px-Aluminium_Ingot.png",
            Self::Bauxite => "40px-Bauxite.png",
            Self::Limestone => "40px-Limestone.png",
            Self::RawQuartz => "40px-Raw_Quartz.png",
            Self::SamOre => "40px-SAM_Ore.png",
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
            Self::Sam => "40px-SAM.png",
            Self::IronRod => "40px-Iron_Rod.png",
            Self::Biomass => "40px-Biomass.png",
            Self::SpitterRemains => "40px-Spitter_Remains.png",
            Self::SteelIngot => "40px-Steel_Ingot.png",
            Self::StingerRemains => "40px-Stinger_Remains.png",
            Self::SteelBeam => "40px-Steel_Beam.png",
            Self::AluminiumCasing => "40px-Aluminium_Casing.png",
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
        };
        format!("file://assets/img/{}", name)
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
    b.round()
}
