use std::range::Range;

use crate::traits::{Traversal, traversal::{Indexer, Segmenter}};

/// Entities that contribute to the navigation of a language.
pub struct Suite<T: Traversal> {
    indexer: T::Indexer,
    segmenter: T::Segmenter
}

impl<T: Traversal> Suite<T> {
    /// Indexes the supplied text.
    pub fn index(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        self.indexer.index(text)
    }

    /// Segments the supplied text.
    pub fn segment(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        self.segmenter.segment(text)
    }
}