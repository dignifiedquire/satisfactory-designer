use crate::{
    node::{Output, Resource},
    util::load_img,
};

use super::{round, Fluid, Pipe};

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
    pub fn clear_clone(&self) -> Self {
        let this = self.clone();
        this
    }

    pub fn header_image(&self) -> String {
        load_img("Water_Extractor.png")
    }

    pub fn name(&self) -> String {
        "Water Extractor".to_string()
    }

    pub fn description(&self) -> String {
        "Extracts water".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        0
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_resource(&self, _input_id: usize) -> crate::node::ResourceType {
        unreachable!("no inputs");
    }

    pub fn output_resource(&self, output_id: usize) -> crate::node::ResourceType {
        assert_eq!(output_id, 0, "1 output");
        crate::node::ResourceType::Fluid
    }

    pub fn output_speed(&self) -> f32 {
        let max = self.output_pipe.map(|p| p.speed()).unwrap_or_default();
        let val = round(120. * (self.speed / 100.));

        if val > max {
            max
        } else {
            val
        }
    }

    pub fn output_fluid(&self) -> Fluid {
        Fluid::Water
    }

    pub fn current_output(&self) -> Option<Output> {
        self.output_pipe.map(|_| Output {
            speed: self.output_speed(),
            resource: Resource::Fluid(Fluid::Water),
        })
    }
}
