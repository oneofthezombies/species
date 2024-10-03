#![no_main]

use js_sys::wasm_bindgen;
use js_sys::JSON;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use web_sys::wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn parse_js_sys(json_string: &str) {
    let _result = JSON::parse(json_string);
}

#[wasm_bindgen]
pub fn parse_js_sys_js_value(dto: &JsValue) {
    let json_string = js_sys::Reflect::get(dto, &JsValue::from_str("value")).unwrap();
    let _result = JSON::parse(&json_string.as_string().unwrap());
}

#[wasm_bindgen]
pub fn parse_serde(json_string: &str) {
    let _result: Result<Value, serde_json::Error> = serde_json::from_str(json_string);
}
