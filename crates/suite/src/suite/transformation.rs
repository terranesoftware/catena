use crate::traits::{Transformation, transformation::{Completer, Edit, Formatter, Propagator, Suggester}};

/// Entities that contribute to the manipulation of a language.
pub struct Suite<T: Transformation> {
    completer: T::Completer,
    formatter: T::Formatter,
    propagator: T::Propagator,
    suggester: T::Suggester,
}

impl<T: Transformation> Suite<T> {
    /// Completes the supplied text at the specified index.
    pub fn complete(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        self.completer.complete(at, text)
    }

    /// Formats the supplied text.
    pub fn format(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        self.formatter.format(text)
    }

    /// If applicable, propagates the edit(s) in the supplied text.
    pub fn propagate(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        self.propagator.propagate(text)
    }

    /// Provides suggestions for the supplied text.
    pub fn suggest(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        self.suggester.suggest(text)
    }
}