use crate::traits::{Comprehension, comprehension::{Descriptions, Diagnostics}};

/// Entities that contribute to the understanding of a language.
pub struct Suite<C: Comprehension> {
    descriptions: C::Descriptions,
    diagnostics: C::Diagnostics
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
}