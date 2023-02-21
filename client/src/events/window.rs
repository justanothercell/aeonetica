use super::*;

pub struct WindowCloseEvent {
    handled: bool
}

impl Event for WindowCloseEvent {
    const NAME: &'static str = "WindowClose";

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self) {
        self.handled = true;
    }
}

impl WindowCloseEvent {
    pub fn new() -> Self {
        Self {
            handled: false
        }
    }
}

pub struct WindowResizeEvent {
    handled: bool,
    width: u32,
    height: u32
}

impl Event for WindowResizeEvent {
    const NAME: &'static str = "WindowResize";

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self) {
        self.handled = true;
    }
}

impl WindowResizeEvent {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            handled: false,
            width,
            height
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
