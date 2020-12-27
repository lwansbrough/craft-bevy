use serde::{Serialize, Deserialize};
use std::marker::{PhantomData};
use bevy::ecs::{Command, Bundle, Entity, Resources, World};

use crate::models::*;

pub struct Synchronize;

pub trait Synchronizable : 'static + Send + Sync + Sized {
    fn type_id() -> u8;
    fn instance_type_id(&self) -> u8 { Self::type_id() }
    fn spawn(world: &mut World, resources: &mut Resources, entity: Entity);
    fn author_serialized_state(&self, resources: &mut Resources) -> Vec<u8>;
    fn consume_serialized_state(&mut self, state: &Vec<u8>, resources: &mut Resources);
}

pub struct SynchronizableStateAuthoring<TComponent> {
    entity: Entity,
    frame: u32,
    _m: PhantomData<TComponent>
}

impl<TComponent> Command for SynchronizableStateAuthoring<TComponent> where TComponent: Synchronizable {
    fn write(self: Box<Self>, world: &mut World, resources: &mut Resources) {
        let component = world.get::<TComponent>(self.entity).unwrap();
        let component_type_id = component.instance_type_id();
        let serialized_state = component.author_serialized_state(resources);

        let mut synchronized = world.get_mut::<Synchronized<TComponent>>(self.entity).unwrap();

        synchronized.state_frames().push(StateFrame {
            entity_id: self.entity.id(),
            component_type_id,
            frame: self.frame,
            state: serialized_state
        })
    }
}

pub struct SynchronizableStateConsumption<TComponent> {
    entity: Entity,
    state_frame: StateFrame,
    _m: PhantomData<TComponent>
}

impl<TComponent> Command for SynchronizableStateConsumption<TComponent> where TComponent: Synchronizable {
    fn write(self: Box<Self>, world: &mut World, resources: &mut Resources) {

        if let Err(_) = world.get_mut::<Synchronized<TComponent>>(self.entity) {
            return;
        }

        if let Ok(mut component) = world.get_mut::<TComponent>(self.entity) {
            component.consume_serialized_state(&self.state_frame.state, resources);
        } else {
            TComponent::spawn(world, resources, self.entity);
            let mut component = world.get_mut::<TComponent>(self.entity).unwrap();
            component.consume_serialized_state(&self.state_frame.state, resources);
        }

        if let Ok(mut synchronized) = world.get_mut::<Synchronized<TComponent>>(self.entity) {
            synchronized.state_frames().push(StateFrame {
                entity_id: self.entity.id(),
                component_type_id: self.state_frame.component_type_id,
                state: self.state_frame.state,
                frame: self.state_frame.frame
            });
        }
    }
}

pub struct Synchronized<TComponent> {
    command_frame_buffer: CommandFrameBuffer,
    state_frame_buffer: StateFrameBuffer,
    _m: PhantomData<TComponent>
}

impl<TComponent> Default for Synchronized<TComponent> where TComponent: Synchronizable {
    fn default() -> Self {
        Self {
            command_frame_buffer: CommandFrameBuffer::default(),
            state_frame_buffer: StateFrameBuffer::default(),
            _m: PhantomData
        }
    }
}

impl<TComponent> Synchronized<TComponent> where TComponent: Synchronizable {
    pub fn command_frames(&mut self) -> &mut CommandFrameBuffer {
        &mut self.command_frame_buffer
    }

    pub fn state_frames(&mut self) -> &mut StateFrameBuffer {
        &mut self.state_frame_buffer
    }

    pub fn author_state_command(entity: Entity, frame: u32) -> SynchronizableStateAuthoring::<TComponent> {
        SynchronizableStateAuthoring::<TComponent> {
            entity,
            frame,
            _m: PhantomData
        }
    }

    pub fn consume_state_command(entity: Entity, state_frame: StateFrame) -> SynchronizableStateConsumption::<TComponent> {
        SynchronizableStateConsumption::<TComponent> {
            entity,
            state_frame,
            _m: PhantomData
        }
    }
}
