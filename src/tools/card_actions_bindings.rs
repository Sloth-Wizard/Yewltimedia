//! Rust wasm bindings

#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::*;
use web_sys::{NodeList, HtmlElement};

#[wasm_bindgen]
extern "C" {
    /// Document type
    pub type HtmlDocument;
    /// The Document element
    pub static document: HtmlDocument;
    /// querySelector() js binding
    #[wasm_bindgen(method, js_name = querySelector)]
    pub fn query_selector(this: &HtmlDocument, selector: &str) -> Option<HtmlElement>;
    /// querySelectorAll() js binding
    #[wasm_bindgen(method, js_name = querySelectorAll)]
    pub fn query_selector_all(this: &HtmlDocument, selector: &str) -> NodeList;

    /// Log MouseEvent
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    /// console.error
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: JsValue);
}
