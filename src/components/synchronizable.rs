use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use std::marker::{PhantomData};

use crate::models::*;

#[derive(Serialize, Deserialize)]
pub struct Synchronizable<TComponent> {
    #[serde(skip)]
    inputs: VecDeque<InputFrame>,
    #[serde(skip)]
    predictions: VecDeque<StateFrame>,
    #[serde(skip)]
    server_truth: Option<StateFrame>,
    _m: PhantomData<TComponent>
}

impl<TComponent> Default for Synchronizable<TComponent> where TComponent: 'static + Send + Sync {
    fn default() -> Self {
        Self {
            inputs:  VecDeque::<InputFrame>::default(),
            predictions: VecDeque::<StateFrame>::default(),
            server_truth: None,
            _m: PhantomData
        }
    }
}

impl<TComponent> Synchronizable<TComponent> where TComponent: 'static + Send + Sync {
    pub fn inputs(&self) -> &VecDeque<InputFrame> {
        &self.inputs
    }

    pub fn inputs_mut(&mut self) -> &mut VecDeque<InputFrame> {
        &mut self.inputs
    }

    pub fn predictions(&self) -> &VecDeque<StateFrame> {
        &self.predictions
    }

    pub fn predictions_mut(&mut self) -> &mut VecDeque<StateFrame> {
        &mut self.predictions
    }

    pub fn server_truth(&self) -> Option<&StateFrame> {
        (&self.server_truth).as_ref()
    }

    pub fn set_server_truth(&mut self, frame: StateFrame) {
        self.server_truth = Some(frame);
    }

    pub fn clear_server_truth(&mut self) {
        self.server_truth = None;
    }

    pub fn input_frame(&mut self, frame: u32) -> Option<InputFrame> {
        let mut inputs = self.inputs_mut();
        let mut input = None;

        while !inputs.is_empty() {
            input = match inputs.front() {
                Some(front) => {
                    if front.frame <= frame {
                        inputs.pop_front()
                    } else {
                        break
                    }
                },
                None => None
            }
        }

        input
    }
}