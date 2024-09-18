use egui::Color32;

use super::{calc_output, Material};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ConstructorRecipie {
    AlienDnaCapsule,
}

impl ConstructorRecipie {
    pub fn name(&self) -> String {
        match self {
            Self::AlienDnaCapsule => "Alien DNA Capsule".to_string(),
        }
    }

    pub fn output_color(&self) -> Color32 {
        self.output_material().color()
    }

    pub fn input_material(&self) -> Material {
        match self {
            Self::AlienDnaCapsule => Material::AlienProtein,
        }
    }

    pub fn output_material(&self) -> Material {
        match self {
            Self::AlienDnaCapsule => Material::AlienDnaCapsule,
        }
    }

    pub fn input_speed(&self) -> usize {
        match self {
            Self::AlienDnaCapsule => 10,
        }
    }

    pub fn output_speed(&self, input_size: usize) -> usize {
        if input_size == 0 {
            return 0;
        }

        let (duration, output_size, input_base) = match self {
            Self::AlienDnaCapsule => (6., 1., 1.),
        };

        calc_output(input_size, duration, output_size, input_base)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Constructor {
    pub recipie: Option<ConstructorRecipie>,
    pub speed: f32,
    pub amplified: bool,
}

impl Default for Constructor {
    fn default() -> Self {
        Self {
            recipie: None,
            speed: 100.,
            amplified: false,
        }
    }
}

impl Constructor {
    pub fn header_image(&self) -> &'static str {
        "file://assets/img/20px-Constructor.png"
    }

    pub fn available_recipies(&self) -> Vec<ConstructorRecipie> {
        vec![ConstructorRecipie::AlienDnaCapsule]
    }

    pub fn name(&self) -> String {
        match &self.recipie {
            Some(r) => format!("Constructor ({})", r.name()),
            None => "Constructor".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Constructs things".to_string()
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
