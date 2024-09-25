use strum::VariantArray;

use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{Belt, Material};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AwesomeSink {
    pub current_input: Option<Input>,
}

impl Default for AwesomeSink {
    fn default() -> Self {
        Self {
            current_input: None,
        }
    }
}
impl AwesomeSink {
    pub fn header_image(&self) -> String {
        load_img("AWESOME_Sink.png")
    }

    pub fn name(&self) -> String {
        "AWESOME Sink".to_string()
    }

    pub fn description(&self) -> String {
        "Awesomely sinks things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        1
    }

    pub fn num_outputs(&self) -> usize {
        0
    }
}
