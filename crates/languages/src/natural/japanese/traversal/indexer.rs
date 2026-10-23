use std::range::Range;

use catena_suite::traits::traversal::Indexer;

pub struct JapaneseIndexer;

impl Indexer for JapaneseIndexer {
    fn index(
        &self,
        text: &[u8]
    ) -> Vec<Range<usize>> {
        Vec::new()
    }
}