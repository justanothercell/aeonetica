extern crate glfw;

pub type KeyCode = i32;

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other
}

impl From<glfw::MouseButton> for MouseButton {
    fn from(button: glfw::MouseButton) -> Self {
        match button {
            glfw::MouseButtonLeft => Self::Left,
            glfw::MouseButtonMiddle => Self::Middle,
            glfw::MouseButtonRight => Self::Right,
            _ => Self::Other
        }
    }
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
    WindowResize(i32, i32),
    WindowMoved(i32, i32),

    Unknown()
}

impl From<glfw::WindowEvent> for EventType {
    fn from(event: glfw::WindowEvent) -> Self {
        match event {
            glfw::WindowEvent::Pos(x, y) => Self::WindowMoved(x, y),
            glfw::WindowEvent::FramebufferSize(x, y) => Self::WindowResize(x, y),
            glfw::WindowEvent::Close => Self::WindowClose(),
            glfw::WindowEvent::Key(_, scancode, action, _) => match action {
                glfw::Action::Release => Self::KeyReleased(scancode),
                glfw::Action::Press => Self::KeyPressed(scancode),
                _ => Self::Unknown()
            }
            glfw::WindowEvent::MouseButton(button, action, _) => match action {
                glfw::Action::Press => Self::MouseButtonPressed(button.into()),
                glfw::Action::Release => Self::MouseButtonReleased(button.into()),
                _ => Self::Unknown()
            }
            glfw::WindowEvent::Scroll(v, h) => Self::MouseScrolled(v as f32, h as f32),
            glfw::WindowEvent::CursorPos(x, y) => Self::MouseMoved(x as f32, y as f32),
            _ => Self::Unknown()
        }
    }
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

    pub fn typ(&self) -> &EventType {
        &self.event_type
    } 

    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self) {
        self.handled = true;
    }

    pub(super) fn from_glfw(event: glfw::WindowEvent) -> Self {
        Self::new(event.into())
    }
}
