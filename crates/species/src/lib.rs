#![no_std]

use js_sys::wasm_bindgen;
use js_sys::JSON;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::js_sys;

#[wasm_bindgen]
pub fn parse_js_sys(json_string: &str) {
    let result = JSON::parse(json_string);
    console::log(result)
}
