use fj_math::{Scalar, Winding};

use crate::{
    geometry::SurfacePath,
    objects::{HalfEdge, ObjectSet},
    storage::Handle,
};

/// A cycle of connected edges
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle {
    half_edges: ObjectSet<HalfEdge>,
}

impl Cycle {
    /// Create an instance of `Cycle`
    pub fn new(half_edges: impl IntoIterator<Item = Handle<HalfEdge>>) -> Self {
        let half_edges = half_edges.into_iter().collect();
        Self { half_edges }
    }

    /// Access the edges that make up the cycle
    pub fn half_edges(&self) -> &ObjectSet<HalfEdge> {
        &self.half_edges
    }

    /// Indicate the cycle's winding, assuming a right-handed coordinate system
    ///
    /// Please note that this is not *the* winding of the cycle, only one of the
    /// two possible windings, depending on the direction you look at the
    /// surface that the cycle is defined on from.
    pub fn winding(&self) -> Winding {
        // The cycle could be made up of one or two circles. If that is the
        // case, the winding of the cycle is determined by the winding of the
        // first circle.
        if self.half_edges.len() < 3 {
            let first = self
                .half_edges()
                .iter()
                .next()
                .expect("Invalid cycle: expected at least one edge");

            let [a, b] = first.boundary().inner;
            let edge_direction_positive = a < b;

            let circle = match first.path() {
                SurfacePath::Circle(circle) => circle,
                SurfacePath::Line(_) => unreachable!(
                    "Invalid cycle: less than 3 edges, but not all are circles"
                ),
            };
            let cross_positive = circle.a().cross2d(&circle.b()) > Scalar::ZERO;

            if edge_direction_positive == cross_positive {
                return Winding::Ccw;
            } else {
                return Winding::Cw;
            }
        }

        // Now that we got the special case out of the way, we can treat the
        // cycle as a polygon:
        // https://stackoverflow.com/a/1165943

        let mut sum = Scalar::ZERO;

        for (a, b) in self.half_edges().pairs() {
            let [a, b] = [a, b].map(|edge| edge.start_position());

            sum += (b.u - a.u) * (b.v + a.v);
        }

        if sum > Scalar::ZERO {
            return Winding::Cw;
        }
        if sum < Scalar::ZERO {
            return Winding::Ccw;
        }

        unreachable!("Encountered invalid cycle: {self:#?}");
    }
}
