use buildings::{
    Building, Constructor, Fluid, Material, Merger, Miner, OilExtractor, Packager, Refinery,
    Smelter, Splitter, StorageContainer, WaterExtractor,
};
use eframe::{App, CreationContext};
use egui::{emath::Rot2, vec2, Color32, FontId, Id, Rect, RichText, Ui, Vec2};
use egui_modal::Modal;
use egui_snarl::{
    ui::{AnyPins, BackgroundPattern, PinInfo, SnarlStyle, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};

const BUILDING_COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0xb0);

mod buildings;
mod util;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
enum Node {
    Building(Building),
}

enum Resource {
    Material(Material),
    Fluid(Fluid),
}

impl Resource {
    fn color(&self) -> Color32 {
        match self {
            Self::Material(m) => m.color(),
            Self::Fluid(f) => f.color(),
        }
    }
    fn name(&self) -> String {
        match self {
            Self::Material(m) => m.name(),
            Self::Fluid(f) => f.name(),
        }
    }

    fn image(&self) -> String {
        match self {
            Self::Material(m) => m.image(),
            Self::Fluid(f) => f.image(),
        }
    }
}

impl Node {
    /// The speed for this output
    fn output_speed(
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
    fn output_material(
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

struct Viewer;

impl SnarlViewer<Node> for Viewer {
    fn show_body(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        ui.vertical(|ui| {
            match &mut snarl[node] {
                Node::Building(b) => match b {
                    Building::Miner(m) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref resource) = m.resource {
                                let image = egui::Image::new(resource.image())
                                    .fit_to_exact_size(vec2(x, x))
                                    .show_loading_spinner(true);
                                ui.add(image);
                            } else {
                                ui.add_space(x);
                            }

                            let text = match &m.resource {
                                Some(r) => r.name(),
                                None => "Select Resource",
                            };
                            egui::ComboBox::from_id_source(egui::Id::new("miner_resource"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for resource in m.available_resources() {
                                        let name = resource.name();
                                        ui.horizontal(|ui| {
                                            let image = egui::Image::new(resource.image())
                                                .fit_to_exact_size(vec2(20., 20.))
                                                .show_loading_spinner(true);
                                            ui.add(image);
                                            ui.selectable_value(
                                                &mut m.resource,
                                                Some(resource),
                                                name,
                                            );
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        egui::ComboBox::from_label("Level")
                            .selected_text(m.level.name())
                            .show_ui(ui, |ui| {
                                for level in m.available_levels() {
                                    let name = level.name();
                                    ui.selectable_value(&mut m.level, level, name);
                                }
                            });

                        ui.add_space(10.0 * scale);
                        egui::ComboBox::from_label("Purity")
                            .selected_text(m.resource_purity.name())
                            .show_ui(ui, |ui| {
                                for purity in m.available_purities() {
                                    let name = purity.name();
                                    ui.selectable_value(&mut m.resource_purity, *purity, name);
                                }
                            });
                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut m.speed);
                    }
                    Building::OilExtractor(m) => {
                        let text = match &m.output_pipe {
                            Some(r) => r.name(),
                            None => "Select Pipe".to_string(),
                        };

                        egui::ComboBox::from_label("Pipe")
                            .selected_text(text)
                            .show_ui(ui, |ui| {
                                for pipe in m.available_pipes() {
                                    let name = pipe.name();
                                    ui.selectable_value(&mut m.output_pipe, Some(*pipe), name);
                                }
                            });

                        ui.add_space(10.0 * scale);
                        egui::ComboBox::from_label("Purity")
                            .selected_text(m.resource_purity.name())
                            .show_ui(ui, |ui| {
                                for purity in m.available_purities() {
                                    let name = purity.name();
                                    ui.selectable_value(&mut m.resource_purity, *purity, name);
                                }
                            });
                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut m.speed);
                    }
                    Building::Packager(p) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref recipie) = p.recipie {
                                let images = recipie.image();
                                if let Some(image) = images.0 {
                                    let image = egui::Image::new(image)
                                        .fit_to_exact_size(vec2(x, x))
                                        .show_loading_spinner(true);
                                    ui.add(image);
                                } else {
                                    ui.add_space(x);
                                }
                                if let Some(image) = images.1 {
                                    let image = egui::Image::new(image)
                                        .fit_to_exact_size(vec2(x, x))
                                        .show_loading_spinner(true);
                                    ui.add(image);
                                } else {
                                    ui.add_space(x);
                                }
                            } else {
                                ui.add_space(x * 2.);
                            }

                            let text = match &p.recipie {
                                Some(r) => r.name(),
                                None => "Select Recipie".to_string(),
                            };
                            egui::ComboBox::from_id_source(egui::Id::new("packager_recipie"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipie in p.available_recipies() {
                                        let name = recipie.name();
                                        ui.horizontal(|ui| {
                                            let image = match recipie.image() {
                                                (Some(_image), Some(image)) => image,
                                                (Some(image), None) => image,
                                                (None, Some(image)) => image,
                                                (None, None) => {
                                                    unreachable!("have at least one output")
                                                }
                                            };
                                            let image = egui::Image::new(image)
                                                .fit_to_exact_size(vec2(20., 20.))
                                                .show_loading_spinner(true);
                                            ui.add(image);
                                            ui.selectable_value(
                                                &mut p.recipie,
                                                Some(*recipie),
                                                name,
                                            );
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut p.speed);

                        ui.add_space(10.0 * scale);
                        ui.checkbox(&mut p.amplified, "Sommersloop");
                    }
                    Building::Refinery(p) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref recipie) = p.recipie {
                                let images = recipie.image();
                                if let Some(image) = images.0 {
                                    let image = egui::Image::new(image)
                                        .fit_to_exact_size(vec2(x, x))
                                        .show_loading_spinner(true);
                                    ui.add(image);
                                } else {
                                    ui.add_space(x);
                                }
                                if let Some(image) = images.1 {
                                    let image = egui::Image::new(image)
                                        .fit_to_exact_size(vec2(x, x))
                                        .show_loading_spinner(true);
                                    ui.add(image);
                                } else {
                                    ui.add_space(x);
                                }
                            } else {
                                ui.add_space(x * 2.);
                            }

                            let text = match &p.recipie {
                                Some(r) => r.name(),
                                None => "Select Recipie".to_string(),
                            };
                            egui::ComboBox::from_id_source(egui::Id::new("refinery_recipie"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipie in p.available_recipies() {
                                        let name = recipie.name();
                                        ui.horizontal(|ui| {
                                            let image = match recipie.image() {
                                                (Some(_image), Some(image)) => image,
                                                (Some(image), None) => image,
                                                (None, Some(image)) => image,
                                                (None, None) => {
                                                    unreachable!("have at least one output")
                                                }
                                            };
                                            let image = egui::Image::new(image)
                                                .fit_to_exact_size(vec2(20., 20.))
                                                .show_loading_spinner(true);
                                            ui.add(image);
                                            ui.selectable_value(
                                                &mut p.recipie,
                                                Some(*recipie),
                                                name,
                                            );
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut p.speed);

                        ui.add_space(10.0 * scale);
                        ui.checkbox(&mut p.amplified, "Sommersloop");
                    }
                    Building::WaterExtractor(m) => {
                        let text = match &m.output_pipe {
                            Some(r) => r.name(),
                            None => "Select Pipe".to_string(),
                        };

                        egui::ComboBox::from_label("Pipe")
                            .selected_text(text)
                            .show_ui(ui, |ui| {
                                for pipe in m.available_pipes() {
                                    let name = pipe.name();
                                    ui.selectable_value(&mut m.output_pipe, Some(*pipe), name);
                                }
                            });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut m.speed);
                    }
                    Building::StorageContainer(s) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref material) = s.material {
                                let image = egui::Image::new(material.image())
                                    .fit_to_exact_size(vec2(x, x))
                                    .show_loading_spinner(true);
                                ui.add(image);
                            } else {
                                ui.add_space(x);
                            }

                            let text = match &s.material {
                                Some(m) => m.name(),
                                None => "Select Material".to_string(),
                            };
                            egui::ComboBox::from_id_source(egui::Id::new(
                                "storage_container_material",
                            ))
                            .selected_text(text)
                            .show_ui(ui, |ui| {
                                for material in s.available_materials() {
                                    let name = material.name();
                                    ui.horizontal(|ui| {
                                        let image = egui::Image::new(material.image())
                                            .fit_to_exact_size(vec2(20., 20.))
                                            .show_loading_spinner(true);
                                        ui.add(image);
                                        ui.selectable_value(&mut s.material, Some(*material), name);
                                    });
                                }
                            });
                        });

                        let text = match &s.output_belt {
                            Some(m) => m.name(),
                            None => "Select Belt".to_string(),
                        };

                        ui.add_space(10.0 * scale);
                        egui::ComboBox::from_label("Belt")
                            .selected_text(text)
                            .show_ui(ui, |ui| {
                                for level in s.available_levels() {
                                    let name = level.name();
                                    ui.selectable_value(&mut s.output_belt, Some(*level), name);
                                }
                            });
                    }
                    Building::Smelter(s) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref recipie) = s.recipie {
                                let image = egui::Image::new(recipie.image())
                                    .fit_to_exact_size(vec2(x, x))
                                    .show_loading_spinner(true);
                                ui.add(image);
                            } else {
                                ui.add_space(x);
                            }

                            let text = match &s.recipie {
                                Some(r) => r.name(),
                                None => "Select Recipie".to_string(),
                            };
                            egui::ComboBox::from_id_source(egui::Id::new("smelter_recipie"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipie in s.available_recipies() {
                                        let name = recipie.name();
                                        ui.horizontal(|ui| {
                                            let image = egui::Image::new(recipie.image())
                                                .fit_to_exact_size(vec2(20., 20.))
                                                .show_loading_spinner(true);
                                            ui.add(image);
                                            ui.selectable_value(
                                                &mut s.recipie,
                                                Some(*recipie),
                                                name,
                                            );
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut s.speed);

                        ui.add_space(10.0 * scale);
                        ui.checkbox(&mut s.amplified, "Sommersloop");
                    }
                    Building::Splitter(_) => {}
                    Building::Merger(_) => {}
                    Building::Constructor(s) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref recipie) = s.recipie {
                                let image = egui::Image::new(recipie.image())
                                    .fit_to_exact_size(vec2(x, x))
                                    .show_loading_spinner(true);
                                ui.add(image);
                            } else {
                                ui.add_space(x);
                            }

                            let text = match &s.recipie {
                                Some(r) => r.name(),
                                None => "Select Recipie".to_string(),
                            };

                            egui::ComboBox::from_id_source(egui::Id::new("constructor_recipie"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipie in s.available_recipies() {
                                        let name = recipie.name();

                                        ui.horizontal(|ui| {
                                            let image = egui::Image::new(recipie.image())
                                                .fit_to_exact_size(vec2(20., 20.))
                                                .show_loading_spinner(true);
                                            ui.add(image);
                                            ui.selectable_value(
                                                &mut s.recipie,
                                                Some(*recipie),
                                                name,
                                            );
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut s.speed);

                        ui.add_space(10.0 * scale);
                        ui.checkbox(&mut s.amplified, "Sommersloop");
                    }
                },
            }

            ui.add_space(10.0 * scale);
        });
    }

    fn has_body(&mut self, node: &Node) -> bool {
        match node {
            Node::Building(_) => true,
        }
    }

    fn show_header(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        match &snarl[node] {
            node @ Node::Building(b) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let x = 25. * scale;
                        let image = egui::Image::new(b.header_image())
                            .fit_to_exact_size(vec2(x, x))
                            .show_loading_spinner(true);
                        ui.add(image);
                        ui.add_space(5. * scale);

                        let title = self.title(node);
                        let text = RichText::new(title).font(FontId::proportional(15.0 * scale));
                        ui.label(text);
                        ui.add_space(5. * scale);
                    });
                });
            }
            node => {
                ui.label(self.title(node));
            }
        }
    }

    #[inline]
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        // Validate connection
        match (&snarl[from.id.node], &snarl[to.id.node]) {
            (Node::Building(_), Node::Building(_)) => {}
            (_, Node::Building(_)) => {
                return;
            }
            (Node::Building(_), _) => {
                return;
            }
        }

        for &remote in &to.remotes {
            snarl.disconnect(remote, to.id);
        }

        snarl.connect(from.id, to.id);
    }

    fn title(&mut self, node: &Node) -> String {
        match node {
            Node::Building(b) => b.name(),
        }
    }

    fn inputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Building(b) => b.inputs(),
        }
    }

    fn outputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Building(b) => b.outputs(),
        }
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) -> PinInfo {
        match snarl[pin.id.node] {
            Node::Building(ref b) => match b {
                Building::Miner(_) => {
                    unreachable!("Miner has no inputs")
                }
                Building::OilExtractor(_) => {
                    unreachable!("Miner has no inputs")
                }
                Building::WaterExtractor(_) => {
                    unreachable!("Miner has no inputs")
                }
                Building::StorageContainer(_) => {
                    unreachable!("Storage Container has no inputs")
                }
                Building::Packager(p) => {
                    let (actual_input_speed, resource) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed =
                                snarl[remote.node].output_speed(snarl, remote.node, remote.output);
                            let material = snarl[remote.node].output_material(
                                snarl,
                                remote.node,
                                remote.output,
                            );
                            (speed, material)
                        }
                        _ => unreachable!("only one output"),
                    };

                    let color = resource
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    if pin.id.input == 0 {
                        let max_input_speed = p
                            .recipie
                            .map(|r| r.max_output_speed_fluid())
                            .unwrap_or_default();
                        ui.horizontal(|ui| {
                            add_resource_image(ui, scale, &resource);
                            ui.label(format!(
                                "{}/m^3 ({}/m^3)",
                                actual_input_speed, max_input_speed
                            ));
                        });

                        PinInfo::circle().with_fill(color)
                    } else if pin.id.input == 1 {
                        let max_input_speed = p
                            .recipie
                            .map(|r| r.max_output_speed_fluid())
                            .unwrap_or_default();
                        ui.horizontal(|ui| {
                            add_resource_image(ui, scale, &resource);
                            ui.label(format!(
                                "{}/min ({}/min)",
                                actual_input_speed, max_input_speed
                            ));
                        });

                        PinInfo::square().with_fill(color)
                    } else {
                        unreachable!("only two inputs");
                    }
                }
                Building::Refinery(p) => {
                    let (actual_input_speed, resource) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed =
                                snarl[remote.node].output_speed(snarl, remote.node, remote.output);
                            let material = snarl[remote.node].output_material(
                                snarl,
                                remote.node,
                                remote.output,
                            );
                            (speed, material)
                        }
                        _ => unreachable!("only one output"),
                    };

                    let color = resource
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    if pin.id.input == 0 {
                        let max_input_speed = p
                            .recipie
                            .map(|r| r.max_output_speed_fluid())
                            .unwrap_or_default();
                        ui.horizontal(|ui| {
                            add_resource_image(ui, scale, &resource);
                            ui.label(format!(
                                "{}/m^3 ({}/m^3)",
                                actual_input_speed, max_input_speed
                            ));
                        });

                        PinInfo::circle().with_fill(color)
                    } else if pin.id.input == 1 {
                        let max_input_speed = p
                            .recipie
                            .map(|r| r.max_output_speed_fluid())
                            .unwrap_or_default();
                        ui.horizontal(|ui| {
                            add_resource_image(ui, scale, &resource);
                            ui.label(format!(
                                "{}/min ({}/min)",
                                actual_input_speed, max_input_speed
                            ));
                        });

                        PinInfo::square().with_fill(color)
                    } else {
                        unreachable!("only two inputs");
                    }
                }
                Building::Smelter(ref s) => {
                    assert_eq!(pin.id.input, 0, "Smelter node has only one input");

                    let (actual_input_speed, resource) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed =
                                snarl[remote.node].output_speed(snarl, remote.node, remote.output);
                            let material = snarl[remote.node].output_material(
                                snarl,
                                remote.node,
                                remote.output,
                            );
                            (speed, material)
                        }
                        _ => unreachable!("only one output"),
                    };

                    let max_input_speed = s.input_speed();
                    let color = resource
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &resource);
                        ui.label(format!(
                            "{}/min ({}/min)",
                            actual_input_speed, max_input_speed
                        ));
                    });
                    PinInfo::square().with_fill(color)
                }
                Building::Splitter(_) => {
                    assert_eq!(pin.id.input, 0, "Splitter node has only one input");

                    let (actual_input_speed, material) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed =
                                snarl[remote.node].output_speed(snarl, remote.node, remote.output);
                            let material = snarl[remote.node].output_material(
                                snarl,
                                remote.node,
                                remote.output,
                            );
                            (speed, material)
                        }
                        _ => unreachable!("only one output"),
                    };

                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/min", actual_input_speed,));
                    });

                    PinInfo::square().with_fill(color)
                }
                Building::Merger(_) => {
                    // 3 inputs
                    let (actual_input_speed, material) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed =
                                snarl[remote.node].output_speed(snarl, remote.node, remote.output);
                            let material = snarl[remote.node].output_material(
                                snarl,
                                remote.node,
                                remote.output,
                            );
                            (speed, material)
                        }
                        _ => unreachable!("only one output"),
                    };

                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/min", actual_input_speed,));
                    });

                    PinInfo::square().with_fill(color)
                }
                Building::Constructor(ref s) => {
                    assert_eq!(pin.id.input, 0, "Constructor node has only one input");

                    let (actual_input_speed, material) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed =
                                snarl[remote.node].output_speed(snarl, remote.node, remote.output);
                            let material = snarl[remote.node].output_material(
                                snarl,
                                remote.node,
                                remote.output,
                            );
                            (speed, material)
                        }
                        _ => unreachable!("only one output"),
                    };

                    let max_input_speed = s.input_speed();
                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!(
                            "{}/min ({}/min)",
                            actual_input_speed, max_input_speed
                        ));
                    });

                    PinInfo::square().with_fill(color)
                }
            },
        }
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) -> PinInfo {
        match snarl[pin.id.node] {
            Node::Building(ref b) => match b {
                Building::Miner(_) => {
                    assert_eq!(pin.id.output, 0, "Miner has only one output");
                    let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
                    let material =
                        snarl[pin.id.node].output_material(snarl, pin.id.node, pin.id.output);
                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/min", speed));
                    });

                    PinInfo::square().with_fill(color)
                }
                Building::Packager(p) => {
                    let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
                    let material =
                        snarl[pin.id.node].output_material(snarl, pin.id.node, pin.id.output);
                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    if pin.id.output == 0 {
                        // Fluid
                        if p.recipie.as_ref().and_then(|r| r.output_fluid()).is_some() {
                            let max_speed = p
                                .recipie
                                .as_ref()
                                .map(|r| r.max_output_speed_fluid())
                                .unwrap_or_default();
                            ui.horizontal(|ui| {
                                add_resource_image(ui, scale, &material);
                                ui.label(format!("{}/m^3 ({}/m^3)", speed, max_speed));
                            });
                        }

                        PinInfo::circle().with_fill(color)
                    } else if pin.id.output == 1 {
                        // Material
                        if p.recipie
                            .as_ref()
                            .and_then(|r| r.output_material())
                            .is_some()
                        {
                            let max_speed = p
                                .recipie
                                .as_ref()
                                .map(|r| r.max_output_speed_material())
                                .unwrap_or_default();
                            ui.horizontal(|ui| {
                                add_resource_image(ui, scale, &material);
                                ui.label(format!("{}/min ({}/min)", speed, max_speed));
                            });
                        }
                        PinInfo::square().with_fill(color)
                    } else {
                        unreachable!("only two outputs");
                    }
                }
                Building::Refinery(p) => {
                    let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
                    let material =
                        snarl[pin.id.node].output_material(snarl, pin.id.node, pin.id.output);
                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    if pin.id.output == 0 {
                        // Fluid
                        if p.recipie.as_ref().and_then(|r| r.output_fluid()).is_some() {
                            let max_speed = p
                                .recipie
                                .as_ref()
                                .map(|r| r.max_output_speed_fluid())
                                .unwrap_or_default();
                            ui.horizontal(|ui| {
                                add_resource_image(ui, scale, &material);
                                ui.label(format!("{}/m^3 ({}/m^3)", speed, max_speed));
                            });
                        }

                        PinInfo::circle().with_fill(color)
                    } else if pin.id.output == 1 {
                        // Material
                        if p.recipie
                            .as_ref()
                            .and_then(|r| r.output_material())
                            .is_some()
                        {
                            let max_speed = p
                                .recipie
                                .as_ref()
                                .map(|r| r.max_output_speed_material())
                                .unwrap_or_default();
                            ui.horizontal(|ui| {
                                add_resource_image(ui, scale, &material);
                                ui.label(format!("{}/min ({}/min)", speed, max_speed));
                            });
                        }
                        PinInfo::square().with_fill(color)
                    } else {
                        unreachable!("only two outputs");
                    }
                }
                Building::WaterExtractor(w) => {
                    assert_eq!(pin.id.output, 0, "Water Extractor has only one output");
                    let speed = w.output_speed();
                    let fluid = Resource::Fluid(w.output_fluid());
                    let color = fluid.color();

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &Some(fluid));
                        ui.label(format!("{}/m^3", speed));
                    });

                    PinInfo::circle().with_fill(color)
                }
                Building::OilExtractor(o) => {
                    assert_eq!(pin.id.output, 0, "Oil Extractor has only one output");
                    let speed = o.output_speed();
                    let fluid = Resource::Fluid(o.output_fluid());
                    let color = fluid.color();

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &Some(fluid));
                        ui.label(format!("{}/m^3", speed));
                    });

                    PinInfo::circle().with_fill(color)
                }
                Building::StorageContainer(s) => {
                    assert_eq!(pin.id.output, 0, "Storage Container has only one output");
                    let speed = s.output_speed();
                    let material = s.output_material().map(Resource::Material);
                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/min", speed));
                    });

                    PinInfo::square().with_fill(color)
                }
                Building::Smelter(s) => {
                    assert_eq!(pin.id.output, 0, "Smelter node has only one output");
                    let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
                    let material =
                        snarl[pin.id.node].output_material(snarl, pin.id.node, pin.id.output);

                    let max_speed = s
                        .recipie
                        .as_ref()
                        .map(|r| r.max_output_speed())
                        .unwrap_or_default();

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/min ({}/min)", speed, max_speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::square().with_fill(color)
                }
                Building::Splitter(_s) => {
                    let (speed, material) = if !pin.remotes.is_empty() {
                        let speed =
                            snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
                        let material =
                            snarl[pin.id.node].output_material(snarl, pin.id.node, pin.id.output);
                        (speed, material)
                    } else {
                        (0., None)
                    };

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/min", speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::square().with_fill(color)
                }
                Building::Merger(_m) => {
                    let (speed, material) = if !pin.remotes.is_empty() {
                        let speed =
                            snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
                        let material =
                            snarl[pin.id.node].output_material(snarl, pin.id.node, pin.id.output);
                        (speed, material)
                    } else {
                        (0., None)
                    };

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/min", speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::square().with_fill(color)
                }
                Building::Constructor(s) => {
                    assert_eq!(pin.id.output, 0, "Constructor node has only one output");
                    let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
                    let material =
                        snarl[pin.id.node].output_material(snarl, pin.id.node, pin.id.output);
                    let max_speed = s
                        .recipie
                        .as_ref()
                        .map(|r| r.max_output_speed())
                        .unwrap_or_default();

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/min ({}/min)", speed, max_speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::square().with_fill(color)
                }
            },
        }
    }

    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut Snarl<Node>) -> bool {
        true
    }

    fn show_graph_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        ui.label("Add building");
        ui.separator();

        if ui.button("Miner").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Miner(Miner::default())));
            ui.close_menu();
        }
        if ui.button("Water Extractor").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::WaterExtractor(WaterExtractor::default())),
            );
            ui.close_menu();
        }
        if ui.button("Oil Extractor").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::OilExtractor(OilExtractor::default())),
            );
            ui.close_menu();
        }
        ui.separator();

        if ui.button("Smelter").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Smelter(Smelter::default())));
            ui.close_menu();
        }

        ui.separator();
        if ui.button("Constructor").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::Constructor(Constructor::default())),
            );
            ui.close_menu();
        }

        if ui.button("Packager").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Packager(Packager::default())));
            ui.close_menu();
        }

        if ui.button("Refinery").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Refinery(Refinery::default())));
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Splitter").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Splitter(Splitter::default())));
            ui.close_menu();
        }
        if ui.button("Merger").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Merger(Merger::default())));
            ui.close_menu();
        }

        ui.separator();
        if ui.button("Storage Container").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::StorageContainer(StorageContainer::default())),
            );
            ui.close_menu();
        }
    }

    fn has_dropped_wire_menu(&mut self, _src_pins: AnyPins, _snarl: &mut Snarl<Node>) -> bool {
        true
    }

    fn show_dropped_wire_menu(
        &mut self,
        _pos: egui::Pos2,
        ui: &mut Ui,
        _scale: f32,
        _src_pins: AnyPins,
        _snarl: &mut Snarl<Node>,
    ) {
        ui.label("Add node");
        // TODO:
    }

    fn has_node_menu(&mut self, _node: &Node) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        ui.label("Building");
        if ui.button("Duplicate").clicked() {
            let node = snarl.get_node_info(node).expect("missing node");
            let pos = node.pos + Vec2::new(5., 5.);
            snarl.insert_node(pos, node.value.clone());
            ui.close_menu();
        }

        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close_menu();
        }
    }
}

