mod utils;

use wasm_bindgen::prelude::*;
use web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//Thus right here imports 'window alert'

//So lets see what we can do with arrays and returning values in Rust.__rust_force_expr!
//    /*let links2=vec!["www.google.com","www.apple.com","www.192.168.1.1"];
    //Vectors are resizable arrays.
    //let navigation_links:[String,3]=["www.google.com","www.apple.com","www.192.192.192"];

    //Since were only using three links we only want three}

#[wasm_bindgen]
pub fn main(){
    let document=web_sys::window().unwrap().document().unwrap();
    let body=document.body().unwrap();
    //Math from Rust
    let x:i8=1; 
    let y:i8=2;
    let answer=x+y;
    let string_answer=answer.to_string();
    let p=document.create_element("p").unwrap();//initiate the element in html.
    p.set_inner_html(&string_answer); //set the p tag in html.
    body.append_child(&p).unwrap(); //adding the value to the P tag. WTF

    //First Assign the nav container
    let google:String="www.google.com".to_string();
    let apple:String="www.apple.com".to_string();
    let router:String="192.168.1.1".to_string();
    let navigation_links=[google,apple,router];
    let href=document.create_element("a").unwrap(); //THIS <> HTML
    href.set_inner_html(&format!("{:?}", navigation_links.into_iter()));
    body.append_child(&href).unwrap(); //<>

    //Create a canvas element
    let canvas=document.create_element("div").unwrap();
    canvas.set_id("canvas");
    body.append_child(&canvas).unwrap();

    

    
}//end of main