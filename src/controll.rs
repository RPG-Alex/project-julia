use bevy::{
    ecs::{event::EventReader, system::Query},
    input::mouse::MouseWheel,
};

use crate::sets::julia::PostProcessSettings;

#[allow(dead_code, unused_variables)]
pub fn zoom_with_mouse_wheel(
    mut scroll_events: EventReader<MouseWheel>,
    mut settings: Query<&mut PostProcessSettings>, 
) {
    for mut settings in settings.iter_mut() {
        for event in scroll_events.read() {
            match event.y {
                // Positive y value means scrolling up (zoom in)
                // Negative y value means scrolling down (zoom out)
                _ if event.y > 0.0 => {
                    settings.view.w = settings.view.w * 0.9;
                    settings.view.z = settings.view.z * 0.9;
                } // Zoom in
                _ => {
                    settings.view.w = settings.view.w * 1.1;
                    settings.view.z = settings.view.z * 1.1
                } // Zoom out
            }
        }
    }
}
