use std::range::Range;

use catena_suite::traits::traversal::Grammar;

pub struct RustGrammar;

impl Grammar for RustGrammar {
    fn grammar(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        Range::default()
    }
}