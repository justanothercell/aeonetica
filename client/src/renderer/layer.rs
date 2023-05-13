use aeonetica_engine::math::camera::Camera;
use aeonetica_engine::util::id_map::IdMap;

use crate::{client_runtime::ClientHandleBox, data_store::DataStore};
use crate::renderer::Renderer;
use crate::renderer::window::events::Event;

pub trait Layer {
    fn instantiate_camera(&self) -> Camera;

    fn attach(&self) {} // run on layer creation
    fn quit(&self) {} // run on layer deletion
    #[allow(unused_variables)]
    fn update_camera(&self, store: &mut DataStore, camera: &mut Camera, delta_time: f64) {}
    #[allow(unused_variables)]
    fn pre_handles_update(&mut self, store: &mut DataStore, renderer: &mut Renderer, delta_time: f64) {}
    #[allow(unused_variables)]
    fn post_handles_update(&mut self, store: &mut DataStore, renderer: &mut Renderer, delta_time: f64) {}
    #[allow(unused_variables)]
    fn event(&self, handles: &mut IdMap<ClientHandleBox>, event: &Event) -> bool { false }

    fn active(&self) -> bool { true }
    fn name(&self) -> &'static str { "Layer" }
    fn is_overlay(&self) -> bool { false }
}
