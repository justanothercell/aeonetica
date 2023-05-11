use aeonetica_engine::util::id_map::IdMap;

use crate::{client_runtime::ClientHandleBox, data_store::DataStore};
use crate::renderer::context::LayerHandles;

use super::window::events::Event;

pub trait Layer {
    fn instantiate() -> Self where Self: Sized;

    fn on_attach(&self); // run on layer creation
    fn on_detach(&self); // run on layer deletion
    
    fn on_update(&self, store: &mut DataStore, handles: LayerHandles, delta_time: f64); // run on every client update
    fn on_event(&self, handles: &mut IdMap<ClientHandleBox>, event: &Event) -> bool;

    fn active(&self) -> bool { true }

    fn name(&self) -> &'static str {
        "Layer"
    }

    fn is_overlay(&self) -> bool {
        false
    }
}
