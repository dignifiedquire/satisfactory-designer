use egui::{vec2, Color32, FontId, Id, RichText, Ui, Vec2};
use egui_dock::SurfaceIndex;
use egui_snarl::{
    ui::{AnyPins, PinInfo, SnarlViewer},
    InPin, NodeId, OutPin,
};
use strum::VariantArray;

use crate::{
    app::{EdgeDetails, GraphIdx, NodeGraph, Snarl},
    buildings::{Building, Fluid, Material, SomersloopSlot1, SomersloopSlot2, SomersloopSlot4},
    node::{Input, Node, Output, Resource},
};

const BUILDING_COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0xb0);
const INVALID_COLOR: Color32 = Color32::from_rgb(144, 20, 0);

pub struct Viewer<'a> {
    pub snarl_id_source: String,
    pub snarl_ui_id: Option<Id>,
    pub index: (SurfaceIndex, egui_dock::NodeIndex, usize),
    pub graph: &'a mut NodeGraph,
    pub group_edits: &'a mut Vec<(
        SurfaceIndex,
        egui_dock::NodeIndex,
        usize,
        NodeId,
        NodeGraph,
        Snarl,
    )>,
}

impl Viewer<'_> {
    fn show_input_building(
        &self,
        graph_idx: GraphIdx,
        b: &Building,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &Snarl,
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

                    let actual_input_speed = p
                        .current_input_fluid
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_fluid = p.current_input_fluid.as_ref().map(|i| i.resource);
                    single_input(
                        fluid,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_fluid,
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
                    let actual_input_speed = p
                        .current_input_material
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        p.current_input_material.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
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
                    let actual_input_speed = f
                        .current_input_material_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        f.current_input_material_0.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
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
                    let actual_input_speed = f
                        .current_input_material_1
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        f.current_input_material_1.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
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

                    let actual_input_speed = f
                        .current_input_material_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        f.current_input_material_0.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
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
                    let actual_input_speed = f
                        .current_input_material_1
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        f.current_input_material_1.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
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
            Building::Manufacturer(f) => match pin.id.input {
                0 => {
                    let max_input_speed = f
                        .recipe
                        .map(|r| r.input_material_speed().0)
                        .unwrap_or_default();
                    let material = f.input_material().map(|(a, _, _, _)| Resource::Material(a));
                    let actual_input_speed = f
                        .current_input_material_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        f.current_input_material_0.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                1 => {
                    let max_input_speed = f
                        .recipe
                        .map(|r| r.input_material_speed().1)
                        .unwrap_or_default();
                    let material = f.input_material().map(|(a, _, _, _)| Resource::Material(a));
                    let actual_input_speed = f
                        .current_input_material_1
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        f.current_input_material_1.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                2 => {
                    let max_input_speed = f
                        .recipe
                        .map(|r| r.input_material_speed().2)
                        .unwrap_or_default();
                    let material = f.input_material().map(|(_, _, c, _)| Resource::Material(c));
                    let actual_input_speed = f
                        .current_input_material_2
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        f.current_input_material_2.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                3 => {
                    let max_input_speed = f
                        .recipe
                        .map(|r| r.input_material_speed().3)
                        .unwrap_or_default();
                    let material = f
                        .input_material()
                        .and_then(|(_, _, _, d)| d.map(Resource::Material));
                    let actual_input_speed = f
                        .current_input_material_3
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        f.current_input_material_3.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        pin,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                _ => unreachable!("only four inputs"),
            },
            Building::Refinery(p) => {
                if pin.id.input == 0 {
                    let max_input_speed =
                        p.recipe.map(|r| r.input_fluid_speed()).unwrap_or_default();
                    let fluid = p.input_fluid().map(Resource::Fluid);
                    let actual_input_speed = p
                        .current_input_fluid
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_fluid = p.current_input_fluid.as_ref().map(|i| i.resource);
                    single_input(
                        fluid,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_fluid,
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
                    let actual_input_speed = p
                        .current_input_material
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        p.current_input_material.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
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
                let actual_input_speed = s
                    .current_input
                    .as_ref()
                    .map(|i| i.speed)
                    .unwrap_or_default();
                let actual_input_material = s.current_input.as_ref().map(|i| i.resource);
                single_input(
                    material,
                    max_input_speed,
                    actual_input_speed,
                    actual_input_material,
                    ui,
                    pin,
                    scale,
                    snarl,
                    PinInfo::square(),
                )
            }
            Building::PipelineJunction(_) => {
                // 4 inputs
                let actual_input_speed = 0.;
                let input_fluid = None;

                let color = input_fluid
                    .as_ref()
                    .map(|m: &Resource| m.color())
                    .unwrap_or(BUILDING_COLOR);

                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &input_fluid);
                    ui.label(format!("{}/m^3", actual_input_speed,));
                });

                PinInfo::circle().with_fill(color)
            }
            Building::Splitter(s) => {
                assert_eq!(pin.id.input, 0, "Splitter node has only one input");

                let (actual_input_speed, material) = match s.current_input {
                    Some(ref input) => {
                        (input.speed, Some(input.resource))
                    }
                    None => (0., None),
                };

                let color = material
                    .as_ref()
                    .map(|m: &Resource| m.color())
                    .unwrap_or(BUILDING_COLOR);

                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &material);
                    ui.label(format!("{}/min", actual_input_speed,));
                });

                PinInfo::square().with_fill(color)
            }
            Building::Merger(m) => {
                // 3 inputs

                let current_input = match pin.id.input {
                    0 => &m.current_input_0,
                    1 => &m.current_input_1,
                    2 => &m.current_input_2,
                    _ => unreachable!("3 inputs"),
                };

                let resource = current_input.as_ref().map(|i| i.resource);
                let color = resource.map(|r| r.color()).unwrap_or(BUILDING_COLOR);
                let actual_input_speed =
                    current_input.as_ref().map(|r| r.speed).unwrap_or_default();

                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &resource);
                    ui.label(format!("{}/min", actual_input_speed));
                });

                PinInfo::square().with_fill(color)
            }
            Building::Constructor(ref s) => {
                assert_eq!(pin.id.input, 0, "Constructor node has only one input");

                let material = s.input_material().map(Resource::Material);
                let max_input_speed = s.input_speed();

                let actual_input_speed = s
                    .current_input
                    .as_ref()
                    .map(|i| i.speed)
                    .unwrap_or_default();
                let actual_input_material = s.current_input.as_ref().map(|i| i.resource);

                single_input(
                    material,
                    max_input_speed,
                    actual_input_speed,
                    actual_input_material,
                    ui,
                    pin,
                    scale,
                    snarl,
                    PinInfo::square(),
                )
            }
            Building::AwesomeSink(ref s) => {
                assert_eq!(pin.id.input, 0, "Awesome sink node has only one input");

                let color = match s.current_input {
                    Some(ref input) => {
                        let color = input.resource.color();

                        ui.horizontal(|ui| {
                            add_resource_image(ui, scale, &Some(input.resource));
                            ui.label(format!("{}/min", input.speed));
                        });
                        color
                    }
                    None => BUILDING_COLOR,
                };

                PinInfo::square().with_fill(color)
            }
        }
    }
}

