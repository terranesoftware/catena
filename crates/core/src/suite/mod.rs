mod comprehension;
mod transformation;
mod traversal;

use crate::Language;

/// Entities that contribute to the use of a language.
pub struct Suite<L: Language> {
    comprehension: comprehension::Suite<L>,
    transformation: transformation::Suite<L>,
    traversal: traversal::Suite<L>
}

impl<L: Language> Suite<L> {
    /// Returns a reference to the contained comprehension `Suite`.
    pub fn comprehension(&self) -> &comprehension::Suite<L> {
        &self.comprehension
    }

    /// Returns a reference to the contained transformation `Suite`.
    pub fn transformation(&self) -> &transformation::Suite<L> {
        &self.transformation
    }

    /// Returns a reference to the contained traversal `Suite`.
    pub fn traversal(&self) -> &traversal::Suite<L> {
        &self.traversal
    }
}