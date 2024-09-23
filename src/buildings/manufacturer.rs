use strum::VariantArray;

use crate::util::load_img;

use super::{calc_output4, Material, SomersloopSlot4};

macro_rules! r {
    ($($literal_name:expr => $name:ident, $input_speed_0:expr => $input_material_0:expr, $input_speed_1:expr => $input_material_1:expr, $input_speed_2:expr => $input_material_2:expr, $input_speed_3:expr => $input_material_3:expr, $duration:expr, $output_speed:expr, $output_material:expr),* $(,)*) => {
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
        pub enum ManufacturerRecipe {
            $(
              #[strum(to_string = $literal_name)]
              $name,
            )*
        }

        impl ManufacturerRecipe {
            pub fn name(&self) -> String {
                self.to_string()
            }

            pub fn image(&self) -> String {
                self.output_material().image()
            }

            pub fn input_material(&self) -> (Material, Material, Material, Option<Material>) {
                match self {
                    $(
                        Self::$name => ($input_material_0, $input_material_1, $input_material_2, $input_material_3),
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

            pub fn input_material_speed(&self) -> (f32, f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            (60. / $duration) * $input_speed_0,
                            (60. / $duration) * $input_speed_1,
                            (60. / $duration) * $input_speed_2,
                            (60. / $duration) * $input_speed_3,
                        ),
                    )*
                }
            }
            pub fn max_output_speed_material(&self) -> f32 {
                self.output_speed_inner(None)
            }

            pub fn output_speed_material(
                &self,
                input_material_size_0: f32,
                input_material_size_1: f32,
                input_material_size_2: f32,
                input_material_size_3: f32,
            ) -> f32 {
                self.output_speed_inner(Some((
                    input_material_size_0,
                    input_material_size_1,
                    input_material_size_2,
                    input_material_size_3,
                )))
            }

            // returns
            // (duration, material_input_0, material_input_1, material_input_2, material_input_3, material_output)
            pub fn stats(&self) -> (f32, f32, f32, f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            $duration,
                            $input_speed_0,
                            $input_speed_1,
                            $input_speed_2,
                            $input_speed_3,
                            $output_speed,
                        ),
                    )*
                }
            }
        }
    }
}

