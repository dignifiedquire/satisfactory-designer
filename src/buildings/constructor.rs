use strum::VariantArray;

use crate::util::load_img;

use super::{calc_output, Material};

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
pub enum ConstructorRecipe {
    #[strum(to_string = "Alien DNA Capsule")]
    AlienDnaCapsule,
    #[strum(to_string = "Aluminum Casing")]
    AluminumCasing,
    #[strum(to_string = "Biomass Alien Protein")]
    BiomassAlienProtein,
    #[strum(to_string = "Biomass (Leaves)")]
    BiomassLeaves,
    #[strum(to_string = "Biomass (Mycelia)")]
    BiomassMycelia,
    #[strum(to_string = "Biomass (Wood)")]
    BiomassWood,
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
    #[strum(to_string = "Hatcher Protein")]
    HatcherProtein,
    #[strum(to_string = "Hog Protein")]
    HogProtein,
    #[strum(to_string = "Iron Plate")]
    IronPlate,
    #[strum(to_string = "Iron Rebar")]
    IronRebar,
    #[strum(to_string = "Iron Rod")]
    IronRod,
    #[strum(to_string = "Power Shard (1)")]
    PowerShard1,
    #[strum(to_string = "Power Shard (2)")]
    PowerShard2,
    #[strum(to_string = "Power Shard (5)")]
    PowerShard5,
    #[strum(to_string = "Quartz Crystal")]
    QuartzCrystal,
    #[strum(to_string = "Quickwire")]
    Quickwire,
    #[strum(to_string = "Reanimated SAM")]
    ReanimatedSam,
    #[strum(to_string = "Screw")]
    Screw,
    #[strum(to_string = "Silica")]
    Silica,
    #[strum(to_string = "Solid Biofuel")]
    SolidBiofuel,
    #[strum(to_string = "Spitter Protein")]
    SpitterProtein,
    #[strum(to_string = "Steel Beam")]
    SteelBeam,
    #[strum(to_string = "Steel Pipe")]
    SteelPipe,
    #[strum(to_string = "Stinger Protein")]
    StingerProtein,
    #[strum(to_string = "Wire")]
    Wire,
    #[strum(to_string = "Aluminum Beam")]
    AluminumBeam,
    #[strum(to_string = "Aluminum Rod")]
    AluminumRod,
    #[strum(to_string = "Biocoal")]
    Biocoal,
    #[strum(to_string = "Cast Screw")]
    CastScrew,
    #[strum(to_string = "Caterium Wire")]
    CateriumWire,
    #[strum(to_string = "Charcoal")]
    Charcoal,
    #[strum(to_string = "Iron Pipe")]
    IronPipe,
    #[strum(to_string = "Iron Wire")]
    IronWire,
    #[strum(to_string = "Steel Canister")]
    SteelCanister,
    #[strum(to_string = "Steel Rod")]
    SteelRod,
    #[strum(to_string = "Steel Screw")]
    SteelScrew,
}

impl ConstructorRecipe {
    pub fn name(&self) -> String {
        self.to_string()
    }

    pub fn image(&self) -> String {
        self.output_material().image()
    }

    pub fn input_material(&self) -> Material {
        match self {
            Self::AlienDnaCapsule => Material::AlienProtein,
            Self::AluminumCasing => Material::AluminumIngot,
            Self::BiomassAlienProtein => Material::AlienProtein,
            Self::BiomassLeaves => Material::Leaves,
            Self::BiomassMycelia => Material::Mycelia,
            Self::BiomassWood => Material::Wood,
            Self::Cable => Material::Wire,
            Self::Concrete => Material::Limestone,
            Self::CopperPowder => Material::CopperIngot,
            Self::CopperSheet => Material::CopperIngot,
            Self::EmptyCanister => Material::Plastic,
            Self::EmptyFluidTank => Material::AluminumIngot,
            Self::FicsiteTrigon => Material::FicsiteIngot,
            Self::HatcherProtein => Material::HatcherRemains,
            Self::HogProtein => Material::HogRemains,
            Self::IronPlate => Material::IronIngot,
            Self::IronRebar => Material::IronRod,
            Self::IronRod => Material::IronIngot,
            Self::PowerShard1 => Material::BluePowerSlug,
            Self::PowerShard2 => Material::YellowPowerSlug,
            Self::PowerShard5 => Material::PurplePowerSlug,
            Self::QuartzCrystal => Material::RawQuartz,
            Self::Quickwire => Material::CateriumIngot,
            Self::ReanimatedSam => Material::Sam,
            Self::Screw => Material::IronRod,
            Self::Silica => Material::RawQuartz,
            Self::SolidBiofuel => Material::Biomass,
            Self::SpitterProtein => Material::SpitterRemains,
            Self::SteelBeam => Material::SteelIngot,
            Self::SteelPipe => Material::SteelIngot,
            Self::StingerProtein => Material::StingerRemains,
            Self::Wire => Material::CopperIngot,
            Self::AluminumBeam => Material::AluminumIngot,
            Self::AluminumRod => Material::AluminumIngot,
            Self::Biocoal => Material::Biomass,
            Self::CastScrew => Material::IronIngot,
            Self::CateriumWire => Material::CateriumIngot,
            Self::Charcoal => Material::Wood,
            Self::IronPipe => Material::IronIngot,
            Self::IronWire => Material::IronIngot,
            Self::SteelCanister => Material::SteelIngot,
            Self::SteelRod => Material::SteelIngot,
            Self::SteelScrew => Material::SteelBeam,
        }
    }

