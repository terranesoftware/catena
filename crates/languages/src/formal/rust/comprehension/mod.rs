mod description;
use description::RustDescription;

mod diagnostic;
use diagnostic::RustDiagnostic;

mod highlight;
use highlight::RustHighlight;

use catena_suite::traits::Comprehension;

use crate::formal::Rust;

impl Comprehension for Rust {
    type Description = RustDescription;
    type Diagnostic = RustDiagnostic;
    type Highlight = RustHighlight;
}