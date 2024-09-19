use buildings::{Building, Constructor, Material, Merger, Miner, Smelter, Splitter};
use eframe::{App, CreationContext};
use egui::{emath::Rot2, style::Spacing, vec2, Color32, FontId, Id, Rect, RichText, Ui, Vec2};
use egui_snarl::{
    ui::{AnyPins, BackgroundPattern, PinInfo, SnarlStyle, SnarlViewer},
    InPin, InPinId, NodeId, OutPin, OutPinId, Snarl,
};

const BUILDING_COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0xb0);

mod buildings;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
enum Node {
    Building(Building),
}

impl Node {
    /// The speed for this output
    fn output_speed(&self, snarl: &Snarl<Node>, remote_node: NodeId) -> f32 {
        match self {
            Node::Building(b) => match b {
                Building::Miner(remote_m) => remote_m.output_speed(),
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

                            let base_speed = snarl[output.node].output_speed(snarl, output.node);

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
                        .map(|(output, _input)| snarl[output.node].output_speed(snarl, output.node))
                        .unwrap_or_default();
                    remote_s.output_speed(input_speed)
                }
                Building::Constructor(remote_s) => {
                    let input_wire = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node);

                    let input_speed = input_wire
                        .map(|(output, _input)| snarl[output.node].output_speed(snarl, output.node))
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

                        let base_speed = snarl[output.node].output_speed(snarl, output.node);

                        speed += base_speed / num_connections;
                    }
                    speed
                }
            },
        }
    }

    /// The output material
    fn output_material(&self, snarl: &Snarl<Node>, remote_node: NodeId) -> Option<Material> {
        match self {
            Node::Building(b) => match b {
                Building::Miner(remote_m) => {
                    remote_m.resource.as_ref().map(|r| r.output_material())
                }
                Building::Splitter(_remote_s) => {
                    let input_wire = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node);

                    match input_wire {
                        Some((output, _input)) => {
                            snarl[output.node].output_material(snarl, output.node)
                        }
                        None => None,
                    }
                }
                Building::Smelter(remote_s) => {
                    remote_s.recipie.as_ref().map(|r| r.output_material())
                }
                Building::Constructor(remote_s) => {
                    remote_s.recipie.as_ref().map(|r| r.output_material())
                }
                Building::Merger(_remote_m) => {
                    // For now we just grab the first one, as we don't support sushi belts (yet)
                    let input_wire = snarl
                        .wires()
                        .find(|(_output, input)| input.node == remote_node);

                    match input_wire {
                        Some((output, _input)) => {
                            snarl[output.node].output_material(snarl, output.node)
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
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        ui.vertical(|ui| {
            ui.add_space(10.0 * scale);
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
                                    ui.selectable_value(&mut m.resource_purity, purity, name);
                                }
                            });
                        ui.add_space(10.0 * scale);
                        add_speed_ui(ui, &mut m.speed);
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
                                                Some(recipie),
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
            _ => false,
        }
    }

    fn show_header(
        &mut self,
        node: NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        match &snarl[node] {
            node @ Node::Building(b) => {
                ui.vertical(|ui| {
                    ui.add_space(5.);
                    ui.horizontal(|ui| {
                        let x = 30. * scale;
                        let image = egui::Image::new(b.header_image())
                            .fit_to_exact_size(vec2(x, x))
                            .show_loading_spinner(true);
                        ui.add(image);
                        ui.add_space(5.);

                        let title = self.title(node);
                        let text = RichText::new(title).font(FontId::proportional(15.0 * scale));
                        ui.label(text);
                        ui.add_space(5.);
                    });
                    ui.add_space(5.);
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
                Building::Smelter(ref s) => {
                    assert_eq!(pin.id.input, 0, "Smelter node has only one input");

                    let (actual_input_speed, material) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed = snarl[remote.node].output_speed(snarl, remote.node);
                            let material = snarl[remote.node].output_material(snarl, remote.node);
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
                        add_material_image(ui, scale, &material);
                        ui.label(format!(
                            "{}/min ({}/min)",
                            actual_input_speed, max_input_speed
                        ));
                    });
                    ui.add_space(15.0 * scale);

                    PinInfo::circle().with_fill(color)
                }
                Building::Splitter(_) => {
                    assert_eq!(pin.id.input, 0, "Splitter node has only one input");

                    let (actual_input_speed, material) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed = snarl[remote.node].output_speed(snarl, remote.node);
                            let material = snarl[remote.node].output_material(snarl, remote.node);
                            (speed, material)
                        }
                        _ => unreachable!("only one output"),
                    };

                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    ui.horizontal(|ui| {
                        add_material_image(ui, scale, &material);
                        ui.label(format!("{}/min", actual_input_speed,));
                    });

                    PinInfo::circle().with_fill(color)
                }
                Building::Merger(_) => {
                    // 3 inputs
                    let (actual_input_speed, material) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed = snarl[remote.node].output_speed(snarl, remote.node);
                            let material = snarl[remote.node].output_material(snarl, remote.node);
                            (speed, material)
                        }
                        _ => unreachable!("only one output"),
                    };

                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    ui.horizontal(|ui| {
                        add_material_image(ui, scale, &material);
                        ui.label(format!("{}/min", actual_input_speed,));
                    });

                    PinInfo::circle().with_fill(color)
                }
                Building::Constructor(ref s) => {
                    assert_eq!(pin.id.input, 0, "Constructor node has only one input");

                    let (actual_input_speed, material) = match &*pin.remotes {
                        [] => (0., None),
                        [remote] => {
                            let speed = snarl[remote.node].output_speed(snarl, remote.node);
                            let material = snarl[remote.node].output_material(snarl, remote.node);
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
                        add_material_image(ui, scale, &material);
                        ui.label(format!(
                            "{}/min ({}/min)",
                            actual_input_speed, max_input_speed
                        ));
                    });

                    PinInfo::circle().with_fill(color)
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
                    let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node);
                    let material = snarl[pin.id.node].output_material(snarl, pin.id.node);
                    let color = material
                        .as_ref()
                        .map(|m| m.color())
                        .unwrap_or(BUILDING_COLOR);

                    ui.horizontal(|ui| {
                        add_material_image(ui, scale, &material);
                        ui.label(format!("{}/min", speed));
                    });

                    PinInfo::circle().with_fill(color)
                }
                Building::Smelter(s) => {
                    assert_eq!(pin.id.output, 0, "Smelter node has only one output");
                    let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node);
                    let material = snarl[pin.id.node].output_material(snarl, pin.id.node);

                    let max_speed = s
                        .recipie
                        .as_ref()
                        .map(|r| r.max_output_speed())
                        .unwrap_or_default();

                    ui.horizontal(|ui| {
                        add_material_image(ui, scale, &material);
                        ui.label(format!("{}/min ({}/min)", speed, max_speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::circle().with_fill(color)
                }
                Building::Splitter(_s) => {
                    let (speed, material) = if !pin.remotes.is_empty() {
                        let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node);
                        let material = snarl[pin.id.node].output_material(snarl, pin.id.node);
                        (speed, material)
                    } else {
                        (0., None)
                    };

                    ui.horizontal(|ui| {
                        add_material_image(ui, scale, &material);
                        ui.label(format!("{}/min", speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::circle().with_fill(color)
                }
                Building::Merger(_m) => {
                    let (speed, material) = if !pin.remotes.is_empty() {
                        let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node);
                        let material = snarl[pin.id.node].output_material(snarl, pin.id.node);
                        (speed, material)
                    } else {
                        (0., None)
                    };

                    ui.horizontal(|ui| {
                        add_material_image(ui, scale, &material);
                        ui.label(format!("{}/min", speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::circle().with_fill(color)
                }
                Building::Constructor(s) => {
                    assert_eq!(pin.id.output, 0, "Constructor node has only one output");
                    let speed = snarl[pin.id.node].output_speed(snarl, pin.id.node);
                    let material = snarl[pin.id.node].output_material(snarl, pin.id.node);
                    let max_speed = s
                        .recipie
                        .as_ref()
                        .map(|r| r.max_output_speed())
                        .unwrap_or_default();

                    ui.horizontal(|ui| {
                        add_material_image(ui, scale, &material);
                        ui.label(format!("{}/min ({}/min)", speed, max_speed));
                    });

                    let color = material.map(|m| m.color()).unwrap_or(BUILDING_COLOR);
                    PinInfo::circle().with_fill(color)
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
        if ui.button("Smelter").clicked() {
            snarl.insert_node(pos, Node::Building(Building::Smelter(Smelter::default())));
            ui.close_menu();
        }
        if ui.button("Constructor").clicked() {
            snarl.insert_node(
                pos,
                Node::Building(Building::Constructor(Constructor::default())),
            );
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
    }

    fn has_dropped_wire_menu(&mut self, _src_pins: AnyPins, _snarl: &mut Snarl<Node>) -> bool {
        true
    }

    fn show_dropped_wire_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut Ui,
        _scale: f32,
        src_pins: AnyPins,
        snarl: &mut Snarl<Node>,
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct ExprNode {
    text: String,
    bindings: Vec<String>,
    values: Vec<f64>,
    expr: Expr,
}

impl ExprNode {
    fn new() -> Self {
        ExprNode {
            text: "0".to_string(),
            bindings: Vec::new(),
            values: Vec::new(),
            expr: Expr::Val(0.0),
        }
    }

    fn eval(&self) -> f64 {
        self.expr.eval(&self.bindings, &self.values)
    }
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
enum UnOp {
    Pos,
    Neg,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
enum Expr {
    Var(String),
    Val(f64),
    UnOp {
        op: UnOp,
        expr: Box<Expr>,
    },
    BinOp {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
}

impl Expr {
    fn eval(&self, bindings: &[String], args: &[f64]) -> f64 {
        let binding_index =
            |name: &str| bindings.iter().position(|binding| binding == name).unwrap();

        match self {
            Expr::Var(ref name) => args[binding_index(name)],
            Expr::Val(value) => *value,
            Expr::UnOp { op, ref expr } => match op {
                UnOp::Pos => expr.eval(bindings, args),
                UnOp::Neg => -expr.eval(bindings, args),
            },
            Expr::BinOp {
                ref lhs,
                op,
                ref rhs,
            } => match op {
                BinOp::Add => lhs.eval(bindings, args) + rhs.eval(bindings, args),
                BinOp::Sub => lhs.eval(bindings, args) - rhs.eval(bindings, args),
                BinOp::Mul => lhs.eval(bindings, args) * rhs.eval(bindings, args),
                BinOp::Div => lhs.eval(bindings, args) / rhs.eval(bindings, args),
            },
        }
    }

    fn extend_bindings(&self, bindings: &mut Vec<String>) {
        match self {
            Expr::Var(name) => {
                if !bindings.contains(name) {
                    bindings.push(name.clone());
                }
            }
            Expr::Val(_) => {}
            Expr::UnOp { expr, .. } => {
                expr.extend_bindings(bindings);
            }
            Expr::BinOp { lhs, rhs, .. } => {
                lhs.extend_bindings(bindings);
                rhs.extend_bindings(bindings);
            }
        }
    }
}

impl syn::parse::Parse for UnOp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![+]) {
            input.parse::<syn::Token![+]>()?;
            Ok(UnOp::Pos)
        } else if lookahead.peek(syn::Token![-]) {
            input.parse::<syn::Token![-]>()?;
            Ok(UnOp::Neg)
        } else {
            Err(lookahead.error())
        }
    }
}

impl syn::parse::Parse for BinOp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![+]) {
            input.parse::<syn::Token![+]>()?;
            Ok(BinOp::Add)
        } else if lookahead.peek(syn::Token![-]) {
            input.parse::<syn::Token![-]>()?;
            Ok(BinOp::Sub)
        } else if lookahead.peek(syn::Token![*]) {
            input.parse::<syn::Token![*]>()?;
            Ok(BinOp::Mul)
        } else if lookahead.peek(syn::Token![/]) {
            input.parse::<syn::Token![/]>()?;
            Ok(BinOp::Div)
        } else {
            Err(lookahead.error())
        }
    }
}

impl syn::parse::Parse for Expr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let lhs;
        if lookahead.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let expr = content.parse::<Expr>()?;
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        // } else if lookahead.peek(syn::LitFloat) {
        //     let lit = input.parse::<syn::LitFloat>()?;
        //     let value = lit.base10_parse::<f64>()?;
        //     let expr = Expr::Val(value);
        //     if input.is_empty() {
        //         return Ok(expr);
        //     }
        //     lhs = expr;
        } else if lookahead.peek(syn::LitInt) {
            let lit = input.parse::<syn::LitInt>()?;
            let value = lit.base10_parse::<f64>()?;
            let expr = Expr::Val(value);
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else if lookahead.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            let expr = Expr::Var(ident.to_string());
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else {
            let unop = input.parse::<UnOp>()?;

            return Self::parse_with_unop(unop, input);
        }

        let binop = input.parse::<BinOp>()?;

        Self::parse_binop(Box::new(lhs), binop, input)
    }
}

impl Expr {
    fn parse_with_unop(op: UnOp, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let lhs;
        if lookahead.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let expr = Expr::UnOp {
                op,
                expr: Box::new(content.parse::<Expr>()?),
            };
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else if lookahead.peek(syn::LitFloat) {
            let lit = input.parse::<syn::LitFloat>()?;
            let value = lit.base10_parse::<f64>()?;
            let expr = Expr::UnOp {
                op,
                expr: Box::new(Expr::Val(value)),
            };
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else if lookahead.peek(syn::LitInt) {
            let lit = input.parse::<syn::LitInt>()?;
            let value = lit.base10_parse::<f64>()?;
            let expr = Expr::UnOp {
                op,
                expr: Box::new(Expr::Val(value)),
            };
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else if lookahead.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            let expr = Expr::UnOp {
                op,
                expr: Box::new(Expr::Var(ident.to_string())),
            };
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else {
            return Err(lookahead.error());
        }

        let op = input.parse::<BinOp>()?;

        Self::parse_binop(Box::new(lhs), op, input)
    }

    fn parse_binop(lhs: Box<Expr>, op: BinOp, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let rhs;
        if lookahead.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            rhs = Box::new(content.parse::<Expr>()?);
            if input.is_empty() {
                return Ok(Expr::BinOp { lhs, op, rhs });
            }
        } else if lookahead.peek(syn::LitFloat) {
            let lit = input.parse::<syn::LitFloat>()?;
            let value = lit.base10_parse::<f64>()?;
            rhs = Box::new(Expr::Val(value));
            if input.is_empty() {
                return Ok(Expr::BinOp { lhs, op, rhs });
            }
        } else if lookahead.peek(syn::LitInt) {
            let lit = input.parse::<syn::LitInt>()?;
            let value = lit.base10_parse::<f64>()?;
            rhs = Box::new(Expr::Val(value));
            if input.is_empty() {
                return Ok(Expr::BinOp { lhs, op, rhs });
            }
        } else if lookahead.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            rhs = Box::new(Expr::Var(ident.to_string()));
            if input.is_empty() {
                return Ok(Expr::BinOp { lhs, op, rhs });
            }
        } else {
            return Err(lookahead.error());
        }

        let next_op = input.parse::<BinOp>()?;

        match (op, next_op) {
            (BinOp::Add | BinOp::Sub, BinOp::Mul | BinOp::Div) => {
                let rhs = Self::parse_binop(rhs, next_op, input)?;
                Ok(Expr::BinOp {
                    lhs,
                    op,
                    rhs: Box::new(rhs),
                })
            }
            _ => {
                let lhs = Expr::BinOp { lhs, op, rhs };
                Self::parse_binop(Box::new(lhs), next_op, input)
            }
        }
    }
}

pub struct DemoApp {
    snarl: Snarl<Node>,
    style: SnarlStyle,
    snarl_ui_id: Option<Id>,
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
            .replace(BackgroundPattern::custom(|style, viewport, ui| {
                let spacing = vec2(50.0, 50.0);
                let angle = 0.0;

                let bg_stroke = style
                    .bg_pattern_stroke
                    .unwrap_or(ui.visuals().widgets.noninteractive.bg_stroke);

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

                        let top = (rot * vec2(x, pattern_bounds.min.y)).to_pos2();
                        let bottom = (rot * vec2(x, pattern_bounds.max.y)).to_pos2();

                        let top = viewport.graph_pos_to_screen(top);
                        let bottom = viewport.graph_pos_to_screen(bottom);

                        // ui.painter().line_segment([top, bottom], bg_stroke);
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
        }
    }
}

impl App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_switch(ui);

                if ui.button("Clear All").clicked() {
                    self.snarl = Default::default();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.snarl_ui_id = Some(ui.id());

            self.snarl.show(&mut Viewer, &self.style, "snarl", ui);
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

fn add_material_image(ui: &mut Ui, scale: f32, material: &Option<Material>) {
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
