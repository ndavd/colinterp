mod color;
use crate::color::HexColor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_flat_palette(
    color_a: js_sys::JsString,
    color_b: js_sys::JsString,
    n: js_sys::Number,
) -> js_sys::Uint8Array {
    // Validation left to frontend
    let color_a = HexColor::from_str(&color_a.as_string().unwrap()).unwrap();
    let color_b = HexColor::from_str(&color_b.as_string().unwrap()).unwrap();
    let flat_palette = HexColor::get_palette(color_a, color_b, n.as_f64().unwrap() as usize)
        .iter()
        .flat_map(|color| [color.r, color.g, color.b])
        .collect::<Vec<_>>();
    js_sys::Uint8Array::from(&flat_palette[..])
}
