use crate::{
    node::{Output, Resource},
    util::load_img,
};

use super::{Material, ResourceType, Selectable};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Miner {
    pub resource: Option<ResourceType>,
    pub resource_purity: ResourcePurity,
    pub level: MinerLevel,
    pub speed: f32,
}

impl Default for Miner {
    fn default() -> Self {
        Self {
            resource: None,
            resource_purity: ResourcePurity::Normal,
            level: MinerLevel::Mk1,
            speed: 100.,
        }
    }
}

#[derive(
    Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, strum::VariantArray,
)]
pub enum MinerLevel {
    Mk1,
    Mk2,
    Mk3,
}

impl MinerLevel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Mk1 => "Mk.1",
            Self::Mk2 => "Mk.2",
            Self::Mk3 => "Mk.3",
        }
    }

    pub fn mining_speed(&self) -> usize {
        match self {
            Self::Mk1 => 60,
            Self::Mk2 => 120,
            Self::Mk3 => 240,
        }
    }
}

impl Miner {
    pub fn clear_clone(&self) -> Self {
        Self {
            resource: self.resource.clone(),
            resource_purity: self.resource_purity.clone(),
            level: self.level.clone(),
            speed: self.speed.clone(),
        }
    }

    pub fn header_image(&self) -> String {
        let name = match self.level {
            MinerLevel::Mk1 => "Miner_Mk.1.png",
            MinerLevel::Mk2 => "Miner_Mk.2.png",
            MinerLevel::Mk3 => "Miner_Mk.3.png",
        };
        load_img(name)
    }

    pub fn name(&self) -> String {
        match &self.resource {
            Some(r) => format!(
                "Miner {:?} ({} {})",
                self.level,
                r.name(),
                self.resource_purity.name()
            ),
            None => "Miner".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Mines things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        0
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_speed(&self) -> usize {
        0
    }

    pub fn input_resource(&self, _input_id: usize) -> crate::node::ResourceType {
        unreachable!("no inputs");
    }

    pub fn output_resource(&self, output_id: usize) -> crate::node::ResourceType {
        assert_eq!(output_id, 0, "1 output");
        crate::node::ResourceType::Material
    }

    pub fn output_speed(&self) -> f32 {
        match self.resource {
            Some(_) => {
                // (Mining Speed) in items/min = (Purity Modifier) * (Overclock percentage) / 100 * (Default Mining Speed) items/min
                let val = self.resource_purity.modifier()
                    * (self.speed / 100.)
                    * self.level.mining_speed() as f32;
                val.round()
            }
            None => 0.,
        }
    }

    pub fn input_material(&self) -> Option<Material> {
        None
    }

    pub fn output_material(&self) -> Option<Material> {
        self.resource.map(|r| r.output_material())
    }

    pub fn current_output(&self) -> Option<Output> {
        self.resource.map(|r| Output {
            speed: self.output_speed(),
            resource: Resource::Material(r.output_material()),
        })
    }
}

#[derive(
    Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, strum::VariantArray,
)]
pub enum ResourcePurity {
    Impure,
    Normal,
    Pure,
}

impl ResourcePurity {
    pub fn modifier(&self) -> f32 {
        match self {
            Self::Impure => 0.5,
            Self::Normal => 1.,
            Self::Pure => 2.,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Impure => "Impure",
            Self::Normal => "Normal",
            Self::Pure => "Pure",
        }
    }
}
