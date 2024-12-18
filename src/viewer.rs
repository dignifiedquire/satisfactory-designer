use std::collections::HashMap;

use egui::{vec2, Color32, FontId, Id, Response, RichText, Ui, Vec2};
use egui_dock::SurfaceIndex;
use egui_snarl::{
    ui::{AnyPins, PinInfo, SnarlViewer},
    InPin, NodeId, OutPin,
};
use petgraph::visit::EdgeRef;
use strum::VariantArray;

use crate::{
    app::{EdgeDetails, GraphIdx, GroupEdit, NodeGraph, Snarl},
    buildings::{
        Building, Fluid, Material, MinerLevel, Pipe, ResourcePurity, Selectable, SomersloopSlot1,
        SomersloopSlot2, SomersloopSlot4,
    },
    node::{Node, Output, Resource},
};

const BUILDING_COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0xb0);
const INVALID_COLOR: Color32 = Color32::from_rgb(144, 20, 0);

pub struct Viewer<'a> {
    pub snarl_id_source: String,
    pub snarl_ui_id: Option<Id>,
    pub index: (SurfaceIndex, egui_dock::NodeIndex, usize),
    pub graph: &'a mut NodeGraph,
    pub group_edits: &'a mut Vec<(GroupEdit, NodeGraph, Snarl)>,
}

