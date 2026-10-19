use crate::traits::{Transformation, transformation::{Completer, Edit, Formatter, Propagator, Suggester}};

/// Entities that contribute to the manipulation of a language.
pub struct Suite<T: Transformation> {
    completer: T::Completer,
    formatter: T::Formatter,
    propagator: T::Propagator,
    suggester: T::Suggester,
}

impl<T: Transformation> Suite<T> {
    pub fn complete(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        self.completer.complete(at, text)
    }

    pub fn format(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        self.formatter.format(text)
    }

    pub fn propagate(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        self.propagator.propagate(text)
    }

    pub fn suggest(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        self.suggester.suggest(text)
    }
}