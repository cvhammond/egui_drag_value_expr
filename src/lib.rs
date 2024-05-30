use egui::*;
use rhai::{Dynamic, Engine, Scope};

pub struct DragValueExpr<'a> {
    scope: Option<&'a Scope<'a>>,
    drag_value: DragValue<'a>,
}

impl<'a> DragValueExpr<'a> {
    pub fn new(value: &'a mut f64, scope: Option<&'a Scope>) -> Self {
        Self {
            scope,
            drag_value: DragValue::new(value),
        }
    }

    pub fn from_drag_value(drag_value: DragValue<'a>, scope: Option<&'a Scope>) -> Self {
        Self { scope, drag_value }
    }
}

impl<'a> Widget for DragValueExpr<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(self.drag_value.custom_parser(|s| {
            let engine = Engine::new();
            let mut scope = match self.scope {
                Some(scope) => scope.clone(),
                None => Scope::new(),
            };
            let result = engine.eval_expression_with_scope::<Dynamic>(&mut scope, s);
            match result {
                Ok(result) => {
                    if result.is::<f64>() {
                        Some(result.cast::<f64>())
                    } else if result.is::<i64>() {
                        Some(result.cast::<i64>() as f64)
                    } else {
                        None
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    None
                }
            }
        }))
    }
}