    pub fn output_material(&self) -> Material {
        match self {
            Self::AlienDnaCapsule => Material::AlienDnaCapsule,
            Self::AluminumCasing => Material::AluminumCasing,
            Self::BiomassAlienProtein => Material::Biomass,
            Self::BiomassLeaves => Material::Biomass,
            Self::BiomassMycelia => Material::Biomass,
            Self::BiomassWood => Material::Biomass,
            Self::Cable => Material::Cable,
            Self::Concrete => Material::Concrete,
            Self::CopperPowder => Material::CopperPowder,
            Self::CopperSheet => Material::CopperSheet,
            Self::EmptyCanister => Material::EmptyCanister,
            Self::EmptyFluidTank => Material::EmptyFluidTank,
            Self::FicsiteTrigon => Material::FicsiteTrigon,
            Self::HatcherProtein => Material::AlienProtein,
            Self::HogProtein => Material::AlienProtein,
            Self::IronPlate => Material::IronPlate,
            Self::IronRebar => Material::IronRebar,
            Self::IronRod => Material::IronRod,
            Self::PowerShard1 => Material::PowerShard,
            Self::PowerShard2 => Material::PowerShard,
            Self::PowerShard5 => Material::PowerShard,
            Self::QuartzCrystal => Material::QuartzCrystal,
            Self::Quickwire => Material::Quickwire,
            Self::ReanimatedSam => Material::ReanimatedSam,
            Self::Screw => Material::Screw,
            Self::Silica => Material::Silica,
            Self::SolidBiofuel => Material::SolidBiofuel,
            Self::SpitterProtein => Material::AlienProtein,
            Self::SteelBeam => Material::SteelBeam,
            Self::SteelPipe => Material::SteelPipe,
            Self::StingerProtein => Material::AlienProtein,
            Self::Wire => Material::Wire,
            Self::AluminumBeam => Material::SteelBeam,
            Self::AluminumRod => Material::IronRod,
            Self::Biocoal => Material::Coal,
            Self::CastScrew => Material::Screw,
            Self::CateriumWire => Material::Wire,
            Self::Charcoal => Material::Coal,
            Self::IronPipe => Material::SteelPipe,
            Self::IronWire => Material::Wire,
            Self::SteelCanister => Material::EmptyCanister,
            Self::SteelRod => Material::IronRod,
            Self::SteelScrew => Material::Screw,
        }
    }

    pub fn input_speed(&self) -> f32 {
        match self {
            Self::AlienDnaCapsule => 10.,
            Self::AluminumCasing => 90.,
            Self::BiomassAlienProtein => 15.,
            Self::BiomassLeaves => 120.,
            Self::BiomassMycelia => 15.,
            Self::BiomassWood => 60.,
            Self::Cable => 60.,
            Self::Concrete => 45.,
            Self::CopperPowder => 300.,
            Self::CopperSheet => 20.,
            Self::EmptyCanister => 30.,
            Self::EmptyFluidTank => 60.,
            Self::FicsiteTrigon => 10.,
            Self::HatcherProtein => 20.,
            Self::HogProtein => 20.,
            Self::IronPlate => 30.,
            Self::IronRebar => 15.,
            Self::IronRod => 15.,
            Self::PowerShard1 => 7.5,
            Self::PowerShard2 => 5.,
            Self::PowerShard5 => 2.5,
            Self::QuartzCrystal => 37.5,
            Self::Quickwire => 12.,
            Self::ReanimatedSam => 120.,
            Self::Screw => 10.,
            Self::Silica => 22.5,
            Self::SolidBiofuel => 120.,
            Self::SpitterProtein => 20.,
            Self::SteelBeam => 60.,
            Self::SteelPipe => 30.,
            Self::StingerProtein => 20.,
            Self::Wire => 15.,
            Self::AluminumBeam => 22.5,
            Self::AluminumRod => 7.5,
            Self::Biocoal => 37.5,
            Self::CastScrew => 12.5,
            Self::CateriumWire => 15.,
            Self::Charcoal => 15.,
            Self::IronPipe => 100.,
            Self::IronWire => 12.5,
            Self::SteelCanister => 40.,
            Self::SteelRod => 12.,
            Self::SteelScrew => 5.,
        }
    }

