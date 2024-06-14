use crate::traits::FractalMaterial2d;
use bevy::{
  asset::{Assets, Handle},
  ecs::{
    event::EventReader,
    system::{Query, Res, ResMut, Resource},
  },
  input::{
    mouse::{MouseButton, MouseWheel},
    ButtonInput,
  },
  math::{Vec2, Vec3},
  time::Time,
  transform::components::Transform,
  window::{CursorIcon, CursorMoved, Window, WindowResized},
};

#[derive(Resource)]
pub struct MouseState
{
  pub position: Vec2,
}

/// Zooms in and out, reacting to the mouse wheel.
pub fn zoom_with_mouse_wheel<M>(
  mut scroll_events: EventReader<MouseWheel>,
  fractal_handles: Query<&Handle<M>>,
  mut fractal_materials: ResMut<Assets<M>>,
) where
  M: FractalMaterial2d,
{
  for handle in fractal_handles.iter() {
    let material = fractal_materials
      .get_mut(handle)
      .expect("Fractal material not found");
    for event in scroll_events.read() {
      if event.y > 0.0 {
        material.zoom_in();
      } else {
        material.zoom_out();
      }
    }
  }
}

/// Click and drag with the mouse to translate the fractal.
pub fn click_and_drag_with_mouse<M>(
  mut mouse_event: EventReader<CursorMoved>,
  fractal_handles: Query<&Handle<M>>,
  mut windows: Query<&mut Window>,
  mouse_click: Res<ButtonInput<MouseButton>>,
  mut mouse_state: ResMut<MouseState>,
  mut fractal_materials: ResMut<Assets<M>>,
) where
  M: FractalMaterial2d,
{
  if let Some(mut window) = windows.iter_mut().next() {
    for event in mouse_event.read() {
      if mouse_click.pressed(MouseButton::Left) {
        window.cursor.icon = CursorIcon::Move;
        for handle in fractal_handles.iter() {
          let material = fractal_materials
            .get_mut(handle)
            .expect("Fractal material not found");
          material.translate(mouse_state.position - event.position);
        }
      } else {
        window.cursor.icon = CursorIcon::Default;
      }
      mouse_state.position = event.position;
    }
  }
}

/// Updates the fractal material with the current time and aspect ratio. Scales
/// the screen covering triangle on the way.
pub fn update_fractal_material<M>(
  mut fractals: Query<(&Handle<M>, &mut Transform)>,
  mut fractals_materials: ResMut<Assets<M>>,
  time: Res<Time>,
  mut resize_reader: EventReader<WindowResized>,
) where
  M: FractalMaterial2d,
{
  for (handle, mut transform) in fractals.iter_mut() {
    let material = fractals_materials
      .get_mut(handle)
      .expect("Fractal material not found");

    // Update the time in the material.
    material.set_timer(time.elapsed_seconds());
    for e in resize_reader.read() {
      material.resize_screen(e.width, e.height);
      // Scale the triangle to cover the screen.
      transform.scale = Vec3::new(e.width * 0.5f32, e.height * 0.5f32, 1.0f32);
    }
  }
}
