use std::range::Range;

pub struct Description {
    pub subjects: Vec<Range<usize>>,
    pub text: String
}