use aeonetica_engine::math::vector::Vector2;

extern crate glfw;

pub use glfw::Key as KeyCode;

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
pub enum Event {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    MouseScrolled(Vector2<f32>),
    MouseMoved(Vector2<f32>),
    WindowClose(),
    WindowResize(Vector2<i32>),

    Unknown()
}

impl Event {
    pub(super) fn from_glfw(glfw_event: glfw::WindowEvent) -> Self {
        match glfw_event {
            glfw::WindowEvent::FramebufferSize(x, y) => Self::WindowResize(Vector2::new(x, y)),
            glfw::WindowEvent::Close => Self::WindowClose(),
            glfw::WindowEvent::Key(key, _, action, _) => match action {
                glfw::Action::Release => Self::KeyReleased(key),
                glfw::Action::Press => Self::KeyPressed(key),
                _ => Self::Unknown()
            }
            glfw::WindowEvent::MouseButton(button, action, _) => match action {
                glfw::Action::Press => Self::MouseButtonPressed(button.into()),
                glfw::Action::Release => Self::MouseButtonReleased(button.into()),
                _ => Self::Unknown()
            }
            glfw::WindowEvent::Scroll(v, h) => Self::MouseScrolled(Vector2::new(v as f32, h as f32)),
            glfw::WindowEvent::CursorPos(x, y) => Self::MouseMoved(Vector2::new(x as f32, y as f32)),
            _ => Self::Unknown()
        }
    }
}