use strum::VariantArray;

use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{calc_output, calc_output2, Fluid, Material, Selectable};

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
pub enum PackagerRecipe {
    #[strum(to_string = "Packaged Alumina Solution")]
    PackagedAluminaSolution,
    #[strum(to_string = "Packaged Fuel")]
    PackagedFuel,
    #[strum(to_string = "Packaged Heavy Oil Residue")]
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
    #[strum(to_string = "Unpackage Alumina Solution")]
    UnpackageAluminaSolution,
    #[strum(to_string = "Unpackage Fuel")]
    UnpackageFuel,
    #[strum(to_string = "Unpackage Heavy Oil Residue")]
    UnpackageHeavyOilResidue,
    #[strum(to_string = "Unpackage Ionized Fuel")]
    UnpackageIonizedFuel,
    #[strum(to_string = "Unpackage Liquid Biofuel")]
    UnpackageLiquidBiofuel,
    #[strum(to_string = "Unpackage Nitric Acid")]
    UnpackageNitricAcid,
    #[strum(to_string = "Unpackage Nitrogen Gas")]
    UnpackageNitrogenGas,
    #[strum(to_string = "Unpackage Oil")]
    UnpackageOil,
    #[strum(to_string = "Unpackage Rocket Fuel")]
    UnpackageRocketFuel,
    #[strum(to_string = "Unpackage Sulfuric Acid")]
    UnpackageSulfuricAcid,
    #[strum(to_string = "Unpackage Turbofuel")]
    UnpackageTurbofuel,
    #[strum(to_string = "Unpackage Water")]
    UnpackageWater,
}

impl Selectable for PackagerRecipe {
    const NAME: &'static str = "Recipe";

    fn name(&self) -> String {
        self.to_string()
    }

    fn image(&self) -> String {
        let a = self.output_material().map(|m| m.image());
        let b = self.output_fluid().map(|m| m.image());
        match (a, b) {
            (Some(_image), Some(image)) => image,
            (Some(image), None) => image,
            (None, Some(image)) => image,
            (None, None) => {
                unreachable!("have at least one output")
            }
        }
    }
}

impl PackagerRecipe {
    pub fn input_material(&self) -> Option<Material> {
        match self {
            Self::PackagedAluminaSolution => Some(Material::EmptyCanister),
            Self::PackagedFuel => Some(Material::EmptyCanister),
            Self::PackagedHeavyOilResidue => Some(Material::EmptyCanister),
            Self::PackagedIonizedFuel => Some(Material::EmptyFluidTank),
            Self::PackagedLiquidBiofuel => Some(Material::EmptyCanister),
            Self::PackagedNitricAcid => Some(Material::EmptyFluidTank),
            Self::PackagedNitrogenGas => Some(Material::EmptyFluidTank),
            Self::PackagedOil => Some(Material::EmptyCanister),
            Self::PackagedRocketFuel => Some(Material::EmptyFluidTank),
            Self::PackagedSulfuricAcid => Some(Material::EmptyCanister),
            Self::PackagedTurbofuel => Some(Material::EmptyCanister),
            Self::PackagedWater => Some(Material::EmptyCanister),
            Self::UnpackageAluminaSolution => Some(Material::PackagedAluminaSolution),
            Self::UnpackageFuel => Some(Material::PackagedFuel),
            Self::UnpackageHeavyOilResidue => Some(Material::PackagedHeavyOilResidue),
            Self::UnpackageIonizedFuel => Some(Material::PackagedIonizedFuel),
            Self::UnpackageLiquidBiofuel => Some(Material::PackagedLiquidBiofuel),
            Self::UnpackageNitricAcid => Some(Material::PackagedNitricAcid),
            Self::UnpackageNitrogenGas => Some(Material::PackagedNitrogenGas),
            Self::UnpackageOil => Some(Material::PackagedOil),
            Self::UnpackageRocketFuel => Some(Material::PackagedRocketFuel),
            Self::UnpackageSulfuricAcid => Some(Material::PackagedSulfuricAcid),
            Self::UnpackageTurbofuel => Some(Material::PackagedTurbofuel),
            Self::UnpackageWater => Some(Material::PackagedWater),
        }
    }

