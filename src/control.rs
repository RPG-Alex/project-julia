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

use crate::traits::FractalMaterial2d;

#[derive(Resource)]
pub struct MouseState
{
  pub position: Vec2,
}

pub fn zoom_with_mouse_wheel<M>(
  mut scroll_events: EventReader<MouseWheel>,
  julia_handles: Query<&Handle<M>>,
  mut julia_materials: ResMut<Assets<M>>,
) where
  M: FractalMaterial2d,
{
  for handle in julia_handles.iter() {
    let material = julia_materials
      .get_mut(handle)
      .expect("Julia material not found");
    for event in scroll_events.read() {
      if event.y > 0.0 {
        material.zoom_in();
      } else {
        material.zoom_out();
      }
    }
  }
}

pub fn click_and_drag_with_mouse<M>(
  mut mouse_event: EventReader<CursorMoved>,
  julia_handles: Query<&Handle<M>>,
  mut windows: Query<&mut Window>,
  mouse_click: Res<ButtonInput<MouseButton>>,
  mut mouse_state: ResMut<MouseState>,
  mut julia_materials: ResMut<Assets<M>>,
) where
  M: FractalMaterial2d,
{
  if let Some(mut window) = windows.iter_mut().next() {
    for event in mouse_event.read() {
      if mouse_click.pressed(MouseButton::Left) {
        window.cursor.icon = CursorIcon::Move;
        for handle in julia_handles.iter() {
          let material = julia_materials
            .get_mut(handle)
            .expect("Julia material not found");
          material.translate(mouse_state.position - event.position);
          // let complex_target =
          //   screen_position_to_complex(mouse_state.position, material.screen,
          // material.view); let current_mouse_complex =
          //   screen_position_to_complex(event.position, material.screen,
          // material.view); let complex_shift = complex_target -
          // current_mouse_complex; material.view.x +=
          // complex_shift.x; material.view.y += complex_shift.y;
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
  mut julias: Query<(&Handle<M>, &mut Transform)>,
  mut julia_materials: ResMut<Assets<M>>,
  time: Res<Time>,
  mut resize_reader: EventReader<WindowResized>,
) where
  M: FractalMaterial2d,
{
  for (handle, mut transform) in julias.iter_mut() {
    let material = julia_materials
      .get_mut(handle)
      .expect("Julia material not found");

    // Update the time in the material.
    material.set_timer(time.elapsed_seconds());
    for e in resize_reader.read() {
      material.resize_screen(e.width, e.height);
      // Scale the triangle to cover the screen.
      transform.scale = Vec3::new(e.width * 0.5f32, e.height * 0.5f32, 1.0f32);
    }
  }
}
