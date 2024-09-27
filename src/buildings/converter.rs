use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{calc_output2_2, round, Fluid, Material, Selectable, SomersloopSlot2};

macro_rules! r {
    ($($literal_name:expr => $name:ident, $input_speed_material_0:expr => $input_material_0:expr, $input_speed_material_1:expr => $input_material_1:expr, $duration:expr, $output_speed_fluid:expr => $output_fluid:expr, $output_speed_material:expr => $output_material:expr),* $(,)*) => {
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
        pub enum ConverterRecipe {
            $(
              #[strum(to_string = $literal_name)]
              $name,
            )*
        }

        impl super::Selectable for ConverterRecipe {
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

        impl ConverterRecipe {
            pub fn input_material(&self) -> (Option<Material>, Option<Material>) {
                match self {
                    $(
                        Self::$name => ($input_material_0, $input_material_1),
                    )*
                }
            }

            pub fn output_material(&self) -> Option<Material> {
                match self {
                    $(
                        Self::$name => $output_material,
                    )*
                }
            }

            pub fn output_fluid(&self) -> Option<Fluid> {
                match self {
                    $(
                        Self::$name => $output_fluid,
                    )*
                }
            }

            pub fn input_speed(&self) -> (f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            (60. / $duration) * $input_speed_material_0,
                            (60. / $duration) * $input_speed_material_1,
                        ),
                    )*
                }
            }
            pub fn max_output_speed(&self) -> (f32, f32) {
                self.output_speed_inner(None)
            }

            pub fn output_speed(
                &self,
                input_speed_material_0: f32,
                input_speed_material_1: f32,
            ) -> (f32, f32) {
                self.output_speed_inner(Some((
                    input_speed_material_0,
                    input_speed_material_1,
                )))
            }

            // returns
            // (duration, material_input, material_input, fluid_output, material_output)
            pub fn stats(&self) -> (f32, f32, f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            $duration,
                            $input_speed_material_0,
                            $input_speed_material_1,
                            $output_speed_fluid,
                            $output_speed_material,
                        ),
                    )*
                }
            }
        }
    }
}

