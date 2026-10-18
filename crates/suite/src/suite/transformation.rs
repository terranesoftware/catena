use crate::traits::Transformation;

/// Entities that contribute to the manipulation of a language.
pub struct Suite<T: Transformation> {
    completions: T::Completions,
    formatter: T::Formatter,
    propagator: T::Propagator,
    suggestions: T::Suggestions,
}

impl<T: Transformation> Suite<T> {
    /// Returns a reference to the contained `Completions` entity.
    pub fn completions(&self) -> &T::Completions {
        &self.completions
    }

    /// Returns a reference to the contained `Formatter` entity.
    pub fn formatter(&self) -> &T::Formatter {
        &self.formatter
    }

    /// Returns a reference to the contained `Propagator` entity.
    pub fn propagator(&self) -> &T::Propagator {
        &self.propagator
    }

    /// Returns a reference to the contained `Suggestions` entity.
    pub fn suggestions(&self) -> &T::Suggestions {
        &self.suggestions
    }
}