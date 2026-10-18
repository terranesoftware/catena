mod descriptions;
use descriptions::EnglishDescriptions;

mod diagnostics;
use diagnostics::EnglishDiagnostics;

use catena_suite::traits::Comprehension;

use crate::natural::English;

impl Comprehension for English {
    type Descriptions = EnglishDescriptions;
    type Diagnostics = EnglishDiagnostics;
}