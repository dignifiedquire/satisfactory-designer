use strum::VariantArray;

use crate::util::load_img;

use super::{Fluid, Pipe};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct WaterExtractor {
    pub output_pipe: Option<Pipe>,
    pub speed: f32,
}

impl Default for WaterExtractor {
    fn default() -> Self {
        Self {
            output_pipe: None,
            speed: 100.,
        }
    }
}

impl WaterExtractor {
    pub fn header_image(&self) -> String {
        load_img("Water_Extractor.png")
    }

    pub fn name(&self) -> String {
        "Water Extractor".to_string()
    }

    pub fn description(&self) -> String {
        "Extracts water".to_string()
    }

    pub fn available_pipes(&self) -> &'static [Pipe] {
        Pipe::VARIANTS
    }

    pub fn num_inputs(&self) -> usize {
        0
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn output_speed(&self) -> f32 {
        let max = self.output_pipe.map(|p| p.speed()).unwrap_or_default();
        let val = (120. * (self.speed / 100.)).round();

        if val > max {
            max
        } else {
            val
        }
    }

    pub fn output_fluid(&self) -> Fluid {
        Fluid::Water
    }
}
