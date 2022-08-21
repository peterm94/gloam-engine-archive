use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use data_url::DataUrl;
use image::ImageFormat;
use js_sys::{Array, Function, JsString};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::console::log_1;

use crate::renderer::{Renderer, Texture};

#[wasm_bindgen(typescript_custom_section)]
const SCRIPT: &'static str = r#"
interface JsGameObject {
    init(): void;
    update(delta: number): void;
}
"#;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsGameObject")]
    pub type JsGameObject;

    #[wasm_bindgen(structural, method)]
    pub fn update(this: &JsGameObject, delta: f64);

    #[wasm_bindgen(structural, method)]
    pub fn init(this: &JsGameObject);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "(object: JsGameObject) => void")]
    pub type WithObjFn;

    #[wasm_bindgen(typescript_type = "number[]")]
    pub type JsNumArray;
}

pub static mut ID_COUNT: usize = 1;
pub static mut NEXT_OBJECTS: Vec<(usize, JsGameObject)> = vec![];

thread_local! {
pub static OBJECTS: RefCell<HashMap<usize, JsGameObject>> = RefCell::new(HashMap::new());
pub static OBJECTS_INDEX: RefCell<ObjectsIndex> = RefCell::new(ObjectsIndex::default());
pub static RENDERER: RefCell<Renderer> = RefCell::new(Renderer::new("canvas").unwrap());
}
pub static mut TEXTURES: Vec<Texture> = vec![];
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
    pub fn update_once(delta: f64) {
        Gloam::update(delta);
    }

    fn update(delta: f64) {
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
                    object.update(delta);
                }
            });

            // Render
            RENDERER.with(|renderer| {
                let renderer = renderer.borrow();
                for texture in &TEXTURES {
                    renderer.draw_image(texture, 0, 0);
                }
            });
        }
    }

    // TODO https://webpack.js.org/guides/asset-modules/#resource-assets
    pub fn load_texture(img_data: &str) -> usize {
        let url = DataUrl::process(img_data).unwrap();
        let (body, ..) = url.decode_to_vec().unwrap();

        let img = image::load_from_memory(&body).unwrap();
        let img = img.to_rgba8();
        let len = RENDERER.with(|renderer| {
            let tex = renderer.borrow().load_texture(img);
            unsafe { TEXTURES.push(tex) };
            unsafe { TEXTURES.len() }
        });
        return len - 1;
    }

    pub fn add_object(js_object: JsGameObject) -> usize {
        let name = Gloam::get_js_obj_name(&js_object);
        unsafe {
            let id = ID_COUNT;
            ID_COUNT += 1;
            NEXT_OBJECTS.push((id, js_object));

            Gloam::add_type(name, id);
            id
        }
    }

    fn add_type(name: String, id: usize) {
        OBJECTS_INDEX.with(|index| {
            let mut index = index.borrow_mut();
            if let Some(inner) = index.types.get_mut(&name) {
                inner.push(id);
            } else {
                index.types.insert(name, Box::new(vec![id]));
            }
        });
    }

    pub fn destroy_object(id: usize) {
        unsafe { DEL_OBJECTS.push(id) };
    }

    pub fn with_object(id: usize, f: &js_sys::Function) {
        OBJECTS.with(|objects| {
            let this = JsValue::null();
            let objects = objects.borrow();
            if let Some(obj) = objects.get(&id) {
                f.call1(&this, obj).unwrap();
            }
        });
    }

    pub fn with_objects(ids: JsNumArray, f: &WithObjFn) {
        let ids = JsValue::from(ids).unchecked_into::<Array>();
        let f = JsValue::from(f).unchecked_into::<Function>();

        let this = JsValue::null();

        OBJECTS.with(|objects| {
            let objects = objects.borrow();

            for id in ids.iter() {
                let id = JsValue::from(id).as_f64().unwrap() as usize;
                match ids.length() {
                    // TODO can I make this dynamic?
                    1 => {
                        if let Some(o1) = objects.get(&id) {
                            f.call1(&this, o1).unwrap();
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    pub fn with_type(type_name: &JsString, f: &WithObjFn) {
        let f = JsValue::from(f).unchecked_into::<Function>();
        let name: String = type_name.into();
        OBJECTS_INDEX.with(|index| {
            if let Some(ids) = index.borrow().types.get(&name) {
                OBJECTS.with(|objects| {
                    let objects = objects.borrow();
                    for id in ids.iter() {
                        let this = JsValue::null();
                        if let Some(obj) = objects.get(&id) {
                            f.call1(&this, obj).unwrap();
                        }
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

    pub fn start() {
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let outer_f = f.clone();

        let window = web_sys::window().unwrap();
        if let Some(perf) = window.performance() {
            let mut prev_delta = perf.now();

            *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                let now = perf.now();
                let delta = now - prev_delta;
                prev_delta = now;

                Gloam::update(delta);

                // TODO https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
                window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
            }) as Box<dyn FnMut()>));
        }

        let window = web_sys::window().unwrap();
        window.request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
    }
}
