use std::sync::mpsc::Receiver;

pub struct InputEvent {
    pub active: bool,
    pub event_type: EventType
}

pub enum EventType {
    Right,
    Down,
    Left,
    Up,
    A,
    B,
    Start,
    Select,
    Quit,
    Save,
    Load
}

pub struct InputSource {
    src: Receiver<InputEvent>
}

impl InputSource {
    pub fn new(src: Receiver<InputEvent>) -> Self {
        Self { src: src }
    }
    pub fn get_input_event(&self) -> Option<InputEvent> {
        self.src.try_recv().ok()
    }
}
