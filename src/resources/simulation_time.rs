use bevy::prelude::*;
use std::time::{Duration, Instant};

pub struct SimulationTime {
    frame: u32,
    last_execution: Instant,
    initial_frame_duration: f32,
    current_frame_duration: f32
}

impl SimulationTime {
    pub fn new(speed: u16) -> SimulationTime {
        let frame_duration = (1000.0 / (speed as f32));

        SimulationTime {
            frame: 0,
            last_execution: Instant::now(),
            initial_frame_duration: frame_duration,
            current_frame_duration: frame_duration
        }
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.frame = frame;
    }

    pub fn frame(&self) -> u32 {
        self.frame
    }

    pub fn frame_duration(&self) -> f32 {
        self.current_frame_duration
    }

    pub fn last_execution(&self) -> Instant {
        self.last_execution
    }

    pub fn can_tick(&self) -> bool {
        self.last_execution.elapsed() >= Duration::from_millis(self.current_frame_duration as u64)
    }

    pub fn tick(&mut self) {
        self.frame += 1;
        self.last_execution = Instant::now()
    }

    pub fn adjust_speed(&mut self, speed: u16) {
        self.current_frame_duration = (1000.0 / (speed as f32));
    }
}

pub fn simulation_time_system(time: Res<Time>, mut sim_time: ResMut<SimulationTime>) {
    if sim_time.can_tick() {
        sim_time.tick()
    }
}
