use std::rc::Rc;

use wasm_bindgen::prelude::*;

use crate::game_tree::Tree;
use crate::nodes::{AsciiRenderer, Script};

pub trait Node {
    fn update(&self);
}

#[derive(Default)]
struct Transform {
    x: f32,
    y: f32,
}

#[wasm_bindgen(typescript_custom_section)]
const SCRIPT: &'static str = r#"
interface JsGameObject {
    update: (ga: GameObjectShim) => void;
}
"#;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsGameObject")]
    pub type JsGameObject;

    #[wasm_bindgen(structural, method)]
    pub fn update(this: &JsGameObject, ga: GameObjectShim);
}

#[wasm_bindgen]
pub struct GameObjectShim {
    inner: Rc<GameObject>,
}

#[wasm_bindgen]
impl GameObjectShim {
    pub fn id(&self) -> usize {
        self.inner.id
    }
}

pub struct GameObject {
    id: usize,
    transform: Transform,
}

#[wasm_bindgen]
pub struct Game {
    id_count: usize,
    objects: Vec<(Rc<GameObject>, Box<JsGameObject>)>,
    renderer: AsciiRenderer,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { id_count: 0, objects: vec![], renderer: AsciiRenderer::new() }
    }

    pub fn add_game_object(&mut self, js_object: JsGameObject) -> usize {
        let id = self.id_count;
        self.id_count += 1;
        let ga = Rc::new(GameObject { id, transform: Default::default() });
        self.objects.push((ga, Box::new(js_object)));
        return id;
    }

    pub fn update(&mut self) {
        for (ga, js) in &self.objects {
            js.update(GameObjectShim { inner: ga.clone() });
        }
        self.draw();
    }

    pub fn draw(&mut self) {
        let mut value = String::new();
        for (ga, ..) in &self.objects {
            value += &ga.id.to_string();
        }

        self.renderer.set_text(value);
    }
}