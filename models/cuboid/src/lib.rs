use fj::{
    core::{
        objects::{Region, Sketch, Solid},
        operations::{
            build::{BuildRegion, BuildSketch},
            insert::Insert,
            sweep::SweepSketch,
            update::UpdateSketch,
        },
        services::Services,
        storage::Handle,
    },
    math::{Scalar, Vector},
};

pub fn model(
    size: impl Into<Vector<3>>,
    services: &mut Services,
) -> Handle<Solid> {
    let [x, y, z] = size.into().components;

    let surface = services.objects.surfaces.xy_plane();
    let path = Vector::from([Scalar::ZERO, Scalar::ZERO, z]);

    let sketch = Sketch::empty().add_region(
        Region::polygon(
            [
                [-x / 2., -y / 2.],
                [x / 2., -y / 2.],
                [x / 2., y / 2.],
                [-x / 2., y / 2.],
            ],
            services,
        )
        .insert(services),
    );

    sketch
        .sweep_sketch(surface, path, services)
        .insert(services)
}
