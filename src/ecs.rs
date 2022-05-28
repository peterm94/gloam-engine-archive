use std::cell::RefCell;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::console;

type EcsId = usize;


#[derive(Default)]
pub struct Components {
    components: Vec<Box<RefCell<dyn std::any::Any>>>,
}


#[wasm_bindgen]
#[derive(Default)]
pub struct Game {
    entities: HashMap<EcsId, Components>,
    ecs_id_count: EcsId,
}

#[wasm_bindgen]
impl Game {
    pub fn new(_canvas_id: &str) -> Self {
        Game::default()
    }

    pub fn update(&self) {}

    pub fn draw(&self) {}

    pub fn add_entity(&mut self) -> EcsId {
        let entity_id = self.ecs_id_count;
        self.ecs_id_count += 1;
        self.entities.insert(entity_id, Components::default());

        return entity_id;
    }

    pub fn add_component_web(&mut self, entity_id: EcsId, component: JsValue) -> String {
        console::log_1(&format!("hello").into());

        let proto = js_sys::Object::get_prototype_of(&component);
        let constructor = proto.constructor();
        let name = constructor.name().as_string().unwrap();
        console::log_1(&format!("constructor name: {}", &name).into());
        return name;
    }
}