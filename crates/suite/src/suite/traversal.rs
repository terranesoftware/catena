use crate::traits::Traversal;

/// Entities that contribute to the navigation of a language.
pub struct Suite<T: Traversal> {
    grammar: T::Grammar,
    indexes: Vec<T::Index>
}

impl<T: Traversal> Suite<T> {
    /// Returns a reference to the contained grammar entity.
    pub fn grammar(&self) -> &T::Grammar {
        &self.grammar
    }

    /// Returns a reference to the contained index entities.
    pub fn indexes(&self) -> &[T::Index] {
        &self.indexes
    }
}