mod completion;
use completion::EnglishCompletion;

mod formatter;
use formatter::EnglishFormatter;

mod propagator;
use propagator::EnglishPropagator;

mod suggestion;
use suggestion::EnglishSuggestion;

use catena_suite::traits::Transformation;

use crate::natural::English;

impl Transformation for English {
    type Completion = EnglishCompletion;
    type Formatter = EnglishFormatter;
    type Propagator = EnglishPropagator;
    type Suggestion = EnglishSuggestion;
}