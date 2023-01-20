use wasm_bindgen::prelude::*;
use web_sys::{NodeList, HtmlElement};

#[wasm_bindgen]
extern {
    pub type HTMLDocument;
    pub static document: HTMLDocument;
    #[wasm_bindgen(method, js_name = querySelector)]
    pub fn query_selector(this: &HTMLDocument, selector: &str) -> Option<HtmlElement>;
    #[wasm_bindgen(method, js_name = querySelectorAll)]
    pub fn query_selector_all(this: &HTMLDocument, selector: &str) -> NodeList;
}

#[wasm_bindgen]
extern "C" {
    // Log MouseEvent
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: JsValue);
}
