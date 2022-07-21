use std::{cell::RefCell, rc::Rc};

use anyhow::{Context, Result};
use js_sys::{Array, Function, Promise};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlImageElement};

use crate::error::WebError;

pub struct AssetsLoader {
    pub base_path: String,
}

#[cfg_attr(test, mockall::automock)]
impl AssetsLoader {
    pub async fn load_image(&self, name: &str) -> Result<HtmlImageElement> {
        let document = window()
            .ok_or(WebError::NoWindow)?
            .document()
            .ok_or(WebError::NoDocument)?;

        let image = document
            .create_element("img")
            .map_err(WebError::from)
            .context("creating image element")?
            .dyn_into::<HtmlImageElement>()
            .or(Err(WebError::Cast("HtmlImageElement")))?;

        image.set_src(format!("{}{}", self.base_path, name).as_str());

        let image = Rc::new(RefCell::new(Some(image)));
        let p = create_load_promise(image.clone());

        JsFuture::from(p)
            .await
            .map_err(WebError::from)
            .context("awaiting load/error callbacks")?;

        Ok(image.take().unwrap())
    }
}

impl Default for AssetsLoader {
    fn default() -> Self {
        Self {
            base_path: "/assets/".to_string(),
        }
    }
}

fn create_load_promise(image: Rc<RefCell<Option<HtmlImageElement>>>) -> Promise {
    Promise::new(&mut move |resolve, reject| {
        let load_cb_ref = Rc::new(RefCell::new(None));
        let error_cb_ref = Rc::new(RefCell::new(None));

        let load_cb = create_promise_callback(
            resolve,
            load_cb_ref.clone(),
            error_cb_ref.clone(),
            image.clone(),
        );

        let error_cb = create_promise_callback(
            reject,
            load_cb_ref.clone(),
            error_cb_ref.clone(),
            image.clone(),
        );

        load_cb_ref.replace(Some(load_cb));
        error_cb_ref.replace(Some(error_cb));

        add_event_listener_with_callback(&image, "load", &load_cb_ref);
        add_event_listener_with_callback(&image, "error", &error_cb_ref);
    })
}

fn create_promise_callback(
    func: Function,
    load_cb_ref: Rc<RefCell<Option<Function>>>,
    error_cb_ref: Rc<RefCell<Option<Function>>>,
    image: Rc<RefCell<Option<HtmlImageElement>>>,
) -> Function {
    Closure::once_into_js(move || {
        func.apply(&JsValue::NULL, &Array::new()).unwrap();

        remove_event_listener_with_callback(&image, "load", load_cb_ref);
        remove_event_listener_with_callback(&image, "error", error_cb_ref);
    })
    .dyn_into::<Function>()
    .unwrap()
}

fn remove_event_listener_with_callback(
    image: &Rc<RefCell<Option<HtmlImageElement>>>,
    event_type: &str,
    cb: Rc<RefCell<Option<Function>>>,
) {
    image
        .borrow()
        .as_ref()
        .unwrap()
        .remove_event_listener_with_callback(event_type, cb.borrow().as_ref().unwrap())
        .unwrap();

    cb.take();
}

fn add_event_listener_with_callback(
    image: &Rc<RefCell<Option<HtmlImageElement>>>,
    event_type: &str,
    cb: &Rc<RefCell<Option<Function>>>,
) {
    image
        .borrow()
        .as_ref()
        .unwrap()
        .add_event_listener_with_callback(event_type, cb.borrow().as_ref().unwrap())
        .unwrap();
}
