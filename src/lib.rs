use serde::{Deserialize, Serialize};
use stremio_core::types::addon::{
    Descriptor, DescriptorPreview, Manifest, ManifestPreview, ResourceResponse,
};
use stremio_core::types::resource::{MetaItem, MetaItemPreview, Stream, Subtitles};
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
pub fn meta_item(data: &JsValue) -> JsValue {
    validate::<MetaItem>(data)
}

#[wasm_bindgen]
pub fn meta_item_preview(data: &JsValue) -> JsValue {
    validate::<MetaItemPreview>(data)
}

#[wasm_bindgen]
pub fn stream(data: &JsValue) -> JsValue {
    validate::<Stream>(data)
}

#[wasm_bindgen]
pub fn subtitles(data: &JsValue) -> JsValue {
    validate::<Subtitles>(data)
}

#[wasm_bindgen]
pub fn resource_response(data: &JsValue) -> JsValue {
    validate::<ResourceResponse>(data)
}