    pub fn max_output_speed(&self) -> f32 {
        self.output_speed_inner(None)
    }

    pub fn output_speed(&self, input_size: f32) -> f32 {
        self.output_speed_inner(Some(input_size))
    }

    fn output_speed_inner(&self, input_size: Option<f32>) -> f32 {
        if input_size == Some(0.) {
            return 0.;
        }

        let (input_base, duration, output_size) = match self {
            Self::AlienDnaCapsule => (1., 6., 1.),
            Self::AluminumCasing => (3., 2., 2.),
            Self::BiomassAlienProtein => (1., 4., 100.),
            Self::BiomassLeaves => (10., 5., 5.),
            Self::BiomassMycelia => (1., 4., 10.),
            Self::BiomassWood => (1., 4., 20.),
            Self::Cable => (2., 2., 1.),
            Self::Concrete => (3., 4., 1.),
            Self::CopperPowder => (30., 6., 5.),
            Self::CopperSheet => (2., 6., 1.),
            Self::EmptyCanister => (2., 4., 4.),
            Self::EmptyFluidTank => (1., 6., 3.),
            Self::FicsiteTrigon => (1., 3., 1.),
            Self::HatcherProtein => (1., 3., 1.),
            Self::HogProtein => (1., 3., 1.),
            Self::IronPlate => (3., 6., 2.),
            Self::IronRebar => (1., 4., 1.),
            Self::IronRod => (1., 4., 1.),
            Self::PowerShard1 => (1., 8., 1.),
            Self::PowerShard2 => (1., 12., 2.),
            Self::PowerShard5 => (1., 24., 5.),
            Self::QuartzCrystal => (5., 8., 3.),
            Self::Quickwire => (1., 5., 5.),
            Self::ReanimatedSam => (4., 2., 1.),
            Self::Screw => (1., 6., 4.),
            Self::Silica => (3., 8., 5.),
            Self::SolidBiofuel => (8., 4., 4.),
            Self::SpitterProtein => (1., 3., 1.),
            Self::SteelBeam => (4., 4., 1.),
            Self::SteelPipe => (3., 6., 2.),
            Self::StingerProtein => (1., 3., 1.),
            Self::Wire => (1., 4., 2.),
            Self::AluminumBeam => (3., 8., 3.),
            Self::AluminumRod => (1., 8., 7.),
            Self::Biocoal => (5., 8., 6.),
            Self::CastScrew => (5., 24., 20.),
            Self::CateriumWire => (1., 4., 8.),
            Self::Charcoal => (1., 4., 10.),
            Self::IronPipe => (20., 12., 5.),
            Self::IronWire => (5., 24., 9.),
            Self::SteelCanister => (4., 6., 4.),
            Self::SteelRod => (1., 5., 4.),
            Self::SteelScrew => (1., 12., 52.),
        };

        calc_output(input_size, duration, output_size, input_base)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Constructor {
    pub recipe: Option<ConstructorRecipe>,
    pub speed: f32,
    pub amplified: bool,
}

impl Default for Constructor {
    fn default() -> Self {
        Self {
            recipe: None,
            speed: 100.,
            amplified: false,
        }
    }
}

impl Constructor {
    pub fn header_image(&self) -> String {
        load_img("Constructor.png")
    }

    pub fn available_recipes(&self) -> &'static [ConstructorRecipe] {
        ConstructorRecipe::VARIANTS
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Constructor ({})", r.name()),
            None => "Constructor".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Constructs things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        1
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_speed(&self) -> f32 {
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.input_speed())
            .unwrap_or_default();
        (base * (self.speed / 100.)).round()
    }

    pub fn output_speed(&self, input_size: f32) -> f32 {
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.output_speed(input_size))
            .unwrap_or_default();
        let amplification = if self.amplified { 2. } else { 1. };

        // TODO: take speed into account for input_size

        (base as f32 * (self.speed / 100.) * amplification).round()
    }

    pub fn input_material(&self) -> Option<Material> {
        self.recipe.as_ref().map(|r| r.input_material())
    }

    pub fn output_material(&self) -> Option<Material> {
        self.recipe.as_ref().map(|r| r.output_material())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_speed() {
        assert_eq!(ConstructorRecipe::SteelPipe.output_speed(0.), 0.);
        assert_eq!(ConstructorRecipe::SteelPipe.output_speed(15.), 10.);
        assert_eq!(ConstructorRecipe::SteelPipe.output_speed(30.), 20.);
        assert_eq!(ConstructorRecipe::SteelPipe.output_speed(60.), 20.);

        assert_eq!(ConstructorRecipe::SteelPipe.max_output_speed(), 20.);
    }
}
