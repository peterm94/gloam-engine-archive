use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::console;

type EcsId = usize;
type CompType = String;


#[derive(Default)]
pub struct Components {
    components: Vec<Box<RefCell<dyn Any>>>,
}

struct Entity {
    entity_id: EcsId,
    components: HashMap<CompType, Box<RefCell<dyn Any>>>,
}

impl Entity {
    fn add_component(&mut self, comp_type: CompType, component: Box<RefCell<dyn Any>>) {
        self.components.insert(comp_type, component);
    }

    fn get_component(&mut self, comp_type: CompType) -> Option<&mut Box<RefCell<dyn Any>>> {
        self.components.get_mut(&comp_type)
    }
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

    pub fn add_component_web(&mut self, entity_id: EcsId, component: JsValue) {
        console::log_1(&format!("Adding web component").into());

        let proto = js_sys::Object::get_prototype_of(&component);
        let constructor = proto.constructor();
        let name = constructor.name().as_string().unwrap();
        console::log_1(&format!("Component type: {}", &name).into());
        if let Some(e) = self.entities.get_mut(&entity_id) {
            let wrapped_comp = Box::new(RefCell::new(component));
            e.add_component(name, wrapped_comp);
        }
    }

    // TODO make this function signature something real and make it work.
    pub fn get_component_web(&mut self, entity_id: EcsId, comp_type: CompType) -> Option<usize> {
        if let Some(e) = self.entities.get_mut(&entity_id) {
            // return e.get_component(comp_type);
            return None;
        }
        return None;
    }


    pub fn start(&self) {
        console::log_1(&format!("Starting game...").into());
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

    }
}

struct Comp;