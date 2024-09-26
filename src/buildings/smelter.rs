use strum::VariantArray;

use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{calc_output, round, Material, Selectable, SomersloopSlot1};

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
pub enum SmelterRecipe {
    #[strum(to_string = "Caterium Ingot")]
    CateriumIngot,
    #[strum(to_string = "Copper Ingot")]
    CopperIngot,
    #[strum(to_string = "Iron Ingot")]
    IronIngot,
    #[strum(to_string = "Pure Aluminum Ingot")]
    PureAluminumIngot,
}

impl Selectable for SmelterRecipe {
    const NAME: &'static str = "Recipe";

    fn name(&self) -> String {
        self.to_string()
    }

    fn image(&self) -> String {
        self.output_material().image()
    }
}

impl SmelterRecipe {
    pub fn input_material(&self) -> Material {
        match self {
            Self::CateriumIngot => Material::CateriumOre,
            Self::CopperIngot => Material::CopperOre,
            Self::IronIngot => Material::IronOre,
            Self::PureAluminumIngot => Material::AluminumScrap,
        }
    }

    pub fn output_material(&self) -> Material {
        match self {
            Self::CateriumIngot => Material::CateriumIngot,
            Self::CopperIngot => Material::CopperIngot,
            Self::IronIngot => Material::IronIngot,
            Self::PureAluminumIngot => Material::AluminumIngot,
        }
    }

    pub fn input_speed(&self) -> usize {
        match self {
            Self::CateriumIngot => 45,
            Self::CopperIngot => 30,
            Self::IronIngot => 30,
            Self::PureAluminumIngot => 60,
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

        let (duration, output_size, input_base) = match self {
            Self::CateriumIngot => (4., 1., 3.),
            Self::CopperIngot => (2., 1., 1.),
            Self::IronIngot => (2., 1., 1.),
            Self::PureAluminumIngot => (2., 1., 2.),
        };

        calc_output(input_size, duration, output_size, input_base)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Smelter {
    pub recipe: Option<SmelterRecipe>,
    pub speed: f32,
    pub amplified: SomersloopSlot1,
    pub current_input: Option<Input>,
}

impl Default for Smelter {
    fn default() -> Self {
        Self {
            recipe: None,
            speed: 100.,
            amplified: SomersloopSlot1::Empty,
            current_input: None,
        }
    }
}

impl Smelter {
    pub fn clear_clone(&self) -> Self {
        let mut this = self.clone();
        this.current_input = None;
        this
    }

    pub fn header_image(&self) -> String {
        load_img("Smelter.png")
    }

    pub fn available_recipes(&self) -> &'static [SmelterRecipe] {
        SmelterRecipe::VARIANTS
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Smelter ({})", r.name()),
            None => "Smelter".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Smelts things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        1
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_resource(&self, input_id: usize) -> crate::node::ResourceType {
        assert_eq!(input_id, 0, "1 input");
        crate::node::ResourceType::Material
    }

    pub fn output_resource(&self, output_id: usize) -> crate::node::ResourceType {
        assert_eq!(output_id, 0, "1 output");
        crate::node::ResourceType::Material
    }

    pub fn input_speed(&self) -> f32 {
        let base = self
            .recipe
            .as_ref()
            .map(|r| r.input_speed())
            .unwrap_or_default();
        round(base as f32 * (self.speed / 100.))
    }

    pub fn output_speed(&self) -> f32 {
        let input_speed = self
            .current_input
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();

        let base = self
            .recipe
            .as_ref()
            .map(|r| r.output_speed(input_speed))
            .unwrap_or_default();
        let amplification = self.amplified.factor();

        // TODO: take speed into account for input_speed

        round(base as f32 * (self.speed / 100.) * amplification)
    }

    pub fn input_material(&self) -> Option<Material> {
        self.recipe.as_ref().map(|r| r.input_material())
    }

    pub fn output_material(&self) -> Option<Material> {
        self.recipe.as_ref().map(|r| r.output_material())
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
    fn test_output_speed() {
        assert_eq!(SmelterRecipe::CateriumIngot.output_speed(0.), 0.);
        assert_eq!(SmelterRecipe::CateriumIngot.output_speed(15.), 5.);
        assert_eq!(SmelterRecipe::CateriumIngot.output_speed(45.), 15.);
        assert_eq!(SmelterRecipe::CateriumIngot.output_speed(60.), 15.);

        assert_eq!(SmelterRecipe::IronIngot.output_speed(0.), 0.);
        assert_eq!(SmelterRecipe::IronIngot.output_speed(10.), 10.);
        assert_eq!(SmelterRecipe::IronIngot.output_speed(30.), 30.);
        assert_eq!(SmelterRecipe::IronIngot.output_speed(60.), 30.);

        assert_eq!(SmelterRecipe::PureAluminumIngot.output_speed(0.), 0.);
        assert_eq!(SmelterRecipe::PureAluminumIngot.output_speed(10.), 5.);
        assert_eq!(SmelterRecipe::PureAluminumIngot.output_speed(30.), 15.);
        assert_eq!(SmelterRecipe::PureAluminumIngot.output_speed(60.), 30.);
        assert_eq!(SmelterRecipe::PureAluminumIngot.output_speed(120.), 30.);
    }
}
