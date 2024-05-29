use egui::*;
use egui_drag_value_expr::DragValueExpr;

fn main() {
    let _ = eframe::run_native(
        "Basic DragValueExpr Example",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1024.0, 768.0)),
            ..Default::default()
        },
        Box::new(|_app| Box::<App>::default()),
    );
}

#[derive(Default)]
struct App {
    value: f64,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(&mut DragValueExpr::new(&mut self.value, None));
            });
        });
    }
}
