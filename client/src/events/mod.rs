pub mod key;
pub mod mouse;
pub mod window;

pub trait Event {
    const NAME: &'static str;

    fn handled(&self) -> bool;
    fn set_handled(&mut self);

    fn name() -> &'static str {
        Self::NAME
    }
}

pub type EventFn<T> = fn(&T) -> bool;

pub struct Dispatcher<T> {
    event: T,
}

impl<T> Dispatcher<T> {
    pub fn new(event: T) -> Self
        where T: Event {
        Self {
            event
        }
    }

    pub fn dispatch(&mut self, func: EventFn<T>)
        where T: Event {
        let handled = func(&self.event);
        if handled {
            self.event.set_handled();
        }
    }
}
