use std::range::Range;

use catena_suite::traits::traversal::Segmenter;

pub struct PythonSegmenter;

impl Segmenter for PythonSegmenter {
    fn segment(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        Range::default()
    }
}