use egui::Color32;
use egui_snarl::{NodeId, Snarl};

use crate::buildings::{Building, Fluid, Material};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum Node {
    Building(Building),
}

pub enum Resource {
    Material(Material),
    Fluid(Fluid),
}

impl Resource {
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

impl Node {
    /// The speed for this output
    pub fn output_speed(
        &self,
        snarl: &Snarl<Node>,
        remote_node: NodeId,
        remote_node_output: usize,
    ) -> f32 {
        match self {
            Node::Building(b) => match b {
                Building::Miner(remote_m) => remote_m.output_speed(),
                Building::OilExtractor(m) => m.output_speed(),
                Building::WaterExtractor(w) => w.output_speed(),
                Building::StorageContainer(s) => s.output_speed(),
                Building::Packager(p) => {
                    let input_wire_fluid = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node && input.input == 0);

                    let input_wire_material = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node && input.input == 1);

                    let input_fluid_speed = input_wire_fluid
                        .map(|(output, _input)| {
                            snarl[output.node].output_speed(snarl, output.node, output.output)
                        })
                        .unwrap_or_default();

                    let input_material_speed = input_wire_material
                        .map(|(output, _input)| {
                            snarl[output.node].output_speed(snarl, output.node, output.output)
                        })
                        .unwrap_or_default();

                    match remote_node_output {
                        0 => p.output_fluid_speed(input_material_speed, input_fluid_speed),
                        1 => p.output_material_speed(input_material_speed, input_fluid_speed),
                        _ => unreachable!("only two outputs"),
                    }
                }
                Building::Refinery(p) => {
                    let input_wire_fluid = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node && input.input == 0);

                    let input_wire_material = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node && input.input == 1);

                    let input_fluid_speed = input_wire_fluid
                        .map(|(output, _input)| {
                            snarl[output.node].output_speed(snarl, output.node, output.output)
                        })
                        .unwrap_or_default();

                    let input_material_speed = input_wire_material
                        .map(|(output, _input)| {
                            snarl[output.node].output_speed(snarl, output.node, output.output)
                        })
                        .unwrap_or_default();

                    match remote_node_output {
                        0 => p.output_fluid_speed(input_material_speed, input_fluid_speed),
                        1 => p.output_material_speed(input_material_speed, input_fluid_speed),
                        _ => unreachable!("only two outputs"),
                    }
                }
                Building::Splitter(_remote_s) => {
                    let input_wire = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node);

                    match input_wire {
                        Some((output, _input)) => {
                            // TODO: this is expensive, find a better way
                            let num_connections = snarl
                                .wires()
                                .filter(|(o, _i)| o.node == remote_node)
                                .count() as f32;

                            let base_speed =
                                snarl[output.node].output_speed(snarl, output.node, output.output);

                            base_speed / num_connections
                        }
                        None => 0.,
                    }
                }
                Building::Smelter(remote_s) => {
                    let input_wire = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node);

                    let input_speed = input_wire
                        .map(|(output, _input)| {
                            snarl[output.node].output_speed(snarl, output.node, output.output)
                        })
                        .unwrap_or_default();
                    remote_s.output_speed(input_speed)
                }
                Building::Constructor(remote_s) => {
                    let input_wire = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node);

                    let input_speed = input_wire
                        .map(|(output, _input)| {
                            snarl[output.node].output_speed(snarl, output.node, output.output)
                        })
                        .unwrap_or_default();
                    remote_s.output_speed(input_speed)
                }

                Building::Merger(_remote_m) => {
                    let wires = snarl
                        .wires()
                        .filter(|(_output, input)| input.node == remote_node);

                    let mut speed = 0.;
                    for (output, _input) in wires {
                        // TODO: this is expensive, find a better way
                        let num_connections = snarl
                            .wires()
                            .filter(|(o, _i)| o.node == remote_node)
                            .count() as f32;

                        let base_speed =
                            snarl[output.node].output_speed(snarl, output.node, output.output);

                        speed += base_speed / num_connections;
                    }
                    speed
                }
            },
        }
    }

    /// The output material
    pub fn output_material(
        &self,
        snarl: &Snarl<Node>,
        remote_node: NodeId,
        remote_node_output: usize,
    ) -> Option<Resource> {
        match self {
            Node::Building(b) => match b {
                Building::Miner(remote_m) => remote_m
                    .resource
                    .as_ref()
                    .map(|r| Resource::Material(r.output_material())),
                Building::OilExtractor(m) => Some(Resource::Fluid(m.output_fluid())),
                Building::WaterExtractor(w) => Some(Resource::Fluid(w.output_fluid())),
                Building::Packager(p) => match remote_node_output {
                    0 => p
                        .recipie
                        .as_ref()
                        .and_then(|r| r.output_fluid())
                        .map(Resource::Fluid),
                    1 => p
                        .recipie
                        .as_ref()
                        .and_then(|r| r.output_material())
                        .map(Resource::Material),
                    _ => unreachable!("only two outputs"),
                },
                Building::Refinery(p) => match remote_node_output {
                    0 => p
                        .recipie
                        .as_ref()
                        .and_then(|r| r.output_fluid())
                        .map(Resource::Fluid),
                    1 => p
                        .recipie
                        .as_ref()
                        .and_then(|r| r.output_material())
                        .map(Resource::Material),
                    _ => unreachable!("only two outputs"),
                },
                Building::StorageContainer(s) => s.output_material().map(Resource::Material),
                Building::Splitter(_remote_s) => {
                    let input_wire = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node);

                    match input_wire {
                        Some((output, _input)) => {
                            snarl[output.node].output_material(snarl, output.node, output.output)
                        }
                        None => None,
                    }
                }
                Building::Smelter(remote_s) => remote_s
                    .recipie
                    .as_ref()
                    .map(|r| Resource::Material(r.output_material())),
                Building::Constructor(remote_s) => remote_s
                    .recipie
                    .as_ref()
                    .map(|r| Resource::Material(r.output_material())),
                Building::Merger(_remote_m) => {
                    // For now we just grab the first one, as we don't support sushi belts (yet)
                    let input_wire = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node);

                    match input_wire {
                        Some((output, _input)) => {
                            snarl[output.node].output_material(snarl, output.node, output.output)
                        }
                        None => None,
                    }
                }
            },
        }
    }
}
