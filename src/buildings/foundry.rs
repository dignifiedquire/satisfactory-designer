use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{calc_output2, round, Material, Selectable, SomersloopSlot2};

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
pub enum FoundryRecipe {
    #[strum(to_string = "Aluminum Ingot")]
    AluminumIngot,
    #[strum(to_string = "Steel Ingot")]
    SteelIngot,
    #[strum(to_string = "Basic Iron Ingot")]
    BasicIronIngot,
    #[strum(to_string = "Coke Steel Ingot")]
    CokeSteelIngot,
    #[strum(to_string = "Compacted Steel Ingot")]
    CompactedSteelIngot,
    #[strum(to_string = "Copper Alloy Ingot")]
    CopperAlloyIngot,
    #[strum(to_string = "Fused Quartz Crystal")]
    FusedQuartzCrystal,
    #[strum(to_string = "Iron Alloy Ingot")]
    IronAlloyIngot,
    #[strum(to_string = "Molded Beam")]
    MoldedBeam,
    #[strum(to_string = "Molded Steel Pipe")]
    MoldedSteelPipe,
    #[strum(to_string = "Solid Steel Ingot")]
    SolidSteelIngot,
    #[strum(to_string = "Steel Cast Plate")]
    SteelCastPlate,
    #[strum(to_string = "Tempered Caterium Ingot")]
    TemperedCateriumIngot,
    #[strum(to_string = "Tempered Copper Ingot")]
    TemperedCopperIngot,
}

impl Selectable for FoundryRecipe {
    const NAME: &'static str = "Recipe";

    fn name(&self) -> String {
        self.to_string()
    }

    fn image(&self) -> String {
        self.output_material().image()
    }
}

impl FoundryRecipe {
    pub fn input_material(&self) -> (Material, Material) {
        match self {
            Self::AluminumIngot => (Material::AluminumScrap, Material::Silica),
            Self::SteelIngot => (Material::IronOre, Material::Coal),
            Self::BasicIronIngot => (Material::IronOre, Material::Limestone),
            Self::CokeSteelIngot => (Material::IronOre, Material::PetroleumCoke),
            Self::CompactedSteelIngot => (Material::IronOre, Material::CompactedCoal),
            Self::CopperAlloyIngot => (Material::CopperOre, Material::IronOre),
            Self::FusedQuartzCrystal => (Material::RawQuartz, Material::Coal),
            Self::IronAlloyIngot => (Material::IronOre, Material::CopperOre),
            Self::MoldedBeam => (Material::SteelIngot, Material::Concrete),
            Self::MoldedSteelPipe => (Material::SteelIngot, Material::Concrete),
            Self::SolidSteelIngot => (Material::IronIngot, Material::Coal),
            Self::SteelCastPlate => (Material::IronIngot, Material::SteelIngot),
            Self::TemperedCateriumIngot => (Material::CateriumOre, Material::PetroleumCoke),
            Self::TemperedCopperIngot => (Material::CopperOre, Material::PetroleumCoke),
        }
    }
    pub fn output_material(&self) -> Material {
        match self {
            Self::AluminumIngot => Material::AluminumIngot,
            Self::SteelIngot => Material::SteelIngot,
            Self::BasicIronIngot => Material::IronIngot,
            Self::CokeSteelIngot => Material::SteelIngot,
            Self::CompactedSteelIngot => Material::SteelIngot,
            Self::CopperAlloyIngot => Material::CopperIngot,
            Self::FusedQuartzCrystal => Material::QuartzCrystal,
            Self::IronAlloyIngot => Material::IronIngot,
            Self::MoldedBeam => Material::SteelBeam,
            Self::MoldedSteelPipe => Material::SteelPipe,
            Self::SolidSteelIngot => Material::SteelIngot,
            Self::SteelCastPlate => Material::IronPlate,
            Self::TemperedCateriumIngot => Material::CateriumIngot,
            Self::TemperedCopperIngot => Material::CopperIngot,
        }
    }

    pub fn input_material_speed(&self) -> (f32, f32) {
        match self {
            Self::AluminumIngot => (90., 75.),
            Self::SteelIngot => (45., 45.),
            Self::BasicIronIngot => (25., 40.),
            Self::CokeSteelIngot => (75., 75.),
            Self::CompactedSteelIngot => (5., 2.5),
            Self::CopperAlloyIngot => (50., 50.),
            Self::FusedQuartzCrystal => (75., 36.),
            Self::IronAlloyIngot => (40., 10.),
            Self::MoldedBeam => (120., 80.),
            Self::MoldedSteelPipe => (50., 30.),
            Self::SolidSteelIngot => (40., 40.),
            Self::SteelCastPlate => (15., 15.),
            Self::TemperedCateriumIngot => (45., 15.),
            Self::TemperedCopperIngot => (25., 40.),
        }
    }

