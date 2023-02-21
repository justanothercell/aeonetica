use super::*;

pub struct MouseMovedEvent {
    handled: bool,
    x: f32,
    y: f32
}

impl Event for MouseMovedEvent {
    const NAME: &'static str = "MouseMoved";

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self) {
        self.handled = true;
    }
}

impl MouseMovedEvent {
    fn new(x: f32, y: f32) -> Self {
        Self {
            handled: false,
            x,
            y
        }
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

pub struct MouseScrolledEvent {
    handled: bool,
    x_offset: f32,
    y_offset: f32
}

impl Event for MouseScrolledEvent {
    const NAME: &'static str = "MouseScrolled";

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self) {
        self.handled = true;
    }
}

impl MouseScrolledEvent {
    fn new(x_offset: f32, y_offset: f32) -> Self {
        Self {
            handled: false,
            x_offset,
            y_offset
        }
    }

    fn x_offset(&self) -> f32 {
        self.x_offset
    }

    fn y_offset(&self) -> f32 {
        self.y_offset
    }
}

#[derive(Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle
}

pub trait MouseButtonEvent {
    fn mouse_button(&self) -> MouseButton;   
}

pub struct MouseButtonPressedEvent {
    handled: bool,
    mouse_button: MouseButton
}

impl Event for MouseButtonPressedEvent {
    const NAME: &'static str = "MouseButtonPressed";

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self) {
        self.handled = true;
    }
}

impl MouseButtonEvent for MouseButtonPressedEvent {
    fn mouse_button(&self) -> MouseButton {
        self.mouse_button
    }
}

impl MouseButtonPressedEvent {
    pub fn new(mouse_button: MouseButton) -> Self {
        Self {
            handled: false,
            mouse_button
        }
    }
}

pub struct MouseButtonReleasedEvent {
    handled: bool,
    mouse_button: MouseButton
}

impl Event for MouseButtonReleasedEvent {
    const NAME: &'static str = "MouseButtonReleased";

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self) {
        self.handled = true;
    }
}

impl MouseButtonEvent for MouseButtonReleasedEvent {
    fn mouse_button(&self) -> MouseButton {
        self.mouse_button
    }
}

impl MouseButtonReleasedEvent {
    pub fn new(mouse_button: MouseButton) -> Self {
        Self {
            handled: false,
            mouse_button
        }
    }
}
