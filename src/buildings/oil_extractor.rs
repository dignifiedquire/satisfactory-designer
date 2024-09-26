use crate::{
    node::{Output, Resource},
    util::load_img,
};

use super::{miner::ResourcePurity, round, Fluid, Pipe};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct OilExtractor {
    pub output_pipe: Option<Pipe>,
    pub resource_purity: ResourcePurity,
    pub speed: f32,
}

impl Default for OilExtractor {
    fn default() -> Self {
        Self {
            output_pipe: None,
            resource_purity: ResourcePurity::Normal,
            speed: 100.,
        }
    }
}

impl OilExtractor {
    pub fn clear_clone(&self) -> Self {
        let this = self.clone();
        this
    }

    pub fn header_image(&self) -> String {
        load_img("Oil_Extractor.png")
    }

    pub fn name(&self) -> String {
        "Oil Extractor".to_string()
    }

    pub fn description(&self) -> String {
        "Extracts oil".to_string()
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
        let val = round(120. * self.resource_purity.modifier() * (self.speed / 100.));

        if val > max {
            max
        } else {
            val
        }
    }

    pub fn output_fluid(&self) -> Fluid {
        Fluid::CrudeOil
    }

    pub fn current_output(&self) -> Option<Output> {
        self.output_pipe.map(|_| Output {
            speed: self.output_speed(),
            resource: Resource::Fluid(Fluid::CrudeOil),
        })
    }
}
