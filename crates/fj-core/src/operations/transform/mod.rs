//! API for transforming objects

mod curve;
mod cycle;
mod edge;
mod face;
mod region;
mod shell;
mod solid;
mod surface;
mod vertex;

use std::collections::BTreeMap;

use fj_math::{Transform, Vector};
use type_map::TypeMap;

use crate::{
    operations::insert::Insert,
    storage::{Handle, ObjectId},
    Instance,
};

/// Transform an object
///
/// # Implementation Note
///
/// So far, a general `transform` method is available, along some convenience
/// methods for more specific transformations.
///
/// More convenience methods can be added as required. The only reason this
/// hasn't been done so far, is that no one has put in the work yet.
pub trait TransformObject: Sized {
    /// Transform the object
    fn transform(&self, transform: &Transform, core: &mut Instance) -> Self {
        let mut cache = TransformCache::default();
        self.transform_with_cache(transform, core, &mut cache)
    }

    /// Transform the object using the provided cache
    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Instance,
        cache: &mut TransformCache,
    ) -> Self;

    /// Translate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn translate(
        &self,
        offset: impl Into<Vector<3>>,
        core: &mut Instance,
    ) -> Self {
        self.transform(&Transform::translation(offset), core)
    }

    /// Rotate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn rotate(
        &self,
        axis_angle: impl Into<Vector<3>>,
        core: &mut Instance,
    ) -> Self {
        self.transform(&Transform::rotation(axis_angle), core)
    }
}

impl<T> TransformObject for Handle<T>
where
    T: Clone + Insert<Inserted = Handle<T>> + TransformObject + 'static,
{
    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Instance,
        cache: &mut TransformCache,
    ) -> Self {
        if let Some(object) = cache.get(self) {
            return object.clone();
        }

        let transformed = self
            .clone_object()
            .transform_with_cache(transform, core, cache)
            .insert(&mut core.services);

        cache.insert(self.clone(), transformed.clone());

        transformed
    }
}

/// A cache for transformed objects
///
/// See [`TransformObject`].
#[derive(Default)]
pub struct TransformCache(TypeMap);

impl TransformCache {
    fn get<T: 'static>(&mut self, key: &Handle<T>) -> Option<&Handle<T>> {
        // Silencing Clippy warning due to false positive in Rust 1.73.0. See:
        // https://github.com/rust-lang/rust-clippy/issues/11390#issuecomment-1750951533
        #[allow(clippy::unwrap_or_default)]
        let map = self
            .0
            .entry::<BTreeMap<ObjectId, Handle<T>>>()
            .or_insert_with(BTreeMap::new);

        map.get(&key.id())
    }

    fn insert<T: 'static>(&mut self, key: Handle<T>, value: Handle<T>) {
        // Silencing Clippy warning due to false positive in Rust 1.73.0. See:
        // https://github.com/rust-lang/rust-clippy/issues/11390#issuecomment-1750951533
        #[allow(clippy::unwrap_or_default)]
        let map = self
            .0
            .entry::<BTreeMap<ObjectId, Handle<T>>>()
            .or_insert_with(BTreeMap::new);

        map.insert(key.id(), value);
    }
}
