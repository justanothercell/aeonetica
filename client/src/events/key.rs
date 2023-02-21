use super::*;

pub type KeyCode = i32;

pub trait KeyEvent {
    fn key_code(&self) -> KeyCode;
    fn is_alphanumeric_key(&self);
}

pub struct KeyPressedEvent {
    handled: bool,
    key_code: KeyCode,
    repeat_count: i32,
}

impl Event for KeyPressedEvent {
    const NAME: &'static str = "KeyPressed";

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self) {
        self.handled = true
    }
}

impl KeyEvent for KeyPressedEvent {
    fn is_alphanumeric_key(&self) {
        todo!()
    }

    fn key_code(&self) -> KeyCode {
        self.key_code
    }
}

impl KeyPressedEvent {
    pub fn new(key_code: KeyCode, repeat_count: i32) -> Self {
        Self {
            handled: false,
            key_code,
            repeat_count
        }
    }

    pub fn repeat_count(&self) -> i32 {
        self.repeat_count
    }
}

pub struct KeyReleasedEvent {
    handled: bool,
    key_code: KeyCode,
}

impl Event for KeyReleasedEvent {
    const NAME: &'static str = "KeyReleased";

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self) {
        self.handled = true
    }
}

impl KeyEvent for KeyReleasedEvent {
    fn is_alphanumeric_key(&self) {
        todo!()
    }

    fn key_code(&self) -> KeyCode {
        self.key_code
    }
}

impl KeyReleasedEvent {
    pub fn new(key_code: KeyCode) -> Self {
        Self {
            handled: false,
            key_code,
        }
    }
}
