pub mod vector2;
pub use vector2::*;

pub mod vector3;
pub use vector3::*;

pub trait IntoVector<T> {
    type Vector;

    fn into_vector(self) -> Self::Vector;
}
