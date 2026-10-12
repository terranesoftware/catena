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
    pub fn comprehension(&self) -> &comprehension::Suite<L> {
        &self.comprehension
    }

    pub fn transformation(&self) -> &transformation::Suite<L> {
        &self.transformation
    }

    pub fn traversal(&self) -> &traversal::Suite<L> {
        &self.traversal
    }
}