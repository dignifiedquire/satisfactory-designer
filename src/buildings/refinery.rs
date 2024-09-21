use strum::VariantArray;

use crate::util::load_img;

use super::{calc_output, calc_output2, round, Fluid, Material};

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
pub enum RefineryRecipe {
    #[strum(to_string = "Alumina Solution")]
    AluminaSolution,
    #[strum(to_string = "Aluminum Scrap")]
    AluminumScrap,
    #[strum(to_string = "Fuel")]
    Fuel,
    #[strum(to_string = "Ionized Fuel")]
    IonizedFuel,
    #[strum(to_string = "Liquid Biofuel")]
    LiquidBiofuel,
    #[strum(to_string = "Petroleum Coke")]
    PetroleumCoke,
    #[strum(to_string = "Plastic")]
    Plastic,
    #[strum(to_string = "Residual Fuel")]
    ResidualFuel,
    #[strum(to_string = "Residual Plastic")]
    ResidualPlastic,
    #[strum(to_string = "Residual Rubber")]
    ResidualRubber,
    #[strum(to_string = "Rubber")]
    Rubber,
    #[strum(to_string = "Smokeless Powder")]
    SmokelessPowder,
    #[strum(to_string = "Sulfuric Acid")]
    SulfuricAcid,
    #[strum(to_string = "Coated Cable")]
    CoatedCable,
    #[strum(to_string = "Diluted Packaged Fuel")]
    DilutedPackagedFuel,
    #[strum(to_string = "Electrode Aluminum Scrap")]
    ElectrodeAluminumScrap,
    #[strum(to_string = "Heavy Oi lResidue")]
    HeavyOilResidue,
    #[strum(to_string = "Leached Caterium Ingot")]
    LeachedCateriumIngot,
    #[strum(to_string = "Leached Copper Ingot")]
    LeachedCopperIngot,
    #[strum(to_string = "Leached Ironingot")]
    LeachedIroningot,
    #[strum(to_string = "Polyester Fabric")]
    PolyesterFabric,
    #[strum(to_string = "Polymer Resin")]
    PolymerResin,
    #[strum(to_string = "Pure Caterium Ingot")]
    PureCateriumIngot,
    #[strum(to_string = "Pure Copper Ingot")]
    PureCopperIngot,
    #[strum(to_string = "Pure Iron Ingot")]
    PureIronIngot,
    #[strum(to_string = "Pure Quartz Crystal")]
    PureQuartzCrystal,
    #[strum(to_string = "Quartz Purification")]
    QuartzPurification,
    #[strum(to_string = "Recycled Plastic")]
    RecycledPlastic,
    #[strum(to_string = "Recycled Rubber")]
    RecycledRubber,
    #[strum(to_string = "Sloppy Alumina")]
    SloppyAlumina,
    #[strum(to_string = "Steamed Copper Sheet")]
    SteamedCopperSheet,
    #[strum(to_string = "Turbo Heavy Fuel")]
    TurboHeavyFuel,
    #[strum(to_string = "Turbofuel")]
    Turbofuel,
    #[strum(to_string = "Wet Concrete")]
    WetConcrete,
}

impl RefineryRecipe {
    pub fn name(&self) -> String {
        self.to_string()
    }

    pub fn image(&self) -> (Option<String>, Option<String>) {
        let a = self.output_material().map(|m| m.image());
        let b = self.output_fluid().map(|m| m.image());
        (a, b)
    }

    pub fn input_material(&self) -> Option<Material> {
        match self {
            Self::AluminaSolution => Some(Material::Bauxite),
            Self::AluminumScrap => Some(Material::Coal),
            Self::Fuel => None,
            Self::IonizedFuel => Some(Material::PowerShard),
            Self::LiquidBiofuel => Some(Material::SolidBiofuel),
            Self::PetroleumCoke => None,
            Self::Plastic => None,
            Self::ResidualFuel => None,
            Self::ResidualPlastic => Some(Material::PolymerResin),
            Self::ResidualRubber => Some(Material::PolymerResin),
            Self::Rubber => None,
            Self::SmokelessPowder => Some(Material::BlackPowder),
            Self::SulfuricAcid => Some(Material::Sulfur),
            Self::CoatedCable => Some(Material::Wire),
            Self::DilutedPackagedFuel => Some(Material::PackagedWater),
            Self::ElectrodeAluminumScrap => Some(Material::PetroleumCoke),
            Self::HeavyOilResidue => None,
            Self::LeachedCateriumIngot => Some(Material::CateriumOre),
            Self::LeachedCopperIngot => Some(Material::CopperOre),
            Self::LeachedIroningot => Some(Material::IronOre),
            Self::PolyesterFabric => Some(Material::PolymerResin),
            Self::PolymerResin => None,
            Self::PureCateriumIngot => Some(Material::CateriumOre),
            Self::PureCopperIngot => Some(Material::CopperOre),
            Self::PureIronIngot => Some(Material::IronOre),
            Self::PureQuartzCrystal => Some(Material::RawQuartz),
            Self::QuartzPurification => Some(Material::RawQuartz),
            Self::RecycledPlastic => Some(Material::Rubber),
            Self::RecycledRubber => Some(Material::Plastic),
            Self::SloppyAlumina => Some(Material::Bauxite),
            Self::SteamedCopperSheet => Some(Material::CopperIngot),
            Self::TurboHeavyFuel => Some(Material::CompactedCoal),
            Self::Turbofuel => Some(Material::CompactedCoal),
            Self::WetConcrete => Some(Material::Limestone),
        }
    }

