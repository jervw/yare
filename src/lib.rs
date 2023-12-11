mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

// Will be called in content.js
#[wasm_bindgen]
pub fn hello_content() {
    alert("Hello from the content script!");
}

// Will be called in background.js
#[wasm_bindgen]
pub fn hello_background() {
    log("Hello from the background script!");
}
