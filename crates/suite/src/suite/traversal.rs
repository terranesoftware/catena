use crate::traits::{Traversal, traversal::{Grammar, Index}};

/// Entities that contribute to the navigation of a language.
pub struct Suite<T: Traversal> {
    grammar: T::Grammar,
    index: T::Index
}

impl<T: Traversal> Suite<T> {
    pub fn grammar(
        &self,
        text: &[u8]
    ) {
        self.grammar.grammar(text);
    }

    pub fn index(
        &self,
        text: &[u8]
    ) {
        self.index.index(text);
    }
}