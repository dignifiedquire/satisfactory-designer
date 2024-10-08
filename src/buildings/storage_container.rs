use crate::{
    node::{Output, Resource},
    util::load_img,
};

use super::{Belt, Material, Selectable};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct StorageContainer {
    pub material: Option<Material>,
    pub output_belt: Option<Belt>,
}

impl Default for StorageContainer {
    fn default() -> Self {
        Self {
            material: None,
            output_belt: None,
        }
    }
}
impl StorageContainer {
    pub fn clear_clone(&self) -> Self {
        let this = self.clone();
        this
    }

    pub fn header_image(&self) -> String {
        load_img("Storage_Container.png")
    }

    pub fn name(&self) -> String {
        match &self.material {
            Some(r) => format!("Storage Container ({})", r.name()),
            None => "Storage Container".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Stores things".to_string()
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
        crate::node::ResourceType::Material
    }

    pub fn input_speed(&self) -> f32 {
        0.
    }

    pub fn output_speed(&self) -> f32 {
        self.output_belt
            .as_ref()
            .map(|b| b.speed())
            .unwrap_or_default()
    }

    pub fn input_material(&self) -> Option<Material> {
        None
    }
    pub fn output_material(&self) -> Option<Material> {
        self.material
    }

    pub fn current_output(&self) -> Option<Output> {
        self.material.map(|m| Output {
            speed: self.output_speed(),
            resource: Resource::Material(m),
        })
    }
}
