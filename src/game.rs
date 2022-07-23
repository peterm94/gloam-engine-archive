use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use wasm_bindgen::prelude::*;

use crate::game_tree::Tree;
use crate::nodes::{AsciiRenderer, Script};

thread_local! {
    pub static GAME: GameStorage = GameStorage {
                id_count: 1,
                objects: HashMap::new(),
                // renderer: AsciiRenderer::new(),
    };
}

unsafe impl Sync for GameStorage {}


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
    init: () => void;
    update: () => void;
}
"#;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsGameObject")]
    pub type JsGameObject;

    #[wasm_bindgen(structural, method)]
    pub fn update(this: &JsGameObject);

    #[wasm_bindgen(structural, method)]
    pub fn init(this: &JsGameObject);
}


pub struct GameObject {
    id: usize,
    parent: usize,
    transform: Transform,
}

#[derive(Eq, Hash, PartialEq)]
pub struct ObjRef {
    id: usize,
    parent: usize,
}

pub struct GameStorage {
    pub id_count: usize,
    // objects: Vec<(Rc<GameObject>, Box<JsGameObject>)>,
    pub objects: HashMap<ObjRef, Box<JsGameObject>>,
    // renderer: AsciiRenderer,
}

impl GameStorage {
    pub fn add_game_object(&mut self, js_object: JsGameObject) -> usize {
        let id = self.id_count;
        self.id_count += 1;

        self.objects.insert(ObjRef { id, parent: 0 }, Box::new(js_object));

        // let ga = Rc::new(GameObject { id, parent: 0, transform: Default::default() });
        // self.objects.push((ga, Box::new(js_object)));

        // call init on it
        return id;
    }
}

#[wasm_bindgen]
pub struct Game {
    // game_storage: GameStorage
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self) {
        GAME.with(|storage| {
            for (obj_ref, script) in storage.objects.iter() {
                script.update();
            }
        });

        self.draw();
    }

    pub fn draw(&mut self) {
        // let mut value = String::new();
        // for (ga, ..) in &self.game_storage.objects {
        //     value += &ga.id.to_string();
        // }
        //
        // self.game_storage.borrow_mut().renderer.set_text(value);
    }

    // TODO remove this, add a way for a game init that takes the shim like the others
    pub fn add_game_object(&mut self, js_object: JsGameObject) -> usize {
        // GAME.borrow().add_game_object(js_object)
        0
    }
}