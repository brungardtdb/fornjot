use std::{collections::BTreeSet, fmt::Debug, slice, vec};

use itertools::Itertools;

use crate::storage::Handle;

/// An ordered set of object handles
///
/// This is the data structure used by all objects that reference multiple
/// objects of the same type. It is a set, not containing any duplicate
/// elements, and it maintains the insertion order of those elements.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Handles<T> {
    // This is supposed to be a set data structure, so what is that `Vec` doing
    // here? Well, it's here because we need it to preserve insertion order, but
    // that doesn't explain why it is here *alone*.
    //
    // If you look closely, you'll notice that this is an immutable data
    // structure (since it is used in objects, and objects themselves are
    // immutable). We make sure there are no duplicates when this is
    // constructed (see the `FromIterator` implementation below), but after
    // that, we're fine.
    inner: Vec<Handle<T>>,
}

impl<T> Handles<T> {
    /// Create a new instances of `Handles` from an iterator over `Handle<T>`
    ///
    /// # Panics
    ///
    /// Panics, if the iterator contains duplicate `Handle`s.
    pub fn new(handles: impl IntoIterator<Item = Handle<T>>) -> Self
    where
        T: Debug + Ord,
    {
        let mut added = BTreeSet::new();
        let mut inner = Vec::new();

        for handle in handles {
            if added.contains(&handle) {
                panic!(
                    "Constructing `HandleSet` with duplicate handle: {:?}",
                    handle
                );
            }

            added.insert(handle.clone());
            inner.push(handle);
        }

        Self { inner }
    }

    /// Return the number of handles in this set
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Indicate whether the set is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Return the n-th item
    pub fn nth(&self, index: usize) -> Option<&Handle<T>> {
        self.inner.get(index)
    }

    /// Return the n-th item, treating the index space as circular
    ///
    /// If the length of `Handles` is `i`, then retrieving the i-th edge using
    /// this method, is the same as retrieving the 0-th one.
    pub fn nth_circular(&self, index: usize) -> &Handle<T> {
        let index = index % self.len();
        self.nth(index)
            .expect("Index must be valid, due to modulo above")
    }

    /// Return the index of the item, if available
    pub fn index_of(&self, handle: &Handle<T>) -> Option<usize> {
        self.inner.iter().position(|h| h.id() == handle.id())
    }

    /// Access the item after the provided one
    ///
    /// Returns `None`, if the provided item is not in this iterator.
    pub fn after(&self, handle: &Handle<T>) -> Option<&Handle<T>> {
        self.index_of(handle)
            .map(|index| self.nth_circular(index + 1))
    }

    /// Access an iterator over the handles
    pub fn iter(&self) -> slice::Iter<Handle<T>> {
        self.inner.iter()
    }

    /// Return iterator over the pairs of all handles
    pub fn pairs(&self) -> impl Iterator<Item = (&Handle<T>, &Handle<T>)> {
        self.iter().circular_tuple_windows()
    }

    /// Create a new instance in which the provided item is updated
    ///
    /// # Panics
    ///
    /// Panics, if the provided item is not present.
    /// Panics, if the update results in a duplicate item.
    #[must_use]
    pub fn update(
        &self,
        handle: &Handle<T>,
        update: impl FnOnce(&Handle<T>) -> Handle<T>,
    ) -> Self
    where
        T: Debug + Ord,
    {
        let mut updated = Some(update(handle));

        let items = self.iter().map(|h| {
            if h.id() == handle.id() {
                updated
                    .take()
                    .expect("`Handles` should not contain same item twice")
            } else {
                h.clone()
            }
        });

        let handles = items.collect();

        assert!(updated.is_none(), "Edge not found in cycle");

        handles
    }
}

impl<O> FromIterator<Handle<O>> for Handles<O>
where
    O: Debug + Ord,
{
    fn from_iter<T: IntoIterator<Item = Handle<O>>>(handles: T) -> Self {
        Self::new(handles)
    }
}

impl<T> IntoIterator for Handles<T> {
    type Item = Handle<T>;
    type IntoIter = vec::IntoIter<Handle<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'r, T> IntoIterator for &'r Handles<T> {
    // You might wonder why we're returning references to handles here, when
    // `Handle` already is kind of reference, and easily cloned.
    //
    // Most of the time that doesn't make a difference, but there are use cases
    // where dealing with owned `Handle`s is inconvenient, for example when
    // using iterator adapters. You can't return a reference to the argument of
    // an adapter's closure, if you own that argument. You can, if you just
    // reference the argument.
    type Item = &'r Handle<T>;
    type IntoIter = slice::Iter<'r, Handle<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
