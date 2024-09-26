use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{calc_output3, round, Fluid, Material, Selectable, SomersloopSlot4};

macro_rules! r {
    ($($literal_name:expr => $name:ident, $input_speed_fluid_0:expr => $input_fluid_0:expr, $input_speed_material_0:expr => $input_material_0:expr, $input_speed_material_1:expr => $input_material_1:expr, $duration:expr, $output_speed_material:expr => $output_material:expr),* $(,)*) => {
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
        pub enum ParticleAcceleratorRecipe {
            $(
              #[strum(to_string = $literal_name)]
              $name,
            )*
        }

        impl super::Selectable for ParticleAcceleratorRecipe {
            const NAME: &'static str = "Recipe";

            fn name(&self) -> String {
                self.to_string()
            }

            fn image(&self) -> String {
                self.output_material().image()
            }
        }

        impl ParticleAcceleratorRecipe {
            pub fn input_material(&self) -> (Option<Fluid>, Option<Material>, Option<Material>) {
                match self {
                    $(
                        Self::$name => ($input_fluid_0, $input_material_0, $input_material_1),
                    )*
                }
            }

            pub fn output_material(&self) -> Material {
                match self {
                    $(
                        Self::$name => $output_material,
                    )*
                }
            }

            pub fn input_speed(&self) -> (f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            (60. / $duration) * $input_speed_fluid_0,
                            (60. / $duration) * $input_speed_material_0,
                            (60. / $duration) * $input_speed_material_1,
                        ),
                    )*
                }
            }
            pub fn max_output_speed(&self) -> f32 {
                self.output_speed_inner(None)
            }

            pub fn output_speed(
                &self,
                input_speed_fluid_0: f32,
                input_speed_material_0: f32,
                input_speed_material_1: f32,
            ) -> f32 {
                self.output_speed_inner(Some((
                    input_speed_fluid_0,
                    input_speed_material_0,
                    input_speed_material_1,
                )))
            }

            // returns
            // (duration, fluid_input_0, material_input_0, material_input_1, material_output)
            pub fn stats(&self) -> (f32, f32, f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            $duration,
                            $input_speed_fluid_0,
                            $input_speed_material_0,
                            $input_speed_material_1,
                            $output_speed_material,
                        ),
                    )*
                }
            }
        }
    }
}

r!(
    "Dark Matter Crystal" => DarkMatterCrystal,
    5. => Some(Fluid::DarkMatterResidue),
    1. => Some(Material::Diamonds), 0. => None,
    2.,
    1. => Material::DarkMatterCrystal,

    "Diamonds" => Diamonds,
    0. => None,
    20. => Some(Material::Coal), 0. => None,
    2.,
    1. => Material::Diamonds,

    "Ficsonium" => Ficsonium,
    20. => Some(Fluid::DarkMatterResidue),
    1. => Some(Material::PlutoniumWaste), 1. => Some(Material::SingularityCell),
    6.,
    1. => Material::Ficsonium,

    "Nuclear Pasta" => NuclearPasta,
    0. => None,
    200. => Some(Material::CopperPowder), 1. => Some(Material::PressureConversionCube),
    120.,
    1. => Material::NuclearPasta,

    "Plutonium Pellet" => PlutoniumPellet,
    0. => None,
    100. => Some(Material::NonFissileUranium), 25. => Some(Material::UraniumWaste),
    60.,
    30. => Material::PlutoniumPellet,

    "Cloudy Diamonds" => CloudyDiamonds,
    0. => None,
    12. => Some(Material::Coal), 24. => Some(Material::Limestone),
    3.,
    1. => Material::Diamonds,

    "Dark Crystallization Matter " => DarkMatterCrystallization,
    10. => Some(Fluid::DarkMatterResidue),
    0. => None, 0. => None,
    3.,
    1. => Material::DarkMatterCrystal,

    "Dark Matter Trap" => DarkMatterTrap,
    5. => Some(Fluid::DarkMatterResidue),
    1. => Some(Material::TimeCrystal), 0. => None,
    2.,
    2. => Material::DarkMatterCrystal,

    "Instant Plutonium Cell" => InstantPlutoniumCell,
    0. => None,
    150. => Some(Material::NonFissileUranium), 20. => Some(Material::AluminumCasing),
    120.,
    20. => Material::EncasedPlutoniumCell,

    "Oil-Based Diamonds" => OilBasedDiamonds,
    10. => Some(Fluid::CrudeOil),
    0. => None, 0. => None,
    3.,
    2. => Material::Diamonds,

    "Petroleum Diamonds" => PetroleumDiamonds,
    0. => None,
    24. => Some(Material::PetroleumCoke), 0. => None,
    2.,
    1. => Material::Diamonds,

    "Turbo Diamonds" => TurboDiamonds,
    0. => None,
    30. => Some(Material::Coal), 2. => Some(Material::PackagedTurbofuel),
    3.,
    3. => Material::Diamonds,
);

