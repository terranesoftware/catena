mod description;
use description::EnglishDescription;

mod diagnostic;
use diagnostic::EnglishDiagnostic;

mod highlight;
use highlight::EnglishHighlight;

use catena_suite::traits::Comprehension;

use crate::natural::English;

impl Comprehension for English {
    type Description = EnglishDescription;
    type Diagnostic = EnglishDiagnostic;
    type Highlight = EnglishHighlight;
}