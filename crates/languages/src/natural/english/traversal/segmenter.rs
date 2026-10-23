use std::range::Range;

use catena_suite::traits::traversal::Segmenter;

pub struct EnglishSegmenter;

impl Segmenter for EnglishSegmenter {
    fn segment(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        Range::default()
    }
}