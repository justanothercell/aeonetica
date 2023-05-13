use std::fmt::Debug;
use std::marker::Destruct;
use std::ops::{ControlFlow, DerefMut, FromResidual};
use std::ops::Deref;

#[derive(Copy, Clone, Debug, Default)]
pub enum Nullable<T> {
    Value(T),
    #[default]
    Null
}

#[allow(unused)]
impl<T> Nullable<T> {
    #[inline]
    pub const fn some(value: T) -> Self {
        Self::Value(value)
    }
    #[inline]
    pub const fn none() -> Self {
        Self::Null
    }
    #[inline]
    pub const fn option(self) -> Option<T> where T: ~const Destruct {
        match self {
            Nullable::Null => None,
            Nullable::Value(v) => Some(v)
        }
    }
    #[inline]
    pub fn ref_option(&self) -> Option<&T> {
        self.as_ref().into()
    }
    #[inline]
    pub fn mut_option(&mut self) -> Option<&mut T> {
        self.as_mut().into()
    }
    #[inline]
    pub const fn as_ref(&self) -> Nullable<&T> {
        match self {
            Nullable::Value(ref v) => Nullable::Value(v),
            Nullable::Null => Nullable::Null
        }
    }
    #[inline]
    pub const fn as_mut(&mut self) -> Nullable<&mut T> {
        match self {
            Nullable::Value(ref mut v) => Nullable::Value(v),
            Nullable::Null => Nullable::Null
        }
    }
    #[inline]
    pub const fn is_null(&self) -> bool {
        matches!(&self, Nullable::Null)
    }
    #[inline]
    pub const fn is_value(&self) -> bool {
        !self.is_null()
    }
    #[inline]
    #[track_caller]
    pub const fn unwrap(self) -> T where T: ~const Destruct {
        match self {
            Nullable::Value(val) => val,
            Nullable::Null => panic!("called `Nullable::unwrap()` on a `Null` value"),
        }
    }
    #[inline]
    #[track_caller]
    pub const fn except(self, msg: &str) -> T where T: ~const Destruct {
        match self {
            Nullable::Value(val) => val,
            Nullable::Null => panic!("{}", msg),
        }
    }
    #[inline]
    pub fn unwrap_or(self, default: T) -> T where T: ~const Destruct {
        self.option().unwrap_or(default)
    }
    #[inline]
    pub fn unwrap_or_else<F: ~const FnOnce() -> T + ~const Destruct>(self, f: F) -> T{
        self.option().unwrap_or_else(f)
    }
    #[inline]
    pub fn unwrap_or_default(self) -> T where T: Default {
        self.option().unwrap_or_default()
    }
    #[inline]
    #[track_caller]
    pub const unsafe fn unwrap_unchecked(self) -> T where T: ~const Destruct {
        self.option().unwrap_unchecked()
    }
    #[inline]
    pub const fn map<U, F: ~const FnOnce(T) -> U + ~const Destruct>(self, f: F) -> Nullable<U> where T: ~const Destruct {
        match self {
            Nullable::Value(x) => Nullable::Value(f(x)),
            Nullable::Null => Nullable::Null,
        }
    }
    #[inline]
    pub const fn inspect<F: ~const FnOnce(&T) + ~const Destruct>(self, f: F) -> Self {
        if let Nullable::Value(ref x) = self {
            f(x);
        }
        self
    }
    #[inline]
    pub fn map_or<U: ~const Destruct, F: ~const FnOnce(T) -> U + ~const Destruct>(self, default: U, f: F) -> U {
        self.option().map_or(default, f)
    }
    #[inline]
    pub fn map_or_else<U, D: ~const FnOnce() -> U + ~const Destruct, F: ~const FnOnce(T) -> U + ~const Destruct>(self, default: D, f: F) -> U {
        self.option().map_or_else(default, f)
    }
    #[inline]
    pub fn ok_or<E: ~const Destruct>(self, err: E) -> Result<T, E> {
        self.option().ok_or(err)
    }
    #[inline]
    pub fn ok_or_else<E, F: ~const FnOnce() -> E + ~const Destruct>(self, err: F) -> Result<T, E> {
        self.option().ok_or_else(err)
    }
    #[inline]
    pub fn as_deref(&self) -> Nullable<&T::Target> where T: Deref {
        match self.as_ref() {
            Nullable::Value(t) => Nullable::Value(t.deref()),
            Nullable::Null => Nullable::Null,
        }
    }
    #[inline]
    pub fn as_deref_mut(&mut self) -> Nullable<&mut T::Target> where T: DerefMut {
        match self.as_mut() {
            Nullable::Value(t) => Nullable::Value(t.deref_mut()),
            Nullable::Null => Nullable::Null,
        }
    }
    #[inline]
    pub const fn and<U: ~const Destruct>(self, nullableb: Nullable<U>) -> Nullable<U> where T: ~const Destruct {
        match self {
            Nullable::Value(_) => nullableb,
            Nullable::Null => Nullable::Null,
        }
    }
    #[inline]
    pub const fn and_then<U, F: ~const FnOnce(T) -> Nullable<U> + ~const Destruct>(self, f: F) -> Nullable<U> where T: ~const Destruct {
        match self {
            Nullable::Value(x) => f(x),
            Nullable::Null => Nullable::Null,
        }
    }
    #[inline]
    pub const fn filter<P: ~const FnOnce(&T) -> bool + ~const Destruct>(self, predicate: P) -> Self where T: ~const Destruct {
        if let Nullable::Value(x) = self {
            if predicate(&x) {
                return Nullable::Value(x);
            }
        }
        Nullable::Null
    }
    #[inline]
    pub const fn or(self, nullableb: Nullable<T>) -> Nullable<T> where T: ~const Destruct {
        match self {
            Nullable::Value(x) => Nullable::Value(x),
            Nullable::Null => nullableb,
        }
    }
    #[inline]
    pub const fn or_else<F: ~const FnOnce() -> Nullable<T> + ~const Destruct>(self, f: F) -> Nullable<T> where T: ~const Destruct {
        match self {
            Nullable::Value(x) => Nullable::Value(x),
            Nullable::Null => f(),
        }
    }
    #[inline]
    pub const fn xor(self, nullableb: Nullable<T>) -> Nullable<T> where T: ~const Destruct {
        match (self, nullableb) {
            (Nullable::Value(a), Nullable::Null) => Nullable::Value(a),
            (Nullable::Null, Nullable::Value(b)) => Nullable::Value(b),
            _ => Nullable::Null,
        }
    }
    #[must_use = "if you intended to set a value, consider assignment instead"]
    #[inline]
    pub const fn insert(&mut self, value: T) -> &mut T where T: ~const Destruct {
        *self = Nullable::Value(value);
        unsafe { self.as_mut().unwrap_unchecked() }
    }
    #[inline]
    pub const fn get_or_insert(&mut self, value: T) -> &mut T where T: ~const Destruct {
        if let Nullable::Null = *self {
            *self = Nullable::Value(value);
        }
        unsafe { self.as_mut().unwrap_unchecked() }
    }
    #[inline]
    pub const fn get_or_insert_with<F: ~const FnOnce() -> T + ~const Destruct>(&mut self, f: F) -> &mut T {
        if let Nullable::Null = *self {
            std::mem::forget(std::mem::replace(self, Nullable::Value(f())))
        }
        unsafe { self.as_mut().unwrap_unchecked() }
    }
    #[inline]
    pub const fn take(&mut self) -> Nullable<T> {
        std::mem::replace(self, Nullable::Null)
    }
    #[inline]
    pub const fn replace(&mut self, value: T) -> Nullable<T> {
        std::mem::replace(self, Nullable::Value(value))
    }
    #[must_use]
    #[inline]
    pub fn contains<U>(&self, x: &U) -> bool where U: PartialEq<T>{
        match self {
            Nullable::Value(y) => x.eq(y),
            Nullable::Null => false,
        }
    }
    #[inline]
    pub const fn zip<U: ~const Destruct>(self, other: Nullable<U>) -> Nullable<(T, U)> where T: ~const Destruct {
        match (self, other) {
            (Nullable::Value(a), Nullable::Value(b)) => Nullable::Value((a, b)),
            _ => Nullable::Null,
        }
    }
    #[inline]
    pub const fn zip_with<U: ~const Destruct, F: ~const FnOnce(T, U) -> R + ~const Destruct, R>(self, other: Nullable<U>, f: F) -> Nullable<R> where T: ~const Destruct {
        match (self, other) {
            (Nullable::Value(a), Nullable::Value(b)) => Nullable::Value(f(a, b)),
            _ => Nullable::Null,
        }
    }
}

