use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{calc_output2, Material, Selectable, SomersloopSlot2};

macro_rules! r {
    ($($literal_name:expr => $name:ident, $input_speed_0:expr, $input_material_0:expr, $input_speed_1:expr, $input_material_1:expr, $duration:expr, $output_speed:expr, $output_material:expr),* $(,)*) => {
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
        pub enum AssemblerRecipe {
            $(
              #[strum(to_string = $literal_name)]
              $name,
            )*
        }

        impl super::Selectable for AssemblerRecipe {
            const NAME: &'static str = "Recipe";

            fn name(&self) -> String {
                self.to_string()
            }

            fn image(&self) -> String {
                self.output_material().image()
            }
        }

        impl AssemblerRecipe {

            pub fn input_material(&self) -> (Material, Material) {
                match self {
                    $(
                        Self::$name => ($input_material_0, $input_material_1),
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

            pub fn input_material_speed(&self) -> (f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            (60. / $duration) * $input_speed_0,
                            (60. / $duration) * $input_speed_1,
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
            ) -> f32 {
                self.output_speed_inner(Some((input_material_size_0, input_material_size_1)))
            }

            // returns
            // (duration, material_input_0, material_input_1, material_output)
            pub fn stats(&self) -> (f32, f32, f32, f32) {
                match self {
                    $(
                        Self::$name => (
                            $duration,
                            $input_speed_0,
                            $input_speed_1,
                            $output_speed,
                        ),
                    )*
                }
            }
        }
    }
}

r!(
    "AI Limiter" => AILimiter, 5., Material::CopperSheet, 20., Material::Quickwire, 12., 1., Material::AILimiter,
    "Alclad Aluminum Sheet" => AlcladAluminumSheet, 3., Material::AluminumIngot, 1., Material::CopperIngot, 6., 3., Material::AlcladAluminumSheet,
    "Assembly Director System" => AssemblyDirectorSystem, 2., Material::AdaptiveControlUnit, 1., Material::Supercomputer, 80., 1., Material::AssemblyDirectorSystem,
    "Automated Wiring" => AutomatedWiring, 1., Material::Stator, 20., Material::Cable, 24., 1., Material::AutomatedWiring,
    "Black Powder" => BlackPowder, 1., Material::Coal, 1., Material::Sulfur, 4., 2., Material::BlackPowder,
    "Circuit Board" => CircuitBoard, 2., Material::CopperSheet, 4., Material::Plastic, 8., 1., Material::CircuitBoard,
    "Cluster Nobelisk" => ClusterNobelisk, 3., Material::Nobelisk, 4., Material::SmokelessPowder, 24., 1., Material::ClusterNobelisk,
    "Electromagnetic Control Rod" => ElectromagneticControlRod, 3., Material::Stator, 2., Material::AILimiter, 30., 2., Material::ElectromagneticControlRod,
    "Encased Industrial Beam" => EncasedIndustrialBeam, 3., Material::SteelBeam, 6., Material::Concrete, 10., 1., Material::EncasedIndustrialBeam,
    "Encased Plutonium Cell" => EncasedPlutoniumCell, 2., Material::PlutoniumPellet, 4., Material::Concrete, 12., 1., Material::EncasedPlutoniumCell,
    "Fabric" => Fabric, 1., Material::Mycelia, 5., Material::Biomass, 4., 1., Material::Fabric,
    "Gas Nobelisk" => GasNobelisk, 1., Material::Nobelisk, 10., Material::Biomass, 12., 1., Material::GasNobelisk,
    "Heat Sink" => HeatSink, 5., Material::AlcladAluminumSheet, 3., Material::CopperSheet, 8., 1., Material::HeatSink,
    "Homing Rifle Ammo" => HomingRifleAmmo, 20., Material::RifleAmmo, 1., Material::HighSpeedConnector, 24., 10., Material::HomingRifleAmmo,
    "Magnetic Field Generator" => MagneticFieldGenerator, 5., Material::VersatileFramework, 2., Material::ElectromagneticControlRod, 120., 2., Material::MagneticFieldGenerator,
    "Modular Frame" => ModularFrame, 3., Material::ReinforcedIronPlate, 12., Material::IronRod, 60., 2., Material::ModularFrame,
    "Motor" => Motor, 2., Material::Rotor, 2., Material::Stator, 12., 1., Material::Motor,
    "Nobelisk" => Nobelisk, 2., Material::BlackPowder, 2., Material::SteelPipe, 6., 1., Material::Nobelisk,
    "Pulse Nobelisk" => PulseNobelisk, 5., Material::Nobelisk, 1., Material::CrystalOscillator, 60., 5., Material::PulseNobelisk,
    "Reinforced Iron Plate" => ReinforcedIronPlate, 6., Material::IronPlate, 12., Material::Screw, 12., 1., Material::ReinforcedIronPlate,
    "Rifle Ammo" => RifleAmmo, 3., Material::CopperSheet, 2., Material::SmokelessPowder, 12., 15., Material::RifleAmmo,
    "Rotor" => Rotor, 5., Material::IronRod, 25., Material::Screw, 15., 1., Material::Rotor,
    "Shatter Rebar" => ShatterRebar, 2., Material::IronRebar, 3., Material::QuartzCrystal, 12., 1., Material::ShatterRebar,
    "Smart Plating" => SmartPlating, 1., Material::ReinforcedIronPlate, 1., Material::Rotor, 30., 1., Material::SmartPlating,
    "Stator" => Stator, 3., Material::SteelPipe, 8., Material::Wire, 12., 1., Material::Stator,
    "Stun Rebar" => StunRebar, 1., Material::IronRebar, 5., Material::Quickwire, 6., 1., Material::StunRebar,
    "Versatile Framework" => VersatileFramework, 1., Material::ModularFrame, 12., Material::SteelBeam, 24., 2., Material::VersatileFramework,
    "Adhered Iron Plate" => AdheredIronPlate, 3., Material::IronPlate, 1., Material::Rubber, 16., 1., Material::ReinforcedIronPlate,
    "Alclad Casing" => AlcladCasing, 20., Material::AluminumIngot, 10., Material::CopperIngot, 8., 15., Material::AluminumCasing,
    "Automated Miner" => AutomatedMiner, 4., Material::SteelPipe, 4., Material::IronPlate, 60., 1., Material::PortableMiner,
    "Bolted Frame" => BoltedFrame, 3., Material::ReinforcedIronPlate, 56., Material::Screw, 24., 2., Material::ModularFrame,
    "Bolted IronPlate" => BoltedIronPlate, 18., Material::IronPlate, 50., Material::Screw, 12., 3., Material::ReinforcedIronPlate,
    "Caterium Circuit Board" => CateriumCircuitBoard, 10., Material::Plastic, 30., Material::Quickwire, 48., 7., Material::CircuitBoard,
    "Cheap Silica" => CheapSilica, 3., Material::RawQuartz, 5., Material::Limestone, 8., 7., Material::Silica,
    "Coated Iron Canister" => CoatedIronCanister, 2., Material::IronPlate, 1., Material::CopperSheet, 4., 4., Material::EmptyCanister,
    "Coated Iron Plate" => CoatedIronPlate, 5., Material::IronIngot, 1., Material::Plastic, 8., 10., Material::IronPlate,
    "Compacted Coal" => CompactedCoal, 5., Material::Coal, 5., Material::Sulfur, 12., 5., Material::CompactedCoal,
    "Copper Rotor" => CopperRotor, 6., Material::CopperSheet, 52., Material::Screw, 16., 3., Material::Rotor,
    "Crystal Computer" => CrystalComputer, 3., Material::CircuitBoard, 1., Material::CrystalOscillator, 36., 2., Material::Computer,
    "Electric Motor" => ElectricMotor, 1., Material::ElectromagneticControlRod, 2., Material::Rotor, 16., 2., Material::Motor,
    "Electrode Circuit Board" => ElectrodeCircuitBoard, 4., Material::Rubber, 8., Material::PetroleumCoke, 12., 1., Material::CircuitBoard,
    "Electromagnetic Connection Rod" => ElectromagneticConnectionRod, 2., Material::Stator, 1., Material::HighSpeedConnector, 15., 2., Material::ElectromagneticControlRod,
    "Encased Industrial Pipe" => EncasedIndustrialPipe, 6., Material::SteelPipe, 5., Material::Concrete, 15., 1., Material::EncasedIndustrialBeam,
    "Fine Black Powder" => FineBlackPowder, 1., Material::Sulfur, 2., Material::CompactedCoal, 8., 6., Material::BlackPowder,
    "Fine Concrete" => FineConcrete, 3., Material::Silica, 12., Material::Limestone, 12., 10., Material::Concrete,
    "Fused Quickwire" => FusedQuickwire, 1., Material::CateriumIngot, 5., Material::CopperIngot, 8., 12., Material::Quickwire,
    "Fused Wire" => FusedWire, 4., Material::CopperIngot, 1., Material::CateriumIngot, 20., 30., Material::Wire,
    "Heat Exchanger" => HeatExchanger, 3., Material::AluminumCasing, 3., Material::Rubber, 6., 1., Material::HeatSink,
    "Insulated Cable" => InsulatedCable, 9., Material::Wire, 6., Material::Rubber, 12., 20., Material::Cable,
    "OC Supercomputer" => OCSupercomputer, 2., Material::RadioControlUnit, 2., Material::CoolingSystem, 20., 1., Material::Supercomputer,
    "Plastic AI Limiter" => PlasticAILimiter, 30., Material::Quickwire, 7., Material::Plastic, 15., 2., Material::AILimiter,
    "Plutonium Fuel Unit" => PlutoniumFuelUnit, 20., Material::EncasedPlutoniumCell, 1., Material::PressureConversionCube, 120., 1., Material::PlutoniumFuelRod,
    "Quickwire Cable" => QuickwireCable, 3., Material::Quickwire, 2., Material::Rubber, 24., 11., Material::Cable,
    "Quickwire Stator" => QuickwireStator, 4., Material::SteelPipe, 15., Material::Quickwire, 15., 2., Material::Stator,
    "Rubber Concrete" => RubberConcrete, 10., Material::Limestone, 2., Material::Rubber, 6., 9., Material::Concrete,
    "Silicon Circuit Board" => SiliconCircuitBoard, 11., Material::CopperSheet, 11., Material::Silica, 24., 5., Material::CircuitBoard,
    "Steel Rotor" => SteelRotor, 2., Material::SteelPipe, 6., Material::Wire, 12., 1., Material::Rotor,
    "Steeled Frame" => SteeledFrame, 2., Material::ReinforcedIronPlate, 10., Material::SteelPipe, 60., 3., Material::ModularFrame,
    "Stitched Iron Plate" => StitchedIronPlate, 10., Material::IronPlate, 20., Material::Wire, 32., 3., Material::ReinforcedIronPlate,
);

impl AssemblerRecipe {
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
pub struct Assembler {
    pub recipe: Option<AssemblerRecipe>,
    pub speed: f32,
    pub amplified: SomersloopSlot2,
    pub current_input_material_0: Option<Input>,
    pub current_input_material_1: Option<Input>,
}

impl Default for Assembler {
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

impl Assembler {
    pub fn clear_clone(&self) -> Self {
        let mut this = self.clone();
        this.current_input_material_0 = None;
        this.current_input_material_1 = None;
        this
    }

    pub fn header_image(&self) -> String {
        load_img("Assembler.png")
    }

    pub fn name(&self) -> String {
        match &self.recipe {
            Some(r) => format!("Assembler ({})", r.name()),
            None => "Assembler".to_string(),
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
            0 => crate::node::ResourceType::Fluid,
            _ => unreachable!("1 output"),
        }
    }

    pub fn input_material_speed(&self) -> (f32, f32) {
        let (base_0, base_1) = self
            .recipe
            .as_ref()
            .map(|r| r.input_material_speed())
            .unwrap_or_default();
        let a = (base_0 as f32 * (self.speed / 100.)).round();
        let b = (base_1 as f32 * (self.speed / 100.)).round();

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

        (base as f32 * (self.speed / 100.) * amplification).round()
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
        assert_eq!(AssemblerRecipe::AILimiter.output_speed_material(0., 0.), 0.);
        assert_eq!(
            AssemblerRecipe::AILimiter.output_speed_material(25., 100.),
            5.
        );

        assert_eq!(
            AssemblerRecipe::AssemblyDirectorSystem.output_speed_material(0., 0.),
            0.
        );
        assert_eq!(
            AssemblerRecipe::AssemblyDirectorSystem.output_speed_material(1.5, 0.75),
            0.75
        );
    }
}
