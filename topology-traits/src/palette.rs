//! Implementation of a topology for all color types in Palette

use crate::Geodesic;
use palette::Mix;

// We must use derive for all possible color spaces
// impl<T, R> Geodesic<R> for T where T: Mix<Scalar = R> {}
