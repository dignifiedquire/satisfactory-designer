use eframe::CreationContext;
use egui::{emath::Rot2, vec2, Color32, Id, Rect, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, SurfaceIndex};
use egui_modal::Modal;
use egui_snarl::{
    ui::{BackgroundPattern, SnarlStyle, Viewport},
    NodeId,
};
use serde::{Deserialize, Serialize};

use crate::node::Node;
use crate::viewer::Viewer;

pub struct App {
    tree: DockState<TabState>,
    show_about: bool,
    counter: usize,
    tab_viewer: TabViewer,
}

#[derive(Default)]
struct TabViewer {
    added_nodes: Vec<(SurfaceIndex, NodeIndex)>,
    group_edits: Vec<(GroupEdit, NodeGraph, Snarl)>,
    group_saves: Vec<(GroupEdit, NodeGraph, Snarl)>,
    current_tab_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEdit {
    pub surface: SurfaceIndex,
    pub node_idx: NodeIndex,
    pub source_tab: usize,
    pub node_id: NodeId,
}

pub type NodeGraph = petgraph::stable_graph::StableDiGraph<Node, EdgeDetails>;
pub type GraphIdx = petgraph::graph::NodeIndex<u32>;

pub type Snarl = egui_snarl::Snarl<GraphIdx>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDetails {
    /// The input number to which the edge is going
    pub input: usize,
    /// The output number from which the edge is coming
    pub output: usize,
}

#[derive(Serialize, Deserialize)]
struct TabState {
    name: String,
    id: usize,
    snarl_ui_id: Option<Id>,
    snarl: Snarl,
    graph: NodeGraph,
    style: SnarlStyle,
    index: (SurfaceIndex, NodeIndex),
    group_edit: Option<GroupEdit>,
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = TabState;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&tab.name).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.snarl_ui_id = Some(ui.id());
        let snarl_id_source = format!("{}-{}", tab.name, ui.id().value());
        let mut viewer = Viewer {
            snarl_id_source: snarl_id_source.clone(),
            snarl_ui_id: Some(ui.id()),
            group_edits: &mut self.group_edits,
            graph: &mut tab.graph,
            index: (tab.index.0, tab.index.1, tab.id),
        };
        tab.snarl.show(&mut viewer, &tab.style, snarl_id_source, ui);
    }

    fn on_add(&mut self, surface: SurfaceIndex, node: NodeIndex) {
        self.added_nodes.push((surface, node));
    }

    fn on_close(&mut self, closing_tab: &mut Self::Tab) -> bool {
        if let Some(group_state) = closing_tab.group_edit.take() {
            let current_graph = closing_tab.graph.clone();
            let current_snarl = closing_tab.snarl.clone();
            self.group_saves
                .push((group_state, current_graph, current_snarl));
        }

        true
    }
}

const STORAGE_STRING: &'static str = "satisfactory-designer-tree";

