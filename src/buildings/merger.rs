use super::Material;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Merger {}

impl Merger {
    pub fn header_image(&self) -> &'static str {
        "file://assets/img/20px-Conveyor_Merger.png"
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
}
