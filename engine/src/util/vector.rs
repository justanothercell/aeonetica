use std::ops::*;
use std::f64;

pub trait IntoVector<T> {
    type Vector;

    fn into_vector(self) -> Self::Vector;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

impl<T> IntoVector<T> for (T, T) {
    type Vector = Vector2<T>;

    fn into_vector(self) -> Self::Vector {
        Self::Vector::from(self)
    }
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn map<U>(self, func: impl Fn(T) -> U) -> Vector2<U> {
        Vector2::<U> {
            x: func(self.x),
            y: func(self.y)
        }
    }

    pub fn mag_sq(&self) -> T where T: Add<Output=T> + Mul<Output=T> + Copy {
        self.x * self.x + self.y * self.y
    }

    pub fn area(&self) -> T where T: Mul<Output=T> + Copy  {
        self.x * self.y
    }
}

impl Vector2<i32> {
    pub fn half(mut self) -> Self {
        self.x >>= 1;
        self.y >>= 1;
        self
    }

    pub fn to_f32(&self) -> Vector2<f32> {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

impl Vector2<u32> {
    pub fn to_f32(&self) -> Vector2<f32> {
        Vector2::new(self.x as f32, self.y as f32)
    }

    pub fn signed(&self) -> Vector2<i32> {
        Vector2::new(self.x as i32, self.y as i32)
    }
}

impl Vector2<f32> {
    pub fn round(&self) -> Vector2<f32> {
        Vector2::new(self.x.round(), self.y.round())
    }

    pub fn floor(&self) -> Vector2<f32> {
        Vector2::new(self.x.floor(), self.y.floor())
    }

    pub fn ceil(&self) -> Vector2<f32> {
        Vector2::new(self.x.ceil(), self.y.ceil())
    }

    pub fn mag(&self) -> f32 {
        f32::sqrt(self.mag_sq())
    }

    pub fn half(mut self) -> Self {
        self.x /= 2.0;
        self.y /= 2.0;
        self
    }

    pub fn rotate(mut self, theta: f32) -> Self {
        let (x, y) = (self.x, self.y);
        self.x = x * theta.cos() - y * theta.sin();
        self.y = x * theta.sin() + y * theta.cos();

        self
    }
}

impl Vector2<f64> {
    pub fn round(&self) -> Vector2<f64> {
        Vector2::new(self.x.round(), self.y.round())
    }

    pub fn floor(&self) -> Vector2<f64> {
        Vector2::new(self.x.floor(), self.y.floor())
    }

    pub fn ceil(&self) -> Vector2<f64> {
        Vector2::new(self.x.ceil(), self.y.ceil())
    }

    pub fn mag(&self) -> f64 {
        f64::sqrt(self.mag_sq())
    }

    pub fn half(mut self) -> Self {
        self.x /= 2.0;
        self.y /= 2.0;
        self
    }
}

impl<T: Into<f64> + Copy> Vector2<T> {
    pub fn dist(&self, other: &Self) -> f64 {
        let x = self.x.into() - other.x.into();
        let y = self.y.into() - other.y.into();
        f64::sqrt(x * x + y * y)
    }
}

impl<T: Copy> Vector2<T> {
    #[inline(always)]
    pub fn x(&self) -> T {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> T {
        self.y
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}

impl<T> Into<(T, T)> for Vector2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> Into<[T; 2]> for Vector2<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T: Copy> From<[T; 2]> for Vector2<T> {
    fn from(value: [T; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1]
        }
    }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T: Div<Output = T>> Div for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl<T: AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: MulAssign> MulAssign for Vector2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: DivAssign> DivAssign for Vector2<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vector2() {
        let output = Vector2::from((42, 69));
        assert_eq!(Vector2::default() + output, output);
        assert_eq!(Vector2::from((21, 34)) + Vector2::from((21, 35)), output);
    }
}
