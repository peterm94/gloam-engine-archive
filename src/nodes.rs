use wasm_bindgen::prelude::*;

trait Node {
    fn update();
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

#[wasm_bindgen]
pub fn run_update(script: &Script) {
    let _ = script.update();
}