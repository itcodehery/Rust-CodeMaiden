mod editor;

// Editor struct
pub struct Editor {
    buffers: Vec<Document>,
    current_focused_idx: usize,
    is_quittable: bool,
}

impl Editor {
    pub fn new(buffers: Vec<Document>) -> Self {
        Self {
            buffers,
            current_focused_idx: 0,
            is_quittable: true,
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
