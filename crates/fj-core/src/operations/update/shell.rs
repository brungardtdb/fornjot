use crate::{
    objects::{Face, Shell},
    storage::Handle,
};

/// Update a [`Shell`]
pub trait UpdateShell {
    /// Add faces to the shell
    #[must_use]
    fn add_faces(&self, faces: impl IntoIterator<Item = Handle<Face>>) -> Self;

    /// Update a face of the shell
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn update_face<const N: usize>(
        &self,
        handle: &Handle<Face>,
        update: impl FnOnce(&Handle<Face>) -> [Handle<Face>; N],
    ) -> Self;

    /// Remove a face from the shell
    #[must_use]
    fn remove_face(&self, handle: &Handle<Face>) -> Self;
}

impl UpdateShell for Shell {
    fn add_faces(&self, faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        let faces = self.faces().iter().cloned().chain(faces);
        Shell::new(faces)
    }

    fn update_face<const N: usize>(
        &self,
        handle: &Handle<Face>,
        update: impl FnOnce(&Handle<Face>) -> [Handle<Face>; N],
    ) -> Self {
        let faces = self
            .faces()
            .replace(handle, update(handle))
            .expect("Face not found");
        Shell::new(faces)
    }

    fn remove_face(&self, handle: &Handle<Face>) -> Self {
        let faces = self
            .faces()
            .iter()
            .filter(|face| face.id() != handle.id())
            .cloned();

        Shell::new(faces)
    }
}
