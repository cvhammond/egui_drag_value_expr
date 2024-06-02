# egui_drag_value_expr

![Crates.io](https://img.shields.io/crates/v/c3dio.svg)

An egui DragValue widget that accepts expression-based values.


 ## Usage

 A rhai::Scope can be passed to the DragValueExpr widget to allow the user to enter expressions that reference variables in the scope.

 With a scope:
 ```no_run
 let mut scope = rhai::Scope::new();
 scope.push("ten", 10.);
 ui.add(DragValueExpr::new(&mut self.value, Some(&scope)));
 ```

 Without a scope:
 ```no_run
 ui.add(DragValueExpr::new(&mut self.value, None));
 ```

## Contributing

PRs, feature requests, and issues are welcome!

