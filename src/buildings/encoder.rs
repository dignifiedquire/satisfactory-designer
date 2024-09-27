use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{calc_output4_2, round, Fluid, Material, Selectable, SomersloopSlot4};

macro_rules! r {
    ($($literal_name:expr => $name:ident, $input_speed_fluid_0:expr => $input_fluid_0:expr, $input_speed_material_0:expr => $input_material_0:expr, $input_speed_material_1:expr => $input_material_1:expr, $input_speed_material_2:expr => $input_material_2:expr, $duration:expr, $output_speed_fluid:expr => $output_fluid:expr, $output_speed_material:expr => $output_material:expr),* $(,)*) => {
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
        pub enum QuantumEncoderRecipe {
            $(
              #[strum(to_string = $literal_name)]
              $name,
            )*
        }

        impl super::Selectable for QuantumEncoderRecipe {
            const NAME: &'static str = "Recipe";

            fn name(&self) -> String {
                self.to_string()
            }

            fn image(&self) -> String {
                self.output_material().image()
            }
        }

        impl QuantumEncoderRecipe {
            pub fn input_material(&self) -> (Fluid, Material, Material, Material) {
                match self {
                    $(
                        Self::$name => ($input_fluid_0, $input_material_0, $input_material_1, $input_material_2),
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

            pub fn output_fluid(&self) -> Fluid {
                match self {
                    $(
                        Self::$name => $output_fluid,
                    )*
                }
            }

            pub fn input_speed(&self) -> (f32, f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            (60. / $duration) * $input_speed_fluid_0,
                            (60. / $duration) * $input_speed_material_0,
                            (60. / $duration) * $input_speed_material_1,
                            (60. / $duration) * $input_speed_material_2,
                        ),
                    )*
                }
            }
            pub fn max_output_speed(&self) -> (f32, f32) {
                self.output_speed_inner(None)
            }

            pub fn output_speed(
                &self,
                input_speed_fluid_0: f32,
                input_speed_material_0: f32,
                input_speed_material_1: f32,
                input_speed_material_2: f32,
            ) -> (f32, f32) {
                self.output_speed_inner(Some((
                    input_speed_fluid_0,
                    input_speed_material_0,
                    input_speed_material_1,
                    input_speed_material_2,
                )))
            }

            // returns
            // (duration, fluid_input_0, material_input_0, material_input_1, material_input_2, fluid_output, material_output)
            pub fn stats(&self) -> (f32, f32, f32, f32, f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            $duration,
                            $input_speed_fluid_0,
                            $input_speed_material_0,
                            $input_speed_material_1,
                            $input_speed_material_2,
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
    "AI Expansion Server" => AIExpansionServer,
    25. => Fluid::ExcitedPhotonicMatter,
    1. => Material::MagneticFieldGenerator,
    1. => Material::NeuralQuantumProcessor,
    1. => Material::SuperpositionOscillator,
    15.,
    25. => Fluid::DarkMatterResidue, 1. => Material::AIExpansionServer,

    "Alien Power Matrix" => AlienPowerMatrix,
    24. => Fluid::ExcitedPhotonicMatter,
    5. => Material::SAMFluctuator,
    3. => Material::PowerShard,
    3. => Material::SuperpositionOscillator,
    24.,
    24. => Fluid::DarkMatterResidue, 1. => Material::AlienPowerMatrix,

    "Ficsonium Fuel Rod" => FicsoniumFuelRod,
    20. => Fluid::ExcitedPhotonicMatter,
    2. => Material::Ficsonium,
    2. => Material::ElectromagneticControlRod,
    40. => Material::FicsiteTrigon,
    24.,
    20. => Fluid::DarkMatterResidue, 1. => Material::FicsoniumFuelRod,

    "Neural-Quantum Processor" => NeuralQuantumProcessor,
    25. => Fluid::ExcitedPhotonicMatter,
    5. => Material::TimeCrystal,
    1. => Material::Supercomputer,
    15. => Material::FicsiteTrigon,
    20.,
    25. => Fluid::DarkMatterResidue, 1. => Material::NeuralQuantumProcessor,

    "Superposition Oscillator" => SuperpositionOscillator,
    25. => Fluid::ExcitedPhotonicMatter,
    6. => Material::DarkMatterCrystal,
    1. => Material::CrystalOscillator,
    9. => Material::AlcladAluminumSheet,
    12.,
    25. => Fluid::DarkMatterResidue, 1. => Material::SuperpositionOscillator,

    "Synthetic Power Shard" => SyntheticPowerShard,
    12. => Fluid::ExcitedPhotonicMatter,
    2. => Material::TimeCrystal,
    2. => Material::DarkMatterCrystal,
    12. => Material::QuartzCrystal,
    12.,
    12. => Fluid::DarkMatterResidue, 1. => Material::PowerShard,
);

impl QuantumEncoderRecipe {
    fn output_speed_inner(&self, input_speed: Option<(f32, f32, f32, f32)>) -> (f32, f32) {
        let input_fluid_0_speed = input_speed.map(|(a, _, _, _)| a);
        let input_material_0_speed = input_speed.map(|(_, b, _, _)| b);
        let input_material_1_speed = input_speed.map(|(_, _, c, _)| c);
        let input_material_2_speed = input_speed.map(|(_, _, _, d)| d);
        let (
            duration,
            fluid_input_0_speed,
            material_input_0_speed,
            material_input_1_speed,
            material_input_2_speed,
            fluid_output_speed,
            material_output_speed,
        ) = self.stats();

        if input_fluid_0_speed == Some(0.0)
            || input_material_0_speed == Some(0.0)
            || input_material_1_speed == Some(0.0)
            || input_material_2_speed == Some(0.0)
        {
            return (0., 0.);
        }

        calc_output4_2(
            input_speed,
            duration,
            fluid_output_speed,
            material_output_speed,
            fluid_input_0_speed,
            material_input_0_speed,
            material_input_1_speed,
            material_input_2_speed,
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuantumEncoder {
    pub recipe: Option<QuantumEncoderRecipe>,
    pub speed: f32,
    pub amplified: SomersloopSlot4,
    pub current_input_fluid_0: Option<Input>,
    pub current_input_material_0: Option<Input>,
    pub current_input_material_1: Option<Input>,
    pub current_input_material_2: Option<Input>,
}

impl Default for QuantumEncoder {
    fn default() -> Self {
        Self {
            recipe: None,
            speed: 100.,
            amplified: SomersloopSlot4::Empty,
            current_input_fluid_0: None,
            current_input_material_0: None,
            current_input_material_1: None,
            current_input_material_2: None,
        }
    }
}

impl QuantumEncoder {
    pub fn clear_clone(&self) -> Self {
        Self {
            recipe: self.recipe.clone(),
            speed: self.speed.clone(),
            amplified: self.amplified.clone(),
            ..Default::default()
        }
    }

    pub fn header_image(&self) -> String {
        load_img("Quantum_Encoder.png")
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Quantum Encoder ({})", r.name()),
            None => "Quantum Encoder".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Quantum encodes things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        4
    }

    pub fn num_outputs(&self) -> usize {
        2
    }
    pub fn input_resource(&self, input_id: usize) -> crate::node::ResourceType {
        match input_id {
            0 => crate::node::ResourceType::Fluid,
            1 => crate::node::ResourceType::Material,
            2 => crate::node::ResourceType::Material,
            3 => crate::node::ResourceType::Material,
            _ => unreachable!("4 outputs"),
        }
    }

    pub fn output_resource(&self, output_id: usize) -> crate::node::ResourceType {
        match output_id {
            0 => crate::node::ResourceType::Fluid,
            1 => crate::node::ResourceType::Material,
            _ => unreachable!("2 outputs"),
        }
    }

    pub fn input_speed(&self) -> (f32, f32, f32, f32) {
        let (base_0, base_1, base_2, base_3) = self
            .recipe
            .as_ref()
            .map(|r| r.input_speed())
            .unwrap_or_default();
        let a = round(base_0 as f32 * (self.speed / 100.));
        let b = round(base_1 as f32 * (self.speed / 100.));
        let c = round(base_2 as f32 * (self.speed / 100.));
        let d = round(base_3 as f32 * (self.speed / 100.));

        (a, b, c, d)
    }

    pub fn output_material(&self) -> Option<Material> {
        self.recipe.as_ref().map(|r| r.output_material())
    }

    pub fn output_fluid(&self) -> Option<Fluid> {
        self.recipe.as_ref().map(|r| r.output_fluid())
    }

    pub fn output_speed(&self) -> (f32, f32) {
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
        let input_material_2_speed = self
            .current_input_material_2
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();
        let (base_fluid, base_material) = self
            .recipe
            .as_ref()
            .map(|r| {
                r.output_speed(
                    input_fluid_0_speed,
                    input_material_0_speed,
                    input_material_1_speed,
                    input_material_2_speed,
                )
            })
            .unwrap_or_default();
        let amplification = self.amplified.factor();

        // TODO: take speed into account for input_speed

        let fluid = round(base_fluid as f32 * (self.speed / 100.) * amplification);
        let material = round(base_material as f32 * (self.speed / 100.) * amplification);
        (fluid, material)
    }

    pub fn input_material(&self) -> Option<(Fluid, Material, Material, Material)> {
        self.recipe.map(|r| r.input_material())
    }

    pub fn current_output_material(&self) -> Option<Output> {
        self.recipe.map(|r| Output {
            speed: self.output_speed().1,
            resource: Resource::Material(r.output_material()),
        })
    }

    pub fn current_output_fluid(&self) -> Option<Output> {
        self.recipe.map(|r| Output {
            speed: self.output_speed().0,
            resource: Resource::Fluid(r.output_fluid()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_speed_adaptive_control_unit() {
        assert_eq!(
            QuantumEncoderRecipe::AIExpansionServer.output_speed(0., 0., 0., 0.),
            (0., 0.)
        );
        assert_eq!(
            QuantumEncoderRecipe::AIExpansionServer.output_speed(25., 1., 1., 1.),
            (25., 1.)
        );
    }
}
