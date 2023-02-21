pub type KeyCode = u32;

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16)
}

#[derive(Debug)]
pub enum EventType {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    MouseScrolled(f32, f32),
    MouseMoved(f32, f32),
    WindowClose(),
    WindowResize(u32, u32),

    Unknown()
}

#[derive(Debug)]
pub struct Event {
    event_type: EventType,
    handled: bool
}

impl From<EventType> for Event {
    fn from(value: EventType) -> Self {
        Self::new(value)
    }
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Self {
            event_type,
            handled: false
        }
    }

    pub fn typ(&mut self) -> &EventType {
        &self.event_type
    } 

    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self) {
        self.handled = true;
    }
}