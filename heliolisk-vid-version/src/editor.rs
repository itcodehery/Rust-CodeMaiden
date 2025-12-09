use crate::Document;
use std::marker::PhantomData;

pub struct NavigateMode;
pub struct EditMode;
pub struct SelectMode;
pub struct CommandMode;

// Editor struct
pub struct Editor<State = NavigateMode> {
    buffers: Vec<Document>,
    current_focused_idx: usize,
    is_quittable: bool,
    state: PhantomData<State>,
}

impl Editor {
    pub fn new(buffers: Vec<Document>) -> Self {
        Self {
            buffers,
            current_focused_idx: 0,
            is_quittable: true,
            state: PhantomData::<NavigateMode>,
        }
    }

    pub fn buffer_switch_forward(&mut self) {
        if !(self.buffers.len() < 2) {
            if self.current_focused_idx + 1 == self.buffers.len() {
                self.current_focused_idx = 0;
            } else {
                self.current_focused_idx += 1;
            }
        }
    }

    pub fn buffer_switch_backward(&mut self) {
        if self.buffers.len() < 2 {
            return;
        } else {
            if self.current_focused_idx == 0 {
                self.current_focused_idx = self.buffers.len();
            } else {
                self.current_focused_idx -= 1;
            }
        }
    }
}
