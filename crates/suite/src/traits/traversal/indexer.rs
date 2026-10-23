use std::range::Range;

pub trait Indexer {
    fn index(
        &self,
        text: &[u8]
    ) -> Vec<Range<usize>>;
}