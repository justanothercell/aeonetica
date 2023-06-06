use aeonetica_engine::{math::camera::Camera, time::Time};

use super::{Renderer, layer::LayerUpdater, util::Target};

pub trait Pipeline {
    fn pipeline(&mut self, renderer: &mut Renderer, camera: &Camera, target: &Target, updater: LayerUpdater, time: Time);
}

pub(super) struct DefaultPipeline;

impl DefaultPipeline {
    pub(super) fn new() -> Self {
        Self
    }
}

impl Pipeline for DefaultPipeline {
    fn pipeline(&mut self, renderer: &mut Renderer, camera: &Camera, target: &Target, mut updater: LayerUpdater, time: Time) {
        renderer.begin_scene(camera);
        updater.update(renderer, time);
        renderer.draw_vertices(target);
        renderer.end_scene();
    }
}
