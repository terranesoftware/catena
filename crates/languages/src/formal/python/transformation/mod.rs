mod completion;
use completion::PythonCompletion;

mod formatter;
use formatter::PythonFormatter;

mod propagator;
use propagator::PythonPropagator;

mod suggestion;
use suggestion::PythonSuggestion;

use catena_core::traits::Transformation;

use crate::formal::Python;

impl Transformation for Python {
    type Completion = PythonCompletion;
    type Formatter = PythonFormatter;
    type Propagator = PythonPropagator;
    type Suggestion = PythonSuggestion;
}