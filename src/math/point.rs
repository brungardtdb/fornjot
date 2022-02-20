use std::ops;

use super::{
    coordinates::{Uv, Xyz, T},
    Scalar, Vector,
};

/// An n-dimensional point
///
/// The dimensionality is defined by the const generic argument `D`.
///
/// # Implementation Note
///
/// The goal of this type is to eventually implement `Eq` and `Hash`, making it
/// easier to work with vectors. This is a work in progress.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Point<const D: usize> {
    pub coords: Vector<D>,
}

impl<const D: usize> Point<D> {
    /// Construct a `Point` at the origin of the coordinate system
    pub fn origin() -> Self {
        nalgebra::Point::<_, D>::origin().into()
    }

    /// Construct a `Point` from an array
    pub fn from_array(array: [f64; D]) -> Self {
        Self {
            coords: array.map(Scalar::from_f64).into(),
        }
    }

    /// Construct a `Point` from an nalgebra vector
    pub fn from_na(point: nalgebra::Point<f64, D>) -> Self {
        Self {
            coords: point.coords.into(),
        }
    }

    /// Convert the point into an nalgebra point
    pub fn to_na(&self) -> nalgebra::Point<f64, D> {
        nalgebra::Point {
            coords: self.coords.into(),
        }
    }

    /// Convert to a 1-dimensional point
    pub fn to_t(&self) -> Point<1> {
        Point {
            coords: self.coords.to_t(),
        }
    }
}

impl ops::Deref for Point<1> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.coords.deref()
    }
}

impl ops::Deref for Point<2> {
    type Target = Uv;

    fn deref(&self) -> &Self::Target {
        self.coords.deref()
    }
}

impl ops::Deref for Point<3> {
    type Target = Xyz;

    fn deref(&self) -> &Self::Target {
        self.coords.deref()
    }
}

impl ops::DerefMut for Point<1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.coords.deref_mut()
    }
}

impl ops::DerefMut for Point<2> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.coords.deref_mut()
    }
}

impl ops::DerefMut for Point<3> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.coords.deref_mut()
    }
}

impl<const D: usize> From<[Scalar; D]> for Point<D> {
    fn from(array: [Scalar; D]) -> Self {
        Self {
            coords: array.into(),
        }
    }
}

impl<const D: usize> From<[f64; D]> for Point<D> {
    fn from(array: [f64; D]) -> Self {
        Self::from_array(array)
    }
}

impl<const D: usize> From<nalgebra::Point<f64, D>> for Point<D> {
    fn from(point: nalgebra::Point<f64, D>) -> Self {
        Self::from_na(point)
    }
}

impl<const D: usize> From<Point<D>> for [f32; D] {
    fn from(point: Point<D>) -> Self {
        point.coords.into()
    }
}

impl<const D: usize> From<Point<D>> for [f64; D] {
    fn from(point: Point<D>) -> Self {
        point.coords.into()
    }
}

impl<const D: usize> ops::Neg for Point<D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.to_na().neg().into()
    }
}

impl<const D: usize> ops::Add<Vector<D>> for Point<D> {
    type Output = Self;

    fn add(self, rhs: Vector<D>) -> Self::Output {
        self.to_na().add(rhs.to_na()).into()
    }
}

impl<const D: usize> ops::Sub<Point<D>> for Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Point<D>) -> Self::Output {
        self.to_na().sub(rhs.to_na()).into()
    }
}

impl<const D: usize> ops::Sub<Point<D>> for &Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Point<D>) -> Self::Output {
        self.to_na().sub(rhs.to_na()).into()
    }
}

impl<const D: usize> ops::Mul<f64> for Point<D> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.to_na().mul(rhs).into()
    }
}

impl<const D: usize> approx::AbsDiffEq for Point<D> {
    type Epsilon = <Vector<D> as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.coords.abs_diff_eq(&other.coords, epsilon)
    }
}
