use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn lookup(v: f64, ramp: &str) -> Option<String> {
    let n: usize = ramp.len();
    if n == 0 { return None; }

    let v: f64 = to_clamp(v, 0.0, 1.0)?;
    let k: usize = ((v * (n - 1) as f64).round() as usize).min(n - 1);
    Some((ramp.as_bytes()[k] as char).to_string())
}

pub fn to_clamp(v: f64, u: f64, l: f64) -> Option<f64> {
    if !v.is_finite() { return None; }
    let (l, u) = if l > u { (u, l) } else { (l, u) };
    if v < l { return Some(l); }
    if v > u { return Some(u); }
    Some(v)
}