r!(
    "Bauxite (Caterium)" => BauxiteCaterium,
    1. => Some(Material::ReanimatedSAM), 15. => Some(Material::CateriumOre),
    6.,
    0. => None, 12. => Some(Material::Bauxite),

    "Bauxite (Copper)" => BauxiteCopper,
    1. => Some(Material::ReanimatedSAM), 18. => Some(Material::CopperOre),
    6.,
    0. => None, 12. => Some(Material::Bauxite),

    "Caterium Ore (Copper)" => CateriumOreCopper,
    1. => Some(Material::ReanimatedSAM), 15. => Some(Material::CopperOre),
    6.,
    0. => None, 12. => Some(Material::CateriumOre),

    "Caterium Ore (Quartz)" => CateriumOreQuartz,
    1. => Some(Material::ReanimatedSAM), 12. => Some(Material::RawQuartz),
    6.,
    0. => None, 12. => Some(Material::CateriumOre),

    "Coal (Iron)" => CoalIron,
    1. => Some(Material::ReanimatedSAM), 18. => Some(Material::IronOre),
    6.,
    0. => None, 12. => Some(Material::Coal),

    "Coal (Limestone)" => CoalLimestone,
    1. => Some(Material::ReanimatedSAM), 36. => Some(Material::Limestone),
    6.,
    0. => None, 12. => Some(Material::Coal),

    "Copper Ore (Quartz)" => CopperOreQuartz,
    1. => Some(Material::ReanimatedSAM), 10. => Some(Material::CopperOre),
    6.,
    0. => None, 12. => Some(Material::CopperOre),

    "Copper Ore (Sulfur)" => CopperOreSulfur,
    1. => Some(Material::ReanimatedSAM), 12. => Some(Material::Sulfur),
    6.,
    0. => None, 12. => Some(Material::CopperOre),

    "Dark Matter Residue" => DarkMatterResidue,
    5. => Some(Material::ReanimatedSAM), 0. => None,
    6.,
    10. => Some(Fluid::DarkMatterResidue), 0. => None,

    "Excited Photonic Matter" => ExcitedPhotonicMatter,
    0. => None, 0. => None,
    3.,
    10. => Some(Fluid::ExcitedPhotonicMatter), 0. => None,

    "Ficsite Ingot (Aluminum)" => FicsiteIngotAluminum,
    2. => Some(Material::ReanimatedSAM), 4. => Some(Material::AluminumIngot),
    2.,
    0. => None, 1. => Some(Material::FicsiteIngot),

    "Ficsite Ingot (Caterium)" => FicsiteIngotCaterium,
    3. => Some(Material::ReanimatedSAM), 4. => Some(Material::CateriumIngot),
    4.,
    0. => None, 11. => Some(Material::FicsiteIngot),

    "Ficsite Ingot (Iron)" => FicsiteIngotIron,
    4. => Some(Material::ReanimatedSAM), 24. => Some(Material::IronIngot),
    6.,
    0. => None, 1. => Some(Material::FicsiteIngot),

    "Iron Ore (Limestone)" => IronOreLimestone,
    1. => Some(Material::ReanimatedSAM), 24. => Some(Material::Limestone),
    6.,
    0. => None, 12. => Some(Material::IronOre),

    "Limestone (Sulfur)" => LimestoneSulfur,
    1. => Some(Material::ReanimatedSAM), 2. => Some(Material::Sulfur),
    6.,
    0. => None, 12. => Some(Material::Limestone),

    "Nitrogen Gas (Bauxite)" => NitrogenGasBauxite,
    1. => Some(Material::ReanimatedSAM), 10. => Some(Material::Bauxite),
    6.,
    12. => Some(Fluid::NitrogenGas), 0. => None,

    "Nitrogen Gas (Caterium)" => NitrogenGasCaterium,
    1. => Some(Material::ReanimatedSAM), 12. => Some(Material::CateriumOre),
    6.,
    12. => Some(Fluid::NitrogenGas), 0. => None,

    "Raw Quartz (Bauxite)" => RawQuartzBauxite,
    1. => Some(Material::ReanimatedSAM), 10. => Some(Material::Bauxite),
    6.,
    0. => None, 12. => Some(Material::Bauxite),

    "Raw Quartz (Coal)" => RawQuartzCoal,
    1. => Some(Material::ReanimatedSAM), 24. => Some(Material::Coal),
    6.,
    0. => None, 12. => Some(Material::RawQuartz),

    "Sulfur (Coal)" => SulfurCoal,
    1. => Some(Material::ReanimatedSAM), 24. => Some(Material::Coal),
    6.,
    0. => None, 12. => Some(Material::Sulfur),

    "Sulfur (Iron)" => SulfurIron,
    1. => Some(Material::ReanimatedSAM), 30. => Some(Material::IronOre),
    6.,
    0. => None, 12. => Some(Material::Sulfur),

    "Time Crystal" => TimeCrystal,
    2. => Some(Material::Diamonds), 0. => None,
    10.,
    0. => None, 1. => Some(Material::TimeCrystal),

    "Uranium Ore (Bauxite)" => UraniumOreBauxite,
    1. => Some(Material::ReanimatedSAM), 48. => Some(Material::Bauxite),
    6.,
    0. => None, 12. => Some(Material::Uranium),

    "Dark-Ion Fuel" => DarkIonFuel,
    12. => Some(Material::PackagedRocketFuel), 4. => Some(Material::DarkMatterCrystal),
    3.,
    10. => Some(Fluid::IonizedFuel), 2. => Some(Material::CompactedCoal),

    "Pink Diamonds" => PinkDiamonds,
    8. => Some(Material::Coal), 3. => Some(Material::QuartzCrystal),
    3.,
    0. => None, 1. => Some(Material::Diamonds),
);

