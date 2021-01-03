use bevy::{
    prelude::*,
    window::{CreateWindow, WindowResized}
};
use crate::resources::WindowResizeEventListenerState;

pub fn window_resolution_system(
    mut state: ResMut<WindowResizeEventListenerState>,
    mut create_events: ResMut<Events<CreateWindow>>,
    mut resize_events: ResMut<Events<WindowResized>>
) {
    for event in state.create_events.iter(&create_events) {
        state.resolution = Some([event.descriptor.width, event.descriptor.height]);
    }
    
    for event in state.resize_events.iter(&resize_events) {
        state.resolution = Some([event.width, event.height]);
    }
}
