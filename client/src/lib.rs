#![recursion_limit="512"]

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

mod app;
mod routes;
mod components;
mod views;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