    pub fn input_fluid(&self) -> Option<Fluid> {
        match self {
            Self::PackagedAluminaSolution => Some(Fluid::AluminaSolution),
            Self::PackagedFuel => Some(Fluid::Fuel),
            Self::PackagedHeavyOilResidue => Some(Fluid::HeavyOilResidue),
            Self::PackagedIonizedFuel => Some(Fluid::IonizedFuel),
            Self::PackagedLiquidBiofuel => Some(Fluid::LiquidBiofuel),
            Self::PackagedNitricAcid => Some(Fluid::NitricAcid),
            Self::PackagedNitrogenGas => Some(Fluid::NitrogenGas),
            Self::PackagedOil => Some(Fluid::CrudeOil),
            Self::PackagedRocketFuel => Some(Fluid::RocketFuel),
            Self::PackagedSulfuricAcid => Some(Fluid::SulfuricAcid),
            Self::PackagedTurbofuel => Some(Fluid::Turbofuel),
            Self::PackagedWater => Some(Fluid::Water),
            Self::UnpackageAluminaSolution => None,
            Self::UnpackageFuel => None,
            Self::UnpackageHeavyOilResidue => None,
            Self::UnpackageIonizedFuel => None,
            Self::UnpackageLiquidBiofuel => None,
            Self::UnpackageNitricAcid => None,
            Self::UnpackageNitrogenGas => None,
            Self::UnpackageOil => None,
            Self::UnpackageRocketFuel => None,
            Self::UnpackageSulfuricAcid => None,
            Self::UnpackageTurbofuel => None,
            Self::UnpackageWater => None,
        }
    }

    pub fn output_material(&self) -> Option<Material> {
        match self {
            Self::PackagedAluminaSolution => Some(Material::PackagedAluminaSolution),
            Self::PackagedFuel => Some(Material::PackagedFuel),
            Self::PackagedHeavyOilResidue => Some(Material::PackagedHeavyOilResidue),
            Self::PackagedIonizedFuel => Some(Material::PackagedIonizedFuel),
            Self::PackagedLiquidBiofuel => Some(Material::PackagedLiquidBiofuel),
            Self::PackagedNitricAcid => Some(Material::PackagedNitricAcid),
            Self::PackagedNitrogenGas => Some(Material::PackagedNitrogenGas),
            Self::PackagedOil => Some(Material::PackagedOil),
            Self::PackagedRocketFuel => Some(Material::PackagedRocketFuel),
            Self::PackagedSulfuricAcid => Some(Material::PackagedSulfuricAcid),
            Self::PackagedTurbofuel => Some(Material::PackagedTurbofuel),
            Self::PackagedWater => Some(Material::PackagedWater),
            Self::UnpackageAluminaSolution => Some(Material::EmptyCanister),
            Self::UnpackageFuel => Some(Material::EmptyCanister),
            Self::UnpackageHeavyOilResidue => Some(Material::EmptyCanister),
            Self::UnpackageIonizedFuel => Some(Material::EmptyFluidTank),
            Self::UnpackageLiquidBiofuel => Some(Material::EmptyCanister),
            Self::UnpackageNitricAcid => Some(Material::EmptyFluidTank),
            Self::UnpackageNitrogenGas => Some(Material::EmptyFluidTank),
            Self::UnpackageOil => Some(Material::EmptyCanister),
            Self::UnpackageRocketFuel => Some(Material::EmptyFluidTank),
            Self::UnpackageSulfuricAcid => Some(Material::EmptyCanister),
            Self::UnpackageTurbofuel => Some(Material::EmptyCanister),
            Self::UnpackageWater => Some(Material::EmptyCanister),
        }
    }

    pub fn output_fluid(&self) -> Option<Fluid> {
        match self {
            Self::PackagedAluminaSolution => None,
            Self::PackagedFuel => None,
            Self::PackagedHeavyOilResidue => None,
            Self::PackagedIonizedFuel => None,
            Self::PackagedLiquidBiofuel => None,
            Self::PackagedNitricAcid => None,
            Self::PackagedNitrogenGas => None,
            Self::PackagedOil => None,
            Self::PackagedRocketFuel => None,
            Self::PackagedSulfuricAcid => None,
            Self::PackagedTurbofuel => None,
            Self::PackagedWater => None,
            Self::UnpackageAluminaSolution => Some(Fluid::AluminaSolution),
            Self::UnpackageFuel => Some(Fluid::Fuel),
            Self::UnpackageHeavyOilResidue => Some(Fluid::HeavyOilResidue),
            Self::UnpackageIonizedFuel => Some(Fluid::IonizedFuel),
            Self::UnpackageLiquidBiofuel => Some(Fluid::LiquidBiofuel),
            Self::UnpackageNitricAcid => Some(Fluid::NitricAcid),
            Self::UnpackageNitrogenGas => Some(Fluid::NitrogenGas),
            Self::UnpackageOil => Some(Fluid::CrudeOil),
            Self::UnpackageRocketFuel => Some(Fluid::RocketFuel),
            Self::UnpackageSulfuricAcid => Some(Fluid::SulfuricAcid),
            Self::UnpackageTurbofuel => Some(Fluid::Turbofuel),
            Self::UnpackageWater => Some(Fluid::Water),
        }
    }

