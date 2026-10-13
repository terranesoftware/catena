mod description;
use description::JapaneseDescription;

mod diagnostic;
use diagnostic::JapaneseDiagnostic;

mod highlight;
use highlight::JapaneseHighlight;

use catena_suite::traits::Comprehension;

use crate::natural::Japanese;

impl Comprehension for Japanese {
    type Description = JapaneseDescription;
    type Diagnostic = JapaneseDiagnostic;
    type Highlight = JapaneseHighlight;
}