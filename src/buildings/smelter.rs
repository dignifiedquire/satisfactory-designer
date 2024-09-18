use egui::Color32;

use super::{calc_output, Material};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SmelterRecipie {
    CateriumIngot,
    CopperIngot,
    IronIngot,
    PureAluminiumIngot,
}

impl SmelterRecipie {
    pub fn name(&self) -> String {
        match self {
            SmelterRecipie::CateriumIngot => "Caterium Ingot".to_string(),
            SmelterRecipie::CopperIngot => "Copper Ingot".to_string(),
            SmelterRecipie::IronIngot => "Iron Ingot".to_string(),
            SmelterRecipie::PureAluminiumIngot => "Pure Aluminium Ingot".to_string(),
        }
    }

    pub fn output_color(&self) -> Color32 {
        self.output_material().color()
    }

    pub fn input_material(&self) -> Material {
        match self {
            Self::CateriumIngot => Material::CateriumOre,
            Self::CopperIngot => Material::CopperOre,
            Self::IronIngot => Material::IronOre,
            Self::PureAluminiumIngot => Material::AluminiumScrap,
        }
    }

    pub fn output_material(&self) -> Material {
        match self {
            Self::CateriumIngot => Material::CateriumIngot,
            Self::CopperIngot => Material::CopperIngot,
            Self::IronIngot => Material::IronIngot,
            Self::PureAluminiumIngot => Material::AluminiumIngot,
        }
    }

    pub fn input_speed(&self) -> usize {
        match self {
            Self::CateriumIngot => 45,
            Self::CopperIngot => 30,
            Self::IronIngot => 30,
            Self::PureAluminiumIngot => 60,
        }
    }

    pub fn output_speed(&self, input_size: usize) -> usize {
        if input_size == 0 {
            return 0;
        }

        let (duration, output_size, input_base) = match self {
            Self::CateriumIngot => (4., 1., 3.),
            Self::CopperIngot => (2., 1., 1.),
            Self::IronIngot => (2., 1., 1.),
            Self::PureAluminiumIngot => (2., 1., 2.),
        };

        calc_output(input_size, duration, output_size, input_base)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Smelter {
    pub recipie: Option<SmelterRecipie>,
    pub speed: f32,
    pub amplified: bool,
}

impl Default for Smelter {
    fn default() -> Self {
        Self {
            recipie: None,
            speed: 100.,
            amplified: false,
        }
    }
}

impl Smelter {
    pub fn header_image(&self) -> &'static str {
        "file://assets/img/20px-Smelter.png"
    }

    pub fn available_recipies(&self) -> Vec<SmelterRecipie> {
        vec![
            SmelterRecipie::CateriumIngot,
            SmelterRecipie::CopperIngot,
            SmelterRecipie::IronIngot,
            SmelterRecipie::PureAluminiumIngot,
        ]
    }

    pub fn name(&self) -> String {
        match &self.recipie {
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

    pub fn input_speed(&self) -> usize {
        let base = self
            .recipie
            .as_ref()
            .map(|r| r.input_speed())
            .unwrap_or_default();
        (base as f32 * (self.speed / 100.)).round() as usize
    }

    pub fn output_speed(&self, input_size: usize) -> usize {
        let base = self
            .recipie
            .as_ref()
            .map(|r| r.output_speed(input_size))
            .unwrap_or_default();
        let amplification = if self.amplified { 2. } else { 1. };

        // TODO: take speed into account for input_size

        (base as f32 * (self.speed / 100.) * amplification).round() as usize
    }

    pub fn input_material(&self) -> Option<Material> {
        match self.recipie {
            Some(ref r) => Some(r.input_material()),
            None => None,
        }
    }
}
