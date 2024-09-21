use eframe::CreationContext;
use egui::{emath::Rot2, vec2, Color32, Id, Rect, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, SurfaceIndex};
use egui_modal::Modal;
use egui_snarl::{
    ui::{BackgroundPattern, SnarlStyle, Viewport},
    Snarl,
};
use serde::{Deserialize, Serialize};

use crate::node::Node;
use crate::viewer::Viewer;

pub struct App {
    tree: DockState<TabState>,
    show_about: bool,
    counter: usize,
}

struct TabViewer<'a> {
    added_nodes: &'a mut Vec<(SurfaceIndex, NodeIndex)>,
}

#[derive(Serialize, Deserialize)]
struct TabState {
    name: String,
    snarl_ui_id: Option<Id>,
    snarl: Snarl<Node>,
    style: SnarlStyle,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
            name: "First Factory".to_string(),
            snarl_ui_id: None,
            snarl: Snarl::new(),
            style: default_style(),
        }
    }
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = TabState;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&tab.name).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.snarl_ui_id = Some(ui.id());
        tab.snarl.show(
            &mut Viewer,
            &tab.style,
            format!("{}-{}", tab.name, ui.id().value()),
            ui,
        );
    }

    fn on_add(&mut self, surface: SurfaceIndex, node: NodeIndex) {
        self.added_nodes.push((surface, node));
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

        fn default_dock() -> DockState<TabState> {
            DockState::new(vec![TabState::default()])
        }

        let tree = match cx.storage {
            None => default_dock(),
            Some(storage) => {
                let mut tree = storage
                    .get_string(STORAGE_STRING)
                    .and_then(|dock| serde_json::from_str(&dock).ok())
                    .unwrap_or_else(default_dock);
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

                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut added_nodes = Vec::new();
            DockArea::new(&mut self.tree)
                .show_add_buttons(true)
                .style({
                    let mut style = egui_dock::Style::from_egui(ctx.style().as_ref());
                    style
                })
                .show(
                    ctx,
                    &mut TabViewer {
                        added_nodes: &mut added_nodes,
                    },
                );

            added_nodes.drain(..).for_each(|(surface, node)| {
                self.tree.set_focused_node_and_surface((surface, node));
                let mut style = SnarlStyle::new();
                style
                    .bg_pattern
                    .replace(BackgroundPattern::custom(dot_background));
                self.tree.push_to_focused_leaf(TabState {
                    name: format!("Factory {}", self.counter),
                    snarl_ui_id: None,
                    snarl: Snarl::new(),
                    style,
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
