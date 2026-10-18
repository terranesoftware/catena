use std::range::Range;

pub struct Edit {
    region: Range<usize>,
    replacement: Vec<u8>
}

impl Edit {
    /// Returns a copy of the contained region.
    pub fn region(&self) -> Range<usize> {
        self.region
    }

    /// Returns a reference to the contained replacement.
    pub fn replacement(&self) -> &[u8] {
        &self.replacement
    }
}