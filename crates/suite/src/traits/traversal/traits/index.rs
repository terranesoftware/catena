use std::range::Range;

pub trait Index {
    fn index(
        &self,
        text: &[u8]
    ) -> Range<usize>;
}