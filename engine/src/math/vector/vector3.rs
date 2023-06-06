use std::{ops::{Add, Mul, Sub, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, Neg}, fmt::Display};

use nanoserde::{SerBin, DeBin};

use super::{IntoVector, Vector2};

#[derive(SerBin, DeBin, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> IntoVector<T> for (T, T, T) {
    type Vector = Vector3<T>;

    fn into_vector(self) -> Self::Vector {
        Self::Vector {
            x: self.0,
            y: self.1,
            z: self.2
        }
    }
}

impl<T> IntoVector<T> for (Vector2<T>, T) {
    type Vector = Vector3<T>;

    fn into_vector(self) -> Self::Vector {
        Self::Vector {
            x: self.0.x,
            y: self.0.y,
            z: self.1
        }
    }
}

impl<T> Vector3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn map<U>(self, func: impl Fn(T) -> U) -> Vector3<U> {
        Vector3::<U> {
            x: func(self.x),
            y: func(self.y),
            z: func(self.z)
        }
    }

    pub fn mag_sq(&self) -> T where T: Add<Output=T> + Mul<Output=T> + Copy {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn area(&self) -> T where T: Mul<Output=T> + Copy  {
        self.x * self.y * self.z
    }

    pub fn into_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl Vector3<i32> {
    pub fn half(mut self) -> Self {
        self.x >>= 1;
        self.y >>= 1;
        self.z >>= 1;
        self
    }

    pub fn to_f32(&self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    pub fn to_f64(&self) -> Vector3<f64> {
        Vector3::new(self.x as f64, self.y as f64, self.z as f64)
    }
}

impl Vector3<u32> {
    pub fn to_f32(&self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    pub fn to_f64(&self) -> Vector3<f64> {
        Vector3::new(self.x as f64, self.y as f64, self.z as f64)
    }

    pub fn signed(&self) -> Vector3<i32> {
        Vector3::new(self.x as i32, self.y as i32, self.z as i32)
    }
}

impl Vector3<f32> {
    pub fn round_i32(&self) -> Vector3<i32> {
        Vector3::new(self.x.round() as i32, self.y.round() as i32, self.z.round() as i32)
    }

    pub fn round(&self) -> Vector3<f32> {
        Vector3::new(self.x.round(), self.y.round(), self.z.round())
    }

    pub fn floor(&self) -> Vector3<f32> {
        Vector3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    pub fn ceil(&self) -> Vector3<f32> {
        Vector3::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    pub fn mag(&self) -> f32 {
        f32::sqrt(self.mag_sq())
    }

    pub fn normalized(&self) -> Self {
        *self / self.mag()
    }

    pub fn half(mut self) -> Self {
        self.x /= 2.0;
        self.y /= 2.0;
        self.z /= 2.0;
        self
    }

    pub fn double(mut self) -> Self {
        self.x *= 2.0;
        self.y *= 2.0;
        self.z *= 2.0;
        self
    } 

    pub fn to_f64(&self) -> Vector3<f64> {
        Vector3::new(self.x as f64, self.y as f64, self.z as f64)
    }

    pub fn to_i32(&self) -> Vector3<i32> {
        Vector3::new(self.x as i32, self.y as i32, self.z as i32)
    }
}

impl Vector3<f64> {
    pub fn round(&self) -> Vector3<f64> {
        Vector3::new(self.x.round(), self.y.round(), self.z.round())
    }

    pub fn floor(&self) -> Vector3<f64> {
        Vector3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    pub fn ceil(&self) -> Vector3<f64> {
        Vector3::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    pub fn mag(&self) -> f64 {
        f64::sqrt(self.mag_sq())
    }

    pub fn half(mut self) -> Self {
        self.x /= 2.0;
        self.y /= 2.0;
        self.z /= 2.0;
        self
    }
}

impl<T: PartialOrd> Vector3<T> {
    pub fn clamp(mut self, lo: Self, hi: Self) -> Self {
        if self.x < lo.x {
            self.x = lo.x
        }
        if self.y < lo.y {
            self.y = lo.y
        }
        if self.z < lo.z {
            self.z = lo.z
        }
        if self.x > hi.x {
            self.x = hi.x
        }
        if self.y > hi.y {
            self.y = hi.y
        }
        if self.z > hi.z {
            self.z = hi.z
        }

        self
    }
}

impl<T: Into<f64> + Copy> Vector3<T> {
    pub fn dist(&self, other: &Self) -> f64 {
        let x = self.x.into() - other.x.into();
        let y = self.y.into() - other.y.into();
        let z = self.z.into() - other.z.into();
        f64::sqrt(x * x + y * y + z * z)
    }
}

impl<T: Copy> Vector3<T> {
    #[inline(always)]
    pub fn x(&self) -> T {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> T {
        self.y
    }

    #[inline(always)]
    pub fn z(&self) -> T {
        self.z
    }
}

impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl<T: Div<Output = T>> Div for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl<T: Rem<Output = T> + Copy> Rem<T> for Vector3<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs
        }
    }
}

impl<T: AddAssign> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: SubAssign> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: MulAssign> MulAssign for Vector3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: DivAssign> DivAssign for Vector3<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl<T: Display> Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}