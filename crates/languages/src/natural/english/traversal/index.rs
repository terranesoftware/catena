use std::range::Range;

use catena_suite::traits::traversal::Index;

pub struct EnglishIndex;

impl Index for EnglishIndex {
    fn index(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        Range::default()
    }
}