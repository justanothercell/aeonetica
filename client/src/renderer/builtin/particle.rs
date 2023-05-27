use std::rc::Rc;

use aeonetica_engine::math::vector::Vector2;

use crate::renderer::material::Material;

pub struct ParticleEmitter<M: Material> {
    position: Vector2<f32>,
    lifetime: f32,
    particles: Vec<Particle>,

    material: Rc<M>,
    params: M::Data<4>
}

pub struct Particle {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    lifetime: f32
}

