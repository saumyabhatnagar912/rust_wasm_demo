extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C"{
    type HTMLDocument;
    pub type Element;

    static document : HTMLDocument;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(method, getter)]
    fn body(this : &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner(this: &Element, html: &str);
}


#[wasm_bindgen]
pub fn add(num1: &str, num2: &str, el : Element){
    let num1_int = num1.parse::<i32>().unwrap();
    let num2_int = num2.parse::<i32>().unwrap();
    let sum = num1_int + num2_int;
    let sum_string = sum.to_string();
    let sum_str: &str = &sum_string;
    el.set_inner(&format!("The Sum of {} and {} is: {}",num1, num2, sum_str));
    document.body().append(el);
    log(&format!("{} + {} =  {}", num1_int, num2_int, sum));
}

