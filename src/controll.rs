use bevy::{
  ecs::{
    event::EventReader,
    system::{Query, Res, ResMut, Resource},
  },
  input::{
    mouse::{MouseButton, MouseWheel},
    ButtonInput,
  },
  math::{Vec2, Vec4},
  window::{CursorIcon, CursorMoved, Window},
};

use crate::sets::julia::PostProcessSettings;

#[derive(Resource)]
pub struct MouseState
{
  pub position: Vec2,
}

pub fn zoom_with_mouse_wheel(
  mut scroll_events: EventReader<MouseWheel>,
  mut settings: Query<&mut PostProcessSettings>,
)
{
  for mut settings in settings.iter_mut() {
    for event in scroll_events.read() {
      if event.y > 0.0 {
        settings.view.w *= 0.9;
        settings.view.z *= 0.9;
        settings.max_iter += 3;
      } else {
        settings.view.w *= 1.1;
        settings.view.z *= 1.1;
        settings.max_iter -= 3;
        settings.max_iter = settings.max_iter.max(100);
      } // Zoom out
    }
  }
}

fn screen_position_to_complex(position: Vec2, screen: Vec2, view: Vec4) -> Vec2
{
  Vec2::new(
    (position.x - screen.x / 2.0) / screen.x * view.z + view.x,
    (screen.y / 2.0 - position.y) / screen.y * view.w + view.y,
  )
}

pub fn click_and_drag_with_mouse(
  mut mouse_event: EventReader<CursorMoved>,
  mut settings: Query<&mut PostProcessSettings>,
  mut windows: Query<&mut Window>,
  mouse_click: Res<ButtonInput<MouseButton>>,
  mut mouse_state: ResMut<MouseState>,
)
{
  if let Some(mut window) = windows.iter_mut().next() {
    for event in mouse_event.read() {
      if mouse_click.pressed(MouseButton::Left) {
        window.cursor.icon = CursorIcon::Move;
        for mut setting in settings.iter_mut() {
          let complex_target =
            screen_position_to_complex(mouse_state.position, setting.screen, setting.view);
          let current_mouse_complex =
            screen_position_to_complex(event.position, setting.screen, setting.view);
          let complex_shift = complex_target - current_mouse_complex;
          setting.view.x += complex_shift.x;
          setting.view.y += complex_shift.y;
        }
      } else {
        window.cursor.icon = CursorIcon::Default;
      }
      mouse_state.position = event.position;
    }
  }
}
