use std::range::Range;

use catena_suite::traits::traversal::Grammar;

pub struct JapaneseGrammar;

impl Grammar for JapaneseGrammar {
    fn grammar(
        &self,
        text: &[u8]
    ) -> Range<usize> {
        Range::default()
    }
}