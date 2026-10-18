use crate::traits::{Comprehension, comprehension::{Descriptions, Diagnostics, Highlights}};

/// Entities that contribute to the understanding of a language.
pub struct Suite<C: Comprehension> {
    descriptions: C::Descriptions,
    diagnostics: C::Diagnostics,
    highlights: C::Highlights
}

impl<C: Comprehension> Suite<C> {
    pub fn descriptions(
        &self,
        text: &[u8]
    ) {
        self.descriptions.descriptions(text);
    }

    pub fn diagnostics(
        &self,
        text: &[u8]
    ) {
        self.diagnostics.diagnostics(text);
    }

    pub fn highlights(
        &self,
        text: &[u8]
    ) {
        self.highlights.highlights(text);
    }
}