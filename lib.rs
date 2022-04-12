mod utils;

use funniversaries::entities::anniversary::Anniversary;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate_future_anniversaries(date: &str) -> JsValue {
    let anniversaries = funniversaries::find_anniversaries_future(date);

    filter(anniversaries)
}

#[wasm_bindgen]
pub fn generate_past_anniversaries(date: &str) -> JsValue {
    let anniversaries = funniversaries::find_anniversaries_past(date);

    filter(anniversaries)
}

#[wasm_bindgen]
pub fn generate_all_anniversaries(date: &str) -> JsValue {
    let anniversaries = funniversaries::find_anniversaries_all(date);

    filter(anniversaries)
}

fn filter(mut anniversaries: Vec<Anniversary>) -> JsValue {
    // Sort anniversaries by date
    anniversaries.sort_by(|a, b| b.date.cmp(&a.date));

    JsValue::from_serde(&anniversaries).unwrap()
}
