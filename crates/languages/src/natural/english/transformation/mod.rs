mod completer;
use completer::EnglishCompleter;

mod formatter;
use formatter::EnglishFormatter;

mod propagator;
use propagator::EnglishPropagator;

mod suggester;
use suggester::EnglishSuggester;

use catena_suite::traits::Transformation;

use crate::natural::English;

impl Transformation for English {
    type Completer = EnglishCompleter;
    type Formatter = EnglishFormatter;
    type Propagator = EnglishPropagator;
    type Suggester = EnglishSuggester;
}