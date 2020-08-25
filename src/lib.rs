use stremio_core::types::MetaPreview;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn meta_preview(data: &JsValue) -> JsValue {
    JsValue::from_serde(&data.into_serde::<MetaPreview>().ok()).unwrap()
}