impl<T, U> Nullable<(T, U)> {
    #[inline]
    pub const fn unzip(self) -> (Nullable<T>, Nullable<U>) where T: ~const Destruct, U: ~const Destruct {
        match self {
            Nullable::Value((a, b)) => (Nullable::Value(a), Nullable::Value(b)),
            Nullable::Null => (Nullable::Null, Nullable::Null),
        }
    }
}

impl<T> Nullable<&T> {
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub const fn copied(self) -> Nullable<T> where T: Copy, {
        match self {
            Nullable::Value(&v) => Nullable::Value(v),
            Nullable::Null => Nullable::Null,
        }
    }
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn cloned(self) -> Nullable<T> where T: Clone {
        match self {
            Nullable::Value(t) => Nullable::Value(t.clone()),
            Nullable::Null => Nullable::Null,
        }
    }
}

impl<T> Nullable<&mut T> {
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub const fn copied(self) -> Nullable<T> where T: Copy {
        match self {
            Nullable::Value(&mut v) => Nullable::Value(v),
            Nullable::Null => Nullable::Null,
        }
    }
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn cloned(self) -> Nullable<T> where T: Clone{
        match self {
            Nullable::Value(t) => Nullable::Value(t.clone()),
            Nullable::Null => Nullable::Null,
        }
    }
}

