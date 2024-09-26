use crate::{
    node::{Input, Output},
    util::load_img,
};

use super::Material;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Merger {
    pub current_input_0: Option<Input>,
    pub current_input_1: Option<Input>,
    pub current_input_2: Option<Input>,
}

impl Merger {
    pub fn clear_clone(&self) -> Self {
        let mut this = self.clone();
        this.current_input_0 = None;
        this.current_input_1 = None;
        this.current_input_2 = None;

        this
    }

    pub fn header_image(&self) -> String {
        load_img("Conveyor_Merger.png")
    }

    pub fn name(&self) -> String {
        "Merger".to_string()
    }

    pub fn description(&self) -> String {
        "Merges things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        3
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_material(&self) -> Option<Material> {
        None
    }

    pub fn current_output(&self) -> Option<Output> {
        let mut speed = 0.;
        let mut resource = None;

        if let Some(ref input) = self.current_input_0 {
            speed += input.speed;
            resource = Some(input.resource);
        }

        if let Some(ref input) = self.current_input_1 {
            if resource.is_none() {
                resource = Some(input.resource);
            }
            if resource.is_some() && resource == Some(input.resource) {
                speed += input.speed;
            }
        }

        if let Some(ref input) = self.current_input_2 {
            if resource.is_none() {
                resource = Some(input.resource);
            }
            if resource.is_some() && resource == Some(input.resource) {
                speed += input.speed;
            }
        }

        resource.map(|r| Output { speed, resource: r })
    }
}
