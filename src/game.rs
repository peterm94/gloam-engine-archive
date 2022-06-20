use wasm_bindgen::prelude::*;

use crate::game_tree::Tree;
use crate::nodes::{AsciiRenderer, Script};

pub trait Node {
    fn update(&self);
}

#[wasm_bindgen]
pub struct Game {
    tree: Tree<Box<dyn Node>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { tree: Tree { all_nodes: vec![] } }
    }

    pub fn add_web_script(&mut self, script: Script) -> usize {
        self.tree.node(Box::new(script))
    }

    pub fn get_node(&self, node_id : usize) -> Option<Box<dyn Node>> {
        None
    }

    pub fn create_renderer(&mut self) -> usize {
        let renderer = AsciiRenderer::new();
        let node_id = self.tree.node(Box::new(renderer));

        return node_id;
    }

    pub fn run_all_scripts(&self) {
        for x in &self.tree.all_nodes {
            x.update();
        }
    }
}
