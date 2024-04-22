use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen(js_namespace = crypto)]
extern "C" {
    pub fn randomUUID() -> String;
}
