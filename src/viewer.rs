use egui::{vec2, Color32, FontId, Id, RichText, Ui, Vec2};
use egui_dock::SurfaceIndex;
use egui_snarl::{
    ui::{AnyPins, PinInfo, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};
use strum::VariantArray;

use crate::{
    buildings::{
        Assembler, Building, Constructor, Fluid, Foundry, Material, Merger, Miner, OilExtractor,
        Packager, Refinery, Smelter, SomersloopSlot1, SomersloopSlot2, Splitter, StorageContainer,
        WaterExtractor,
    },
    node::{Node, Resource},
};

const BUILDING_COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0xb0);
const INVALID_COLOR: Color32 = Color32::from_rgb(144, 20, 0);

pub struct Viewer<'a> {
    pub snarl_id_source: String,
    pub snarl_ui_id: Option<Id>,
    pub index: (SurfaceIndex, egui_dock::NodeIndex, usize),
    pub group_edits: &'a mut Vec<(
        SurfaceIndex,
        egui_dock::NodeIndex,
        usize,
        NodeId,
        Snarl<Node>,
    )>,
}

impl Viewer<'_> {
    fn show_input_building(
        &mut self,
        b: &Building,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &Snarl<Node>,
    ) -> PinInfo {
        match b {
            Building::Miner(_) => {
                unreachable!("Miner has no inputs")
            }
            Building::OilExtractor(_) => {
                unreachable!("Oil Extractor has no inputs")
            }
            Building::WaterExtractor(_) => {
                unreachable!("Water extractor has no inputs")
            }
            Building::StorageContainer(_) => {
                unreachable!("Storage Container has no inputs")
            }
            Building::Packager(p) => {
                if pin.id.input == 0 {
                    let max_input_speed =
                        p.recipe.map(|r| r.input_fluid_speed()).unwrap_or_default();
                    let fluid = p.input_fluid().map(Resource::Fluid);
                    single_input(
                        fluid,
                        max_input_speed,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::circle(),
                    )
                } else if pin.id.input == 1 {
                    let max_input_speed = p
                        .recipe
                        .map(|r| r.input_material_speed())
                        .unwrap_or_default();
                    let material = p.input_material().map(Resource::Material);
                    single_input(
                        material,
                        max_input_speed,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else {
                    unreachable!("only two inputs");
                }
            }
            Building::Foundry(f) => {
                if pin.id.input == 0 {
                    let max_input_speed = f
                        .recipe
                        .map(|r| r.input_material_speed().0)
                        .unwrap_or_default();
                    let material = f.input_material().map(|(a, _)| Resource::Material(a));
                    single_input(
                        material,
                        max_input_speed,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else if pin.id.input == 1 {
                    let max_input_speed = f
                        .recipe
                        .map(|r| r.input_material_speed().1)
                        .unwrap_or_default();
                    let material = f.input_material().map(|(_, b)| Resource::Material(b));
                    single_input(
                        material,
                        max_input_speed,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else {
                    unreachable!("only two inputs");
                }
            }
            Building::Assembler(f) => {
                if pin.id.input == 0 {
                    let max_input_speed = f
                        .recipe
                        .map(|r| r.input_material_speed().0)
                        .unwrap_or_default();
                    let material = f.input_material().map(|(a, _)| Resource::Material(a));
                    single_input(
                        material,
                        max_input_speed,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else if pin.id.input == 1 {
                    let max_input_speed = f
                        .recipe
                        .map(|r| r.input_material_speed().1)
                        .unwrap_or_default();
                    let material = f.input_material().map(|(_, b)| Resource::Material(b));
                    single_input(
                        material,
                        max_input_speed,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else {
                    unreachable!("only two inputs");
                }
            }
            Building::Refinery(p) => {
                if pin.id.input == 0 {
                    let max_input_speed =
                        p.recipe.map(|r| r.input_fluid_speed()).unwrap_or_default();
                    let fluid = p.input_fluid().map(Resource::Fluid);
                    single_input(
                        fluid,
                        max_input_speed,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::circle(),
                    )
                } else if pin.id.input == 1 {
                    let max_input_speed = p
                        .recipe
                        .map(|r| r.input_material_speed())
                        .unwrap_or_default();
                    let material = p.input_material().map(Resource::Material);
                    single_input(
                        material,
                        max_input_speed,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else {
                    unreachable!("only two inputs");
                }
            }
            Building::Smelter(ref s) => {
                assert_eq!(pin.id.input, 0, "Smelter node has only one input");

                let material = s.input_material().map(Resource::Material);
                let max_input_speed = s.input_speed();

                single_input(
                    material,
                    max_input_speed,
                    ui,
                    pin,
                    scale,
                    snarl,
                    PinInfo::square(),
                )
            }
            Building::Splitter(_) => {
                assert_eq!(pin.id.input, 0, "Splitter node has only one input");

                let (actual_input_speed, material) = match &*pin.remotes {
                    [] => (0., None),
                    [remote] => {
                        let speed =
                            snarl[remote.node].output_speed(snarl, remote.node, remote.output);
                        let material =
                            snarl[remote.node].output_resource(snarl, remote.node, remote.output);
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
                        let material =
                            snarl[remote.node].output_resource(snarl, remote.node, remote.output);
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

                let material = s.input_material().map(Resource::Material);
                let max_input_speed = s.input_speed();

                single_input(
                    material,
                    max_input_speed,
                    ui,
                    pin,
                    scale,
                    snarl,
                    PinInfo::square(),
                )
            }
        }
    }
}

impl SnarlViewer<Node> for Viewer<'_> {
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
                Node::Group { snarl, .. } => {
                    for node in snarl.nodes() {
                        ui.horizontal(|ui| {
                            let x = 25. * scale;
                            if let Some(img) = node.header_image() {
                                let image = egui::Image::new(img)
                                    .fit_to_exact_size(vec2(x, x))
                                    .show_loading_spinner(true);
                                ui.add(image);
                            } else {
                                ui.add_space(x);
                            }
                            ui.add_space(5. * scale);
                            ui.label(node.name());
                        });
                        ui.add_space(5. * scale);
                    }
                }
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
                                None => "Select Resource".to_string(),
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
                                                Some(*resource),
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
                                    ui.selectable_value(&mut m.level, *level, name);
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
                            if let Some(ref recipe) = p.recipe {
                                let images = recipe.image();
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

                            let text = match &p.recipe {
                                Some(r) => r.name(),
                                None => "Select Recipe".to_string(),
                            };
                            egui::ComboBox::from_id_source(egui::Id::new("packager_recipe"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipe in p.available_recipes() {
                                        let name = recipe.name();
                                        ui.horizontal(|ui| {
                                            let image = match recipe.image() {
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
                                            ui.selectable_value(&mut p.recipe, Some(*recipe), name);
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut p.speed);
                    }
                    Building::Foundry(f) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref recipe) = f.recipe {
                                let image = recipe.image();
                                let image = egui::Image::new(image)
                                    .fit_to_exact_size(vec2(x, x))
                                    .show_loading_spinner(true);
                                ui.add(image);
                            } else {
                                ui.add_space(x * 2.);
                            }

                            let text = match &f.recipe {
                                Some(r) => r.name(),
                                None => "Select Recipe".to_string(),
                            };
                            egui::ComboBox::from_id_source(egui::Id::new("foundry_recipe"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipe in f.available_recipes() {
                                        let name = recipe.name();
                                        ui.horizontal(|ui| {
                                            let image = recipe.image();
                                            let image = egui::Image::new(image)
                                                .fit_to_exact_size(vec2(20., 20.))
                                                .show_loading_spinner(true);
                                            ui.add(image);
                                            ui.selectable_value(&mut f.recipe, Some(*recipe), name);
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut f.speed);

                        ui.add_space(10.0 * scale);
                        add_somersloop2_ui(ui, &mut f.amplified);
                    }
                    Building::Assembler(f) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref recipe) = f.recipe {
                                let image = recipe.image();
                                let image = egui::Image::new(image)
                                    .fit_to_exact_size(vec2(x, x))
                                    .show_loading_spinner(true);
                                ui.add(image);
                            } else {
                                ui.add_space(x * 2.);
                            }

                            let text = match &f.recipe {
                                Some(r) => r.name(),
                                None => "Select Recipe".to_string(),
                            };
                            egui::ComboBox::from_id_source(egui::Id::new("assembler_recipe"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipe in f.available_recipes() {
                                        let name = recipe.name();
                                        ui.horizontal(|ui| {
                                            let image = recipe.image();
                                            let image = egui::Image::new(image)
                                                .fit_to_exact_size(vec2(20., 20.))
                                                .show_loading_spinner(true);
                                            ui.add(image);
                                            ui.selectable_value(&mut f.recipe, Some(*recipe), name);
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut f.speed);

                        ui.add_space(10.0 * scale);
                        egui::ComboBox::from_id_source(egui::Id::new("assembler_amplification"))
                            .selected_text(f.amplified.name())
                            .show_ui(ui, |ui| {
                                for var in SomersloopSlot2::VARIANTS {
                                    let name = var.name();
                                    ui.selectable_value(&mut f.amplified, *var, name);
                                }
                            });
                    }
                    Building::Refinery(p) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref recipe) = p.recipe {
                                let images = recipe.image();
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

                            let text = match &p.recipe {
                                Some(r) => r.name(),
                                None => "Select Recipe".to_string(),
                            };
                            egui::ComboBox::from_id_source(egui::Id::new("refinery_recipe"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipe in p.available_recipes() {
                                        let name = recipe.name();
                                        ui.horizontal(|ui| {
                                            let image = match recipe.image() {
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
                                            ui.selectable_value(&mut p.recipe, Some(*recipe), name);
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut p.speed);

                        ui.add_space(10.0 * scale);
                        add_somersloop2_ui(ui, &mut p.amplified);
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
                            if let Some(ref recipe) = s.recipe {
                                let image = egui::Image::new(recipe.image())
                                    .fit_to_exact_size(vec2(x, x))
                                    .show_loading_spinner(true);
                                ui.add(image);
                            } else {
                                ui.add_space(x);
                            }

                            let text = match &s.recipe {
                                Some(r) => r.name(),
                                None => "Select Recipe".to_string(),
                            };
                            egui::ComboBox::from_id_source(egui::Id::new("smelter_recipe"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipe in s.available_recipes() {
                                        let name = recipe.name();
                                        ui.horizontal(|ui| {
                                            let image = egui::Image::new(recipe.image())
                                                .fit_to_exact_size(vec2(20., 20.))
                                                .show_loading_spinner(true);
                                            ui.add(image);
                                            ui.selectable_value(&mut s.recipe, Some(*recipe), name);
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut s.speed);

                        ui.add_space(10.0 * scale);
                        add_somersloop1_ui(ui, &mut s.amplified);
                    }
                    Building::Splitter(_) => {}
                    Building::Merger(_) => {}
                    Building::Constructor(s) => {
                        ui.horizontal(|ui| {
                            let x = 20. * scale;
                            if let Some(ref recipe) = s.recipe {
                                let image = egui::Image::new(recipe.image())
                                    .fit_to_exact_size(vec2(x, x))
                                    .show_loading_spinner(true);
                                ui.add(image);
                            } else {
                                ui.add_space(x);
                            }

                            let text = match &s.recipe {
                                Some(r) => r.name(),
                                None => "Select Recipe".to_string(),
                            };

                            egui::ComboBox::from_id_source(egui::Id::new("constructor_recipe"))
                                .selected_text(text)
                                .show_ui(ui, |ui| {
                                    for recipe in s.available_recipes() {
                                        let name = recipe.name();

                                        ui.horizontal(|ui| {
                                            let image = egui::Image::new(recipe.image())
                                                .fit_to_exact_size(vec2(20., 20.))
                                                .show_loading_spinner(true);
                                            ui.add(image);
                                            ui.selectable_value(&mut s.recipe, Some(*recipe), name);
                                        });
                                    }
                                });
                        });

                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut s.speed);

                        ui.add_space(10.0 * scale);
                        add_somersloop1_ui(ui, &mut s.amplified);
                    }
                },
            }

            ui.add_space(10.0 * scale);
        });
    }

    fn has_body(&mut self, node: &Node) -> bool {
        match node {
            Node::Building(_) => true,
            Node::Group { .. } => true,
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
        let node = &snarl[node];

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let x = 25. * scale;
                if let Some(img) = node.header_image() {
                    let image = egui::Image::new(img)
                        .fit_to_exact_size(vec2(x, x))
                        .show_loading_spinner(true);
                    ui.add(image);
                    ui.add_space(5. * scale);
                }

                let title = self.title(node);
                let text = RichText::new(title).font(FontId::proportional(15.0 * scale));
                ui.label(text);
                ui.add_space(5. * scale);
            });
        });
    }

    #[inline]
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        // Validate connection
        match (&snarl[from.id.node], &snarl[to.id.node]) {
            (Node::Group { .. }, Node::Group { .. }) => {}
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
        node.name()
    }

    fn inputs(&mut self, node: &Node) -> usize {
        node.inputs()
    }

    fn outputs(&mut self, node: &Node) -> usize {
        node.outputs()
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) -> PinInfo {
        match snarl[pin.id.node] {
            Node::Group { ref snarl, .. } => {
                let mut counter = 0;
                let mut building = None;

                for b in snarl.nodes() {
                    counter += b.inputs();
                    if b.inputs() > 0 && counter > pin.id.input {
                        building = Some((b, counter - pin.id.input - 1));
                        break;
                    }
                }
                let (building, output_id) = building.unwrap();

                let mut fake_pin = pin.clone();
                fake_pin.id.input = output_id;

                let building = match building {
                    Node::Building(b) => b,
                    Node::Group { .. } => todo!("nested groups are not supported yet"),
                };
                self.show_input_building(building, &fake_pin, ui, scale, snarl)
            }
            Node::Building(ref b) => self.show_input_building(b, pin, ui, scale, snarl),
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
            Node::Group { .. } => {
                // TODO
                PinInfo::square().with_fill(BUILDING_COLOR)
            }
            Node::Building(ref b) => match b {
                Building::Miner(_) => {
                    assert_eq!(pin.id.output, 0, "Miner has only one output");
                    let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
                    let material =
                        snarl[pin.id.node].output_resource(snarl, pin.id.node, pin.id.output);
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
                    if pin.id.output == 0 {
                        // Fluid
                        let fluid = p.output_fluid();
                        let max_speed = p
                            .recipe
                            .as_ref()
                            .map(|r| r.max_output_speed_fluid())
                            .unwrap_or_default();

                        fluid_output(fluid, max_speed, ui, scale, pin, snarl)
                    } else if pin.id.output == 1 {
                        // Material
                        let material = p.output_material();
                        let max_speed = p
                            .recipe
                            .as_ref()
                            .map(|r| r.max_output_speed_material())
                            .unwrap_or_default();

                        material_output(material, max_speed, ui, scale, pin, snarl)
                    } else {
                        unreachable!("only two outputs");
                    }
                }
                Building::Refinery(p) => {
                    if pin.id.output == 0 {
                        // Fluid
                        let fluid = p.output_fluid();
                        let max_speed = p
                            .recipe
                            .as_ref()
                            .map(|r| r.max_output_speed_fluid())
                            .unwrap_or_default();

                        fluid_output(fluid, max_speed, ui, scale, pin, snarl)
                    } else if pin.id.output == 1 {
                        // Material
                        let material = p.output_material();
                        let max_speed = p
                            .recipe
                            .as_ref()
                            .map(|r| r.max_output_speed_material())
                            .unwrap_or_default();

                        material_output(material, max_speed, ui, scale, pin, snarl)
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

                    let material = s.output_material();
                    let max_speed = s
                        .recipe
                        .as_ref()
                        .map(|r| r.max_output_speed())
                        .unwrap_or_default();

                    material_output(material, max_speed, ui, scale, pin, snarl)
                }
                Building::Foundry(f) => {
                    assert_eq!(pin.id.output, 0, "Foundry node has only one output");

                    let material = f.output_material();
                    let max_speed = f
                        .recipe
                        .as_ref()
                        .map(|r| r.max_output_speed_material())
                        .unwrap_or_default();

                    material_output(material, max_speed, ui, scale, pin, snarl)
                }
                Building::Assembler(f) => {
                    assert_eq!(pin.id.output, 0, "Assembler node has only one output");

                    let material = f.output_material();
                    let max_speed = f
                        .recipe
                        .as_ref()
                        .map(|r| r.max_output_speed_material())
                        .unwrap_or_default();

                    material_output(material, max_speed, ui, scale, pin, snarl)
                }
                Building::Splitter(_s) => {
                    let (speed, material) = if !pin.remotes.is_empty() {
                        let speed =
                            snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
                        let material =
                            snarl[pin.id.node].output_resource(snarl, pin.id.node, pin.id.output);
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
                            snarl[pin.id.node].output_resource(snarl, pin.id.node, pin.id.output);
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
                    let material = s.output_material();
                    let max_speed = s
                        .recipe
                        .as_ref()
                        .map(|r| r.max_output_speed())
                        .unwrap_or_default();

                    material_output(material, max_speed, ui, scale, pin, snarl)
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
        if ui.button("Add Miner").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Miner(Miner::default())));
            ui.close_menu();
        }
        if ui.button("Add Water Extractor").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::WaterExtractor(WaterExtractor::default())),
            );
            ui.close_menu();
        }
        if ui.button("Add Oil Extractor").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::OilExtractor(OilExtractor::default())),
            );
            ui.close_menu();
        }
        ui.separator();

        if ui.button("Add Smelter").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Smelter(Smelter::default())));
            ui.close_menu();
        }

        if ui.button("Add Foundry").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Foundry(Foundry::default())));
            ui.close_menu();
        }

        ui.separator();
        if ui.button("Add Assembler").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::Assembler(Assembler::default())),
            );
            ui.close_menu();
        }
        if ui.button("Add Constructor").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::Constructor(Constructor::default())),
            );
            ui.close_menu();
        }

        if ui.button("Add Packager").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Packager(Packager::default())));
            ui.close_menu();
        }

        if ui.button("Add Refinery").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Refinery(Refinery::default())));
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Add Splitter").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Splitter(Splitter::default())));
            ui.close_menu();
        }
        if ui.button("Add Merger").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Merger(Merger::default())));
            ui.close_menu();
        }

        ui.separator();
        if ui.button("Add Storage Container").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::StorageContainer(StorageContainer::default())),
            );
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Group").clicked() {
            if let Some(snarl_ui_id) = self.snarl_ui_id {
                let selected = Snarl::<Node>::get_selected_nodes_at(
                    &self.snarl_id_source,
                    snarl_ui_id,
                    ui.ctx(),
                );
                let mut selected = selected
                    .into_iter()
                    .map(|id| (id, &snarl[id]))
                    .collect::<Vec<_>>();

                selected.sort_by_key(|(id, _)| *id);

                let mut buildings = Snarl::new();
                let mut to_remove = Vec::new();
                let mut num_inputs = 0;
                let mut num_outputs = 0;
                for (id, node) in selected {
                    let info = snarl.get_node_info(id).unwrap();
                    num_outputs += node.outputs();
                    num_inputs += node.inputs();
                    buildings.insert_node(info.pos, node.clone());
                    to_remove.push(id);
                }
                // copy wires
                let mut connections = Vec::new();
                for (output, input) in snarl.wires() {
                    if to_remove.contains(&output.node) && to_remove.contains(&input.node) {
                        num_inputs -= 1;
                        num_outputs -= 1;
                        connections.push((output, input));
                    }
                }

                for id in to_remove {
                    snarl.remove_node(id);
                }
                for (output, input) in connections {
                    buildings.connect(output, input);
                }

                snarl.insert_node(
                    pos,
                    Node::Group {
                        snarl: buildings,
                        num_inputs,
                        num_outputs,
                    },
                );
            }

            ui.close_menu();
        }

        ui.separator();
        if ui.button("Clear All").clicked() {
            // TODO: add warning
            *snarl = Snarl::default();
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
        node_id: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        let node = snarl.get_node_info(node_id).expect("missing node");
        ui.label(node.value.name());

        match &node.value {
            Node::Building(_) => {}
            Node::Group { snarl, .. } => {
                if ui.button("Edit").clicked() {
                    self.group_edits.push((
                        self.index.0,
                        self.index.1,
                        self.index.2,
                        node_id,
                        snarl.clone(),
                    ));

                    ui.close_menu();
                }
            }
        }

        if ui.button("Duplicate").clicked() {
            let pos = node.pos + Vec2::new(5., 5.);
            let new_node = node.value.clone();
            snarl.insert_node(pos, new_node);
            ui.close_menu();
        }

        if ui.button("Remove").clicked() {
            snarl.remove_node(node_id);
            ui.close_menu();
        }
    }
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

