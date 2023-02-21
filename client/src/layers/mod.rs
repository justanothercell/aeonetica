use crate::events::Event;

pub trait Layer {
    fn on_attach(&self) where Self: Sized; // run on layer creation
    fn on_detach(&self) where Self: Sized; // run on layer deletion
    
    fn on_update(&self) where Self: Sized; // run on every client update
    fn on_event<E>(&self, event: &mut Event) where Self: Sized;

    fn name(&self) -> &'static str 
        where Self: Sized {
        "Layer"
    }
}
