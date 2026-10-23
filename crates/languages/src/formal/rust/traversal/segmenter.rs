use std::range::Range;

use catena_suite::traits::traversal::Segmenter;

pub struct RustSegmenter;

impl Segmenter for RustSegmenter {
    fn segment(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        Range::default()
    }
}