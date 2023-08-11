#![doc = include_str!("../../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!(
    "The topology-traits crate needs a library for floats. Please enable either \"std\" or \"libm\" as a feature."
);

#[cfg(feature = "palette")]
mod palette;

use core::ops::Mul;
use num_traits::real::Real;

/// Main trait for topological spaces.
///
/// The associated type `Path` must implement `Connected` and if possible should implement `Length`.
pub trait Geodesic<R = f64>
where
    Self: Sized,
{
    /// The type of paths in this space. Usually a newtype of \[Self,Self\].
    type Path: Connected<Self, R>;
    /// Function which returns the shortest path between `self` as start point and `to` as end point.
    ///
    /// Often this returns a line are something equivalent from `self` to `to`.
    fn shortest_path(self, to: Self) -> Self::Path;

    /// Merge between `self` and `to` using `factor`.
    ///
    /// This can be thought of creating a point on (one of) the shortest Paths
    /// between the two given points.
    ///
    /// Merging `self` with a factor of `Zero` should return a copy of `self`.
    /// Merging `to` with a factor of `One` should return a copy of `to`.
    /// The `factor` decides how similar the result will be to either
    /// `self` or `to`.
    fn lerp(self, to: Self, factor: R) -> Self {
        self.shortest_path(to).contract(factor)
    }
}

/// Trait for connected Paths.
pub trait Connected<P, R = f64> {
    /// Returns a point between the start point and end point. `factor` decides how close the point is to the start and end point.
    ///
    /// Contracting with a factor of `Zero` should return the start point.
    /// Contracting with a factor of `One` should return the end point.
    /// The factor decides how similar the result will be to the start or end point.
    fn contract(&self, factor: R) -> P;
}

/// Trait for elements having a length or norm.
pub trait Length<R = f64> {
    /// Returns the length of the element. This should always be non-negative.
    fn length(&self) -> R;
}

/// All elements in a quasi metric space should implement this trait.
///
/// If the struct implementing this trait also implements `Geodesic`,
/// the `distance(to)` method should be equivalent to `self.shortest_path(to).length()`.
pub trait QuasiMetric<R = f64> {
    /// Returns the length of the shortest path between the two points. The distance should always be non-negative.
    fn distance(self, to: Self) -> R;
}

impl<R, T> QuasiMetric<R> for T
where
    T: Geodesic<R>,
    T::Path: Length<R>,
{
    fn distance(self, to: Self) -> R {
        self.shortest_path(to).length()
    }
}

// Implement Geodesic for S X T, if S and T have a Geodesic -> create a marco for that (with any amount of tuples (8 should be enough for now))
// In general we have: Vectors (work like composites or reals) and rotations (any dimension) and composites of these.
// For the future: implement any rotational space! (Geometric Algebra)
// macro saying which fields are translations and which are rotations (list of translations, list of lists of rotations, where the inner list are the rotations values
// connected which each other, creating higher dimensional rotations -> the min,max values would be a parameter -> define rotations through integers!)

// Macro for Compositions! (we assume each field to be non-correlated to the other, and a field can be either a translation or a rotation)
// location -> we can use addition and mulitplication!
// orientation -> we must first define the smallest rotation from the start point to the endpoint? How would we do it?

// Macro needs to now:
// - access of variables
// - constructor

// TODO: Path should always be a newtype! We should give it an id!

impl<R, S, T> Connected<(S, T), R> for (S::Path, T::Path)
where
    S: Geodesic<R>,
    T: Geodesic<R>,
    R: Copy,
{
    fn contract(&self, factor: R) -> (S, T) {
        (self.0.contract(factor), self.1.contract(factor))
    }
}

impl<R, S, T> Geodesic<R> for (S, T)
where
    S: Geodesic<R>,
    T: Geodesic<R>,
    R: Copy,
{
    type Path = (S::Path, T::Path);

    fn shortest_path(self, to: Self) -> Self::Path {
        (self.0.shortest_path(to.0), self.1.shortest_path(to.1))
    }
}

impl<R, T> Connected<[T; 2], R> for [T::Path; 2]
where
    T: Geodesic<R>,
    R: Copy,
{
    fn contract(&self, factor: R) -> [T; 2] {
        [self[0].contract(factor), self[1].contract(factor)]
    }
}

impl<R, T> Geodesic<R> for [T; 2]
where
    T: Geodesic<R> + Copy,
    R: Copy,
{
    type Path = [T::Path; 2];

    fn shortest_path(self, to: Self) -> Self::Path {
        [self[0].shortest_path(to[0]), self[1].shortest_path(to[1])]
    }
}

// Implement Geodesic for all real numbers (for now only f64 and f32) [Generics cannot be used as we have no way of saying that all reals NEVER are a compound -> conflicting implementations]

impl<R: Real> Connected<f64, R> for [f64; 2]
where
    f64: Mul<R, Output = f64>,
{
    fn contract(&self, factor: R) -> f64 {
        self[0] * (R::one() - factor) + self[1] * factor
    }
}

impl Length<f64> for [f64; 2] {
    fn length(&self) -> f64 {
        (self[1] - self[0]).abs()
    }
}

impl<R: Real> Geodesic<R> for f64
where
    f64: Mul<R, Output = f64>,
{
    type Path = [f64; 2];
    fn shortest_path(self, to: Self) -> Self::Path {
        [self, to]
    }
}

impl<R: Real> Connected<f32, R> for [f32; 2]
where
    f32: Mul<R, Output = f32>,
{
    fn contract(&self, factor: R) -> f32 {
        self[0] * (R::one() - factor) + self[1] * factor
    }
}

impl Length<f32> for [f32; 2] {
    fn length(&self) -> f32 {
        (self[1] - self[0]).abs()
    }
}

impl<R: Real> Geodesic<R> for f32
where
    f32: Mul<R, Output = f32>,
{
    type Path = [f32; 2];
    fn shortest_path(self, to: Self) -> Self::Path {
        [self, to]
    }
}

//TODO: derive called linear (as we call it linear space -> any space which is homeomorph to R^n)
// impl<E, T> Merge<T> for E
// where
//     E: Add<Output = E> + Mul<T, Output = E> + Copy,
//     T: Real,
// {
//     fn merge(self, other: Self, factor: T) -> E {
//         self * (T::one() - factor) + other * factor
//     }
// }
