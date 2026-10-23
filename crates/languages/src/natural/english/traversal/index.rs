use std::range::Range;

use catena_suite::traits::traversal::Indexer;

pub struct EnglishIndexer;

impl Indexer for EnglishIndexer {
    fn index(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        Range::default()
    }
}