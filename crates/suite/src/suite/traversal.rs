use crate::traits::Traversal;

/// Entities that contribute to the navigation of a language.
pub struct Suite<T: Traversal> {
    grammar: T::Grammar,
    index: T::Index
}

impl<T: Traversal> Suite<T> {
    /// Returns a reference to the contained `Grammar` entity.
    pub fn grammar(&self) -> &T::Grammar {
        &self.grammar
    }

    /// Returns a reference to the contained `Index` entity.
    pub fn index(&self) -> &T::Index {
        &self.index
    }
}