use wasm_bindgen::prelude::*;

trait Node {
    fn update(&self);
}

#[wasm_bindgen(typescript_custom_section)]
const SCRIPT: &'static str = r#"
interface Script {
    update: () => void;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Script")]
    pub type Script;

    #[wasm_bindgen(structural, method)]
    pub fn update(this: &Script);
}


impl Node for Script {
    fn update(&self) {
        run_update(self)
    }
}

struct RustScript;
impl Node for RustScript {
    fn update(&self) {
        // println!("don't panic");
        web_sys::console::log_1(&"I made this in rust and didn't export the type".into());
    }
}

#[wasm_bindgen]
pub struct Game {
    scripts: Vec<Box<dyn Node>>,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Self { scripts: vec![] }
    }

    pub fn add_rust_script(&mut self) {
        self.scripts.push(Box::new(RustScript));
    }

    pub fn add_web_script(&mut self, script: Script) {
        self.scripts.push(Box::new(script));
    }

    pub fn run_all_scripts(&self) {
        for x in &self.scripts {
            x.update();
        }
    }
}

#[wasm_bindgen]
pub fn run_update(script: &Script) {
    let _ = script.update();
}

#[cfg(test)]
mod tests
{
    use crate::nodes::{Node, run_update, Script};

    struct Hello {}

    impl Node for Hello {
        fn update(&self) {
            println!("yay");
        }
    }

    #[test]
    fn test_rust_call_works() {
        let stuff: Vec<Box<dyn Node>> = vec![Box::new(Hello {}), Box::new(Script { obj: Default::default() })];
        for x in stuff {
            x.update();
        }
    }
}