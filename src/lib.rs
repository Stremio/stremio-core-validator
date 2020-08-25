use serde::{Deserialize, Serialize};
use stremio_core::types::addons::Manifest;
use stremio_core::types::MetaPreview;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

fn validate<T: Serialize + for<'a> Deserialize<'a>>(data: &JsValue) -> JsValue {
    JsValue::from_serde(&data.into_serde::<T>().ok()).unwrap()
}

#[wasm_bindgen]
pub fn meta_preview(data: &JsValue) -> JsValue {
    validate::<MetaPreview>(data)
}

#[wasm_bindgen]
pub fn manifest(data: &JsValue) -> JsValue {
    validate::<Manifest>(data)
}
