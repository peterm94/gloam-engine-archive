use wasm_bindgen::prelude::*;
use crate::game::Node;


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

#[wasm_bindgen]
pub struct AsciiRenderer {
    node: web_sys::Node,
}

#[wasm_bindgen]
impl AsciiRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        // Manufacture the element we're gonna append
        let val = document.create_element("p").unwrap();
        val.set_text_content(Some("Hello from Rust!"));

        let node = body.append_child(&val).unwrap();
        Self { node }
    }

    pub fn set_text(&mut self, value: String) {
        self.node.set_text_content(Some(value.as_str()));
    }
}

impl Node for AsciiRenderer {
    fn update(&self) {
        // no op
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