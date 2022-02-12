use crate::types::Url;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = parseUrl)]
pub fn parse_url(input: String) -> Result<JsValue, JsValue> {
    let url = Url::new(&input).map_err(|error| JsValue::from(error.to_string()))?;

    JsValue::from_serde(&url.to_serializable()).map_err(|error| JsValue::from(error.to_string()))
}
