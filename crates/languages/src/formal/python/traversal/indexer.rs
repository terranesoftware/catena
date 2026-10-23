use std::range::Range;

use catena_suite::traits::traversal::Indexer;

pub struct PythonIndexer;

impl Indexer for PythonIndexer {
    fn index(
        &self,
        text: &[u8]
    ) -> Vec<Range<usize>> {
        Vec::new()
    }
}