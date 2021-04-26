use bevy::{app::Events, prelude::*, window::{CreateWindow, WindowResized}};
use crate::resources::WindowResizeEventListenerState;

pub fn window_resolution_system(
    mut state: ResMut<WindowResizeEventListenerState>,
    mut create_events: EventReader<CreateWindow>,
    mut resize_events: EventReader<WindowResized>
) {
    for event in create_events.iter() {
        state.resolution = Some([event.descriptor.width, event.descriptor.height]);
    }
    
    for event in resize_events.iter() {
        state.resolution = Some([event.width, event.height]);
    }
}
