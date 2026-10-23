use std::range::Range;

pub trait Segmenter {
    fn segment(
        &self,
        text: &[u8]
    ) -> Vec<Range<usize>>;
}