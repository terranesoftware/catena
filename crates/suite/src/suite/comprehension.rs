use crate::traits::{Comprehension, comprehension::{Descriptions, Diagnostics, Highlights}};

/// Entities that contribute to the understanding of a language.
pub struct Suite<C: Comprehension> {
    descriptions: C::Descriptions,
    diagnostics: C::Diagnostics,
    highlights: C::Highlights
}

impl<C: Comprehension> Suite<C> {
    pub fn descriptions(&self) {
        self.descriptions.descriptions();
    }

    pub fn diagnostics(&self) {
        self.diagnostics.diagnostics();
    }

    pub fn highlights(&self) {
        self.highlights.highlights();
    }
}