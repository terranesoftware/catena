use crate::traits::Comprehension;

/// Entities that contribute to the understanding of a language.
pub struct Suite<C: Comprehension> {
    descriptions: C::Descriptions,
    diagnostics: C::Diagnostics,
    highlights: C::Highlights
}

impl<C: Comprehension> Suite<C> {
    /// Returns the a reference to the contained `Descriptions` entity.
    pub fn descriptions(&self) -> &C::Descriptions {
        &self.descriptions
    }

    /// Returns a reference to the contained `Diagnostics` entity.
    pub fn diagnostics(&self) -> &C::Diagnostics {
        &self.diagnostics
    }

    /// Returns a reference to the contained `Highlights` entity.
    pub fn highlights(&self) -> &C::Highlights {
        &self.highlights
    }
}