mod completion;
use completion::JapaneseCompletion;

mod formatter;
use formatter::JapaneseFormatter;

mod propagator;
use propagator::JapanesePropagator;

mod suggestion;
use suggestion::JapaneseSuggestion;

use catena_suite::traits::Transformation;

use crate::natural::Japanese;

impl Transformation for Japanese {
    type Completion = JapaneseCompletion;
    type Formatter = JapaneseFormatter;
    type Propagator = JapanesePropagator;
    type Suggestion = JapaneseSuggestion;
}