impl SnarlViewer<GraphIdx> for Viewer<'_> {
    fn show_body(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl,
    ) {
        ui.vertical(|ui| {
            let graph_idx = snarl[node];
            let node = self.graph.node_weight_mut(graph_idx).unwrap();
            match node {
                Node::Group { snarl, .. } => {
                    todo!()
                    // for node in snarl.nodes() {
                    //     ui.horizontal(|ui| {
                    //         let x = 25. * scale;
                    //         if let Some(img) = node.header_image() {
                    //             let image = egui::Image::new(img)
                    //                 .fit_to_exact_size(vec2(x, x))
                    //                 .show_loading_spinner(true);
                    //             ui.add(image);
                    //         } else {
                    //             ui.add_space(x);
                    //         }
                    //         ui.add_space(5. * scale);
                    //         ui.label(node.name());
                    //     });
                    //     ui.add_space(5. * scale);
                    // }
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
                    Building::Manufacturer(f) => {
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
                            egui::ComboBox::from_id_source(egui::Id::new("manufacturer_recipe"))
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
                        egui::ComboBox::from_id_source(egui::Id::new("manufacturer_amplification"))
                            .selected_text(f.amplified.name())
                            .show_ui(ui, |ui| {
                                for var in SomersloopSlot4::VARIANTS {
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
                    Building::PipelineJunction(_) => {}
                    Building::Splitter(_) => {}
                    Building::Merger(_) => {}
                    Building::AwesomeSink(_) => {}
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

    fn has_body(&mut self, node: &GraphIdx) -> bool {
        let node = self.graph.node_weight(*node).unwrap();
        match node {
            Node::Building(_) => true,
            Node::Group { .. } => true,
        }
    }

    fn show_header(
        &mut self,
        node_id: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl,
    ) {
        let graph_idx = snarl[node_id];
        let node = self.graph.node_weight_mut(graph_idx).unwrap();

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

                let title = node.name();
                let text = RichText::new(title).font(FontId::proportional(15.0 * scale));
                ui.label(text);
                ui.add_space(5. * scale);
            });
        });
    }

    #[inline]
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl) {
        // TODO: Validate connection

        // Update graph
        let node_from_idx = snarl[from.id.node];
        let node_to_idx = snarl[to.id.node];

        // connect graph
        self.graph.add_edge(
            node_from_idx,
            node_to_idx,
            EdgeDetails {
                input: to.id.input,
                output: from.id.output,
            },
        );

        // connect snarl
        snarl.connect(from.id, to.id);

        // Update cached values
        let node_from = self.graph.node_weight_mut(node_from_idx).unwrap();
        node_from.set_current_output_connected(from.id.output);
        let node_from_output = node_from.current_output(from.id.output);

        let externals = self.graph.externals(petgraph::Direction::Outgoing);
        for target in externals {
            println!("{}", self.graph.node_weight(target).unwrap().name());
            let paths = petgraph::algo::all_simple_paths::<Vec<_>, _>(
                &*self.graph,
                node_to_idx,
                target,
                1,
                None,
            );
            for path in paths {
                let path: Vec<_> = path
                    .into_iter()
                    .map(|n| self.graph.node_weight(n).unwrap().name())
                    .collect();
                println!("{:?}", path);
            }
        }

        dbg!(&node_from_output);

        // - grab output from "from"

        if let Some(output) = node_from_output {
            let node_to = self.graph.node_weight_mut(node_to_idx).unwrap();
            node_to.set_current_input(output, to.id.input);

            // move to function
            let parent_idx = node_from_idx;

            // walk affected nodes
            let mut neighbors = self
                .graph
                .neighbors_directed(parent_idx, petgraph::Direction::Outgoing)
                .detach();
            while let Some(node_idx) = neighbors.next_node(&self.graph) {
                let mut neighbors = self
                    .graph
                    .neighbors_directed(node_idx, petgraph::Direction::Outgoing)
                    .detach();

                while let Some((edge_idx, next_node_idx)) = neighbors.next(&self.graph) {
                    let edge = self.graph.edge_weight(edge_idx).unwrap().clone();

                    let node = self.graph.node_weight(node_idx).unwrap();
                    let output = node.current_output(edge.output);
                    let next_node = self.graph.node_weight_mut(next_node_idx).unwrap();

                    match output {
                        Some(output) => {
                            next_node.set_current_input(output, edge.input);
                        }
                        None => {
                            next_node.clear_current_input(edge.input);
                        }
                    }
                }
            }
        }
    }

    fn disconnect(&mut self, from: &OutPin, to: &InPin, snarl: &mut egui_snarl::Snarl<GraphIdx>) {
        // Update graph
        let node_from_idx = snarl[from.id.node];
        let node_to_idx = snarl[to.id.node];

        // disconnect graph
        if let Some(edge) = self.graph.find_edge(node_from_idx, node_to_idx) {
            self.graph.remove_edge(edge);
        }

        // disconnect snarl
        snarl.disconnect(from.id, to.id);

        // Update cached values
        let node_from = self.graph.node_weight_mut(node_from_idx).unwrap();
        node_from.set_current_output_disconnected(from.id.output);

        let node_to = self.graph.node_weight_mut(node_to_idx).unwrap();

        // - clear output on "to"
        node_to.clear_current_input(to.id.input);

        // TODO: walk affected nodes

    }

    fn title(&mut self, graph_idx: &GraphIdx) -> String {
        let node = self.graph.node_weight(*graph_idx).unwrap();
        node.name()
    }

    fn inputs(&mut self, graph_idx: &GraphIdx) -> usize {
        let node = self.graph.node_weight(*graph_idx).unwrap();
        node.inputs()
    }

    fn outputs(&mut self, graph_idx: &GraphIdx) -> usize {
        let node = self.graph.node_weight(*graph_idx).unwrap();
        node.outputs()
    }

    fn show_input(&mut self, pin: &InPin, ui: &mut Ui, scale: f32, snarl: &mut Snarl) -> PinInfo {
        let graph_idx = snarl[pin.id.node];
        let node = self.graph.node_weight(graph_idx).unwrap();
        match node {
            Node::Group { ref snarl, .. } => {
                let mut counter = 0;
                // let mut building = None;

                todo!()
                // for b in snarl.nodes() {
                //     counter += b.inputs();
                //     if b.inputs() > 0 && counter > pin.id.input {
                //         building = Some((b, counter - pin.id.input - 1));
                //         break;
                //     }
                // }
                // let (building, output_id) = building.unwrap();

                // let mut fake_pin = pin.clone();
                // fake_pin.id.input = output_id;

                // let building = match building {
                //     Node::Building(b) => b,
                //     Node::Group { .. } => todo!("nested groups are not supported yet"),
                // };
                // self.show_input_building(building, &fake_pin, ui, scale, snarl)
            }
            Node::Building(ref b) => self.show_input_building(graph_idx, b, pin, ui, scale, snarl),
        }
    }

    fn show_output(&mut self, pin: &OutPin, ui: &mut Ui, scale: f32, snarl: &mut Snarl) -> PinInfo {
        let graph_idx = snarl[pin.id.node];
        let node = self.graph.node_weight(graph_idx).unwrap();
        match node {
            Node::Group { .. } => {
                // TODO
                PinInfo::square().with_fill(BUILDING_COLOR)
            }
            Node::Building(ref b) => match b {
                Building::Miner(m) => {
                    assert_eq!(pin.id.output, 0, "Miner has only one output");

                    let speed = m.output_speed();
                    let material = m.output_material().map(Resource::Material);
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

                        fluid_output(fluid, max_speed, p.current_output_fluid(), ui, scale)
                    } else if pin.id.output == 1 {
                        // Material
                        let material = p.output_material();
                        let max_speed = p
                            .recipe
                            .as_ref()
                            .map(|r| r.max_output_speed_material())
                            .unwrap_or_default();

                        material_output(material, max_speed, p.current_output_material(), ui, scale)
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

                        fluid_output(fluid, max_speed, p.current_output_fluid(), ui, scale)
                    } else if pin.id.output == 1 {
                        // Material
                        let material = p.output_material();
                        let max_speed = p
                            .recipe
                            .as_ref()
                            .map(|r| r.max_output_speed_material())
                            .unwrap_or_default();

                        material_output(material, max_speed, p.current_output_material(), ui, scale)
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

                    material_output(material, max_speed, s.current_output(), ui, scale)
                }
                Building::Foundry(f) => {
                    assert_eq!(pin.id.output, 0, "Foundry node has only one output");

                    let material = f.output_material();
                    let max_speed = f
                        .recipe
                        .as_ref()
                        .map(|r| r.max_output_speed_material())
                        .unwrap_or_default();

                    material_output(material, max_speed, f.current_output(), ui, scale)
                }
                Building::Assembler(f) => {
                    assert_eq!(pin.id.output, 0, "Assembler node has only one output");

                    let material = f.output_material();
                    let max_speed = f
                        .recipe
                        .as_ref()
                        .map(|r| r.max_output_speed_material())
                        .unwrap_or_default();

                    material_output(material, max_speed, f.current_output(), ui, scale)
                }
                Building::Manufacturer(f) => {
                    assert_eq!(pin.id.output, 0, "Assembler node has only one output");

                    let material = f.output_material();
                    let max_speed = f
                        .recipe
                        .as_ref()
                        .map(|r| r.max_output_speed_material())
                        .unwrap_or_default();

                    material_output(material, max_speed, f.current_output(), ui, scale)
                }
                Building::PipelineJunction(_s) => {
                    let speed = 0.;
                    let material = None;

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/m^3", speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::circle().with_fill(color)
                }
                Building::Splitter(s) => {
                    let output = match pin.id.output {
                        0 => s.current_output_0(),
                        1 => s.current_output_1(),
                        2 => s.current_output_2(),
                        _ => unreachable!("3 outputs"),
                    };
                    let (speed, material) = match output {
                        Some(output) => (output.speed, Some(output.resource)),
                        None => (0., None)
                    };

                    ui.horizontal(|ui| {
                        add_resource_image(ui, scale, &material);
                        ui.label(format!("{}/min", speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::square().with_fill(color)
                }
                Building::Merger(m) => {
                    let color = match m.current_output() {
                        Some(output) => {
                            let color = output.resource.color();
                            ui.horizontal(|ui| {
                                add_resource_image(ui, scale, &Some(output.resource));
                                ui.label(format!("{}/min", output.speed));
                            });
                            color
                        }
                        None => BUILDING_COLOR,
                    };

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

                    material_output(material, max_speed, s.current_output(), ui, scale)
                }
                Building::AwesomeSink(_) => {
                    unreachable!("no outputs");
                }
            },
        }
    }

    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut Snarl) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: egui::Pos2, ui: &mut Ui, _scale: f32, snarl: &mut Snarl) {
        enum MenuItem {
            Building(Building),
            Group,
            Sep,
        }
        let items = vec![
            MenuItem::Building(Building::Miner(Default::default())),
            MenuItem::Building(Building::WaterExtractor(Default::default())),
            MenuItem::Building(Building::OilExtractor(Default::default())),
            MenuItem::Sep,
            MenuItem::Building(Building::Smelter(Default::default())),
            MenuItem::Building(Building::Foundry(Default::default())),
            MenuItem::Building(Building::Assembler(Default::default())),
            MenuItem::Building(Building::Constructor(Default::default())),
            MenuItem::Building(Building::Packager(Default::default())),
            MenuItem::Building(Building::Refinery(Default::default())),
            MenuItem::Building(Building::Manufacturer(Default::default())),
            MenuItem::Sep,
            MenuItem::Building(Building::Splitter(Default::default())),
            MenuItem::Building(Building::Merger(Default::default())),
            MenuItem::Sep,
            MenuItem::Building(Building::PipelineJunction(Default::default())),
            MenuItem::Sep,
            MenuItem::Building(Building::AwesomeSink(Default::default())),
            MenuItem::Sep,
            MenuItem::Building(Building::StorageContainer(Default::default())),
            MenuItem::Sep,
            MenuItem::Group,
        ];

        for item in items {
            match item {
                MenuItem::Building(b) => {
                    if ui.button(format!("Add {}", b.name())).clicked() {
                        let graph_idx = self.graph.add_node(Node::Building(b));
                        snarl.insert_node(pos, graph_idx);
                        ui.close_menu();
                    }
                }
                MenuItem::Group => {
                    if ui.button("Group").clicked() {
                        if let Some(snarl_ui_id) = self.snarl_ui_id {
                            let selected = Snarl::get_selected_nodes_at(
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
                            let mut sub_graph = NodeGraph::default();
                            let mut to_remove = Vec::new();
                            let mut num_inputs = 0;
                            let mut num_outputs = 0;
                            for (id, graph_idx) in selected {
                                let node = self.graph.node_weight(*graph_idx).unwrap();
                                let info = snarl.get_node_info(id).unwrap();
                                num_outputs += node.outputs();
                                num_inputs += node.inputs();
                                let new_graph_idx = sub_graph.add_node(node.clone());
                                buildings.insert_node(info.pos, new_graph_idx);
                                to_remove.push((id, *graph_idx));
                            }
                            // copy wires
                            // TODO
                            // let mut connections = Vec::new();
                            // for (output, input) in snarl.wires() {
                            //     if to_remove.contains(&output.node)
                            //         && to_remove.contains(&input.node)
                            //     {
                            //         num_inputs -= 1;
                            //         num_outputs -= 1;
                            //         connections.push((output, input));
                            //     }
                            // }

                            // for id in to_remove {
                            //     snarl.remove_node(id);
                            // }
                            // for (output, input) in connections {
                            //     buildings.connect(output, input);
                            // }

                            let node = Node::Group {
                                graph: sub_graph,
                                snarl: buildings,
                                num_inputs,
                                num_outputs,
                            };
                            let graph_idx = self.graph.add_node(node);
                            snarl.insert_node(pos, graph_idx);
                        }

                        ui.close_menu();
                    }
                }
                MenuItem::Sep => {
                    ui.separator();
                }
            }
        }

        ui.separator();
        if ui.button("Clear All").clicked() {
            // TODO: add warning
            *snarl = Snarl::default();
            ui.close_menu();
        }
    }

    fn has_dropped_wire_menu(&mut self, _src_pins: AnyPins, _snarl: &mut Snarl) -> bool {
        true
    }

    fn show_dropped_wire_menu(
        &mut self,
        _pos: egui::Pos2,
        ui: &mut Ui,
        _scale: f32,
        _src_pins: AnyPins,
        _snarl: &mut Snarl,
    ) {
        ui.label("Add node");
        // TODO:
    }

    fn has_node_menu(&mut self, _node: &GraphIdx) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node_id: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl,
    ) {
        let node_info = snarl.get_node_info(node_id).unwrap();
        let graph_idx = node_info.value;
        let node = self.graph.node_weight(graph_idx).unwrap();
        ui.label(node.name());

        match node {
            Node::Building(_) => {}
            Node::Group { snarl, graph, .. } => {
                if ui.button("Edit").clicked() {
                    self.group_edits.push((
                        self.index.0,
                        self.index.1,
                        self.index.2,
                        node_id,
                        graph.clone(),
                        snarl.clone(),
                    ));

                    ui.close_menu();
                }
            }
        }

        if ui.button("Duplicate").clicked() {
            let pos = node_info.pos + Vec2::new(5., 5.);
            let new_graph_idx = self.graph.add_node(node.clone());
            snarl.insert_node(pos, new_graph_idx);
            ui.close_menu();
        }

        if ui.button("Remove").clicked() {
            self.graph.remove_node(graph_idx);
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
    actual_input_speed: f32,
    actual_input_material: Option<Resource>,
    ui: &mut Ui,
    pin: &InPin,
    scale: f32,
    snarl: &Snarl,
    pin_info: PinInfo,
) -> PinInfo {
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
    current_output: Option<Output>,
    ui: &mut Ui,
    scale: f32,
) -> PinInfo {
    let color = fluid.as_ref().map(|m| m.color()).unwrap_or(BUILDING_COLOR);
    if let Some(fluid) = fluid {
        let fluid = Resource::Fluid(fluid);
        match current_output {
            Some(current_output) => {
                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &Some(current_output.resource));
                    ui.label(format!("{}/m^3 ({}/m^3)", current_output.speed, max_speed));
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
    current_output: Option<Output>,
    ui: &mut Ui,
    scale: f32,
) -> PinInfo {
    let color = material
        .as_ref()
        .map(|m| m.color())
        .unwrap_or(BUILDING_COLOR);
    if let Some(material) = material {
        let material = Resource::Material(material);
        match current_output {
            Some(current_output) => {
                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &Some(current_output.resource));
                    ui.label(format!("{}/min ({}/min)", current_output.speed, max_speed));
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