impl ParticleAcceleratorRecipe {
    fn output_speed_inner(&self, input_speed: Option<(f32, f32, f32)>) -> f32 {
        let input_fluid_0_speed = input_speed.map(|(a, _, _)| a);
        let input_material_0_speed = input_speed.map(|(_, b, _)| b);
        let input_material_1_speed = input_speed.map(|(_, _, c)| c);
        let (
            duration,
            fluid_input_0_speed,
            material_input_0_speed,
            material_input_1_speed,
            material_output_speed,
        ) = self.stats();

        if self.input_material().0.is_some() && input_fluid_0_speed == Some(0.0) {
            return 0.;
        }

        if self.input_material().1.is_some() && input_material_0_speed == Some(0.0) {
            return 0.;
        }

        if self.input_material().2.is_some() && input_material_1_speed == Some(0.0) {
            return 0.;
        }

        calc_output3(
            input_speed,
            duration,
            material_output_speed,
            fluid_input_0_speed,
            material_input_0_speed,
            material_input_1_speed,
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParticleAccelerator {
    pub recipe: Option<ParticleAcceleratorRecipe>,
    pub speed: f32,
    pub amplified: SomersloopSlot4,
    pub current_input_fluid_0: Option<Input>,
    pub current_input_material_0: Option<Input>,
    pub current_input_material_1: Option<Input>,
}

impl Default for ParticleAccelerator {
    fn default() -> Self {
        Self {
            recipe: None,
            speed: 100.,
            amplified: SomersloopSlot4::Empty,
            current_input_fluid_0: None,
            current_input_material_0: None,
            current_input_material_1: None,
        }
    }
}

impl ParticleAccelerator {
    pub fn clear_clone(&self) -> Self {
        Self {
            recipe: self.recipe.clone(),
            speed: self.speed.clone(),
            amplified: self.amplified.clone(),
            ..Default::default()
        }
    }

    pub fn header_image(&self) -> String {
        load_img("Particle_Accelerator.png")
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Particle Accelerator ({})", r.name()),
            None => "Particle Accelerator".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Accelerates things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        3
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_resource(&self, input_id: usize) -> crate::node::ResourceType {
        match input_id {
            0 => crate::node::ResourceType::Fluid,
            1 => crate::node::ResourceType::Material,
            2 => crate::node::ResourceType::Material,
            _ => unreachable!("3 outputs"),
        }
    }

    pub fn output_resource(&self, output_id: usize) -> crate::node::ResourceType {
        match output_id {
            0 => crate::node::ResourceType::Material,
            _ => unreachable!("1 output"),
        }
    }

    pub fn input_speed(&self) -> (f32, f32, f32) {
        let (base_0, base_1, base_2) = self
            .recipe
            .as_ref()
            .map(|r| r.input_speed())
            .unwrap_or_default();
        let a = round(base_0 as f32 * (self.speed / 100.));
        let b = round(base_1 as f32 * (self.speed / 100.));
        let c = round(base_2 as f32 * (self.speed / 100.));

        (a, b, c)
    }

    pub fn output_material(&self) -> Option<Material> {
        self.recipe.as_ref().map(|r| r.output_material())
    }

    pub fn output_speed(&self) -> f32 {
        let input_fluid_0_speed = self
            .current_input_fluid_0
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();
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
        let base_material = self
            .recipe
            .as_ref()
            .map(|r| {
                r.output_speed(
                    input_fluid_0_speed,
                    input_material_0_speed,
                    input_material_1_speed,
                )
            })
            .unwrap_or_default();
        let amplification = self.amplified.factor();

        // TODO: take speed into account for input_speed
        round(base_material as f32 * (self.speed / 100.) * amplification)
    }

    pub fn input_material(&self) -> Option<(Option<Fluid>, Option<Material>, Option<Material>)> {
        self.recipe.map(|r| r.input_material())
    }

    pub fn current_output_material(&self) -> Option<Output> {
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
    fn test_output_speed_dark_matter_crystal() {
        assert_eq!(
            ParticleAcceleratorRecipe::DarkMatterCrystal.output_speed(0., 0., 0.),
            0.
        );
        assert_eq!(
            ParticleAcceleratorRecipe::DarkMatterCrystal.output_speed(5., 1., 0.),
            1.,
        );
    }

    #[test]
    fn test_output_speed_nuclear_pasta() {
        assert_eq!(
            ParticleAcceleratorRecipe::NuclearPasta.output_speed(0., 0., 0.),
            0.
        );
        assert_eq!(
            ParticleAcceleratorRecipe::NuclearPasta.output_speed(0., 200., 1.),
            0.5,
        );
    }
}
