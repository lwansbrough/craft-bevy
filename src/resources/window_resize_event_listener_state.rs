use bevy::{
    prelude::*,
    window::{CreateWindow, WindowResized}
};
use crate::events::*;
use crate::models::*;

#[derive(Default)]
pub struct WindowResizeEventListenerState {
    pub resolution: Option<[f32; 2]>
}
