mod description;
use description::PythonDescription;

mod diagnostic;
use diagnostic::PythonDiagnostic;

mod highlight;
use highlight::PythonHighlight;

use catena_core::traits::Comprehension;

use crate::formal::Python;

impl Comprehension for Python {
    type Description = PythonDescription;
    type Diagnostic = PythonDiagnostic;
    type Highlight = PythonHighlight;
}