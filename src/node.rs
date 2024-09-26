use crate::{
    app::{NodeGraph, Snarl},
    buildings::{Building, Fluid, Material, Selectable},
};
use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum Node {
    Building(Building),
    Group {
        snarl: Snarl,
        graph: NodeGraph,
        /// Number of open inputs
        num_inputs: usize,
        /// Number of open outputs
        num_outputs: usize,
    },
}

impl Node {
    /// Clone, but with caches reset
    pub fn clear_clone(&self) -> Self {
        match self {
            Self::Group { .. } => {
                // TODO
                self.clone()
            }
            Self::Building(b) => Self::Building(b.clear_clone()),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Building(b) => b.name(),
            Self::Group { snarl, .. } => format!("Group ({})", snarl.nodes().count()),
        }
    }

    pub fn header_image(&self) -> Option<String> {
        match self {
            Self::Building(b) => Some(b.header_image()),
            Self::Group { .. } => None,
        }
    }

    pub fn inputs(&self) -> usize {
        match self {
            Self::Building(b) => b.inputs(),
            Self::Group { num_inputs, .. } => *num_inputs,
        }
    }

    pub fn outputs(&self) -> usize {
        match self {
            Self::Building(b) => b.outputs(),
            Self::Group { num_outputs, .. } => *num_outputs,
        }
    }

    pub fn input_resource(&self, input_id: usize) -> ResourceType {
        match self {
            Self::Group { .. } => {
                todo!()
            }
            Self::Building(b) => b.input_resource(input_id),
        }
    }

    pub fn output_resource(&self, output_id: usize) -> ResourceType {
        match self {
            Self::Group { .. } => {
                todo!()
            }
            Self::Building(b) => b.output_resource(output_id),
        }
    }

    pub fn current_output(&self, output_id: usize) -> Option<Output> {
        match self {
            Self::Group { .. } => None,
            Self::Building(b) => b.current_output(output_id),
        }
    }

    pub fn set_current_input(&mut self, input: Output, input_id: usize) {
        match self {
            Self::Group { .. } => {
                // TODO
            }
            Self::Building(b) => b.set_current_input(input, input_id),
        }
    }

    pub fn clear_current_input(&mut self, input_id: usize) {
        match self {
            Self::Group { .. } => {
                // TODO
            }
            Self::Building(b) => b.clear_current_input(input_id),
        }
    }

    pub fn set_current_output_connected(&mut self, output_id: usize) {
        match self {
            Self::Group { .. } => {}
            Self::Building(b) => b.set_current_output_connected(output_id),
        }
    }

    pub fn set_current_output_disconnected(&mut self, output_id: usize) {
        match self {
            Self::Group { .. } => {}
            Self::Building(b) => b.set_current_output_disconnected(output_id),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::Group { .. } => "Group".to_string(),
            Self::Building(b) => b.description(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub speed: f32,
    pub resource: Resource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Input {
    pub speed: f32,
    pub resource: Resource,
}

impl From<Output> for Input {
    fn from(value: Output) -> Self {
        Self {
            speed: value.speed,
            resource: value.resource,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Material,
    Fluid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Resource {
    Material(Material),
    Fluid(Fluid),
}

impl Resource {
    pub fn typ(&self) -> ResourceType {
        match self {
            Self::Material(_) => ResourceType::Material,
            Self::Fluid(_) => ResourceType::Fluid,
        }
    }

    pub fn color(&self) -> Color32 {
        match self {
            Self::Material(m) => m.color(),
            Self::Fluid(f) => f.color(),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Self::Material(m) => m.name(),
            Self::Fluid(f) => f.name(),
        }
    }

    pub fn image(&self) -> String {
        match self {
            Self::Material(m) => m.image(),
            Self::Fluid(f) => f.image(),
        }
    }
}
