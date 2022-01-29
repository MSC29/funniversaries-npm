mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate_anniversaries(date: &str) -> JsValue {
    let mut anniversaries = funniversaries::find_anniversaries_future(date);

    // Sort people by age
    anniversaries.sort_by(|a, b| b.date.cmp(&a.date));

    JsValue::from_serde(&anniversaries).unwrap()
}
