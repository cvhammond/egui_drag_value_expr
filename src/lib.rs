use egui::*;
use rhai::{Engine, Scope};

pub struct DragValueExpr<'a> {
    value: &'a mut f64,
    scope: Option<&'a Scope<'a>>,
}

impl<'a> DragValueExpr<'a> {
    pub fn new(value: &'a mut f64, scope: Option<&'a Scope>) -> Self {
        Self { value, scope }
    }
}

impl<'a> Widget for &mut DragValueExpr<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self.value).custom_parser(|s| {
            let engine = Engine::new();
            let mut scope = match self.scope {
                Some(scope) => scope.clone(),
                None => Scope::new(),
            };
            let result = engine.eval_expression_with_scope::<f64>(&mut scope, s);
            match result {
                Ok(result) => {
                    dbg!(result);
                    Some(result)
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    None
                }
            }
        }))
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
