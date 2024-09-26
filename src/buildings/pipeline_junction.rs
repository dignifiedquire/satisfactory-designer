use crate::{
    node::{Input, Output, Resource},
    util::load_img,
};

use super::{round, Material};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PipelineJunction {
    pub current_input_0: Option<Input>,
    pub current_input_1: Option<Input>,
    pub current_input_2: Option<Input>,
    pub current_input_3: Option<Input>,
    pub output_0_connected: bool,
    pub output_1_connected: bool,
    pub output_2_connected: bool,
    pub output_3_connected: bool,
}

impl PipelineJunction {
    pub fn clear_clone(&self) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn header_image(&self) -> String {
        load_img("Pipeline_Junction.png")
    }

    pub fn name(&self) -> String {
        "Pipeline Junction".to_string()
    }

    pub fn description(&self) -> String {
        "Junction for fluids".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        4
    }

    pub fn num_outputs(&self) -> usize {
        4
    }

    pub fn input_resource(&self, input_id: usize) -> crate::node::ResourceType {
        assert!(input_id < 4, "4 inputs");
        crate::node::ResourceType::Fluid
    }

    pub fn output_resource(&self, output_id: usize) -> crate::node::ResourceType {
        assert!(output_id < 4, "4 outputs");
        crate::node::ResourceType::Fluid
    }

    pub fn input_material(&self) -> Option<Material> {
        None
    }

    fn current_output_raw(&self) -> (f32, Option<Resource>) {
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
        let speed = round(speed / self.num_outputs_connected() as f32);
        (speed, resource)
    }

    fn num_outputs_connected(&self) -> usize {
        self.output_0_connected as usize
            + self.output_1_connected as usize
            + self.output_2_connected as usize
            + self.output_3_connected as usize
    }

    pub fn current_output_0(&self) -> Option<Output> {
        let (speed, resource) = self.current_output_raw();
        if self.output_0_connected {
            if let Some(resource) = resource {
                Some(Output { speed, resource })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn current_output_1(&self) -> Option<Output> {
        let (speed, resource) = self.current_output_raw();
        if self.output_1_connected {
            if let Some(resource) = resource {
                Some(Output { speed, resource })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn current_output_2(&self) -> Option<Output> {
        let (speed, resource) = self.current_output_raw();
        if self.output_2_connected {
            if let Some(resource) = resource {
                Some(Output { speed, resource })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn current_output_3(&self) -> Option<Output> {
        let (speed, resource) = self.current_output_raw();
        if self.output_3_connected {
            if let Some(resource) = resource {
                Some(Output { speed, resource })
            } else {
                None
            }
        } else {
            None
        }
    }
}
