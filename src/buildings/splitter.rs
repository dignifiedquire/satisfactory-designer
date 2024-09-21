use crate::util::load_img;

use super::Material;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Splitter {}

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
}
