use egui::{DragValue, Separator};
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
            ui.add(DragValueExpr::new(&mut self.value, None));
            ui.add(Separator::default());
            ui.add(DragValueExpr::from_drag_value(
                    DragValue::new(&mut self.value)
                    .fixed_decimals(2),
                    None,
            ));
            ui.add(Separator::default());
            let mut scope = rhai::Scope::new();
            scope.push("ten", 10 as i64);
            scope.push("self", self.value);
            ui.add(DragValueExpr::new(&mut self.value, Some(&scope)));
        });
    }
}