    pub fn input_fluid(&self) -> Option<Fluid> {
        match self {
            Self::AluminaSolution => Some(Fluid::Water),
            Self::AluminumScrap => Some(Fluid::AluminaSolution),
            Self::Fuel => Some(Fluid::CrudeOil),
            Self::IonizedFuel => Some(Fluid::RocketFuel),
            Self::LiquidBiofuel => Some(Fluid::Water),
            Self::PetroleumCoke => Some(Fluid::HeavyOilResidue),
            Self::Plastic => Some(Fluid::CrudeOil),
            Self::ResidualFuel => Some(Fluid::HeavyOilResidue),
            Self::ResidualPlastic => Some(Fluid::Water),
            Self::ResidualRubber => Some(Fluid::Water),
            Self::Rubber => Some(Fluid::CrudeOil),
            Self::SmokelessPowder => Some(Fluid::HeavyOilResidue),
            Self::SulfuricAcid => Some(Fluid::Water),
            Self::CoatedCable => Some(Fluid::HeavyOilResidue),
            Self::DilutedPackagedFuel => Some(Fluid::HeavyOilResidue),
            Self::ElectrodeAluminumScrap => Some(Fluid::AluminaSolution),
            Self::HeavyOilResidue => Some(Fluid::CrudeOil),
            Self::LeachedCateriumIngot => Some(Fluid::SulfuricAcid),
            Self::LeachedCopperIngot => Some(Fluid::SulfuricAcid),
            Self::LeachedIroningot => Some(Fluid::SulfuricAcid),
            Self::PolyesterFabric => Some(Fluid::Water),
            Self::PolymerResin => Some(Fluid::CrudeOil),
            Self::PureCateriumIngot => Some(Fluid::Water),
            Self::PureCopperIngot => Some(Fluid::Water),
            Self::PureIronIngot => Some(Fluid::Water),
            Self::PureQuartzCrystal => Some(Fluid::Water),
            Self::QuartzPurification => Some(Fluid::NitricAcid),
            Self::RecycledPlastic => Some(Fluid::Fuel),
            Self::RecycledRubber => Some(Fluid::Fuel),
            Self::SloppyAlumina => Some(Fluid::Water),
            Self::SteamedCopperSheet => Some(Fluid::Water),
            Self::TurboHeavyFuel => Some(Fluid::HeavyOilResidue),
            Self::Turbofuel => Some(Fluid::Fuel),
            Self::WetConcrete => Some(Fluid::Water),
        }
    }

