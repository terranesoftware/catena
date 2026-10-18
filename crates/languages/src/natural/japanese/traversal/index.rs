use std::range::Range;

use catena_suite::traits::traversal::Index;

pub struct JapaneseIndex;

impl Index for JapaneseIndex {
    fn index(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        Range::default()
    }
}