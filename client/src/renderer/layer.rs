use super::window::events::Event;

pub trait Layer {
    fn instantiate() -> Self where Self: Sized;

    fn on_attach(&self); // run on layer creation
    fn on_detach(&self); // run on layer deletion
    
    fn on_update(&self, delta_time: f64); // run on every client update
    fn on_event(&self, event: &Event) -> bool;

    fn name(&self) -> &'static str {
        "Layer"
    }

    fn is_overlay(&self) -> bool {
        false
    }
}
