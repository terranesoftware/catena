mod completer;
use completer::PythonCompleter;

mod formatter;
use formatter::PythonFormatter;

mod propagator;
use propagator::PythonPropagator;

mod suggester;
use suggester::PythonSuggester;

use catena_suite::traits::Transformation;

use crate::formal::Python;

impl Transformation for Python {
    type Completer = PythonCompleter;
    type Formatter = PythonFormatter;
    type Propagator = PythonPropagator;
    type Suggester = PythonSuggester;
}