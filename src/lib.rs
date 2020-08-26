use serde::{Deserialize, Serialize};
use stremio_core::types::addons::{Manifest, ManifestPreview};
use stremio_core::types::{MetaDetail, MetaPreview, Stream};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

fn validate<T: Serialize + for<'a> Deserialize<'a>>(data: &JsValue) -> JsValue {
    JsValue::from_serde(&data.into_serde::<T>().ok()).unwrap()
}

#[wasm_bindgen]
pub fn manifest(data: &JsValue) -> JsValue {
    validate::<Manifest>(data)
}

#[wasm_bindgen]
pub fn manifest_preview(data: &JsValue) -> JsValue {
    validate::<ManifestPreview>(data)
}

#[wasm_bindgen]
pub fn meta_preview(data: &JsValue) -> JsValue {
    validate::<MetaPreview>(data)
}

#[wasm_bindgen]
pub fn meta_detail(data: &JsValue) -> JsValue {
    validate::<MetaDetail>(data)
}

#[wasm_bindgen]
pub fn stream(data: &JsValue) -> JsValue {
    validate::<Stream>(data)
}
