use bevy::prelude::*;
use nalgebra::{Complex, Normed};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, julia_fractal_system)
        .run();
}

fn setup(commands: &mut Commands) {
    // Setup code (e.g., camera, UI, initial entities)
}

fn julia_fractal_system() {
    // System to generate and render Julia fractals
    // Use the existing julia function or adapt it for Bevy rendering
}

fn julia(c_re: f64, c_im: f64, x: f64, y: f64) -> u8 {
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
