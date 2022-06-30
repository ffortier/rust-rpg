extern crate console_error_panic_hook;

use app::AppComponent;
use std::panic;
use wasm_bindgen::prelude::*;

mod app;

#[wasm_bindgen(start)]
pub fn start() {
  wasm_logger::init(wasm_logger::Config::default());
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  yew::start_app::<AppComponent>();
}