    pub fn output_material(&self) -> Option<Material> {
        match self {
            Self::AluminaSolution => Some(Material::Silica),
            Self::AluminumScrap => Some(Material::AluminumScrap),
            Self::Fuel => Some(Material::PolymerResin),
            Self::IonizedFuel => Some(Material::CompactedCoal),
            Self::LiquidBiofuel => None,
            Self::PetroleumCoke => Some(Material::PetroleumCoke),
            Self::Plastic => Some(Material::Plastic),
            Self::ResidualFuel => None,
            Self::ResidualPlastic => Some(Material::Plastic),
            Self::ResidualRubber => Some(Material::Rubber),
            Self::Rubber => Some(Material::Rubber),
            Self::SmokelessPowder => Some(Material::SmokelessPowder),
            Self::SulfuricAcid => None,
            Self::CoatedCable => Some(Material::Cable),
            Self::DilutedPackagedFuel => Some(Material::PackagedFuel),
            Self::ElectrodeAluminumScrap => Some(Material::AluminumScrap),
            Self::HeavyOilResidue => Some(Material::PolymerResin),
            Self::LeachedCateriumIngot => Some(Material::CateriumIngot),
            Self::LeachedCopperIngot => Some(Material::CopperIngot),
            Self::LeachedIroningot => Some(Material::IronIngot),
            Self::PolyesterFabric => Some(Material::Fabric),
            Self::PolymerResin => Some(Material::PolymerResin),
            Self::PureCateriumIngot => Some(Material::CateriumIngot),
            Self::PureCopperIngot => Some(Material::CopperIngot),
            Self::PureIronIngot => Some(Material::IronIngot),
            Self::PureQuartzCrystal => Some(Material::QuartzCrystal),
            Self::QuartzPurification => Some(Material::QuartzCrystal),
            Self::RecycledPlastic => Some(Material::Plastic),
            Self::RecycledRubber => Some(Material::Rubber),
            Self::SloppyAlumina => None,
            Self::SteamedCopperSheet => Some(Material::CopperSheet),
            Self::TurboHeavyFuel => None,
            Self::Turbofuel => None,
            Self::WetConcrete => Some(Material::Concrete),
        }
    }

    pub fn output_fluid(&self) -> Option<Fluid> {
        match self {
            Self::AluminaSolution => Some(Fluid::AluminaSolution),
            Self::AluminumScrap => Some(Fluid::Water),
            Self::Fuel => Some(Fluid::Fuel),
            Self::IonizedFuel => Some(Fluid::IonizedFuel),
            Self::LiquidBiofuel => Some(Fluid::LiquidBiofuel),
            Self::PetroleumCoke => None,
            Self::Plastic => Some(Fluid::HeavyOilResidue),
            Self::ResidualFuel => Some(Fluid::Fuel),
            Self::ResidualPlastic => None,
            Self::ResidualRubber => None,
            Self::Rubber => Some(Fluid::HeavyOilResidue),
            Self::SmokelessPowder => None,
            Self::SulfuricAcid => Some(Fluid::SulfuricAcid),
            Self::CoatedCable => None,
            Self::DilutedPackagedFuel => None,
            Self::ElectrodeAluminumScrap => Some(Fluid::Water),
            Self::HeavyOilResidue => Some(Fluid::HeavyOilResidue),
            Self::LeachedCateriumIngot => None,
            Self::LeachedCopperIngot => None,
            Self::LeachedIroningot => None,
            Self::PolyesterFabric => None,
            Self::PolymerResin => Some(Fluid::HeavyOilResidue),
            Self::PureCateriumIngot => None,
            Self::PureCopperIngot => None,
            Self::PureIronIngot => None,
            Self::PureQuartzCrystal => None,
            Self::QuartzPurification => Some(Fluid::DissolvedSilica),
            Self::RecycledPlastic => None,
            Self::RecycledRubber => None,
            Self::SloppyAlumina => Some(Fluid::AluminaSolution),
            Self::SteamedCopperSheet => None,
            Self::TurboHeavyFuel => Some(Fluid::Turbofuel),
            Self::Turbofuel => Some(Fluid::Turbofuel),
            Self::WetConcrete => None,
        }
    }

    pub fn input_material_speed(&self) -> f32 {
        match self {
            Self::AluminaSolution => 120.,
            Self::AluminumScrap => 120.,
            Self::Fuel => 0.,
            Self::IonizedFuel => 2.5,
            Self::LiquidBiofuel => 90.,
            Self::PetroleumCoke => 0.,
            Self::Plastic => 0.,
            Self::ResidualFuel => 0.,
            Self::ResidualPlastic => 60.,
            Self::ResidualRubber => 40.,
            Self::Rubber => 0.,
            Self::SmokelessPowder => 20.,
            Self::SulfuricAcid => 50.,
            Self::CoatedCable => 37.5,
            Self::DilutedPackagedFuel => 60.,
            Self::ElectrodeAluminumScrap => 60.,
            Self::HeavyOilResidue => 0.,
            Self::LeachedCateriumIngot => 54.,
            Self::LeachedCopperIngot => 45.,
            Self::LeachedIroningot => 50.,
            Self::PolyesterFabric => 30.,
            Self::PolymerResin => 0.,
            Self::PureCateriumIngot => 24.,
            Self::PureCopperIngot => 15.,
            Self::PureIronIngot => 35.,
            Self::PureQuartzCrystal => 67.5,
            Self::QuartzPurification => 120.,
            Self::RecycledPlastic => 30.,
            Self::RecycledRubber => 30.,
            Self::SloppyAlumina => 200.,
            Self::SteamedCopperSheet => 22.5,
            Self::TurboHeavyFuel => 30.,
            Self::Turbofuel => 15.,
            Self::WetConcrete => 120.,
        }
    }