    pub fn input_material_speed(&self) -> f32 {
        match self {
            Self::PackagedAluminaSolution => 120.,
            Self::PackagedFuel => 40.,
            Self::PackagedHeavyOilResidue => 30.,
            Self::PackagedIonizedFuel => 40.,
            Self::PackagedLiquidBiofuel => 40.,
            Self::PackagedNitricAcid => 30.,
            Self::PackagedNitrogenGas => 60.,
            Self::PackagedOil => 30.,
            Self::PackagedRocketFuel => 60.,
            Self::PackagedSulfuricAcid => 40.,
            Self::PackagedTurbofuel => 20.,
            Self::PackagedWater => 60.,
            Self::UnpackageAluminaSolution => 120.,
            Self::UnpackageFuel => 60.,
            Self::UnpackageHeavyOilResidue => 20.,
            Self::UnpackageIonizedFuel => 40.,
            Self::UnpackageLiquidBiofuel => 60.,
            Self::UnpackageNitricAcid => 20.,
            Self::UnpackageNitrogenGas => 60.,
            Self::UnpackageOil => 60.,
            Self::UnpackageRocketFuel => 60.,
            Self::UnpackageSulfuricAcid => 60.,
            Self::UnpackageTurbofuel => 20.,
            Self::UnpackageWater => 120.,
        }
    }

    pub fn input_fluid_speed(&self) -> f32 {
        match self {
            Self::PackagedAluminaSolution => 120.,
            Self::PackagedFuel => 40.,
            Self::PackagedHeavyOilResidue => 30.,
            Self::PackagedIonizedFuel => 80.,
            Self::PackagedLiquidBiofuel => 40.,
            Self::PackagedNitricAcid => 30.,
            Self::PackagedNitrogenGas => 240.,
            Self::PackagedOil => 30.,
            Self::PackagedRocketFuel => 120.,
            Self::PackagedSulfuricAcid => 40.,
            Self::PackagedTurbofuel => 20.,
            Self::PackagedWater => 60.,
            Self::UnpackageAluminaSolution => 0.,
            Self::UnpackageFuel => 0.,
            Self::UnpackageHeavyOilResidue => 0.,
            Self::UnpackageIonizedFuel => 0.,
            Self::UnpackageLiquidBiofuel => 0.,
            Self::UnpackageNitricAcid => 0.,
            Self::UnpackageNitrogenGas => 0.,
            Self::UnpackageOil => 0.,
            Self::UnpackageRocketFuel => 0.,
            Self::UnpackageSulfuricAcid => 0.,
            Self::UnpackageTurbofuel => 0.,
            Self::UnpackageWater => 0.,
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
            Self::PackagedAluminaSolution => (1., 2., 2., 0., 2.),
            Self::PackagedFuel => (3., 2., 2., 0., 2.),
            Self::PackagedHeavyOilResidue => (4., 2., 2., 0., 2.),
            Self::PackagedIonizedFuel => (3., 4., 2., 0., 2.),
            Self::PackagedLiquidBiofuel => (3., 2., 2., 0., 2.),
            Self::PackagedNitricAcid => (2., 1., 1., 0., 1.),
            Self::PackagedNitrogenGas => (1., 4., 1., 0., 1.),
            Self::PackagedOil => (4., 2., 2., 0., 1.),
            Self::PackagedRocketFuel => (1., 2., 1., 0., 1.),
            Self::PackagedSulfuricAcid => (3., 2., 2., 0., 2.),
            Self::PackagedTurbofuel => (6., 2., 2., 0., 2.),
            Self::PackagedWater => (2., 2., 2., 0., 2.),
            Self::UnpackageAluminaSolution => (1., 0., 2., 2., 2.),
            Self::UnpackageFuel => (2., 0., 2., 2., 2.),
            Self::UnpackageHeavyOilResidue => (6., 0., 2., 2., 2.),
            Self::UnpackageIonizedFuel => (3., 0., 2., 4., 2.),
            Self::UnpackageLiquidBiofuel => (2., 0., 1., 1., 1.),
            Self::UnpackageNitricAcid => (3., 0., 1., 1., 1.),
            Self::UnpackageNitrogenGas => (1., 0., 1., 4., 1.),
            Self::UnpackageOil => (2., 0., 2., 2., 2.),
            Self::UnpackageRocketFuel => (1., 0., 1., 2., 1.),
            Self::UnpackageSulfuricAcid => (1., 0., 1., 1., 1.),
            Self::UnpackageTurbofuel => (6., 0., 2., 2., 2.),
            Self::UnpackageWater => (1., 0., 2., 2., 2.),
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
pub struct Packager {
    pub recipe: Option<PackagerRecipe>,
    pub speed: f32,
    pub current_input_fluid: Option<Input>,
    pub current_input_material: Option<Input>,
}

impl Default for Packager {
    fn default() -> Self {
        Self {
            recipe: None,
            speed: 100.,
            current_input_fluid: None,
            current_input_material: None,
        }
    }
}

impl Packager {
    pub fn clear_clone(&self) -> Self {
        let mut this = self.clone();
        this.current_input_fluid = None;
        this.current_input_material = None;
        this
    }

    pub fn header_image(&self) -> String {
        load_img("Packager.png")
    }

    pub fn available_recipes(&self) -> &'static [PackagerRecipe] {
        PackagerRecipe::VARIANTS
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Packager ({})", r.name()),
            None => "Packager".to_string(),
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

    pub fn input_resource(&self, input_id: usize) -> crate::node::ResourceType {
        match input_id {
            0 => crate::node::ResourceType::Fluid,
            1 => crate::node::ResourceType::Material,
            _ => unreachable!("2 inputs"),
        }
    }

    pub fn output_resource(&self, output_id: usize) -> crate::node::ResourceType {
        match output_id {
            0 => crate::node::ResourceType::Fluid,
            1 => crate::node::ResourceType::Material,
            _ => unreachable!("2 outputs"),
        }
    }

    pub fn input_material_speed(&self) -> f32 {
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.input_material_speed())
            .unwrap_or_default();
        (base as f32 * (self.speed / 100.)).round()
    }

    pub fn input_fluid_speed(&self) -> f32 {
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.input_fluid_speed())
            .unwrap_or_default();
        (base as f32 * (self.speed / 100.)).round()
    }

    pub fn output_material(&self) -> Option<Material> {
        self.recipe.as_ref().and_then(|r| r.output_material())
    }

    pub fn output_fluid(&self) -> Option<Fluid> {
        self.recipe.as_ref().and_then(|r| r.output_fluid())
    }

    pub fn output_material_speed(&self) -> f32 {
        let input_material_speed = self
            .current_input_material
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();
        let input_fluid_speed = self
            .current_input_fluid
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.output_speed_material(input_material_speed, input_fluid_speed))
            .unwrap_or_default();

        // TODO: take speed into account for input_speed

        (base as f32 * (self.speed / 100.)).round()
    }

