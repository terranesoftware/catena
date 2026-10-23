mod completer;
use completer::JapaneseCompleter;

mod formatter;
use formatter::JapaneseFormatter;

mod propagator;
use propagator::JapanesePropagator;

mod suggester;
use suggester::JapaneseSuggester;

use catena_suite::traits::Transformation;

use crate::natural::Japanese;

impl Transformation for Japanese {
    type Completer = JapaneseCompleter;
    type Formatter = JapaneseFormatter;
    type Propagator = JapanesePropagator;
    type Suggester = JapaneseSuggester;
}