use crate::traits::{Transformation, transformation::{Completions, Edit, Formatter, Propagator, Suggestions}};

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
    ) -> Vec<Edit> {
        self.completions.completions(at, text)
    }

    pub fn formatter(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        self.formatter.formatter(text)
    }

    pub fn propagator(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        self.propagator.propagator(text)
    }

    pub fn suggestions(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        self.suggestions.suggestions(text)
    }
}