use crate::{node::{Input, Output}, util::load_img};

use super::{round, Material};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Splitter {
    pub current_input: Option<Input>,
    pub output_0_connected: bool,
    pub output_1_connected: bool,
    pub output_2_connected: bool,
}

impl Splitter {
    pub fn header_image(&self) -> String {
        load_img("Conveyor_Splitter.png")
    }

    pub fn name(&self) -> String {
        "Splitter".to_string()
    }

    pub fn description(&self) -> String {
        "Splits things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        1
    }

    pub fn num_outputs(&self) -> usize {
        3
    }

    pub fn input_material(&self) -> Option<Material> {
        None
    }

    fn num_outputs_connected(&self) -> usize {
        self.output_0_connected as usize +
        self.output_1_connected as usize +
        self.output_2_connected as usize
    }

    pub fn current_output_0(&self) -> Option<Output> {
        if self.output_0_connected {
            if let Some(ref input) = self.current_input {
                Some(Output {
                    speed: round(input.speed / self.num_outputs_connected() as f32),
                    resource: input.resource,

                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn current_output_1(&self) -> Option<Output> {
        if self.output_1_connected {
            if let Some(ref input) = self.current_input {
                Some(Output {
                    speed: round(input.speed / self.num_outputs_connected() as f32),
                    resource: input.resource,

                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn current_output_2(&self) -> Option<Output> {
        if self.output_2_connected {
            if let Some(ref input) = self.current_input {
                Some(Output {
                    speed: round(input.speed / self.num_outputs_connected() as f32),
                    resource: input.resource,

                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
