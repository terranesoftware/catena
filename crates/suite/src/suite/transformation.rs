use crate::traits::{Transformation, transformation::{Completions, Formatter, Propagator, Suggestions}};

/// Entities that contribute to the manipulation of a language.
pub struct Suite<T: Transformation> {
    completions: T::Completions,
    formatter: T::Formatter,
    propagator: T::Propagator,
    suggestions: T::Suggestions,
}

impl<T: Transformation> Suite<T> {
    pub fn completions(&self) {
        self.completions.completions();
    }

    pub fn formatter(&self) {
        self.formatter.formatter();
    }

    pub fn propagator(&self) {
        self.propagator.propagator();
    }

    pub fn suggestions(&self) {
        self.suggestions.suggestions();
    }
}