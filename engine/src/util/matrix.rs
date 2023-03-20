use std::array;
use std::ops::*;

use super::axis::Axis;
use super::vector::Vector2;

#[derive(Clone, Debug, Default)]
pub struct Matrix4<T>([T; 16]);

impl<T> Matrix4<T> {
    pub fn new(data: [T; 16]) -> Self {
        Self(data)
    }

    pub fn fill(value: T) -> Self
    where
        T: Clone,
    {
        Self(array::from_fn(|_| value.clone()))
    }

    pub unsafe fn value_ptr(&self) -> *const T {
        self.0.as_ptr()
    }
}

impl<T: Clone + Default> From<T> for Matrix4<T> {
    fn from(value: T) -> Self {
        let mut matrix = Self::default();
        matrix.0[0] = value.clone();
        matrix.0[5] = value.clone();
        matrix.0[10] = value.clone();
        matrix.0[15] = value.clone();

        matrix
    }
}

impl Matrix4<f32> {
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, far: f32, near: f32) -> Self {
        let mut m = Matrix4::from(1.0);
        m.0[0] = 2.0 / (right - left);
        m.0[5] = 2.0 / (top - bottom);
        m.0[10] = -1.0;

        m.0[12] = -(right + left) / (right - left);
        m.0[13] = -(top + bottom) / (top - bottom);
        m.0[14] = -(far + near) / (far - near);
        m
    }

    pub fn translate(mut self, pos: &Vector2<f32>) -> Self {
        let t = self.clone();
        self.identity();
        self.0[12] = pos.x();
        self.0[13] = pos.y();
        self.0[14] = 0.0;

        self *= t;
        self
    }

    pub fn identity(&mut self) -> &mut Self {
        self.0.fill(0.0);

        self.0[0] = 1.0;
        self.0[5] = 1.0;
        self.0[10] = 1.0;
        self.0[15] = 1.0;

        self
    }

    pub fn rotate(mut self, radians: f32, axis: Axis) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        match axis {
            Axis::X => {
                self.0[0] = c;
                self.0[2] = -s;
                self.0[8] = s;
                self.0[10] = -c;
            }
            Axis::Y => {
                self.0[0] = c;
                self.0[2] = -s;
                self.0[8] = s;
                self.0[10] = c;
            }
            Axis::Z => {
                self.0[0] = c;
                self.0[1] = -s;
                self.0[4] = s;
                self.0[5] = c;
            }
        }

        self
    }

    pub fn inverse(&self) -> Self {
        let mut inv = Matrix4::default();

        inv.0[0] = self.0[5] * self.0[10] * self.0[15]
            - self.0[5] * self.0[11] * self.0[14]
            - self.0[9] * self.0[6] * self.0[15]
            + self.0[9] * self.0[7] * self.0[14]
            + self.0[13] * self.0[6] * self.0[11]
            - self.0[13] * self.0[7] * self.0[10];

        inv.0[4] = -self.0[4] * self.0[10] * self.0[15]
            + self.0[4] * self.0[11] * self.0[14]
            + self.0[8] * self.0[6] * self.0[15]
            - self.0[8] * self.0[7] * self.0[14]
            - self.0[12] * self.0[6] * self.0[11]
            + self.0[12] * self.0[7] * self.0[10];

        inv.0[8] = self.0[4] * self.0[9] * self.0[15]
            - self.0[4] * self.0[11] * self.0[13]
            - self.0[8] * self.0[5] * self.0[15]
            + self.0[8] * self.0[7] * self.0[13]
            + self.0[12] * self.0[5] * self.0[11]
            - self.0[12] * self.0[7] * self.0[9];

        inv.0[12] = -self.0[4] * self.0[9] * self.0[14]
            + self.0[4] * self.0[10] * self.0[13]
            + self.0[8] * self.0[5] * self.0[14]
            - self.0[8] * self.0[6] * self.0[13]
            - self.0[12] * self.0[5] * self.0[10]
            + self.0[12] * self.0[6] * self.0[9];

        inv.0[1] = -self.0[1] * self.0[10] * self.0[15]
            + self.0[1] * self.0[11] * self.0[14]
            + self.0[9] * self.0[2] * self.0[15]
            - self.0[9] * self.0[3] * self.0[14]
            - self.0[13] * self.0[2] * self.0[11]
            + self.0[13] * self.0[3] * self.0[10];

        inv.0[5] = self.0[0] * self.0[10] * self.0[15]
            - self.0[0] * self.0[11] * self.0[14]
            - self.0[8] * self.0[2] * self.0[15]
            + self.0[8] * self.0[3] * self.0[14]
            + self.0[12] * self.0[2] * self.0[11]
            - self.0[12] * self.0[3] * self.0[10];

        inv.0[9] = -self.0[0] * self.0[9] * self.0[15]
            + self.0[0] * self.0[11] * self.0[13]
            + self.0[8] * self.0[1] * self.0[15]
            - self.0[8] * self.0[3] * self.0[13]
            - self.0[12] * self.0[1] * self.0[11]
            + self.0[12] * self.0[3] * self.0[9];

        inv.0[13] = self.0[0] * self.0[9] * self.0[14]
            - self.0[0] * self.0[10] * self.0[13]
            - self.0[8] * self.0[1] * self.0[14]
            + self.0[8] * self.0[2] * self.0[13]
            + self.0[12] * self.0[1] * self.0[10]
            - self.0[12] * self.0[2] * self.0[9];

        inv.0[2] = self.0[1] * self.0[6] * self.0[15]
            - self.0[1] * self.0[7] * self.0[14]
            - self.0[5] * self.0[2] * self.0[15]
            + self.0[5] * self.0[3] * self.0[14]
            + self.0[13] * self.0[2] * self.0[7]
            - self.0[13] * self.0[3] * self.0[6];

        inv.0[6] = -self.0[0] * self.0[6] * self.0[15]
            + self.0[0] * self.0[7] * self.0[14]
            + self.0[4] * self.0[2] * self.0[15]
            - self.0[4] * self.0[3] * self.0[14]
            - self.0[12] * self.0[2] * self.0[7]
            + self.0[12] * self.0[3] * self.0[6];

        inv.0[10] = self.0[0] * self.0[5] * self.0[15]
            - self.0[0] * self.0[7] * self.0[13]
            - self.0[4] * self.0[1] * self.0[15]
            + self.0[4] * self.0[3] * self.0[13]
            + self.0[12] * self.0[1] * self.0[7]
            - self.0[12] * self.0[3] * self.0[5];

        inv.0[14] = -self.0[0] * self.0[5] * self.0[14]
            + self.0[0] * self.0[6] * self.0[13]
            + self.0[4] * self.0[1] * self.0[14]
            - self.0[4] * self.0[2] * self.0[13]
            - self.0[12] * self.0[1] * self.0[6]
            + self.0[12] * self.0[2] * self.0[5];

        inv.0[3] = -self.0[1] * self.0[6] * self.0[11]
            + self.0[1] * self.0[7] * self.0[10]
            + self.0[5] * self.0[2] * self.0[11]
            - self.0[5] * self.0[3] * self.0[10]
            - self.0[9] * self.0[2] * self.0[7]
            + self.0[9] * self.0[3] * self.0[6];

        inv.0[7] = self.0[0] * self.0[6] * self.0[11]
            - self.0[0] * self.0[7] * self.0[10]
            - self.0[4] * self.0[2] * self.0[11]
            + self.0[4] * self.0[3] * self.0[10]
            + self.0[8] * self.0[2] * self.0[7]
            - self.0[8] * self.0[3] * self.0[6];

        inv.0[11] = -self.0[0] * self.0[5] * self.0[11]
            + self.0[0] * self.0[7] * self.0[9]
            + self.0[4] * self.0[1] * self.0[11]
            - self.0[4] * self.0[3] * self.0[9]
            - self.0[8] * self.0[1] * self.0[7]
            + self.0[8] * self.0[3] * self.0[5];

        inv.0[15] = self.0[0] * self.0[5] * self.0[10]
            - self.0[0] * self.0[6] * self.0[9]
            - self.0[4] * self.0[1] * self.0[10]
            + self.0[4] * self.0[2] * self.0[9]
            + self.0[8] * self.0[1] * self.0[6]
            - self.0[8] * self.0[2] * self.0[5];

        let mut det = self.0[0] * inv.0[0]
            + self.0[1] * inv.0[4]
            + self.0[2] * inv.0[8]
            + self.0[3] * inv.0[12];
        if det == 0.0 {
            return inv;
        }

        det = 1.0 / det;

        for i in 0..16 {
            inv.0[i] = inv.0[i] * det;
        }

        inv
    }
}

