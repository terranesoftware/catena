mod descriptions;
use descriptions::JapaneseDescriptions;

mod diagnostics;
use diagnostics::JapaneseDiagnostics;

mod highlights;
use highlights::JapaneseHighlights;

use catena_suite::traits::Comprehension;

use crate::natural::Japanese;

impl Comprehension for Japanese {
    type Descriptions = JapaneseDescriptions;
    type Diagnostics = JapaneseDiagnostics;
    type Highlights = JapaneseHighlights;
}