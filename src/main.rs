use bevy::prelude::*;

mod color_gradient;
mod controll;
mod sets;
use controll::{click_and_drag_with_mouse, zoom_with_mouse_wheel};
use sets::julia;

fn main()
{
  App::new()
    .add_plugins((DefaultPlugins, julia::PostProcessPlugin))
    .add_systems(Startup, julia::setup)
    .add_systems(
      Update,
      (julia::resize_window, zoom_with_mouse_wheel, click_and_drag_with_mouse),
    )
    .run();
}
