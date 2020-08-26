use serde::{Deserialize, Serialize};
use stremio_core::types::addons::{
    Descriptor, DescriptorPreview, Manifest, ManifestPreview, ResourceResponse,
};
use stremio_core::types::{MetaDetail, MetaPreview, Stream, SubtitlesSource};
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
pub fn descriptor(data: &JsValue) -> JsValue {
    validate::<Descriptor>(data)
}

#[wasm_bindgen]
pub fn descriptor_preview(data: &JsValue) -> JsValue {
    validate::<DescriptorPreview>(data)
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

#[wasm_bindgen]
pub fn subtitles_source(data: &JsValue) -> JsValue {
    validate::<SubtitlesSource>(data)
}

#[wasm_bindgen]
pub fn resource_response(data: &JsValue) -> JsValue {
    validate::<ResourceResponse>(data)
}