    pub fn input_fluid_speed(&self) -> f32 {
        match self {
            Self::AluminaSolution => 180.,
            Self::AluminumScrap => 240.,
            Self::Fuel => 60.,
            Self::IonizedFuel => 40.,
            Self::LiquidBiofuel => 45.,
            Self::PetroleumCoke => 40.,
            Self::Plastic => 30.,
            Self::ResidualFuel => 60.,
            Self::ResidualPlastic => 20.,
            Self::ResidualRubber => 40.,
            Self::Rubber => 30.,
            Self::SmokelessPowder => 10.,
            Self::SulfuricAcid => 50.,
            Self::CoatedCable => 15.,
            Self::DilutedPackagedFuel => 30.,
            Self::ElectrodeAluminumScrap => 180.,
            Self::HeavyOilResidue => 30.,
            Self::LeachedCateriumIngot => 30.,
            Self::LeachedCopperIngot => 25.,
            Self::LeachedIroningot => 10.,
            Self::PolyesterFabric => 30.,
            Self::PolymerResin => 30.,
            Self::PureCateriumIngot => 60.,
            Self::PureCopperIngot => 24.,
            Self::PureIronIngot => 10.,
            Self::PureQuartzCrystal => 37.5,
            Self::QuartzPurification => 10.,
            Self::RecycledPlastic => 30.,
            Self::RecycledRubber => 30.,
            Self::SloppyAlumina => 200.,
            Self::SteamedCopperSheet => 22.5,
            Self::TurboHeavyFuel => 37.5,
            Self::Turbofuel => 22.5,
            Self::WetConcrete => 100.,
        }
    }

    pub fn max_output_speed_material(&self) -> f32 {
        if self.output_material().is_none() {
            return 0.;
        }

        self.output_speed_inner(None)
    }

    pub fn max_output_speed_fluid(&self) -> f32 {
        if self.output_fluid().is_none() {
            return 0.;
        }

        self.output_speed_inner(None)
    }

    pub fn output_speed_material(&self, input_material_size: f32, input_fluid_size: f32) -> f32 {
        if self.output_material().is_none() {
            return 0.;
        }

        self.output_speed_inner(Some((input_material_size, input_fluid_size)))
    }

    pub fn output_speed_fluid(&self, input_material_size: f32, input_fluid_size: f32) -> f32 {
        if self.output_fluid().is_none() {
            return 0.;
        }

        self.output_speed_inner(Some((input_material_size, input_fluid_size)))
    }

    // returns
    // (duration, fluid_input, material_input, fluid_output, material_output)
    fn stats(&self) -> (f32, f32, f32, f32, f32) {
        match self {
            Self::AluminaSolution => (6., 18., 12., 12., 5.),
            Self::AluminumScrap => (1., 4., 2., 2., 6.),
            Self::Fuel => (6., 6., 0., 4., 3.),
            Self::IonizedFuel => (24., 16., 1., 16., 2.),
            Self::LiquidBiofuel => (4., 3., 6., 4., 0.),
            Self::PetroleumCoke => (6., 4., 0., 0., 12.),
            Self::Plastic => (6., 3., 0., 1., 2.),
            Self::ResidualFuel => (6., 6., 0., 4., 0.),
            Self::ResidualPlastic => (6., 2., 6., 0., 2.),
            Self::ResidualRubber => (6., 4., 4., 0., 2.),
            Self::Rubber => (6., 3., 0., 2., 2.),
            Self::SmokelessPowder => (6., 1., 2., 0., 2.),
            Self::SulfuricAcid => (6., 5., 5., 5., 0.),
            Self::CoatedCable => (8., 2., 5., 0., 9.),
            Self::DilutedPackagedFuel => (2., 1., 2., 0., 2.),
            Self::ElectrodeAluminumScrap => (4., 12., 4., 7., 20.),
            Self::HeavyOilResidue => (6., 3., 0., 4., 2.),
            Self::LeachedCateriumIngot => (10., 5., 9., 0., 6.),
            Self::LeachedCopperIngot => (12., 5., 9., 0., 22.),
            Self::LeachedIroningot => (6., 1., 5., 0., 22.),
            Self::PolyesterFabric => (2., 1., 1., 0., 1.),
            Self::PolymerResin => (6., 6., 0., 2., 13.),
            Self::PureCateriumIngot => (5., 2., 2., 0., 1.),
            Self::PureCopperIngot => (23., 4., 6., 0., 15.),
            Self::PureIronIngot => (12., 4., 7., 0., 13.),
            Self::PureQuartzCrystal => (8., 5., 9., 0., 7.),
            Self::QuartzPurification => (12., 2., 24., 12., 15.),
            Self::RecycledPlastic => (12., 6., 6., 0., 12.),
            Self::RecycledRubber => (12., 6., 6., 0., 12.),
            Self::SloppyAlumina => (3., 10., 10., 12., 0.),
            Self::SteamedCopperSheet => (8., 3., 3., 0., 3.),
            Self::TurboHeavyFuel => (8., 5., 4., 4., 0.),
            Self::Turbofuel => (16., 6., 4., 5., 0.),
            Self::WetConcrete => (3., 5., 6., 0., 4.),
        }
    }

