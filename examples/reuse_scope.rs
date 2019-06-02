extern crate rhai;
use rhai::{Engine, Scope};

fn main() {
    let mut engine = Engine::new();
    let mut scope: Scope = Vec::new();

    assert!(engine
        .eval_with_scope::<()>(&mut scope, "let x = 4 + 5")
        .is_ok());

    if let Ok(result) = engine.eval_with_scope::<i64>(&mut scope, "x") {
        println!("result: {}", result);
    }
}