pub struct DemoApp {
    snarl: Snarl<Node>,
    style: SnarlStyle,
    snarl_ui_id: Option<Id>,
    show_about: bool,
}

impl DemoApp {
    pub fn new(cx: &CreationContext) -> Self {
        egui_extras::install_image_loaders(&cx.egui_ctx);

        cx.egui_ctx.style_mut(|style| {
            style.visuals.extreme_bg_color = Color32::from_hex("#1E1E1E").unwrap();
            style.animation_time *= 10.0;
        });

        let snarl = match cx.storage {
            None => Snarl::new(),
            Some(storage) => storage
                .get_string("snarl")
                .and_then(|snarl| serde_json::from_str(&snarl).ok())
                .unwrap_or_else(Snarl::new),
        };
        let mut style = SnarlStyle::new();
        style
            .bg_pattern
            .replace(BackgroundPattern::custom(|_style, viewport, ui| {
                // Dot grid background

                let spacing = vec2(50.0, 50.0);
                let angle = 0.0;

                let spacing = vec2(spacing.x.max(1.0), spacing.y.max(1.0));

                let rot = Rot2::from_angle(angle);
                let rot_inv = rot.inverse();

                let graph_viewport = Rect::from_min_max(
                    viewport.screen_pos_to_graph(viewport.rect.min),
                    viewport.screen_pos_to_graph(viewport.rect.max),
                );

                let pattern_bounds = graph_viewport.rotate_bb(rot_inv);

                let min_x = (pattern_bounds.min.x / spacing.x).ceil();
                let max_x = (pattern_bounds.max.x / spacing.x).floor();

                let min_y = (pattern_bounds.min.y / spacing.y).ceil();
                let max_y = (pattern_bounds.max.y / spacing.y).floor();

                for x in 0..=(max_x - min_x) as i64 {
                    for y in 0..=(max_y - min_y) as i64 {
                        #[allow(clippy::cast_possible_truncation)]
                        let x = (x as f32 + min_x) * spacing.x;
                        #[allow(clippy::cast_possible_truncation)]
                        let y = (y as f32 + min_y) * spacing.y;

                        let pos = egui::Pos2::new(x, y);
                        let pos = viewport.graph_pos_to_screen(pos);
                        let radius = viewport.scale * 1.0;
                        ui.painter().circle_filled(
                            pos,
                            radius,
                            Color32::from_hex("#7E7E7E").unwrap(),
                        );
                    }
                }
            }));

        DemoApp {
            snarl,
            style,
            snarl_ui_id: None,
            show_about: false,
        }
    }
}

