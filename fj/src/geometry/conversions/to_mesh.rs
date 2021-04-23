use std::convert::Infallible;

use nalgebra::Point3;

use crate::geometry::{
    conversions::ToPolygon,
    triangulation::brute_force::{self, triangulate},
    Mesh,
};

pub trait ToMesh {
    type Error;

    fn to_mesh(self, tolerance: f32) -> Result<Mesh, Self::Error>;
}

impl ToMesh for Mesh {
    type Error = Infallible;

    fn to_mesh(self, _tolerance: f32) -> Result<Mesh, Self::Error> {
        Ok(self)
    }
}

impl<T> ToMesh for T
where
    T: ToPolygon,
{
    type Error = brute_force::InternalError;

    fn to_mesh(self, tolerance: f32) -> Result<Mesh, Self::Error> {
        let mut mesh = Mesh::new();

        let polygon = self.to_polygon(tolerance);
        let triangles = triangulate(polygon)?;

        for triangle in triangles {
            let a_x: f32 = triangle.a.x.into();
            let a_y: f32 = triangle.a.y.into();
            let b_x: f32 = triangle.b.x.into();
            let b_y: f32 = triangle.b.y.into();
            let c_x: f32 = triangle.c.x.into();
            let c_y: f32 = triangle.c.y.into();

            let a = mesh.vertex(Point3::new(a_x, a_y, 0.0));
            let b = mesh.vertex(Point3::new(b_x, b_y, 0.0));
            let c = mesh.vertex(Point3::new(c_x, c_y, 0.0));

            mesh.triangle(a, b, c);
        }

        Ok(mesh)
    }
}
