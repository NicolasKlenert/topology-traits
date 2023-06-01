#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::{Add, Mul};
use num_traits::real::Real;

/// All elements in a quasi metric space should implement this trait.
///
/// If the struct implementing this trait also implements `Topology`,
/// the `distance(to)` method should be equivalent to `self.shortest_path(to).length()`.
pub trait QuasiMetric<T = f64> {
    /// Returns the length of the shortest path between the two points.
    fn distance(self, to: Self) -> T;
}

/// Trait for elements having a length.
pub trait Length<T = f64> {
    /// Returns the length of the element.
    fn length(&self) -> T;
}

/// The merge trait is used to merge two elements together.
///
/// Often this is a linear interpolation between two elements.
/// In the case of Quaternions it is a spherical linear interpolation.
///
/// A default implementation of this trait is provided for all `E` that
/// are `Add<Output = E> + Mul<T, Output = E> + Copy` as these
/// operations let us assume that the elements live in a vector-like space.
///
/// If the struct implementing this trait also implements `Topology`,
/// the `merge(factor)` method should be equivalent to `self.shortest_path(to).contract(factor)`.
pub trait Merge<T = f64> {
    /// Merge between `self` and `to` using `factor`.
    ///
    /// This can be thought of creating a point on (one of) the shortest Paths
    /// between the two given points.
    ///
    /// Merging `self` with a factor of `Zero` should return a copy of `self`.
    /// Merging `to` with a factor of `One` should return a copy of `to`.
    /// It is assumed that the factor decides how similar the result will be to either
    /// `self` or `to`.
    fn merge(self, to: Self, factor: T) -> Self;
}

/// Trait for structures containing two elements which can be merged together.
pub trait Connected<P, T = f64> {
    /// Returns the merged point of the start point and end point inside `self` with weight `factor`.
    ///
    /// Contracting with a factor of `Zero` should return the start point.
    /// Contracting with a factor of `One` should return the end point.
    /// The factor decides how similar the result will be to the start or end point.
    fn contract(&self, factor: T) -> P;
}

/// Main trait for topological spaces.
///
/// The associated type `Path` should implement as many traits as possible which allows to implement
/// other traits trivially.
/// - Length -> QuasiMetric
/// - Merge -> Connected
pub trait Topology {
    /// The type of paths in this space. Usually \[Self,Self\].
    type Path;
    /// Function which returns the shortest path between `self` as start point and `to` as end point.
    ///
    /// Often this returns a line are something equivalent from `self` to `to`.
    fn shortest_path(self, to: Self) -> Self::Path;
}

impl<E, T> Merge<T> for E
where
    E: Add<Output = E> + Mul<T, Output = E> + Copy,
    T: Real,
{
    fn merge(self, other: Self, factor: T) -> E {
        self * (T::one() - factor) + other * factor
    }
}
