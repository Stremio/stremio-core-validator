use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use stremio_core::types::addon::{
    Descriptor, DescriptorPreview, Manifest, ManifestPreview, ResourceResponse,
};
use stremio_core::types::resource::{MetaItem, MetaItemPreview, Stream, Subtitles, Video};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

fn validate<T: Serialize + for<'a> Deserialize<'a>>(data: &JsValue) -> Result<JsValue, JsValue> {
    js_sys::JSON::stringify(&data)
        .map(|data| Into::<String>::into(data))
        .and_then(|data| {
            let mut deserializer = Deserializer::from_str(data.as_str());
            serde_path_to_error::deserialize::<_, T>(&mut deserializer)
                .map(|data| JsValue::from_serde(&data).expect("data serialization failed"))
                .map_err(|error| JsValue::from(js_sys::Error::new(error.to_string().as_str())))
        })
}

#[wasm_bindgen]
pub fn manifest(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<Manifest>(data)
}

#[wasm_bindgen]
pub fn manifest_preview(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<ManifestPreview>(data)
}

#[wasm_bindgen]
pub fn descriptor(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<Descriptor>(data)
}

#[wasm_bindgen]
pub fn descriptor_preview(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<DescriptorPreview>(data)
}

#[wasm_bindgen]
pub fn meta_item(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<MetaItem>(data)
}

#[wasm_bindgen]
pub fn meta_item_preview(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<MetaItemPreview>(data)
}

#[wasm_bindgen]
pub fn stream(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<Stream>(data)
}

#[wasm_bindgen]
pub fn subtitles(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<Subtitles>(data)
}

#[wasm_bindgen]
pub fn video(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<Video>(data)
}

#[wasm_bindgen]
pub fn resource_response(data: &JsValue) -> Result<JsValue, JsValue> {
    validate::<ResourceResponse>(data)
}
