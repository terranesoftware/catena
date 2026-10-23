use std::range::Range;

use catena_suite::traits::traversal::Segmenter;

pub struct JapaneseSegmenter;

impl Segmenter for JapaneseSegmenter {
    fn segment(
        &self,
        text: &[u8]
    ) -> Vec<Range<usize>> {
        Vec::new()
    }
}