    pub fn max_output_speed_material(&self) -> f32 {
        self.output_speed_inner(None)
    }

    pub fn output_speed_material(
        &self,
        input_material_size_0: f32,
        input_material_size_1: f32,
    ) -> f32 {
        self.output_speed_inner(Some((input_material_size_0, input_material_size_1)))
    }

    // returns
    // (duration, material_input_0, material_input_1, material_output)
    fn stats(&self) -> (f32, f32, f32, f32) {
        match self {
            Self::AluminumIngot => (4., 6., 5., 4.),
            Self::SteelIngot => (4., 3., 3., 3.),
            Self::BasicIronIngot => (12., 5., 8., 10.),
            Self::CokeSteelIngot => (12., 15., 15., 20.),
            Self::CompactedSteelIngot => (24., 2., 1., 4.),
            Self::CopperAlloyIngot => (6., 5., 5., 10.),
            Self::FusedQuartzCrystal => (20., 25., 12., 18.),
            Self::IronAlloyIngot => (12., 8., 2., 15.),
            Self::MoldedBeam => (12., 24., 16., 9.),
            Self::MoldedSteelPipe => (6., 5., 3., 5.),
            Self::SolidSteelIngot => (3., 2., 2., 3.),
            Self::SteelCastPlate => (4., 1., 1., 3.),
            Self::TemperedCateriumIngot => (8., 6., 2., 3.),
            Self::TemperedCopperIngot => (12., 5., 8., 12.),
        }
    }

    fn output_speed_inner(&self, input_size: Option<(f32, f32)>) -> f32 {
        let input_material_0_size = input_size.map(|(a, _)| a);
        let input_material_1_size = input_size.map(|(_, b)| b);
        let (duration, material_input_0_size, material_input_1_size, material_output_size) =
            self.stats();

        if input_material_0_size == Some(0.0) || input_material_1_size == Some(0.0) {
            return 0.;
        }

        calc_output2(
            input_size,
            duration,
            material_output_size,
            material_input_0_size,
            material_input_1_size,
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Foundry {
    pub recipe: Option<FoundryRecipe>,
    pub speed: f32,
    pub amplified: SomersloopSlot2,
    pub current_input_material_0: Option<Input>,
    pub current_input_material_1: Option<Input>,
}

impl Default for Foundry {
    fn default() -> Self {
        Self {
            recipe: None,
            speed: 100.,
            amplified: SomersloopSlot2::Empty,
            current_input_material_0: None,
            current_input_material_1: None,
        }
    }
}

impl Foundry {
    pub fn clear_clone(&self) -> Self {
        let mut this = self.clone();
        this.current_input_material_0 = None;
        this.current_input_material_1 = None;
        this
    }
    pub fn header_image(&self) -> String {
        load_img("Foundry.png")
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Foundry ({})", r.name()),
            None => "Foundry".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Smelts more things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        2
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_resource(&self, input_id: usize) -> crate::node::ResourceType {
        match input_id {
            0 => crate::node::ResourceType::Material,
            1 => crate::node::ResourceType::Material,
            _ => unreachable!("2 inputs"),
        }
    }

    pub fn output_resource(&self, output_id: usize) -> crate::node::ResourceType {
        match output_id {
            0 => crate::node::ResourceType::Material,
            _ => unreachable!("1 output"),
        }
    }

    pub fn input_material_speed(&self) -> (f32, f32) {
        let (base_0, base_1) = self
            .recipe
            .as_ref()
            .map(|r| r.input_material_speed())
            .unwrap_or_default();
        let a = round(base_0 as f32 * (self.speed / 100.));
        let b = round(base_1 as f32 * (self.speed / 100.));

        (a, b)
    }

    pub fn output_material(&self) -> Option<Material> {
        self.recipe.as_ref().map(|r| r.output_material())
    }

    pub fn output_speed(&self) -> f32 {
        let input_material_0_speed = self
            .current_input_material_0
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();
        let input_material_1_speed = self
            .current_input_material_1
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.output_speed_material(input_material_0_speed, input_material_1_speed))
            .unwrap_or_default();
        let amplification = self.amplified.factor();

        // TODO: take speed into account for input_speed

        round(base as f32 * (self.speed / 100.) * amplification)
    }

    pub fn input_material(&self) -> Option<(Material, Material)> {
        self.recipe.map(|r| r.input_material())
    }

    pub fn current_output(&self) -> Option<Output> {
        self.recipe.map(|r| Output {
            speed: self.output_speed(),
            resource: Resource::Material(r.output_material()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_speed_packaged_water() {
        assert_eq!(FoundryRecipe::MoldedBeam.output_speed_material(0., 0.), 0.);
        assert_eq!(
            FoundryRecipe::MoldedBeam.output_speed_material(120., 80.),
            45.
        );
    }
}
