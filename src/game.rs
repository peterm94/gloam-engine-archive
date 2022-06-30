use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
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
    init: (ga: GameObjectShim) => void;
    update: (ga: GameObjectShim) => void;
}
"#;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsGameObject")]
    pub type JsGameObject;

    #[wasm_bindgen(structural, method)]
    pub fn update(this: &JsGameObject, ga: GameObjectShim);

    #[wasm_bindgen(structural, method)]
    pub fn init(this: &JsGameObject, ga: GameObjectShim);

}

// This needs to actually be the game, like a global namespace thingo that has all global functions
#[wasm_bindgen]
pub struct GameObjectShim {
    inner: Rc<GameObject>,
    game_storage: Rc<RefCell<GameStorage>>,
}

#[wasm_bindgen]
impl GameObjectShim {
    pub fn id(&self) -> usize {
        self.inner.id
    }
    pub fn add_game_object(&mut self, js_object: JsGameObject) -> usize {
        self.game_storage.borrow_mut().add_game_object(js_object)
        // init
    }
}

pub struct GameObject {
    id: usize,
    parent: usize,
    transform: Transform,
}

pub struct GameStorage {
    id_count: usize,
    objects: Vec<(Rc<GameObject>, Box<JsGameObject>)>,
    renderer: AsciiRenderer,

}

impl GameStorage {
    pub fn add_game_object(&mut self, js_object: JsGameObject) -> usize {
        let id = self.id_count;
        self.id_count += 1;
        let ga = Rc::new(GameObject { id, parent: 0, transform: Default::default() });
        self.objects.push((ga, Box::new(js_object)));
        return id;
    }
}

#[wasm_bindgen]
pub struct Game {
    game_storage: Rc<RefCell<GameStorage>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            game_storage: Rc::new(RefCell::new(GameStorage {
                id_count: 1,
                objects: vec![],
                renderer: AsciiRenderer::new(),
            }))
        }
    }

    pub fn update(&mut self) {
        for (ga, js) in &self.game_storage.borrow().objects {
            js.update(GameObjectShim { inner: ga.clone(), game_storage: self.game_storage.clone() });
        }
        self.draw();
    }

    pub fn init(&mut self, object_id: usize) {
        if let Some((_, js)) = self.game_storage.borrow().objects.iter().find(|x|x.0.id == object_id)
        {

        }
    }

    pub fn draw(&mut self) {
        let mut value = String::new();
        for (ga, ..) in &self.game_storage.borrow().objects {
            value += &ga.id.to_string();
        }

        self.game_storage.borrow_mut().renderer.set_text(value);
    }

    // TODO remove this, add a way for a game init that takes the shim like the others
    pub fn add_game_object(&mut self, js_object: JsGameObject) -> usize {
        self.game_storage.borrow_mut().add_game_object(js_object)
    }
}