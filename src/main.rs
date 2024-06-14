mod color_gradient;
mod control;
mod fractal_plugin;
mod sets;
mod traits;

use bevy::{
  app::{App, Startup},
  math::Vec2,
  prelude::{Camera2dBundle, Commands},
  DefaultPlugins,
};
use fractal_plugin::FractalPlugin2d;
use sets::{julia::JuliaMaterial, mandelbrot::MandelbrotMaterial};

fn main()
{
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FractalPlugin2d::<MandelbrotMaterial>::default())
    // .add_plugins(FractalPlugin2d::<JuliaMaterial>::default())
    .add_systems(Startup, create_camera)
    .insert_resource(control::MouseState {
      position: Vec2::ZERO,
    })
    .run();
}

fn create_camera(mut commands: Commands) { commands.spawn(Camera2dBundle::default()); }
