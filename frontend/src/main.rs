#![no_main]

use leptos::*;
use wasm_bindgen::prelude::*;

mod app;
mod components;

use app::*;

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    
    // Include CSS
    let style = include_str!("styles/global.css");
    leptos::document().head().unwrap()
        .insert_adjacent_html("beforeend", &format!("<style>{}</style>", style))
        .unwrap();
    
    // Mount the main app that handles routing internally
    mount_to_body(App)
}
