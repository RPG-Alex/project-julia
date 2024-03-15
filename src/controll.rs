use bevy::{
  ecs::{
    event::EventReader,
    system::{Query, ResMut},
  },
  input::{
    mouse::{MouseButton, MouseMotion, MouseWheel},
    ButtonInput,
  },
  window::{CursorIcon, Window},
};

use crate::sets::julia::PostProcessSettings;

#[allow(dead_code, unused_variables)]
pub fn zoom_with_mouse_wheel(
  mut scroll_events: EventReader<MouseWheel>,
  mut settings: Query<&mut PostProcessSettings>,
)
{
  for mut settings in settings.iter_mut() {
    for event in scroll_events.read() {
      match event.y {
        // Positive y value means scrolling up (zoom in)
        // Negative y value means scrolling down (zoom out)
        _ if event.y > 0.0 => {
          settings.view.w *= 0.9;
          settings.view.z *= 0.9;
        }, // Zoom in
        _ => {
          settings.view.w *= 1.1;
          settings.view.z *= 1.1
        }, // Zoom out
      }
    }
  }
}

pub fn click_and_drag_with_mouse(
  mut mouse_event: EventReader<MouseMotion>,
  mut settings: Query<&mut PostProcessSettings>,
  mut windows: Query<&mut Window>,
  mouse_click: ResMut<ButtonInput<MouseButton>>,
)
{
  if let Some(mut window) = windows.iter_mut().next() {
    if mouse_click.pressed(MouseButton::Left) {
      window.cursor.icon = CursorIcon::Move;
      for event in mouse_event.read() {
        for mut setting in settings.iter_mut() {
          setting.view.x += -event.delta.x * 0.003 * setting.view.z;
          setting.view.y += event.delta.y * 0.003 * setting.view.z;
        }
      }
    } else {
      window.cursor.icon = CursorIcon::Default;
    }
  }
}
