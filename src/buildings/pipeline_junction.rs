use crate::util::load_img;

use super::Material;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PipelineJunction {}

impl PipelineJunction {
    pub fn clear_clone(&self) -> Self {
        let this = self.clone();
        this
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
}
