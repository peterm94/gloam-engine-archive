use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

use js_sys::JsString;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::console::log_1;

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
pub static OBJECTS_INDEX: RefCell<ObjectsIndex> = RefCell::new(ObjectsIndex::default());
}
pub static mut DEL_OBJECTS: Vec<usize> = vec![];

#[derive(Default)]
pub struct ObjectsIndex {
    names: HashMap<String, usize>,
    types: HashMap<String, Box<Vec<usize>>>,
    tags: HashMap<String, Box<Vec<usize>>>,
}


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
        let name = Gloam::get_js_obj_name(&js_object);

        console::log_1(&format!("ADD {name}").into());
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

    pub fn with_object(id: usize, f: &js_sys::Function) {
        OBJECTS.with(|objects| {
            let this = JsValue::null();
            let objects = objects.borrow();
            let obj = objects.get(&id).unwrap();
            f.call1(&this, obj);
        });
    }

    pub fn with_type(type_name: &JsString, f: &js_sys::Function) {
        let name: String = type_name.into();
        OBJECTS_INDEX.with(|index| {
            if let Some(ids) = index.borrow().types.get(&name) {
                OBJECTS.with(|objects| {
                   let objects = objects.borrow();
                    for id in ids.iter() {
                        let this = JsValue::null();
                        let obj = objects.get(id).unwrap();
                        f.call1(&this, obj);
                    }
                });
            }
        })
    }

    pub fn find_objs_with_type(type_name: &JsString) -> Vec<usize> {
        unimplemented!()
    }

    pub fn find_obj_with_type(type_name: &JsString) -> usize {
        unimplemented!()
    }

    fn get_js_obj_name(x: &JsValue) -> String {
        let proto = js_sys::Object::get_prototype_of(x);
        let constructor = proto.constructor();
        constructor.name().as_string().unwrap()
    }
}