fn single_input(
    resource: Option<Resource>,
    max_input_speed: f32,
    ui: &mut Ui,
    pin: &InPin,
    scale: f32,
    snarl: &Snarl<Node>,
    pin_info: PinInfo,
) -> PinInfo {
    let (actual_input_speed, actual_input_material) = match &*pin.remotes {
        [] => (0., None),
        [remote] => {
            let speed = snarl[remote.node].output_speed(snarl, remote.node, remote.output);
            let material = snarl[remote.node].output_resource(snarl, remote.node, remote.output);
            (speed, material)
        }
        _ => unreachable!("only one output"),
    };

    let color = match (resource, actual_input_material) {
        (Some(resource), Some(actual_input_material)) => {
            let (actual_input_speed, color) = if resource == actual_input_material {
                let v = format!("{}/min", actual_input_speed);
                let color = resource.color();

                (v, color)
            } else {
                ("NA".to_string(), INVALID_COLOR)
            };
            ui.horizontal(|ui| {
                add_resource_image(ui, scale, &Some(resource));
                ui.label(format!("{} ({}/min)", actual_input_speed, max_input_speed));
            });
            color
        }
        (Some(resource), None) => {
            ui.horizontal(|ui| {
                add_resource_image(ui, scale, &Some(resource));
                ui.label(format!("NA ({}/min)", max_input_speed));
            });
            INVALID_COLOR
        }
        (None, Some(_actual_input_material)) => {
            ui.horizontal(|ui| {
                add_resource_image(ui, scale, &None);
                ui.label(format!("{}/min (NA)", actual_input_speed));
            });

            INVALID_COLOR
        }
        (None, None) => BUILDING_COLOR,
    };

    pin_info.with_fill(color)
}