    pub fn output_fluid_speed(&self) -> f32 {
        let input_material_speed = self
            .current_input_material
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();
        let input_fluid_speed = self
            .current_input_fluid
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.output_speed_fluid(input_material_speed, input_fluid_speed))
            .unwrap_or_default();

        // TODO: take speed into account for input_speed

        (base as f32 * (self.speed / 100.)).round()
    }

    pub fn input_material(&self) -> Option<Material> {
        match self.recipe {
            Some(ref r) => r.input_material(),
            None => None,
        }
    }

    pub fn input_fluid(&self) -> Option<Fluid> {
        match self.recipe {
            Some(ref r) => r.input_fluid(),
            None => None,
        }
    }

    pub fn current_output_fluid(&self) -> Option<Output> {
        self.recipe.and_then(|r| {
            r.output_fluid().map(|output_fluid| Output {
                speed: self.output_fluid_speed(),
                resource: Resource::Fluid(output_fluid),
            })
        })
    }
    pub fn current_output_material(&self) -> Option<Output> {
        self.recipe.and_then(|r| {
            r.output_material().map(|output_material| Output {
                speed: self.output_material_speed(),
                resource: Resource::Material(output_material),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_speed_packaged_water() {
        assert_eq!(
            PackagerRecipe::PackagedWater.output_speed_material(0., 0.),
            0.
        );
        assert_eq!(PackagerRecipe::PackagedWater.output_speed_fluid(0., 0.), 0.);
        assert_eq!(
            PackagerRecipe::PackagedWater.output_speed_material(60., 60.),
            60.
        );
        assert_eq!(
            PackagerRecipe::PackagedWater.output_speed_fluid(60., 60.),
            0.
        );
    }

    #[test]
    fn test_output_speed_unpackage_water() {
        assert_eq!(
            PackagerRecipe::UnpackageWater.output_speed_material(0., 0.),
            0.
        );
        assert_eq!(
            PackagerRecipe::UnpackageWater.output_speed_fluid(0., 0.),
            0.
        );

        assert_eq!(
            PackagerRecipe::UnpackageWater.output_speed_material(120., 0.),
            120.
        );
        assert_eq!(
            PackagerRecipe::UnpackageWater.output_speed_fluid(120., 0.),
            120.
        );

        assert_eq!(
            PackagerRecipe::UnpackageWater.output_speed_material(60., 0.),
            60.,
            "material",
        );
        assert_eq!(
            PackagerRecipe::UnpackageWater.output_speed_fluid(60., 0.),
            60.,
            "fluid"
        );
    }
}
