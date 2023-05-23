use aeonetica_engine::math::camera::Camera;

use super::{Renderer, buffer::framebuffer::FrameBuffer, layer::LayerUpdater};

pub type Pipeline = fn(renderer: &mut Renderer, camera: &Camera, target: &FrameBuffer, updater: LayerUpdater, deltat_time: f64);

pub fn default_pipeline(renderer: &mut Renderer, camera: &Camera, target: &FrameBuffer, updater: LayerUpdater, delta_time: f64) {
    renderer.begin_scene(camera);
    updater.update(renderer, delta_time);
    renderer.draw_vertices(target);
    renderer.end_scene();
}
