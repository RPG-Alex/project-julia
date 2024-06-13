use bevy::{prelude::*, sprite::Material2dPlugin};

mod color_gradient;
mod controll;
mod sets;
use controll::{click_and_drag_with_mouse, zoom_with_mouse_wheel};
use sets::julia;

fn main()
{
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(Material2dPlugin::<julia::JuliaMaterial>::default())
    .insert_resource(controll::MouseState {
      position: Vec2::ZERO,
    })
    .add_systems(Startup, julia::create_julia_triangle)
    .add_systems(
      Update,
      (julia::update_julia_triangle, zoom_with_mouse_wheel, click_and_drag_with_mouse),
    )
    .run();
}
