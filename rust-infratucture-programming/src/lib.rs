mod utils;
mod stats; //WASM display usage stats in browser using system info stuff.

use wasm_bindgen::prelude::*;
use web_sys::Window;

#[wasm_bindgen] //ignore error.
extern "C"{
    #[wasm_bindgen(js_namespace=console)]
    fn log(s:&str);
}
#[wasm_bindgen(start)]
fn run(){
    log("Hello World!");
}
#[wasm_bindgen]
pub fn make_the_window_small(){
    let window=web_sys::window().unwrap();
    window.resize_to(500,500).expect("Window Size Could Not Be Reloaded!");
}