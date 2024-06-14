use bevy::{prelude::*, sprite::Material2dPlugin};

mod color_gradient;
mod controll;
mod sets;
mod traits;
use controll::{click_and_drag_with_mouse, update_fractal_material, zoom_with_mouse_wheel};
use sets::julia::{create_screen_covering_triangle, JuliaMaterial};

fn main()
{
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(Material2dPlugin::<JuliaMaterial>::default())
    .insert_resource(controll::MouseState {
      position: Vec2::ZERO,
    })
    .add_systems(Startup, create_screen_covering_triangle::<JuliaMaterial>)
    .add_systems(
      Update,
      (
        update_fractal_material::<JuliaMaterial>,
        zoom_with_mouse_wheel::<JuliaMaterial>,
        click_and_drag_with_mouse::<JuliaMaterial>,
      ),
    )
    .run();
}
