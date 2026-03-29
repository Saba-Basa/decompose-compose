use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
pub fn lookup(v: f64, ramp: &str) -> Option<String> {    if !ramp.is_ascii() { return None; }
    let n: usize = ramp.len();
    if n == 0 { return None; }

    let v: f64 = v.clamp(0.0,1.0);
    let k: usize = (v * (n - 1) as f64).round() as usize;
    let k: usize = k.min(n - 1);
    Some((ramp.as_bytes()[k] as char).to_string())

}