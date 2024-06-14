mod color_gradient;
mod control;
mod fractal_plugin;
mod sets;
mod traits;

use bevy::{app::App, math::Vec2, DefaultPlugins};
use fractal_plugin::FractalPlugin2d;
use sets::{julia::JuliaMaterial, mandelbrot::MandelbrotMaterial};

fn main()
{
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FractalPlugin2d::<MandelbrotMaterial>::default())
    // .add_plugins(FractalPlugin2d::<JuliaMaterial>::default())
    .insert_resource(control::MouseState {
      position: Vec2::ZERO,
    })
    .run();
}
