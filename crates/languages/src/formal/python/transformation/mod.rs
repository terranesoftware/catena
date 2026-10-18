mod completions;
use completions::PythonCompletions;

mod formatter;
use formatter::PythonFormatter;

mod propagator;
use propagator::PythonPropagator;

mod suggestions;
use suggestions::PythonSuggestions;

use catena_suite::traits::Transformation;

use crate::formal::Python;

impl Transformation for Python {
    type Completions = PythonCompletions;
    type Formatter = PythonFormatter;
    type Propagator = PythonPropagator;
    type Suggestions = PythonSuggestions;
}