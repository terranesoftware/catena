use crate::traits::{Comprehension, comprehension::{Descriptor, Diagnoser, descriptor::Description, diagnoser::Diagnostic}};

/// Entities that contribute to the understanding of a language.
pub struct Suite<C: Comprehension> {
    descriptor: C::Descriptor,
    diagnoser: C::Diagnoser
}

impl<C: Comprehension> Suite<C> {
    /// Describes the supplied text.
    pub fn describe(
        &self,
        text: &[u8]
    ) -> Description {
        self.descriptor.describe(text)
    }

    /// Diagnoses the supplied text.
    pub fn diagnose(
        &self,
        text: &[u8]
    ) -> Vec<Diagnostic> {
        self.diagnoser.diagnose(text)
    }
}