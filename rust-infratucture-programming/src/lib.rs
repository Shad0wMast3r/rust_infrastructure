mod utils;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//Thus right here imports 'window alert'
#[wasm_bindgen]
pub fn main(){
    let document=web_sys::window().unwrap().document().unwrap();
    let body=document.body().unwrap();

    //Math from Rust
    let p=document.create_element("p").unwrap();
    let x:i8=1;
    let y:i8=2;
    let answer:i8=x+y;
    let string_answer=answer.to_string();
    p.set_inner_html(&string_answer);
    body.append_child(&p).unwrap();

    //Create a canvas element
    let canvas=document.create_element("div").unwrap();
    canvas.set_id("canvas");
    body.append_child(&canvas).unwrap();

    //return the value
}