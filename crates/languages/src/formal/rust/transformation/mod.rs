mod completer;
use completer::RustCompleter;

mod formatter;
use formatter::RustFormatter;

mod propagator;
use propagator::RustPropagator;

mod suggester;
use suggester::RustSuggester;

use catena_suite::traits::Transformation;

use crate::formal::Rust;

impl Transformation for Rust {
    type Completer = RustCompleter;
    type Formatter = RustFormatter;
    type Propagator = RustPropagator;
    type Suggester = RustSuggester;
}