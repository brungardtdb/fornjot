use fj_math::{Line, Point, Scalar, Vector};

use crate::{
    objects::{Curve, GlobalCurve, Surface},
    path::{GlobalPath, SurfacePath},
    stores::{Handle, Stores},
};

/// API for building a [`Curve`]
///
/// Also see [`Curve::builder`].
pub struct CurveBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`Curve`] is defined in
    pub surface: Surface,
}

impl<'a> CurveBuilder<'a> {
    /// Build a line that represents the u-axis on the surface
    pub fn build_u_axis(self) -> Curve {
        let a = Point::origin();
        let b = a + Vector::unit_u();

        self.build_line_from_points([a, b])
    }

    /// Build a line that represents the v-axis on the surface
    pub fn build_v_axis(self) -> Curve {
        let a = Point::origin();
        let b = a + Vector::unit_v();

        self.build_line_from_points([a, b])
    }

    /// Build a circle from the given radius
    pub fn build_circle_from_radius(self, radius: impl Into<Scalar>) -> Curve {
        let radius = radius.into();

        let path = SurfacePath::circle_from_radius(radius);
        let global_form =
            GlobalCurve::partial(self.stores).circle_from_radius(radius);

        Curve::new(self.surface, path, global_form)
    }

    /// Build a line from the given points
    pub fn build_line_from_points(
        &self,
        points: [impl Into<Point<2>>; 2],
    ) -> Curve {
        let points = points.map(Into::into);

        let path = SurfacePath::Line(Line::from_points(points));
        let global_form = self.stores.global_curves.insert(
            GlobalCurve::from_path(GlobalPath::Line(Line::from_points(
                points
                    .map(|point| self.surface.point_from_surface_coords(point)),
            ))),
        );

        Curve::new(self.surface, path, global_form)
    }
}

/// API for building a [`GlobalCurve`]
///
/// Also see [`GlobalCurve::partial`].
pub struct PartialGlobalCurve<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The path that defines the [`GlobalCurve`]
    ///
    /// Must be provided before [`PartialGlobalCurve::build`] is called.
    pub path: Option<GlobalPath>,
}

impl<'a> PartialGlobalCurve<'a> {
    /// Provide a path for the partial global curve
    pub fn with_path(mut self, path: GlobalPath) -> Self {
        self.path = Some(path);
        self
    }

    /// Build a line that represents the x-axis
    pub fn x_axis(&self) -> Handle<GlobalCurve> {
        self.stores
            .global_curves
            .insert(GlobalCurve::from_path(GlobalPath::x_axis()))
    }

    /// Build a line that represents the y-axis
    pub fn y_axis(&self) -> Handle<GlobalCurve> {
        self.stores
            .global_curves
            .insert(GlobalCurve::from_path(GlobalPath::y_axis()))
    }

    /// Build a line that represents the z-axis
    pub fn z_axis(&self) -> Handle<GlobalCurve> {
        self.stores
            .global_curves
            .insert(GlobalCurve::from_path(GlobalPath::z_axis()))
    }

    /// Build a circle from the given radius
    pub fn circle_from_radius(
        &self,
        radius: impl Into<Scalar>,
    ) -> Handle<GlobalCurve> {
        let path = GlobalPath::circle_from_radius(radius);
        self.stores
            .global_curves
            .insert(GlobalCurve::from_path(path))
    }

    /// Create a line from the given points
    pub fn line_from_points(
        &self,
        points: [impl Into<Point<3>>; 2],
    ) -> Handle<GlobalCurve> {
        let line = Line::from_points(points);
        self.stores
            .global_curves
            .insert(GlobalCurve::from_path(GlobalPath::Line(line)))
    }

    /// Build a full [`GlobalCurve`] from the partial global curve
    ///
    /// # Panics
    ///
    /// Panics, if no path was provided.
    pub fn build(self) -> Handle<GlobalCurve> {
        let path = self.path.expect("Can't build `GlobalCurve` without a path");
        self.stores
            .global_curves
            .insert(GlobalCurve::from_path(path))
    }
}
