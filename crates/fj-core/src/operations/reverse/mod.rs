//! Reverse the direction/orientation of objects

use crate::Instance;

mod cycle;
mod edge;
mod face;
mod region;

/// Reverse the direction/orientation of an object
pub trait Reverse {
    /// Reverse the direction/orientation of the object
    #[must_use]
    fn reverse(&self, core: &mut Instance) -> Self;
}

/// Reverse the direction of the curve coordinate systems within an object
pub trait ReverseCurveCoordinateSystems {
    /// Reverse the direction of the curve coordinate systems within an object
    ///
    /// This will not have any effect on object positions in global coordinates.
    #[must_use]
    fn reverse_curve_coordinate_systems(&self, core: &mut Instance) -> Self;
}
