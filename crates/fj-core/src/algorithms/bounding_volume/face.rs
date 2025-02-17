use fj_math::Aabb;

use crate::{geometry::GlobalPath, objects::Face};

impl super::BoundingVolume<3> for Face {
    fn aabb(&self) -> Option<Aabb<3>> {
        self.region().exterior().aabb().map(|aabb2| {
            let surface = self.surface().geometry();

            match surface.u {
                GlobalPath::Circle(circle) => {
                    // This is not the most precise way to calculate the AABB,
                    // doing it for the whole circle, but it should do.

                    let aabb_bottom = circle.aabb();
                    let aabb_top = Aabb {
                        min: aabb_bottom.min + surface.v,
                        max: aabb_bottom.max + surface.v,
                    };

                    aabb_bottom.merged(&aabb_top)
                }
                GlobalPath::Line(_) => Aabb {
                    min: surface.point_from_surface_coords(aabb2.min),
                    max: surface.point_from_surface_coords(aabb2.max),
                },
            }
        })
    }
}
