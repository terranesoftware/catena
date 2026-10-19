use std::range::Range;

use crate::traits::{Traversal, traversal::{Indexer, Segmenter}};

/// Entities that contribute to the navigation of a language.
pub struct Suite<T: Traversal> {
    indexer: T::Indexer,
    segmenter: T::Segmenter
}

impl<T: Traversal> Suite<T> {
    pub fn index(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        self.indexer.index(text)
    }

    pub fn segment(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        self.segmenter.segment(text)
    }
}