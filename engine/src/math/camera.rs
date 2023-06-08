use super::{matrix::*, vector::*, axis::Axis};

pub struct Camera {
    projection_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    view_projection_matrix: Matrix4<f32>,
    position: Vector2<f32>,
    world_position: Vector2<f32>,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    rotation: f32
}

impl Camera {
   pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        debug_assert!(left < right && top < bottom);

        let projection_matrix = Matrix4::ortho(left, right, bottom, top, far, near);
        let view_matrix = Matrix4::from(1.0);

        Self {
            view_projection_matrix: &projection_matrix * &view_matrix,
            projection_matrix,
            view_matrix,
            position: Vector2::default(),
            world_position: Vector2::default(),
            left,
            right,
            bottom,
            top,
            rotation: 0.0
        }
    }

    pub fn fov_size(&self) -> Vector2<f32> {
        Vector2::new(self.left.abs() + self.right.abs(), self.bottom.abs() + self.top.abs())
    }

    pub fn bottom_left(&self) -> Vector2<f32> {
        Vector2::new(self.left, self.bottom)
    }

    pub fn projection_matrix(&self) -> &Matrix4<f32> {
        &self.projection_matrix
    }

    pub fn view_matrix(&self) -> &Matrix4<f32> {
        &self.view_matrix
    }

    pub fn view_projection_matrix(&self) -> &Matrix4<f32>  {
        &self.view_projection_matrix
    }

    pub fn set_projection(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        self.projection_matrix = Matrix4::ortho(left, right, bottom, top, far, near);
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;
        self.recalculate_view_matrix();
    }

    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.world_position = position;
        self.position = position.double() / (self.fov_size() * Vector2::new(1.0, -1.0));
        self.recalculate_view_matrix();
    } 

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.recalculate_view_matrix();
    }

    pub fn in_fov(&mut self, point: Vector2<f32>) -> bool {
        point > Vector2::new(self.left, self.top) + self.position && point < Vector2::new(self.right, self.bottom) + self.position
    }

    pub fn to_world(&mut self, point: Vector2<f32>, range: Vector2<f32>) -> Vector2<f32> {
        point / range * self.fov_size() + Vector2::new(self.left, self.top) + self.world_position
    }

    fn recalculate_view_matrix(&mut self) {
        let transform = Matrix4::from(1.0_f32).translate(&self.position) * Matrix4::from(1.0_f32).rotate(self.rotation, Axis::Z);
        self.view_matrix = Matrix4::inverse(&transform);
        self.view_projection_matrix = &self.projection_matrix * &self.view_matrix;
    }
}
