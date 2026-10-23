use std::range::Range;

use catena_suite::traits::traversal::Indexer;

pub struct RustIndexer;

impl Indexer for RustIndexer {
    fn index(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        Range::default()
    }
}