use strum::VariantArray;

use super::{Belt, Material, ResourceType};

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
    pub fn header_image(&self) -> &'static str {
        "file://assets/img/Storage_Container.png"
    }

    pub fn name(&self) -> String {
        match &self.material {
            Some(r) => format!("Storage Container ({})", r.name(),),
            None => "Storage Container".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Stores things".to_string()
    }

    pub fn available_materials(&self) -> &'static [Material] {
        Material::VARIANTS
    }

    pub fn available_levels(&self) -> &'static [Belt] {
        Belt::VARIANTS
    }

    pub fn num_inputs(&self) -> usize {
        0
    }

    pub fn num_outputs(&self) -> usize {
        1
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
}
