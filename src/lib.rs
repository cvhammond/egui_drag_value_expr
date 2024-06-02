//! # egui_drag_value_expr
//!
//! An egui DragValue widget that accepts expression-based values.
//!
//! ## Usage
//!
//! A rhai::Scope can be passed to the DragValueExpr widget to allow the user to enter expressions that reference variables in the scope.
//!
//! With a scope:
//! ```no_run
//! let mut scope = rhai::Scope::new();
//! scope.push("ten", 10.);
//! ui.add(DragValueExpr::new(&mut self.value, Some(&scope)));
//! ```
//!
//! Without a scope:
//! ```no_run
//! ui.add(DragValueExpr::new(&mut self.value, None));
//! ```

use egui::*;
use rhai::{Dynamic, Engine, Scope};

/// A widget that allows the user to edit a f64 value using a drag slider and a text field.
/// The text field allows the user to enter an expression that will be evaluated by a Rhai
/// expression-only Engine instance.
pub struct DragValueExpr<'a> {
    scope: Option<&'a Scope<'a>>,
    drag_value: DragValue<'a>,
}

impl<'a> DragValueExpr<'a> {
    /// Create a new DragValueExpr from a mutable reference to a f64 and an optional Scope.
    pub fn new(value: &'a mut f64, scope: Option<&'a Scope>) -> Self {
        Self {
            scope,
            drag_value: DragValue::new(value).update_while_editing(false),
        }
    }

    /// Create a new DragValueExpr from a DragValue and an optional Scope.
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
