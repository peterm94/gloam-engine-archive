use std::cell::RefCell;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::console;

type EcsId = usize;
type CompType = String;


#[derive(Default)]
pub struct Components {
    components: Vec<Box<RefCell<dyn std::any::Any>>>,
}

struct Entity {
    entity_id: EcsId,
    components: HashMap<CompType, Box<RefCell<dyn std::any::Any>>>,
}

impl Entity {
    fn new(entity_id: EcsId) -> Self {
        Self { entity_id, components: Default::default() }
    }
}


#[wasm_bindgen]
#[derive(Default)]
pub struct Game {
    entities: HashMap<EcsId, Entity>,
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
        self.entities.insert(entity_id, Entity::new(entity_id));

        return entity_id;
    }

    pub fn add_component_web(&mut self, entity_id: EcsId, component: JsValue) -> String {
        console::log_1(&format!("Adding web component").into());

        let proto = js_sys::Object::get_prototype_of(&component);
        let constructor = proto.constructor();
        let name = constructor.name().as_string().unwrap();
        console::log_1(&format!("constructor name: {}", &name).into());
        return name;
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::Game;

    #[test]
    fn test_native_comp() {
        struct A;
        struct B;

        let mut game = Game::new("");

        game.
    }
}

struct Comp;