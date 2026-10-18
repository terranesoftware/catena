mod descriptions;
use descriptions::JapaneseDescriptions;

mod diagnostics;
use diagnostics::JapaneseDiagnostics;

use catena_suite::traits::Comprehension;

use crate::natural::Japanese;

impl Comprehension for Japanese {
    type Descriptions = JapaneseDescriptions;
    type Diagnostics = JapaneseDiagnostics;
}