impl ConverterRecipe {
    fn output_speed_inner(&self, input_speed: Option<(f32, f32)>) -> (f32, f32) {
        let input_material_0_speed = input_speed.map(|(a, _)| a);
        let input_material_1_speed = input_speed.map(|(_, b)| b);
        let (
            duration,
            material_0_input_speed,
            material_1_input_speed,
            fluid_output_speed,
            material_output_speed,
        ) = self.stats();

        if self.input_material().0.is_some() && input_material_0_speed == Some(0.0) {
            return (0., 0.);
        }

        if self.input_material().1.is_some() && input_material_1_speed == Some(0.0) {
            return (0., 0.);
        }

        calc_output2_2(
            input_speed,
            duration,
            fluid_output_speed,
            material_output_speed,
            material_0_input_speed,
            material_1_input_speed,
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Converter {
    pub recipe: Option<ConverterRecipe>,
    pub speed: f32,
    pub amplified: SomersloopSlot2,
    pub current_input_material_0: Option<Input>,
    pub current_input_material_1: Option<Input>,
}

impl Default for Converter {
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

impl Converter {
    pub fn clear_clone(&self) -> Self {
        Self {
            recipe: self.recipe.clone(),
            speed: self.speed.clone(),
            amplified: self.amplified.clone(),
            ..Default::default()
        }
    }

    pub fn header_image(&self) -> String {
        load_img("Converter.png")
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Converter ({})", r.name()),
            None => "Converter".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Converts things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        2
    }

    pub fn num_outputs(&self) -> usize {
        2
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
            0 => crate::node::ResourceType::Fluid,
            1 => crate::node::ResourceType::Material,
            _ => unreachable!("2 outputs"),
        }
    }

    pub fn input_speed(&self) -> (f32, f32) {
        let (base_0, base_1) = self
            .recipe
            .as_ref()
            .map(|r| r.input_speed())
            .unwrap_or_default();
        let a = round(base_0 as f32 * (self.speed / 100.));
        let b = round(base_1 as f32 * (self.speed / 100.));

        (a, b)
    }

    pub fn output_material(&self) -> Option<Material> {
        self.recipe.as_ref().and_then(|r| r.output_material())
    }

    pub fn output_fluid(&self) -> Option<Fluid> {
        self.recipe.as_ref().and_then(|r| r.output_fluid())
    }

    pub fn output_speed(&self) -> (f32, f32) {
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

        let (base_material_0, base_material_1) = self
            .recipe
            .as_ref()
            .map(|r| r.output_speed(input_material_0_speed, input_material_1_speed))
            .unwrap_or_default();
        let amplification = self.amplified.factor();

        // TODO: take speed into account for input_speed

        let material_0 = round(base_material_0 as f32 * (self.speed / 100.) * amplification);
        let material_1 = round(base_material_1 as f32 * (self.speed / 100.) * amplification);
        (material_0, material_1)
    }

    pub fn input_material(&self) -> Option<(Option<Material>, Option<Material>)> {
        self.recipe.map(|r| r.input_material())
    }

    pub fn current_output_material(&self) -> Option<Output> {
        self.recipe
            .and_then(|r| r.output_material())
            .map(|mat| Output {
                speed: self.output_speed().1,
                resource: Resource::Material(mat),
            })
    }

    pub fn current_output_fluid(&self) -> Option<Output> {
        self.recipe
            .and_then(|r| r.output_fluid())
            .map(|fluid| Output {
                speed: self.output_speed().0,
                resource: Resource::Fluid(fluid),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_speed_adaptive_control_unit() {
        assert_eq!(
            ConverterRecipe::BauxiteCaterium.output_speed(0., 0.),
            (0., 0.)
        );
        assert_eq!(
            ConverterRecipe::BauxiteCaterium.output_speed(1., 15.),
            (0., 12.)
        );
    }
}
