use crate::traits::Comprehension;

/// Entities that contribute to the understanding of a language.
pub struct Suite<C: Comprehension> {
    descriptions: Vec<C::Description>,
    diagnostics: Vec<C::Diagnostic>,
    highlights: Vec<C::Highlight>
}

impl<C: Comprehension> Suite<C> {
    /// Returns the a reference to the contained description entities.
    pub fn descriptions(&self) -> &[C::Description] {
        &self.descriptions
    }

    /// Returns a reference to the contained diagnostic entities.
    pub fn diagnostics(&self) -> &[C::Diagnostic] {
        &self.diagnostics
    }

    /// Returns a reference to the contained highlight entities.
    pub fn highlights(&self) -> &[C::Highlight] {
        &self.highlights
    }
}