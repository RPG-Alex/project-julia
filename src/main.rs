use bevy::prelude::*;
use nalgebra::{Complex, Normed};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, julia_fractal_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// System to generate and render Julia fractals
fn julia_fractal_system() {
    
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