impl App {
    pub fn new(cx: &CreationContext) -> Self {
        egui_extras::install_image_loaders(&cx.egui_ctx);

        cx.egui_ctx.style_mut(|style| {
            style.visuals.extreme_bg_color = Color32::from_hex("#1E1E1E").unwrap();
            style.animation_time *= 10.0;
        });

        fn default_dock(
            tab_viewer: &mut TabViewer,
            index: (SurfaceIndex, NodeIndex),
        ) -> DockState<TabState> {
            let tab_id = tab_viewer.current_tab_index;
            tab_viewer.current_tab_index += 1;
            DockState::new(vec![TabState {
                name: "First factory".to_string(),
                id: tab_id,
                snarl_ui_id: None,
                graph: Default::default(),
                snarl: Snarl::new(),
                style: default_style(),
                group_edit: None,
                index: (index.0, index.1),
            }])
        }
        let mut tab_viewer = TabViewer::default();
        let index = (SurfaceIndex::main(), NodeIndex::root());

        let tree = match cx.storage {
            None => default_dock(&mut tab_viewer, index),
            Some(storage) => {
                let mut tree = storage
                    .get_string(STORAGE_STRING)
                    .and_then(|dock| serde_json::from_str(&dock).ok())
                    .unwrap_or_else(|| default_dock(&mut tab_viewer, index));
                for (_, tab) in tree.iter_all_tabs_mut() {
                    // style is not persisted atm
                    // TOOD: store zoom details
                    tab.style = default_style();
                }
                tree
            }
        };

        App {
            tree,
            show_about: false,
            counter: 1,
            tab_viewer,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

                egui::widgets::global_theme_preference_switch(ui);
            });
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| {
                DockArea::new(&mut self.tree)
                    .show_add_buttons(true)
                    .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
                    .show_inside(ui, &mut self.tab_viewer);

                self.tab_viewer
                    .group_edits
                    .drain(..)
                    .for_each(|(group_state, graph, snarl)| {
                        self.tree.set_focused_node_and_surface((
                            group_state.surface,
                            group_state.node_idx,
                        ));
                        let mut style = SnarlStyle::new();
                        style
                            .bg_pattern
                            .replace(BackgroundPattern::custom(dot_background));

                        let tab_id = self.tab_viewer.current_tab_index;
                        self.tab_viewer.current_tab_index += 1;
                        self.tree.push_to_focused_leaf(TabState {
                            name: format!("Editing Group {}", group_state.node_id.0),
                            id: tab_id,
                            snarl_ui_id: None,
                            graph,
                            snarl,
                            style,
                            group_edit: Some(group_state.clone()),
                            index: (group_state.surface, group_state.node_idx),
                        });
                    });

                self.tab_viewer.group_saves.drain(..).for_each(
                    |(group_state, group_graph, group_snarl)| {
                        // Grap the current tab
                        let tab = &mut self.tree[group_state.surface][group_state.node_idx];
                        match tab {
                            egui_dock::Node::Leaf { tabs, .. } => {
                                let source_tab = &mut tabs[group_state.source_tab];

                                let node_info =
                                    source_tab.snarl.get_node_info(group_state.node_id).unwrap();
                                let graph_idx = node_info.value;
                                let node = source_tab.graph.node_weight_mut(graph_idx).unwrap();

                                match node {
                                    Node::Building(_) => unreachable!("invalid group building"),
                                    Node::Group {
                                        snarl,
                                        graph,
                                        inputs,
                                        outputs,
                                    } => {
                                        *graph = group_graph;
                                        *snarl = group_snarl;

                                        // update inputs and outputs
                                        inputs.clear();
                                        outputs.clear();
                                        for (id, node) in snarl.nodes_ids_data() {
                                            let graph_idx = node.value;
                                            let node = graph.node_weight(graph_idx).unwrap();

                                            for i in 0..node.outputs() {
                                                outputs.push((
                                                    id,
                                                    graph_idx,
                                                    i,
                                                    node.current_output(i),
                                                ));
                                            }
                                            for i in 0..node.inputs() {
                                                inputs.push((
                                                    id,
                                                    graph_idx,
                                                    i,
                                                    node.current_input(i),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                            _ => unreachable!("invalid node"),
                        }
                    },
                );

                self.tab_viewer
                    .added_nodes
                    .drain(..)
                    .for_each(|(surface, node)| {
                        self.tree.set_focused_node_and_surface((surface, node));
                        let mut style = SnarlStyle::new();
                        style
                            .bg_pattern
                            .replace(BackgroundPattern::custom(dot_background));

                        let tab_id = self.tab_viewer.current_tab_index;
                        self.tab_viewer.current_tab_index += 1;
                        self.tree.push_to_focused_leaf(TabState {
                            name: format!("Factory {}", self.counter),
                            id: tab_id,
                            snarl_ui_id: None,
                            graph: Default::default(),
                            snarl: Snarl::new(),
                            style,
                            group_edit: None,
                            index: (surface, node),
                        });
                        self.counter += 1;
                    });

                if self.show_about {
                    let modal = Modal::new(ctx, "About");
                    modal.show(|ui| {
                        modal.title(ui, "About");
                        modal.frame(ui, |ui| {
                            ui.add_space(20.);
                            ui.hyperlink("https://github.com/dignifiedquire/satisfactory-designer");
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
        let tree = serde_json::to_string(&self.tree).unwrap();
        storage.set_string(STORAGE_STRING, tree);
    }
}

fn default_style() -> SnarlStyle {
    let mut style = SnarlStyle::new();
    style
        .bg_pattern
        .replace(BackgroundPattern::custom(dot_background));
    style
}

fn dot_background(_style: &SnarlStyle, viewport: &Viewport, ui: &mut Ui) {
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
            ui.painter()
                .circle_filled(pos, radius, Color32::from_hex("#7E7E7E").unwrap());
        }
    }
}
