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

// /// The speed for this output
// pub fn output_speed(
//     mut initial_node: Option<GraphIdx>,
//     graph: &NodeGraph,
//     remote_node: GraphIdx,
//     remote_node_output: usize,
// ) -> f32 {
//     if initial_node == Some(remote_node) {
//         // circuit breaker
//         return 0.
//     }
//     if initial_node == None {
//         initial_node = Some(remote_node);
//     }
//     match self {
//         Node::Group { .. } => {
//             // TODO
//             0.
//         }
//         Node::Building(b) => match b {
//             Building::Miner(remote_m) => remote_m.output_speed(),
//             Building::OilExtractor(m) => m.output_speed(),
//             Building::WaterExtractor(w) => w.output_speed(),
//             Building::StorageContainer(s) => s.output_speed(),
//             Building::Packager(p) => {
//                 // let input_wire_fluid = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 0);

//                 // let input_wire_material = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 1);

//                 // let input_fluid_speed = input_wire_fluid
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_speed = input_wire_material
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_fluid = input_wire_fluid
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material = input_wire_material
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let expected_input_fluid = p.input_fluid().map(Resource::Fluid);
//                 // let expected_input_material = p.input_material().map(Resource::Material);

//                 // let is_valid = expected_input_fluid == input_fluid
//                 //     && expected_input_material == input_material;
//                 // if !is_valid {
//                 //     return 0.;
//                 // }

//                 // match remote_node_output {
//                 //     0 => p.output_fluid_speed(input_material_speed, input_fluid_speed),
//                 //     1 => p.output_material_speed(input_material_speed, input_fluid_speed),
//                 //     _ => unreachable!("only two outputs"),
//                 // }
//                 todo!()
//             }
//             Building::Foundry(p) => {
//                 // let input_wire_material_0 = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 0);

//                 // let input_wire_material_1 = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 1);

//                 // let input_material_0_speed = input_wire_material_0
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_1_speed = input_wire_material_1
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_0 = input_wire_material_0
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_1 = input_wire_material_1
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let (expected_input_material_0, expected_input_material_1) = p
//                 //     .input_material()
//                 //     .map(|(a, b)| (Resource::Material(a), Resource::Material(b)))
//                 //     .unzip();

//                 // let is_valid = expected_input_material_0 == input_material_0
//                 //     && expected_input_material_1 == input_material_1;
//                 // if !is_valid {
//                 //     return 0.;
//                 // }
//                 // p.output_material_speed(input_material_0_speed, input_material_1_speed)
//                 todo!()
//             }
//             Building::Assembler(p) => {
//                 // let input_wire_material_0 = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 0);

//                 // let input_wire_material_1 = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 1);

//                 // let input_material_0_speed = input_wire_material_0
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_1_speed = input_wire_material_1
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_0 = input_wire_material_0
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_1 = input_wire_material_1
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let (expected_input_material_0, expected_input_material_1) = p
//                 //     .input_material()
//                 //     .map(|(a, b)| (Resource::Material(a), Resource::Material(b)))
//                 //     .unzip();

//                 // let is_valid = expected_input_material_0 == input_material_0
//                 //     && expected_input_material_1 == input_material_1;
//                 // if !is_valid {
//                 //     return 0.;
//                 // }
//                 // p.output_material_speed(input_material_0_speed, input_material_1_speed)
//                 todo!()
//             }
//             Building::Manufacturer(p) => {
//                 // let input_wire_material_0 = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 0);

//                 // let input_wire_material_1 = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 1);

//                 // let input_wire_material_2 = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 2);

//                 // let input_wire_material_3 = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 3);

