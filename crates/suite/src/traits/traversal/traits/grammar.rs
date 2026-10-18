use std::range::Range;

pub trait Grammar {
    fn grammar(
        &self,
        text: &[u8]
    ) -> Range<usize>;
}