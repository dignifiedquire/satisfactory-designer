use eframe::CreationContext;
use egui::{emath::Rot2, vec2, Color32, Id, Rect};
use egui_modal::Modal;
use egui_snarl::{
    ui::{BackgroundPattern, SnarlStyle},
    Snarl,
};

use crate::node::Node;
use crate::viewer::Viewer;

pub struct App {
    snarl: Snarl<Node>,
    style: SnarlStyle,
    snarl_ui_id: Option<Id>,
    show_about: bool,
}

impl App {
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

        App {
            snarl,
            style,
            snarl_ui_id: None,
            show_about: false,
        }
    }
}

impl eframe::App for App {
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
        let snarl = serde_json::to_string(&self.snarl).unwrap();
        storage.set_string("snarl", snarl);

        let style = serde_json::to_string(&self.style).unwrap();
        storage.set_string("style", style);
    }
}
