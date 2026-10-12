use crate::traits::Transformation;

/// Entities that contribute to the manipulation of a language.
pub struct Suite<T: Transformation> {
    completions: Vec<T::Completion>,
    formatter: T::Formatter,
    propagator: T::Propagator,
    suggestions: Vec<T::Suggestion>,
}

impl<T: Transformation> Suite<T> {
    pub fn completions(&self) -> &[T::Completion] {
        &self.completions
    }

    pub fn formatter(&self) -> &T::Formatter {
        &self.formatter
    }

    pub fn propagator(&self) -> &T::Propagator {
        &self.propagator
    }

    pub fn suggestions(&self) -> &[T::Suggestion] {
        &self.suggestions
    }
}