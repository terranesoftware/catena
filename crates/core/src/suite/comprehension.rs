use crate::traits::Comprehension;

/// Entities that contribute to the understanding of a language.
pub struct Suite<C: Comprehension> {
    descriptions: Vec<C::Description>,
    diagnostics: Vec<C::Diagnostic>,
    highlights: Vec<C::Highlight>
}

impl<C: Comprehension> Suite<C> {
    pub fn descriptions(&self) -> &[C::Description] {
        &self.descriptions
    }

    pub fn diagnostics(&self) -> &[C::Diagnostic] {
        &self.diagnostics
    }

    pub fn highlights(&self) -> &[C::Highlight] {
        &self.highlights
    }
}