r!(
    "Adaptive Control Unit" => AdaptiveControlUnit,
    5. => Material::AutomatedWiring, 5. => Material::CircuitBoard,
    1. => Material::HeavyModularFrame, 2. => Some(Material::Computer),
    60., 1., Material::AdaptiveControlUnit,

    "Ballistic Warp Drive" => BallisticWarpDrive,
    1. => Material::ThermalPropulsionRocket, 5. => Material::SingularityCell,
    2. => Material::SuperpositionOscillator, 40. => Some(Material::DarkMatterCrystal),
    60., 1., Material::BallisticWarpDrive,

    "Computer" => Computer,
    4. => Material::CircuitBoard,
    8. => Material::Cable,
    16. => Material::Plastic,
    0. => None,
    24., 1., Material::Computer,

    "Crystal Oscillator" => CrystalOscillator,
    36. => Material::QuartzCrystal,
    28. => Material::Cable,
    5. => Material::ReinforcedIronPlate,
    0. => None,
    120., 2., Material::CrystalOscillator,

    "Explosive Rebar" => ExplosiveRebar,
    2. => Material::IronRebar,
    2. => Material::SmokelessPowder,
    2. => Material::SteelPipe,
    0. => None,
    12., 1., Material::ExplosiveRebar,

    "Gas Filter" => GasFilter,
    2. => Material::Fabric,
    4. => Material::Coal,
    2. => Material::IronPlate,
    0. => None,
    8., 1., Material::GasFilter,

    "Heavy Modular Frame" => HeavyModularFrame,
    5. => Material::ModularFrame,
    20. => Material::SteelPipe,
    5. => Material::EncasedIndustrialBeam,
    120. => Some(Material::Screw),
    30., 1., Material::HeavyModularFrame,

    "High-Speed Connector" => HighSpeedConnector,
    56. => Material::Quickwire,
    10. => Material::Cable,
    1. => Material::CircuitBoard,
    0. => None,
    16., 1., Material::HighSpeedConnector,

    "Iodine-Infused Filter" => IodineInfusedFilter,
    1. => Material::GasFilter,
    8. => Material::Quickwire,
    1. => Material::AluminumCasing,
    0. => None,
    16., 1., Material::IodineInfusedFilter,

    "Modular Engine" => ModularEngine,
    2. => Material::Motor,
    15. => Material::Rubber,
    2. => Material::SmartPlating,
    0. => None,
    60., 1., Material::ModularEngine,

    "Nuke Nobelisk" => NukeNobelisk,
    5. => Material::Nobelisk,
    20. => Material::EncasedUraniumCell,
    10. => Material::SmokelessPowder,
    6. => Some(Material::AILimiter),
    120., 1., Material::NukeNobelisk,

    "Plutonium Fuel Rod" => PlutoniumFuelRod,
    30. => Material::EncasedPlutoniumCell,
    18. => Material::SteelBeam,
    6. => Material::ElectromagneticControlRod,
    10. => Some(Material::HeatSink),
    240., 1., Material::PlutoniumFuelRod,

    "Radio Control Unit" => RadioControlUnit,
    32. => Material::AluminumCasing,
    1. => Material::CrystalOscillator,
    2. => Material::Computer,
    0. => None,
    48., 2., Material::RadioControlUnit,

    "SAM Fluctuator" => SAMFluctuator,
    6. => Material::ReanimatedSAM,
    5. => Material::Wire,
    3. => Material::SteelPipe,
    0. => None,
    6., 1., Material::SAMFluctuator,

    "Singularity Cell" => SingularityCell,
    1. => Material::NuclearPasta,
    20. => Material::DarkMatterCrystal,
    100. => Material::IronPlate,
    200. => Some(Material::Concrete),
    60., 10., Material::SingularityCell,

    "Supercomputer" => Supercomputer,
    4. => Material::Computer,
    2. => Material::AILimiter,
    3. => Material::HighSpeedConnector,
    28. => Some(Material::Plastic),
    32., 1., Material::Supercomputer,

    "Thermal Propulsion Rocket" => ThermalPropulsionRocket,
    5. => Material::ModularEngine,
    2. => Material::TurboMotor,
    6. => Material::CoolingSystem,
    2. => Some(Material::FusedModularFrame),
    120., 2., Material::ThermalPropulsionRocket,

    "Turbo Motor" => TurboMotor,
    4. => Material::CoolingSystem,
    2. => Material::RadioControlUnit,
    4. => Material::Motor,
    24. => Some(Material::Rubber),
    32., 1., Material::TurboMotor,

    "Turbo Rifle Ammo" => TurboRifleAmmo,
    25. => Material::RifleAmmo,
    3. => Material::AluminumCasing,
    3. => Material::PackagedTurbofuel,
    0. => None,
    12., 50., Material::TurboRifleAmmo,

    "Uranium Fuel Rod" => UraniumFuelRod,
    50. => Material::EncasedUraniumCell,
    3. => Material::EncasedIndustrialBeam,
    5. => Material::ElectromagneticControlRod,
    0. => None,
    150., 1., Material::UraniumFuelRod,

    "Automated Speed Wiring" => AutomatedSpeedWiring,
    2. => Material::Stator,
    40. => Material::Wire,
    1. => Material::HighSpeedConnector,
    0. => None,
    32., 4., Material::AutomatedWiring,

    "Caterium Computer" => CateriumComputer,
    4. => Material::CircuitBoard,
    14. => Material::Quickwire,
    6. => Material::Rubber,
    0. => None,
    16., 1., Material::Computer,

    "Classic Battery" => ClassicBattery,
    6. => Material::Sulfur,
    7. => Material::AlcladAluminumSheet,
    8. => Material::Plastic,
    12. => Some(Material::Wire),
    8., 4., Material::Battery,

    "Flexible Framework" => FlexibleFramework,
    1. => Material::ModularFrame,
    6. => Material::SteelBeam,
    8. => Material::Rubber,
    0. => None,
    16., 2., Material::VersatileFramework,

    "Heavy Encased Frame" => HeavyEncasedFrame,
    8. => Material::ModularFrame,
    10. => Material::EncasedIndustrialBeam,
    36. => Material::SteelPipe,
    22. => Some(Material::Concrete),
    64., 3., Material::HeavyModularFrame,

    "Heavy Flexible Frame" => HeavyFlexibleFrame,
    5. => Material::ModularFrame,
    3. => Material::EncasedIndustrialBeam,
    20. => Material::Rubber,
    104. => Some(Material::Screw),
    16., 1., Material::HeavyModularFrame,

    "Infused Uranium Cell" => InfusedUraniumCell,
    5. => Material::Uranium,
    3. => Material::Silica,
    5. => Material::Sulfur,
    15. => Some(Material::Quickwire),
    12., 4., Material::EncasedUraniumCell,

    "Insulated Crystal Oscillator" => InsulatedCrystalOscillator,
    10. => Material::QuartzCrystal,
    7. => Material::Rubber,
    1. => Material::AILimiter,
    0. => None,
    32., 1., Material::CrystalOscillator,

    "Plastic Smart Plating" => PlasticSmartPlating,
    1. => Material::ReinforcedIronPlate,
    1. => Material::Rotor,
    3. => Material::Plastic,
    0. => None,
    24., 2., Material::SmartPlating,

    "Radio Connection Unit" => RadioConnectionUnit,
    4. => Material::HeatSink,
    2. => Material::HighSpeedConnector,
    12. => Material::QuartzCrystal,
    0. => None,
    16., 1., Material::RadioControlUnit,

    "Radio Control System" => RadioControlSystem,
    1. => Material::CrystalOscillator,
    10. => Material::CircuitBoard,
    60. => Material::AluminumCasing,
    30. => Some(Material::Rubber),
    40., 3., Material::RadioControlUnit,

    "Rigor Motor" => RigorMotor,
    3. => Material::Rotor,
    3. => Material::Stator,
    1. => Material::CrystalOscillator,
    0. => None,
    48., 6., Material::Motor,

    "Silicon High-Speed Connector" => SiliconHighSpeedConnector,
    60. => Material::Quickwire,
    25. => Material::Silica,
    2. => Material::CircuitBoard,
    0. => None,
    40., 2., Material::HighSpeedConnector,

    "Super-State Computer" => SuperStateComputer,
    3. => Material::Computer,
    1. => Material::ElectromagneticControlRod,
    10. => Material::Battery,
    25. => Some(Material::Wire),
    25., 1., Material::Supercomputer,

    "Turbo Electric Motor" => TurboElectricMotor,
    7. => Material::Motor,
    9. => Material::RadioControlUnit,
    5. => Material::ElectromagneticControlRod,
    7. => Some(Material::Rotor),
    64., 3., Material::TurboMotor,

    "Turbo Pressure Motor" => TurboPressureMotor,
    4. => Material::Motor,
    1. => Material::PressureConversionCube,
    24. => Material::PackagedNitrogenGas,
    8. => Some(Material::Stator),
    32., 2., Material::TurboMotor,

    "Uranium Fuel Unit" => UraniumFuelUnit,
    100. => Material::EncasedUraniumCell,
    10. => Material::ElectromagneticControlRod,
    3. => Material::CrystalOscillator,
    10. => Some(Material::Rotor),
    300., 3., Material::UraniumFuelRod,
);

