use crate::traits::Transformation;

/// Entities that contribute to the manipulation of a language.
pub struct Suite<T: Transformation> {
    completions: Vec<T::Completion>,
    formatter: T::Formatter,
    propagator: T::Propagator,
    suggestions: Vec<T::Suggestion>,
}

impl<T: Transformation> Suite<T> {
    /// Returns a reference to the contained completion entities.
    pub fn completions(&self) -> &[T::Completion] {
        &self.completions
    }

    /// Returns a reference to the contained formatter entity.
    pub fn formatter(&self) -> &T::Formatter {
        &self.formatter
    }

    /// Returns a reference to the contained propagator entity.
    pub fn propagator(&self) -> &T::Propagator {
        &self.propagator
    }

    /// Returns a reference to the contained suggestion entities.
    pub fn suggestions(&self) -> &[T::Suggestion] {
        &self.suggestions
    }
}