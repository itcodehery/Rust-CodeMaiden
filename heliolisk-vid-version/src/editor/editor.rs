mod editor;

// Editor struct
pub struct Editor {
    buffers: Vec<Document>,
    current_focused_idx: usize,
    is_quittable: bool,
}