impl<T: Copy + Default + Mul<Output = T> + Add<Output = T>> Mul for &Matrix4<T> {
    type Output = Matrix4<T>;

    fn mul(self, rhs: Self) -> Matrix4<T> {
        let mut ret = Matrix4::default();

        ret.0[0] = (self.0[0] * rhs.0[0])
            + (self.0[1] * rhs.0[4])
            + (self.0[2] * rhs.0[8])
            + (self.0[3] * rhs.0[12]);
        ret.0[1] = (self.0[0] * rhs.0[1])
            + (self.0[1] * rhs.0[5])
            + (self.0[2] * rhs.0[9])
            + (self.0[3] * rhs.0[13]);
        ret.0[2] = (self.0[0] * rhs.0[2])
            + (self.0[1] * rhs.0[6])
            + (self.0[2] * rhs.0[10])
            + (self.0[3] * rhs.0[14]);
        ret.0[3] = (self.0[0] * rhs.0[3])
            + (self.0[1] * rhs.0[7])
            + (self.0[2] * rhs.0[11])
            + (self.0[3] * rhs.0[15]);

        ret.0[4] = (self.0[4] * rhs.0[0])
            + (self.0[5] * rhs.0[4])
            + (self.0[6] * rhs.0[8])
            + (self.0[7] * rhs.0[12]);
        ret.0[5] = (self.0[4] * rhs.0[1])
            + (self.0[5] * rhs.0[5])
            + (self.0[6] * rhs.0[9])
            + (self.0[7] * rhs.0[13]);
        ret.0[6] = (self.0[4] * rhs.0[2])
            + (self.0[5] * rhs.0[6])
            + (self.0[6] * rhs.0[10])
            + (self.0[7] * rhs.0[14]);
        ret.0[7] = (self.0[4] * rhs.0[3])
            + (self.0[5] * rhs.0[7])
            + (self.0[6] * rhs.0[11])
            + (self.0[7] * rhs.0[15]);

        ret.0[8] = (self.0[8] * rhs.0[0])
            + (self.0[9] * rhs.0[4])
            + (self.0[10] * rhs.0[8])
            + (self.0[11] * rhs.0[12]);
        ret.0[9] = (self.0[8] * rhs.0[1])
            + (self.0[9] * rhs.0[5])
            + (self.0[10] * rhs.0[9])
            + (self.0[11] * rhs.0[13]);
        ret.0[10] = (self.0[8] * rhs.0[2])
            + (self.0[9] * rhs.0[6])
            + (self.0[10] * rhs.0[10])
            + (self.0[11] * rhs.0[14]);
        ret.0[11] = (self.0[8] * rhs.0[3])
            + (self.0[9] * rhs.0[7])
            + (self.0[10] * rhs.0[11])
            + (self.0[11] * rhs.0[15]);

        ret.0[12] = (self.0[12] * rhs.0[0])
            + (self.0[13] * rhs.0[4])
            + (self.0[14] * rhs.0[8])
            + (self.0[15] * rhs.0[12]);
        ret.0[13] = (self.0[12] * rhs.0[1])
            + (self.0[13] * rhs.0[5])
            + (self.0[14] * rhs.0[9])
            + (self.0[15] * rhs.0[13]);
        ret.0[14] = (self.0[12] * rhs.0[2])
            + (self.0[13] * rhs.0[6])
            + (self.0[14] * rhs.0[10])
            + (self.0[15] * rhs.0[14]);
        ret.0[15] = (self.0[12] * rhs.0[3])
            + (self.0[13] * rhs.0[7])
            + (self.0[14] * rhs.0[11])
            + (self.0[15] * rhs.0[15]);

        ret
    }
}

impl<T: Copy + Default + Mul<Output = T> + Add<Output = T>> Mul for Matrix4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl<T: Copy + Default + Mul<Output = T> + Add<Output = T>> MulAssign for Matrix4<T> {
    fn mul_assign(&mut self, rhs: Self) {
        let tmp = self.clone();
        *self = tmp * rhs;
    }
}