impl Viewer<'_> {
    fn show_input_building(
        &self,
        b: &Building,
        input: usize,
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
                if input == 0 {
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
                        scale,
                        snarl,
                        PinInfo::circle(),
                    )
                } else if input == 1 {
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
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else {
                    unreachable!("only two inputs");
                }
            }
            Building::Foundry(f) => {
                if input == 0 {
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
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else if input == 1 {
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
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else {
                    unreachable!("only two inputs");
                }
            }
            Building::Assembler(f) => {
                if input == 0 {
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
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else if input == 1 {
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
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else {
                    unreachable!("only two inputs");
                }
            }
            Building::Manufacturer(f) => match input {
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
                    let material = f.input_material().map(|(_, b, _, _)| Resource::Material(b));
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
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                _ => unreachable!("only four inputs"),
            },
            Building::Refinery(p) => {
                if input == 0 {
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
                        scale,
                        snarl,
                        PinInfo::circle(),
                    )
                } else if input == 1 {
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
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                } else {
                    unreachable!("only two inputs");
                }
            }
            Building::Smelter(ref s) => {
                assert_eq!(input, 0, "Smelter node has only one input");

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
                    scale,
                    snarl,
                    PinInfo::square(),
                )
            }
            Building::PipelineJunction(s) => {
                // 4 inputs

                let current_input = match input {
                    0 => &s.current_input_0,
                    1 => &s.current_input_1,
                    2 => &s.current_input_2,
                    3 => &s.current_input_3,
                    _ => unreachable!("4 inputs"),
                };

                let input_fluid = current_input.as_ref().map(|i| i.resource);
                let color = input_fluid.map(|r| r.color()).unwrap_or(BUILDING_COLOR);
                let actual_input_speed =
                    current_input.as_ref().map(|r| r.speed).unwrap_or_default();

                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &input_fluid);
                    ui.label(format!("{}/m^3", actual_input_speed,));
                });

                PinInfo::circle().with_fill(color)
            }
            Building::Splitter(s) => {
                assert_eq!(input, 0, "Splitter node has only one input");

                let (actual_input_speed, material) = match s.current_input {
                    Some(ref input) => (input.speed, Some(input.resource)),
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

                let current_input = match input {
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
                assert_eq!(input, 0, "Constructor node has only one input");

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
                    scale,
                    snarl,
                    PinInfo::square(),
                )
            }
            Building::AwesomeSink(ref s) => {
                assert_eq!(input, 0, "Awesome sink node has only one input");

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
            Building::Blender(b) => match input {
                0 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().0).unwrap_or_default();
                    let fluid = b.input_material().map(|b| Resource::Fluid(b.0));

                    let actual_input_speed = b
                        .current_input_fluid_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_fluid = b.current_input_fluid_0.as_ref().map(|i| i.resource);
                    single_input(
                        fluid,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_fluid,
                        ui,
                        scale,
                        snarl,
                        PinInfo::circle(),
                    )
                }
                1 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().1).unwrap_or_default();
                    let fluid = b.input_material().and_then(|b| b.1).map(Resource::Fluid);

                    let actual_input_speed = b
                        .current_input_fluid_1
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_fluid = b.current_input_fluid_1.as_ref().map(|i| i.resource);
                    single_input(
                        fluid,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_fluid,
                        ui,
                        scale,
                        snarl,
                        PinInfo::circle(),
                    )
                }
                2 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().2).unwrap_or_default();
                    let material = b.input_material().and_then(|b| b.2).map(Resource::Material);

                    let actual_input_speed = b
                        .current_input_material_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        b.current_input_material_0.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                3 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().3).unwrap_or_default();
                    let material = b.input_material().and_then(|b| b.3).map(Resource::Material);

                    let actual_input_speed = b
                        .current_input_material_1
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        b.current_input_material_1.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                _ => unreachable!("4 inputs"),
            },
            Building::ParticleAccelerator(b) => match input {
                0 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().0).unwrap_or_default();
                    let fluid = b.input_material().and_then(|b| b.0).map(Resource::Fluid);

                    let actual_input_speed = b
                        .current_input_fluid_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_fluid = b.current_input_fluid_0.as_ref().map(|i| i.resource);
                    single_input(
                        fluid,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_fluid,
                        ui,
                        scale,
                        snarl,
                        PinInfo::circle(),
                    )
                }
                1 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().1).unwrap_or_default();
                    let material = b.input_material().and_then(|b| b.1).map(Resource::Material);

                    let actual_input_speed = b
                        .current_input_material_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        b.current_input_material_0.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                2 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().2).unwrap_or_default();
                    let material = b.input_material().and_then(|b| b.2).map(Resource::Material);

                    let actual_input_speed = b
                        .current_input_material_1
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        b.current_input_material_1.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                _ => unreachable!("3 inputs"),
            },
            Building::QuantumEncoder(b) => match input {
                0 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().0).unwrap_or_default();
                    let fluid = b.input_material().map(|b| Resource::Fluid(b.0));

                    let actual_input_speed = b
                        .current_input_fluid_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_fluid = b.current_input_fluid_0.as_ref().map(|i| i.resource);
                    single_input(
                        fluid,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_fluid,
                        ui,
                        scale,
                        snarl,
                        PinInfo::circle(),
                    )
                }
                1 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().1).unwrap_or_default();
                    let material = b.input_material().map(|b| b.1).map(Resource::Material);

                    let actual_input_speed = b
                        .current_input_material_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        b.current_input_material_0.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                2 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().2).unwrap_or_default();
                    let material = b.input_material().map(|b| b.2).map(Resource::Material);

                    let actual_input_speed = b
                        .current_input_material_1
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        b.current_input_material_1.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                3 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().3).unwrap_or_default();
                    let material = b.input_material().map(|b| b.3).map(Resource::Material);

                    let actual_input_speed = b
                        .current_input_material_2
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        b.current_input_material_2.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                _ => unreachable!("4 inputs"),
            },
            Building::Converter(b) => match input {
                0 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().0).unwrap_or_default();
                    let material = b.input_material().and_then(|b| b.0).map(Resource::Material);

                    let actual_input_speed = b
                        .current_input_material_0
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        b.current_input_material_0.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                1 => {
                    let max_input_speed = b.recipe.map(|r| r.input_speed().1).unwrap_or_default();
                    let material = b.input_material().and_then(|b| b.1).map(Resource::Material);

                    let actual_input_speed = b
                        .current_input_material_1
                        .as_ref()
                        .map(|i| i.speed)
                        .unwrap_or_default();
                    let actual_input_material =
                        b.current_input_material_1.as_ref().map(|i| i.resource);
                    single_input(
                        material,
                        max_input_speed,
                        actual_input_speed,
                        actual_input_material,
                        ui,
                        scale,
                        snarl,
                        PinInfo::square(),
                    )
                }
                _ => unreachable!("3 inputs"),
            },
        }
    }

    fn show_output_building(
        &self,
        b: &Building,
        output: usize,
        ui: &mut Ui,
        scale: f32,
        _snarl: &Snarl,
    ) -> PinInfo {
        match b {
            Building::Miner(m) => {
                assert_eq!(output, 0, "Miner has only one output");

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
                if output == 0 {
                    // Fluid
                    let fluid = p.output_fluid();
                    let max_speed = p
                        .recipe
                        .as_ref()
                        .map(|r| r.max_output_speed_fluid())
                        .unwrap_or_default();

                    fluid_output(fluid, max_speed, p.current_output_fluid(), ui, scale)
                } else if output == 1 {
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
                if output == 0 {
                    // Fluid
                    let fluid = p.output_fluid();
                    let max_speed = p
                        .recipe
                        .as_ref()
                        .map(|r| r.max_output_speed_fluid())
                        .unwrap_or_default();

                    fluid_output(fluid, max_speed, p.current_output_fluid(), ui, scale)
                } else if output == 1 {
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
                assert_eq!(output, 0, "Water Extractor has only one output");
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
                assert_eq!(output, 0, "Oil Extractor has only one output");
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
                assert_eq!(output, 0, "Storage Container has only one output");
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
                assert_eq!(output, 0, "Smelter node has only one output");

                let material = s.output_material();
                let max_speed = s
                    .recipe
                    .as_ref()
                    .map(|r| r.max_output_speed())
                    .unwrap_or_default();

                material_output(material, max_speed, s.current_output(), ui, scale)
            }
            Building::Foundry(f) => {
                assert_eq!(output, 0, "Foundry node has only one output");

                let material = f.output_material();
                let max_speed = f
                    .recipe
                    .as_ref()
                    .map(|r| r.max_output_speed_material())
                    .unwrap_or_default();

                material_output(material, max_speed, f.current_output(), ui, scale)
            }
            Building::Assembler(f) => {
                assert_eq!(output, 0, "Assembler node has only one output");

                let material = f.output_material();
                let max_speed = f
                    .recipe
                    .as_ref()
                    .map(|r| r.max_output_speed_material())
                    .unwrap_or_default();

                material_output(material, max_speed, f.current_output(), ui, scale)
            }
            Building::Manufacturer(f) => {
                assert_eq!(output, 0, "Assembler node has only one output");

                let material = f.output_material();
                let max_speed = f
                    .recipe
                    .as_ref()
                    .map(|r| r.max_output_speed_material())
                    .unwrap_or_default();

                material_output(material, max_speed, f.current_output(), ui, scale)
            }
            Building::PipelineJunction(s) => {
                let output = match output {
                    0 => s.current_output_0(),
                    1 => s.current_output_1(),
                    2 => s.current_output_2(),
                    3 => s.current_output_3(),
                    _ => unreachable!("4 outputs"),
                };
                let (speed, fluid) = match output {
                    Some(output) => (output.speed, Some(output.resource)),
                    None => (0., None),
                };

                ui.horizontal(|ui| {
                    add_resource_image(ui, scale, &fluid);
                    ui.label(format!("{}/m^3", speed));
                });

                let color = fluid.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                PinInfo::circle().with_fill(color)
            }
            Building::Splitter(s) => {
                let output = match output {
                    0 => s.current_output_0(),
                    1 => s.current_output_1(),
                    2 => s.current_output_2(),
                    _ => unreachable!("3 outputs"),
                };
                let (speed, material) = match output {
                    Some(output) => (output.speed, Some(output.resource)),
                    None => (0., None),
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
                assert_eq!(output, 0, "Constructor node has only one output");
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
            Building::Blender(b) => {
                let (max_speed_fluid, max_speed_material) = b
                    .recipe
                    .as_ref()
                    .map(|r| r.max_output_speed())
                    .unwrap_or_default();

                match output {
                    0 => {
                        let fluid = b.output_fluid();
                        let max_speed = max_speed_fluid;

                        fluid_output(fluid, max_speed, b.current_output_fluid(), ui, scale)
                    }
                    1 => {
                        let material = b.output_material();
                        let max_speed = max_speed_material;

                        material_output(material, max_speed, b.current_output_material(), ui, scale)
                    }
                    _ => unreachable!("2 outputs"),
                }
            }
            Building::ParticleAccelerator(b) => {
                let max_speed_material = b
                    .recipe
                    .as_ref()
                    .map(|r| r.max_output_speed())
                    .unwrap_or_default();

                match output {
                    0 => {
                        let material = b.output_material();

                        material_output(
                            material,
                            max_speed_material,
                            b.current_output_material(),
                            ui,
                            scale,
                        )
                    }
                    _ => unreachable!("1 output"),
                }
            }
            Building::QuantumEncoder(b) => {
                let (max_speed_fluid, max_speed_material) = b
                    .recipe
                    .as_ref()
                    .map(|r| r.max_output_speed())
                    .unwrap_or_default();

                match output {
                    0 => {
                        let fluid = b.output_fluid();
                        let max_speed = max_speed_fluid;

                        fluid_output(fluid, max_speed, b.current_output_fluid(), ui, scale)
                    }
                    1 => {
                        let material = b.output_material();
                        let max_speed = max_speed_material;

                        material_output(material, max_speed, b.current_output_material(), ui, scale)
                    }
                    _ => unreachable!("2 outputs"),
                }
            }
            Building::Converter(b) => {
                let (max_speed_fluid, max_speed_material) = b
                    .recipe
                    .as_ref()
                    .map(|r| r.max_output_speed())
                    .unwrap_or_default();

                match output {
                    0 => fluid_output(
                        b.output_fluid(),
                        max_speed_fluid,
                        b.current_output_fluid(),
                        ui,
                        scale,
                    ),
                    1 => material_output(
                        b.output_material(),
                        max_speed_material,
                        b.current_output_material(),
                        ui,
                        scale,
                    ),
                    _ => unreachable!("2 outputs"),
                }
            }
        }
    }

    fn refresh_node(&mut self, node_idx: GraphIdx) {
        use petgraph::prelude::NodeIndex;

        // Find all paths
        fn all_paths_dfs(
            graph: &NodeGraph,
            start: NodeIndex,
            end: NodeIndex,
            visited: &mut HashMap<NodeIndex, Vec<NodeIndex>>,
            path: &mut Vec<NodeIndex>,
            all_paths: &mut Vec<Vec<NodeIndex>>,
        ) {
            path.push(start); // Add the current node to the path
            if start == end {
                // If the current node is the destination
                all_paths.push(path.clone()); // Add a copy of the path to the results
            } else {
                // Explore neighbors
                for neighbor in
                    graph.neighbors_directed(start, petgraph::prelude::Direction::Outgoing)
                {
                    let count = path.iter().filter(|n| *n == &neighbor).count();
                    // This number is..special
                    // A tradeoff between accuracy and calculation time.
                    const MAX_COUNT: usize = 5;
                    if count < MAX_COUNT {
                        all_paths_dfs(graph, neighbor, end, visited, path, all_paths);
                    }
                }
            }
            // Backtrack to explore other paths
            path.pop();
        }

        let externals: Vec<_> = self
            .graph
            .externals(petgraph::Direction::Outgoing)
            .collect();
        for target in externals {
            let mut path = Vec::new();
            let mut paths = Vec::new();
            let mut visited = HashMap::new();
            all_paths_dfs(
                &*self.graph,
                node_idx,
                target,
                &mut visited,
                &mut path,
                &mut paths,
            );
            for path in paths {
                let mut start_idx = path[0];
                for next_node_idx in path.into_iter().skip(1) {
                    let edges: Vec<_> = self
                        .graph
                        .edges_connecting(start_idx, next_node_idx)
                        .map(|e| e.weight().clone())
                        .collect();
                    for edge in edges {
                        let output = self
                            .graph
                            .node_weight(start_idx)
                            .unwrap()
                            .current_output(edge.output);
                        let node = self.graph.node_weight_mut(next_node_idx).unwrap();
                        if let Some(output) = output {
                            node.set_current_input(output, edge.input);
                        } else {
                            node.clear_current_input(edge.input);
                        }
                    }
                    // step forward
                    start_idx = next_node_idx;
                }
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
        let mut changed = false;
        let graph_idx = snarl[node];
        let node = self.graph.node_weight_mut(graph_idx).unwrap();

        ui.vertical(|ui| match node {
            Node::Group { snarl, graph, .. } => {
                for node_idx in snarl.nodes() {
                    let node = &graph[*node_idx];
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
                    changed |= general_selector(ui, scale, &mut m.resource).changed;
                    ui.add_space(10.0 * scale);

                    changed |= level_selector(ui, scale, &mut m.level).changed;
                    ui.add_space(10.0 * scale);

                    changed |= purity_selector(ui, scale, &mut m.resource_purity).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut m.speed).changed;
                }
                Building::OilExtractor(m) => {
                    changed |= pipe_selector(ui, scale, &mut m.output_pipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= purity_selector(ui, scale, &mut m.resource_purity).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut m.speed).changed;
                }
                Building::Packager(p) => {
                    changed |= general_selector(ui, scale, &mut p.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut p.speed).changed;
                }
                Building::Foundry(f) => {
                    changed |= general_selector(ui, scale, &mut f.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut f.speed).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_somersloop2_ui(ui, &mut f.amplified).changed;
                }
                Building::Assembler(f) => {
                    changed |= general_selector(ui, scale, &mut f.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut f.speed).changed;

                    ui.add_space(10.0 * scale);
                    changed |= add_somersloop2_ui(ui, &mut f.amplified).changed;
                }
                Building::Manufacturer(f) => {
                    changed |= general_selector(ui, scale, &mut f.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut f.speed).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_somersloop4_ui(ui, &mut f.amplified).changed;
                }
                Building::Refinery(p) => {
                    changed |= general_selector(ui, scale, &mut p.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut p.speed).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_somersloop2_ui(ui, &mut p.amplified).changed;
                }
                Building::WaterExtractor(m) => {
                    changed |= pipe_selector(ui, scale, &mut m.output_pipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut m.speed).changed;
                }
                Building::StorageContainer(s) => {
                    changed |= general_selector(ui, scale, &mut s.material).changed;
                    ui.add_space(10.0 * scale);

                    changed |= general_selector(ui, scale, &mut s.output_belt).changed;
                }
                Building::Smelter(s) => {
                    changed |= general_selector(ui, scale, &mut s.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut s.speed).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_somersloop1_ui(ui, &mut s.amplified).changed;
                }
                Building::PipelineJunction(_) => {}
                Building::Splitter(_) => {}
                Building::Merger(_) => {}
                Building::AwesomeSink(_) => {}
                Building::Constructor(s) => {
                    changed |= general_selector(ui, scale, &mut s.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut s.speed).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_somersloop1_ui(ui, &mut s.amplified).changed;
                }
                Building::Blender(b) => {
                    changed |= general_selector(ui, scale, &mut b.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut b.speed).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_somersloop4_ui(ui, &mut b.amplified).changed;
                }
                Building::ParticleAccelerator(b) => {
                    changed |= general_selector(ui, scale, &mut b.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut b.speed).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_somersloop4_ui(ui, &mut b.amplified).changed;
                }
                Building::QuantumEncoder(b) => {
                    changed |= general_selector(ui, scale, &mut b.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut b.speed).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_somersloop4_ui(ui, &mut b.amplified).changed;
                }
                Building::Converter(b) => {
                    changed |= general_selector(ui, scale, &mut b.recipe).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_speed_ui(ui, &mut b.speed).changed;
                    ui.add_space(10.0 * scale);

                    changed |= add_somersloop2_ui(ui, &mut b.amplified).changed;
                }
            },
        });

        if changed {
            self.refresh_node(graph_idx);
        }
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
                ui.label(text).on_hover_ui(|ui| {
                    let text =
                        RichText::new(node.description()).font(FontId::proportional(15.0 * scale));
                    ui.label(text);
                });
                ui.add_space(5. * scale);
            });
        });
    }

    #[inline]
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl) {
        let node_from_idx = snarl[from.id.node];
        let node_to_idx = snarl[to.id.node];

        // Validate input-output resource (pipe vs belt)
        let node_from = self.graph.node_weight(node_from_idx).unwrap();
        let node_to = self.graph.node_weight(node_to_idx).unwrap();
        if node_from.output_resource(from.id.output) != node_to.input_resource(to.id.input) {
            return;
        }

        // Update graph

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
        self.graph
            .node_weight_mut(node_from_idx)
            .unwrap()
            .set_current_output_connected(from.id.output);
        self.refresh_node(node_from_idx);
    }

    fn disconnect(&mut self, from: &OutPin, to: &InPin, snarl: &mut egui_snarl::Snarl<GraphIdx>) {
        // Update graph
        let node_from_idx = snarl[from.id.node];
        let node_to_idx = snarl[to.id.node];

        // disconnect graph
        let edge = self
            .graph
            .edges_connecting(node_from_idx, node_to_idx)
            .find(|e| {
                let weight = e.weight();
                weight.output == from.id.output && weight.input == to.id.input
            });

        if let Some(edge) = edge {
            self.graph.remove_edge(edge.id());
        }

        // disconnect snarl
        snarl.disconnect(from.id, to.id);

        // Update cached values

        // Clear input
        self.graph
            .node_weight_mut(node_from_idx)
            .unwrap()
            .set_current_output_disconnected(from.id.output);
        self.graph
            .node_weight_mut(node_to_idx)
            .unwrap()
            .clear_current_input(to.id.input);

        self.refresh_node(node_from_idx);
        self.refresh_node(node_to_idx);
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
            Node::Group {
                ref snarl,
                graph,
                inputs,
                ..
            } => {
                let (_node_id, node_idx, input_id, _current_input) = &inputs[pin.id.input];

                let building = graph.node_weight(*node_idx).unwrap();

                let building = match building {
                    Node::Building(b) => b,
                    Node::Group { .. } => todo!("nested groups are not supported yet"),
                };

                self.show_input_building(building, *input_id, ui, scale, snarl)
            }
            Node::Building(ref b) => self.show_input_building(b, pin.id.input, ui, scale, snarl),
        }
    }

    fn show_output(&mut self, pin: &OutPin, ui: &mut Ui, scale: f32, snarl: &mut Snarl) -> PinInfo {
        let graph_idx = snarl[pin.id.node];
        let node = self.graph.node_weight(graph_idx).unwrap();
        match node {
            Node::Group {
                snarl,
                graph,
                outputs,
                ..
            } => {
                let (_node_id, node_idx, output_id, _current_output) = &outputs[pin.id.output];

                let building = graph.node_weight(*node_idx).unwrap();

                let building = match building {
                    Node::Building(b) => b,
                    Node::Group { .. } => todo!("nested groups are not supported yet"),
                };

                self.show_output_building(building, *output_id, ui, scale, snarl)
            }
            Node::Building(ref b) => self.show_output_building(b, pin.id.output, ui, scale, snarl),
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
            MenuItem::Building(Building::Blender(Default::default())),
            MenuItem::Building(Building::ParticleAccelerator(Default::default())),
            MenuItem::Building(Building::QuantumEncoder(Default::default())),
            MenuItem::Building(Building::Converter(Default::default())),
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
                            let mut inputs = vec![];
                            let mut outputs = vec![];

                            let mut graph_idx_map = HashMap::new();

                            // TODO: preserve connections outside the group

                            for (id, graph_idx) in selected {
                                let node = self.graph.node_weight(*graph_idx).unwrap();
                                let info = snarl.get_node_info(id).unwrap();
                                let new_graph_idx = sub_graph.add_node(node.clone());
                                buildings.insert_node(info.pos, new_graph_idx);
                                graph_idx_map.insert(graph_idx, new_graph_idx);

                                for i in 0..node.outputs() {
                                    outputs.push((id, new_graph_idx, i, node.current_output(i)));
                                }
                                for i in 0..node.inputs() {
                                    inputs.push((id, new_graph_idx, i, node.current_input(i)));
                                }

                                to_remove.push((id, *graph_idx));
                            }

                            // copy wires
                            let mut connections = Vec::new();
                            for (node_id_a, idx_a) in &to_remove {
                                for (node_id_b, idx_b) in &to_remove {
                                    for edge in self.graph.edges_connecting(*idx_a, *idx_b) {
                                        connections.push((
                                            *node_id_a,
                                            *node_id_b,
                                            *idx_a,
                                            *idx_b,
                                            edge.id(),
                                        ));
                                    }
                                }
                            }

                            for (node_id_a, node_id_b, idx_a, idx_b, edge_id) in connections {
                                let new_idx_a = graph_idx_map.get(&idx_a).unwrap();
                                let new_idx_b = graph_idx_map.get(&idx_b).unwrap();
                                let edge_details = self.graph.edge_weight(edge_id).unwrap();
                                sub_graph.add_edge(*new_idx_a, *new_idx_b, edge_details.clone());

                                let wire = snarl
                                    .wires()
                                    .find(|(input, output)| {
                                        output.node == node_id_a && input.node == node_id_b
                                    })
                                    .unwrap();
                                buildings.connect(wire.0, wire.1);
                            }

                            // Remove nodes from snarl and the graph;
                            for (id, graph_idx) in &to_remove {
                                snarl.remove_node(*id);
                                self.graph.remove_node(*graph_idx);
                            }

                            let node = Node::Group {
                                graph: sub_graph,
                                snarl: buildings,
                                inputs,
                                outputs,
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
            self.graph.clear();
            ui.close_menu();
        }
    }

    fn has_dropped_wire_menu(&mut self, _src_pins: AnyPins, _snarl: &mut Snarl) -> bool {
        false
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
                        GroupEdit {
                            surface: self.index.0,
                            node_idx: self.index.1,
                            source_tab: self.index.2,
                            node_id,
                        },
                        graph.clone(),
                        snarl.clone(),
                    ));

                    ui.close_menu();
                }
            }
        }

        if ui.button("Duplicate").clicked() {
            let pos = node_info.pos + Vec2::new(5., 5.);
            let new_graph_idx = self.graph.add_node(node.clear_clone());
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

fn level_selector(ui: &mut Ui, _scale: f32, level: &mut MinerLevel) -> Response {
    let r = egui::ComboBox::from_label("Level")
        .selected_text(level.name())
        .show_ui(ui, |ui| {
            MinerLevel::VARIANTS
                .into_iter()
                .map(|l| {
                    let name = l.name();
                    ui.selectable_value(level, *l, name)
                })
                .reduce(|acc, r| acc | r)
                .unwrap()
        });
    r.inner.unwrap_or(r.response)
}

fn purity_selector(ui: &mut Ui, _scale: f32, purity: &mut ResourcePurity) -> Response {
    let r = egui::ComboBox::from_label("Purity")
        .selected_text(purity.name())
        .show_ui(ui, |ui| {
            ResourcePurity::VARIANTS
                .into_iter()
                .map(|p| {
                    let name = p.name();
                    ui.selectable_value(purity, *p, name)
                })
                .reduce(|acc, r| acc | r)
                .unwrap()
        });

    r.inner.unwrap_or(r.response)
}

fn pipe_selector(ui: &mut Ui, _scale: f32, pipe: &mut Option<Pipe>) -> Response {
    let text = match pipe {
        Some(p) => p.name(),
        None => "Select Pipe".to_string(),
    };

    let r = egui::ComboBox::from_label("Pipe")
        .selected_text(text)
        .show_ui(ui, |ui| {
            Pipe::VARIANTS
                .into_iter()
                .map(|p| {
                    let name = p.name();
                    ui.selectable_value(pipe, Some(*p), name)
                })
                .reduce(|acc, r| acc | r)
                .unwrap()
        });

    r.inner.unwrap_or(r.response)
}

fn general_selector<S: Selectable>(ui: &mut Ui, scale: f32, resource: &mut Option<S>) -> Response {
    ui.horizontal(|ui| {
        let x = 20. * scale;
        if let Some(ref resource) = resource {
            let image = egui::Image::new(resource.image())
                .fit_to_exact_size(vec2(x, x))
                .show_loading_spinner(true);
            ui.add(image);
        } else {
            ui.add_space(x);
        }

        let text = match resource {
            Some(r) => r.name(),
            None => format!("Select {}", S::NAME),
        };

        let r = egui::ComboBox::from_id_salt(egui::Id::new(format!("{}_resource", S::NAME)))
            .selected_text(text)
            .show_ui(ui, |ui| {
                S::VARIANTS
                    .into_iter()
                    .map(|r| {
                        let name = r.name();
                        ui.horizontal(|ui| {
                            let image = egui::Image::new(r.image())
                                .fit_to_exact_size(vec2(20., 20.))
                                .show_loading_spinner(true);
                            ui.add(image);
                            ui.selectable_value(resource, Some(r.clone()), name)
                        })
                        .inner
                    })
                    .reduce(|acc, r| acc | r)
                    .unwrap()
            });
        r.inner.unwrap_or(r.response)
    })
    .inner
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

fn add_speed_ui(ui: &mut Ui, value: &mut f32) -> Response {
    ui.horizontal(|ui| {
        let overclock = egui::DragValue::new(value).range(0.0..=250.0).suffix("%");
        let response = ui.add(overclock);
        ui.label("Speed");
        response
    })
    .inner
}

fn single_input(
    resource: Option<Resource>,
    max_input_speed: f32,
    actual_input_speed: f32,
    actual_input_material: Option<Resource>,
    ui: &mut Ui,
    scale: f32,
    _snarl: &Snarl,
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

    PinInfo::square().with_fill(color)
}

fn add_somersloop1_ui(ui: &mut Ui, amplified: &mut SomersloopSlot1) -> Response {
    let r = egui::ComboBox::from_id_salt(egui::Id::new("amplification1"))
        .selected_text(amplified.name())
        .show_ui(ui, |ui| {
            SomersloopSlot1::VARIANTS
                .into_iter()
                .map(|var| {
                    let name = var.name();
                    ui.selectable_value(amplified, *var, name)
                })
                .reduce(|acc, r| acc | r)
                .unwrap()
        });
    r.inner.unwrap_or(r.response)
}

fn add_somersloop2_ui(ui: &mut Ui, amplified: &mut SomersloopSlot2) -> Response {
    let r = egui::ComboBox::from_id_salt(egui::Id::new("amplification2"))
        .selected_text(amplified.name())
        .show_ui(ui, |ui| {
            SomersloopSlot2::VARIANTS
                .into_iter()
                .map(|var| {
                    let name = var.name();
                    ui.selectable_value(amplified, *var, name)
                })
                .reduce(|acc, r| acc | r)
                .unwrap()
        });
    r.inner.unwrap_or(r.response)
}

fn add_somersloop4_ui(ui: &mut Ui, amplified: &mut SomersloopSlot4) -> Response {
    let r = egui::ComboBox::from_id_salt(egui::Id::new("amplification4"))
        .selected_text(amplified.name())
        .show_ui(ui, |ui| {
            SomersloopSlot4::VARIANTS
                .into_iter()
                .map(|var| {
                    let name = var.name();
                    ui.selectable_value(amplified, *var, name)
                })
                .reduce(|acc, r| acc | r)
                .unwrap()
        });
    r.inner.unwrap_or(r.response)
}