impl App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_switch(ui);

                if ui.button("Clear All").clicked() {
                    self.snarl = Default::default();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.snarl_ui_id = Some(ui.id());

            self.snarl.show(&mut Viewer, &self.style, "snarl", ui);
            if self.show_about {
                let modal = Modal::new(ctx, "About");
                modal.show(|ui| {
                    modal.title(ui, "About");
                    modal.frame(ui, |ui| {
                        ui.add_space(20.);
                        ui.hyperlink(
                            "https://github.com/dignifiedquire/satisfactory-designer",
                        );
                        ui.add_space(20.);
                    });
                    modal.buttons(ui, |ui| {
                        // After clicking, the modal is automatically closed
                        if modal.button(ui, "Ok").clicked() {
                            self.show_about = false;
                        }
                    });
                });
                modal.open();
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let snarl = serde_json::to_string(&self.snarl).unwrap();
        storage.set_string("snarl", snarl);

        let style = serde_json::to_string(&self.style).unwrap();
        storage.set_string("style", style);
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Satisfactory Designer",
        native_options,
        Box::new(|cx| Ok(Box::new(DemoApp::new(cx)))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "Satisfactory Designer",
                web_options,
                Box::new(|cx| Ok(Box::new(DemoApp::new(cx)))),
            )
            .await
            .expect("failed to start eframe");
    });
}

fn add_resource_image(ui: &mut Ui, scale: f32, material: &Option<Resource>) {
    if let Some(material) = material {
        let image = egui::Image::new(material.image())
            .max_height(20. * scale)
            .maintain_aspect_ratio(true)
            .show_loading_spinner(true);
        ui.add(image).on_hover_ui(|ui| {
            ui.style_mut().interaction.selectable_labels = true;
            ui.label(material.name());
        });
    } else {
        ui.add_space(20. * scale);
    }
}

fn add_speed_ui(ui: &mut Ui, value: &mut f32) {
    ui.horizontal(|ui| {
        let overclock = egui::DragValue::new(value).range(0.0..=250.0).suffix("%");
        ui.add(overclock);
        ui.label("Speed");
    });
}
