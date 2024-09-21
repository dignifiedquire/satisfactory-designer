use egui::{vec2, FontId, RichText, Ui, Vec2};
use egui_snarl::{
    ui::{AnyPins, PinInfo, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};

use crate::{
    add_resource_image, add_speed_ui,
    buildings::{
        Building, Constructor, Merger, Miner, OilExtractor, Packager, Refinery, Smelter, Splitter,
        StorageContainer, WaterExtractor,
    },
    node::{Node, Resource},
    BUILDING_COLOR,
};

pub struct Viewer;

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
