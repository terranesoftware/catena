use crate::traits::Traversal;

/// Entities that contribute to the navigation of a language.
pub struct Suite<T: Traversal> {
    grammar: T::Grammar,
    indexes: Vec<T::Index>
}

impl<T: Traversal> Suite<T> {
    pub fn grammar(&self) -> &T::Grammar {
        &self.grammar
    }

    pub fn indexes(&self) -> &[T::Index] {
        &self.indexes
    }
}