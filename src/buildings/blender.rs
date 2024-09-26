use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{calc_output4_2, round, Fluid, Material, Selectable, SomersloopSlot4};

macro_rules! r {
    ($($literal_name:expr => $name:ident, $input_speed_fluid_0:expr => $input_fluid_0:expr, $input_speed_fluid_1:expr => $input_fluid_1:expr, $input_speed_material_0:expr => $input_material_0:expr, $input_speed_material_1:expr => $input_material_1:expr, $duration:expr, $output_speed_fluid:expr => $output_fluid:expr, $output_speed_material:expr => $output_material:expr),* $(,)*) => {
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
        pub enum BlenderRecipe {
            $(
              #[strum(to_string = $literal_name)]
              $name,
            )*
        }

        impl super::Selectable for BlenderRecipe {
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

        impl BlenderRecipe {
            pub fn input_material(&self) -> (Fluid, Option<Fluid>, Option<Material>, Option<Material>) {
                match self {
                    $(
                        Self::$name => ($input_fluid_0, $input_fluid_1, $input_material_0, $input_material_1),
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

            pub fn input_speed(&self) -> (f32, f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            (60. / $duration) * $input_speed_fluid_0,
                            (60. / $duration) * $input_speed_fluid_1,
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
                input_speed_fluid_0: f32,
                input_speed_fluid_1: f32,
                input_speed_material_0: f32,
                input_speed_material_1: f32,
            ) -> (f32, f32) {
                self.output_speed_inner(Some((
                    input_speed_fluid_0,
                    input_speed_fluid_1,
                    input_speed_material_0,
                    input_speed_material_1,
                )))
            }

            // returns
            // (duration, fluid_input_0, fluid_input_1, material_input_0, material_input_1, fluid_output, material_output)
            pub fn stats(&self) -> (f32, f32, f32, f32, f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            $duration,
                            $input_speed_fluid_0,
                            $input_speed_fluid_1,
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
    "Battery" => Battery,
    2.5 => Fluid::SulfuricAcid, 2. => Some(Fluid::AluminaSolution),
    1. => Some(Material::AluminumCasing), 0. => None,
    3.,
    1.5 => Some(Fluid::Water), 1. => Some(Material::Battery),

    "Biochemical Sculptor" => BiochemicalSculptor,
    20. => Fluid::Water, 0. => None,
    1. => Some(Material::AssemblyDirectorSystem), 80. => Some(Material::FicsiteTrigon),
    120.,
    0. => None, 4. => Some(Material::BiochemicalSculptor),

    "Cooling System" => CoolingSystem,
    5. => Fluid::Water, 25. => Some(Fluid::NitrogenGas),
    2. => Some(Material::HeatSink), 2. => Some(Material::Rubber),
    10.,
    0. => None, 1. => Some(Material::CoolingSystem),

    "Encased Uranium Cell" => EncasedUraniumCell,
    8. => Fluid::SulfuricAcid, 0. => None,
    10. => Some(Material::Uranium), 3. => Some(Material::Concrete),
    12.,
    2. => Some(Fluid::SulfuricAcid), 5. => Some(Material::EncasedUraniumCell),

    "Fused Modular Frame" => FusedModularFrame,
    25. => Fluid::NitrogenGas, 0. => None,
    1. => Some(Material::HeavyModularFrame), 50. => Some(Material::AluminumCasing),
    40.,
    0. => None, 1. => Some(Material::FusedModularFrame),

    "Nitric Acid" => NitricAcid,
    12. => Fluid::NitrogenGas, 3. => Some(Fluid::Water),
    1. => Some(Material::IronPlate), 0. => None,
    6.,
    3. => Some(Fluid::NitricAcid), 0. => None,

    "Non-Fissile Uranium" => NonFissileUranium,
    6. => Fluid::NitricAcid, 6. => Some(Fluid::SulfuricAcid),
    15. => Some(Material::UraniumWaste), 10. => Some(Material::Silica),
    24.,
    6. => Some(Fluid::Water), 20. => Some(Material::NonFissileUranium),

    "Rocket Fuel" => RocketFuel,
    6. => Fluid::Turbofuel, 1. => Some(Fluid::NitricAcid),
    0. => None, 0. => None,
    6.,
    10. => Some(Fluid::RocketFuel), 1. => Some(Material::CompactedCoal),

    "Turbo Rifle Ammo" => TurboRifleAmmo,
    3. => Fluid::Turbofuel, 0. => None,
    25. => Some(Material::RifleAmmo), 3. => Some(Material::AluminumCasing),
    12.,
    0. => None, 50. => Some(Material::TurboRifleAmmo),

    "Cooling Device" => CoolingDevice,
    24. => Fluid::NitrogenGas, 0. => None,
    4. => Some(Material::HeatSink), 1. => Some(Material::Motor),
    24.,
    0. => None, 2. => Some(Material::CoolingSystem),

    "Diluted Fuel" => DilutedFuel,
    5. => Fluid::HeavyOilResidue, 10. => Some(Fluid::Water),
    0. => None, 0. => None,
    6.,
    100. => Some(Fluid::Fuel), 0. => None,

    "Distilled Silica" => DistilledSilica,
    12. => Fluid::DissolvedSilica, 10. => Some(Fluid::Water),
    5. => Some(Material::Limestone), 0. => None,
    6.,
    8. => Some(Fluid::Water), 27. => Some(Material::Silica),

    "Fertile Uranium" => FertileUranium,
    3. => Fluid::NitricAcid, 5. => Some(Fluid::SulfuricAcid),
    5. => Some(Material::Uranium), 5. => Some(Material::UraniumWaste),
    12.,
    8. => Some(Fluid::Water), 20. => Some(Material::NonFissileUranium),

    "Heat-Fused Frame" => HeatFusedFrame,
    8. => Fluid::NitricAcid, 10. => Some(Fluid::Fuel),
    1. => Some(Material::HeavyModularFrame), 50. => Some(Material::AluminumIngot),
    20.,
    0. => None, 1. => Some(Material::FusedModularFrame),

    "Instant Scrap" => InstantScrap,
    5. => Fluid::SulfuricAcid, 6. => Some(Fluid::Water),
    15. => Some(Material::Bauxite), 10. => Some(Material::Coal),
    6.,
    5. => Some(Fluid::Water), 30. => Some(Material::AluminumScrap),

    "Nitro Rocket Fuel" => NitroRocketFuel,
    4. => Fluid::Fuel, 3. => Some(Fluid::NitrogenGas),
    4. => Some(Material::Sulfur), 2. => Some(Material::Coal),
    2.4,
    6. => Some(Fluid::RocketFuel), 1. => Some(Material::CompactedCoal),

    "Turbo Blend Fuel" => TurboBlendFuel,
    2. => Fluid::Fuel, 4. => Some(Fluid::HeavyOilResidue),
    3. => Some(Material::Sulfur), 3. => Some(Material::PetroleumCoke),
    8.,
    6. => Some(Fluid::Turbofuel), 0. => None,
);

impl BlenderRecipe {
    fn output_speed_inner(&self, input_speed: Option<(f32, f32, f32, f32)>) -> (f32, f32) {
        let input_fluid_0_speed = input_speed.map(|(a, _, _, _)| a);
        let input_fluid_1_speed = input_speed.map(|(_, b, _, _)| b);
        let input_material_0_speed = input_speed.map(|(_, _, c, _)| c);
        let input_material_1_speed = input_speed.map(|(_, _, _, d)| d);
        let (
            duration,
            fluid_input_0_speed,
            fluid_input_1_speed,
            material_input_0_speed,
            material_input_1_speed,
            fluid_output_speed,
            material_output_speed,
        ) = self.stats();

        if input_fluid_0_speed == Some(0.0) {
            return (0., 0.);
        }

        if self.input_material().1.is_some() && input_fluid_1_speed == Some(0.0) {
            return (0., 0.);
        }

        if self.input_material().2.is_some() && input_material_0_speed == Some(0.0) {
            return (0., 0.);
        }

        if self.input_material().3.is_some() && input_material_1_speed == Some(0.0) {
            return (0., 0.);
        }

        calc_output4_2(
            input_speed,
            duration,
            fluid_output_speed,
            material_output_speed,
            fluid_input_0_speed,
            fluid_input_1_speed,
            material_input_0_speed,
            material_input_1_speed,
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Blender {
    pub recipe: Option<BlenderRecipe>,
    pub speed: f32,
    pub amplified: SomersloopSlot4,
    pub current_input_fluid_0: Option<Input>,
    pub current_input_fluid_1: Option<Input>,
    pub current_input_material_0: Option<Input>,
    pub current_input_material_1: Option<Input>,
}

impl Default for Blender {
    fn default() -> Self {
        Self {
            recipe: None,
            speed: 100.,
            amplified: SomersloopSlot4::Empty,
            current_input_fluid_0: None,
            current_input_fluid_1: None,
            current_input_material_0: None,
            current_input_material_1: None,
        }
    }
}

impl Blender {
    pub fn clear_clone(&self) -> Self {
        Self {
            recipe: self.recipe.clone(),
            speed: self.speed.clone(),
            amplified: self.amplified.clone(),
            ..Default::default()
        }
    }

    pub fn header_image(&self) -> String {
        load_img("Blender.png")
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Blender ({})", r.name()),
            None => "Blender".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Smelts more things".to_string()
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
            1 => crate::node::ResourceType::Fluid,
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
        self.recipe.as_ref().and_then(|r| r.output_material())
    }

    pub fn output_fluid(&self) -> Option<Fluid> {
        self.recipe.as_ref().and_then(|r| r.output_fluid())
    }

    pub fn output_speed(&self) -> (f32, f32) {
        let input_fluid_0_speed = self
            .current_input_fluid_0
            .as_ref()
            .map(|i| i.speed)
            .unwrap_or_default();
        let input_fluid_1_speed = self
            .current_input_fluid_1
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
        let (base_fluid, base_material) = self
            .recipe
            .as_ref()
            .map(|r| {
                r.output_speed(
                    input_fluid_0_speed,
                    input_fluid_1_speed,
                    input_material_0_speed,
                    input_material_1_speed,
                )
            })
            .unwrap_or_default();
        let amplification = self.amplified.factor();

        // TODO: take speed into account for input_speed

        let fluid = round(base_fluid as f32 * (self.speed / 100.) * amplification);
        let material = round(base_material as f32 * (self.speed / 100.) * amplification);
        (fluid, material)
    }

    pub fn input_material(
        &self,
    ) -> Option<(Fluid, Option<Fluid>, Option<Material>, Option<Material>)> {
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
            BlenderRecipe::Battery.output_speed(0., 0., 0., 0.),
            (0., 0.)
        );
        assert_eq!(
            BlenderRecipe::Battery.output_speed(2.5, 2., 1., 0.),
            (1.5, 1.)
        );
    }
}
