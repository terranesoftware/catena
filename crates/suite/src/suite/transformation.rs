use crate::traits::{Transformation, transformation::{Completions, Formatter, Propagator, Suggestions}};

/// Entities that contribute to the manipulation of a language.
pub struct Suite<T: Transformation> {
    completions: T::Completions,
    formatter: T::Formatter,
    propagator: T::Propagator,
    suggestions: T::Suggestions,
}

impl<T: Transformation> Suite<T> {
    pub fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) {
        self.completions.completions(at, text);
    }

    pub fn formatter(
        &self,
        text: &[u8]
    ) {
        self.formatter.formatter(text);
    }

    pub fn propagator(
        &self,
        text: &[u8]
    ) {
        self.propagator.propagator(text);
    }

    pub fn suggestions(
        &self,
        text: &[u8]
    ) {
        self.suggestions.suggestions(text);
    }
}