    fn output_speed_inner(&self, input_size: Option<(f32, f32)>) -> f32 {
        let input_material_size = input_size.map(|(a, _)| a);
        let input_fluid_size = input_size.map(|(_, b)| b);

        let (
            duration,
            fluid_input_size,
            material_input_size,
            _fluid_output_size,
            material_output_size,
        ) = self.stats();

        match (
            self.input_material().is_some(),
            self.input_fluid().is_some(),
        ) {
            (true, true) => {
                if input_fluid_size == Some(0.0) || input_material_size == Some(0.0) {
                    return 0.;
                }

                calc_output2(
                    input_size,
                    duration,
                    material_output_size,
                    material_input_size,
                    fluid_input_size,
                )
            }
            (false, true) => {
                if input_fluid_size == Some(0.0) {
                    return 0.;
                }

                calc_output(
                    input_fluid_size,
                    duration,
                    material_output_size,
                    fluid_input_size,
                )
            }
            (true, false) => {
                if input_material_size == Some(0.0) {
                    return 0.;
                }

                calc_output(
                    input_material_size,
                    duration,
                    material_output_size,
                    material_input_size,
                )
            }
            (false, false) => unreachable!("always one input"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Refinery {
    pub recipe: Option<RefineryRecipe>,
    pub speed: f32,
    pub amplified: bool,
}

impl Default for Refinery {
    fn default() -> Self {
        Self {
            recipe: None,
            speed: 100.,
            amplified: false,
        }
    }
}

impl Refinery {
    pub fn header_image(&self) -> String {
        load_img("Refinery.png")
    }

    pub fn available_recipes(&self) -> &'static [RefineryRecipe] {
        RefineryRecipe::VARIANTS
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Refinery ({})", r.name()),
            None => "Refinery".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Smelts things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        2
    }

    pub fn num_outputs(&self) -> usize {
        2
    }

    pub fn output_material_speed(&self, input_material_size: f32, input_fluid_size: f32) -> f32 {
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.output_speed_material(input_material_size, input_fluid_size))
            .unwrap_or_default();
        let amplification = if self.amplified { 2. } else { 1. };

        // TODO: take speed into account for input_size

        round(base as f32 * (self.speed / 100.) * amplification)
    }

    pub fn output_fluid_speed(&self, input_material_size: f32, input_fluid_size: f32) -> f32 {
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.output_speed_fluid(input_material_size, input_fluid_size))
            .unwrap_or_default();
        let amplification = if self.amplified { 2. } else { 1. };

        // TODO: take speed into account for input_size

        round(base as f32 * (self.speed / 100.) * amplification)
    }

    pub fn output_material(&self) -> Option<Material> {
        self.recipe.as_ref().and_then(|r| r.output_material())
    }

    pub fn output_fluid(&self) -> Option<Fluid> {
        self.recipe.as_ref().and_then(|r| r.output_fluid())
    }

    pub fn input_material(&self) -> Option<Material> {
        self.recipe.as_ref().and_then(|r| r.input_material())
    }
    pub fn input_fluid(&self) -> Option<Fluid> {
        self.recipe.as_ref().and_then(|r| r.input_fluid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_speed_steamed_copper_sheet() {
        assert_eq!(
            RefineryRecipe::SteamedCopperSheet.output_speed_material(0., 0.),
            0.
        );
        assert_eq!(
            RefineryRecipe::SteamedCopperSheet.output_speed_fluid(0., 0.),
            0.
        );
        assert_eq!(
            RefineryRecipe::SteamedCopperSheet.output_speed_material(22.5, 22.5),
            22.5
        );
        assert_eq!(
            RefineryRecipe::SteamedCopperSheet.output_speed_fluid(22.5, 22.5),
            0.
        );
    }
}