impl ManufacturerRecipe {
    fn output_speed_inner(&self, input_size: Option<(f32, f32, f32, f32)>) -> f32 {
        let input_material_0_size = input_size.map(|(a, _, _, _)| a);
        let input_material_1_size = input_size.map(|(_, b, _, _)| b);
        let input_material_2_size = input_size.map(|(_, _, c, _)| c);
        let input_material_3_size = input_size.map(|(_, _, _, d)| d);
        let (
            duration,
            material_input_0_size,
            material_input_1_size,
            material_input_2_size,
            material_input_3_size,
            material_output_size,
        ) = self.stats();

        if input_material_0_size == Some(0.0)
            || input_material_1_size == Some(0.0)
            || input_material_2_size == Some(0.0)
            || input_material_3_size == Some(0.0)
        {
            return 0.;
        }

        calc_output4(
            input_size,
            duration,
            material_output_size,
            material_input_0_size,
            material_input_1_size,
            material_input_2_size,
            material_input_3_size,
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Manufacturer {
    pub recipe: Option<ManufacturerRecipe>,
    pub speed: f32,
    pub amplified: SomersloopSlot4,
}

impl Default for Manufacturer {
    fn default() -> Self {
        Self {
            recipe: None,
            speed: 100.,
            amplified: SomersloopSlot4::Empty,
        }
    }
}

impl Manufacturer {
    pub fn header_image(&self) -> String {
        load_img("Manufacturer.png")
    }

    pub fn available_recipes(&self) -> &'static [ManufacturerRecipe] {
        ManufacturerRecipe::VARIANTS
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Manufacturer ({})", r.name()),
            None => "Manufacturer".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Smelts more things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        4
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_material_speed(&self) -> (f32, f32, f32, f32) {
        let (base_0, base_1, base_2, base_3) = self
            .recipe
            .as_ref()
            .map(|r| r.input_material_speed())
            .unwrap_or_default();
        let a = (base_0 as f32 * (self.speed / 100.)).round();
        let b = (base_1 as f32 * (self.speed / 100.)).round();
        let c = (base_2 as f32 * (self.speed / 100.)).round();
        let d = (base_3 as f32 * (self.speed / 100.)).round();

        (a, b, c, d)
    }

    pub fn output_material(&self) -> Option<Material> {
        self.recipe.as_ref().map(|r| r.output_material())
    }

    pub fn output_material_speed(
        &self,
        input_material_0_size: f32,
        input_material_1_size: f32,
        input_material_2_size: f32,
        input_material_3_size: f32,
    ) -> f32 {
        let base = self
            .recipe
            .as_ref()
            .map(|r| {
                r.output_speed_material(
                    input_material_0_size,
                    input_material_1_size,
                    input_material_2_size,
                    input_material_3_size,
                )
            })
            .unwrap_or_default();
        let amplification = self.amplified.factor();

        // TODO: take speed into account for input_size

        (base as f32 * (self.speed / 100.) * amplification).round()
    }

    pub fn input_material(&self) -> Option<(Material, Material, Material, Option<Material>)> {
        self.recipe.map(|r| r.input_material())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_speed_packaged_water() {
        assert_eq!(
            ManufacturerRecipe::AdaptiveControlUnit.output_speed_material(0., 0., 0., 0.),
            0.
        );
        assert_eq!(
            ManufacturerRecipe::AdaptiveControlUnit.output_speed_material(0., 10., 10., 10.),
            0.
        );
        assert_eq!(
            ManufacturerRecipe::AdaptiveControlUnit.output_speed_material(10., 10., 10., 0.),
            0.
        );

        assert_eq!(
            ManufacturerRecipe::AdaptiveControlUnit.output_speed_material(5., 5., 1., 2.),
            1.
        );
    }
}
