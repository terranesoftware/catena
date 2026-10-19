use crate::traits::{Comprehension, comprehension::{Descriptor, Diagnoser}};

/// Entities that contribute to the understanding of a language.
pub struct Suite<C: Comprehension> {
    descriptor: C::Descriptor,
    diagnoser: C::Diagnoser
}

impl<C: Comprehension> Suite<C> {
    pub fn describe(
        &self,
        text: &[u8]
    ) {
        self.descriptor.describe(text);
    }

    pub fn diagnose(
        &self,
        text: &[u8]
    ) {
        self.diagnoser.diagnose(text);
    }
}