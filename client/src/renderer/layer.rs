use aeonetica_engine::math::camera::Camera;

use crate::data_store::DataStore;
use crate::renderer::Renderer;
use crate::renderer::window::events::Event;

#[allow(unused_variables)]
pub trait Layer {
    fn instantiate_camera(&self) -> Camera;

    fn attach(&mut self, renderer: &mut Renderer) {} // run on layer creation
    fn quit(&mut self, renderer: &mut Renderer) {} // run on layer deletion

    fn update_camera(&mut self, store: &mut DataStore, camera: &mut Camera, delta_time: f64) {}
    fn pre_handles_update(&mut self, store: &mut DataStore, renderer: &mut Renderer, delta_time: f64) {}
    fn post_handles_update(&mut self, store: &mut DataStore, renderer: &mut Renderer, delta_time: f64) {}

    fn event(&mut self, event: &Event) -> bool { false } // run on window event

    fn active(&self) -> bool { true }
    fn name(&self) -> &'static str { "Layer" }
    fn is_overlay(&self) -> bool { false }
}
