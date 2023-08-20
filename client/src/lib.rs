use app::*;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use log::Level;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {

    setup_logging();

    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}

#[cfg(feature = "hydrate")]
fn setup_logging() {
    console_log::init_with_level(Level::Debug).expect("error initializing log");
    console_error_panic_hook::set_once();
}