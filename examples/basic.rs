use egui::{DragValue, Separator};
use egui_drag_value_expr::DragValueExpr;

/// Create basic eframe app
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

/// Simple data structure to hold mutable f64 value
#[derive(Default)]
struct App {
    value: f64,
}

/// Three separate DragValueExpr widgets are created to demonstrate a few different ways to use the widget.
/// - The first DragValueExpr is created from a mutable reference to a f64 and an optional Scope.
/// - The second DragValueExpr is created from a DragValue with a custom setting and an optional Scope.
/// - The third DragValueExpr is created from a mutable reference to a f64 and an optional Scope.
///     Try entering an expression like "ten * 2" to see the value update.
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
            ui.add(DragValueExpr::new(&mut self.value, Some(&scope)));
        });
    }
}