//                 // let input_material_0_speed = input_wire_material_0
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_1_speed = input_wire_material_1
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_2_speed = input_wire_material_2
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_3_speed = input_wire_material_3
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_0 = input_wire_material_0
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_1 = input_wire_material_1
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_2 = input_wire_material_2
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_3 = input_wire_material_3
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let (
//                 //     expected_input_material_0,
//                 //     expected_input_material_1,
//                 //     expected_input_material_2,
//                 //     expected_input_material_3,
//                 // ) = match p.input_material() {
//                 //     Some((a, b, c, d)) => (
//                 //         Some(Resource::Material(a)),
//                 //         Some(Resource::Material(b)),
//                 //         Some(Resource::Material(c)),
//                 //         d.map(Resource::Material),
//                 //     ),
//                 //     None => (None, None, None, None),
//                 // };

//                 // let is_valid = expected_input_material_0 == input_material_0
//                 //     && expected_input_material_1 == input_material_1
//                 //     && expected_input_material_2 == input_material_2
//                 //     && expected_input_material_3 == input_material_3;
//                 // if !is_valid {
//                 //     return 0.;
//                 // }
//                 // p.output_material_speed(
//                 //     input_material_0_speed,
//                 //     input_material_1_speed,
//                 //     input_material_2_speed,
//                 //     input_material_3_speed,
//                 // )
//                 todo!()
//             }
//             Building::Refinery(p) => {
//                 // let input_wire_fluid = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 0);

//                 // let input_wire_material = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node && input.input == 1);

//                 // let input_fluid_speed = input_wire_fluid
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material_speed = input_wire_material
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_fluid = input_wire_fluid
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material = input_wire_material
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let expected_input_fluid = p.input_fluid().map(Resource::Fluid);
//                 // let expected_input_material = p.input_material().map(Resource::Material);

//                 // let is_valid = expected_input_fluid == input_fluid
//                 //     && expected_input_material == input_material;

//                 // if !is_valid {
//                 //     return 0.;
//                 // }

//                 // match remote_node_output {
//                 //     0 => p.output_fluid_speed(input_material_speed, input_fluid_speed),
//                 //     1 => p.output_material_speed(input_material_speed, input_fluid_speed),
//                 //     _ => unreachable!("only two outputs"),
//                 // }
//                 todo!()
//             }
//             Building::PipelineJunction(_) => {
//                 // let input_wires = snarl
//                 //     .wires()
//                 //     .filter(|(_output, input)| input.node == remote_node);
//                 // let mut input_speed = 0.;
//                 // for (output, _input) in input_wires {
//                 //     // TODO: this is expensive, find a better way
//                 //     let num_connections = snarl
//                 //         .wires()
//                 //         .filter(|(o, _i)| o.node == remote_node)
//                 //         .count() as f32;

//                 //     let base_speed =
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output);

//                 //     input_speed += base_speed / num_connections;
//                 // }
//                 //
//                 // input_speed
//                 todo!()
//             }
//             Building::Splitter(_remote_s) => {
//                 // let input_wire = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node);

//                 // match input_wire {
//                 //     Some((output, _input)) => {
//                 //         // TODO: this is expensive, find a better way
//                 //         let num_connections = snarl
//                 //             .wires()
//                 //             .filter(|(o, _i)| o.node == remote_node)
//                 //             .count() as f32;

//                 //         let base_speed =
//                 //             snarl[output.node].output_speed(initial_node, snarl, output.node, output.output);

//                 //         base_speed / num_connections
//                 //     }
//                 //     None => 0.,
//                 // }
//                 todo!()
//             }
//             Building::Smelter(s) => {
//                 // let input_wire = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node);

//                 // let input_speed = input_wire
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material = input_wire
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let expected_input_material = s.input_material().map(Resource::Material);

//                 // let is_valid = expected_input_material == input_material;
//                 // if !is_valid {
//                 //     return 0.;
//                 // }

//                 // s.output_speed(input_speed)
//                 todo!()
//             }
//             Building::Constructor(c) => {
//                 // let input_wire = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node);

//                 // let input_speed = input_wire
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let input_material = input_wire
//                 //     .map(|(output, _input)| {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     })
//                 //     .unwrap_or_default();

//                 // let expected_input_material = c.input_material().map(Resource::Material);

//                 // let is_valid = expected_input_material == input_material;
//                 // if !is_valid {
//                 //     return 0.;
//                 // }
//                 // c.output_speed(input_speed)
//                 todo!()
//             }
//             Building::Merger(_remote_m) => {
//                 // let wires = snarl
//                 //     .wires()
//                 //     .filter(|(_output, input)| input.node == remote_node);

