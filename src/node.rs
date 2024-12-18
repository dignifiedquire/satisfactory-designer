use crate::{
    app::{NodeGraph, Snarl},
    buildings::{Building, Fluid, Material, Selectable},
};
use egui::Color32;
use petgraph::prelude::NodeIndex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Node {
    Building(Building),
    Group {
        snarl: Snarl,
        graph: NodeGraph,
        inputs: Vec<(egui_snarl::NodeId, NodeIndex, usize, Option<Input>)>,
        outputs: Vec<(egui_snarl::NodeId, NodeIndex, usize, Option<Output>)>,
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
            Self::Group { inputs, .. } => inputs.len(),
        }
    }

    pub fn outputs(&self) -> usize {
        match self {
            Self::Building(b) => b.outputs(),
            Self::Group { outputs, .. } => outputs.len(),
        }
    }

    pub fn input_resource(&self, input_id: usize) -> ResourceType {
        match self {
            Self::Group { inputs, graph, .. } => {
                let (_, node_idx, inner_input_id, _) = &inputs[input_id];
                let node = graph.node_weight(*node_idx).unwrap();
                node.input_resource(*inner_input_id)
            }
            Self::Building(b) => b.input_resource(input_id),
        }
    }

    pub fn output_resource(&self, output_id: usize) -> ResourceType {
        match self {
            Self::Group { outputs, graph, .. } => {
                let (_, node_idx, inner_output_id, _) = &outputs[output_id];
                let node = graph.node_weight(*node_idx).unwrap();
                node.output_resource(*inner_output_id)
            }
            Self::Building(b) => b.output_resource(output_id),
        }
    }

    pub fn current_output(&self, output_id: usize) -> Option<Output> {
        match self {
            Self::Group { outputs, graph, .. } => {
                let (_, node_idx, inner_output_id, _) = &outputs[output_id];
                let node = graph.node_weight(*node_idx).unwrap();
                node.current_output(*inner_output_id)
            }
            Self::Building(b) => b.current_output(output_id),
        }
    }

    pub fn current_input(&self, input_id: usize) -> Option<Input> {
        match self {
            Self::Group { inputs, graph, .. } => {
                let (_, node_idx, inner_input_id, _) = &inputs[input_id];
                let node = graph.node_weight(*node_idx).unwrap();
                node.current_input(*inner_input_id)
            }
            Self::Building(b) => b.current_input(input_id),
        }
    }

    pub fn set_current_input(&mut self, input: Output, input_id: usize) {
        match self {
            Self::Group { inputs, graph, .. } => {
                let (_, node_idx, inner_input_id, _) = &inputs[input_id];
                let node = graph.node_weight_mut(*node_idx).unwrap();
                node.set_current_input(input, *inner_input_id);
            }
            Self::Building(b) => b.set_current_input(input, input_id),
        }
    }

    pub fn clear_current_input(&mut self, input_id: usize) {
        match self {
            Self::Group { inputs, graph, .. } => {
                let (_, node_idx, inner_input_id, _) = &inputs[input_id];
                let node = graph.node_weight_mut(*node_idx).unwrap();
                node.clear_current_input(*inner_input_id);
            }
            Self::Building(b) => b.clear_current_input(input_id),
        }
    }

    pub fn set_current_output_connected(&mut self, output_id: usize) {
        match self {
            Self::Group { outputs, graph, .. } => {
                let (_, node_idx, inner_output_id, _) = &outputs[output_id];
                let node = graph.node_weight_mut(*node_idx).unwrap();
                node.set_current_output_connected(*inner_output_id);
            }
            Self::Building(b) => b.set_current_output_connected(output_id),
        }
    }

    pub fn set_current_output_disconnected(&mut self, output_id: usize) {
        match self {
            Self::Group { outputs, graph, .. } => {
                let (_, node_idx, inner_output_id, _) = &outputs[output_id];
                let node = graph.node_weight_mut(*node_idx).unwrap();
                node.set_current_output_disconnected(*inner_output_id);
            }
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
