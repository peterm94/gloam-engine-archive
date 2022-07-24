use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use wasm_bindgen::prelude::*;

use crate::game_tree::Tree;
use crate::nodes::{AsciiRenderer, Script};

thread_local! {
    pub static GAME: Rc<RefCell<GameStorage>> = Rc::new(RefCell::new(GameStorage {
                id_count: 1,
                objects: HashMap::new(),
                // renderer: AsciiRenderer::new(),
    }));
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
    init(): void;
    update(): void;
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
    pub objects: HashMap<usize, Rc<JsGameObject>>,
    // renderer: AsciiRenderer,
}

impl GameStorage {
    pub fn add_game_object(&mut self, js_object: JsGameObject) -> usize {
        let id = self.id_count;
        self.id_count += 1;

        self.objects.insert(id, Rc::new(js_object));
        return id;
    }

    pub fn get_script(&self, obj_id: usize) -> Rc<JsGameObject>
    {
        self.objects.get(&obj_id).unwrap().clone()
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
        Self { game_storage: GAME.with(|storage| { storage.clone() }) }
    }

    // pub(crate) fn wrapping(game_storage: Rc<RefCell<GameStorage>>) -> Self {
    //     Self { game_storage }
    // }

    pub fn update(&self) {

        // Add pending objects and run init on them

        // Call update on everything

        GAME.with(|storage| {
            // let storage = storage.clone().borrow();
            // for (obj_ref, script) in storage.objects.iter() {
            //     script.update();
            // }
        });

        self.draw();
    }

    pub fn draw(&self) {
        // let mut value = String::new();
        // for (ga, ..) in &self.game_storage.objects {
        //     value += &ga.id.to_string();
        // }
        //
        // self.game_storage.borrow_mut().renderer.set_text(value);
    }

    pub fn add_game_object(&self, js_object: JsGameObject) -> usize {

        let storage: &RefCell<GameStorage> = self.game_storage.borrow();
        let id = storage.borrow_mut().add_game_object(js_object);
        let script: Rc<JsGameObject> = storage.borrow().get_script(id);

        script.init();
        id
    }
}