//                 // let mut speed = 0.;
//                 // for (output, _input) in wires {
//                 //     // TODO: this is expensive, find a better way
//                 //     let num_connections = snarl
//                 //         .wires()
//                 //         .filter(|(o, _i)| o.node == remote_node)
//                 //         .count() as f32;

//                 //     let base_speed =
//                 //         snarl[output.node].output_speed(initial_node, snarl, output.node, output.output);

//                 //     speed += base_speed / num_connections;
//                 // }
//                 // speed
//                 todo!()
//             }
//         },
//     }
// }

// /// The output material
// pub fn output_resource(
//     mut initial_node: Option<GraphIdx>,
//     graph: &NodeGraph,
//     remote_node: GraphIdx,
//     remote_node_output: usize,
// ) -> Option<Resource> {
//     if initial_node == Some(remote_node) {
//         // circuit breaker
//         return None;
//     }
//     if initial_node == None {
//         initial_node = Some(remote_node);
//     }
//     match self {
//         Node::Group { .. } => {
//             // TODO
//             None
//         }
//         Node::Building(b) => match b {
//             Building::Miner(remote_m) => remote_m
//                 .resource
//                 .as_ref()
//                 .map(|r| Resource::Material(r.output_material())),
//             Building::OilExtractor(m) => Some(Resource::Fluid(m.output_fluid())),
//             Building::WaterExtractor(w) => Some(Resource::Fluid(w.output_fluid())),
//             Building::Packager(p) => match remote_node_output {
//                 0 => p
//                     .recipe
//                     .as_ref()
//                     .and_then(|r| r.output_fluid())
//                     .map(Resource::Fluid),
//                 1 => p
//                     .recipe
//                     .as_ref()
//                     .and_then(|r| r.output_material())
//                     .map(Resource::Material),
//                 _ => unreachable!("only two outputs"),
//             },
//             Building::Refinery(p) => match remote_node_output {
//                 0 => p
//                     .recipe
//                     .as_ref()
//                     .and_then(|r| r.output_fluid())
//                     .map(Resource::Fluid),
//                 1 => p
//                     .recipe
//                     .as_ref()
//                     .and_then(|r| r.output_material())
//                     .map(Resource::Material),
//                 _ => unreachable!("only two outputs"),
//             },
//             Building::StorageContainer(s) => s.output_material().map(Resource::Material),
//             Building::PipelineJunction(_) => {
//                 // let input_wire = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node);

//                 // match input_wire {
//                 //     Some((output, _input)) => {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     }
//                 //     None => None,
//                 // }
//                 todo!()
//             }
//             Building::Splitter(_remote_s) => {
//                 // let input_wire = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node);

//                 // match input_wire {
//                 //     Some((output, _input)) => {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     }
//                 //     None => None,
//                 // }
//                 todo!()
//             }
//             Building::Smelter(remote_s) => remote_s
//                 .recipe
//                 .as_ref()
//                 .map(|r| Resource::Material(r.output_material())),
//             Building::Foundry(f) => f.output_material().map(Resource::Material),
//             Building::Assembler(a) => a.output_material().map(Resource::Material),
//             Building::Manufacturer(a) => a.output_material().map(Resource::Material),
//             Building::Constructor(remote_s) => remote_s
//                 .recipe
//                 .as_ref()
//                 .map(|r| Resource::Material(r.output_material())),
//             Building::Merger(_remote_m) => {
//                 // // For now we just grab the first one, as we don't support sushi belts (yet)
//                 // let input_wire = snarl
//                 //     .wires()
//                 //     .find(|(_output, input)| input.node == remote_node);

//                 // match input_wire {
//                 //     Some((output, _input)) => {
//                 //         snarl[output.node].output_resource(initial_node, snarl, output.node, output.output)
//                 //     }
//                 //     None => None,
//                 // }
//                 todo!()
//             }
//         },
//     }
// }
