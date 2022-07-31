use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

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

pub static mut ID_COUNT: usize = 1;
pub static mut NEXT_OBJECTS: Vec<(usize, JsGameObject)> = vec![];

thread_local! {
pub static OBJECTS: RefCell<HashMap<usize, JsGameObject>> = RefCell::new(HashMap::new());
}
pub static mut DEL_OBJECTS: Vec<usize> = vec![];

#[wasm_bindgen]
struct Gloam;

#[wasm_bindgen]
impl Gloam {
    pub fn update() {
        unsafe {
            DEL_OBJECTS.drain(..).for_each(|x| {
                OBJECTS.with(|objects| {
                    objects.borrow_mut().remove(&x);
                })
            });

            if !NEXT_OBJECTS.is_empty() {
                // move the pending additions out of the static so it doesn't cause problems with init()
                let mut temp = vec![];
                temp.append(&mut NEXT_OBJECTS);

                // init each object
                temp.iter().for_each(|(_, x)| x.init());

                // Put them in the global object map
                OBJECTS.with(|objects| {
                    let mut objects = objects.borrow_mut();
                    temp.into_iter().for_each(|(k, v)| { objects.insert(k, v); });
                });
            }
            OBJECTS.with(|objects| {
                for (_, object) in objects.borrow().iter() {
                    object.update();
                }
            });
        }
    }

    pub fn add_object(js_object: JsGameObject) -> usize {
        unsafe {
            let id = ID_COUNT;
            ID_COUNT += 1;
            NEXT_OBJECTS.push((id, js_object));
            id
        }
    }

    pub fn destroy_object(id: usize) {
        unsafe { DEL_OBJECTS.push(id) };
    }
}