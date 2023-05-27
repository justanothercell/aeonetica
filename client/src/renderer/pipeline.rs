use aeonetica_engine::math::camera::Camera;

use super::{Renderer, layer::LayerUpdater, util::Target};

pub trait Pipeline {
    fn pipeline(&mut self, renderer: &mut Renderer, camera: &Camera, target: &Target, updater: LayerUpdater, delta_time: f64);
}

pub(super) struct DefaultPipeline;

impl DefaultPipeline {
    pub(super) fn new() -> Self {
        Self
    }
}

impl Pipeline for DefaultPipeline {
    fn pipeline(&mut self, renderer: &mut Renderer, camera: &Camera, target: &Target, updater: LayerUpdater, delta_time: f64) {
        renderer.begin_scene(camera);
        updater.update(renderer, delta_time);
        renderer.draw_vertices(target);
        renderer.end_scene();
    }
}
