mod descriptions;
use descriptions::RustDescriptions;

mod diagnostics;
use diagnostics::RustDiagnostics;

mod highlights;
use highlights::RustHighlights;

use catena_suite::traits::Comprehension;

use crate::formal::Rust;

impl Comprehension for Rust {
    type Descriptions = RustDescriptions;
    type Diagnostics = RustDiagnostics;
    type Highlights = RustHighlights;
}