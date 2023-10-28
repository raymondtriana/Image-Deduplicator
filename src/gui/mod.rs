use std::result;

use eframe::egui::{self, Context, ScrollArea, Ui};

pub fn launch() {
    let native_options = eframe::NativeOptions::default();
    let result = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
    match result {
        Ok(t) => t,
        Err(error) => {
            panic!("COULDNT CREATE THE GUI!. {:?}", error)
        }
    }
}

#[derive(Default)]

struct MyApp {}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        render_header(ctx);
        render_side_panel(ctx);
        render_cental_panel(ctx);
    }
}

fn render_cental_panel(ctx: &Context){
    egui::CentralPanel::default().show(ctx, |ui|{
        render_scroll_area(ui, ctx)
    });
}

fn render_side_panel(ctx: &Context){
    egui::SidePanel::left("side_panel_left").show(ctx, |ui|{
        ui.label("text");
    });
}

fn render_scroll_area(ui:&mut Ui, ctx: &Context){
    egui::ScrollArea::new([true, true]).show(ui, |ui| {
        for a in (0..200) {
            ui.label("Hello World!");
        }
    });
}

fn render_header(ctx: &Context) {
    let header_widget = egui::TopBottomPanel::top("main_header").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
        });
        ui.label("text");
    });
}
