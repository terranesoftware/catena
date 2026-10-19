mod severity;
pub use severity::Severity;

use std::range::Range;

pub struct Diagnostic {
    region: Range<usize>,
    severity: Severity,
    text: String
}

impl Diagnostic {
    /// Returns a copy of the contained region.
    pub fn region(&self) -> Range<usize> {
        self.region
    }

    /// Returns a copy of the contained `Severity`.
    pub fn severity(&self) -> Severity {
        self.severity
    }

    /// Returns a reference to the contained text.
    pub fn text(&self) -> &str {
        &self.text
    }
}