use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "
export function toObject(a, b) {
    return { a, b };
}
")]
extern "C" {
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(a: Vec<i32>, b: Vec<i32>) -> JsValue;
}

#[wasm_bindgen(js_name = doMath)]
pub fn do_math() -> JsValue {
    let a = vec![1, 2];
    let b = vec![3, 4];
    to_object(a, b)
}
