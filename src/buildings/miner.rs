use super::{Material, ResourceType};

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

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
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
    pub fn header_image(&self) -> &'static str {
        "file://assets/img/20px-Miner_Mk.1.png"
    }

    pub fn name(&self) -> String {
        match &self.resource {
            Some(r) => format!(
                "Miner {:?} ({} {})",
                self.level,
                r.name(),
                self.resource_purity.name()
            ),
            None => "Resource Node".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Mines things".to_string()
    }

    pub fn available_purities(&self) -> Vec<ResourcePurity> {
        vec![
            ResourcePurity::Impure,
            ResourcePurity::Normal,
            ResourcePurity::Pure,
        ]
    }

    pub fn available_levels(&self) -> Vec<MinerLevel> {
        vec![MinerLevel::Mk1, MinerLevel::Mk2, MinerLevel::Mk3]
    }

    pub fn available_resources(&self) -> Vec<ResourceType> {
        use ResourceType::*;
        vec![
            Bauxite,
            CateriumOre,
            CopperOre,
            IronOre,
            Limestone,
            RawQuartz,
            SamOre,
            Sulfur,
            Uranium,
        ]
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

    pub fn output_speed(&self) -> usize {
        match self.resource {
            Some(_) => {
                // (Mining Speed) in items/min = (Purity Modifier) * (Overclock percentage) / 100 * (Default Mining Speed) items/min
                let val = self.resource_purity.modifier()
                    * (self.speed / 100.)
                    * self.level.mining_speed() as f32;
                val.round() as usize
            }
            None => 0,
        }
    }

    pub fn input_material(&self) -> Option<Material> {
        None
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
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