impl<T> Deref for Nullable<T>{
    type Target = T;
    #[track_caller]
    fn deref(&self) -> &<Self as Deref>::Target {
        self.ref_option().unwrap()
    }
}

impl<T> DerefMut for Nullable<T>{
    #[track_caller]
    fn deref_mut(&mut self) -> &mut T {
        self.mut_option().unwrap()
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    #[inline]
    fn from(value: Option<T>) -> Self {
        match value {
            None => Nullable::Null,
            Some(v) => Nullable::Value(v)
        }
    }
}

impl<T> From<Nullable<T>> for Option<T> {
    #[inline]
    fn from(value: Nullable<T>) -> Self{
        match value {
            Nullable::Null => None,
            Nullable::Value(v) => Some(v)
        }
    }
}

impl<T> Nullable<Option<T>> {
    #[inline]
    pub const fn flatten(self) -> Nullable<T> where T: ~const Destruct {
        match self {
            Nullable::Value(inner) => match inner {
                None => Nullable::Null,
                Some(v) => Nullable::Value(v)
            },
            Nullable::Null => Nullable::Null,
        }
    }
    #[inline]
    pub const fn opt_flatten(self) -> Option<T> where T: ~const Destruct {
        match self {
            Nullable::Value(inner) => inner,
            Nullable::Null => None,
        }
    }
}

impl<T> Nullable<Nullable<T>> {
    #[inline]
    pub const fn flatten(self) -> Nullable<T> where T: ~const Destruct {
        match self {
            Nullable::Value(inner) => inner,
            Nullable::Null => Nullable::Null,
        }
    }
    #[inline]
    pub fn opt_flatten(self) -> Option<T> where T: ~const Destruct {
        match self {
            Nullable::Value(inner) => inner.into(),
            Nullable::Null => None,
        }
    }
}

impl<T> FromResidual for Nullable<T> {
    #[inline]
    fn from_residual(_residual: Nullable<std::convert::Infallible>) -> Self {
        Nullable::Null
    }
}

impl<T> FromResidual<Option<std::convert::Infallible>> for Nullable<T> {
    #[inline]
    fn from_residual(_residual: Option<std::convert::Infallible>) -> Self {
        Nullable::Null
    }
}

impl<T> std::ops::Try for Nullable<T> {
    type Output = T;
    type Residual = Nullable<std::convert::Infallible>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Nullable::Value(output)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Nullable::Value(v) => ControlFlow::Continue(v),
            Nullable::Null => ControlFlow::Break(Nullable::Null),
        }
    }
}
