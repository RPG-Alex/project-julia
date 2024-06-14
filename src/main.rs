mod color_gradient;
mod controll;
mod fractal_plugin;
mod sets;
mod traits;

use bevy::{app::App, math::Vec2, DefaultPlugins};
use fractal_plugin::FractalPlugin2d;
use sets::julia::JuliaMaterial;

fn main()
{
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FractalPlugin2d::<JuliaMaterial>::default())
    .insert_resource(controll::MouseState {
      position: Vec2::ZERO,
    })
    .run();
}
