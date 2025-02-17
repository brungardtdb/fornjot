use fj_math::Vector;

use crate::{
    objects::{Face, Shell},
    operations::insert::Insert,
    storage::Handle,
    Instance,
};

use super::{SweepCache, SweepRegion};

/// # Sweep a [`Face`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepFace {
    /// # Sweep the [`Face`] into a [`Shell`]
    fn sweep_face(
        &self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Instance,
    ) -> Shell;
}

impl SweepFace for Handle<Face> {
    fn sweep_face(
        &self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Instance,
    ) -> Shell {
        // Please note that this function uses the words "bottom" and "top" in a
        // specific sense:
        //
        // - "Bottom" refers to the origin of the sweep. The bottom face is the
        //   original face, or a face in the same place.
        // - "Top" refers to the location of the face that was created by
        //   translating the bottom face along the path.
        // - "Side" refers to new faces created in between bottom and top.
        //
        // These words are specifically *not* meant in the sense of z-axis
        // locations, and depending on the direction of `path`, the two meanings
        // might actually be opposite.

        let path = path.into();

        let bottom_face = self;
        let other_faces = bottom_face
            .region()
            .sweep_region(bottom_face.surface(), path, cache, core)
            .all_faces()
            .map(|side_face| side_face.insert(&mut core.services));

        let mut faces = Vec::new();
        faces.push(bottom_face.clone());
        faces.extend(other_faces);

        Shell::new(faces)
    }
}