fn fluid_output(
    fluid: Option<Fluid>,
    max_speed: f32,
    ui: &mut Ui,
    scale: f32,
    pin: &OutPin,
    snarl: &Snarl<Node>,
) -> PinInfo {
    let color = fluid.as_ref().map(|m| m.color()).unwrap_or(BUILDING_COLOR);
    if let Some(fluid) = fluid {
        let output_speed = snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
        let output_fluid = snarl[pin.id.node].output_resource(snarl, pin.id.node, pin.id.output);

        let fluid = Resource::Fluid(fluid);
        match output_fluid {
            Some(_output_fluid) => {
                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &Some(fluid));
                    ui.label(format!("{}/m^3 ({}/m^3)", output_speed, max_speed));
                });
            }
            None => {
                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &Some(fluid));
                    ui.label(format!("NA ({}/m^3)", max_speed));
                });
            }
        }
    }

    PinInfo::circle().with_fill(color)
}

fn material_output(
    material: Option<Material>,
    max_speed: f32,
    ui: &mut Ui,
    scale: f32,
    pin: &OutPin,
    snarl: &Snarl<Node>,
) -> PinInfo {
    let color = material
        .as_ref()
        .map(|m| m.color())
        .unwrap_or(BUILDING_COLOR);
    if let Some(material) = material {
        let output_speed = snarl[pin.id.node].output_speed(snarl, pin.id.node, pin.id.output);
        let output_material = snarl[pin.id.node].output_resource(snarl, pin.id.node, pin.id.output);

        let material = Resource::Material(material);
        match output_material {
            Some(_output_material) => {
                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &Some(material));
                    ui.label(format!("{}/min ({}/min)", output_speed, max_speed));
                });
            }
            None => {
                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &Some(material));
                    ui.label(format!("NA ({}/min)", max_speed));
                });
            }
        }
    }

    PinInfo::circle().with_fill(color)
}

fn add_somersloop1_ui(ui: &mut Ui, amplified: &mut SomersloopSlot1) {
    egui::ComboBox::from_id_source(egui::Id::new("amplification1"))
        .selected_text(amplified.name())
        .show_ui(ui, |ui| {
            for var in SomersloopSlot1::VARIANTS {
                let name = var.name();
                ui.selectable_value(amplified, *var, name);
            }
        });
}

fn add_somersloop2_ui(ui: &mut Ui, amplified: &mut SomersloopSlot2) {
    egui::ComboBox::from_id_source(egui::Id::new("amplification2"))
        .selected_text(amplified.name())
        .show_ui(ui, |ui| {
            for var in SomersloopSlot2::VARIANTS {
                let name = var.name();
                ui.selectable_value(amplified, *var, name);
            }
        });
}
