use wasm_bindgen::prelude::*;
use nalgebra::{Complex, Normed};

#[wasm_bindgen]
pub fn julia(c_re:f64, c_im:f64, x:f64, y:f64) -> u8 {
    let c = Complex::new(c_re, c_im);
    let mut z = Complex::new(x, y);

    for i in 0..255 {
        if z.norm() > 2.0 {
            return i as u8;
        }
        z = z * z + c;
    }
    255
}

