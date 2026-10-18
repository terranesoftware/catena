mod descriptions;
use descriptions::PythonDescriptions;

mod diagnostics;
use diagnostics::PythonDiagnostics;

mod highlights;
use highlights::PythonHighlights;

use catena_suite::traits::Comprehension;

use crate::formal::Python;

impl Comprehension for Python {
    type Descriptions = PythonDescriptions;
    type Diagnostics = PythonDiagnostics;
    type Highlights = PythonHighlights;
}