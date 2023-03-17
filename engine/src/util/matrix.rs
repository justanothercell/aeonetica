use std::array;
use std::ops::*;

use super::vector::Vector2;

#[derive(Clone, Debug, Default)]
pub struct Matrix4<T>([T; 16]);

impl<T> Matrix4<T> {
    pub fn new(data: [T; 16]) -> Self {
        Self(data)
    }

    pub fn fill(value: T) -> Self 
        where T: Clone {
        Self(array::from_fn(|_| value.clone()))
    }

    pub fn ortho(left: T, right: T, bottom: T, top: T, far: T, near: T) -> Self
        where T: Copy + Default + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Neg<Output = T> + From<f64> {
            let mut m = Matrix4::default();
        	m.0[0] = <f64 as Into<T>>::into(2.0) / (right - left);
	        m.0[5] = <f64 as Into<T>>::into(2.0) / (top - bottom);
	        m.0[10] = (-1.0).into();

	        m.0[12] = -(right + left) / (right - left);
	        m.0[13] = -(top + bottom) / (top - bottom);
	        m.0[14] = -(far + near) / (far - near);
            m
    }

    pub fn identity(&mut self) -> &mut Self
        where T: Clone + Default + From<f64> {
        self.0.fill(T::default());

        self.0[0] = (1.0).into();
        self.0[5] = (1.0).into();
        self.0[10] = (1.0).into();
        self.0[15] = (1.0).into();

        self
    }

    pub fn translate(&mut self, pos: &Vector2<T>) -> &mut Self
        where T: Clone + Copy + Default + From<f64> + Mul<Output = T> + Add<Output = T> {
        let t = self.clone();
        self.identity();
        self.0[12] = pos.x();
        self.0[13] = pos.y();
        self.0[14] = T::default();

        *self *= t;
        self
    }
}

impl<T: Copy + Default + Mul<Output = T> + Add<Output = T>> Mul for Matrix4<T> {
    type Output = Matrix4<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = Matrix4::default();
        
        ret.0[0] = (self.0[0]*rhs.0[0])+(self.0[1]*rhs.0[4])+(self.0[2]*rhs.0[8])+(self.0[3]*rhs.0[12]);
        ret.0[1] = (self.0[0]*rhs.0[1])+(self.0[1]*rhs.0[5])+(self.0[2]*rhs.0[9])+(self.0[3]*rhs.0[13]);
        ret.0[2] = (self.0[0]*rhs.0[2])+(self.0[1]*rhs.0[6])+(self.0[2]*rhs.0[10])+(self.0[3]*rhs.0[14]);
        ret.0[3] = (self.0[0]*rhs.0[3])+(self.0[1]*rhs.0[7])+(self.0[2]*rhs.0[11])+(self.0[3]*rhs.0[15]);
    
        ret.0[4] = (self.0[4]*rhs.0[0])+(self.0[5]*rhs.0[4])+(self.0[6]*rhs.0[8])+(self.0[7]*rhs.0[12]);
        ret.0[5] = (self.0[4]*rhs.0[1])+(self.0[5]*rhs.0[5])+(self.0[6]*rhs.0[9])+(self.0[7]*rhs.0[13]);
        ret.0[6] = (self.0[4]*rhs.0[2])+(self.0[5]*rhs.0[6])+(self.0[6]*rhs.0[10])+(self.0[7]*rhs.0[14]);
        ret.0[7] = (self.0[4]*rhs.0[3])+(self.0[5]*rhs.0[7])+(self.0[6]*rhs.0[11])+(self.0[7]*rhs.0[15]);
    
        ret.0[8] = (self.0[8]*rhs.0[0])+(self.0[9]*rhs.0[4])+(self.0[10]*rhs.0[8])+(self.0[11]*rhs.0[12]);
        ret.0[9] = (self.0[8]*rhs.0[1])+(self.0[9]*rhs.0[5])+(self.0[10]*rhs.0[9])+(self.0[11]*rhs.0[13]);
        ret.0[10] = (self.0[8]*rhs.0[2])+(self.0[9]*rhs.0[6])+(self.0[10]*rhs.0[10])+(self.0[11]*rhs.0[14]);
        ret.0[11] = (self.0[8]*rhs.0[3])+(self.0[9]*rhs.0[7])+(self.0[10]*rhs.0[11])+(self.0[11]*rhs.0[15]);
    
        ret.0[12] = (self.0[12]*rhs.0[0])+(self.0[13]*rhs.0[4])+(self.0[14]*rhs.0[8])+(self.0[15]*rhs.0[12]);
        ret.0[13] = (self.0[12]*rhs.0[1])+(self.0[13]*rhs.0[5])+(self.0[14]*rhs.0[9])+(self.0[15]*rhs.0[13]);
        ret.0[14] = (self.0[12]*rhs.0[2])+(self.0[13]*rhs.0[6])+(self.0[14]*rhs.0[10])+(self.0[15]*rhs.0[14]);
        ret.0[15] = (self.0[12]*rhs.0[3])+(self.0[13]*rhs.0[7])+(self.0[14]*rhs.0[11])+(self.0[15]*rhs.0[15]);    

        ret
    }
}

impl<T: Copy + Default + Mul<Output = T> + Add<Output = T>> MulAssign for Matrix4<T> {
    fn mul_assign(&mut self, rhs: Self) {
        let tmp = self.clone();
        *self = tmp * rhs;
    }
}